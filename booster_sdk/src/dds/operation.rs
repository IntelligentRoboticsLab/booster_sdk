//! Asynchronous RPC operations (SDK 1.8).
//!
//! An operation is started with a regular RPC request whose header carries
//! `call_mode = 1` (operation), `operation_command = 1` (start), the request
//! UUID as `req_id` and a client-generated `operation_id`. The start response
//! only says whether the service accepted the operation. Progress and the
//! final result are published afterwards on a separate `*OperationEvent` topic
//! as `RpcRespMsg` samples whose `uuid` is the operation id and whose header is
//! `{"event_type", "result_code", "api_id", "operation_id", "status", "message"}`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::Value;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::types::{
    BoosterError, RPC_STATUS_CODE_INTERNAL_SERVER_ERROR, RPC_STATUS_CODE_INVALID,
    RPC_STATUS_CODE_SUCCESS, Result, RpcError,
};

use super::messages::RpcRespMsg;
use super::node::DdsNode;
use super::qos::qos_reliable_keep_last;
use super::rpc::{MatchCount, RequestHeader, RpcClient, RpcOperationCommand, json_i64, poll_until};
use super::topics::{TYPE_RPC_RESP, TopicSpec};

/// Default time to wait for the service to accept an operation.
pub const DEFAULT_OPERATION_START_TIMEOUT: Duration = Duration::from_millis(1000);

/// Event topic for an operation-capable RPC service.
pub fn operation_event_topic(name: &str) -> TopicSpec {
    TopicSpec {
        name: name.to_owned(),
        type_name: TYPE_RPC_RESP,
        qos: qos_reliable_keep_last(64),
        kind: rustdds::TopicKind::NoKey,
    }
}

/// Kind of operation event (C++ `ApiOperationEventType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ApiOperationEventType {
    #[default]
    Unknown,
    Progress,
    Finished,
}

impl From<i64> for ApiOperationEventType {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Progress,
            2 => Self::Finished,
            _ => Self::Unknown,
        }
    }
}

impl From<ApiOperationEventType> for i32 {
    fn from(value: ApiOperationEventType) -> Self {
        match value {
            ApiOperationEventType::Unknown => 0,
            ApiOperationEventType::Progress => 1,
            ApiOperationEventType::Finished => 2,
        }
    }
}

/// Final outcome of an asynchronous operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ApiOperationResultCode {
    #[default]
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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ApiOperationProgress {
    pub operation_id: String,
    pub api_id: i64,
    pub status: i64,
    pub message: String,
    pub body: String,
}

/// Final result of an operation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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

/// Outcome of the start RPC (C++ `ApiOperationStartResult`).
///
/// `status == 0` means the service accepted the operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiOperationStartResult {
    pub status: i32,
    pub operation_id: String,
    pub api_id: i64,
    pub message: String,
    /// Set when the operation was accepted.
    pub handle: Option<ApiOperationHandle>,
}

/// Local submit outcome (C++ `ApiOperationSubmitResult`).
///
/// `status == 0` means the SDK created the operation id and started the start
/// RPC in the background.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiOperationSubmitResult {
    pub status: i32,
    pub operation_id: String,
    pub message: String,
    pub handle: Option<ApiOperationHandle>,
}

/// Called once with the start RPC outcome.
pub type StartCallback = Box<dyn FnOnce(&ApiOperationStartResult) + Send>;
/// Called for every progress event.
pub type ProgressCallback = Box<dyn FnMut(&ApiOperationHandle, &ApiOperationProgress) + Send>;
/// Called once with the final result (also when the start was rejected).
pub type ResultCallback = Box<dyn FnOnce(&ApiOperationResult) + Send>;

/// Callbacks for [`RpcOperationClient::run_with_callbacks`] /
/// [`RpcOperationClient::submit`] (C++ `SendApiRequestOptions`).
pub struct SendApiRequestOptions {
    pub start_callback: Option<StartCallback>,
    pub progress_callback: Option<ProgressCallback>,
    pub result_callback: Option<ResultCallback>,
    pub start_timeout: Duration,
}

impl Default for SendApiRequestOptions {
    fn default() -> Self {
        Self {
            start_callback: None,
            progress_callback: None,
            result_callback: None,
            start_timeout: DEFAULT_OPERATION_START_TIMEOUT,
        }
    }
}

impl SendApiRequestOptions {
    #[must_use]
    pub fn on_start(
        mut self,
        callback: impl FnOnce(&ApiOperationStartResult) + Send + 'static,
    ) -> Self {
        self.start_callback = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_progress(
        mut self,
        callback: impl FnMut(&ApiOperationHandle, &ApiOperationProgress) + Send + 'static,
    ) -> Self {
        self.progress_callback = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_result(
        mut self,
        callback: impl FnOnce(&ApiOperationResult) + Send + 'static,
    ) -> Self {
        self.result_callback = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn with_start_timeout(mut self, start_timeout: Duration) -> Self {
        self.start_timeout = start_timeout;
        self
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
    event_matches: Arc<MatchCount>,
}

fn error_status(err: &BoosterError) -> i32 {
    match err {
        BoosterError::Rpc(rpc) => rpc.status_code(),
        _ => RPC_STATUS_CODE_INTERNAL_SERVER_ERROR,
    }
}

impl RpcOperationClient {
    /// Subscribe to `event_topic` on `node`.
    pub fn new(node: &DdsNode, event_topic: &str) -> Result<Self> {
        let mut reader =
            node.subscribe_reader::<RpcRespMsg>(&operation_event_topic(event_topic))?;
        let registry: Arc<Registry> = Arc::new(Mutex::new(HashMap::new()));
        let event_matches = Arc::new(MatchCount::default());
        let weak = Arc::downgrade(&registry);
        let matches = Arc::clone(&event_matches);
        let topic = event_topic.to_owned();

        std::thread::spawn(move || {
            use rustdds::StatusEvented;
            loop {
                let Some(registry) = weak.upgrade() else {
                    break;
                };
                while let Some(status) = reader.try_recv_status() {
                    matches.apply_reader_status(&status);
                }
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

        Ok(Self {
            registry,
            event_matches,
        })
    }

    /// Whether the event reader has matched the service's event writer.
    #[must_use]
    pub fn is_event_path_matched(&self) -> bool {
        self.event_matches.is_matched()
    }

    /// Wait for the RPC service and, with `require_event_path`, for the event
    /// writer to be discovered (C++ `RpcOperationClient::WaitForService`).
    ///
    /// C++ defaults: [`super::DEFAULT_WAIT_FOR_SERVICE_TIMEOUT`], `true`.
    pub async fn wait_for_service(
        &self,
        rpc: &RpcClient,
        timeout: Duration,
        require_event_path: bool,
    ) -> bool {
        poll_until(timeout, || {
            rpc.is_service_matched(true) && (!require_event_path || self.is_event_path_matched())
        })
        .await
    }

    /// Start an operation and wait up to `start_timeout` for the service to accept it.
    pub async fn start(
        &self,
        rpc: &RpcClient,
        api_id: i32,
        body: impl Into<String>,
        start_timeout: Duration,
    ) -> Result<ApiOperation> {
        self.start_with_id(
            rpc,
            Uuid::new_v4().to_string(),
            api_id,
            body.into(),
            start_timeout,
        )
        .await
    }

    async fn start_with_id(
        &self,
        rpc: &RpcClient,
        operation_id: String,
        api_id: i32,
        body: String,
        start_timeout: Duration,
    ) -> Result<ApiOperation> {
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

        let header = RequestHeader::operation(api_id, RpcOperationCommand::Start, operation_id);
        // On failure `operation` is dropped, which unregisters it.
        rpc.send_request(header, body, Some(start_timeout)).await?;

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
        let header = RequestHeader::operation(
            handle.api_id,
            RpcOperationCommand::Cancel,
            handle.operation_id.clone(),
        );
        rpc.send_request(header, String::new(), timeout).await?;
        Ok(())
    }

    /// Start an operation and drive it to completion, invoking the callbacks
    /// in `options` (C++ `AsyncSendApiRequest` with `SendApiRequestOptions`).
    ///
    /// The start callback always runs once. If the start is rejected or fails
    /// the result callback receives a `Rejected` result carrying the start
    /// status. Returns the final result.
    pub async fn run_with_callbacks(
        &self,
        rpc: &RpcClient,
        api_id: i32,
        body: impl Into<String>,
        options: SendApiRequestOptions,
    ) -> ApiOperationResult {
        self.run_with_id(
            rpc,
            Uuid::new_v4().to_string(),
            api_id,
            body.into(),
            options,
        )
        .await
    }

    /// Spawn [`Self::run_with_callbacks`] on the current tokio runtime and
    /// return immediately (C++ `AsyncSendApiRequest`).
    ///
    /// Returns status 500 when called outside a tokio runtime.
    pub fn submit(
        self: &Arc<Self>,
        rpc: Arc<RpcClient>,
        api_id: i32,
        body: impl Into<String>,
        options: SendApiRequestOptions,
    ) -> ApiOperationSubmitResult {
        let operation_id = Uuid::new_v4().to_string();
        let Ok(runtime) = tokio::runtime::Handle::try_current() else {
            return ApiOperationSubmitResult {
                status: RPC_STATUS_CODE_INTERNAL_SERVER_ERROR,
                operation_id,
                message: "no tokio runtime available".to_owned(),
                handle: None,
            };
        };
        let handle = ApiOperationHandle {
            operation_id: operation_id.clone(),
            api_id,
        };
        let this = Arc::clone(self);
        let body = body.into();
        let id = operation_id.clone();
        runtime.spawn(async move {
            this.run_with_id(&rpc, id, api_id, body, options).await;
        });
        ApiOperationSubmitResult {
            status: RPC_STATUS_CODE_SUCCESS,
            operation_id,
            message: String::new(),
            handle: Some(handle),
        }
    }

    async fn run_with_id(
        &self,
        rpc: &RpcClient,
        operation_id: String,
        api_id: i32,
        body: String,
        mut options: SendApiRequestOptions,
    ) -> ApiOperationResult {
        let started = self
            .start_with_id(
                rpc,
                operation_id.clone(),
                api_id,
                body,
                options.start_timeout,
            )
            .await;

        let mut operation = match started {
            Ok(operation) => {
                if let Some(callback) = options.start_callback.take() {
                    callback(&ApiOperationStartResult {
                        status: RPC_STATUS_CODE_SUCCESS,
                        operation_id: operation_id.clone(),
                        api_id: i64::from(api_id),
                        message: String::new(),
                        handle: Some(operation.handle().clone()),
                    });
                }
                operation
            }
            Err(err) => {
                let status = error_status(&err);
                let message = err.to_string();
                if let Some(callback) = options.start_callback.take() {
                    callback(&ApiOperationStartResult {
                        status,
                        operation_id: operation_id.clone(),
                        api_id: i64::from(api_id),
                        message: message.clone(),
                        handle: None,
                    });
                }
                let result = ApiOperationResult {
                    code: ApiOperationResultCode::Rejected,
                    operation_id,
                    api_id: i64::from(api_id),
                    status: i64::from(status),
                    message,
                    body: String::new(),
                };
                if let Some(callback) = options.result_callback.take() {
                    callback(&result);
                }
                return result;
            }
        };

        let handle = operation.handle().clone();
        let result = loop {
            match operation.next_event().await {
                Some(ApiOperationEvent::Progress(progress)) => {
                    if let Some(callback) = options.progress_callback.as_mut() {
                        callback(&handle, &progress);
                    }
                }
                Some(ApiOperationEvent::Finished(result)) => break result,
                None => {
                    break ApiOperationResult {
                        code: ApiOperationResultCode::Unknown,
                        operation_id: operation_id.clone(),
                        api_id: i64::from(api_id),
                        status: i64::from(RPC_STATUS_CODE_INVALID),
                        message: "operation event stream closed".to_owned(),
                        body: String::new(),
                    };
                }
            }
        };
        if let Some(callback) = options.result_callback.take() {
            callback(&result);
        }
        result
    }
}

/// Build an operation event sample (C++ `RpcOperationServer::PublishOperationEvent`).
pub(crate) fn encode_event(
    operation_id: &str,
    api_id: i64,
    event_type: ApiOperationEventType,
    result_code: ApiOperationResultCode,
    status: i64,
    message: &str,
    body: &str,
) -> RpcRespMsg {
    let header = serde_json::json!({
        "event_type": i32::from(event_type),
        "result_code": i32::from(result_code),
        "api_id": api_id,
        "operation_id": operation_id,
        "status": status,
        "message": message,
    });
    RpcRespMsg {
        uuid: operation_id.to_owned(),
        header: header.to_string(),
        body: body.to_owned(),
    }
}

fn parse_event(msg: RpcRespMsg) -> Option<ApiOperationEvent> {
    let header: Value = serde_json::from_str(msg.header.trim()).ok()?;
    let api_id = json_i64(header.get("api_id")).unwrap_or(0);
    let status = json_i64(header.get("status")).unwrap_or(0);
    let message = header
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();

    match ApiOperationEventType::from(json_i64(header.get("event_type"))?) {
        ApiOperationEventType::Progress => {
            Some(ApiOperationEvent::Progress(ApiOperationProgress {
                operation_id: msg.uuid,
                api_id,
                status,
                message,
                body: msg.body,
            }))
        }
        ApiOperationEventType::Finished => Some(ApiOperationEvent::Finished(ApiOperationResult {
            code: json_i64(header.get("result_code")).map_or(
                ApiOperationResultCode::Unknown,
                ApiOperationResultCode::from,
            ),
            operation_id: msg.uuid,
            api_id,
            status,
            message,
            body: msg.body,
        })),
        ApiOperationEventType::Unknown => None,
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
        let header: Value = RequestHeader::operation(2031, RpcOperationCommand::Start, "abc")
            .with_req_id("req")
            .to_json();
        assert_eq!(header["api_id"], 2031);
        assert_eq!(header["call_mode"], 1);
        assert_eq!(header["operation_command"], 1);
        assert_eq!(header["operation_id"], "abc");
        assert_eq!(header["req_id"], "req");
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
    fn encoded_events_round_trip() {
        let encoded = encode_event(
            "op-9",
            1100,
            ApiOperationEventType::Finished,
            ApiOperationResultCode::Succeeded,
            0,
            "done",
            "{\"x\":1}",
        );
        let header: Value = serde_json::from_str(&encoded.header).unwrap();
        assert_eq!(header["event_type"], 2);
        assert_eq!(header["result_code"], 1);
        assert_eq!(header["operation_id"], "op-9");
        let Some(ApiOperationEvent::Finished(result)) = parse_event(encoded) else {
            panic!("expected result");
        };
        assert_eq!(result.operation_id, "op-9");
        assert_eq!(result.message, "done");
        assert!(result.is_success());
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
