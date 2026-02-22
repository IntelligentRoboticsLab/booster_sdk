//! AI and LUI high-level RPC clients.

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::dds::{
    AI_API_TOPIC, DdsNode, DdsSubscription, LUI_API_TOPIC, RpcClient, RpcClientOptions,
    ai_subtitle_topic, lui_asr_chunk_topic,
};
use crate::types::Result;

use super::util::{EmptyResponse, serialize_param};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum AiApiId {
    StartAiChat = 2000,
    StopAiChat = 2001,
    Speak = 2002,
    StartFaceTracking = 2003,
    StopFaceTracking = 2004,
}

impl From<AiApiId> for i32 {
    fn from(value: AiApiId) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for AiApiId {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            2000 => Ok(Self::StartAiChat),
            2001 => Ok(Self::StopAiChat),
            2002 => Ok(Self::Speak),
            2003 => Ok(Self::StartFaceTracking),
            2004 => Ok(Self::StopFaceTracking),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum LuiApiId {
    StartAsr = 1000,
    StopAsr = 1001,
    StartTts = 1050,
    StopTts = 1051,
    SendTtsText = 1052,
}

impl From<LuiApiId> for i32 {
    fn from(value: LuiApiId) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for LuiApiId {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            1000 => Ok(Self::StartAsr),
            1001 => Ok(Self::StopAsr),
            1050 => Ok(Self::StartTts),
            1051 => Ok(Self::StopTts),
            1052 => Ok(Self::SendTtsText),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TtsConfig {
    pub voice_type: String,
    pub ignore_bracket_text: Vec<i8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmConfig {
    pub system_prompt: String,
    pub welcome_msg: String,
    pub prompt_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsrConfig {
    pub interrupt_speech_duration: i32,
    pub interrupt_keywords: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartAiChatParameter {
    pub interrupt_mode: bool,
    pub asr_config: AsrConfig,
    pub llm_config: LlmConfig,
    pub tts_config: TtsConfig,
    pub enable_face_tracking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakParameter {
    pub msg: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LuiTtsConfig {
    pub voice_type: String,
}

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

pub const BOOSTER_ROBOT_USER_ID: &str = "BoosterRobot";

/// High-level RPC client for AI chat features.
pub struct AiClient {
    rpc: RpcClient,
    node: DdsNode,
}

impl AiClient {
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(AI_API_TOPIC))
    }

    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::new(options.with_service_topic(AI_API_TOPIC))?;
        let node = rpc.node().clone();
        Ok(Self { rpc, node })
    }

    pub fn node(&self) -> &DdsNode {
        &self.node
    }

    pub async fn send_api_request(&self, api_id: AiApiId, param: &str) -> Result<()> {
        self.rpc
            .call_with_body::<EmptyResponse>(i32::from(api_id), param.to_owned(), None)
            .await?;
        Ok(())
    }

    pub async fn send_api_request_with_response<R>(&self, api_id: AiApiId, param: &str) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        self.rpc
            .call_with_body(i32::from(api_id), param.to_owned(), None)
            .await
    }

    pub async fn start_ai_chat(&self, param: &StartAiChatParameter) -> Result<()> {
        self.send_api_request(AiApiId::StartAiChat, &serialize_param(param)?)
            .await
    }

    pub async fn stop_ai_chat(&self) -> Result<()> {
        self.send_api_request(AiApiId::StopAiChat, "").await
    }

    pub async fn speak(&self, param: &SpeakParameter) -> Result<()> {
        self.send_api_request(AiApiId::Speak, &serialize_param(param)?)
            .await
    }

    pub async fn start_face_tracking(&self) -> Result<()> {
        self.send_api_request(AiApiId::StartFaceTracking, "").await
    }

    pub async fn stop_face_tracking(&self) -> Result<()> {
        self.send_api_request(AiApiId::StopFaceTracking, "").await
    }

    pub fn subscribe_subtitle(&self) -> Result<DdsSubscription<Subtitle>> {
        self.node.subscribe(&ai_subtitle_topic(), 16)
    }
}

/// High-level RPC client for LUI ASR/TTS features.
pub struct LuiClient {
    rpc: RpcClient,
    node: DdsNode,
}

impl LuiClient {
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(LUI_API_TOPIC))
    }

    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::new(options.with_service_topic(LUI_API_TOPIC))?;
        let node = rpc.node().clone();
        Ok(Self { rpc, node })
    }

    pub fn node(&self) -> &DdsNode {
        &self.node
    }

    pub async fn send_api_request(&self, api_id: LuiApiId, param: &str) -> Result<()> {
        self.rpc
            .call_with_body::<EmptyResponse>(i32::from(api_id), param.to_owned(), None)
            .await?;
        Ok(())
    }

    pub async fn send_api_request_with_response<R>(
        &self,
        api_id: LuiApiId,
        param: &str,
    ) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        self.rpc
            .call_with_body(i32::from(api_id), param.to_owned(), None)
            .await
    }

    pub async fn start_asr(&self) -> Result<()> {
        self.send_api_request(LuiApiId::StartAsr, "").await
    }

    pub async fn stop_asr(&self) -> Result<()> {
        self.send_api_request(LuiApiId::StopAsr, "").await
    }

    pub async fn start_tts(&self, config: &LuiTtsConfig) -> Result<()> {
        self.send_api_request(LuiApiId::StartTts, &serialize_param(config)?)
            .await
    }

    pub async fn stop_tts(&self) -> Result<()> {
        self.send_api_request(LuiApiId::StopTts, "").await
    }

    pub async fn send_tts_text(&self, param: &LuiTtsParameter) -> Result<()> {
        self.send_api_request(LuiApiId::SendTtsText, &serialize_param(param)?)
            .await
    }

    pub fn subscribe_asr_chunk(&self) -> Result<DdsSubscription<AsrChunk>> {
        self.node.subscribe(&lui_asr_chunk_topic(), 16)
    }
}
