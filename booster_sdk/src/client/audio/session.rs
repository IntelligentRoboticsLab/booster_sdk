//! Per-session handles mirroring the C++ `AudioPlayer`, `AudioRecorder`,
//! `AudioLocalizer` and `AudioCaptureStream` classes.
//!
//! Each handle wraps an `Arc<AudioClient>` plus the service session id and a
//! cached snapshot of the session state. The cache is updated from RPC results
//! (`init`/`start`/… and `get_info`) and, when the caller forwards topic
//! samples, from [`AudioProgressTopic`] / [`AudioEventTopic`] via the
//! `apply_*` methods (the C++ classes do this with internal listeners).
//!
//! Unlike the C++ destructors, dropping a handle does not destroy the
//! service-side session; call `destroy()` explicitly.

use std::sync::{Arc, Mutex};

use super::{
    AudioCaptureFrameTopic, AudioCaptureStreamInfo, AudioCaptureStreamOptions,
    AudioCaptureStreamState, AudioClient, AudioEventTopic, AudioProgressTopic, AudioTopicHeader,
    InitCaptureStreamResponse, PlayerInfo, PlayerInitOptions, PlayerState, RecorderInfo,
    RecorderInitOptions, RecorderState, SessionType,
};
use crate::{
    dds::DdsSubscription,
    types::{BoosterError, Result},
};

#[derive(Debug, Default)]
struct SessionData<I> {
    session_id: Option<i64>,
    info: I,
}

/// Session id plus cached info, guarded by a short-lived mutex (never held
/// across an await point).
#[derive(Debug)]
struct SessionCell<I> {
    kind: &'static str,
    session_type: SessionType,
    data: Mutex<SessionData<I>>,
}

impl<I: Clone + Default> SessionCell<I> {
    fn new(kind: &'static str, session_type: SessionType, session_id: Option<i64>) -> Self {
        Self {
            kind,
            session_type,
            data: Mutex::new(SessionData {
                session_id,
                info: I::default(),
            }),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, SessionData<I>> {
        self.data.lock().expect("audio session mutex")
    }

    fn session_id(&self) -> Option<i64> {
        self.lock().session_id
    }

    fn require_id(&self) -> Result<i64> {
        self.session_id().ok_or_else(|| {
            BoosterError::Validation(format!("audio {} is not initialized", self.kind))
        })
    }

    fn require_uninitialized(&self) -> Result<()> {
        match self.session_id() {
            Some(id) => Err(BoosterError::Validation(format!(
                "audio {} already has session {id}; destroy it first",
                self.kind
            ))),
            None => Ok(()),
        }
    }

    fn start_session(&self, session_id: i64, info: I) {
        let mut data = self.lock();
        data.session_id = Some(session_id);
        data.info = info;
    }

    fn end_session(&self) {
        let mut data = self.lock();
        data.session_id = None;
        data.info = I::default();
    }

    fn info(&self) -> I {
        self.lock().info.clone()
    }

    fn update(&self, f: impl FnOnce(&mut I)) {
        f(&mut self.lock().info);
    }

    /// Apply `f` if `header` refers to this session.
    fn update_if_owned(&self, header: &AudioTopicHeader, f: impl FnOnce(&mut I)) -> bool {
        let mut data = self.lock();
        let owned = data.session_id == Some(header.session_id)
            && header.session_type_enum() == Some(self.session_type);
        if owned {
            f(&mut data.info);
        }
        owned
    }

    fn owns(&self, header: &AudioTopicHeader) -> bool {
        self.session_id() == Some(header.session_id)
            && header.session_type_enum() == Some(self.session_type)
    }
}

/// One playback session (C++ `AudioPlayer`).
#[derive(Debug)]
pub struct AudioPlayer {
    client: Arc<AudioClient>,
    session: SessionCell<PlayerInfo>,
}

impl AudioPlayer {
    /// Create an uninitialized player; call [`AudioPlayer::init`] next.
    #[must_use]
    pub fn new(client: Arc<AudioClient>) -> Self {
        Self {
            client,
            session: SessionCell::new("player", SessionType::Player, None),
        }
    }

    /// Wrap an existing service-side player session.
    #[must_use]
    pub fn attach(client: Arc<AudioClient>, session_id: i64) -> Self {
        Self {
            client,
            session: SessionCell::new("player", SessionType::Player, Some(session_id)),
        }
    }

    #[must_use]
    pub fn client(&self) -> &Arc<AudioClient> {
        &self.client
    }

    /// Service session id, or `None` before `init` / after `destroy`.
    #[must_use]
    pub fn session_id(&self) -> Option<i64> {
        self.session.session_id()
    }

    /// Initialize the source and format; returns the new session id.
    pub async fn init(&self, options: &PlayerInitOptions) -> Result<i64> {
        self.session.require_uninitialized()?;
        let response = self.client.init_player(options).await?;
        self.session.start_session(
            response.session_id,
            PlayerInfo {
                state: PlayerState::Ready.into(),
                ..PlayerInfo::default()
            },
        );
        Ok(response.session_id)
    }

    fn set_cached_state(&self, state: PlayerState) {
        self.session.update(|info| info.state = state.into());
    }

    /// Start or resume playback.
    pub async fn start(&self) -> Result<()> {
        self.client.start_player(self.session.require_id()?).await?;
        self.set_cached_state(PlayerState::Playing);
        Ok(())
    }

    /// Pause playback.
    pub async fn pause(&self) -> Result<()> {
        self.client.pause_player(self.session.require_id()?).await?;
        self.set_cached_state(PlayerState::Paused);
        Ok(())
    }

    /// Stop playback and reset the play cursor.
    pub async fn stop(&self) -> Result<()> {
        self.client.stop_player(self.session.require_id()?).await?;
        self.set_cached_state(PlayerState::Stopped);
        Ok(())
    }

    /// Reset a stopped/completed player for reuse.
    pub async fn reset(&self) -> Result<()> {
        self.client.reset_player(self.session.require_id()?).await?;
        self.session.update(|info| {
            info.state = PlayerState::Ready.into();
            info.played_bytes = 0;
        });
        Ok(())
    }

    /// Destroy the service-side session.
    pub async fn destroy(&self) -> Result<()> {
        self.client
            .destroy_player(self.session.require_id()?)
            .await?;
        self.session.end_session();
        Ok(())
    }

    /// Alias for [`AudioPlayer::destroy`].
    pub async fn release(&self) -> Result<()> {
        self.destroy().await
    }

    /// Set the session volume.
    pub async fn set_volume(&self, volume: f32) -> Result<()> {
        self.client
            .set_player_volume(self.session.require_id()?, volume)
            .await?;
        self.session.update(|info| info.volume = volume);
        Ok(())
    }

    /// Select an output endpoint by [`super::AudioDeviceInfo::device_id`].
    pub async fn set_preferred_device(&self, device_id: impl AsRef<str>) -> Result<()> {
        self.client
            .set_preferred_device(self.session.require_id()?, device_id)
            .await
    }

    /// Query playback information from the service and refresh the cache.
    pub async fn get_info(&self) -> Result<PlayerInfo> {
        let info = self
            .client
            .get_player_info(self.session.require_id()?)
            .await?;
        self.session.update(|cached| *cached = info.clone());
        Ok(info)
    }

    /// Append PCM bytes when the source type is `PcmStream`.
    pub async fn push_pcm_stream(&self, data: &[u8]) -> Result<()> {
        self.client
            .send_pcm_data(self.session.require_id()?, data.to_vec())
            .await
    }

    /// Last known playback information, without an RPC.
    #[must_use]
    pub fn cached_info(&self) -> PlayerInfo {
        self.session.info()
    }

    /// Last known state, without an RPC. Unknown raw values map to `Error`.
    #[must_use]
    pub fn cached_state(&self) -> PlayerState {
        self.cached_info()
            .state_enum()
            .unwrap_or(PlayerState::Error)
    }

    /// Whether a topic header refers to this player session.
    #[must_use]
    pub fn owns(&self, header: &AudioTopicHeader) -> bool {
        self.session.owns(header)
    }

    /// Update the cache from a progress sample; returns whether it matched.
    pub fn apply_progress(&self, progress: &AudioProgressTopic) -> bool {
        self.session.update_if_owned(&progress.header, |info| {
            info.played_bytes = progress.position_bytes;
            info.total_bytes = progress.duration_bytes;
            info.state = progress.state;
        })
    }

    /// Update the cached state from an event sample; returns whether it matched.
    pub fn apply_event(&self, event: &AudioEventTopic) -> bool {
        self.session
            .update_if_owned(&event.header, |info| info.state = event.state)
    }
}

/// One recording session (C++ `AudioRecorder`).
#[derive(Debug)]
pub struct AudioRecorder {
    client: Arc<AudioClient>,
    session: SessionCell<RecorderInfo>,
}

impl AudioRecorder {
    /// Create an uninitialized recorder; call [`AudioRecorder::init`] next.
    #[must_use]
    pub fn new(client: Arc<AudioClient>) -> Self {
        Self {
            client,
            session: SessionCell::new("recorder", SessionType::Recorder, None),
        }
    }

    /// Wrap an existing service-side recorder session.
    #[must_use]
    pub fn attach(client: Arc<AudioClient>, session_id: i64) -> Self {
        Self {
            client,
            session: SessionCell::new("recorder", SessionType::Recorder, Some(session_id)),
        }
    }

    #[must_use]
    pub fn client(&self) -> &Arc<AudioClient> {
        &self.client
    }

    #[must_use]
    pub fn session_id(&self) -> Option<i64> {
        self.session.session_id()
    }

    /// Initialize the output path and PCM format; returns the new session id.
    pub async fn init(&self, options: &RecorderInitOptions) -> Result<i64> {
        self.session.require_uninitialized()?;
        let response = self.client.init_recorder(options).await?;
        self.session.start_session(
            response.session_id,
            RecorderInfo {
                state: RecorderState::Ready.into(),
                captured_bytes: 0,
            },
        );
        Ok(response.session_id)
    }

    fn set_cached_state(&self, state: RecorderState) {
        self.session.update(|info| info.state = state.into());
    }

    /// Start or resume recording.
    pub async fn start(&self) -> Result<()> {
        self.client
            .start_recorder(self.session.require_id()?)
            .await?;
        self.set_cached_state(RecorderState::Recording);
        Ok(())
    }

    /// Pause recording.
    pub async fn pause(&self) -> Result<()> {
        self.client
            .pause_recorder(self.session.require_id()?)
            .await?;
        self.set_cached_state(RecorderState::Paused);
        Ok(())
    }

    /// Stop recording and finalize the output.
    pub async fn stop(&self) -> Result<()> {
        self.client
            .stop_recorder(self.session.require_id()?)
            .await?;
        self.set_cached_state(RecorderState::Stopped);
        Ok(())
    }

    /// Destroy the service-side session.
    pub async fn destroy(&self) -> Result<()> {
        self.client
            .destroy_recorder(self.session.require_id()?)
            .await?;
        self.session.end_session();
        Ok(())
    }

    /// Alias for [`AudioRecorder::destroy`].
    pub async fn release(&self) -> Result<()> {
        self.destroy().await
    }

    /// Select an input endpoint by [`super::AudioDeviceInfo::device_id`].
    pub async fn set_preferred_device(&self, device_id: impl AsRef<str>) -> Result<()> {
        self.client
            .set_preferred_device(self.session.require_id()?, device_id)
            .await
    }

    /// Query recording information from the service and refresh the cache.
    pub async fn get_info(&self) -> Result<RecorderInfo> {
        let info = self
            .client
            .get_recorder_info(self.session.require_id()?)
            .await?;
        self.session.update(|cached| *cached = info.clone());
        Ok(info)
    }

    #[must_use]
    pub fn cached_info(&self) -> RecorderInfo {
        self.session.info()
    }

    /// Last known state, without an RPC. Unknown raw values map to `Error`.
    #[must_use]
    pub fn cached_state(&self) -> RecorderState {
        self.cached_info()
            .state_enum()
            .unwrap_or(RecorderState::Error)
    }

    #[must_use]
    pub fn owns(&self, header: &AudioTopicHeader) -> bool {
        self.session.owns(header)
    }

    /// Update the cache from a progress sample; returns whether it matched.
    pub fn apply_progress(&self, progress: &AudioProgressTopic) -> bool {
        self.session.update_if_owned(&progress.header, |info| {
            info.captured_bytes = progress.position_bytes;
            info.state = progress.state;
        })
    }

    /// Update the cached state from an event sample; returns whether it matched.
    pub fn apply_event(&self, event: &AudioEventTopic) -> bool {
        self.session
            .update_if_owned(&event.header, |info| info.state = event.state)
    }
}

/// Direction-of-arrival estimates (C++ `AudioLocalizer`).
///
/// Only available with a compatible microphone array; otherwise the service
/// fails with [`super::AUDIO_ERR_NOT_SUPPORTED`].
#[derive(Debug, Clone)]
pub struct AudioLocalizer {
    client: Arc<AudioClient>,
}

impl AudioLocalizer {
    #[must_use]
    pub fn new(client: Arc<AudioClient>) -> Self {
        Self { client }
    }

    #[must_use]
    pub fn client(&self) -> &Arc<AudioClient> {
        &self.client
    }

    /// Current DOA angle in degrees.
    pub async fn get_doa_angle(&self) -> Result<i32> {
        self.client.get_doa_angle().await
    }
}

#[derive(Debug, Clone, Default)]
struct CaptureStreamCache {
    info: AudioCaptureStreamInfo,
    data_topic_name: String,
}

/// Raw/NAEC microphone PCM stream (C++ `AudioCaptureStream`).
///
/// Frames are delivered over DDS; use [`AudioCaptureStream::subscribe_frames`]
/// and convert samples with `AudioCaptureFrame::from`.
#[derive(Debug)]
pub struct AudioCaptureStream {
    client: Arc<AudioClient>,
    session: SessionCell<CaptureStreamCache>,
}

impl AudioCaptureStream {
    /// Create an uninitialized capture stream; call [`AudioCaptureStream::init`] next.
    #[must_use]
    pub fn new(client: Arc<AudioClient>) -> Self {
        Self {
            client,
            session: SessionCell::new("capture stream", SessionType::CaptureStream, None),
        }
    }

    /// Wrap an existing service-side capture stream session.
    #[must_use]
    pub fn attach(
        client: Arc<AudioClient>,
        session_id: i64,
        data_topic_name: impl Into<String>,
    ) -> Self {
        let stream = Self {
            client,
            session: SessionCell::new(
                "capture stream",
                SessionType::CaptureStream,
                Some(session_id),
            ),
        };
        let data_topic_name = data_topic_name.into();
        stream
            .session
            .update(|cache| cache.data_topic_name = data_topic_name);
        stream
    }

    #[must_use]
    pub fn client(&self) -> &Arc<AudioClient> {
        &self.client
    }

    #[must_use]
    pub fn session_id(&self) -> Option<i64> {
        self.session.session_id()
    }

    /// Data topic name returned by the service at `init` (empty before).
    #[must_use]
    pub fn data_topic_name(&self) -> String {
        self.session.info().data_topic_name
    }

    /// Negotiate the requested raw/NAEC streams and formats.
    pub async fn init(
        &self,
        options: &AudioCaptureStreamOptions,
    ) -> Result<InitCaptureStreamResponse> {
        self.session.require_uninitialized()?;
        let response = self.client.init_capture_stream(options).await?;
        self.session.start_session(
            response.session_id,
            CaptureStreamCache {
                info: AudioCaptureStreamInfo {
                    state: AudioCaptureStreamState::Ready.into(),
                    raw_enabled: options.enable_raw_pcm,
                    naec_enabled: options.enable_naec_pcm,
                    ..AudioCaptureStreamInfo::default()
                },
                data_topic_name: response.data_topic_name.clone(),
            },
        );
        Ok(response)
    }

    fn set_cached_state(&self, state: AudioCaptureStreamState) {
        self.session.update(|cache| cache.info.state = state.into());
    }

    /// Start frame delivery.
    pub async fn start(&self) -> Result<()> {
        self.client
            .start_capture_stream(self.session.require_id()?)
            .await?;
        self.set_cached_state(AudioCaptureStreamState::Streaming);
        Ok(())
    }

    /// Pause frame delivery.
    pub async fn pause(&self) -> Result<()> {
        self.client
            .pause_capture_stream(self.session.require_id()?)
            .await?;
        self.set_cached_state(AudioCaptureStreamState::Paused);
        Ok(())
    }

    /// Stop frame delivery.
    pub async fn stop(&self) -> Result<()> {
        self.client
            .stop_capture_stream(self.session.require_id()?)
            .await?;
        self.set_cached_state(AudioCaptureStreamState::Stopped);
        Ok(())
    }

    /// Destroy the service-side stream.
    pub async fn destroy(&self) -> Result<()> {
        self.client
            .destroy_capture_stream(self.session.require_id()?)
            .await?;
        self.session.end_session();
        Ok(())
    }

    /// Select an input endpoint by [`super::AudioDeviceInfo::device_id`].
    pub async fn set_preferred_device(&self, device_id: impl AsRef<str>) -> Result<()> {
        self.client
            .set_preferred_device(self.session.require_id()?, device_id)
            .await
    }

    /// Query negotiated formats and counters and refresh the cache.
    pub async fn get_info(&self) -> Result<AudioCaptureStreamInfo> {
        let info = self
            .client
            .get_capture_stream_info(self.session.require_id()?)
            .await?;
        self.session.update(|cache| cache.info = info.clone());
        Ok(info)
    }

    #[must_use]
    pub fn cached_info(&self) -> AudioCaptureStreamInfo {
        self.session.info().info
    }

    /// Last known state, without an RPC. Unknown raw values map to `Error`.
    #[must_use]
    pub fn cached_state(&self) -> AudioCaptureStreamState {
        self.cached_info()
            .state_enum()
            .unwrap_or(AudioCaptureStreamState::Error)
    }

    #[must_use]
    pub fn owns(&self, header: &AudioTopicHeader) -> bool {
        self.session.owns(header)
    }

    /// Update the cached state from an event sample; returns whether it matched.
    pub fn apply_event(&self, event: &AudioEventTopic) -> bool {
        self.session
            .update_if_owned(&event.header, |cache| cache.info.state = event.state)
    }

    /// Subscribe to this stream's PCM frames (requires a prior `init`).
    pub fn subscribe_frames(&self) -> Result<DdsSubscription<AudioCaptureFrameTopic>> {
        self.session.require_id()?;
        self.client
            .subscribe_capture_frames(&self.data_topic_name())
    }
}
