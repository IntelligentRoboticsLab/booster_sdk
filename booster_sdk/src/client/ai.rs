//! AI and LUI high-level RPC clients.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dds::{
    AI_API_TOPIC, DdsNode, DdsSubscription, LUI_API_OPERATION_EVENT_TOPIC, LUI_API_TOPIC,
    RpcClient, RpcClientOptions, RpcOperationClient, ai_subtitle_topic, lui_asr_chunk_topic,
};
use crate::types::{BoosterError, Result, RpcError};

crate::api_id_enum! {
    /// AI chat RPC API identifiers.
    AiApiId {
        StartAiChat = 2000,
        StopAiChat = 2001,
        Speak = 2002,
        StartFaceTracking = 2003,
        StopFaceTracking = 2004,
    }
}

crate::api_id_enum! {
    /// LUI speech RPC API identifiers.
    LuiApiId {
        StartAsr = 1000,
        StopAsr = 1001,
        StartTts = 1050,
        StopTts = 1051,
        SendTtsText = 1052,
        SynthesizeSpeech = 1100,
        RecognizeAudioOnce = 1101,
        StartAudioRecognizer = 1102,
        StopAudioRecognizer = 1103,
        RecognizeAudioInSession = 1104,
    }
}

/// Maximum text length for [`LuiClient::synthesize_speech`], in Unicode code points.
pub const LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS: usize = 1000;
/// Maximum audio duration for LUI recognition requests, in milliseconds.
pub const LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS: u64 = 120 * 1000;
/// Maximum decoded audio size for LUI recognition requests, in bytes.
pub const LUI_MAX_RECOGNIZE_AUDIO_BYTES: u64 = 6 * 1024 * 1024;

const LUI_OPERATION_START_TIMEOUT: Duration = Duration::from_millis(3000);
const LUI_SYNTHESIZE_TIMEOUT: Duration = Duration::from_secs(1800);
const LUI_RECOGNIZE_TIMEOUT: Duration = Duration::from_secs(900);

/// TTS configuration for AI chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TtsConfig {
    pub voice_type: String,
    pub ignore_bracket_text: Vec<i8>,
}

/// LLM prompt configuration for AI chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmConfig {
    pub system_prompt: String,
    pub welcome_msg: String,
    pub prompt_name: String,
}

/// ASR interruption configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsrConfig {
    pub interrupt_speech_duration: i32,
    pub interrupt_keywords: Vec<String>,
}

/// Parameters for starting AI chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartAiChatParameter {
    /// Optional AgentHub persona identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persona_id: Option<String>,
    pub interrupt_mode: bool,
    pub asr_config: AsrConfig,
    pub llm_config: LlmConfig,
    pub tts_config: TtsConfig,
    pub enable_face_tracking: bool,
}

/// Parameters for AI speech output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakParameter {
    pub msg: String,
}

/// LUI TTS startup configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LuiTtsConfig {
    pub voice_type: String,
}

/// Parameters for sending TTS text to LUI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LuiTtsParameter {
    pub text: String,
}

/// AI subtitle topic payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subtitle {
    pub magic_number: String,
    pub text: String,
    pub language: String,
    pub user_id: String,
    pub seq: i32,
    pub definite: bool,
    pub paragraph: bool,
    pub round_id: i32,
}

/// Speech synthesis request for [`LuiClient::synthesize_speech`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LuiSynthesizeSpeechRequest {
    pub text: String,
    pub voice_type: String,
    /// Speech speed ratio: 1.0 = normal, 0.5 = slowest, 2.0 = fastest.
    pub speed: f64,
    /// Whether the robot should also play the synthesized audio.
    pub playback: bool,
}

impl LuiSynthesizeSpeechRequest {
    /// Request with the default voice, normal speed and no playback.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            voice_type: "default".to_owned(),
            speed: 1.0,
            playback: false,
        }
    }
}

/// Synthesized audio returned by [`LuiClient::synthesize_speech`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LuiSynthesizeSpeechResponse {
    pub audio_base64: String,
    pub sample_rate_hz: i32,
    pub channels: i32,
    pub bits_per_sample: i32,
    pub format: String,
}

/// Audio recognition request for the LUI recognize APIs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LuiRecognizeAudioRequest {
    /// `"file"` or `"pcm"`.
    pub input_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub file_path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub audio_base64: String,
    pub sample_rate_hz: i32,
    pub channels: i32,
    pub bits_per_sample: i32,
    pub format: String,
}

impl LuiRecognizeAudioRequest {
    /// Recognize a `.wav` or `.mp3` file on the robot.
    pub fn from_file(file_path: impl Into<String>) -> Self {
        Self {
            input_type: "file".to_owned(),
            file_path: file_path.into(),
            audio_base64: String::new(),
            sample_rate_hz: 16000,
            channels: 1,
            bits_per_sample: 16,
            format: "pcm_s16le".to_owned(),
        }
    }

    /// Recognize base64-encoded 16 kHz mono 16-bit little-endian raw PCM.
    pub fn from_pcm_base64(audio_base64: impl Into<String>) -> Self {
        Self {
            input_type: "pcm".to_owned(),
            file_path: String::new(),
            audio_base64: audio_base64.into(),
            sample_rate_hz: 16000,
            channels: 1,
            bits_per_sample: 16,
            format: "pcm_s16le".to_owned(),
        }
    }
}

/// Recognition result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LuiRecognizeAudioResponse {
    pub text: String,
}

/// Utterance entry inside an [`AsrChunk`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsrUtterance {
    pub text: String,
    pub definite: bool,
    pub start_time_ms: u32,
    pub end_time_ms: u32,
    pub speaker_id: String,
    pub emotion: String,
    pub gender: String,
    pub lid_lang: String,
    pub speech_rate: f32,
    pub volume_db: f32,
    pub additions_json: String,
}

/// LUI ASR chunk topic payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsrChunk {
    pub session_id: String,
    pub client_id: String,
    pub role: String,
    pub text: String,
    pub is_final: bool,
    pub start_time_ms: u32,
    pub end_time_ms: u32,
    pub speaker_id: String,
    pub emotion: String,
    pub gender: String,
    pub lid_lang: String,
    pub speech_rate: f32,
    pub volume_db: f32,
    pub provider_logid: String,
    pub raw_json: String,
    pub utterances: Vec<AsrUtterance>,
}

/// User identifier used by robot-generated subtitle entries.
pub const BOOSTER_ROBOT_USER_ID: &str = "BoosterRobot";

/// High-level RPC client for AI chat features.
pub struct AiClient {
    rpc: RpcClient,
}

impl AiClient {
    /// Create an AI client with default options.
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(AI_API_TOPIC))
    }

    /// Create an AI client with a custom startup wait before first RPC.
    pub fn with_startup_wait(startup_wait: Duration) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(AI_API_TOPIC).with_startup_wait(startup_wait),
        )
    }

    /// Create an AI client with custom RPC options.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::for_topic(options, AI_API_TOPIC)?;
        Ok(Self { rpc })
    }

    /// Access the underlying DDS node.
    pub fn node(&self) -> &DdsNode {
        self.rpc.node()
    }

    /// Start AI chat with the provided configuration.
    pub async fn start_ai_chat(&self, param: &StartAiChatParameter) -> Result<()> {
        self.rpc.call_serialized(AiApiId::StartAiChat, param).await
    }

    /// Stop the active AI chat session.
    pub async fn stop_ai_chat(&self) -> Result<()> {
        self.rpc.call_void(AiApiId::StopAiChat, "").await
    }

    /// Request the AI service to speak a message.
    pub async fn speak(&self, param: &SpeakParameter) -> Result<()> {
        self.rpc.call_serialized(AiApiId::Speak, param).await
    }

    /// Enable face tracking in the AI service.
    pub async fn start_face_tracking(&self) -> Result<()> {
        self.rpc.call_void(AiApiId::StartFaceTracking, "").await
    }

    /// Disable face tracking in the AI service.
    pub async fn stop_face_tracking(&self) -> Result<()> {
        self.rpc.call_void(AiApiId::StopFaceTracking, "").await
    }

    /// Subscribe to AI subtitle messages.
    pub fn subscribe_subtitle(&self) -> Result<DdsSubscription<Subtitle>> {
        self.rpc.node().subscribe(&ai_subtitle_topic(), 16)
    }
}

#[derive(Debug, Default)]
struct LuiSessions {
    asr: Option<String>,
    audio_recognizer: Option<String>,
    tts: Option<(String, String)>,
}

/// High-level RPC client for LUI ASR/TTS features.
///
/// Since SDK 1.8 every ASR, TTS and recognizer session is owned by the client
/// instance that started it, identified by a per-client id.
pub struct LuiClient {
    rpc: RpcClient,
    client_id: String,
    sessions: Mutex<LuiSessions>,
    operations: Mutex<Option<Arc<RpcOperationClient>>>,
}

impl LuiClient {
    /// Create a LUI client with default options.
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(LUI_API_TOPIC))
    }

    /// Create a LUI client with a custom startup wait before first RPC.
    pub fn with_startup_wait(startup_wait: Duration) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(LUI_API_TOPIC).with_startup_wait(startup_wait),
        )
    }

    /// Create a LUI client with custom RPC options.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::for_topic(options, LUI_API_TOPIC)?;
        Ok(Self {
            rpc,
            client_id: Uuid::new_v4().to_string(),
            sessions: Mutex::new(LuiSessions::default()),
            operations: Mutex::new(None),
        })
    }

    /// Access the underlying DDS node.
    pub fn node(&self) -> &DdsNode {
        self.rpc.node()
    }

    /// Identifier of this client instance, sent with every session request.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Session id of the ASR session started by this client, if any.
    pub fn current_asr_session_id(&self) -> Option<String> {
        self.sessions().ok()?.asr.clone()
    }

    /// Session id of the audio recognizer started by this client, if any.
    pub fn current_audio_recognizer_session_id(&self) -> Option<String> {
        self.sessions().ok()?.audio_recognizer.clone()
    }

    /// Session id of the TTS session started by this client, if any.
    pub fn current_tts_session_id(&self) -> Option<String> {
        self.sessions().ok()?.tts.as_ref().map(|(id, _)| id.clone())
    }

    fn sessions(&self) -> Result<std::sync::MutexGuard<'_, LuiSessions>> {
        self.sessions
            .lock()
            .map_err(|_| BoosterError::Other("LUI session state poisoned".to_owned()))
    }

    fn session_body(&self, session_id: &str) -> serde_json::Value {
        serde_json::json!({ "session_id": session_id, "client_id": self.client_id })
    }

    fn operation_client(&self) -> Result<Arc<RpcOperationClient>> {
        let mut operations = self
            .operations
            .lock()
            .map_err(|_| BoosterError::Other("operation client poisoned".to_owned()))?;
        if let Some(client) = operations.as_ref() {
            return Ok(Arc::clone(client));
        }
        let client = Arc::new(RpcOperationClient::new(
            self.rpc.node(),
            LUI_API_OPERATION_EVENT_TOPIC,
        )?);
        *operations = Some(Arc::clone(&client));
        Ok(client)
    }

    async fn run_operation<R>(&self, api_id: LuiApiId, body: String, timeout: Duration) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let mut operation = self
            .operation_client()?
            .start(&self.rpc, api_id.into(), body, LUI_OPERATION_START_TIMEOUT)
            .await?;
        let body = operation.wait_timeout(timeout).await?.into_body()?;
        Ok(serde_json::from_str(body.trim())?)
    }

    /// Start ASR. Recognized text is published on the ASR chunk topic.
    ///
    /// Does nothing if this client already started ASR.
    pub async fn start_asr(&self) -> Result<()> {
        if self.sessions()?.asr.is_some() {
            return Ok(());
        }
        let session_id = Uuid::new_v4().to_string();
        self.rpc
            .call_void(
                LuiApiId::StartAsr,
                self.session_body(&session_id).to_string(),
            )
            .await?;
        self.sessions()?.asr = Some(session_id);
        Ok(())
    }

    /// Stop the ASR session started by this client.
    pub async fn stop_asr(&self) -> Result<()> {
        let Some(session_id) = self.sessions()?.asr.clone() else {
            return Ok(());
        };
        self.rpc
            .call_void(
                LuiApiId::StopAsr,
                self.session_body(&session_id).to_string(),
            )
            .await?;
        self.sessions()?.asr = None;
        Ok(())
    }

    /// Start a TTS session owned by this client.
    ///
    /// Succeeds without a request if this client already has a TTS session
    /// with the same voice; fails with a conflict for a different voice.
    pub async fn start_tts(&self, config: &LuiTtsConfig) -> Result<()> {
        if let Some((_, voice_type)) = &self.sessions()?.tts {
            if *voice_type == config.voice_type {
                return Ok(());
            }
            return Err(RpcError::Conflict(format!(
                "TTS already started with voice type '{voice_type}'"
            ))
            .into());
        }
        let session_id = Uuid::new_v4().to_string();
        let mut body = self.session_body(&session_id);
        body["voice_type"] = config.voice_type.clone().into();
        self.rpc
            .call_void(LuiApiId::StartTts, body.to_string())
            .await?;
        self.sessions()?.tts = Some((session_id, config.voice_type.clone()));
        Ok(())
    }

    /// Stop the TTS session started by this client.
    pub async fn stop_tts(&self) -> Result<()> {
        let Some((session_id, _)) = self.sessions()?.tts.clone() else {
            return Ok(());
        };
        self.rpc
            .call_void(
                LuiApiId::StopTts,
                self.session_body(&session_id).to_string(),
            )
            .await?;
        self.sessions()?.tts = None;
        Ok(())
    }

    fn tts_session_id(&self) -> Result<String> {
        self.sessions()?
            .tts
            .as_ref()
            .map(|(id, _)| id.clone())
            .ok_or_else(|| RpcError::BadRequest("TTS is not started".to_owned()).into())
    }

    /// Queue text in this client's TTS session.
    ///
    /// Requires [`Self::start_tts`]. The service rejects requests sent more
    /// than once per second.
    pub async fn send_tts_text(&self, param: &LuiTtsParameter) -> Result<()> {
        let mut body = self.session_body(&self.tts_session_id()?);
        body["text"] = param.text.clone().into();
        self.rpc
            .call_void(LuiApiId::SendTtsText, body.to_string())
            .await
    }

    /// Synthesize speech in this client's TTS session and return the audio.
    ///
    /// Requires [`Self::start_tts`].
    pub async fn synthesize_speech(
        &self,
        req: &LuiSynthesizeSpeechRequest,
    ) -> Result<LuiSynthesizeSpeechResponse> {
        let text_len = req.text.chars().count();
        if text_len > LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS {
            return Err(RpcError::BadRequest(format!(
                "text length {text_len} exceeds limit {LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS}"
            ))
            .into());
        }
        let session_id = self.tts_session_id()?;
        let mut body = serde_json::to_value(req)?;
        body["session_id"] = session_id.into();
        body["client_id"] = self.client_id.clone().into();
        self.run_operation(
            LuiApiId::SynthesizeSpeech,
            body.to_string(),
            LUI_SYNTHESIZE_TIMEOUT,
        )
        .await
    }

    /// Recognize an audio file or PCM payload without a session.
    pub async fn recognize_audio_once(
        &self,
        req: &LuiRecognizeAudioRequest,
    ) -> Result<LuiRecognizeAudioResponse> {
        validate_recognize_audio(req)?;
        self.run_operation(
            LuiApiId::RecognizeAudioOnce,
            serde_json::to_string(req)?,
            LUI_RECOGNIZE_TIMEOUT,
        )
        .await
    }

    /// Start a reusable audio recognizer session owned by this client.
    ///
    /// Does nothing if this client already started one.
    pub async fn start_audio_recognizer(&self) -> Result<()> {
        if self.sessions()?.audio_recognizer.is_some() {
            return Ok(());
        }
        let session_id = Uuid::new_v4().to_string();
        self.rpc
            .call_void(
                LuiApiId::StartAudioRecognizer,
                self.session_body(&session_id).to_string(),
            )
            .await?;
        self.sessions()?.audio_recognizer = Some(session_id);
        Ok(())
    }

    /// Stop the audio recognizer session started by this client.
    pub async fn stop_audio_recognizer(&self) -> Result<()> {
        let Some(session_id) = self.sessions()?.audio_recognizer.clone() else {
            return Ok(());
        };
        self.rpc
            .call_void(
                LuiApiId::StopAudioRecognizer,
                self.session_body(&session_id).to_string(),
            )
            .await?;
        self.sessions()?.audio_recognizer = None;
        Ok(())
    }

    /// Recognize audio in this client's audio recognizer session.
    ///
    /// Requires [`Self::start_audio_recognizer`].
    pub async fn recognize_audio_in_session(
        &self,
        req: &LuiRecognizeAudioRequest,
    ) -> Result<LuiRecognizeAudioResponse> {
        validate_recognize_audio(req)?;
        let session_id =
            self.sessions()?.audio_recognizer.clone().ok_or_else(|| {
                RpcError::BadRequest("audio recognizer is not started".to_owned())
            })?;
        let mut body = serde_json::to_value(req)?;
        body["session_id"] = session_id.into();
        body["client_id"] = self.client_id.clone().into();
        self.run_operation(
            LuiApiId::RecognizeAudioInSession,
            body.to_string(),
            LUI_RECOGNIZE_TIMEOUT,
        )
        .await
    }

    /// Subscribe to ASR chunk messages.
    pub fn subscribe_asr_chunk(&self) -> Result<DdsSubscription<AsrChunk>> {
        self.rpc.node().subscribe(&lui_asr_chunk_topic(), 16)
    }
}

/// Decoded byte count of a base64 payload, derived from its length.
fn estimate_base64_decoded_bytes(base64: &str) -> u64 {
    let padding = base64.bytes().rev().take_while(|&b| b == b'=').count() as u64;
    (base64.len() as u64 / 4 * 3).saturating_sub(padding)
}

/// Client-side check of the PCM limits the LUI service enforces.
fn validate_recognize_audio(req: &LuiRecognizeAudioRequest) -> Result<()> {
    if req.input_type != "pcm" {
        return Ok(());
    }
    let bytes = estimate_base64_decoded_bytes(&req.audio_base64);
    if bytes > LUI_MAX_RECOGNIZE_AUDIO_BYTES {
        return Err(RpcError::BadRequest(format!(
            "audio size {bytes} bytes exceeds limit {LUI_MAX_RECOGNIZE_AUDIO_BYTES} bytes"
        ))
        .into());
    }
    let bytes_per_second = u64::try_from(
        i64::from(req.sample_rate_hz) * i64::from(req.channels) * i64::from(req.bits_per_sample)
            / 8,
    )
    .unwrap_or(0);
    if let Some(duration_ms) = (bytes * 1000).checked_div(bytes_per_second)
        && duration_ms > LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS
    {
        return Err(RpcError::BadRequest(format!(
            "audio duration {duration_ms} ms exceeds limit {LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS} ms"
        ))
        .into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_decoded_size_accounts_for_padding() {
        assert_eq!(estimate_base64_decoded_bytes("QUJD"), 3);
        assert_eq!(estimate_base64_decoded_bytes("QUI="), 2);
        assert_eq!(estimate_base64_decoded_bytes("QQ=="), 1);
    }

    #[test]
    fn recognize_request_omits_empty_fields() {
        let json = serde_json::to_value(LuiRecognizeAudioRequest::from_file("/tmp/a.wav")).unwrap();
        assert_eq!(json["input_type"], "file");
        assert_eq!(json["file_path"], "/tmp/a.wav");
        assert!(json.get("audio_base64").is_none());
    }

    #[test]
    fn rejects_too_long_pcm() {
        // 16 kHz mono s16 = 32000 B/s; 121 s of audio.
        let bytes = 32000 * 121;
        let req = LuiRecognizeAudioRequest::from_pcm_base64("A".repeat(bytes / 3 * 4));
        assert!(validate_recognize_audio(&req).is_err());
        let req = LuiRecognizeAudioRequest::from_pcm_base64("AAAA");
        assert!(validate_recognize_audio(&req).is_ok());
    }

    #[test]
    fn start_ai_chat_omits_missing_persona() {
        let param = StartAiChatParameter {
            persona_id: None,
            interrupt_mode: false,
            asr_config: AsrConfig {
                interrupt_speech_duration: 0,
                interrupt_keywords: vec![],
            },
            llm_config: LlmConfig {
                system_prompt: String::new(),
                welcome_msg: String::new(),
                prompt_name: String::new(),
            },
            tts_config: TtsConfig {
                voice_type: String::new(),
                ignore_bracket_text: vec![],
            },
            enable_face_tracking: false,
        };
        let json = serde_json::to_value(&param).unwrap();
        assert!(json.get("persona_id").is_none());
    }
}
