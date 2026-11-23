//! Error types for the Booster Robotics SDK.

use std::time::Duration;

use thiserror::Error;

/// Main error type for the Booster SDK.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BoosterError {
    #[error("DDS error: {0}")]
    Dds(#[from] DdsError),

    #[error("RPC error: {0}")]
    Rpc(#[from] RpcError),

    #[error("Command error: {0}")]
    Command(#[from] CommandError),

    #[error("State error: {0}")]
    State(#[from] StateError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Operation timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

/// DDS-specific errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DdsError {
    #[error("Failed to initialize DDS: {0}")]
    InitializationFailed(String),

    #[error("Failed to create publisher for topic '{topic}': {reason}")]
    PublisherCreationFailed { topic: String, reason: String },

    #[error("Failed to create subscriber for topic '{topic}': {reason}")]
    SubscriberCreationFailed { topic: String, reason: String },

    #[error("Failed to publish message: {0}")]
    PublishFailed(String),

    #[error("Failed to receive message: {0}")]
    ReceiveFailed(String),

    #[error("DDS participant not initialized")]
    NotInitialized,
}

/// RPC status values shared by SDK services (C++ `error.hpp`).
///
/// [`RPC_STATUS_CODE_INVALID`] (-1) means "no status yet": a response carrying
/// it is not a final answer, so the RPC client keeps waiting for the real
/// response (the C++ client does the same).
pub const RPC_STATUS_CODE_INVALID: i32 = -1;
/// Request completed successfully.
pub const RPC_STATUS_CODE_SUCCESS: i32 = 0;
/// Request or endpoint wait timed out.
pub const RPC_STATUS_CODE_TIMEOUT: i32 = 100;
/// Request parameters are invalid.
pub const RPC_STATUS_CODE_BAD_REQUEST: i32 = 400;
/// Request conflicts with current service state.
pub const RPC_STATUS_CODE_CONFLICT: i32 = 409;
/// Request rate exceeded service limits.
pub const RPC_STATUS_CODE_REQUEST_TOO_FREQUENT: i32 = 429;
/// Service-side internal failure.
pub const RPC_STATUS_CODE_INTERNAL_SERVER_ERROR: i32 = 500;
/// Service refused the request.
pub const RPC_STATUS_CODE_SERVER_REFUSED: i32 = 501;
/// Robot state-machine transition failed.
pub const RPC_STATUS_CODE_STATE_TRANSITION_FAILED: i32 = 502;
/// Motion request refused because the battery is low.
pub const RPC_STATUS_CODE_LOW_BATTERY: i32 = 503;

/// RPC-specific errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RpcError {
    #[error("RPC request timed out after {timeout:?}")]
    Timeout { timeout: Duration },

    /// The service (or endpoint discovery) reported status 100 (timeout).
    #[error("RPC timed out: {0}")]
    RemoteTimeout(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Server refused request: {0}")]
    ServerRefused(String),

    #[error("Request conflicts with current robot state: {0}")]
    Conflict(String),

    #[error("Request rejected because it is too frequent: {0}")]
    RequestTooFrequent(String),

    #[error("State transition failed: {0}")]
    StateTransitionFailed(String),

    #[error("Motion request refused because the battery is low: {0}")]
    LowBattery(String),

    #[error("Invalid RPC status code: {0}")]
    InvalidStatusCode(i32),

    #[error("Request failed with status {status}: {message}")]
    RequestFailed { status: i32, message: String },
}

impl RpcError {
    /// Convert from RPC status code.
    #[inline]
    #[must_use]
    pub fn from_status_code(code: i32, message: String) -> Self {
        match code {
            RPC_STATUS_CODE_TIMEOUT => RpcError::RemoteTimeout(message),
            RPC_STATUS_CODE_BAD_REQUEST => RpcError::BadRequest(message),
            RPC_STATUS_CODE_CONFLICT => RpcError::Conflict(message),
            RPC_STATUS_CODE_REQUEST_TOO_FREQUENT => RpcError::RequestTooFrequent(message),
            RPC_STATUS_CODE_INTERNAL_SERVER_ERROR => RpcError::InternalServerError(message),
            RPC_STATUS_CODE_SERVER_REFUSED => RpcError::ServerRefused(message),
            RPC_STATUS_CODE_STATE_TRANSITION_FAILED => RpcError::StateTransitionFailed(message),
            RPC_STATUS_CODE_LOW_BATTERY => RpcError::LowBattery(message),
            // -1 (invalid / no status) and unknown codes keep their raw value.
            _ => RpcError::RequestFailed {
                status: code,
                message,
            },
        }
    }

    /// The C++ status code equivalent of this error.
    ///
    /// Local timeouts map to [`RPC_STATUS_CODE_TIMEOUT`], like the C++ client.
    #[must_use]
    pub fn status_code(&self) -> i32 {
        match self {
            RpcError::Timeout { .. } | RpcError::RemoteTimeout(_) => RPC_STATUS_CODE_TIMEOUT,
            RpcError::BadRequest(_) => RPC_STATUS_CODE_BAD_REQUEST,
            RpcError::InternalServerError(_) => RPC_STATUS_CODE_INTERNAL_SERVER_ERROR,
            RpcError::ServerRefused(_) => RPC_STATUS_CODE_SERVER_REFUSED,
            RpcError::Conflict(_) => RPC_STATUS_CODE_CONFLICT,
            RpcError::RequestTooFrequent(_) => RPC_STATUS_CODE_REQUEST_TOO_FREQUENT,
            RpcError::StateTransitionFailed(_) => RPC_STATUS_CODE_STATE_TRANSITION_FAILED,
            RpcError::LowBattery(_) => RPC_STATUS_CODE_LOW_BATTERY,
            RpcError::InvalidStatusCode(code) => *code,
            RpcError::RequestFailed { status, .. } => *status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_codes_round_trip() {
        for code in [
            RPC_STATUS_CODE_TIMEOUT,
            RPC_STATUS_CODE_BAD_REQUEST,
            RPC_STATUS_CODE_CONFLICT,
            RPC_STATUS_CODE_REQUEST_TOO_FREQUENT,
            RPC_STATUS_CODE_INTERNAL_SERVER_ERROR,
            RPC_STATUS_CODE_SERVER_REFUSED,
            RPC_STATUS_CODE_STATE_TRANSITION_FAILED,
            RPC_STATUS_CODE_LOW_BATTERY,
            RPC_STATUS_CODE_INVALID,
            1234,
        ] {
            assert_eq!(
                RpcError::from_status_code(code, String::new()).status_code(),
                code
            );
        }
    }

    #[test]
    fn timeout_status_keeps_message() {
        let err = RpcError::from_status_code(RPC_STATUS_CODE_TIMEOUT, "endpoints".to_owned());
        assert!(matches!(err, RpcError::RemoteTimeout(ref m) if m == "endpoints"));
    }
}

/// Command execution errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CommandError {
    #[error("Invalid mode transition from {from:?} to {to:?}")]
    InvalidModeTransition { from: String, to: String },

    #[error("Command parameter out of range: {parameter} = {value} (valid range: {min} to {max})")]
    ParameterOutOfRange {
        parameter: String,
        value: f32,
        min: f32,
        max: f32,
    },

    #[error("Robot not in correct mode for command '{command}': current mode is {current_mode:?}")]
    InvalidMode {
        command: String,
        current_mode: String,
    },

    #[error("Joint index {index} out of range (valid: 0-{max})")]
    InvalidJointIndex { index: usize, max: usize },

    #[error("Command not supported: {0}")]
    NotSupported(String),
}

/// State reading errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StateError {
    #[error("No state data available")]
    NoDataAvailable,

    #[error("State data is stale (last update: {last_update_ms}ms ago)")]
    StaleData { last_update_ms: u64 },

    #[error("Invalid state data: {0}")]
    InvalidData(String),

    #[error("Frame '{0}' not found")]
    FrameNotFound(String),
}

/// Result type alias for Booster SDK operations
pub type Result<T> = std::result::Result<T, BoosterError>;
