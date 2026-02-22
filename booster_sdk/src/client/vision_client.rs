//! Vision service RPC client.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::dds::{RpcClient, RpcClientOptions, VISION_API_TOPIC};
use crate::types::Result;

use super::util::{EmptyResponse, serialize_param};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum VisionApiId {
    StartVisionService = 3000,
    StopVisionService = 3001,
    GetDetectionObject = 3002,
}

impl From<VisionApiId> for i32 {
    fn from(value: VisionApiId) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for VisionApiId {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            3000 => Ok(Self::StartVisionService),
            3001 => Ok(Self::StopVisionService),
            3002 => Ok(Self::GetDetectionObject),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartVisionServiceParameter {
    pub enable_position: bool,
    pub enable_color: bool,
    pub enable_face_detection: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GetDetectionObjectParameter {
    pub focus_ratio: f32,
}

impl Default for GetDetectionObjectParameter {
    fn default() -> Self {
        Self { focus_ratio: 0.33 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectResults {
    pub xmin: i64,
    pub ymin: i64,
    pub xmax: i64,
    pub ymax: i64,
    pub position: Vec<f32>,
    pub tag: String,
    pub conf: f32,
    pub rgb_mean: Vec<i32>,
}

/// High-level RPC client for vision inference APIs.
pub struct VisionClient {
    rpc: RpcClient,
}

impl VisionClient {
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(VISION_API_TOPIC))
    }

    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::new(options.with_service_topic(VISION_API_TOPIC))?;
        Ok(Self { rpc })
    }

    pub async fn send_api_request(&self, api_id: VisionApiId, param: &str) -> Result<()> {
        self.rpc
            .call_with_body::<EmptyResponse>(i32::from(api_id), param.to_owned(), None)
            .await?;
        Ok(())
    }

    pub async fn send_api_request_with_response<R>(
        &self,
        api_id: VisionApiId,
        param: &str,
    ) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        self.rpc
            .call_with_body(i32::from(api_id), param.to_owned(), None)
            .await
    }

    pub async fn start_vision_service(
        &self,
        enable_position: bool,
        enable_color: bool,
        enable_face_detection: bool,
    ) -> Result<()> {
        let param = StartVisionServiceParameter {
            enable_position,
            enable_color,
            enable_face_detection,
        };
        self.send_api_request(VisionApiId::StartVisionService, &serialize_param(&param)?)
            .await
    }

    pub async fn stop_vision_service(&self) -> Result<()> {
        self.send_api_request(VisionApiId::StopVisionService, "{}")
            .await
    }

    pub async fn get_detection_object_with_ratio(
        &self,
        focus_ratio: f32,
    ) -> Result<Vec<DetectResults>> {
        let param = GetDetectionObjectParameter { focus_ratio };
        let value: Value = self
            .send_api_request_with_response(
                VisionApiId::GetDetectionObject,
                &serialize_param(&param)?,
            )
            .await?;

        if value.is_array() {
            return Ok(serde_json::from_value(value)?);
        }

        if let Some(objects) = value.get("objects") {
            return Ok(serde_json::from_value(objects.clone())?);
        }

        Ok(Vec::new())
    }

    pub async fn get_detection_object(&self) -> Result<Vec<DetectResults>> {
        self.get_detection_object_with_ratio(GetDetectionObjectParameter::default().focus_ratio)
            .await
    }
}
