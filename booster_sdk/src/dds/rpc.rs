//! RPC client for high-level API requests over DDS.

use futures::StreamExt;
use rustdds::no_key::DataReaderStream;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::types::{DdsError, Result, RpcError};

use super::DdsNode;
use super::messages::{RpcReqMsg, RpcRespMsg};
use super::topics::{LOCO_API_TOPIC, rpc_request_topic, rpc_response_topic};

#[derive(Debug)]
pub struct RpcClientOptions {
    pub domain_id: u16,
    pub default_timeout: Duration,
    pub startup_wait: Duration,
    pub service_topic: String,
}

impl Default for RpcClientOptions {
    fn default() -> Self {
        Self {
            domain_id: 0,
            // 5 s is a safe default for most commands. Mode changes are slow,
            // so change_mode passes its own longer timeout.
            default_timeout: Duration::from_secs(5),
            // Wait once before the first RPC call so endpoint discovery can settle.
            startup_wait: Duration::from_millis(3000),
            service_topic: LOCO_API_TOPIC.to_owned(),
        }
    }
}

impl RpcClientOptions {
    #[must_use]
    pub fn for_service(service_topic: impl Into<String>) -> Self {
        Self {
            service_topic: service_topic.into(),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn with_service_topic(mut self, service_topic: impl Into<String>) -> Self {
        self.service_topic = service_topic.into();
        self
    }

    #[must_use]
    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    #[must_use]
    pub fn with_startup_wait(mut self, startup_wait: Duration) -> Self {
        self.startup_wait = startup_wait;
        self
    }

    #[must_use]
    pub fn without_startup_wait(self) -> Self {
        self.with_startup_wait(Duration::from_millis(0))
    }
}

pub struct RpcClient {
    node: DdsNode,
    request_writer: rustdds::no_key::DataWriter<RpcReqMsg>,
    response_stream: Mutex<DataReaderStream<RpcRespMsg>>,
    default_timeout: Duration,
    startup_wait: Duration,
    startup_wait_done: AtomicBool,
    service_topic: String,
}

#[derive(Debug, Deserialize, Default)]
struct EmptyResponse {}

fn parse_status_value(value: &Value) -> Option<i32> {
    match value {
        Value::Number(n) => n.as_i64().and_then(|v| i32::try_from(v).ok()),
        Value::String(s) => s.parse::<i32>().ok(),
        _ => None,
    }
}

fn parse_status_from_header(raw_json: &str) -> Option<i32> {
    let value: Value = serde_json::from_str(raw_json.trim()).ok()?;
    let object = value.as_object()?;
    object.get("status").and_then(parse_status_value)
}

fn decode_response_body<R>(body: &str) -> std::result::Result<R, serde_json::Error>
where
    R: DeserializeOwned,
{
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return serde_json::from_str("{}");
    }

    serde_json::from_str(trimmed)
}

fn normalize_service_topic(service_topic: &str) -> String {
    let trimmed = service_topic.trim();
    if trimmed.is_empty() {
        return LOCO_API_TOPIC.to_owned();
    }
    if let Some(base) = trimmed.strip_suffix("Req") {
        return base.to_owned();
    }
    if let Some(base) = trimmed.strip_suffix("Resp") {
        return base.to_owned();
    }
    trimmed.to_owned()
}

fn preview_for_log(value: &str, max_chars: usize) -> String {
    let mut preview = String::new();
    let mut chars = value.chars();
    for _ in 0..max_chars {
        match chars.next() {
            Some(ch) => preview.push(ch),
            None => return preview.replace('\n', "\\n"),
        }
    }
    if chars.next().is_some() {
        preview.push_str("...");
    }
    preview.replace('\n', "\\n")
}

impl RpcClient {
    pub fn for_topic(options: RpcClientOptions, service_topic: impl Into<String>) -> Result<Self> {
        Self::new(options.with_service_topic(service_topic))
    }

    pub fn new(options: RpcClientOptions) -> Result<Self> {
        let node = DdsNode::new(super::DdsConfig {
            domain_id: options.domain_id,
        })?;

        let service_topic = normalize_service_topic(&options.service_topic);
        let request_topic = rpc_request_topic(&service_topic);
        let response_topic = rpc_response_topic(&service_topic);
        let request_writer = node.publisher::<RpcReqMsg>(&request_topic)?;
        let response_stream = node
            .subscribe_reader::<RpcRespMsg>(&response_topic)?
            .async_sample_stream();

        Ok(Self {
            node,
            request_writer: request_writer.into_inner(),
            response_stream: Mutex::new(response_stream),
            default_timeout: options.default_timeout,
            startup_wait: options.startup_wait,
            startup_wait_done: AtomicBool::new(false),
            service_topic,
        })
    }

    pub fn node(&self) -> &DdsNode {
        &self.node
    }

    pub async fn call_void<ApiId>(&self, api_id: ApiId, body: impl Into<String>) -> Result<()>
    where
        ApiId: Into<i32> + Copy,
    {
        self.call_void_with_timeout(api_id, body, None).await
    }

    pub async fn call_void_with_timeout<ApiId>(
        &self,
        api_id: ApiId,
        body: impl Into<String>,
        timeout: Option<Duration>,
    ) -> Result<()>
    where
        ApiId: Into<i32> + Copy,
    {
        self.call_with_body::<EmptyResponse>(api_id.into(), body.into(), timeout)
            .await?;
        Ok(())
    }

    pub async fn call_response<ApiId, R>(&self, api_id: ApiId, body: impl Into<String>) -> Result<R>
    where
        ApiId: Into<i32> + Copy,
        R: DeserializeOwned + Send + 'static,
    {
        self.call_with_body(api_id.into(), body.into(), None).await
    }

    pub async fn call_serialized<ApiId, P>(&self, api_id: ApiId, params: &P) -> Result<()>
    where
        ApiId: Into<i32> + Copy,
        P: Serialize,
    {
        self.call_void(api_id, serde_json::to_string(params)?).await
    }

    pub async fn call_serialized_response<ApiId, P, R>(
        &self,
        api_id: ApiId,
        params: &P,
    ) -> Result<R>
    where
        ApiId: Into<i32> + Copy,
        P: Serialize,
        R: DeserializeOwned + Send + 'static,
    {
        self.call_response(api_id, serde_json::to_string(params)?)
            .await
    }

    pub async fn call<P, R>(&self, api_id: i32, params: &P, timeout: Option<Duration>) -> Result<R>
    where
        P: Serialize,
        R: DeserializeOwned + Send + 'static,
    {
        let body = serde_json::to_string(params).map_err(|e| {
            RpcError::BadRequest(format!("Failed to serialize request parameters: {e}"))
        })?;

        self.call_with_body(api_id, body, timeout).await
    }

    pub async fn call_with_body<R>(
        &self,
        api_id: i32,
        body: impl Into<String>,
        timeout: Option<Duration>,
    ) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        if self.startup_wait > Duration::from_millis(0)
            && !self.startup_wait_done.swap(true, Ordering::SeqCst)
        {
            tracing::debug!(
                target: "booster_sdk::rpc",
                service_topic = %self.service_topic,
                startup_wait_ms = self.startup_wait.as_millis(),
                "initial startup wait before first rpc call"
            );
            tokio::time::sleep(self.startup_wait).await;
        }

        // Single-flight per client: one response stream consumer at a time.
        let mut response_stream = self.response_stream.lock().await;

        let request_id = Uuid::new_v4().to_string();
        let body = body.into();
        let header = serde_json::json!({ "api_id": api_id }).to_string();
        let service_topic = self.service_topic.clone();

        tracing::debug!(
            target: "booster_sdk::rpc",
            service_topic = %service_topic,
            api_id,
            request_uuid = %request_id,
            header = %preview_for_log(&header, 200),
            body = %preview_for_log(&body, 300),
            "send rpc request"
        );

        let request = RpcReqMsg {
            uuid: request_id.clone(),
            header,
            body,
        };

        self.request_writer
            .write(request, None)
            .map_err(|err| RpcError::BadRequest(format!("Failed to send request: {err}")))?;

        let timeout = timeout.unwrap_or(self.default_timeout);
        let deadline = Instant::now() + timeout;

        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            let response = match tokio::time::timeout(remaining, response_stream.next()).await {
                Ok(Some(Ok(sample))) => sample.into_value(),
                Ok(Some(Err(err))) => {
                    tracing::warn!(
                        target: "booster_sdk::rpc",
                        service_topic = %service_topic,
                        api_id,
                        request_uuid = %request_id,
                        error = %err,
                        "rpc receive error"
                    );
                    return Err(DdsError::ReceiveFailed(err.to_string()).into());
                }
                Ok(None) => {
                    return Err(
                        DdsError::ReceiveFailed("rpc response stream closed".to_owned()).into(),
                    );
                }
                Err(_) => {
                    tracing::warn!(
                        target: "booster_sdk::rpc",
                        service_topic = %service_topic,
                        api_id,
                        request_uuid = %request_id,
                        timeout_ms = timeout.as_millis(),
                        "rpc timeout"
                    );
                    return Err(RpcError::Timeout { timeout }.into());
                }
            };

            if response.uuid != request_id {
                tracing::debug!(
                    target: "booster_sdk::rpc",
                    service_topic = %service_topic,
                    api_id,
                    request_uuid = %request_id,
                    response_uuid = %response.uuid,
                    "ignoring response for a different request uuid"
                );
                continue;
            }

            let status_code = parse_status_from_header(&response.header).unwrap_or(0);
            tracing::debug!(
                target: "booster_sdk::rpc",
                service_topic = %service_topic,
                api_id,
                request_uuid = %request_id,
                response_uuid = %response.uuid,
                status_code,
                header = %preview_for_log(&response.header, 200),
                body = %preview_for_log(&response.body, 300),
                "recv rpc response"
            );

            if status_code == -1 {
                tracing::debug!(
                    target: "booster_sdk::rpc",
                    service_topic = %service_topic,
                    api_id,
                    request_uuid = %request_id,
                    "ignoring intermediate status=-1"
                );
                continue;
            }

            if status_code != 0 {
                let message = if response.body.trim().is_empty() {
                    response.header
                } else {
                    response.body
                };
                return Err(RpcError::from_status_code(status_code, message).into());
            }

            let result: R =
                decode_response_body(&response.body).map_err(|err| RpcError::RequestFailed {
                    status: status_code,
                    message: format!("Failed to deserialize response body: {err}"),
                })?;

            return Ok(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_response_body, parse_status_from_header, parse_status_value};
    use serde_json::json;

    #[derive(serde::Deserialize)]
    struct EmptyResponse {}

    #[test]
    fn parse_status_from_header_reads_status_field() {
        assert_eq!(parse_status_from_header(r#"{"status":0}"#), Some(0));
        assert_eq!(parse_status_from_header(r#"{"status":"-1"}"#), Some(-1));
    }

    #[test]
    fn parse_status_value_handles_number_and_string() {
        assert_eq!(parse_status_value(&json!(0)), Some(0));
        assert_eq!(parse_status_value(&json!("-1")), Some(-1));
        assert_eq!(parse_status_value(&json!("not-a-number")), None);
    }

    #[test]
    fn parse_status_from_header_ignores_other_fields() {
        assert_eq!(parse_status_from_header(r#"{"status_code":0}"#), None);
        assert_eq!(parse_status_from_header(r#"{"code":0}"#), None);
    }

    #[test]
    fn empty_body_deserializes_as_empty_object() {
        let _: EmptyResponse = decode_response_body("").expect("empty body should parse");
    }

    #[test]
    fn non_json_body_fails_deserialization() {
        let parsed = decode_response_body::<EmptyResponse>("not-json");
        assert!(parsed.is_err());
    }
}
