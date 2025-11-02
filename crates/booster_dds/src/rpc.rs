//! RPC client for high-level API requests.

use booster_types::{DdsError, Result, RpcError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{Duration, Instant};
use uuid::Uuid;
use zenoh::{Config, query::Reply};

/// RPC request header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestHeader {
    /// Unique request ID
    pub request_id: String,

    /// API identifier
    pub api_id: String,
}

/// RPC request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// Unique request ID
    pub header: RequestHeader,

    /// JSON-serialized request body
    pub body: String,
}

/// RPC response header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHeader {
    /// Request ID this response corresponds to
    pub request_id: String,

    /// Status code (0 = success, non-zero = error)
    pub status: i32,
}

/// RPC response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub header: ResponseHeader,

    /// JSON-serialized response body
    pub body: String,
}

/// RPC status codes (matching C++ SDK)
pub mod status {
    pub const SUCCESS: i32 = 0;
    pub const TIMEOUT: i32 = 100;
    pub const BAD_REQUEST: i32 = 400;
    pub const INTERNAL_SERVER_ERROR: i32 = 500;
    pub const SERVER_REFUSED: i32 = 501;
    pub const STATE_TRANSITION_FAILED: i32 = 502;
    pub const INVALID: i32 = -1;
}

/// Configuration options when creating an [`RpcClient`].
#[derive(Debug)]
pub struct RpcClientOptions {
    /// DDS domain identifier used to prefix key expressions.
    pub domain_id: u16,

    /// Default timeout for RPC calls, defaults to 1000 milliseconds.
    pub default_timeout: Duration,

    /// Optional Zenoh configuration to use when opening the session.
    pub config: Option<Config>,
}

impl Default for RpcClientOptions {
    fn default() -> Self {
        Self {
            domain_id: 0,
            default_timeout: Duration::from_millis(1000),
            config: None,
        }
    }
}

/// Async RPC client for high-level robot control using Zenoh queryable/get
pub struct RpcClient {
    /// Zenoh session
    session: Arc<zenoh::Session>,

    /// Default timeout for requests
    default_timeout: Duration,

    /// Service name (e.g., "loco")
    service_name: String,

    /// Domain ID for key expression prefix
    domain_id: u16,
}

impl RpcClient {
    /// Connect to Zenoh and create a new RPC client using the provided options.
    ///
    /// # Errors
    ///
    /// Returns an error if establishing the Zenoh session fails.
    pub async fn connect(
        service_name: impl Into<String>,
        mut options: RpcClientOptions,
    ) -> Result<Self> {
        let config = options.config.take().unwrap_or_default();

        let session = zenoh::open(config)
            .await
            .map_err(|err| DdsError::InitializationFailed(err.to_string()))?;

        Ok(Self {
            session: Arc::new(session),
            default_timeout: options.default_timeout,
            service_name: service_name.into(),
            domain_id: options.domain_id,
        })
    }

    /// Convenience constructor that overrides only the default timeout.
    ///
    /// # Errors
    ///
    /// Propagates any failure from [`Self::connect`].
    pub async fn new(
        service_name: impl Into<String>,
        default_timeout: Option<Duration>,
    ) -> Result<Self> {
        let mut options = RpcClientOptions::default();
        if let Some(timeout) = default_timeout {
            options.default_timeout = timeout;
        }
        Self::connect(service_name, options).await
    }

    /// Construct an RPC client using an existing Zenoh session.
    pub fn from_session(
        session: Arc<zenoh::Session>,
        service_name: impl Into<String>,
        mut options: RpcClientOptions,
    ) -> Self {
        // Configuration is not used when the caller provides the session.
        options.config = None;

        Self {
            session,
            default_timeout: options.default_timeout,
            service_name: service_name.into(),
            domain_id: options.domain_id,
        }
    }

    /// Build key expression for RPC
    fn build_key_expr(&self, api_id: &str) -> String {
        format!(
            "domain{}/rpc/{}/{}",
            self.domain_id, self.service_name, api_id
        )
    }

    /// Send an RPC request and wait for response.
    ///
    /// Zenoh may multiplex replies for multiple outstanding requests. This method filters
    /// out responses whose `request_id` does not match the one generated for the current
    /// call while the timeout window is still open.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails, the request cannot be delivered, or the
    /// remote side replies with a failure status.
    pub async fn call<P, R>(&self, api_id: &str, params: &P, timeout: Option<Duration>) -> Result<R>
    where
        P: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let request_id = Uuid::new_v4().to_string();

        tracing::debug!("RPC call: {} (request_id: {})", api_id, request_id);

        // Serialize parameters to JSON
        let body = serde_json::to_string(params).map_err(|e| {
            RpcError::BadRequest(format!("Failed to serialize request parameters: {e}"))
        })?;

        let request = Request {
            header: RequestHeader {
                request_id: request_id.clone(),
                api_id: api_id.to_string(),
            },
            body,
        };

        // Serialize request to JSON
        let request_bytes = serde_json::to_vec(&request)
            .map_err(|err| RpcError::BadRequest(format!("Failed to serialize request: {err}")))?;

        // Build key expression for this API
        let key_expr = self.build_key_expr(api_id);

        let timeout = timeout.unwrap_or(self.default_timeout);

        tracing::debug!(
            "GET request to key: {} with timeout: {:?}",
            key_expr,
            timeout
        );

        let replies = self
            .session
            .get(&key_expr)
            .payload(request_bytes)
            .timeout(timeout)
            .await
            .map_err(|err| RpcError::BadRequest(format!("Failed to send request: {err}")))?;

        let deadline = Instant::now() + timeout;

        loop {
            let now = Instant::now();
            let remaining = match deadline.checked_duration_since(now) {
                Some(d) if !d.is_zero() => d,
                _ => {
                    tracing::warn!(
                        "Timed out waiting for RPC reply (api: {}, request_id: {})",
                        api_id,
                        request_id
                    );
                    return Err(RpcError::Timeout { timeout }.into());
                }
            };

            let reply_future = replies.recv_async();
            match tokio::time::timeout(remaining, reply_future).await {
                Err(_) => {
                    tracing::warn!(
                        "Timed out waiting for RPC reply (api: {}, request_id: {})",
                        api_id,
                        request_id
                    );
                    return Err(RpcError::Timeout { timeout }.into());
                }
                Ok(Err(err)) => {
                    return Err(DdsError::ReceiveFailed(err.to_string()).into());
                }
                Ok(Ok(reply)) => {
                    let response = Self::parse_reply(&reply)?;

                    if response.header.request_id != request_id {
                        tracing::warn!(
                            "Ignoring mismatched RPC reply. expected={}, got={}",
                            request_id,
                            response.header.request_id
                        );
                        continue;
                    }

                    if response.header.status != status::SUCCESS {
                        return Err(RpcError::from_status_code(
                            response.header.status,
                            response.body,
                        )
                        .into());
                    }

                    let result: R = serde_json::from_str(&response.body).map_err(|err| {
                        RpcError::RequestFailed {
                            status: response.header.status,
                            message: format!("Failed to deserialize response: {err}"),
                        }
                    })?;

                    return Ok(result);
                }
            }
        }
    }

    /// Parse a Zenoh reply into an RPC response
    fn parse_reply(reply: &Reply) -> Result<Response> {
        match reply.result() {
            Ok(sample) => {
                let payload_bytes = sample.payload().to_bytes();
                let response: Response = serde_json::from_slice(&payload_bytes).map_err(|err| {
                    RpcError::InternalServerError(format!("Failed to deserialize response: {err}"))
                })?;
                Ok(response)
            }
            Err(err) => {
                Err(RpcError::InternalServerError(format!("Reply contained error: {err:?}")).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_codes() {
        assert_eq!(status::SUCCESS, 0);
        assert_eq!(status::TIMEOUT, 100);
        assert_eq!(status::BAD_REQUEST, 400);
    }

    #[test]
    fn test_request_serialization() {
        let request = Request {
            header: RequestHeader {
                request_id: "test-123".to_string(),
                api_id: "TestApi".to_string(),
            },
            body: "{}".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: Request = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.header.request_id, deserialized.header.request_id);
        assert_eq!(request.header.api_id, deserialized.header.api_id);
    }
}
