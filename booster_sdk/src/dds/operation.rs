//! Asynchronous RPC operations (SDK 1.8).
//!
//! An operation is started with a regular RPC request whose header carries
//! `call_mode = 1` (operation), `operation_command = 1` (start) and a
//! client-generated `operation_id`. The start response only says whether the
//! service accepted the operation. Progress and the final result are published
//! afterwards on a separate `*OperationEvent` topic as `RpcRespMsg` samples
//! whose `uuid` is the operation id.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::Value;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::types::{Result, RpcError};

use super::messages::RpcRespMsg;
use super::node::DdsNode;
use super::qos::qos_reliable_keep_last;
use super::rpc::RpcClient;
use super::topics::{TYPE_RPC_RESP, TopicSpec};

/// Default time to wait for the service to accept an operation.
pub const DEFAULT_OPERATION_START_TIMEOUT: Duration = Duration::from_millis(1000);

const CALL_MODE_OPERATION: i32 = 1;
const OPERATION_COMMAND_START: i32 = 1;
const OPERATION_COMMAND_CANCEL: i32 = 2;

const EVENT_TYPE_PROGRESS: i64 = 1;
const EVENT_TYPE_FINISHED: i64 = 2;

/// Event topic for an operation-capable RPC service.
pub fn operation_event_topic(name: &str) -> TopicSpec {
    TopicSpec {
        name: name.to_owned(),
        type_name: TYPE_RPC_RESP,
        qos: qos_reliable_keep_last(64),
        kind: rustdds::TopicKind::NoKey,
    }
}

/// Final outcome of an asynchronous operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApiOperationResultCode {
    Unknown,
    Succeeded,
    Aborted,
    Canceled,
    Rejected,
    Timeout,
}

impl From<i64> for ApiOperationResultCode {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Succeeded,
            2 => Self::Aborted,
            3 => Self::Canceled,
            4 => Self::Rejected,
            5 => Self::Timeout,
            _ => Self::Unknown,
        }
    }
}

impl From<ApiOperationResultCode> for i32 {
    fn from(value: ApiOperationResultCode) -> Self {
        match value {
            ApiOperationResultCode::Unknown => 0,
            ApiOperationResultCode::Succeeded => 1,
            ApiOperationResultCode::Aborted => 2,
            ApiOperationResultCode::Canceled => 3,
            ApiOperationResultCode::Rejected => 4,
            ApiOperationResultCode::Timeout => 5,
        }
    }
}

/// Identifies a started operation, used to cancel it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApiOperationHandle {
    pub operation_id: String,
    pub api_id: i32,
}

/// Intermediate progress published while an operation runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiOperationProgress {
    pub operation_id: String,
    pub api_id: i64,
    pub status: i64,
    pub message: String,
    pub body: String,
}

/// Final result of an operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiOperationResult {
    pub code: ApiOperationResultCode,
    pub operation_id: String,
    pub api_id: i64,
    pub status: i64,
    pub message: String,
    pub body: String,
}

impl ApiOperationResult {
    /// Whether the operation finished successfully.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.code == ApiOperationResultCode::Succeeded && self.status == 0
    }

    /// Convert into the result body, or an error for unsuccessful operations.
    pub fn into_body(self) -> Result<String> {
        if self.is_success() {
            return Ok(self.body);
        }
        let status = i32::try_from(self.status).unwrap_or(i32::MAX);
        let message = if self.message.is_empty() {
            format!("operation finished with {:?}", self.code)
        } else {
            self.message
        };
        if status == 0 {
            return Err(RpcError::RequestFailed { status, message }.into());
        }
        Err(RpcError::from_status_code(status, message).into())
    }
}

/// Event published for an operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiOperationEvent {
    Progress(ApiOperationProgress),
    Finished(ApiOperationResult),
}

type Registry = Mutex<HashMap<String, mpsc::UnboundedSender<ApiOperationEvent>>>;

/// A started operation. Receives progress and the final result.
///
/// Dropping it stops event delivery but does not cancel the operation on the
/// robot; use the owning client's cancel method for that.
pub struct ApiOperation {
    handle: ApiOperationHandle,
    events: mpsc::UnboundedReceiver<ApiOperationEvent>,
    registry: Arc<Registry>,
}

impl ApiOperation {
    /// Handle identifying this operation.
    pub fn handle(&self) -> &ApiOperationHandle {
        &self.handle
    }

    /// Operation id generated for this operation.
    pub fn operation_id(&self) -> &str {
        &self.handle.operation_id
    }

    /// Receive the next progress or result event.
    ///
    /// Returns `None` after the final result has been delivered.
    pub async fn next_event(&mut self) -> Option<ApiOperationEvent> {
        self.events.recv().await
    }

    /// Wait for the final result, skipping progress events.
    pub async fn wait(&mut self) -> Result<ApiOperationResult> {
        while let Some(event) = self.events.recv().await {
            if let ApiOperationEvent::Finished(result) = event {
                return Ok(result);
            }
        }
        Err(RpcError::RequestFailed {
            status: -1,
            message: "operation event stream closed".to_owned(),
        }
        .into())
    }

    /// Wait for the final result for at most `timeout`.
    pub async fn wait_timeout(&mut self, timeout: Duration) -> Result<ApiOperationResult> {
        tokio::time::timeout(timeout, self.wait())
            .await
            .map_err(|_| RpcError::Timeout { timeout })?
    }
}

impl Drop for ApiOperation {
    fn drop(&mut self) {
        if let Ok(mut ops) = self.registry.lock() {
            ops.remove(&self.handle.operation_id);
        }
    }
}

/// Starts and cancels asynchronous operations for one RPC service and routes
/// the service's operation events to the matching [`ApiOperation`].
pub struct RpcOperationClient {
    registry: Arc<Registry>,
}

impl RpcOperationClient {
    /// Subscribe to `event_topic` on `node`.
    pub fn new(node: &DdsNode, event_topic: &str) -> Result<Self> {
        let mut reader =
            node.subscribe_reader::<RpcRespMsg>(&operation_event_topic(event_topic))?;
        let registry: Arc<Registry> = Arc::new(Mutex::new(HashMap::new()));
        let weak = Arc::downgrade(&registry);
        let topic = event_topic.to_owned();

        std::thread::spawn(move || {
            loop {
                let Some(registry) = weak.upgrade() else {
                    break;
                };
                match reader.take_next_sample() {
                    Ok(Some(sample)) => dispatch_event(&registry, &topic, sample.into_value()),
                    Ok(None) => {
                        drop(registry);
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(_) => {
                        drop(registry);
                        std::thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        });

        Ok(Self { registry })
    }

    /// Start an operation and wait up to `start_timeout` for the service to accept it.
    pub async fn start(
        &self,
        rpc: &RpcClient,
        api_id: i32,
        body: impl Into<String>,
        start_timeout: Duration,
    ) -> Result<ApiOperation> {
        let operation_id = Uuid::new_v4().to_string();
        let (sender, events) = mpsc::unbounded_channel();

        // Register before sending so early events are not lost.
        self.registry
            .lock()
            .map_err(|_| RpcError::BadRequest("operation registry poisoned".to_owned()))?
            .insert(operation_id.clone(), sender);

        let operation = ApiOperation {
            handle: ApiOperationHandle {
                operation_id: operation_id.clone(),
                api_id,
            },
            events,
            registry: Arc::clone(&self.registry),
        };

        let header = operation_header(api_id, OPERATION_COMMAND_START, &operation_id);
        // On failure `operation` is dropped, which unregisters it.
        rpc.call_with_header(api_id, header, body.into(), Some(start_timeout))
            .await?;

        Ok(operation)
    }

    /// Ask the service to cancel a running operation.
    ///
    /// The final `Canceled` result, if any, is still delivered to the operation.
    pub async fn cancel(
        &self,
        rpc: &RpcClient,
        handle: &ApiOperationHandle,
        timeout: Option<Duration>,
    ) -> Result<()> {
        let header = operation_header(
            handle.api_id,
            OPERATION_COMMAND_CANCEL,
            &handle.operation_id,
        );
        rpc.call_with_header(handle.api_id, header, String::new(), timeout)
            .await?;
        Ok(())
    }
}

fn operation_header(api_id: i32, command: i32, operation_id: &str) -> String {
    serde_json::json!({
        "api_id": api_id,
        "call_mode": CALL_MODE_OPERATION,
        "operation_command": command,
        "operation_id": operation_id,
    })
    .to_string()
}

fn parse_i64(value: Option<&Value>) -> Option<i64> {
    match value? {
        Value::Number(n) => n.as_i64(),
        Value::String(s) => s.parse().ok(),
        _ => None,
    }
}

fn parse_event(msg: RpcRespMsg) -> Option<ApiOperationEvent> {
    let header: Value = serde_json::from_str(msg.header.trim()).ok()?;
    let api_id = parse_i64(header.get("api_id")).unwrap_or(0);
    let status = parse_i64(header.get("status")).unwrap_or(0);
    let message = header
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();

    match parse_i64(header.get("event_type"))? {
        EVENT_TYPE_PROGRESS => Some(ApiOperationEvent::Progress(ApiOperationProgress {
            operation_id: msg.uuid,
            api_id,
            status,
            message,
            body: msg.body,
        })),
        EVENT_TYPE_FINISHED => Some(ApiOperationEvent::Finished(ApiOperationResult {
            code: parse_i64(header.get("result_code")).map_or(
                ApiOperationResultCode::Unknown,
                ApiOperationResultCode::from,
            ),
            operation_id: msg.uuid,
            api_id,
            status,
            message,
            body: msg.body,
        })),
        _ => None,
    }
}

fn dispatch_event(registry: &Registry, topic: &str, msg: RpcRespMsg) {
    let Ok(mut ops) = registry.lock() else {
        return;
    };
    let operation_id = msg.uuid.clone();
    let Some(sender) = ops.get(&operation_id) else {
        return;
    };

    let Some(event) = parse_event(msg) else {
        tracing::warn!(
            target: "booster_sdk::rpc",
            topic,
            operation_id = %operation_id,
            "failed to parse rpc operation event"
        );
        return;
    };

    let finished = matches!(event, ApiOperationEvent::Finished(_));
    let _ = sender.send(event);
    if finished {
        // Dropping the sender closes the stream after the result.
        ops.remove(&operation_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(header: &str, body: &str) -> RpcRespMsg {
        RpcRespMsg {
            uuid: "op-1".to_owned(),
            header: header.to_owned(),
            body: body.to_owned(),
        }
    }

    #[test]
    fn operation_header_contains_operation_fields() {
        let header: Value =
            serde_json::from_str(&operation_header(2031, OPERATION_COMMAND_START, "abc")).unwrap();
        assert_eq!(header["api_id"], 2031);
        assert_eq!(header["call_mode"], 1);
        assert_eq!(header["operation_command"], 1);
        assert_eq!(header["operation_id"], "abc");
    }

    #[test]
    fn parses_progress_event() {
        let event = parse_event(msg(
            r#"{"event_type":1,"api_id":1100,"status":0,"message":"working"}"#,
            "{}",
        ))
        .unwrap();
        let ApiOperationEvent::Progress(progress) = event else {
            panic!("expected progress");
        };
        assert_eq!(progress.operation_id, "op-1");
        assert_eq!(progress.api_id, 1100);
        assert_eq!(progress.message, "working");
    }

    #[test]
    fn parses_finished_event() {
        let event = parse_event(msg(
            r#"{"event_type":2,"result_code":3,"api_id":1100,"status":0}"#,
            "",
        ))
        .unwrap();
        let ApiOperationEvent::Finished(result) = event else {
            panic!("expected result");
        };
        assert_eq!(result.code, ApiOperationResultCode::Canceled);
        assert!(!result.is_success());
        assert!(result.into_body().is_err());
    }

    #[test]
    fn ignores_unknown_event_type() {
        assert!(parse_event(msg(r#"{"event_type":0}"#, "")).is_none());
        assert!(parse_event(msg("not json", "")).is_none());
    }

    #[tokio::test]
    async fn dispatch_delivers_result_and_closes_stream() {
        let registry: Arc<Registry> = Arc::new(Mutex::new(HashMap::new()));
        let (sender, events) = mpsc::unbounded_channel();
        registry.lock().unwrap().insert("op-1".to_owned(), sender);
        let mut operation = ApiOperation {
            handle: ApiOperationHandle {
                operation_id: "op-1".to_owned(),
                api_id: 1100,
            },
            events,
            registry: Arc::clone(&registry),
        };

        dispatch_event(&registry, "t", msg(r#"{"event_type":1}"#, "p"));
        dispatch_event(
            &registry,
            "t",
            msg(r#"{"event_type":2,"result_code":1,"status":0}"#, "done"),
        );

        let result = operation.wait().await.unwrap();
        assert_eq!(result.into_body().unwrap(), "done");
        assert!(operation.next_event().await.is_none());
        assert!(registry.lock().unwrap().is_empty());
    }
}
