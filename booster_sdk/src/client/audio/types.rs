//! Public audio types mirroring `booster/robot/audio/audio_types.h` and the
//! enumerations of `booster/idl/audio/AudioCommon.h` (SDK 1.8).

use serde::{Deserialize, Serialize};

/// Error code returned when an audio capability is unavailable on the target
/// (C++ `kAudioErrNotSupported`).
pub const AUDIO_ERR_NOT_SUPPORTED: i32 = 7001;

crate::api_id_enum! {
    /// Audio source type.
    AudioSourceType {
        PcmFile = 0,
        WavFile = 1,
        PcmStream = 2,
        Mp3File = 3,
    }
}

crate::api_id_enum! {
    /// Direction used when querying audio devices.
    AudioDeviceQueryType {
        Inputs = 0,
        Outputs = 1,
    }
}

crate::api_id_enum! {
    /// Audio device direction.
    AudioDeviceDirection {
        Input = 0,
        Output = 1,
    }
}

crate::api_id_enum! {
    /// Audio device transport.
    AudioDeviceTransport {
        Builtin = 0,
        Usb = 1,
        Bluetooth = 2,
        Virtual = 3,
        Unknown = 4,
    }
}

crate::api_id_enum! {
    /// Backend responsible for an audio device.
    AudioDeviceBackendAffinity {
        Pulse = 0,
        BoosterAecArray = 1,
        AlsaDiagnostic = 2,
    }
}

crate::api_id_enum! {
    /// Bluetooth scanning state.
    BluetoothScanState {
        Idle = 0,
        Scanning = 1,
    }
}

crate::api_id_enum! {
    /// Bluetooth device connection state.
    BluetoothDeviceState {
        Unknown = 0,
        Paired = 1,
        Connecting = 2,
        Connected = 3,
        Disconnecting = 4,
        Failed = 5,
    }
}

crate::api_id_enum! {
    /// Bluetooth major device class.
    BluetoothMajorClass {
        Audio = 0,
        Peripheral = 1,
        Phone = 2,
        Computer = 3,
        Other = 4,
    }
}

crate::api_id_enum! {
    /// Bluetooth audio profile.
    BluetoothAudioProfile {
        None = 0,
        A2dpSink = 1,
        A2dpSource = 2,
        HfpHeadset = 3,
        HfpAg = 4,
    }
}

crate::api_id_enum! {
    /// Player state.
    PlayerState {
        Idle = 0,
        Ready = 1,
        Playing = 2,
        Paused = 3,
        Stopped = 4,
        Completed = 5,
        Error = 6,
    }
}

crate::api_id_enum! {
    /// Player priority.
    PlayerPriority {
        Low = 0,
        Medium = 1,
        High = 2,
    }
}

crate::api_id_enum! {
    /// Recorder state.
    RecorderState {
        Idle = 0,
        Ready = 1,
        Recording = 2,
        Paused = 3,
        Stopped = 4,
        Error = 5,
    }
}

crate::api_id_enum! {
    /// Audio capture stream state.
    AudioCaptureStreamState {
        Idle = 0,
        Ready = 1,
        Streaming = 2,
        Paused = 3,
        Stopped = 4,
        Error = 5,
    }
}

crate::api_id_enum! {
    /// Kind of audio session a topic message refers to (IDL `SessionType`).
    SessionType {
        Player = 0,
        Recorder = 1,
        Localizer = 2,
        CaptureStream = 3,
    }
}

crate::api_id_enum! {
    /// Service-level audio error code (IDL `AudioErrorCode`).
    AudioErrorCode {
        ServiceDegraded = 0,
        InvalidState = 1,
        DeviceBusy = 2,
        NotSupported = 3,
        LocalizationUnavailable = 4,
        AlsaDeviceNotFound = 5,
        AlgoInitFailed = 6,
        AlgoProcessFailed = 7,
        SystemVolumeCtrlFailed = 8,
        SystemMuteCtrlFailed = 9,
        MediaDecodeFailed = 10,
        MediaEncodeFailed = 11,
        MediaFormatUnsupported = 12,
    }
}

crate::api_id_enum! {
    /// Event code carried by `AudioEventTopic` (IDL `AudioEventCode`).
    AudioEventCode {
        PlayerInitSucceeded = 0,
        PlayerStartSucceeded = 1,
        PlayerPauseSucceeded = 2,
        PlayerStopSucceeded = 3,
        PlayerResetSucceeded = 4,
        PlayerDestroySucceeded = 5,
        RecorderInitSucceeded = 6,
        RecorderStartSucceeded = 7,
        RecorderPauseSucceeded = 8,
        RecorderStopSucceeded = 9,
        RecorderDestroySucceeded = 10,
        PlayerTerminalStateChanged = 11,
        SystemVolumeSetSucceeded = 12,
        SystemMuteSetSucceeded = 13,
        CaptureStreamInitSucceeded = 14,
        CaptureStreamStartSucceeded = 15,
        CaptureStreamPauseSucceeded = 16,
        CaptureStreamStopSucceeded = 17,
        CaptureStreamDestroySucceeded = 18,
    }
}

crate::api_id_enum! {
    /// Error category carried by `AudioErrorTopic` (IDL `AudioErrorCategory`).
    AudioErrorCategory {
        RpcError = 0,
        PlaybackError = 1,
        RecordError = 2,
        CaptureStreamRuntimeError = 3,
        LocalizationError = 4,
        SystemVolumeError = 5,
        SystemMuteError = 6,
        InternalError = 7,
    }
}

crate::api_id_enum! {
    /// Operation an `AudioErrorTopic` refers to (IDL `AudioErrorDetail`).
    AudioErrorDetail {
        None = 0,
        InitPlayer = 1,
        StartPlayer = 2,
        PausePlayer = 3,
        StopPlayer = 4,
        ResetPlayer = 5,
        DestroyPlayer = 6,
        GetPlayerInfo = 7,
        SendPcmData = 8,
        InitRecorder = 9,
        StartRecorder = 10,
        PauseRecorder = 11,
        StopRecorder = 12,
        DestroyRecorder = 13,
        GetRecorderInfo = 14,
        GetDoaAngle = 15,
        SetSystemVolume = 16,
        GetSystemVolume = 17,
        SetSystemMute = 18,
        GetSystemMute = 19,
        InitCaptureStream = 20,
        StartCaptureStream = 21,
        PauseCaptureStream = 22,
        StopCaptureStream = 23,
        DestroyCaptureStream = 24,
        GetCaptureStreamInfo = 25,
    }
}

/// PCM format descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PcmFormat {
    pub sample_rate_hz: i32,
    pub channels: i32,
    pub bits_per_sample: i32,
}

impl Default for PcmFormat {
    fn default() -> Self {
        Self {
            sample_rate_hz: 16_000,
            channels: 1,
            bits_per_sample: 16,
        }
    }
}

/// Player initialization options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerInitOptions {
    pub source_type: AudioSourceType,
    pub source_uri: String,
    pub sample_rate_hz: i32,
    pub channels: i32,
    pub bits_per_sample: i32,
    pub priority: PlayerPriority,
}

impl Default for PlayerInitOptions {
    /// Matches the C++ initializers: `kPcmFile`, empty URI, 16 kHz mono 16-bit, `kMedium`.
    fn default() -> Self {
        Self {
            source_type: AudioSourceType::PcmFile,
            source_uri: String::new(),
            sample_rate_hz: 16_000,
            channels: 1,
            bits_per_sample: 16,
            priority: PlayerPriority::Medium,
        }
    }
}

impl PlayerInitOptions {
    /// Options for a PCM stream player fed through `send_pcm_data`.
    #[must_use]
    pub fn pcm_stream() -> Self {
        Self {
            source_type: AudioSourceType::PcmStream,
            source_uri: "pcm_stream".to_owned(),
            ..Self::default()
        }
    }
}

/// Recorder initialization options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecorderInitOptions {
    pub output_path: String,
    pub sample_rate_hz: i32,
    pub channels: i32,
    pub bits_per_sample: i32,
}

impl Default for RecorderInitOptions {
    /// Matches the C++ initializers: empty path, 16 kHz mono 16-bit.
    fn default() -> Self {
        Self {
            output_path: String::new(),
            sample_rate_hz: 16_000,
            channels: 1,
            bits_per_sample: 16,
        }
    }
}

/// Audio capture stream initialization options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioCaptureStreamOptions {
    pub enable_raw_pcm: bool,
    pub enable_naec_pcm: bool,
    pub requested_raw_format: PcmFormat,
}

impl Default for AudioCaptureStreamOptions {
    fn default() -> Self {
        Self {
            enable_raw_pcm: true,
            enable_naec_pcm: false,
            // BoosterAEC raw PCM default: 16 kHz, 3 channels, 16-bit.
            requested_raw_format: PcmFormat {
                channels: 3,
                ..PcmFormat::default()
            },
        }
    }
}

/// Generic audio service result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceResult {
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
}

/// Response returned by player initialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitPlayerResponse {
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    pub session_id: i64,
}

/// Response returned by recorder initialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitRecorderResponse {
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    pub session_id: i64,
}

/// Response returned by audio capture stream initialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitCaptureStreamResponse {
    pub ret_code: i32,
    #[serde(default)]
    pub ret_msg: String,
    pub session_id: i64,
    /// DDS topic carrying `AudioCaptureFrameTopic` samples for this session.
    /// Pass it to [`super::AudioClient::subscribe_capture_frames`].
    #[serde(default)]
    pub data_topic_name: String,
}

/// Player status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub state: i32,
    pub played_bytes: i64,
    pub total_bytes: i64,
    pub volume: f32,
}

impl Default for PlayerInfo {
    /// Matches the C++ initializers: `kIdle`, zero counters, volume 1.0.
    fn default() -> Self {
        Self {
            state: PlayerState::Idle.into(),
            played_bytes: 0,
            total_bytes: 0,
            volume: 1.0,
        }
    }
}

impl PlayerInfo {
    #[must_use]
    pub fn state_enum(&self) -> Option<PlayerState> {
        PlayerState::try_from(self.state).ok()
    }
}

/// Recorder status.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecorderInfo {
    pub state: i32,
    pub captured_bytes: i64,
}

impl RecorderInfo {
    #[must_use]
    pub fn state_enum(&self) -> Option<RecorderState> {
        RecorderState::try_from(self.state).ok()
    }
}

/// Audio capture stream status.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioCaptureStreamInfo {
    pub state: i32,
    #[serde(default)]
    pub raw_enabled: bool,
    #[serde(default)]
    pub naec_enabled: bool,
    #[serde(default)]
    pub actual_raw_format: PcmFormat,
    #[serde(default)]
    pub actual_naec_format: PcmFormat,
    #[serde(default)]
    pub published_frames: i64,
    #[serde(default)]
    pub dropped_frames: i64,
}

impl AudioCaptureStreamInfo {
    #[must_use]
    pub fn state_enum(&self) -> Option<AudioCaptureStreamState> {
        AudioCaptureStreamState::try_from(self.state).ok()
    }
}

/// One captured audio frame with optional raw and NAEC PCM payloads
/// (C++ `AudioCaptureFrame`). Convert from the DDS sample with `From`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AudioCaptureFrame {
    pub frame_seq: i64,
    pub timestamp_ms: i64,

    pub raw_valid: bool,
    pub raw_format: PcmFormat,
    pub raw_frame_samples_per_channel: i32,
    pub raw_pcm: Vec<i16>,

    pub naec_valid: bool,
    pub naec_format: PcmFormat,
    pub naec_frame_samples_per_channel: i32,
    pub naec_pcm: Vec<i16>,
}

/// Audio input or output device descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    pub device_id: String,
    pub display_name: String,
    pub direction: AudioDeviceDirection,
    pub transport: AudioDeviceTransport,
    pub backend_affinity: AudioDeviceBackendAffinity,
    pub is_available: bool,
    pub is_system_default: bool,
    pub supports_input: bool,
    pub supports_output: bool,
    #[serde(default)]
    pub provider_name: String,
    #[serde(default)]
    pub native_id: String,
    #[serde(default)]
    pub metadata_json: String,
}

impl Default for AudioDeviceInfo {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            display_name: String::new(),
            direction: AudioDeviceDirection::Input,
            transport: AudioDeviceTransport::Unknown,
            backend_affinity: AudioDeviceBackendAffinity::Pulse,
            is_available: false,
            is_system_default: false,
            supports_input: false,
            supports_output: false,
            provider_name: String::new(),
            native_id: String::new(),
            metadata_json: String::new(),
        }
    }
}

/// Detailed error delivered through the audio error topic (C++ `AudioError`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioError {
    pub ret_code: i32,
    pub ret_msg: String,
    pub error_category: i32,
    pub error_detail: i32,
}

impl AudioError {
    #[must_use]
    pub fn category_enum(&self) -> Option<AudioErrorCategory> {
        AudioErrorCategory::try_from(self.error_category).ok()
    }

    #[must_use]
    pub fn detail_enum(&self) -> Option<AudioErrorDetail> {
        AudioErrorDetail::try_from(self.error_detail).ok()
    }
}

/// Bluetooth device descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BluetoothDeviceInfo {
    pub address: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub rssi: i16,
    pub state: BluetoothDeviceState,
    pub major_class: BluetoothMajorClass,
    #[serde(default)]
    pub paired: bool,
    #[serde(default)]
    pub trusted: bool,
    #[serde(default)]
    pub connected: bool,
    #[serde(default)]
    pub is_audio_sink: bool,
    #[serde(default)]
    pub is_audio_source: bool,
    #[serde(default)]
    pub is_hfp_capable: bool,
    #[serde(default)]
    pub connected_profiles: Vec<BluetoothAudioProfile>,
    #[serde(default)]
    pub linked_pulse_sink_id: String,
    #[serde(default)]
    pub linked_pulse_source_id: String,
    #[serde(default)]
    pub last_seen_ms: i64,
}

impl Default for BluetoothDeviceInfo {
    fn default() -> Self {
        Self {
            address: String::new(),
            name: String::new(),
            rssi: 0,
            state: BluetoothDeviceState::Unknown,
            major_class: BluetoothMajorClass::Other,
            paired: false,
            trusted: false,
            connected: false,
            is_audio_sink: false,
            is_audio_source: false,
            is_hfp_capable: false,
            connected_profiles: Vec::new(),
            linked_pulse_sink_id: String::new(),
            linked_pulse_source_id: String::new(),
            last_seen_ms: 0,
        }
    }
}

/// Options for Bluetooth discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BluetoothScanOptions {
    pub timeout_ms: i32,
    pub audio_only: bool,
}

impl Default for BluetoothScanOptions {
    fn default() -> Self {
        Self {
            timeout_ms: 30_000,
            audio_only: true,
        }
    }
}

/// Options for connecting a Bluetooth audio device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BluetoothConnectOptions {
    pub address: String,
    pub auto_pair: bool,
    pub make_default: bool,
    pub preferred_profile: BluetoothAudioProfile,
    pub timeout_ms: i32,
}

impl Default for BluetoothConnectOptions {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl BluetoothConnectOptions {
    #[must_use]
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            auto_pair: true,
            make_default: true,
            preferred_profile: BluetoothAudioProfile::None,
            timeout_ms: 15_000,
        }
    }
}

/// Result returned after connecting a Bluetooth audio device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BluetoothConnectResult {
    pub device: BluetoothDeviceInfo,
    #[serde(default)]
    pub pulse_sink_id: String,
    #[serde(default)]
    pub pulse_source_id: String,
    pub active_profile: BluetoothAudioProfile,
    #[serde(default)]
    pub pulse_endpoint_ready: bool,
    #[serde(default)]
    pub default_sink_applied: bool,
    #[serde(default)]
    pub default_source_applied: bool,
    #[serde(default)]
    pub default_route_error_code: i32,
    #[serde(default)]
    pub default_route_error_msg: String,
}

impl Default for BluetoothConnectResult {
    fn default() -> Self {
        Self {
            device: BluetoothDeviceInfo::default(),
            pulse_sink_id: String::new(),
            pulse_source_id: String::new(),
            active_profile: BluetoothAudioProfile::None,
            pulse_endpoint_ready: false,
            default_sink_applied: false,
            default_source_applied: false,
            default_route_error_code: 0,
            default_route_error_msg: String::new(),
        }
    }
}
