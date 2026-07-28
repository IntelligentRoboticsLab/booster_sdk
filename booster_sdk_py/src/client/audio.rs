use std::sync::Arc;

use booster_sdk::client::audio::{
    AudioCaptureStreamInfo, AudioCaptureStreamOptions, AudioCaptureStreamState, AudioClient,
    AudioDeviceBackendAffinity, AudioDeviceDirection, AudioDeviceInfo, AudioDeviceQueryType,
    AudioDeviceTransport, AudioSourceType, BluetoothAudioProfile, BluetoothConnectOptions,
    BluetoothConnectResult, BluetoothDeviceInfo, BluetoothDeviceState, BluetoothMajorClass,
    BluetoothScanOptions, InitCaptureStreamResponse, InitPlayerResponse, InitRecorderResponse,
    PcmFormat, PlayerInfo, PlayerInitOptions, PlayerPriority, PlayerState, RecorderInfo,
    RecorderInitOptions, RecorderState,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{runtime::wait_for_future, startup_wait_from_seconds, to_py_err};

macro_rules! py_int_enum {
    ($py_name:ident, $public_name:literal, $inner:ty, { $($attr:ident => $variant:ident),+ $(,)? }) => {
        #[pyclass(module = "booster_sdk_bindings", name = $public_name, eq)]
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $py_name($inner);

        #[pymethods]
        impl $py_name {
            $(
                #[classattr]
                const $attr: Self = Self(<$inner>::$variant);
            )+

            fn __int__(&self) -> i32 {
                i32::from(self.0)
            }
        }

        impl From<$py_name> for $inner {
            fn from(value: $py_name) -> Self {
                value.0
            }
        }

        impl From<$inner> for $py_name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }
    };
}

py_int_enum!(
    PyAudioDeviceQueryType,
    "AudioDeviceQueryType",
    AudioDeviceQueryType,
    { INPUTS => Inputs, OUTPUTS => Outputs }
);
py_int_enum!(
    PyAudioDeviceDirection,
    "AudioDeviceDirection",
    AudioDeviceDirection,
    { INPUT => Input, OUTPUT => Output }
);
py_int_enum!(
    PyAudioDeviceTransport,
    "AudioDeviceTransport",
    AudioDeviceTransport,
    {
        BUILTIN => Builtin,
        USB => Usb,
        BLUETOOTH => Bluetooth,
        VIRTUAL => Virtual,
        UNKNOWN => Unknown
    }
);
py_int_enum!(
    PyAudioDeviceBackendAffinity,
    "AudioDeviceBackendAffinity",
    AudioDeviceBackendAffinity,
    {
        PULSE => Pulse,
        BOOSTER_AEC_ARRAY => BoosterAecArray,
        ALSA_DIAGNOSTIC => AlsaDiagnostic
    }
);
py_int_enum!(
    PyBluetoothDeviceState,
    "BluetoothDeviceState",
    BluetoothDeviceState,
    {
        UNKNOWN => Unknown,
        PAIRED => Paired,
        CONNECTING => Connecting,
        CONNECTED => Connected,
        DISCONNECTING => Disconnecting,
        FAILED => Failed
    }
);
py_int_enum!(
    PyBluetoothMajorClass,
    "BluetoothMajorClass",
    BluetoothMajorClass,
    {
        AUDIO => Audio,
        PERIPHERAL => Peripheral,
        PHONE => Phone,
        COMPUTER => Computer,
        OTHER => Other
    }
);
py_int_enum!(
    PyBluetoothAudioProfile,
    "BluetoothAudioProfile",
    BluetoothAudioProfile,
    {
        NONE => None,
        A2DP_SINK => A2dpSink,
        A2DP_SOURCE => A2dpSource,
        HFP_HEADSET => HfpHeadset,
        HFP_AG => HfpAg
    }
);

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

#[pyclass(module = "booster_sdk_bindings", name = "AudioDeviceInfo")]
#[derive(Clone)]
pub struct PyAudioDeviceInfo(AudioDeviceInfo);

#[pymethods]
impl PyAudioDeviceInfo {
    #[getter]
    fn device_id(&self) -> String {
        self.0.device_id.clone()
    }
    #[getter]
    fn display_name(&self) -> String {
        self.0.display_name.clone()
    }
    #[getter]
    fn direction(&self) -> PyAudioDeviceDirection {
        self.0.direction.into()
    }
    #[getter]
    fn transport(&self) -> PyAudioDeviceTransport {
        self.0.transport.into()
    }
    #[getter]
    fn backend_affinity(&self) -> PyAudioDeviceBackendAffinity {
        self.0.backend_affinity.into()
    }
    #[getter]
    fn is_available(&self) -> bool {
        self.0.is_available
    }
    #[getter]
    fn is_system_default(&self) -> bool {
        self.0.is_system_default
    }
    #[getter]
    fn supports_input(&self) -> bool {
        self.0.supports_input
    }
    #[getter]
    fn supports_output(&self) -> bool {
        self.0.supports_output
    }
    #[getter]
    fn provider_name(&self) -> String {
        self.0.provider_name.clone()
    }
    #[getter]
    fn native_id(&self) -> String {
        self.0.native_id.clone()
    }
    #[getter]
    fn metadata_json(&self) -> String {
        self.0.metadata_json.clone()
    }
}

impl From<AudioDeviceInfo> for PyAudioDeviceInfo {
    fn from(value: AudioDeviceInfo) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "BluetoothDeviceInfo")]
#[derive(Clone)]
pub struct PyBluetoothDeviceInfo(BluetoothDeviceInfo);

#[pymethods]
impl PyBluetoothDeviceInfo {
    #[getter]
    fn address(&self) -> String {
        self.0.address.clone()
    }
    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }
    #[getter]
    fn rssi(&self) -> i16 {
        self.0.rssi
    }
    #[getter]
    fn state(&self) -> PyBluetoothDeviceState {
        self.0.state.into()
    }
    #[getter]
    fn major_class(&self) -> PyBluetoothMajorClass {
        self.0.major_class.into()
    }
    #[getter]
    fn paired(&self) -> bool {
        self.0.paired
    }
    #[getter]
    fn trusted(&self) -> bool {
        self.0.trusted
    }
    #[getter]
    fn connected(&self) -> bool {
        self.0.connected
    }
    #[getter]
    fn is_audio_sink(&self) -> bool {
        self.0.is_audio_sink
    }
    #[getter]
    fn is_audio_source(&self) -> bool {
        self.0.is_audio_source
    }
    #[getter]
    fn is_hfp_capable(&self) -> bool {
        self.0.is_hfp_capable
    }
    #[getter]
    fn connected_profiles(&self) -> Vec<PyBluetoothAudioProfile> {
        self.0
            .connected_profiles
            .iter()
            .copied()
            .map(Into::into)
            .collect()
    }
    #[getter]
    fn linked_pulse_sink_id(&self) -> String {
        self.0.linked_pulse_sink_id.clone()
    }
    #[getter]
    fn linked_pulse_source_id(&self) -> String {
        self.0.linked_pulse_source_id.clone()
    }
    #[getter]
    fn last_seen_ms(&self) -> i64 {
        self.0.last_seen_ms
    }
}

impl From<BluetoothDeviceInfo> for PyBluetoothDeviceInfo {
    fn from(value: BluetoothDeviceInfo) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "BluetoothScanOptions")]
#[derive(Clone, Copy)]
pub struct PyBluetoothScanOptions(BluetoothScanOptions);

#[pymethods]
impl PyBluetoothScanOptions {
    #[new]
    #[pyo3(signature = (timeout_ms=30000, audio_only=true))]
    fn new(timeout_ms: i32, audio_only: bool) -> Self {
        Self(BluetoothScanOptions {
            timeout_ms,
            audio_only,
        })
    }
}

impl From<PyBluetoothScanOptions> for BluetoothScanOptions {
    fn from(value: PyBluetoothScanOptions) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "BluetoothConnectOptions")]
#[derive(Clone)]
pub struct PyBluetoothConnectOptions(BluetoothConnectOptions);

#[pymethods]
impl PyBluetoothConnectOptions {
    #[new]
    #[pyo3(signature = (address, auto_pair=true, make_default=true, preferred_profile=None, timeout_ms=15000))]
    fn new(
        address: String,
        auto_pair: bool,
        make_default: bool,
        preferred_profile: Option<PyBluetoothAudioProfile>,
        timeout_ms: i32,
    ) -> Self {
        Self(BluetoothConnectOptions {
            address,
            auto_pair,
            make_default,
            preferred_profile: preferred_profile
                .map(Into::into)
                .unwrap_or(BluetoothAudioProfile::None),
            timeout_ms,
        })
    }
}

impl From<PyBluetoothConnectOptions> for BluetoothConnectOptions {
    fn from(value: PyBluetoothConnectOptions) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "BluetoothConnectResult")]
#[derive(Clone)]
pub struct PyBluetoothConnectResult(BluetoothConnectResult);

#[pymethods]
impl PyBluetoothConnectResult {
    #[getter]
    fn device(&self) -> PyBluetoothDeviceInfo {
        self.0.device.clone().into()
    }
    #[getter]
    fn pulse_sink_id(&self) -> String {
        self.0.pulse_sink_id.clone()
    }
    #[getter]
    fn pulse_source_id(&self) -> String {
        self.0.pulse_source_id.clone()
    }
    #[getter]
    fn active_profile(&self) -> PyBluetoothAudioProfile {
        self.0.active_profile.into()
    }
    #[getter]
    fn pulse_endpoint_ready(&self) -> bool {
        self.0.pulse_endpoint_ready
    }
    #[getter]
    fn default_sink_applied(&self) -> bool {
        self.0.default_sink_applied
    }
    #[getter]
    fn default_source_applied(&self) -> bool {
        self.0.default_source_applied
    }
    #[getter]
    fn default_route_error_code(&self) -> i32 {
        self.0.default_route_error_code
    }
    #[getter]
    fn default_route_error_msg(&self) -> String {
        self.0.default_route_error_msg.clone()
    }
}

impl From<BluetoothConnectResult> for PyBluetoothConnectResult {
    fn from(value: BluetoothConnectResult) -> Self {
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

    fn get_devices(
        &self,
        py: Python<'_>,
        query_type: PyAudioDeviceQueryType,
    ) -> PyResult<Vec<PyAudioDeviceInfo>> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.get_devices(query_type.into()).await },
        )
        .map(|devices| devices.into_iter().map(Into::into).collect())
        .map_err(to_py_err)
    }

    #[pyo3(signature = (options=None))]
    fn start_bluetooth_scan(
        &self,
        py: Python<'_>,
        options: Option<PyBluetoothScanOptions>,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let options: BluetoothScanOptions = options.map(Into::into).unwrap_or_default();
        wait_for_future(
            py,
            async move { client.start_bluetooth_scan(&options).await },
        )
        .map_err(to_py_err)
    }

    fn stop_bluetooth_scan(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_bluetooth_scan().await }).map_err(to_py_err)
    }

    #[pyo3(signature = (include_unpaired=true))]
    fn get_bluetooth_devices(
        &self,
        py: Python<'_>,
        include_unpaired: bool,
    ) -> PyResult<Vec<PyBluetoothDeviceInfo>> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.get_bluetooth_devices(include_unpaired).await
        })
        .map(|devices| devices.into_iter().map(Into::into).collect())
        .map_err(to_py_err)
    }

    fn connect_bluetooth_device(
        &self,
        py: Python<'_>,
        options: PyBluetoothConnectOptions,
    ) -> PyResult<PyBluetoothConnectResult> {
        let client = Arc::clone(&self.client);
        let options: BluetoothConnectOptions = options.into();
        wait_for_future(py, async move {
            client.connect_bluetooth_device(&options).await
        })
        .map(Into::into)
        .map_err(to_py_err)
    }

    fn disconnect_bluetooth_device(&self, py: Python<'_>, address: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.disconnect_bluetooth_device(address).await
        })
        .map_err(to_py_err)
    }

    fn forget_bluetooth_device(&self, py: Python<'_>, address: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.forget_bluetooth_device(address).await },
        )
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
    m.add_class::<PyAudioDeviceQueryType>()?;
    m.add_class::<PyAudioDeviceDirection>()?;
    m.add_class::<PyAudioDeviceTransport>()?;
    m.add_class::<PyAudioDeviceBackendAffinity>()?;
    m.add_class::<PyBluetoothDeviceState>()?;
    m.add_class::<PyBluetoothMajorClass>()?;
    m.add_class::<PyBluetoothAudioProfile>()?;
    m.add_class::<PyAudioDeviceInfo>()?;
    m.add_class::<PyBluetoothDeviceInfo>()?;
    m.add_class::<PyBluetoothScanOptions>()?;
    m.add_class::<PyBluetoothConnectOptions>()?;
    m.add_class::<PyBluetoothConnectResult>()?;
    m.add_class::<PyAudioClient>()?;
    Ok(())
}
