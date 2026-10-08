//! Hand-eye calibration RPC client introduced with Booster SDK 1.7.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    dds::{HAND_EYE_CALIB_API_TOPIC, RpcClient, RpcClientOptions},
    types::Result,
};

crate::api_id_enum! {
    /// Hand-eye calibration RPC API identifiers.
    HandEyeCalibApiId {
        StartCalibration = 3100,
        StopCalibration = 3101,
        GetStatus = 3102,
        GetResult = 3103,
        ApplyResult = 3104,
    }
}

/// Options for starting a hand-eye calibration job.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartHandEyeCalibParameter {
    #[serde(default = "default_publish_feedback")]
    pub publish_feedback: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_size_m: Option<f64>,
}

const fn default_publish_feedback() -> bool {
    true
}

impl Default for StartHandEyeCalibParameter {
    fn default() -> Self {
        Self {
            publish_feedback: true,
            square_size_m: None,
        }
    }
}

/// Current calibration job status.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibStatus {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub stage: String,
    #[serde(default)]
    pub started_at: String,
    #[serde(default)]
    pub finished_at: String,
    #[serde(default)]
    pub progress: f64,
    #[serde(default)]
    pub stage2_done: i32,
    #[serde(default)]
    pub stage2_total: i32,
    #[serde(default)]
    pub stage3_done: i32,
    #[serde(default)]
    pub stage3_total: i32,
    #[serde(default)]
    pub error: Option<Value>,
}

/// Calibration result and quality metrics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibResult {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub reprojection_error_px: f64,
    #[serde(default)]
    pub progress: f64,
    #[serde(default)]
    pub stage2_done: i32,
    #[serde(default)]
    pub stage2_total: i32,
    #[serde(default)]
    pub stage3_done: i32,
    #[serde(default)]
    pub stage3_total: i32,
    #[serde(default)]
    pub error: Option<Value>,
}

/// Result of applying calibration data to the robot.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibApplyResult {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub applied_path: String,
    #[serde(default)]
    pub error: Option<Value>,
}

/// Client for the hand-eye calibration service.
pub struct HandEyeCalibClient {
    rpc: RpcClient,
}

impl HandEyeCalibClient {
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(HAND_EYE_CALIB_API_TOPIC))
    }

    pub fn with_startup_wait(startup_wait: Duration) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(HAND_EYE_CALIB_API_TOPIC).with_startup_wait(startup_wait),
        )
    }

    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        Ok(Self {
            rpc: RpcClient::for_topic(options, HAND_EYE_CALIB_API_TOPIC)?,
        })
    }

    pub async fn start_calibration(&self, param: &StartHandEyeCalibParameter) -> Result<()> {
        self.rpc
            .call_serialized(HandEyeCalibApiId::StartCalibration, param)
            .await
    }

    pub async fn stop_calibration(&self) -> Result<()> {
        self.rpc
            .call_void(HandEyeCalibApiId::StopCalibration, "")
            .await
    }

    pub async fn get_status(&self) -> Result<HandEyeCalibStatus> {
        self.rpc
            .call_response(HandEyeCalibApiId::GetStatus, "")
            .await
    }

    pub async fn get_result(&self) -> Result<HandEyeCalibResult> {
        let mut response: HandEyeCalibResult = self
            .rpc
            .call_response(HandEyeCalibApiId::GetResult, "")
            .await?;
        if response.reprojection_error_px == 0.0 {
            response.reprojection_error_px = response
                .result
                .as_ref()
                .and_then(|result| result.get("rms_px"))
                .and_then(Value::as_f64)
                .unwrap_or(0.0);
        }
        Ok(response)
    }

    pub async fn apply_result(&self) -> Result<HandEyeCalibApplyResult> {
        self.rpc
            .call_response(HandEyeCalibApiId::ApplyResult, "")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn default_start_payload_matches_cpp_sdk() {
        assert_eq!(
            serde_json::to_value(StartHandEyeCalibParameter::default()).unwrap(),
            json!({"publish_feedback": true})
        );
    }

    #[test]
    fn result_accepts_extensible_nested_payload() {
        let result: HandEyeCalibResult = serde_json::from_value(json!({
            "status": "success",
            "result": {"rms_px": 0.42, "matrix": [[1.0]]}
        }))
        .unwrap();
        assert_eq!(result.result.unwrap()["rms_px"], 0.42);
    }
}
