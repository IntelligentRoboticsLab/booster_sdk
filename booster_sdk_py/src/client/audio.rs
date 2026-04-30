use std::sync::Arc;

use booster_sdk::client::audio::{
    AudioCaptureStreamInfo, AudioCaptureStreamOptions, AudioCaptureStreamState, AudioClient,
    AudioSourceType, InitCaptureStreamResponse, InitPlayerResponse, InitRecorderResponse,
    PcmFormat, PlayerInfo, PlayerInitOptions, PlayerPriority, PlayerState, RecorderInfo,
    RecorderInitOptions, RecorderState,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{runtime::wait_for_future, startup_wait_from_seconds, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "AudioSourceType", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyAudioSourceType(AudioSourceType);

#[pymethods]
impl PyAudioSourceType {
    #[classattr]
    const PCM_FILE: Self = Self(AudioSourceType::PcmFile);
    #[classattr]
    const WAV_FILE: Self = Self(AudioSourceType::WavFile);
    #[classattr]
    const PCM_STREAM: Self = Self(AudioSourceType::PcmStream);
    #[classattr]
    const MP3_FILE: Self = Self(AudioSourceType::Mp3File);

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyAudioSourceType> for AudioSourceType {
    fn from(value: PyAudioSourceType) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "PlayerPriority", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyPlayerPriority(PlayerPriority);

#[pymethods]
impl PyPlayerPriority {
    #[classattr]
    const LOW: Self = Self(PlayerPriority::Low);
    #[classattr]
    const MEDIUM: Self = Self(PlayerPriority::Medium);
    #[classattr]
    const HIGH: Self = Self(PlayerPriority::High);

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyPlayerPriority> for PlayerPriority {
    fn from(value: PyPlayerPriority) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "PlayerState", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyPlayerState(PlayerState);

#[pymethods]
impl PyPlayerState {
    #[classattr]
    const IDLE: Self = Self(PlayerState::Idle);
    #[classattr]
    const READY: Self = Self(PlayerState::Ready);
    #[classattr]
    const PLAYING: Self = Self(PlayerState::Playing);
    #[classattr]
    const PAUSED: Self = Self(PlayerState::Paused);
    #[classattr]
    const STOPPED: Self = Self(PlayerState::Stopped);
    #[classattr]
    const COMPLETED: Self = Self(PlayerState::Completed);
    #[classattr]
    const ERROR: Self = Self(PlayerState::Error);

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PlayerState> for PyPlayerState {
    fn from(value: PlayerState) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "RecorderState", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyRecorderState(RecorderState);

#[pymethods]
impl PyRecorderState {
    #[classattr]
    const IDLE: Self = Self(RecorderState::Idle);
    #[classattr]
    const READY: Self = Self(RecorderState::Ready);
    #[classattr]
    const RECORDING: Self = Self(RecorderState::Recording);
    #[classattr]
    const PAUSED: Self = Self(RecorderState::Paused);
    #[classattr]
    const STOPPED: Self = Self(RecorderState::Stopped);
    #[classattr]
    const ERROR: Self = Self(RecorderState::Error);

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<RecorderState> for PyRecorderState {
    fn from(value: RecorderState) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "AudioCaptureStreamState", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyAudioCaptureStreamState(AudioCaptureStreamState);

#[pymethods]
impl PyAudioCaptureStreamState {
    #[classattr]
    const IDLE: Self = Self(AudioCaptureStreamState::Idle);
    #[classattr]
    const READY: Self = Self(AudioCaptureStreamState::Ready);
    #[classattr]
    const STREAMING: Self = Self(AudioCaptureStreamState::Streaming);
    #[classattr]
    const PAUSED: Self = Self(AudioCaptureStreamState::Paused);
    #[classattr]
    const STOPPED: Self = Self(AudioCaptureStreamState::Stopped);
    #[classattr]
    const ERROR: Self = Self(AudioCaptureStreamState::Error);

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<AudioCaptureStreamState> for PyAudioCaptureStreamState {
    fn from(value: AudioCaptureStreamState) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "PcmFormat")]
#[derive(Clone, Copy)]
pub struct PyPcmFormat(PcmFormat);

#[pymethods]
impl PyPcmFormat {
    #[new]
    #[pyo3(signature = (sample_rate_hz=16000, channels=1, bits_per_sample=16))]
    fn new(sample_rate_hz: i32, channels: i32, bits_per_sample: i32) -> Self {
        Self(PcmFormat {
            sample_rate_hz,
            channels,
            bits_per_sample,
        })
    }

    #[getter]
    fn sample_rate_hz(&self) -> i32 {
        self.0.sample_rate_hz
    }

    #[getter]
    fn channels(&self) -> i32 {
        self.0.channels
    }

    #[getter]
    fn bits_per_sample(&self) -> i32 {
        self.0.bits_per_sample
    }
}

impl From<PyPcmFormat> for PcmFormat {
    fn from(value: PyPcmFormat) -> Self {
        value.0
    }
}

impl From<PcmFormat> for PyPcmFormat {
    fn from(value: PcmFormat) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "PlayerInitOptions")]
#[derive(Clone)]
pub struct PyPlayerInitOptions(PlayerInitOptions);

#[pymethods]
impl PyPlayerInitOptions {
    #[new]
    #[pyo3(signature = (source_type, source_uri, sample_rate_hz=16000, channels=1, bits_per_sample=16, priority=None))]
    fn new(
        source_type: PyAudioSourceType,
        source_uri: String,
        sample_rate_hz: i32,
        channels: i32,
        bits_per_sample: i32,
        priority: Option<PyPlayerPriority>,
    ) -> Self {
        Self(PlayerInitOptions {
            source_type: source_type.into(),
            source_uri,
            sample_rate_hz,
            channels,
            bits_per_sample,
            priority: priority.map(Into::into).unwrap_or(PlayerPriority::Medium),
        })
    }

    #[staticmethod]
    fn pcm_stream() -> Self {
        Self(PlayerInitOptions::pcm_stream())
    }
}

impl From<PyPlayerInitOptions> for PlayerInitOptions {
    fn from(value: PyPlayerInitOptions) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "RecorderInitOptions")]
#[derive(Clone)]
pub struct PyRecorderInitOptions(RecorderInitOptions);

#[pymethods]
impl PyRecorderInitOptions {
    #[new]
    #[pyo3(signature = (output_path, sample_rate_hz=16000, channels=1, bits_per_sample=16))]
    fn new(output_path: String, sample_rate_hz: i32, channels: i32, bits_per_sample: i32) -> Self {
        Self(RecorderInitOptions {
            output_path,
            sample_rate_hz,
            channels,
            bits_per_sample,
        })
    }
}

impl From<PyRecorderInitOptions> for RecorderInitOptions {
    fn from(value: PyRecorderInitOptions) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "AudioCaptureStreamOptions")]
#[derive(Clone)]
pub struct PyAudioCaptureStreamOptions(AudioCaptureStreamOptions);

#[pymethods]
impl PyAudioCaptureStreamOptions {
    #[new]
    #[pyo3(signature = (enable_raw_pcm=true, enable_naec_pcm=false, requested_raw_format=None))]
    fn new(
        enable_raw_pcm: bool,
        enable_naec_pcm: bool,
        requested_raw_format: Option<PyPcmFormat>,
    ) -> Self {
        Self(AudioCaptureStreamOptions {
            enable_raw_pcm,
            enable_naec_pcm,
            requested_raw_format: requested_raw_format.map(Into::into).unwrap_or_default(),
        })
    }
}

impl From<PyAudioCaptureStreamOptions> for AudioCaptureStreamOptions {
    fn from(value: PyAudioCaptureStreamOptions) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "InitPlayerResponse")]
#[derive(Clone)]
pub struct PyInitPlayerResponse(InitPlayerResponse);

#[pymethods]
impl PyInitPlayerResponse {
    #[getter]
    fn ret_code(&self) -> i32 {
        self.0.ret_code
    }

    #[getter]
    fn ret_msg(&self) -> String {
        self.0.ret_msg.clone()
    }

    #[getter]
    fn session_id(&self) -> i64 {
        self.0.session_id
    }
}

impl From<InitPlayerResponse> for PyInitPlayerResponse {
    fn from(value: InitPlayerResponse) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "InitRecorderResponse")]
#[derive(Clone)]
pub struct PyInitRecorderResponse(InitRecorderResponse);

#[pymethods]
impl PyInitRecorderResponse {
    #[getter]
    fn ret_code(&self) -> i32 {
        self.0.ret_code
    }

    #[getter]
    fn ret_msg(&self) -> String {
        self.0.ret_msg.clone()
    }

    #[getter]
    fn session_id(&self) -> i64 {
        self.0.session_id
    }
}

impl From<InitRecorderResponse> for PyInitRecorderResponse {
    fn from(value: InitRecorderResponse) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "InitCaptureStreamResponse")]
#[derive(Clone)]
pub struct PyInitCaptureStreamResponse(InitCaptureStreamResponse);

#[pymethods]
impl PyInitCaptureStreamResponse {
    #[getter]
    fn ret_code(&self) -> i32 {
        self.0.ret_code
    }

    #[getter]
    fn ret_msg(&self) -> String {
        self.0.ret_msg.clone()
    }

    #[getter]
    fn session_id(&self) -> i64 {
        self.0.session_id
    }

    #[getter]
    fn data_topic_name(&self) -> String {
        self.0.data_topic_name.clone()
    }
}

impl From<InitCaptureStreamResponse> for PyInitCaptureStreamResponse {
    fn from(value: InitCaptureStreamResponse) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "PlayerInfo")]
#[derive(Clone)]
pub struct PyPlayerInfo(PlayerInfo);

#[pymethods]
impl PyPlayerInfo {
    #[getter]
    fn state(&self) -> i32 {
        self.0.state
    }

    fn state_enum(&self) -> Option<PyPlayerState> {
        self.0.state_enum().map(Into::into)
    }

    #[getter]
    fn played_bytes(&self) -> i64 {
        self.0.played_bytes
    }

    #[getter]
    fn total_bytes(&self) -> i64 {
        self.0.total_bytes
    }

    #[getter]
    fn volume(&self) -> f32 {
        self.0.volume
    }
}

impl From<PlayerInfo> for PyPlayerInfo {
    fn from(value: PlayerInfo) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "RecorderInfo")]
#[derive(Clone)]
pub struct PyRecorderInfo(RecorderInfo);

#[pymethods]
impl PyRecorderInfo {
    #[getter]
    fn state(&self) -> i32 {
        self.0.state
    }

    fn state_enum(&self) -> Option<PyRecorderState> {
        self.0.state_enum().map(Into::into)
    }

    #[getter]
    fn captured_bytes(&self) -> i64 {
        self.0.captured_bytes
    }
}

impl From<RecorderInfo> for PyRecorderInfo {
    fn from(value: RecorderInfo) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "AudioCaptureStreamInfo")]
#[derive(Clone)]
pub struct PyAudioCaptureStreamInfo(AudioCaptureStreamInfo);

#[pymethods]
impl PyAudioCaptureStreamInfo {
    #[getter]
    fn state(&self) -> i32 {
        self.0.state
    }

    fn state_enum(&self) -> Option<PyAudioCaptureStreamState> {
        self.0.state_enum().map(Into::into)
    }

    #[getter]
    fn raw_enabled(&self) -> bool {
        self.0.raw_enabled
    }

    #[getter]
    fn naec_enabled(&self) -> bool {
        self.0.naec_enabled
    }

    #[getter]
    fn actual_raw_format(&self) -> PyPcmFormat {
        self.0.actual_raw_format.into()
    }

    #[getter]
    fn actual_naec_format(&self) -> PyPcmFormat {
        self.0.actual_naec_format.into()
    }

    #[getter]
    fn published_frames(&self) -> i64 {
        self.0.published_frames
    }

    #[getter]
    fn dropped_frames(&self) -> i64 {
        self.0.dropped_frames
    }
}

impl From<AudioCaptureStreamInfo> for PyAudioCaptureStreamInfo {
    fn from(value: AudioCaptureStreamInfo) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "AudioClient", unsendable)]
pub struct PyAudioClient {
    client: Arc<AudioClient>,
}

#[pymethods]
impl PyAudioClient {
    #[new]
    #[pyo3(signature = (startup_wait_sec=None))]
    fn new(startup_wait_sec: Option<f64>) -> PyResult<Self> {
        let startup_wait = startup_wait_from_seconds(startup_wait_sec)?;
        let client = match startup_wait {
            Some(wait) => AudioClient::with_startup_wait(wait),
            None => AudioClient::new(),
        }
        .map_err(to_py_err)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    fn init(&self, py: Python<'_>) -> PyResult<String> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.init().await }).map_err(to_py_err)
    }

    fn client_id(&self) -> Option<String> {
        self.client.client_id()
    }

    fn init_player(
        &self,
        py: Python<'_>,
        options: PyPlayerInitOptions,
    ) -> PyResult<PyInitPlayerResponse> {
        let client = Arc::clone(&self.client);
        let options: PlayerInitOptions = options.into();
        wait_for_future(py, async move { client.init_player(&options).await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn start_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.start_player(session_id).await }).map_err(to_py_err)
    }

    fn pause_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.pause_player(session_id).await }).map_err(to_py_err)
    }

    fn stop_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_player(session_id).await }).map_err(to_py_err)
    }

    fn reset_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.reset_player(session_id).await }).map_err(to_py_err)
    }

    fn destroy_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.destroy_player(session_id).await })
            .map_err(to_py_err)
    }

    fn set_player_volume(&self, py: Python<'_>, session_id: i64, volume: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.set_player_volume(session_id, volume).await
        })
        .map_err(to_py_err)
    }

    fn get_player_info(&self, py: Python<'_>, session_id: i64) -> PyResult<PyPlayerInfo> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_player_info(session_id).await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn send_pcm_data(&self, py: Python<'_>, session_id: i64, pcm_bytes: Vec<u8>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.send_pcm_data(session_id, pcm_bytes).await
        })
        .map_err(to_py_err)
    }

    fn init_recorder(
        &self,
        py: Python<'_>,
        options: PyRecorderInitOptions,
    ) -> PyResult<PyInitRecorderResponse> {
        let client = Arc::clone(&self.client);
        let options: RecorderInitOptions = options.into();
        wait_for_future(py, async move { client.init_recorder(&options).await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn start_recorder(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.start_recorder(session_id).await })
            .map_err(to_py_err)
    }

    fn pause_recorder(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.pause_recorder(session_id).await })
            .map_err(to_py_err)
    }

    fn stop_recorder(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_recorder(session_id).await })
            .map_err(to_py_err)
    }

    fn destroy_recorder(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.destroy_recorder(session_id).await })
            .map_err(to_py_err)
    }

    fn get_recorder_info(&self, py: Python<'_>, session_id: i64) -> PyResult<PyRecorderInfo> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.get_recorder_info(session_id).await },
        )
        .map(Into::into)
        .map_err(to_py_err)
    }

    fn get_doa_angle(&self, py: Python<'_>) -> PyResult<i32> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_doa_angle().await }).map_err(to_py_err)
    }

    fn set_system_volume(&self, py: Python<'_>, volume: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.set_system_volume(volume).await })
            .map_err(to_py_err)
    }

    fn get_system_volume(&self, py: Python<'_>) -> PyResult<f32> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_system_volume().await }).map_err(to_py_err)
    }

    fn set_system_mute(&self, py: Python<'_>, mute: bool) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.set_system_mute(mute).await }).map_err(to_py_err)
    }

    fn get_system_mute(&self, py: Python<'_>) -> PyResult<bool> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_system_mute().await }).map_err(to_py_err)
    }

    fn init_capture_stream(
        &self,
        py: Python<'_>,
        options: PyAudioCaptureStreamOptions,
    ) -> PyResult<PyInitCaptureStreamResponse> {
        let client = Arc::clone(&self.client);
        let options: AudioCaptureStreamOptions = options.into();
        wait_for_future(
            py,
            async move { client.init_capture_stream(&options).await },
        )
        .map(Into::into)
        .map_err(to_py_err)
    }

    fn start_capture_stream(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.start_capture_stream(session_id).await },
        )
        .map_err(to_py_err)
    }

    fn pause_capture_stream(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.pause_capture_stream(session_id).await },
        )
        .map_err(to_py_err)
    }

    fn stop_capture_stream(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.stop_capture_stream(session_id).await },
        )
        .map_err(to_py_err)
    }

    fn destroy_capture_stream(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.destroy_capture_stream(session_id).await
        })
        .map_err(to_py_err)
    }

    fn get_capture_stream_info(
        &self,
        py: Python<'_>,
        session_id: i64,
    ) -> PyResult<PyAudioCaptureStreamInfo> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.get_capture_stream_info(session_id).await
        })
        .map(Into::into)
        .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAudioSourceType>()?;
    m.add_class::<PyPlayerPriority>()?;
    m.add_class::<PyPlayerState>()?;
    m.add_class::<PyRecorderState>()?;
    m.add_class::<PyAudioCaptureStreamState>()?;
    m.add_class::<PyPcmFormat>()?;
    m.add_class::<PyPlayerInitOptions>()?;
    m.add_class::<PyRecorderInitOptions>()?;
    m.add_class::<PyAudioCaptureStreamOptions>()?;
    m.add_class::<PyInitPlayerResponse>()?;
    m.add_class::<PyInitRecorderResponse>()?;
    m.add_class::<PyInitCaptureStreamResponse>()?;
    m.add_class::<PyPlayerInfo>()?;
    m.add_class::<PyRecorderInfo>()?;
    m.add_class::<PyAudioCaptureStreamInfo>()?;
    m.add_class::<PyAudioClient>()?;
    Ok(())
}
