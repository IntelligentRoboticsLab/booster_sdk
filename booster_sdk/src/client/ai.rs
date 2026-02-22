//! AI and LUI high-level RPC clients.

use serde::{Deserialize, Serialize};

use crate::dds::{
    AI_API_TOPIC, DdsNode, DdsSubscription, LUI_API_TOPIC, RpcClient, RpcClientOptions,
    ai_subtitle_topic, lui_asr_chunk_topic,
};
use crate::types::Result;

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
    }
}

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

/// LUI ASR chunk topic payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsrChunk {
    pub text: String,
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

/// High-level RPC client for LUI ASR/TTS features.
pub struct LuiClient {
    rpc: RpcClient,
}

impl LuiClient {
    /// Create a LUI client with default options.
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(LUI_API_TOPIC))
    }

    /// Create a LUI client with custom RPC options.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::for_topic(options, LUI_API_TOPIC)?;
        Ok(Self { rpc })
    }

    /// Access the underlying DDS node.
    pub fn node(&self) -> &DdsNode {
        self.rpc.node()
    }

    /// Start ASR.
    pub async fn start_asr(&self) -> Result<()> {
        self.rpc.call_void(LuiApiId::StartAsr, "").await
    }

    /// Stop ASR.
    pub async fn stop_asr(&self) -> Result<()> {
        self.rpc.call_void(LuiApiId::StopAsr, "").await
    }

    /// Start TTS with the given configuration.
    pub async fn start_tts(&self, config: &LuiTtsConfig) -> Result<()> {
        self.rpc.call_serialized(LuiApiId::StartTts, config).await
    }

    /// Stop TTS.
    pub async fn stop_tts(&self) -> Result<()> {
        self.rpc.call_void(LuiApiId::StopTts, "").await
    }

    /// Send text to TTS.
    pub async fn send_tts_text(&self, param: &LuiTtsParameter) -> Result<()> {
        self.rpc.call_serialized(LuiApiId::SendTtsText, param).await
    }

    /// Subscribe to ASR chunk messages.
    pub fn subscribe_asr_chunk(&self) -> Result<DdsSubscription<AsrChunk>> {
        self.rpc.node().subscribe(&lui_asr_chunk_topic(), 16)
    }
}
