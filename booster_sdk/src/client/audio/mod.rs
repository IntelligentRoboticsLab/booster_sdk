//! Audio service client for Booster SDK v1.8.
//!
//! Mirrors the C++ `booster::robot::audio` API:
//!
//! - [`AudioClient`] corresponds to `AudioManager`: registration, the flat
//!   per-method RPCs, system volume/mute, devices, Bluetooth and the
//!   progress/event/error/Bluetooth topics.
//! - [`AudioPlayer`], [`AudioRecorder`], [`AudioLocalizer`] and
//!   [`AudioCaptureStream`] are per-session handles created from an
//!   `Arc<AudioClient>` (`AudioManager::Create*`).
//!
//! Each audio RPC uses its own `rt/booster/audio/<method>` channel. Unlike other
//! services, the C++ audio API has no robot-name variant, so
//! [`RpcClientOptions::robot_name`] is ignored here.
//!
//! The C++ `CallService` uses a 1000 ms default timeout; this client keeps
//! [`RpcClientOptions::default_timeout`] (5 s by default) so slow calls such
//! as player initialization on a busy robot do not spuriously time out. Use
//! [`AudioClient::with_options`] with
//! [`RpcClientOptions::with_default_timeout`] to match the C++ value.

mod messages;
mod session;
mod types;

pub use messages::*;
pub use session::*;
pub use types::*;

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value, json};

use crate::{
    dds::{
        BluetoothEvent, DdsSubscription, RpcClient, RpcClientOptions, TopicSpec,
        audio_bluetooth_event_topic,
    },
    types::{BoosterError, Result, RpcError},
};

const AUDIO_RPC_API_ID: i32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AudioServiceMethod {
    RegisterClient,
    InitPlayer,
    StartPlayer,
    PausePlayer,
    StopPlayer,
    ResetPlayer,
    DestroyPlayer,
    SetPlayerVolume,
    GetPlayerInfo,
    SendPcmData,
    InitRecorder,
    StartRecorder,
    PauseRecorder,
    StopRecorder,
    DestroyRecorder,
    GetRecorderInfo,
    GetDoaAngle,
    SetSystemVolume,
    GetSystemVolume,
    SetSystemMute,
    GetSystemMute,
    InitCaptureStream,
    StartCaptureStream,
    PauseCaptureStream,
    StopCaptureStream,
    DestroyCaptureStream,
    GetCaptureStreamInfo,
    GetDevices,
    SetPreferredDevice,
    StartBluetoothScan,
    StopBluetoothScan,
    GetBluetoothDevices,
    ConnectBluetoothDevice,
    DisconnectBluetoothDevice,
    ForgetBluetoothDevice,
}

impl AudioServiceMethod {
    fn topic(self) -> &'static str {
        match self {
            Self::RegisterClient => "rt/booster/audio/register_client",
            Self::InitPlayer => "rt/booster/audio/init_player",
            Self::StartPlayer => "rt/booster/audio/start_player",
            Self::PausePlayer => "rt/booster/audio/pause_player",
            Self::StopPlayer => "rt/booster/audio/stop_player",
            Self::ResetPlayer => "rt/booster/audio/reset_player",
            Self::DestroyPlayer => "rt/booster/audio/destroy_player",
            Self::SetPlayerVolume => "rt/booster/audio/set_volume",
            Self::GetPlayerInfo => "rt/booster/audio/get_player_info",
            Self::SendPcmData => "rt/booster/audio/send_pcm_data",
            Self::InitRecorder => "rt/booster/audio/init_recorder",
            Self::StartRecorder => "rt/booster/audio/start_recorder",
            Self::PauseRecorder => "rt/booster/audio/pause_recorder",
            Self::StopRecorder => "rt/booster/audio/stop_recorder",
            Self::DestroyRecorder => "rt/booster/audio/destroy_recorder",
            Self::GetRecorderInfo => "rt/booster/audio/get_recorder_info",
            Self::GetDoaAngle => "rt/booster/audio/get_doa_angle",
            Self::SetSystemVolume => "rt/booster/audio/set_system_volume",
            Self::GetSystemVolume => "rt/booster/audio/get_system_volume",
            Self::SetSystemMute => "rt/booster/audio/set_system_mute",
            Self::GetSystemMute => "rt/booster/audio/get_system_mute",
            Self::InitCaptureStream => "rt/booster/audio/init_capture_stream",
            Self::StartCaptureStream => "rt/booster/audio/start_capture_stream",
            Self::PauseCaptureStream => "rt/booster/audio/pause_capture_stream",
            Self::StopCaptureStream => "rt/booster/audio/stop_capture_stream",
            Self::DestroyCaptureStream => "rt/booster/audio/destroy_capture_stream",
            Self::GetCaptureStreamInfo => "rt/booster/audio/get_capture_stream_info",
            Self::GetDevices => "rt/booster/audio/get_devices",
            Self::SetPreferredDevice => "rt/booster/audio/set_preferred_device",
            Self::StartBluetoothScan => "rt/booster/audio/start_bluetooth_scan",
            Self::StopBluetoothScan => "rt/booster/audio/stop_bluetooth_scan",
            Self::GetBluetoothDevices => "rt/booster/audio/get_bluetooth_devices",
            Self::ConnectBluetoothDevice => "rt/booster/audio/connect_bluetooth_device",
            Self::DisconnectBluetoothDevice => "rt/booster/audio/disconnect_bluetooth_device",
            Self::ForgetBluetoothDevice => "rt/booster/audio/forget_bluetooth_device",
        }
    }
}

#[derive(Debug, Deserialize)]
struct RegisterClientResponse {
    ret_code: i32,
    #[serde(default)]
    ret_msg: String,
    client_id: String,
}

#[derive(Debug, Deserialize)]
struct SystemVolumeResponse {
    ret_code: i32,
    #[serde(default)]
    ret_msg: String,
    volume: f32,
}

#[derive(Debug, Deserialize)]
struct SystemMuteResponse {
    ret_code: i32,
    #[serde(default)]
    ret_msg: String,
    mute: bool,
}

#[derive(Debug, Deserialize)]
struct DoaAngleResponse {
    ret_code: i32,
    #[serde(default)]
    ret_msg: String,
    angle_deg: i32,
}

fn service_result_error(ret_code: i32, ret_msg: String) -> crate::types::BoosterError {
    RpcError::RequestFailed {
        status: ret_code,
        message: ret_msg,
    }
    .into()
}

fn ensure_ret_code(ret_code: i32, ret_msg: String) -> Result<()> {
    if ret_code == 0 {
        Ok(())
    } else {
        Err(service_result_error(ret_code, ret_msg))
    }
}

fn serialize_request<T: Serialize>(request: &T) -> Result<Value> {
    Ok(serde_json::to_value(request)?)
}

/// Inject the `client_id`/`request_id` metadata every audio RPC carries.
fn with_client_meta(request: Value, client_id: String, request_id: String) -> Value {
    let mut object = match request {
        Value::Object(object) => object,
        Value::Null => Map::new(),
        other => {
            let mut object = Map::new();
            object.insert("value".to_owned(), other);
            object
        }
    };
    object.insert("client_id".to_owned(), Value::String(client_id));
    object.insert("request_id".to_owned(), Value::String(request_id));
    Value::Object(object)
}

fn set_preferred_device_body(session_id: i64, device_id: &str) -> Value {
    json!({ "session_id": session_id, "device_id": device_id })
}

/// High-level client for the audio service (C++ `AudioManager`).
///
/// Share it as `Arc<AudioClient>` to create per-session handles with
/// [`AudioClient::create_player`] and friends.
pub struct AudioClient {
    options: RpcClientOptions,
    clients: Mutex<HashMap<AudioServiceMethod, Arc<RpcClient>>>,
    client_id: Mutex<Option<String>>,
    request_sequence: AtomicU64,
}

impl std::fmt::Debug for AudioClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioClient")
            .field("client_id", &self.client_id())
            .finish_non_exhaustive()
    }
}

impl AudioClient {
    /// Create an audio client with default options.
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::default())
    }

    /// Create an audio client with a custom startup wait before first RPC on each audio channel.
    pub fn with_startup_wait(startup_wait: Duration) -> Result<Self> {
        Self::with_options(RpcClientOptions::default().with_startup_wait(startup_wait))
    }

    /// Create an audio client with custom RPC options.
    ///
    /// `options.robot_name` is ignored: the C++ audio service has no
    /// robot-name channel suffix.
    pub fn with_options(mut options: RpcClientOptions) -> Result<Self> {
        if options.robot_name.take().is_some() {
            tracing::debug!("audio client ignores robot_name; audio channels are not namespaced");
        }
        Ok(Self {
            options,
            clients: Mutex::new(HashMap::new()),
            client_id: Mutex::new(None),
            request_sequence: AtomicU64::new(1),
        })
    }

    /// Register with the robot audio service and cache the returned client id.
    pub async fn init(&self) -> Result<String> {
        self.register_client().await
    }

    /// Return the cached audio client id, if registration has completed.
    pub fn client_id(&self) -> Option<String> {
        self.client_id
            .lock()
            .expect("audio client_id mutex")
            .clone()
    }

    /// Whether registration with the audio service has completed
    /// (C++ `AudioManager::IsInitialized`).
    pub fn is_initialized(&self) -> bool {
        self.client_id().is_some()
    }

    /// Drop the cached client id and all per-method RPC channels
    /// (C++ `AudioManager::Shutdown`). A later call re-registers lazily.
    ///
    /// Existing topic subscriptions keep running until they are dropped.
    pub fn shutdown(&self) {
        self.clients.lock().expect("audio clients mutex").clear();
        *self.client_id.lock().expect("audio client_id mutex") = None;
    }

    /// Generate a request identifier (C++ `AudioManager::GenerateRequestId`):
    /// `audio_req_<unix_ms>_<sequence>`.
    pub fn generate_request_id(&self) -> String {
        let seq = self.request_sequence.fetch_add(1, Ordering::Relaxed);
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_millis());
        format!("audio_req_{now_ms}_{seq}")
    }

    fn next_request_id(&self) -> String {
        self.generate_request_id()
    }

    fn rpc_client(&self, method: AudioServiceMethod) -> Result<Arc<RpcClient>> {
        let mut clients = self.clients.lock().expect("audio clients mutex");
        if let Some(client) = clients.get(&method) {
            return Ok(Arc::clone(client));
        }

        let client = Arc::new(RpcClient::for_topic(
            self.options.clone(),
            method.topic().to_owned(),
        )?);
        clients.insert(method, Arc::clone(&client));
        Ok(client)
    }

    async fn register_client(&self) -> Result<String> {
        if let Some(client_id) = self.client_id() {
            return Ok(client_id);
        }

        let response: RegisterClientResponse = self
            .call_raw(
                AudioServiceMethod::RegisterClient,
                json!({ "request_id": self.next_request_id() }),
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;

        let mut client_id = self.client_id.lock().expect("audio client_id mutex");
        *client_id = Some(response.client_id.clone());
        Ok(response.client_id)
    }

    async fn ensure_registered(&self) -> Result<String> {
        match self.client_id() {
            Some(client_id) => Ok(client_id),
            None => self.register_client().await,
        }
    }

    async fn call_raw<R>(&self, method: AudioServiceMethod, request: Value) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        let client = self.rpc_client(method)?;
        client
            .call_response(AUDIO_RPC_API_ID, request.to_string())
            .await
    }

    async fn call_service<R>(&self, method: AudioServiceMethod, request: Value) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        let client_id = self.ensure_registered().await?;
        let request = with_client_meta(request, client_id, self.next_request_id());
        self.call_raw(method, request).await
    }

    async fn call_service_with_timeout<R>(
        &self,
        method: AudioServiceMethod,
        request: Value,
        timeout: Duration,
    ) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        let client_id = self.ensure_registered().await?;
        let request = with_client_meta(request, client_id, self.next_request_id());
        self.rpc_client(method)?
            .call_with_body(AUDIO_RPC_API_ID, request.to_string(), Some(timeout))
            .await
    }

    async fn call_result(&self, method: AudioServiceMethod, request: Value) -> Result<()> {
        let result: ServiceResult = self.call_service(method, request).await?;
        ensure_ret_code(result.ret_code, result.ret_msg)
    }

    /// Initialize an audio player and return its session id response.
    pub async fn init_player(&self, options: &PlayerInitOptions) -> Result<InitPlayerResponse> {
        let response: InitPlayerResponse = self
            .call_service(AudioServiceMethod::InitPlayer, serialize_request(options)?)
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg.clone())?;
        Ok(response)
    }

    pub async fn start_player(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::StartPlayer,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn pause_player(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::PausePlayer,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn stop_player(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::StopPlayer,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn reset_player(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::ResetPlayer,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn destroy_player(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::DestroyPlayer,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn set_player_volume(&self, session_id: i64, volume: f32) -> Result<()> {
        self.call_result(
            AudioServiceMethod::SetPlayerVolume,
            json!({ "session_id": session_id, "volume": volume }),
        )
        .await
    }

    pub async fn get_player_info(&self, session_id: i64) -> Result<PlayerInfo> {
        #[derive(Deserialize)]
        struct Response {
            ret_code: i32,
            #[serde(default)]
            ret_msg: String,
            state: i32,
            played_bytes: i64,
            total_bytes: i64,
            volume: f32,
        }

        let response: Response = self
            .call_service(
                AudioServiceMethod::GetPlayerInfo,
                json!({ "session_id": session_id }),
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(PlayerInfo {
            state: response.state,
            played_bytes: response.played_bytes,
            total_bytes: response.total_bytes,
            volume: response.volume,
        })
    }

    pub async fn send_pcm_data(&self, session_id: i64, pcm_bytes: Vec<u8>) -> Result<()> {
        self.call_result(
            AudioServiceMethod::SendPcmData,
            json!({ "session_id": session_id, "pcm_bytes": pcm_bytes }),
        )
        .await
    }

    pub async fn init_recorder(
        &self,
        options: &RecorderInitOptions,
    ) -> Result<InitRecorderResponse> {
        let response: InitRecorderResponse = self
            .call_service(
                AudioServiceMethod::InitRecorder,
                serialize_request(options)?,
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg.clone())?;
        Ok(response)
    }

    pub async fn start_recorder(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::StartRecorder,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn pause_recorder(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::PauseRecorder,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn stop_recorder(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::StopRecorder,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn destroy_recorder(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::DestroyRecorder,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn get_recorder_info(&self, session_id: i64) -> Result<RecorderInfo> {
        #[derive(Deserialize)]
        struct Response {
            ret_code: i32,
            #[serde(default)]
            ret_msg: String,
            state: i32,
            captured_bytes: i64,
        }

        let response: Response = self
            .call_service(
                AudioServiceMethod::GetRecorderInfo,
                json!({ "session_id": session_id }),
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(RecorderInfo {
            state: response.state,
            captured_bytes: response.captured_bytes,
        })
    }

    pub async fn get_doa_angle(&self) -> Result<i32> {
        let response: DoaAngleResponse = self
            .call_service(AudioServiceMethod::GetDoaAngle, Value::Null)
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(response.angle_deg)
    }

    pub async fn set_system_volume(&self, volume: f32) -> Result<()> {
        self.call_result(
            AudioServiceMethod::SetSystemVolume,
            json!({ "volume": volume }),
        )
        .await
    }

    pub async fn get_system_volume(&self) -> Result<f32> {
        let response: SystemVolumeResponse = self
            .call_service(AudioServiceMethod::GetSystemVolume, Value::Null)
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(response.volume)
    }

    pub async fn set_system_mute(&self, mute: bool) -> Result<()> {
        self.call_result(AudioServiceMethod::SetSystemMute, json!({ "mute": mute }))
            .await
    }

    pub async fn get_system_mute(&self) -> Result<bool> {
        let response: SystemMuteResponse = self
            .call_service(AudioServiceMethod::GetSystemMute, Value::Null)
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(response.mute)
    }

    pub async fn init_capture_stream(
        &self,
        options: &AudioCaptureStreamOptions,
    ) -> Result<InitCaptureStreamResponse> {
        let response: InitCaptureStreamResponse = self
            .call_service(
                AudioServiceMethod::InitCaptureStream,
                serialize_request(options)?,
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg.clone())?;
        Ok(response)
    }

    pub async fn start_capture_stream(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::StartCaptureStream,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn pause_capture_stream(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::PauseCaptureStream,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn stop_capture_stream(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::StopCaptureStream,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn destroy_capture_stream(&self, session_id: i64) -> Result<()> {
        self.call_result(
            AudioServiceMethod::DestroyCaptureStream,
            json!({ "session_id": session_id }),
        )
        .await
    }

    pub async fn get_capture_stream_info(&self, session_id: i64) -> Result<AudioCaptureStreamInfo> {
        #[derive(Deserialize)]
        struct Response {
            ret_code: i32,
            #[serde(default)]
            ret_msg: String,
            #[serde(flatten)]
            info: AudioCaptureStreamInfo,
        }

        let response: Response = self
            .call_service(
                AudioServiceMethod::GetCaptureStreamInfo,
                json!({ "session_id": session_id }),
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(response.info)
    }

    /// Query available input or output devices.
    pub async fn get_devices(
        &self,
        query_type: AudioDeviceQueryType,
    ) -> Result<Vec<AudioDeviceInfo>> {
        #[derive(Deserialize)]
        struct Response {
            ret_code: i32,
            #[serde(default)]
            ret_msg: String,
            #[serde(default)]
            devices: Vec<AudioDeviceInfo>,
        }

        let response: Response = self
            .call_service(
                AudioServiceMethod::GetDevices,
                json!({ "query_type": i32::from(query_type) }),
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(response.devices)
    }

    /// Start Bluetooth device discovery.
    pub async fn start_bluetooth_scan(&self, options: &BluetoothScanOptions) -> Result<()> {
        self.call_result(
            AudioServiceMethod::StartBluetoothScan,
            serialize_request(options)?,
        )
        .await
    }

    /// Stop Bluetooth device discovery.
    pub async fn stop_bluetooth_scan(&self) -> Result<()> {
        self.call_result(AudioServiceMethod::StopBluetoothScan, Value::Null)
            .await
    }

    /// List Bluetooth devices known to the audio service.
    pub async fn get_bluetooth_devices(
        &self,
        include_unpaired: bool,
    ) -> Result<Vec<BluetoothDeviceInfo>> {
        #[derive(Deserialize)]
        struct Response {
            ret_code: i32,
            #[serde(default)]
            ret_msg: String,
            #[serde(default)]
            devices: Vec<BluetoothDeviceInfo>,
        }

        let response: Response = self
            .call_service(
                AudioServiceMethod::GetBluetoothDevices,
                json!({ "include_unpaired": include_unpaired }),
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(response.devices)
    }

    /// Connect and optionally route audio through a Bluetooth device.
    pub async fn connect_bluetooth_device(
        &self,
        options: &BluetoothConnectOptions,
    ) -> Result<BluetoothConnectResult> {
        #[derive(Deserialize)]
        struct Response {
            ret_code: i32,
            #[serde(default)]
            ret_msg: String,
            #[serde(flatten)]
            result: BluetoothConnectResult,
        }

        let timeout_ms = options.timeout_ms.max(0) as u64 + 2_000;
        let response: Response = self
            .call_service_with_timeout(
                AudioServiceMethod::ConnectBluetoothDevice,
                serialize_request(options)?,
                Duration::from_millis(timeout_ms),
            )
            .await?;
        ensure_ret_code(response.ret_code, response.ret_msg)?;
        Ok(response.result)
    }

    /// Disconnect a Bluetooth device by address.
    pub async fn disconnect_bluetooth_device(&self, address: impl Into<String>) -> Result<()> {
        self.call_result(
            AudioServiceMethod::DisconnectBluetoothDevice,
            json!({ "address": address.into() }),
        )
        .await
    }

    /// Remove a paired Bluetooth device from the robot.
    pub async fn forget_bluetooth_device(&self, address: impl Into<String>) -> Result<()> {
        self.call_result(
            AudioServiceMethod::ForgetBluetoothDevice,
            json!({ "address": address.into() }),
        )
        .await
    }

    /// Select the input/output endpoint (an [`AudioDeviceInfo::device_id`])
    /// used by a player, recorder or capture-stream session.
    pub async fn set_preferred_device(
        &self,
        session_id: i64,
        device_id: impl AsRef<str>,
    ) -> Result<()> {
        self.call_result(
            AudioServiceMethod::SetPreferredDevice,
            set_preferred_device_body(session_id, device_id.as_ref()),
        )
        .await
    }

    /// Create an uninitialized player handle (C++ `AudioManager::CreatePlayer`).
    #[must_use]
    pub fn create_player(self: &Arc<Self>) -> AudioPlayer {
        AudioPlayer::new(Arc::clone(self))
    }

    /// Create an uninitialized recorder handle (C++ `AudioManager::CreateRecorder`).
    #[must_use]
    pub fn create_recorder(self: &Arc<Self>) -> AudioRecorder {
        AudioRecorder::new(Arc::clone(self))
    }

    /// Create a direction-of-arrival localizer (C++ `AudioManager::CreateLocalizer`).
    #[must_use]
    pub fn create_localizer(self: &Arc<Self>) -> AudioLocalizer {
        AudioLocalizer::new(Arc::clone(self))
    }

    /// Create an uninitialized capture stream handle
    /// (C++ `AudioManager::CreateCaptureStream`).
    #[must_use]
    pub fn create_capture_stream(self: &Arc<Self>) -> AudioCaptureStream {
        AudioCaptureStream::new(Arc::clone(self))
    }

    fn subscribe_topic<T>(&self, spec: &TopicSpec, buffer: usize) -> Result<DdsSubscription<T>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.rpc_client(AudioServiceMethod::RegisterClient)?
            .node()
            .subscribe(spec, buffer)
    }

    /// Subscribe to Bluetooth lifecycle events.
    pub fn subscribe_bluetooth_events(&self) -> Result<DdsSubscription<BluetoothEvent>> {
        self.subscribe_topic(&audio_bluetooth_event_topic(), 16)
    }

    /// Subscribe to playback/recording progress (`rt/booster/audio/progress`).
    pub fn subscribe_progress(&self) -> Result<DdsSubscription<AudioProgressTopic>> {
        self.subscribe_topic(&audio_progress_topic(), 64)
    }

    /// Subscribe to session lifecycle events (`rt/booster/audio/events`).
    pub fn subscribe_events(&self) -> Result<DdsSubscription<AudioEventTopic>> {
        self.subscribe_topic(&audio_event_topic(), 64)
    }

    /// Subscribe to service errors (`rt/booster/audio/errors`).
    pub fn subscribe_errors(&self) -> Result<DdsSubscription<AudioErrorTopic>> {
        self.subscribe_topic(&audio_error_topic(), 64)
    }

    /// Subscribe to PCM frames of a capture stream, using the
    /// `data_topic_name` returned by [`AudioClient::init_capture_stream`].
    /// See [`capture_frame_topic_name`] for how the name is mapped.
    pub fn subscribe_capture_frames(
        &self,
        data_topic_name: &str,
    ) -> Result<DdsSubscription<AudioCaptureFrameTopic>> {
        if data_topic_name.trim().is_empty() {
            return Err(BoosterError::Validation(
                "capture stream data_topic_name is empty".to_owned(),
            ));
        }
        self.subscribe_topic(&audio_capture_frame_topic(data_topic_name), 256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn bluetooth_options_match_wire_schema() {
        let options = BluetoothConnectOptions::new("AA:BB:CC:DD:EE:FF");
        assert_eq!(
            serde_json::to_value(options).unwrap(),
            json!({
                "address": "AA:BB:CC:DD:EE:FF",
                "auto_pair": true,
                "make_default": true,
                "preferred_profile": 0,
                "timeout_ms": 15000
            })
        );
    }

    #[test]
    fn sdk_1_7_audio_topics_match_installed_service() {
        assert_eq!(
            AudioServiceMethod::GetDevices.topic(),
            "rt/booster/audio/get_devices"
        );
        assert_eq!(
            AudioServiceMethod::ConnectBluetoothDevice.topic(),
            "rt/booster/audio/connect_bluetooth_device"
        );
        assert_eq!(
            AudioServiceMethod::ForgetBluetoothDevice.topic(),
            "rt/booster/audio/forget_bluetooth_device"
        );
        assert_eq!(
            AudioServiceMethod::SetPreferredDevice.topic(),
            "rt/booster/audio/set_preferred_device"
        );
    }

    #[test]
    fn set_preferred_device_request_body() {
        let body = with_client_meta(
            set_preferred_device_body(42, "alsa_output.usb"),
            "client-1".to_owned(),
            "audio_req_1".to_owned(),
        );
        assert_eq!(
            body,
            json!({
                "session_id": 42,
                "device_id": "alsa_output.usb",
                "client_id": "client-1",
                "request_id": "audio_req_1"
            })
        );
    }

    #[test]
    fn client_meta_wraps_null_and_scalars() {
        assert_eq!(
            with_client_meta(Value::Null, "c".to_owned(), "r".to_owned()),
            json!({ "client_id": "c", "request_id": "r" })
        );
        assert_eq!(
            with_client_meta(json!(3), "c".to_owned(), "r".to_owned()),
            json!({ "value": 3, "client_id": "c", "request_id": "r" })
        );
    }

    #[test]
    fn init_option_defaults_match_cpp() {
        assert_eq!(
            serde_json::to_value(PlayerInitOptions::default()).unwrap(),
            json!({
                "source_type": 0,
                "source_uri": "",
                "sample_rate_hz": 16000,
                "channels": 1,
                "bits_per_sample": 16,
                "priority": 1
            })
        );
        assert_eq!(
            serde_json::to_value(PlayerInitOptions::pcm_stream()).unwrap()["source_type"],
            json!(2)
        );
        assert_eq!(
            serde_json::to_value(RecorderInitOptions::default()).unwrap(),
            json!({
                "output_path": "",
                "sample_rate_hz": 16000,
                "channels": 1,
                "bits_per_sample": 16
            })
        );
        assert_eq!(
            serde_json::to_value(AudioCaptureStreamOptions::default()).unwrap(),
            json!({
                "enable_raw_pcm": true,
                "enable_naec_pcm": false,
                "requested_raw_format": {
                    "sample_rate_hz": 16000,
                    "channels": 3,
                    "bits_per_sample": 16
                }
            })
        );
        let info = PlayerInfo::default();
        assert_eq!(info.state_enum(), Some(PlayerState::Idle));
        assert!((info.volume - 1.0).abs() < f32::EPSILON);
        assert_eq!(
            RecorderInfo::default().state_enum(),
            Some(RecorderState::Idle)
        );
    }

    #[test]
    fn audio_common_enums_round_trip() {
        for raw in 0..4 {
            assert_eq!(i32::from(SessionType::try_from(raw).unwrap()), raw);
        }
        assert!(SessionType::try_from(4).is_err());
        for raw in 0..13 {
            assert_eq!(i32::from(AudioErrorCode::try_from(raw).unwrap()), raw);
        }
        assert!(AudioErrorCode::try_from(13).is_err());
        for raw in 0..19 {
            assert_eq!(i32::from(AudioEventCode::try_from(raw).unwrap()), raw);
        }
        assert!(AudioEventCode::try_from(19).is_err());
        for raw in 0..8 {
            assert_eq!(i32::from(AudioErrorCategory::try_from(raw).unwrap()), raw);
        }
        assert!(AudioErrorCategory::try_from(8).is_err());
        for raw in 0..26 {
            assert_eq!(i32::from(AudioErrorDetail::try_from(raw).unwrap()), raw);
        }
        assert!(AudioErrorDetail::try_from(26).is_err());

        assert_eq!(
            AudioEventCode::CaptureStreamDestroySucceeded,
            AudioEventCode::try_from(18).unwrap()
        );
        assert_eq!(
            AudioErrorDetail::GetCaptureStreamInfo,
            AudioErrorDetail::try_from(25).unwrap()
        );
        assert_eq!(
            serde_json::to_value(AudioErrorCategory::InternalError).unwrap(),
            json!(7)
        );
        assert_eq!(AUDIO_ERR_NOT_SUPPORTED, 7001);
    }

    #[test]
    fn audio_topics_use_rt_prefix_and_dds_type_names() {
        let progress = audio_progress_topic();
        assert_eq!(progress.name, "rt/booster/audio/progress");
        assert_eq!(
            progress.type_name,
            "booster_msgs::audio::dds_::AudioProgressTopic_"
        );
        assert_eq!(audio_event_topic().name, "rt/booster/audio/events");
        assert_eq!(audio_error_topic().name, "rt/booster/audio/errors");
        assert_eq!(
            audio_capture_frame_topic("/booster/audio/capture/7").type_name,
            "booster_msgs::audio::dds_::AudioCaptureFrameTopic_"
        );
    }

    #[test]
    fn capture_frame_topic_name_normalization() {
        assert_eq!(
            capture_frame_topic_name("/booster/audio/capture_stream/1"),
            "rt/booster/audio/capture_stream/1"
        );
        assert_eq!(
            capture_frame_topic_name("rt/booster/audio/capture_stream/1"),
            "rt/booster/audio/capture_stream/1"
        );
        assert_eq!(
            capture_frame_topic_name("booster/audio/capture_stream/1"),
            "rt/booster/audio/capture_stream/1"
        );
    }

    #[test]
    fn capture_frame_conversion() {
        let msg = AudioCaptureFrameTopic {
            header: AudioTopicHeader {
                session_type: 3,
                session_id: 9,
                timestamp_ms: 1234,
                ..AudioTopicHeader::default()
            },
            frame_seq: 5,
            raw_valid: true,
            raw_sample_rate_hz: 16_000,
            raw_channels: 3,
            raw_bits_per_sample: 16,
            raw_frame_samples_per_channel: 2,
            raw_pcm: vec![1, 2, 3, 4, 5, 6],
            ..AudioCaptureFrameTopic::default()
        };
        assert_eq!(
            msg.header.session_type_enum(),
            Some(SessionType::CaptureStream)
        );
        let frame = AudioCaptureFrame::from(msg);
        assert_eq!(frame.frame_seq, 5);
        assert_eq!(frame.timestamp_ms, 1234);
        assert_eq!(frame.raw_format.channels, 3);
        assert_eq!(frame.raw_pcm.len(), 6);
        assert!(!frame.naec_valid);
    }

    #[test]
    fn error_topic_converts_to_audio_error() {
        let msg = AudioErrorTopic {
            ret_code: 7001,
            error_category: 4,
            error_detail: 15,
            ret_msg: "no array".to_owned(),
            ..AudioErrorTopic::default()
        };
        assert_eq!(
            msg.error_category_enum(),
            Some(AudioErrorCategory::LocalizationError)
        );
        let err = AudioError::from(&msg);
        assert_eq!(err.ret_code, AUDIO_ERR_NOT_SUPPORTED);
        assert_eq!(err.detail_enum(), Some(AudioErrorDetail::GetDoaAngle));
    }

    #[test]
    fn audio_client_ignores_robot_name() {
        let client =
            AudioClient::with_options(RpcClientOptions::default().with_robot_name("r1")).unwrap();
        assert!(client.options.robot_name.is_none());
        assert!(!client.is_initialized());
        let a = client.generate_request_id();
        let b = client.generate_request_id();
        assert!(a.starts_with("audio_req_"));
        assert_ne!(a, b);
    }

    #[test]
    fn session_handles_require_init() {
        let client = Arc::new(AudioClient::new().unwrap());
        let player = client.create_player();
        assert_eq!(player.session_id(), None);
        assert_eq!(player.cached_state(), PlayerState::Idle);
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        assert!(matches!(
            rt.block_on(player.start()),
            Err(BoosterError::Validation(_))
        ));

        let recorder = AudioRecorder::attach(Arc::clone(&client), 11);
        let progress = AudioProgressTopic {
            header: AudioTopicHeader {
                session_type: 1,
                session_id: 11,
                ..AudioTopicHeader::default()
            },
            position_bytes: 640,
            duration_bytes: 0,
            state: 2,
        };
        assert!(recorder.apply_progress(&progress));
        assert_eq!(recorder.cached_state(), RecorderState::Recording);
        assert_eq!(recorder.cached_info().captured_bytes, 640);
        // Same id, different session type: not ours.
        assert!(!player.apply_progress(&progress));
    }
}
