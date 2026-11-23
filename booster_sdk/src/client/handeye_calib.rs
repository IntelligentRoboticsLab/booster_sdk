//! Hand-eye calibration RPC client introduced with Booster SDK 1.7.

use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize};
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

/// Deserialize a field, mapping an explicit JSON `null` to `T::default()`.
///
/// The C++ SDK skips keys whose value is `null`; combined with
/// `#[serde(default)]` this keeps the default for both missing and `null`.
fn null_as_default<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

fn null_as_true<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<bool>::deserialize(deserializer)?.unwrap_or(true))
}

/// The C++ SDK only emits `square_size_m` when it is greater than zero.
#[allow(clippy::ref_option)]
fn skip_square_size(value: &Option<f64>) -> bool {
    !value.is_some_and(|v| v > 0.0)
}

/// Options for starting a hand-eye calibration job.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartHandEyeCalibParameter {
    #[serde(
        default = "default_publish_feedback",
        deserialize_with = "null_as_true"
    )]
    pub publish_feedback: bool,
    /// Checkerboard square size in meters. Only sent when greater than zero;
    /// otherwise the service uses its configured default.
    #[serde(default, skip_serializing_if = "skip_square_size")]
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
    #[serde(default, deserialize_with = "null_as_default")]
    pub status: String,
    #[serde(default, deserialize_with = "null_as_default")]
    pub job_id: String,
    #[serde(default, deserialize_with = "null_as_default")]
    pub stage: String,
    #[serde(default, deserialize_with = "null_as_default")]
    pub started_at: String,
    #[serde(default, deserialize_with = "null_as_default")]
    pub finished_at: String,
    #[serde(default, deserialize_with = "null_as_default")]
    pub progress: f64,
    #[serde(default, deserialize_with = "null_as_default")]
    pub stage2_done: i32,
    #[serde(default, deserialize_with = "null_as_default")]
    pub stage2_total: i32,
    #[serde(default, deserialize_with = "null_as_default")]
    pub stage3_done: i32,
    #[serde(default, deserialize_with = "null_as_default")]
    pub stage3_total: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

/// Calibration result and quality metrics.
///
/// `reprojection_error_px` follows the C++ precedence: a non-null top-level
/// `reprojection_error_px` wins (even when `0`), otherwise a non-null
/// `result.rms_px` is used, otherwise `0.0`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(from = "RawHandEyeCalibResult")]
pub struct HandEyeCalibResult {
    pub status: String,
    pub job_id: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    pub reprojection_error_px: f64,
    pub progress: f64,
    pub stage2_done: i32,
    pub stage2_total: i32,
    pub stage3_done: i32,
    pub stage3_total: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

/// Wire form of [`HandEyeCalibResult`]; tracks whether the top-level
/// `reprojection_error_px` key was present.
#[derive(Deserialize)]
struct RawHandEyeCalibResult {
    #[serde(default, deserialize_with = "null_as_default")]
    status: String,
    #[serde(default, deserialize_with = "null_as_default")]
    job_id: String,
    #[serde(default, deserialize_with = "null_as_default")]
    summary: String,
    #[serde(default)]
    result: Option<Value>,
    #[serde(default)]
    reprojection_error_px: Option<f64>,
    #[serde(default, deserialize_with = "null_as_default")]
    progress: f64,
    #[serde(default, deserialize_with = "null_as_default")]
    stage2_done: i32,
    #[serde(default, deserialize_with = "null_as_default")]
    stage2_total: i32,
    #[serde(default, deserialize_with = "null_as_default")]
    stage3_done: i32,
    #[serde(default, deserialize_with = "null_as_default")]
    stage3_total: i32,
    #[serde(default)]
    error: Option<Value>,
}

impl From<RawHandEyeCalibResult> for HandEyeCalibResult {
    fn from(raw: RawHandEyeCalibResult) -> Self {
        let rms_px = raw
            .result
            .as_ref()
            .and_then(|result| result.get("rms_px"))
            .and_then(Value::as_f64);
        Self {
            status: raw.status,
            job_id: raw.job_id,
            summary: raw.summary,
            reprojection_error_px: raw.reprojection_error_px.or(rms_px).unwrap_or(0.0),
            result: raw.result,
            progress: raw.progress,
            stage2_done: raw.stage2_done,
            stage2_total: raw.stage2_total,
            stage3_done: raw.stage3_done,
            stage3_total: raw.stage3_total,
            error: raw.error,
        }
    }
}

/// Result of applying calibration data to the robot.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibApplyResult {
    #[serde(default, deserialize_with = "null_as_default")]
    pub status: String,
    #[serde(default, deserialize_with = "null_as_default")]
    pub applied_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

    /// Target a named robot: requests go to
    /// `rt/HandEyeCalibApiTopic/<robot_name>` (C++ `Init(robot_name)`).
    /// An empty name selects the default topic.
    pub fn with_robot_name(robot_name: impl Into<String>) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(HAND_EYE_CALIB_API_TOPIC).with_robot_name(robot_name),
        )
    }

    pub fn with_startup_wait(startup_wait: Duration) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(HAND_EYE_CALIB_API_TOPIC).with_startup_wait(startup_wait),
        )
    }

    /// Create a client with custom RPC options. The service topic is always
    /// the hand-eye topic; `options.robot_name` is honored.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        Ok(Self {
            rpc: RpcClient::for_topic(options, HAND_EYE_CALIB_API_TOPIC)?,
        })
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

    pub async fn start_calibration(&self, param: &StartHandEyeCalibParameter) -> Result<()> {
        self.rpc
            .call_serialized(HandEyeCalibApiId::StartCalibration, param)
            .await
    }

    // The C++ client sends `{}` as the body of every parameterless request.

    pub async fn stop_calibration(&self) -> Result<()> {
        self.rpc
            .call_void(HandEyeCalibApiId::StopCalibration, "{}")
            .await
    }

    pub async fn get_status(&self) -> Result<HandEyeCalibStatus> {
        self.rpc
            .call_response(HandEyeCalibApiId::GetStatus, "{}")
            .await
    }

    pub async fn get_result(&self) -> Result<HandEyeCalibResult> {
        self.rpc
            .call_response(HandEyeCalibApiId::GetResult, "{}")
            .await
    }

    pub async fn apply_result(&self) -> Result<HandEyeCalibApplyResult> {
        self.rpc
            .call_response(HandEyeCalibApiId::ApplyResult, "{}")
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
        assert_eq!(result.reprojection_error_px, 0.42);
        assert_eq!(result.result.unwrap()["rms_px"], 0.42);
    }

    #[test]
    fn tolerates_explicit_nulls() {
        let status: HandEyeCalibStatus = serde_json::from_value(json!({
            "status": null, "job_id": null, "stage": null, "started_at": null,
            "finished_at": null, "progress": null, "stage2_done": null,
            "stage2_total": null, "stage3_done": null, "stage3_total": null,
            "error": null
        }))
        .unwrap();
        assert_eq!(status, HandEyeCalibStatus::default());

        let result: HandEyeCalibResult = serde_json::from_value(json!({
            "status": "running", "job_id": null, "summary": null, "result": null,
            "reprojection_error_px": null, "progress": null, "stage2_done": null,
            "stage2_total": null, "stage3_done": null, "stage3_total": null, "error": null
        }))
        .unwrap();
        assert_eq!(result.status, "running");
        assert_eq!(result.reprojection_error_px, 0.0);
        assert!(result.result.is_none());

        let apply: HandEyeCalibApplyResult = serde_json::from_value(json!({
            "status": null, "applied_path": null, "error": null
        }))
        .unwrap();
        assert_eq!(apply, HandEyeCalibApplyResult::default());

        let start: StartHandEyeCalibParameter = serde_json::from_value(json!({
            "publish_feedback": null, "square_size_m": null
        }))
        .unwrap();
        assert_eq!(start, StartHandEyeCalibParameter::default());
    }

    #[test]
    fn reprojection_error_precedence_matches_cpp() {
        // A present top-level key wins even when it is zero.
        let result: HandEyeCalibResult = serde_json::from_value(json!({
            "reprojection_error_px": 0.0,
            "result": {"rms_px": 0.42}
        }))
        .unwrap();
        assert_eq!(result.reprojection_error_px, 0.0);

        let result: HandEyeCalibResult = serde_json::from_value(json!({
            "reprojection_error_px": 1.5,
            "result": {"rms_px": 0.42}
        }))
        .unwrap();
        assert_eq!(result.reprojection_error_px, 1.5);

        // A null top-level key falls back to result.rms_px.
        let result: HandEyeCalibResult = serde_json::from_value(json!({
            "reprojection_error_px": null,
            "result": {"rms_px": 0.42}
        }))
        .unwrap();
        assert_eq!(result.reprojection_error_px, 0.42);

        let result: HandEyeCalibResult =
            serde_json::from_value(json!({"result": {"rms_px": null}})).unwrap();
        assert_eq!(result.reprojection_error_px, 0.0);
    }

    #[test]
    fn square_size_only_sent_when_positive() {
        for (size, expected) in [
            (
                Some(0.025),
                json!({"publish_feedback": true, "square_size_m": 0.025}),
            ),
            (Some(0.0), json!({"publish_feedback": true})),
            (Some(-1.0), json!({"publish_feedback": true})),
            (None, json!({"publish_feedback": true})),
        ] {
            let param = StartHandEyeCalibParameter {
                publish_feedback: true,
                square_size_m: size,
            };
            assert_eq!(serde_json::to_value(param).unwrap(), expected);
        }
    }

    #[test]
    fn result_omits_empty_optional_payloads() {
        let json = serde_json::to_value(HandEyeCalibResult::default()).unwrap();
        assert!(json.get("result").is_none());
        assert!(json.get("error").is_none());
        assert_eq!(json["reprojection_error_px"], 0.0);
    }
}
