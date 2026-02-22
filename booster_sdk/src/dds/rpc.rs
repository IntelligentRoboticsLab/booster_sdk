//! RPC client for high-level API requests over DDS.

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::time::{Duration, Instant};
use uuid::Uuid;

use rustdds::no_key::DataReader;
use std::sync::{Arc, Mutex};

use crate::types::{DdsError, Result, RpcError};

use super::DdsNode;
use super::messages::{RpcReqMsg, RpcRespMsg};
use super::topics::{LOCO_API_TOPIC, rpc_request_topic, rpc_response_topic};

#[derive(Debug)]
pub struct RpcClientOptions {
    pub domain_id: u16,
    pub default_timeout: Duration,
    pub service_topic: String,
}

impl Default for RpcClientOptions {
    fn default() -> Self {
        Self {
            domain_id: 0,
            // 5 s is a safe default for most commands. Mode changes are slow,
            // so change_mode passes its own longer timeout.
            default_timeout: Duration::from_secs(5),
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
}

pub struct RpcClient {
    node: DdsNode,
    request_writer: rustdds::no_key::DataWriter<RpcReqMsg>,
    response_reader: Arc<Mutex<DataReader<RpcRespMsg>>>,
    default_timeout: Duration,
}

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

impl RpcClient {
    pub fn new(options: RpcClientOptions) -> Result<Self> {
        let node = DdsNode::new(super::DdsConfig {
            domain_id: options.domain_id,
        })?;

        let service_topic = normalize_service_topic(&options.service_topic);
        let request_topic = rpc_request_topic(&service_topic);
        let response_topic = rpc_response_topic(&service_topic);
        let request_writer = node.publisher::<RpcReqMsg>(&request_topic)?;
        let response_reader = node.subscribe_reader::<RpcRespMsg>(&response_topic)?;

        Ok(Self {
            node,
            request_writer: request_writer.into_inner(),
            response_reader: Arc::new(Mutex::new(response_reader)),
            default_timeout: options.default_timeout,
        })
    }

    pub fn node(&self) -> &DdsNode {
        &self.node
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
        let request_id = Uuid::new_v4().to_string();
        let body = body.into();
        let header = serde_json::json!({ "api_id": api_id }).to_string();

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

        let reader = self.response_reader.clone();

        tokio::task::spawn_blocking(move || {
            let mut reader = reader
                .lock()
                .map_err(|err| DdsError::ReceiveFailed(err.to_string()))?;
            loop {
                if Instant::now() >= deadline {
                    return Err(RpcError::Timeout { timeout }.into());
                }

                match reader.take_next_sample() {
                    Ok(Some(sample)) => {
                        let response = sample.into_value();
                        if response.uuid != request_id {
                            continue;
                        }

                        let status_code = parse_status_from_header(&response.header).unwrap_or(0);

                        if status_code == -1 {
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

                        let result: R = decode_response_body(&response.body).map_err(|err| {
                            RpcError::RequestFailed {
                                status: status_code,
                                message: format!("Failed to deserialize response body: {err}"),
                            }
                        })?;

                        return Ok(result);
                    }
                    Ok(None) => std::thread::sleep(Duration::from_millis(5)),
                    Err(err) => {
                        return Err(DdsError::ReceiveFailed(err.to_string()).into());
                    }
                }
            }
        })
        .await
        .map_err(|err| DdsError::ReceiveFailed(err.to_string()))?
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
