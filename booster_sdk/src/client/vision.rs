//! Vision service RPC client.

use std::time::Duration;

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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct StartVisionServiceParameter {
    pub enable_position: bool,
    pub enable_color: bool,
    pub enable_face_detection: bool,
}

/// Parameters for object detection requests.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetDetectionObjectParameter {
    pub focus_ratio: f32,
}

impl Default for GetDetectionObjectParameter {
    fn default() -> Self {
        Self { focus_ratio: 0.33 }
    }
}

/// Single vision detection result.
///
/// Every key is optional on the wire, like the C++ `DetectResults::FromJson`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
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

    /// Target a named robot: requests go to `rt/VisionApiTopic/<robot_name>`
    /// (C++ `Init(robot_name)`). An empty name selects the default topic.
    pub fn with_robot_name(robot_name: impl Into<String>) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(VISION_API_TOPIC).with_robot_name(robot_name),
        )
    }

    /// Create a vision client with a custom startup wait before first RPC.
    pub fn with_startup_wait(startup_wait: Duration) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(VISION_API_TOPIC).with_startup_wait(startup_wait),
        )
    }

    /// Create a vision client with custom RPC options.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::for_topic(options, VISION_API_TOPIC)?;
        Ok(Self { rpc })
    }

    /// Send a raw request and discard the response body (C++ `SendApiRequest`).
    pub async fn send_api_request(&self, api_id: impl Into<i32>, param: &str) -> Result<()> {
        self.rpc.send_api_request(api_id.into(), param).await?;
        Ok(())
    }

    /// Send a raw request and return the response body
    /// (C++ `SendApiRequestWithResponse`).
    pub async fn send_api_request_with_response(
        &self,
        api_id: impl Into<i32>,
        param: &str,
    ) -> Result<String> {
        self.rpc.send_api_request(api_id.into(), param).await
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

        parse_detection_objects(value)
    }

    /// Fetch detected objects with the default focus ratio.
    pub async fn get_detection_object(&self) -> Result<Vec<DetectResults>> {
        self.get_detection_object_with_ratio(GetDetectionObjectParameter::default().focus_ratio)
            .await
    }
}

/// Accept a bare array or `{"objects": [...]}`; anything else is empty.
fn parse_detection_objects(value: Value) -> Result<Vec<DetectResults>> {
    let objects = match value {
        Value::Array(_) => value,
        Value::Object(mut map) => match map.remove("objects") {
            Some(objects @ Value::Array(_)) => objects,
            _ => return Ok(Vec::new()),
        },
        _ => return Ok(Vec::new()),
    };
    Ok(serde_json::from_value(objects)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn detect_results_tolerate_missing_fields() {
        let objects = parse_detection_objects(json!({
            "objects": [{"tag": "ball", "conf": 0.9}, {}]
        }))
        .unwrap();
        assert_eq!(objects.len(), 2);
        assert_eq!(objects[0].tag, "ball");
        assert_eq!(objects[0].xmax, 0);
        assert!(objects[0].position.is_empty());
        assert_eq!(objects[1], DetectResults::default());
    }

    #[test]
    fn detection_body_shapes() {
        assert_eq!(
            parse_detection_objects(json!([{"xmin": 3}])).unwrap()[0].xmin,
            3
        );
        assert!(
            parse_detection_objects(json!({"objects": 1}))
                .unwrap()
                .is_empty()
        );
        assert!(parse_detection_objects(json!({})).unwrap().is_empty());
    }

    #[test]
    fn start_parameter_defaults_to_disabled() {
        let param: StartVisionServiceParameter = serde_json::from_value(json!({})).unwrap();
        assert_eq!(param, StartVisionServiceParameter::default());
        assert!(!param.enable_position);
    }
}
