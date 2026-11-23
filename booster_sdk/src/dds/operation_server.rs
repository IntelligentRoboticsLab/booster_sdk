//! Server side of asynchronous RPC operations (C++ `RpcOperationServer`).
//!
//! Operation lifecycle requests (`call_mode = 1`) arrive on the RPC channel:
//! - start (`operation_command = 1`) calls [`OperationHandler::start`]; the
//!   response body is `{"req_id", "operation_id", "status", "message"}` and
//!   the response header carries the same status;
//! - cancel (`operation_command = 2`) calls [`OperationHandler::cancel`] and
//!   marks the operation `cancel_requested`.
//!
//! [`RpcOperationServer::update_operations`] (called periodically, or by the
//! built-in ticker when [`RpcOperationServerConfig::update_interval`] is set)
//! enforces deadlines, calls [`OperationHandler::tick`], publishes progress
//! every `progress_interval` via [`OperationHandler::build_progress`] and
//! publishes the final result on the operation event topic.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::types::{
    DdsError, RPC_STATUS_CODE_BAD_REQUEST, RPC_STATUS_CODE_CONFLICT, RPC_STATUS_CODE_INVALID,
    RPC_STATUS_CODE_SERVER_REFUSED, RPC_STATUS_CODE_SUCCESS, RPC_STATUS_CODE_TIMEOUT, Result,
};

use super::messages::RpcRespMsg;
use super::node::DdsNode;
use super::operation::{
    ApiOperationEventType, ApiOperationProgress, ApiOperationResult, ApiOperationResultCode,
    encode_event, operation_event_topic,
};
use super::qos::qos_best_effort_keep_last;
use super::rpc::{RpcCallMode, RpcOperationCommand};
use super::rpc_server::{RpcRequest, RpcResponse, RpcServer};

/// Default progress publication interval (C++ `progress_interval_ms`).
pub const DEFAULT_OPERATION_PROGRESS_INTERVAL: Duration = Duration::from_millis(500);

/// Configuration (C++ `RpcOperationServerConfig`).
#[derive(Debug, Clone)]
pub struct RpcOperationServerConfig {
    /// RPC channel, e.g. `rt/LocoApiTopic`.
    pub rpc_channel_name: String,
    /// Operation event topic, e.g. `rt/LocoApiOperationEvent`.
    pub event_channel_name: String,
    /// Reliable event writer (default). Best-effort writers do not match the
    /// SDK's reliable event readers.
    pub event_reliable: bool,
    /// Minimum time between periodic progress events.
    pub progress_interval: Duration,
    /// When set, a background thread calls `update_operations` at this
    /// interval. `None` (C++ behavior) means the owner calls it.
    pub update_interval: Option<Duration>,
}

impl RpcOperationServerConfig {
    #[must_use]
    pub fn new(rpc_channel_name: impl Into<String>, event_channel_name: impl Into<String>) -> Self {
        Self {
            rpc_channel_name: rpc_channel_name.into(),
            event_channel_name: event_channel_name.into(),
            event_reliable: true,
            progress_interval: DEFAULT_OPERATION_PROGRESS_INTERVAL,
            update_interval: None,
        }
    }

    #[must_use]
    pub fn with_progress_interval(mut self, interval: Duration) -> Self {
        self.progress_interval = interval;
        self
    }

    #[must_use]
    pub fn with_update_interval(mut self, interval: Duration) -> Self {
        self.update_interval = Some(interval);
        self
    }
}

/// Start request passed to [`OperationHandler::start`] (C++ `OperationRequestContext`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationRequestContext {
    pub req_id: String,
    pub operation_id: String,
    pub api_id: i64,
    pub param: String,
    pub now: Instant,
}

/// State of a pending operation (C++ `OperationContext`; the payload is
/// passed to the hooks separately).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationContext {
    pub operation_id: String,
    pub api_id: i64,
    pub cancel_requested: bool,
    pub created_at: Instant,
    /// `None` means no deadline.
    pub deadline: Option<Instant>,
    pub last_progress_time: Instant,
    pub last_progress_body: String,
}

/// Outcome of [`OperationHandler::start`] (C++ `OperationStartDecision`).
#[derive(Debug, Clone)]
pub struct OperationStartDecision<P> {
    /// `0` accepts the operation; anything else rejects it with this status.
    pub status: i32,
    pub message: String,
    pub payload: Option<P>,
    /// Operation times out (result `Timeout`, status 100) at this instant.
    pub deadline: Option<Instant>,
}

impl<P> Default for OperationStartDecision<P> {
    fn default() -> Self {
        Self {
            status: RPC_STATUS_CODE_INVALID,
            message: String::new(),
            payload: None,
            deadline: None,
        }
    }
}

impl<P> OperationStartDecision<P> {
    /// Accept with a payload.
    #[must_use]
    pub fn accept(payload: P) -> Self {
        Self {
            status: RPC_STATUS_CODE_SUCCESS,
            payload: Some(payload),
            ..Self::default()
        }
    }

    /// Reject with `status` and `message`.
    #[must_use]
    pub fn reject(status: i32, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn with_deadline(mut self, deadline: Instant) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

/// Outcome of [`OperationHandler::tick`] (C++ `OperationEvalResult`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationEvalResult {
    pub finished: bool,
    pub code: ApiOperationResultCode,
    pub status: i64,
    pub message: String,
    pub body: String,
    /// Extra progress bodies to publish immediately.
    pub progress_bodies: Vec<String>,
}

impl Default for OperationEvalResult {
    fn default() -> Self {
        Self {
            finished: false,
            code: ApiOperationResultCode::Unknown,
            status: i64::from(RPC_STATUS_CODE_INVALID),
            message: String::new(),
            body: String::new(),
            progress_bodies: Vec::new(),
        }
    }
}

impl OperationEvalResult {
    /// Still running.
    #[must_use]
    pub fn running() -> Self {
        Self::default()
    }

    /// Finished with `code`, `status`, `message` and `body`.
    #[must_use]
    pub fn finished(
        code: ApiOperationResultCode,
        status: impl Into<i64>,
        message: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            finished: true,
            code,
            status: status.into(),
            message: message.into(),
            body: body.into(),
            progress_bodies: Vec::new(),
        }
    }

    /// Finished successfully with `body`.
    #[must_use]
    pub fn succeeded(body: impl Into<String>) -> Self {
        Self::finished(
            ApiOperationResultCode::Succeeded,
            RPC_STATUS_CODE_SUCCESS,
            "",
            body,
        )
    }

    #[must_use]
    pub fn with_progress(mut self, body: impl Into<String>) -> Self {
        self.progress_bodies.push(body.into());
        self
    }
}

/// Operation hooks (C++ `RpcOperationServer` virtuals).
///
/// Hooks run on the server's threads; `start` and `cancel` on the request
/// dispatch thread, `tick` and `build_progress` from `update_operations`.
pub trait OperationHandler: Send + Sync + 'static {
    /// Per-operation state (C++ `std::shared_ptr<void> payload`).
    type Payload: Send + 'static;

    /// Accept or reject an operation. Default rejects with 501.
    fn start(&self, ctx: &OperationRequestContext) -> OperationStartDecision<Self::Payload> {
        let _ = ctx;
        OperationStartDecision::reject(
            RPC_STATUS_CODE_SERVER_REFUSED,
            "operation handler is not implemented",
        )
    }

    /// Handle a cancel request; return 0 to accept. Default accepts; the next
    /// `tick` then sees `ctx.cancel_requested`.
    fn cancel(&self, ctx: &OperationContext, payload: Option<&mut Self::Payload>) -> i32 {
        let _ = (ctx, payload);
        RPC_STATUS_CODE_SUCCESS
    }

    /// Advance the operation. Default finishes as `Canceled` once a cancel was
    /// requested and otherwise keeps running.
    fn tick(
        &self,
        ctx: &OperationContext,
        payload: Option<&mut Self::Payload>,
    ) -> OperationEvalResult {
        let _ = payload;
        if ctx.cancel_requested {
            OperationEvalResult::finished(
                ApiOperationResultCode::Canceled,
                RPC_STATUS_CODE_SUCCESS,
                "canceled",
                "",
            )
        } else {
            OperationEvalResult::running()
        }
    }

    /// Body of the periodic progress event; empty skips it. Default empty.
    fn build_progress(&self, ctx: &OperationContext, payload: Option<&Self::Payload>) -> String {
        let _ = (ctx, payload);
        String::new()
    }

    /// Handle a request that is not an operation request (`call_mode != 1`).
    /// `None` (default) answers 400 "call_mode must be operation".
    fn handle_request(&self, request: &RpcRequest) -> Option<RpcResponse> {
        let _ = request;
        None
    }
}

/// Cloneable publisher for operation events (C++ `PublishOperation*`).
#[derive(Clone)]
pub struct OperationEventPublisher {
    writer: Arc<rustdds::no_key::DataWriter<RpcRespMsg>>,
}

impl OperationEventPublisher {
    /// Publish a progress event.
    pub fn publish_progress(&self, progress: &ApiOperationProgress) -> Result<()> {
        self.publish_event(
            &progress.operation_id,
            progress.api_id,
            ApiOperationEventType::Progress,
            ApiOperationResultCode::Unknown,
            progress.status,
            &progress.message,
            &progress.body,
        )
    }

    /// Publish a final result event.
    pub fn publish_result(&self, result: &ApiOperationResult) -> Result<()> {
        self.publish_event(
            &result.operation_id,
            result.api_id,
            ApiOperationEventType::Finished,
            result.code,
            result.status,
            &result.message,
            &result.body,
        )
    }

    /// Publish a raw operation event.
    #[allow(clippy::too_many_arguments)]
    pub fn publish_event(
        &self,
        operation_id: &str,
        api_id: i64,
        event_type: ApiOperationEventType,
        result_code: ApiOperationResultCode,
        status: i64,
        message: &str,
        body: &str,
    ) -> Result<()> {
        let msg = encode_event(
            operation_id,
            api_id,
            event_type,
            result_code,
            status,
            message,
            body,
        );
        self.writer
            .write(msg, None)
            .map_err(|err| DdsError::PublishFailed(err.to_string()).into())
    }
}

struct PendingOperation<P> {
    ctx: OperationContext,
    payload: Option<P>,
}

struct Inner<H: OperationHandler> {
    handler: H,
    events: OperationEventPublisher,
    progress_interval: Duration,
    pending: Mutex<HashMap<String, PendingOperation<H::Payload>>>,
}

fn operation_response(req_id: &str, operation_id: &str, status: i32, message: &str) -> RpcResponse {
    let body = serde_json::json!({
        "req_id": req_id,
        "operation_id": operation_id,
        "status": status,
        "message": message,
    });
    RpcResponse::new(status, body.to_string())
}

impl<H: OperationHandler> Inner<H> {
    fn lock_pending(
        &self,
    ) -> std::sync::MutexGuard<'_, HashMap<String, PendingOperation<H::Payload>>> {
        self.pending.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn handle_request(&self, request: &RpcRequest) -> RpcResponse {
        if request.header.call_mode != RpcCallMode::Operation
            && let Some(response) = self.handler.handle_request(request)
        {
            return response;
        }
        self.handle_operation_request(request)
    }

    fn handle_operation_request(&self, request: &RpcRequest) -> RpcResponse {
        let header = &request.header;
        if header.call_mode != RpcCallMode::Operation {
            return operation_response(
                &header.req_id,
                &header.operation_id,
                RPC_STATUS_CODE_BAD_REQUEST,
                "call_mode must be operation",
            );
        }
        match header.operation_command {
            RpcOperationCommand::Start => self.handle_start(request),
            RpcOperationCommand::Cancel => self.handle_cancel(request),
            _ => operation_response(
                &header.req_id,
                &header.operation_id,
                RPC_STATUS_CODE_BAD_REQUEST,
                "unsupported operation command",
            ),
        }
    }

    fn handle_start(&self, request: &RpcRequest) -> RpcResponse {
        let header = &request.header;
        let (req_id, operation_id) = (&header.req_id, &header.operation_id);
        if req_id.is_empty() || operation_id.is_empty() || header.api_id == 0 {
            return operation_response(
                req_id,
                operation_id,
                RPC_STATUS_CODE_BAD_REQUEST,
                "req_id, operation_id and api_id are required",
            );
        }
        if self.lock_pending().contains_key(operation_id) {
            return operation_response(
                req_id,
                operation_id,
                RPC_STATUS_CODE_CONFLICT,
                "operation_id is already pending",
            );
        }

        let now = Instant::now();
        let decision = self.handler.start(&OperationRequestContext {
            req_id: req_id.clone(),
            operation_id: operation_id.clone(),
            api_id: header.api_id,
            param: request.body.clone(),
            now,
        });
        if decision.status != RPC_STATUS_CODE_SUCCESS {
            return operation_response(req_id, operation_id, decision.status, &decision.message);
        }

        let mut pending = self.lock_pending();
        if pending.contains_key(operation_id) {
            return operation_response(
                req_id,
                operation_id,
                RPC_STATUS_CODE_CONFLICT,
                "operation_id is already pending",
            );
        }
        pending.insert(
            operation_id.clone(),
            PendingOperation {
                ctx: OperationContext {
                    operation_id: operation_id.clone(),
                    api_id: header.api_id,
                    cancel_requested: false,
                    created_at: now,
                    deadline: decision.deadline,
                    last_progress_time: now,
                    last_progress_body: String::new(),
                },
                payload: decision.payload,
            },
        );
        let message = if decision.message.is_empty() {
            "accepted"
        } else {
            decision.message.as_str()
        };
        operation_response(req_id, operation_id, RPC_STATUS_CODE_SUCCESS, message)
    }

    fn handle_cancel(&self, request: &RpcRequest) -> RpcResponse {
        let header = &request.header;
        let (req_id, operation_id) = (&header.req_id, &header.operation_id);
        if req_id.is_empty() || operation_id.is_empty() {
            return operation_response(
                req_id,
                operation_id,
                RPC_STATUS_CODE_BAD_REQUEST,
                "req_id and operation_id are required",
            );
        }
        let mut pending = self.lock_pending();
        let Some(operation) = pending.get_mut(operation_id) else {
            return operation_response(
                req_id,
                operation_id,
                RPC_STATUS_CODE_CONFLICT,
                "operation_id is not pending",
            );
        };
        let status = self
            .handler
            .cancel(&operation.ctx, operation.payload.as_mut());
        if status == RPC_STATUS_CODE_SUCCESS {
            operation.ctx.cancel_requested = true;
            operation_response(req_id, operation_id, status, "cancel requested")
        } else {
            operation_response(req_id, operation_id, status, "cancel request failed")
        }
    }

    fn publish_result(&self, ctx: &OperationContext, eval: &OperationEvalResult) {
        let result = ApiOperationResult {
            code: eval.code,
            operation_id: ctx.operation_id.clone(),
            api_id: ctx.api_id,
            status: eval.status,
            message: eval.message.clone(),
            body: eval.body.clone(),
        };
        self.log_publish(&ctx.operation_id, self.events.publish_result(&result));
    }

    fn publish_progress(&self, ctx: &OperationContext, body: &str) {
        let progress = ApiOperationProgress {
            operation_id: ctx.operation_id.clone(),
            api_id: ctx.api_id,
            status: i64::from(RPC_STATUS_CODE_SUCCESS),
            message: String::new(),
            body: body.to_owned(),
        };
        self.log_publish(&ctx.operation_id, self.events.publish_progress(&progress));
    }

    #[allow(clippy::unused_self)]
    fn log_publish(&self, operation_id: &str, result: Result<()>) {
        if let Err(err) = result {
            tracing::warn!(
                target: "booster_sdk::rpc",
                operation_id,
                %err,
                "failed to publish rpc operation event"
            );
        }
    }

    fn update_operations(&self) {
        let now = Instant::now();
        let mut pending = self.lock_pending();
        pending.retain(|_, operation| {
            if operation
                .ctx
                .deadline
                .is_some_and(|deadline| now >= deadline)
            {
                let eval = OperationEvalResult::finished(
                    ApiOperationResultCode::Timeout,
                    RPC_STATUS_CODE_TIMEOUT,
                    "operation timed out",
                    "",
                );
                self.publish_result(&operation.ctx, &eval);
                return false;
            }

            let eval = self
                .handler
                .tick(&operation.ctx, operation.payload.as_mut());
            for body in &eval.progress_bodies {
                self.publish_progress(&operation.ctx, body);
                operation.ctx.last_progress_time = now;
                operation.ctx.last_progress_body.clone_from(body);
            }
            if eval.finished {
                self.publish_result(&operation.ctx, &eval);
                return false;
            }

            if now.duration_since(operation.ctx.last_progress_time) >= self.progress_interval {
                let body = self
                    .handler
                    .build_progress(&operation.ctx, operation.payload.as_ref());
                operation.ctx.last_progress_time = now;
                if !body.is_empty() {
                    self.publish_progress(&operation.ctx, &body);
                    operation.ctx.last_progress_body = body;
                }
            }
            true
        });
    }
}

/// Operation-capable RPC server (C++ `RpcOperationServer`).
///
/// Stops when dropped.
pub struct RpcOperationServer<H: OperationHandler> {
    inner: Arc<Inner<H>>,
    rpc: RpcServer,
    ticker_stop: Arc<AtomicBool>,
    ticker: Option<JoinHandle<()>>,
}

impl<H: OperationHandler> RpcOperationServer<H> {
    /// Serve operations on `node` with `handler` (C++ `Init`).
    pub fn new(node: &DdsNode, config: &RpcOperationServerConfig, handler: H) -> Result<Self> {
        let mut event_topic = operation_event_topic(&config.event_channel_name);
        if !config.event_reliable {
            event_topic.qos = qos_best_effort_keep_last(64);
        }
        let writer = node.publisher::<RpcRespMsg>(&event_topic)?.into_inner();
        let inner = Arc::new(Inner {
            handler,
            events: OperationEventPublisher {
                writer: Arc::new(writer),
            },
            progress_interval: config.progress_interval,
            pending: Mutex::new(HashMap::new()),
        });

        let request_inner = Arc::clone(&inner);
        let rpc = RpcServer::new(
            node,
            &config.rpc_channel_name,
            move |request: &RpcRequest| request_inner.handle_request(request),
        )?;

        let ticker_stop = Arc::new(AtomicBool::new(false));
        let ticker = match config.update_interval {
            Some(interval) => {
                let weak = Arc::downgrade(&inner);
                let stop = Arc::clone(&ticker_stop);
                Some(
                    std::thread::Builder::new()
                        .name("booster-rpc-operation-ticker".to_owned())
                        .spawn(move || {
                            while !stop.load(Ordering::SeqCst) {
                                std::thread::sleep(interval);
                                let Some(inner) = weak.upgrade() else {
                                    break;
                                };
                                inner.update_operations();
                            }
                        })
                        .map_err(|err| DdsError::InitializationFailed(err.to_string()))?,
                )
            }
            None => None,
        };

        Ok(Self {
            inner,
            rpc,
            ticker_stop,
            ticker,
        })
    }

    /// The operation hooks.
    pub fn handler(&self) -> &H {
        &self.inner.handler
    }

    /// Advance all pending operations (C++ `UpdateOperations`).
    pub fn update_operations(&self) {
        self.inner.update_operations();
    }

    /// Ids of the operations still pending.
    #[must_use]
    pub fn pending_operations(&self) -> Vec<String> {
        self.inner.lock_pending().keys().cloned().collect()
    }

    /// Event publisher, for publishing progress/results directly.
    #[must_use]
    pub fn event_publisher(&self) -> OperationEventPublisher {
        self.inner.events.clone()
    }

    /// Publish a progress event (C++ `PublishOperationProgress`).
    pub fn publish_operation_progress(&self, progress: &ApiOperationProgress) -> Result<()> {
        self.inner.events.publish_progress(progress)
    }

    /// Publish a result event (C++ `PublishOperationResult`).
    pub fn publish_operation_result(&self, result: &ApiOperationResult) -> Result<()> {
        self.inner.events.publish_result(result)
    }

    /// The underlying RPC server.
    pub fn rpc_server(&self) -> &RpcServer {
        &self.rpc
    }

    /// Stop serving and drop pending operations (C++ `Stop`). Idempotent.
    pub fn stop(&mut self) {
        self.ticker_stop.store(true, Ordering::SeqCst);
        if let Some(ticker) = self.ticker.take() {
            let _ = ticker.join();
        }
        self.rpc.stop();
        self.inner.lock_pending().clear();
    }
}

impl<H: OperationHandler> Drop for RpcOperationServer<H> {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
#[allow(clippy::needless_update)]
mod tests {
    use super::*;
    use crate::dds::{
        DdsConfig, RpcClient, RpcClientOptions, RpcOperationClient, SendApiRequestOptions,
    };
    use crate::types::{BoosterError, RpcError};
    use std::sync::atomic::AtomicUsize;

    const RPC_TOPIC: &str = "rt/TestOperationApiTopic";
    const EVENT_TOPIC: &str = "rt/TestOperationApiEvent";
    const API_ECHO: i32 = 1;
    const API_FIRE: i32 = 2;
    const API_SLOW: i32 = 3;
    const API_COUNT: i32 = 4;

    struct Handler {
        fire_count: AtomicUsize,
    }

    impl OperationHandler for Handler {
        type Payload = u32;

        fn start(&self, ctx: &OperationRequestContext) -> OperationStartDecision<u32> {
            match i32::try_from(ctx.api_id).unwrap_or_default() {
                API_SLOW => OperationStartDecision::accept(0),
                API_COUNT => OperationStartDecision::accept(0)
                    .with_deadline(ctx.now + Duration::from_secs(30)),
                _ => OperationStartDecision::reject(RPC_STATUS_CODE_BAD_REQUEST, "unknown api"),
            }
        }

        fn tick(&self, ctx: &OperationContext, payload: Option<&mut u32>) -> OperationEvalResult {
            let ticks = payload.expect("payload");
            *ticks += 1;
            if ctx.cancel_requested {
                return OperationEvalResult::finished(
                    ApiOperationResultCode::Canceled,
                    0,
                    "canceled",
                    "",
                );
            }
            if ctx.api_id == i64::from(API_COUNT) && *ticks >= 3 {
                return OperationEvalResult::succeeded(format!("{{\"ticks\":{ticks}}}"));
            }
            OperationEvalResult::running().with_progress(format!("{{\"tick\":{ticks}}}"))
        }

        fn handle_request(&self, request: &RpcRequest) -> Option<RpcResponse> {
            match i32::try_from(request.api_id()).unwrap_or_default() {
                API_ECHO => Some(RpcResponse::ok(request.body.clone())),
                API_FIRE => {
                    self.fire_count.fetch_add(1, Ordering::SeqCst);
                    Some(RpcResponse::ok_empty())
                }
                _ => Some(RpcResponse::error(
                    RPC_STATUS_CODE_BAD_REQUEST,
                    "unknown api",
                )),
            }
        }
    }

    fn test_domain() -> u16 {
        // Spread test runs over domains 150..=199 to avoid clashing with a
        // robot on domain 0 or concurrent runs.
        150 + u16::try_from(std::process::id() % 50).unwrap_or(0)
    }

    /// Client <-> server loopback over DDS on localhost.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore = "uses DDS discovery on localhost; run with --ignored"]
    async fn loopback_request_fire_and_forget_and_operations() {
        let domain_id = test_domain();
        let server_node = DdsNode::new(DdsConfig {
            domain_id,
            ..Default::default()
        })
        .unwrap();
        let server = RpcOperationServer::new(
            &server_node,
            &RpcOperationServerConfig::new(RPC_TOPIC, EVENT_TOPIC)
                .with_update_interval(Duration::from_millis(20))
                .with_progress_interval(Duration::from_millis(50)),
            Handler {
                fire_count: AtomicUsize::new(0),
            },
        )
        .unwrap();

        let options = RpcClientOptions::for_service(RPC_TOPIC).without_startup_wait();
        let client_node = DdsNode::new(DdsConfig {
            domain_id,
            ..Default::default()
        })
        .unwrap();
        let rpc = Arc::new(RpcClient::with_node(&client_node, &options).unwrap());
        let operations = Arc::new(RpcOperationClient::new(&client_node, EVENT_TOPIC).unwrap());

        assert!(
            operations
                .wait_for_service(&rpc, Duration::from_secs(15), true)
                .await,
            "service not discovered"
        );

        // Request/response.
        let echoed = rpc.send_api_request(API_ECHO, r#"{"a":1}"#).await.unwrap();
        assert_eq!(echoed, r#"{"a":1}"#);
        let err = rpc.send_api_request(99, "").await.unwrap_err();
        assert!(matches!(err, BoosterError::Rpc(RpcError::BadRequest(_))));

        // Fire-and-forget: no response, handled by the server.
        for _ in 0..5 {
            rpc.fire_and_forget(API_FIRE, "{}").await.unwrap();
        }
        let handler_count = || server.handler().fire_count.load(Ordering::SeqCst);
        let deadline = Instant::now() + Duration::from_secs(5);
        while handler_count() < 5 && Instant::now() < deadline {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert_eq!(handler_count(), 5);

        // Operation that finishes on its own, observed via callbacks.
        let progress_seen = Arc::new(AtomicUsize::new(0));
        let progress_counter = Arc::clone(&progress_seen);
        let result = operations
            .run_with_callbacks(
                &rpc,
                API_COUNT,
                "{}",
                SendApiRequestOptions::default()
                    .on_start(|start| assert_eq!(start.status, 0))
                    .on_progress(move |_, _| {
                        progress_counter.fetch_add(1, Ordering::SeqCst);
                    }),
            )
            .await;
        assert!(result.is_success(), "{result:?}");
        assert_eq!(result.body, r#"{"ticks":3}"#);
        assert!(progress_seen.load(Ordering::SeqCst) >= 1);

        // Rejected start.
        let rejected = operations
            .start(&rpc, API_ECHO, "{}", Duration::from_secs(2))
            .await;
        assert!(rejected.is_err());

        // Cancel a long-running operation.
        let mut operation = operations
            .start(&rpc, API_SLOW, "{}", Duration::from_secs(2))
            .await
            .unwrap();
        assert!(matches!(
            operation.next_event().await,
            Some(crate::dds::ApiOperationEvent::Progress(_))
        ));
        operations
            .cancel(&rpc, operation.handle(), Some(Duration::from_secs(2)))
            .await
            .unwrap();
        let result = operation
            .wait_timeout(Duration::from_secs(5))
            .await
            .unwrap();
        assert_eq!(result.code, ApiOperationResultCode::Canceled);
        assert!(server.pending_operations().is_empty());
    }

    #[test]
    fn start_and_cancel_validation() {
        let node = DdsNode::new(DdsConfig {
            domain_id: test_domain(),
            ..Default::default()
        })
        .unwrap();
        let server = RpcOperationServer::new(
            &node,
            &RpcOperationServerConfig::new("rt/TestValidationTopic", "rt/TestValidationEvent"),
            Handler {
                fire_count: AtomicUsize::new(0),
            },
        )
        .unwrap();
        let inner = &server.inner;
        let request = |header: crate::dds::RequestHeader| RpcRequest {
            uuid: "u".to_owned(),
            header,
            body: String::new(),
        };
        let status = |response: RpcResponse| response.status();

        // Missing req_id.
        let start = crate::dds::RequestHeader::operation(
            i64::from(API_SLOW),
            RpcOperationCommand::Start,
            "op",
        );
        assert_eq!(status(inner.handle_request(&request(start.clone()))), 400);

        let start = start.with_req_id("r");
        let accepted = inner.handle_request(&request(start.clone()));
        assert_eq!(accepted.status(), 0);
        let body: serde_json::Value = serde_json::from_str(&accepted.body).unwrap();
        assert_eq!(body["message"], "accepted");
        assert_eq!(body["req_id"], "r");
        assert_eq!(body["operation_id"], "op");
        // Duplicate.
        assert_eq!(status(inner.handle_request(&request(start))), 409);

        // Unknown operation cancel.
        let cancel = crate::dds::RequestHeader::operation(
            i64::from(API_SLOW),
            RpcOperationCommand::Cancel,
            "missing",
        )
        .with_req_id("r");
        assert_eq!(status(inner.handle_request(&request(cancel))), 409);

        let cancel = crate::dds::RequestHeader::operation(
            i64::from(API_SLOW),
            RpcOperationCommand::Cancel,
            "op",
        )
        .with_req_id("r");
        assert_eq!(status(inner.handle_request(&request(cancel))), 0);
        server.update_operations();
        assert!(server.pending_operations().is_empty());

        // Deadline in the past times out on the next update.
        let past = crate::dds::RequestHeader::operation(
            i64::from(API_COUNT),
            RpcOperationCommand::Start,
            "op2",
        )
        .with_req_id("r");
        assert_eq!(status(inner.handle_request(&request(past))), 0);
        inner.lock_pending().get_mut("op2").unwrap().ctx.deadline = Some(Instant::now());
        server.update_operations();
        assert!(server.pending_operations().is_empty());
    }
}
