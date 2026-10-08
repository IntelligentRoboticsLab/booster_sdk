//! Audio service RPC client for Booster SDK v1.7.

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value, json};

use crate::{
    dds::{
        BluetoothEvent, DdsSubscription, RpcClient, RpcClientOptions, audio_bluetooth_event_topic,
    },
    types::{Result, RpcError},
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
            Self::StartBluetoothScan => "rt/booster/audio/start_bluetooth_scan",
            Self::StopBluetoothScan => "rt/booster/audio/stop_bluetooth_scan",
            Self::GetBluetoothDevices => "rt/booster/audio/get_bluetooth_devices",
            Self::ConnectBluetoothDevice => "rt/booster/audio/connect_bluetooth_device",
            Self::DisconnectBluetoothDevice => "rt/booster/audio/disconnect_bluetooth_device",
            Self::ForgetBluetoothDevice => "rt/booster/audio/forget_bluetooth_device",
        }
    }
}

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

impl PlayerInitOptions {
    #[must_use]
    pub fn pcm_stream() -> Self {
        Self {
            source_type: AudioSourceType::PcmStream,
            source_uri: "pcm_stream".to_owned(),
            sample_rate_hz: 16_000,
            channels: 1,
            bits_per_sample: 16,
            priority: PlayerPriority::Medium,
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

impl PlayerInfo {
    #[must_use]
    pub fn state_enum(&self) -> Option<PlayerState> {
        PlayerState::try_from(self.state).ok()
    }
}

/// Recorder status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl AudioCaptureStreamInfo {
    #[must_use]
    pub fn state_enum(&self) -> Option<AudioCaptureStreamState> {
        AudioCaptureStreamState::try_from(self.state).ok()
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

/// High-level client for the v1.6 audio service.
pub struct AudioClient {
    options: RpcClientOptions,
    clients: Mutex<HashMap<AudioServiceMethod, Arc<RpcClient>>>,
    client_id: Mutex<Option<String>>,
    request_sequence: AtomicU64,
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
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
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

    fn next_request_id(&self) -> String {
        let seq = self.request_sequence.fetch_add(1, Ordering::Relaxed);
        format!("audio_req_{seq}")
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
        let request_id = self.next_request_id();
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
        self.call_raw(method, Value::Object(object)).await
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
        let request_id = self.next_request_id();
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
        self.rpc_client(method)?
            .call_with_body(
                AUDIO_RPC_API_ID,
                Value::Object(object).to_string(),
                Some(timeout),
            )
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

    /// Subscribe to Bluetooth lifecycle events.
    pub fn subscribe_bluetooth_events(&self) -> Result<DdsSubscription<BluetoothEvent>> {
        self.rpc_client(AudioServiceMethod::RegisterClient)?
            .node()
            .subscribe(&audio_bluetooth_event_topic(), 16)
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
    }
}
