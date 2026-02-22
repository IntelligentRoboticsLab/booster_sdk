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

/// RPC-specific errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RpcError {
    #[error("RPC request timed out after {timeout:?}")]
    Timeout { timeout: Duration },

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
            100 => RpcError::Timeout {
                timeout: Duration::ZERO,
            },
            400 => RpcError::BadRequest(message),
            409 => RpcError::Conflict(message),
            429 => RpcError::RequestTooFrequent(message),
            500 => RpcError::InternalServerError(message),
            501 => RpcError::ServerRefused(message),
            502 => RpcError::StateTransitionFailed(message),
            _ => RpcError::RequestFailed {
                status: code,
                message,
            },
        }
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
