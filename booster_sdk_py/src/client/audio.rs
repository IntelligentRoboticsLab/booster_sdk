use std::{future::Future, sync::Arc};

use booster_sdk::{
    client::audio::{
        AUDIO_ERR_NOT_SUPPORTED, AUDIO_ERROR_TOPIC, AUDIO_EVENT_TOPIC, AUDIO_PROGRESS_TOPIC,
        AudioCaptureFrame, AudioCaptureFrameTopic, AudioCaptureStream, AudioCaptureStreamInfo,
        AudioCaptureStreamOptions, AudioCaptureStreamState, AudioClient,
        AudioDeviceBackendAffinity, AudioDeviceDirection, AudioDeviceInfo, AudioDeviceQueryType,
        AudioDeviceTransport, AudioError, AudioErrorCategory, AudioErrorCode, AudioErrorDetail,
        AudioErrorTopic, AudioEventCode, AudioEventTopic, AudioLocalizer, AudioPlayer,
        AudioProgressTopic, AudioRecorder, AudioSourceType, AudioTopicHeader,
        BluetoothAudioProfile, BluetoothConnectOptions, BluetoothConnectResult,
        BluetoothDeviceInfo, BluetoothDeviceState, BluetoothMajorClass, BluetoothScanOptions,
        BluetoothScanState, InitCaptureStreamResponse, InitPlayerResponse, InitRecorderResponse,
        PcmFormat, PlayerInfo, PlayerInitOptions, PlayerPriority, PlayerState, RecorderInfo,
        RecorderInitOptions, RecorderState, SessionType, capture_frame_topic_name,
    },
    dds::BluetoothEvent,
};
use pyo3::{
    Bound,
    prelude::*,
    types::{PyBytes, PyModule},
};

use crate::{
    macros::{py_int_enum, py_param_class, py_readonly_class},
    options::rpc_options,
    runtime::wait_for_future,
    subscription::PySubscription,
    to_py_err,
};

/// Block on an SDK future and convert its error.
fn run<T, F>(py: Python<'_>, future: F) -> PyResult<T>
where
    F: Future<Output = booster_sdk::types::Result<T>> + Send + 'static,
    T: Send + 'static,
{
    wait_for_future(py, future).map_err(to_py_err)
}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

py_int_enum!(PyAudioSourceType, "AudioSourceType", AudioSourceType, i32, {
    PCM_FILE => PcmFile,
    WAV_FILE => WavFile,
    PCM_STREAM => PcmStream,
    MP3_FILE => Mp3File,
});
py_int_enum!(PyPlayerPriority, "PlayerPriority", PlayerPriority, i32, {
    LOW => Low,
    MEDIUM => Medium,
    HIGH => High,
});
py_int_enum!(PyPlayerState, "PlayerState", PlayerState, i32, {
    IDLE => Idle,
    READY => Ready,
    PLAYING => Playing,
    PAUSED => Paused,
    STOPPED => Stopped,
    COMPLETED => Completed,
    ERROR => Error,
});
py_int_enum!(PyRecorderState, "RecorderState", RecorderState, i32, {
    IDLE => Idle,
    READY => Ready,
    RECORDING => Recording,
    PAUSED => Paused,
    STOPPED => Stopped,
    ERROR => Error,
});
py_int_enum!(PyAudioCaptureStreamState, "AudioCaptureStreamState", AudioCaptureStreamState, i32, {
    IDLE => Idle,
    READY => Ready,
    STREAMING => Streaming,
    PAUSED => Paused,
    STOPPED => Stopped,
    ERROR => Error,
});
py_int_enum!(PyAudioDeviceQueryType, "AudioDeviceQueryType", AudioDeviceQueryType, i32, {
    INPUTS => Inputs,
    OUTPUTS => Outputs,
});
py_int_enum!(PyAudioDeviceDirection, "AudioDeviceDirection", AudioDeviceDirection, i32, {
    INPUT => Input,
    OUTPUT => Output,
});
py_int_enum!(PyAudioDeviceTransport, "AudioDeviceTransport", AudioDeviceTransport, i32, {
    BUILTIN => Builtin,
    USB => Usb,
    BLUETOOTH => Bluetooth,
    VIRTUAL => Virtual,
    UNKNOWN => Unknown,
});
py_int_enum!(
    PyAudioDeviceBackendAffinity,
    "AudioDeviceBackendAffinity",
    AudioDeviceBackendAffinity,
    i32,
    {
        PULSE => Pulse,
        BOOSTER_AEC_ARRAY => BoosterAecArray,
        ALSA_DIAGNOSTIC => AlsaDiagnostic,
    }
);
py_int_enum!(PyBluetoothScanState, "BluetoothScanState", BluetoothScanState, i32, {
    IDLE => Idle,
    SCANNING => Scanning,
});
py_int_enum!(PyBluetoothDeviceState, "BluetoothDeviceState", BluetoothDeviceState, i32, {
    UNKNOWN => Unknown,
    PAIRED => Paired,
    CONNECTING => Connecting,
    CONNECTED => Connected,
    DISCONNECTING => Disconnecting,
    FAILED => Failed,
});
py_int_enum!(PyBluetoothMajorClass, "BluetoothMajorClass", BluetoothMajorClass, i32, {
    AUDIO => Audio,
    PERIPHERAL => Peripheral,
    PHONE => Phone,
    COMPUTER => Computer,
    OTHER => Other,
});
py_int_enum!(PyBluetoothAudioProfile, "BluetoothAudioProfile", BluetoothAudioProfile, i32, {
    NONE => None,
    A2DP_SINK => A2dpSink,
    A2DP_SOURCE => A2dpSource,
    HFP_HEADSET => HfpHeadset,
    HFP_AG => HfpAg,
});
py_int_enum!(PySessionType, "SessionType", SessionType, i32, {
    PLAYER => Player,
    RECORDER => Recorder,
    LOCALIZER => Localizer,
    CAPTURE_STREAM => CaptureStream,
});
py_int_enum!(PyAudioErrorCode, "AudioErrorCode", AudioErrorCode, i32, {
    SERVICE_DEGRADED => ServiceDegraded,
    INVALID_STATE => InvalidState,
    DEVICE_BUSY => DeviceBusy,
    NOT_SUPPORTED => NotSupported,
    LOCALIZATION_UNAVAILABLE => LocalizationUnavailable,
    ALSA_DEVICE_NOT_FOUND => AlsaDeviceNotFound,
    ALGO_INIT_FAILED => AlgoInitFailed,
    ALGO_PROCESS_FAILED => AlgoProcessFailed,
    SYSTEM_VOLUME_CTRL_FAILED => SystemVolumeCtrlFailed,
    SYSTEM_MUTE_CTRL_FAILED => SystemMuteCtrlFailed,
    MEDIA_DECODE_FAILED => MediaDecodeFailed,
    MEDIA_ENCODE_FAILED => MediaEncodeFailed,
    MEDIA_FORMAT_UNSUPPORTED => MediaFormatUnsupported,
});
py_int_enum!(PyAudioEventCode, "AudioEventCode", AudioEventCode, i32, {
    PLAYER_INIT_SUCCEEDED => PlayerInitSucceeded,
    PLAYER_START_SUCCEEDED => PlayerStartSucceeded,
    PLAYER_PAUSE_SUCCEEDED => PlayerPauseSucceeded,
    PLAYER_STOP_SUCCEEDED => PlayerStopSucceeded,
    PLAYER_RESET_SUCCEEDED => PlayerResetSucceeded,
    PLAYER_DESTROY_SUCCEEDED => PlayerDestroySucceeded,
    RECORDER_INIT_SUCCEEDED => RecorderInitSucceeded,
    RECORDER_START_SUCCEEDED => RecorderStartSucceeded,
    RECORDER_PAUSE_SUCCEEDED => RecorderPauseSucceeded,
    RECORDER_STOP_SUCCEEDED => RecorderStopSucceeded,
    RECORDER_DESTROY_SUCCEEDED => RecorderDestroySucceeded,
    PLAYER_TERMINAL_STATE_CHANGED => PlayerTerminalStateChanged,
    SYSTEM_VOLUME_SET_SUCCEEDED => SystemVolumeSetSucceeded,
    SYSTEM_MUTE_SET_SUCCEEDED => SystemMuteSetSucceeded,
    CAPTURE_STREAM_INIT_SUCCEEDED => CaptureStreamInitSucceeded,
    CAPTURE_STREAM_START_SUCCEEDED => CaptureStreamStartSucceeded,
    CAPTURE_STREAM_PAUSE_SUCCEEDED => CaptureStreamPauseSucceeded,
    CAPTURE_STREAM_STOP_SUCCEEDED => CaptureStreamStopSucceeded,
    CAPTURE_STREAM_DESTROY_SUCCEEDED => CaptureStreamDestroySucceeded,
});
py_int_enum!(PyAudioErrorCategory, "AudioErrorCategory", AudioErrorCategory, i32, {
    RPC_ERROR => RpcError,
    PLAYBACK_ERROR => PlaybackError,
    RECORD_ERROR => RecordError,
    CAPTURE_STREAM_RUNTIME_ERROR => CaptureStreamRuntimeError,
    LOCALIZATION_ERROR => LocalizationError,
    SYSTEM_VOLUME_ERROR => SystemVolumeError,
    SYSTEM_MUTE_ERROR => SystemMuteError,
    INTERNAL_ERROR => InternalError,
});
py_int_enum!(PyAudioErrorDetail, "AudioErrorDetail", AudioErrorDetail, i32, {
    NONE => None,
    INIT_PLAYER => InitPlayer,
    START_PLAYER => StartPlayer,
    PAUSE_PLAYER => PausePlayer,
    STOP_PLAYER => StopPlayer,
    RESET_PLAYER => ResetPlayer,
    DESTROY_PLAYER => DestroyPlayer,
    GET_PLAYER_INFO => GetPlayerInfo,
    SEND_PCM_DATA => SendPcmData,
    INIT_RECORDER => InitRecorder,
    START_RECORDER => StartRecorder,
    PAUSE_RECORDER => PauseRecorder,
    STOP_RECORDER => StopRecorder,
    DESTROY_RECORDER => DestroyRecorder,
    GET_RECORDER_INFO => GetRecorderInfo,
    GET_DOA_ANGLE => GetDoaAngle,
    SET_SYSTEM_VOLUME => SetSystemVolume,
    GET_SYSTEM_VOLUME => GetSystemVolume,
    SET_SYSTEM_MUTE => SetSystemMute,
    GET_SYSTEM_MUTE => GetSystemMute,
    INIT_CAPTURE_STREAM => InitCaptureStream,
    START_CAPTURE_STREAM => StartCaptureStream,
    PAUSE_CAPTURE_STREAM => PauseCaptureStream,
    STOP_CAPTURE_STREAM => StopCaptureStream,
    DESTROY_CAPTURE_STREAM => DestroyCaptureStream,
    GET_CAPTURE_STREAM_INFO => GetCaptureStreamInfo,
});

// ---------------------------------------------------------------------------
// Options (mutable)
// ---------------------------------------------------------------------------

py_param_class!(PyPcmFormat, "PcmFormat", PcmFormat, {
    sample_rate_hz: i32 = 16000,
    channels: i32 = 1,
    bits_per_sample: i32 = 16,
});

py_param_class!(
    PyPlayerInitOptions,
    "PlayerInitOptions",
    PlayerInitOptions,
    {
        source_type: PyAudioSourceType = PyAudioSourceType(AudioSourceType::PcmFile),
        source_uri: String = String::new(),
        sample_rate_hz: i32 = 16000,
        channels: i32 = 1,
        bits_per_sample: i32 = 16,
        priority: PyPlayerPriority = PyPlayerPriority(PlayerPriority::Medium),
    },
    {
        /// Options for a PCM stream player fed through `send_pcm_data`.
        #[staticmethod]
        fn pcm_stream() -> Self {
            Self::from_rust(PlayerInitOptions::pcm_stream())
        }
    }
);

py_param_class!(PyRecorderInitOptions, "RecorderInitOptions", RecorderInitOptions, {
    output_path: String = String::new(),
    sample_rate_hz: i32 = 16000,
    channels: i32 = 1,
    bits_per_sample: i32 = 16,
});

py_param_class!(
    PyAudioCaptureStreamOptions,
    "AudioCaptureStreamOptions",
    AudioCaptureStreamOptions,
    {
        enable_raw_pcm: bool = true,
        enable_naec_pcm: bool = false,
        requested_raw_format: PyPcmFormat =
            PyPcmFormat::from_rust(AudioCaptureStreamOptions::default().requested_raw_format),
    }
);

py_param_class!(PyBluetoothScanOptions, "BluetoothScanOptions", BluetoothScanOptions, {
    timeout_ms: i32 = 30000,
    audio_only: bool = true,
});

py_param_class!(
    PyBluetoothConnectOptions,
    "BluetoothConnectOptions",
    BluetoothConnectOptions,
    {
        address: String = String::new(),
        auto_pair: bool = true,
        make_default: bool = true,
        preferred_profile: PyBluetoothAudioProfile =
            PyBluetoothAudioProfile(BluetoothAudioProfile::None),
        timeout_ms: i32 = 15000,
    }
);

py_param_class!(
    PyAudioError,
    "AudioError",
    AudioError,
    {
        ret_code: i32 = 0,
        ret_msg: String = String::new(),
        error_category: i32 = 0,
        error_detail: i32 = 0,
    },
    {
        fn category_enum(&self) -> Option<PyAudioErrorCategory> {
            self.to_rust().category_enum().map(Into::into)
        }

        fn detail_enum(&self) -> Option<PyAudioErrorDetail> {
            self.to_rust().detail_enum().map(Into::into)
        }
    }
);

// ---------------------------------------------------------------------------
// Responses (read-only)
// ---------------------------------------------------------------------------

py_readonly_class!(PyInitPlayerResponse, "InitPlayerResponse", InitPlayerResponse, {
    ret_code: i32,
    ret_msg: String,
    session_id: i64,
});

py_readonly_class!(PyInitRecorderResponse, "InitRecorderResponse", InitRecorderResponse, {
    ret_code: i32,
    ret_msg: String,
    session_id: i64,
});

py_readonly_class!(
    PyInitCaptureStreamResponse,
    "InitCaptureStreamResponse",
    InitCaptureStreamResponse,
    {
        ret_code: i32,
        ret_msg: String,
        session_id: i64,
        data_topic_name: String,
    }
);

py_readonly_class!(PyPlayerInfo, "PlayerInfo", PlayerInfo, {
    state: i32,
    played_bytes: i64,
    total_bytes: i64,
    volume: f32,
}, {
    fn state_enum(&self) -> Option<PyPlayerState> {
        self.0.state_enum().map(Into::into)
    }
});

py_readonly_class!(PyRecorderInfo, "RecorderInfo", RecorderInfo, {
    state: i32,
    captured_bytes: i64,
}, {
    fn state_enum(&self) -> Option<PyRecorderState> {
        self.0.state_enum().map(Into::into)
    }
});

py_readonly_class!(PyAudioCaptureStreamInfo, "AudioCaptureStreamInfo", AudioCaptureStreamInfo, {
    state: i32,
    raw_enabled: bool,
    naec_enabled: bool,
    actual_raw_format: PyPcmFormat,
    actual_naec_format: PyPcmFormat,
    published_frames: i64,
    dropped_frames: i64,
}, {
    fn state_enum(&self) -> Option<PyAudioCaptureStreamState> {
        self.0.state_enum().map(Into::into)
    }
});

py_readonly_class!(PyAudioDeviceInfo, "AudioDeviceInfo", AudioDeviceInfo, {
    device_id: String,
    display_name: String,
    direction: PyAudioDeviceDirection,
    transport: PyAudioDeviceTransport,
    backend_affinity: PyAudioDeviceBackendAffinity,
    is_available: bool,
    is_system_default: bool,
    supports_input: bool,
    supports_output: bool,
    provider_name: String,
    native_id: String,
    metadata_json: String,
});

py_readonly_class!(PyBluetoothDeviceInfo, "BluetoothDeviceInfo", BluetoothDeviceInfo, {
    address: String,
    name: String,
    rssi: i16,
    state: PyBluetoothDeviceState,
    major_class: PyBluetoothMajorClass,
    paired: bool,
    trusted: bool,
    connected: bool,
    is_audio_sink: bool,
    is_audio_source: bool,
    is_hfp_capable: bool,
    linked_pulse_sink_id: String,
    linked_pulse_source_id: String,
    last_seen_ms: i64,
}, {
    #[getter]
    fn connected_profiles(&self) -> Vec<PyBluetoothAudioProfile> {
        self.0
            .connected_profiles
            .iter()
            .copied()
            .map(Into::into)
            .collect()
    }
});

py_readonly_class!(PyBluetoothConnectResult, "BluetoothConnectResult", BluetoothConnectResult, {
    device: PyBluetoothDeviceInfo,
    pulse_sink_id: String,
    pulse_source_id: String,
    active_profile: PyBluetoothAudioProfile,
    pulse_endpoint_ready: bool,
    default_sink_applied: bool,
    default_source_applied: bool,
    default_route_error_code: i32,
    default_route_error_msg: String,
});

// ---------------------------------------------------------------------------
// Topic messages
// ---------------------------------------------------------------------------

py_readonly_class!(PyAudioTopicHeader, "AudioTopicHeader", AudioTopicHeader, {
    client_id: String,
    request_id: String,
    session_type: u32,
    session_id: i64,
    timestamp_ms: i64,
}, {
    fn session_type_enum(&self) -> Option<PySessionType> {
        self.0.session_type_enum().map(Into::into)
    }
});

py_readonly_class!(PyAudioProgressTopic, "AudioProgressTopic", AudioProgressTopic, {
    header: PyAudioTopicHeader,
    position_bytes: i64,
    duration_bytes: i64,
    state: i32,
});

py_readonly_class!(PyAudioEventTopic, "AudioEventTopic", AudioEventTopic, {
    header: PyAudioTopicHeader,
    event_code: u32,
    state: i32,
    detail: String,
}, {
    fn event_code_enum(&self) -> Option<PyAudioEventCode> {
        self.0.event_code_enum().map(Into::into)
    }
});

py_readonly_class!(PyAudioErrorTopic, "AudioErrorTopic", AudioErrorTopic, {
    header: PyAudioTopicHeader,
    ret_code: i32,
    error_category: u32,
    error_detail: u32,
    ret_msg: String,
}, {
    fn error_category_enum(&self) -> Option<PyAudioErrorCategory> {
        self.0.error_category_enum().map(Into::into)
    }

    fn error_detail_enum(&self) -> Option<PyAudioErrorDetail> {
        self.0.error_detail_enum().map(Into::into)
    }

    /// Convert to the C++-style `AudioError` value.
    fn to_audio_error(&self) -> PyAudioError {
        PyAudioError::from_rust(AudioError::from(&self.0))
    }
});

py_readonly_class!(PyBluetoothEvent, "BluetoothEvent", BluetoothEvent, {
    timestamp_ms: i64,
    event_type: String,
    address: String,
    payload_json: String,
    error_code: i32,
});

fn pcm_to_bytes<'py>(py: Python<'py>, samples: &[i16]) -> Bound<'py, PyBytes> {
    let bytes: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    PyBytes::new(py, &bytes)
}

py_readonly_class!(PyAudioCaptureFrame, "AudioCaptureFrame", AudioCaptureFrame, {
    frame_seq: i64,
    timestamp_ms: i64,
    raw_valid: bool,
    raw_format: PyPcmFormat,
    raw_frame_samples_per_channel: i32,
    raw_pcm: Vec<i16>,
    naec_valid: bool,
    naec_format: PyPcmFormat,
    naec_frame_samples_per_channel: i32,
    naec_pcm: Vec<i16>,
}, {
    /// Raw PCM as little-endian 16-bit bytes.
    fn raw_pcm_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        pcm_to_bytes(py, &self.0.raw_pcm)
    }

    /// NAEC PCM as little-endian 16-bit bytes.
    fn naec_pcm_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        pcm_to_bytes(py, &self.0.naec_pcm)
    }
});

py_readonly_class!(PyAudioCaptureFrameTopic, "AudioCaptureFrameTopic", AudioCaptureFrameTopic, {
    header: PyAudioTopicHeader,
    frame_seq: i64,
    raw_valid: bool,
    raw_sample_rate_hz: i32,
    raw_channels: i32,
    raw_bits_per_sample: i32,
    raw_frame_samples_per_channel: i32,
    raw_pcm: Vec<i16>,
    naec_valid: bool,
    naec_sample_rate_hz: i32,
    naec_channels: i32,
    naec_bits_per_sample: i32,
    naec_frame_samples_per_channel: i32,
    naec_pcm: Vec<i16>,
}, {
    /// Raw PCM as little-endian 16-bit bytes.
    fn raw_pcm_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        pcm_to_bytes(py, &self.0.raw_pcm)
    }

    /// NAEC PCM as little-endian 16-bit bytes.
    fn naec_pcm_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        pcm_to_bytes(py, &self.0.naec_pcm)
    }

    /// Convert to the C++-style `AudioCaptureFrame`.
    fn to_frame(&self) -> PyAudioCaptureFrame {
        PyAudioCaptureFrame(AudioCaptureFrame::from(self.0.clone()))
    }
});

// ---------------------------------------------------------------------------
// Session handles
// ---------------------------------------------------------------------------

/// One playback session (C++ `AudioPlayer`). Dropping it does not destroy
/// the service-side session; call `destroy()`.
#[pyclass(module = "booster_sdk_bindings", name = "AudioPlayer", unsendable)]
pub struct PyAudioPlayer {
    inner: Arc<AudioPlayer>,
}

#[pymethods]
impl PyAudioPlayer {
    #[getter]
    fn session_id(&self) -> Option<i64> {
        self.inner.session_id()
    }

    fn init(&self, py: Python<'_>, options: PyPlayerInitOptions) -> PyResult<i64> {
        let inner = Arc::clone(&self.inner);
        let options = options.to_rust();
        run(py, async move { inner.init(&options).await })
    }

    fn start(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.start().await })
    }

    fn pause(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.pause().await })
    }

    fn stop(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.stop().await })
    }

    fn reset(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.reset().await })
    }

    fn destroy(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.destroy().await })
    }

    fn release(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.release().await })
    }

    fn set_volume(&self, py: Python<'_>, volume: f32) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.set_volume(volume).await })
    }

    fn set_preferred_device(&self, py: Python<'_>, device_id: String) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(
            py,
            async move { inner.set_preferred_device(device_id).await },
        )
    }

    fn get_info(&self, py: Python<'_>) -> PyResult<PyPlayerInfo> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.get_info().await }).map(Into::into)
    }

    /// Push raw PCM bytes to a `PCM_STREAM` player.
    fn push_pcm_stream(&self, py: Python<'_>, data: Vec<u8>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.push_pcm_stream(&data).await })
    }

    fn cached_info(&self) -> PyPlayerInfo {
        self.inner.cached_info().into()
    }

    fn cached_state(&self) -> PyPlayerState {
        self.inner.cached_state().into()
    }

    fn owns(&self, header: PyRef<'_, PyAudioTopicHeader>) -> bool {
        self.inner.owns(&header.0)
    }

    fn apply_progress(&self, progress: PyRef<'_, PyAudioProgressTopic>) -> bool {
        self.inner.apply_progress(&progress.0)
    }

    fn apply_event(&self, event: PyRef<'_, PyAudioEventTopic>) -> bool {
        self.inner.apply_event(&event.0)
    }
}

/// One recording session (C++ `AudioRecorder`).
#[pyclass(module = "booster_sdk_bindings", name = "AudioRecorder", unsendable)]
pub struct PyAudioRecorder {
    inner: Arc<AudioRecorder>,
}

#[pymethods]
impl PyAudioRecorder {
    #[getter]
    fn session_id(&self) -> Option<i64> {
        self.inner.session_id()
    }

    fn init(&self, py: Python<'_>, options: PyRecorderInitOptions) -> PyResult<i64> {
        let inner = Arc::clone(&self.inner);
        let options = options.to_rust();
        run(py, async move { inner.init(&options).await })
    }

    fn start(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.start().await })
    }

    fn pause(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.pause().await })
    }

    fn stop(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.stop().await })
    }

    fn destroy(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.destroy().await })
    }

    fn release(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.release().await })
    }

    fn set_preferred_device(&self, py: Python<'_>, device_id: String) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(
            py,
            async move { inner.set_preferred_device(device_id).await },
        )
    }

    fn get_info(&self, py: Python<'_>) -> PyResult<PyRecorderInfo> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.get_info().await }).map(Into::into)
    }

    fn cached_info(&self) -> PyRecorderInfo {
        self.inner.cached_info().into()
    }

    fn cached_state(&self) -> PyRecorderState {
        self.inner.cached_state().into()
    }

    fn owns(&self, header: PyRef<'_, PyAudioTopicHeader>) -> bool {
        self.inner.owns(&header.0)
    }

    fn apply_progress(&self, progress: PyRef<'_, PyAudioProgressTopic>) -> bool {
        self.inner.apply_progress(&progress.0)
    }

    fn apply_event(&self, event: PyRef<'_, PyAudioEventTopic>) -> bool {
        self.inner.apply_event(&event.0)
    }
}

/// Direction-of-arrival localizer (C++ `AudioLocalizer`).
#[pyclass(module = "booster_sdk_bindings", name = "AudioLocalizer", unsendable)]
pub struct PyAudioLocalizer {
    inner: Arc<AudioLocalizer>,
}

#[pymethods]
impl PyAudioLocalizer {
    /// Current DOA angle in degrees.
    fn get_doa_angle(&self, py: Python<'_>) -> PyResult<i32> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.get_doa_angle().await })
    }
}

/// Raw/NAEC microphone PCM stream (C++ `AudioCaptureStream`).
#[pyclass(
    module = "booster_sdk_bindings",
    name = "AudioCaptureStream",
    unsendable
)]
pub struct PyAudioCaptureStream {
    inner: Arc<AudioCaptureStream>,
}

#[pymethods]
impl PyAudioCaptureStream {
    #[getter]
    fn session_id(&self) -> Option<i64> {
        self.inner.session_id()
    }

    #[getter]
    fn data_topic_name(&self) -> String {
        self.inner.data_topic_name()
    }

    #[pyo3(signature = (options=None))]
    fn init(
        &self,
        py: Python<'_>,
        options: Option<PyAudioCaptureStreamOptions>,
    ) -> PyResult<PyInitCaptureStreamResponse> {
        let inner = Arc::clone(&self.inner);
        let options = options.unwrap_or_default().to_rust();
        run(py, async move { inner.init(&options).await }).map(Into::into)
    }

    fn start(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.start().await })
    }

    fn pause(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.pause().await })
    }

    fn stop(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.stop().await })
    }

    fn destroy(&self, py: Python<'_>) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.destroy().await })
    }

    fn set_preferred_device(&self, py: Python<'_>, device_id: String) -> PyResult<()> {
        let inner = Arc::clone(&self.inner);
        run(
            py,
            async move { inner.set_preferred_device(device_id).await },
        )
    }

    fn get_info(&self, py: Python<'_>) -> PyResult<PyAudioCaptureStreamInfo> {
        let inner = Arc::clone(&self.inner);
        run(py, async move { inner.get_info().await }).map(Into::into)
    }

    fn cached_info(&self) -> PyAudioCaptureStreamInfo {
        self.inner.cached_info().into()
    }

    fn cached_state(&self) -> PyAudioCaptureStreamState {
        self.inner.cached_state().into()
    }

    fn owns(&self, header: PyRef<'_, PyAudioTopicHeader>) -> bool {
        self.inner.owns(&header.0)
    }

    fn apply_event(&self, event: PyRef<'_, PyAudioEventTopic>) -> bool {
        self.inner.apply_event(&event.0)
    }

    /// Subscribe to this stream's PCM frames (requires a prior `init`).
    fn subscribe_frames(&self) -> PyResult<PySubscription> {
        self.inner
            .subscribe_frames()
            .map(PySubscription::of::<AudioCaptureFrameTopic, PyAudioCaptureFrameTopic>)
            .map_err(to_py_err)
    }
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

#[pyclass(module = "booster_sdk_bindings", name = "AudioClient", unsendable)]
pub struct PyAudioClient {
    client: Arc<AudioClient>,
}

#[pymethods]
impl PyAudioClient {
    /// The audio service has no robot-name channel suffix (like C++), so
    /// there is no `robot_name` argument.
    #[new]
    #[pyo3(signature = (startup_wait_sec=None, *, network=None, domain_id=None))]
    fn new(
        startup_wait_sec: Option<f64>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let options = rpc_options(startup_wait_sec, None, network, domain_id)?;
        let client = AudioClient::with_options(options).map_err(to_py_err)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    fn init(&self, py: Python<'_>) -> PyResult<String> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.init().await })
    }

    fn client_id(&self) -> Option<String> {
        self.client.client_id()
    }

    fn is_initialized(&self) -> bool {
        self.client.is_initialized()
    }

    /// Drop the cached client id and RPC channels; re-registers lazily.
    fn shutdown(&self) {
        self.client.shutdown();
    }

    fn generate_request_id(&self) -> String {
        self.client.generate_request_id()
    }

    fn init_player(
        &self,
        py: Python<'_>,
        options: PyPlayerInitOptions,
    ) -> PyResult<PyInitPlayerResponse> {
        let client = Arc::clone(&self.client);
        let options = options.to_rust();
        run(py, async move { client.init_player(&options).await }).map(Into::into)
    }

    fn start_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.start_player(session_id).await })
    }

    fn pause_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.pause_player(session_id).await })
    }

    fn stop_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.stop_player(session_id).await })
    }

    fn reset_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.reset_player(session_id).await })
    }

    fn destroy_player(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.destroy_player(session_id).await })
    }

    fn set_player_volume(&self, py: Python<'_>, session_id: i64, volume: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move {
            client.set_player_volume(session_id, volume).await
        })
    }

    fn get_player_info(&self, py: Python<'_>, session_id: i64) -> PyResult<PyPlayerInfo> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.get_player_info(session_id).await }).map(Into::into)
    }

    fn send_pcm_data(&self, py: Python<'_>, session_id: i64, pcm_bytes: Vec<u8>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move {
            client.send_pcm_data(session_id, pcm_bytes).await
        })
    }

    fn init_recorder(
        &self,
        py: Python<'_>,
        options: PyRecorderInitOptions,
    ) -> PyResult<PyInitRecorderResponse> {
        let client = Arc::clone(&self.client);
        let options = options.to_rust();
        run(py, async move { client.init_recorder(&options).await }).map(Into::into)
    }

    fn start_recorder(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.start_recorder(session_id).await })
    }

    fn pause_recorder(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.pause_recorder(session_id).await })
    }

    fn stop_recorder(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.stop_recorder(session_id).await })
    }

    fn destroy_recorder(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.destroy_recorder(session_id).await })
    }

    fn get_recorder_info(&self, py: Python<'_>, session_id: i64) -> PyResult<PyRecorderInfo> {
        let client = Arc::clone(&self.client);
        run(
            py,
            async move { client.get_recorder_info(session_id).await },
        )
        .map(Into::into)
    }

    fn get_doa_angle(&self, py: Python<'_>) -> PyResult<i32> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.get_doa_angle().await })
    }

    fn set_system_volume(&self, py: Python<'_>, volume: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.set_system_volume(volume).await })
    }

    fn get_system_volume(&self, py: Python<'_>) -> PyResult<f32> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.get_system_volume().await })
    }

    fn set_system_mute(&self, py: Python<'_>, mute: bool) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.set_system_mute(mute).await })
    }

    fn get_system_mute(&self, py: Python<'_>) -> PyResult<bool> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.get_system_mute().await })
    }

    #[pyo3(signature = (options=None))]
    fn init_capture_stream(
        &self,
        py: Python<'_>,
        options: Option<PyAudioCaptureStreamOptions>,
    ) -> PyResult<PyInitCaptureStreamResponse> {
        let client = Arc::clone(&self.client);
        let options = options.unwrap_or_default().to_rust();
        run(
            py,
            async move { client.init_capture_stream(&options).await },
        )
        .map(Into::into)
    }

    fn start_capture_stream(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(
            py,
            async move { client.start_capture_stream(session_id).await },
        )
    }

    fn pause_capture_stream(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(
            py,
            async move { client.pause_capture_stream(session_id).await },
        )
    }

    fn stop_capture_stream(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(
            py,
            async move { client.stop_capture_stream(session_id).await },
        )
    }

    fn destroy_capture_stream(&self, py: Python<'_>, session_id: i64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move {
            client.destroy_capture_stream(session_id).await
        })
    }

    fn get_capture_stream_info(
        &self,
        py: Python<'_>,
        session_id: i64,
    ) -> PyResult<PyAudioCaptureStreamInfo> {
        let client = Arc::clone(&self.client);
        run(py, async move {
            client.get_capture_stream_info(session_id).await
        })
        .map(Into::into)
    }

    fn get_devices(
        &self,
        py: Python<'_>,
        query_type: PyAudioDeviceQueryType,
    ) -> PyResult<Vec<PyAudioDeviceInfo>> {
        let client = Arc::clone(&self.client);
        run(
            py,
            async move { client.get_devices(query_type.into()).await },
        )
        .map(|devices| devices.into_iter().map(Into::into).collect())
    }

    /// Select the input/output endpoint (an `AudioDeviceInfo.device_id`) of a
    /// player, recorder or capture-stream session.
    fn set_preferred_device(
        &self,
        py: Python<'_>,
        session_id: i64,
        device_id: String,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move {
            client.set_preferred_device(session_id, device_id).await
        })
    }

    #[pyo3(signature = (options=None))]
    fn start_bluetooth_scan(
        &self,
        py: Python<'_>,
        options: Option<PyBluetoothScanOptions>,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let options = options.unwrap_or_default().to_rust();
        run(
            py,
            async move { client.start_bluetooth_scan(&options).await },
        )
    }

    fn stop_bluetooth_scan(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move { client.stop_bluetooth_scan().await })
    }

    #[pyo3(signature = (include_unpaired=true))]
    fn get_bluetooth_devices(
        &self,
        py: Python<'_>,
        include_unpaired: bool,
    ) -> PyResult<Vec<PyBluetoothDeviceInfo>> {
        let client = Arc::clone(&self.client);
        run(py, async move {
            client.get_bluetooth_devices(include_unpaired).await
        })
        .map(|devices| devices.into_iter().map(Into::into).collect())
    }

    fn connect_bluetooth_device(
        &self,
        py: Python<'_>,
        options: PyBluetoothConnectOptions,
    ) -> PyResult<PyBluetoothConnectResult> {
        let client = Arc::clone(&self.client);
        let options = options.to_rust();
        run(py, async move {
            client.connect_bluetooth_device(&options).await
        })
        .map(Into::into)
    }

    fn disconnect_bluetooth_device(&self, py: Python<'_>, address: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(py, async move {
            client.disconnect_bluetooth_device(address).await
        })
    }

    fn forget_bluetooth_device(&self, py: Python<'_>, address: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        run(
            py,
            async move { client.forget_bluetooth_device(address).await },
        )
    }

    /// Create an uninitialized player handle (C++ `CreatePlayer`).
    fn create_player(&self) -> PyAudioPlayer {
        PyAudioPlayer {
            inner: Arc::new(self.client.create_player()),
        }
    }

    /// Wrap an existing service-side player session.
    fn attach_player(&self, session_id: i64) -> PyAudioPlayer {
        PyAudioPlayer {
            inner: Arc::new(AudioPlayer::attach(Arc::clone(&self.client), session_id)),
        }
    }

    /// Create an uninitialized recorder handle (C++ `CreateRecorder`).
    fn create_recorder(&self) -> PyAudioRecorder {
        PyAudioRecorder {
            inner: Arc::new(self.client.create_recorder()),
        }
    }

    /// Wrap an existing service-side recorder session.
    fn attach_recorder(&self, session_id: i64) -> PyAudioRecorder {
        PyAudioRecorder {
            inner: Arc::new(AudioRecorder::attach(Arc::clone(&self.client), session_id)),
        }
    }

    /// Create a DOA localizer (C++ `CreateLocalizer`).
    fn create_localizer(&self) -> PyAudioLocalizer {
        PyAudioLocalizer {
            inner: Arc::new(self.client.create_localizer()),
        }
    }

    /// Create an uninitialized capture stream handle (C++ `CreateCaptureStream`).
    fn create_capture_stream(&self) -> PyAudioCaptureStream {
        PyAudioCaptureStream {
            inner: Arc::new(self.client.create_capture_stream()),
        }
    }

    /// Wrap an existing service-side capture stream session.
    fn attach_capture_stream(
        &self,
        session_id: i64,
        data_topic_name: String,
    ) -> PyAudioCaptureStream {
        PyAudioCaptureStream {
            inner: Arc::new(AudioCaptureStream::attach(
                Arc::clone(&self.client),
                session_id,
                data_topic_name,
            )),
        }
    }

    /// Subscribe to Bluetooth lifecycle events.
    fn subscribe_bluetooth_events(&self) -> PyResult<PySubscription> {
        self.client
            .subscribe_bluetooth_events()
            .map(PySubscription::of::<BluetoothEvent, PyBluetoothEvent>)
            .map_err(to_py_err)
    }

    /// Subscribe to playback/recording progress (`rt/booster/audio/progress`).
    fn subscribe_progress(&self) -> PyResult<PySubscription> {
        self.client
            .subscribe_progress()
            .map(PySubscription::of::<AudioProgressTopic, PyAudioProgressTopic>)
            .map_err(to_py_err)
    }

    /// Subscribe to session lifecycle events (`rt/booster/audio/events`).
    fn subscribe_events(&self) -> PyResult<PySubscription> {
        self.client
            .subscribe_events()
            .map(PySubscription::of::<AudioEventTopic, PyAudioEventTopic>)
            .map_err(to_py_err)
    }

    /// Subscribe to service errors (`rt/booster/audio/errors`).
    fn subscribe_errors(&self) -> PyResult<PySubscription> {
        self.client
            .subscribe_errors()
            .map(PySubscription::of::<AudioErrorTopic, PyAudioErrorTopic>)
            .map_err(to_py_err)
    }

    /// Subscribe to a capture stream's PCM frames by its `data_topic_name`.
    fn subscribe_capture_frames(&self, data_topic_name: &str) -> PyResult<PySubscription> {
        self.client
            .subscribe_capture_frames(data_topic_name)
            .map(PySubscription::of::<AudioCaptureFrameTopic, PyAudioCaptureFrameTopic>)
            .map_err(to_py_err)
    }
}

/// DDS topic name of a capture stream's `data_topic_name`.
#[pyfunction(name = "capture_frame_topic_name")]
fn py_capture_frame_topic_name(data_topic_name: &str) -> String {
    capture_frame_topic_name(data_topic_name)
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAudioSourceType>()?;
    m.add_class::<PyPlayerPriority>()?;
    m.add_class::<PyPlayerState>()?;
    m.add_class::<PyRecorderState>()?;
    m.add_class::<PyAudioCaptureStreamState>()?;
    m.add_class::<PyAudioDeviceQueryType>()?;
    m.add_class::<PyAudioDeviceDirection>()?;
    m.add_class::<PyAudioDeviceTransport>()?;
    m.add_class::<PyAudioDeviceBackendAffinity>()?;
    m.add_class::<PyBluetoothScanState>()?;
    m.add_class::<PyBluetoothDeviceState>()?;
    m.add_class::<PyBluetoothMajorClass>()?;
    m.add_class::<PyBluetoothAudioProfile>()?;
    m.add_class::<PySessionType>()?;
    m.add_class::<PyAudioErrorCode>()?;
    m.add_class::<PyAudioEventCode>()?;
    m.add_class::<PyAudioErrorCategory>()?;
    m.add_class::<PyAudioErrorDetail>()?;

    m.add_class::<PyPcmFormat>()?;
    m.add_class::<PyPlayerInitOptions>()?;
    m.add_class::<PyRecorderInitOptions>()?;
    m.add_class::<PyAudioCaptureStreamOptions>()?;
    m.add_class::<PyBluetoothScanOptions>()?;
    m.add_class::<PyBluetoothConnectOptions>()?;
    m.add_class::<PyAudioError>()?;

    m.add_class::<PyInitPlayerResponse>()?;
    m.add_class::<PyInitRecorderResponse>()?;
    m.add_class::<PyInitCaptureStreamResponse>()?;
    m.add_class::<PyPlayerInfo>()?;
    m.add_class::<PyRecorderInfo>()?;
    m.add_class::<PyAudioCaptureStreamInfo>()?;
    m.add_class::<PyAudioDeviceInfo>()?;
    m.add_class::<PyBluetoothDeviceInfo>()?;
    m.add_class::<PyBluetoothConnectResult>()?;

    m.add_class::<PyAudioTopicHeader>()?;
    m.add_class::<PyAudioProgressTopic>()?;
    m.add_class::<PyAudioEventTopic>()?;
    m.add_class::<PyAudioErrorTopic>()?;
    m.add_class::<PyBluetoothEvent>()?;
    m.add_class::<PyAudioCaptureFrame>()?;
    m.add_class::<PyAudioCaptureFrameTopic>()?;

    m.add_class::<PyAudioPlayer>()?;
    m.add_class::<PyAudioRecorder>()?;
    m.add_class::<PyAudioLocalizer>()?;
    m.add_class::<PyAudioCaptureStream>()?;
    m.add_class::<PyAudioClient>()?;

    m.add_function(wrap_pyfunction!(py_capture_frame_topic_name, m)?)?;
    m.add("AUDIO_ERR_NOT_SUPPORTED", AUDIO_ERR_NOT_SUPPORTED)?;
    m.add("AUDIO_PROGRESS_TOPIC", AUDIO_PROGRESS_TOPIC)?;
    m.add("AUDIO_EVENT_TOPIC", AUDIO_EVENT_TOPIC)?;
    m.add("AUDIO_ERROR_TOPIC", AUDIO_ERROR_TOPIC)?;
    Ok(())
}
