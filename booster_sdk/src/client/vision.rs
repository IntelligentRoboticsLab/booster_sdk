//! Vision service RPC client.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::dds::{RpcClient, RpcClientOptions, VISION_API_TOPIC};
use crate::types::Result;

crate::api_id_enum! {
    /// Vision service RPC API identifiers.
    VisionApiId {
        StartVisionService = 3000,
        StopVisionService = 3001,
        GetDetectionObject = 3002,
    }
}

/// Parameters for starting the vision service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartVisionServiceParameter {
    pub enable_position: bool,
    pub enable_color: bool,
    pub enable_face_detection: bool,
}

/// Parameters for object detection requests.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GetDetectionObjectParameter {
    pub focus_ratio: f32,
}

impl Default for GetDetectionObjectParameter {
    fn default() -> Self {
        Self { focus_ratio: 0.33 }
    }
}

/// Single vision detection result.
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
    /// Create a vision client with default options.
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(VISION_API_TOPIC))
    }

    /// Create a vision client with custom RPC options.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::for_topic(options, VISION_API_TOPIC)?;
        Ok(Self { rpc })
    }

    /// Start the vision service with selected features.
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
        self.rpc
            .call_serialized(VisionApiId::StartVisionService, &param)
            .await
    }

    /// Stop the vision service.
    pub async fn stop_vision_service(&self) -> Result<()> {
        self.rpc
            .call_void(VisionApiId::StopVisionService, "{}")
            .await
    }

    /// Fetch detected objects with a custom focus ratio.
    pub async fn get_detection_object_with_ratio(
        &self,
        focus_ratio: f32,
    ) -> Result<Vec<DetectResults>> {
        let param = GetDetectionObjectParameter { focus_ratio };
        let value: Value = self
            .rpc
            .call_serialized_response(VisionApiId::GetDetectionObject, &param)
            .await?;

        if value.is_array() {
            return Ok(serde_json::from_value(value)?);
        }

        if let Some(objects) = value.get("objects") {
            return Ok(serde_json::from_value(objects.clone())?);
        }

        Ok(Vec::new())
    }

    /// Fetch detected objects with the default focus ratio.
    pub async fn get_detection_object(&self) -> Result<Vec<DetectResults>> {
        self.get_detection_object_with_ratio(GetDetectionObjectParameter::default().focus_ratio)
            .await
    }
}
