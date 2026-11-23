//! RPC client for high-level API requests over DDS.
//!
//! Wire format (matches the C++ SDK 1.8):
//! - requests are `RpcReqMsg { uuid, header, body }` on `<channel>Req`, where
//!   `header` is a [`RequestHeader`] JSON object;
//! - responses are `RpcRespMsg { uuid, header, body }` on `<channel>Resp`,
//!   where `uuid` echoes the request and `header` is `{"status": <int>}`.

use futures::{FutureExt, StreamExt};
use rustdds::no_key::{DataReaderEventStream, DataReaderStream};
use rustdds::{DataReaderStatus, DataWriterStatus, StatusEvented};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::types::{
    DdsError, RPC_STATUS_CODE_INVALID, RPC_STATUS_CODE_TIMEOUT, Result, RpcError,
};

use super::DdsNode;
use super::messages::{RpcReqMsg, RpcRespMsg};
use super::topics::{LOCO_API_TOPIC, rpc_request_topic, rpc_response_topic};

/// Default time allowed for DDS endpoint discovery
/// (C++ `RpcClient::kDefaultWaitForServiceTimeoutMs`).
pub const DEFAULT_WAIT_FOR_SERVICE_TIMEOUT_MS: u64 = 5000;
/// [`DEFAULT_WAIT_FOR_SERVICE_TIMEOUT_MS`] as a [`Duration`].
pub const DEFAULT_WAIT_FOR_SERVICE_TIMEOUT: Duration =
    Duration::from_millis(DEFAULT_WAIT_FOR_SERVICE_TIMEOUT_MS);
/// Default time a fire-and-forget request waits for the service's request
/// reader to be discovered (C++ `SendApiRequestFireAndForget` default).
pub const DEFAULT_ENDPOINT_MATCH_TIMEOUT: Duration = Duration::from_millis(1000);

/// RPC delivery and response mode (C++ `RpcCallMode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RpcCallMode {
    /// Normal request; the service publishes one response.
    #[default]
    RequestResponse,
    /// Asynchronous operation lifecycle request (start / cancel).
    Operation,
    /// The service handles the request but publishes no response.
    FireAndForget,
}

impl From<RpcCallMode> for i32 {
    fn from(value: RpcCallMode) -> Self {
        match value {
            RpcCallMode::RequestResponse => 0,
            RpcCallMode::Operation => 1,
            RpcCallMode::FireAndForget => 2,
        }
    }
}

impl RpcCallMode {
    /// Decode a wire value. Unknown values are treated as request-response,
    /// which is how the C++ server treats them (it only special-cases 1 and 2).
    #[must_use]
    pub fn from_wire(value: i64) -> Self {
        match value {
            1 => Self::Operation,
            2 => Self::FireAndForget,
            _ => Self::RequestResponse,
        }
    }
}

/// Lifecycle command for an asynchronous operation (C++ `RpcOperationCommand`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RpcOperationCommand {
    #[default]
    None,
    Start,
    Cancel,
    /// A value this SDK does not know.
    Unknown(i32),
}

impl From<RpcOperationCommand> for i32 {
    fn from(value: RpcOperationCommand) -> Self {
        match value {
            RpcOperationCommand::None => 0,
            RpcOperationCommand::Start => 1,
            RpcOperationCommand::Cancel => 2,
            RpcOperationCommand::Unknown(v) => v,
        }
    }
}

impl From<i32> for RpcOperationCommand {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Start,
            2 => Self::Cancel,
            v => Self::Unknown(v),
        }
    }
}

/// RPC request header (C++ `RequestHeader`), serialized as the JSON string in
/// `RpcReqMsg::header`.
///
/// Wire format: `api_id` and `call_mode` are always present;
/// `operation_command` only when not `None`; `req_id` and `operation_id` only
/// when non-empty.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RequestHeader {
    pub api_id: i64,
    pub call_mode: RpcCallMode,
    pub operation_command: RpcOperationCommand,
    /// Request id used for correlation. The client fills it with the request
    /// UUID when sending, as the C++ client does.
    pub req_id: String,
    pub operation_id: String,
}

impl RequestHeader {
    /// Request-response header for `api_id`.
    #[must_use]
    pub fn new(api_id: impl Into<i64>) -> Self {
        Self {
            api_id: api_id.into(),
            ..Self::default()
        }
    }

    /// Fire-and-forget header for `api_id`.
    #[must_use]
    pub fn fire_and_forget(api_id: impl Into<i64>) -> Self {
        Self {
            call_mode: RpcCallMode::FireAndForget,
            ..Self::new(api_id)
        }
    }

    /// Operation lifecycle header.
    #[must_use]
    pub fn operation(
        api_id: impl Into<i64>,
        command: RpcOperationCommand,
        operation_id: impl Into<String>,
    ) -> Self {
        Self {
            call_mode: RpcCallMode::Operation,
            operation_command: command,
            operation_id: operation_id.into(),
            ..Self::new(api_id)
        }
    }

    #[must_use]
    pub fn with_req_id(mut self, req_id: impl Into<String>) -> Self {
        self.req_id = req_id.into();
        self
    }

    /// JSON value matching C++ `RequestHeader::ToJson`.
    #[must_use]
    pub fn to_json(&self) -> Value {
        let mut map = Map::new();
        map.insert("api_id".to_owned(), Value::from(self.api_id));
        map.insert(
            "call_mode".to_owned(),
            Value::from(i32::from(self.call_mode)),
        );
        if self.operation_command != RpcOperationCommand::None {
            map.insert(
                "operation_command".to_owned(),
                Value::from(i32::from(self.operation_command)),
            );
        }
        if !self.req_id.is_empty() {
            map.insert("req_id".to_owned(), Value::from(self.req_id.clone()));
        }
        if !self.operation_id.is_empty() {
            map.insert(
                "operation_id".to_owned(),
                Value::from(self.operation_id.clone()),
            );
        }
        Value::Object(map)
    }

    /// Serialized header string.
    #[must_use]
    pub fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }

    /// Parse a header (C++ `RequestHeader::FromJson`). `api_id` is required;
    /// the other fields default when absent.
    #[must_use]
    pub fn from_json(value: &Value) -> Option<Self> {
        let object = value.as_object()?;
        let api_id = json_i64(object.get("api_id"))?;
        let call_mode = json_i64(object.get("call_mode"))
            .map_or(RpcCallMode::RequestResponse, RpcCallMode::from_wire);
        let operation_command = json_i64(object.get("operation_command"))
            .and_then(|v| i32::try_from(v).ok())
            .map_or(RpcOperationCommand::None, RpcOperationCommand::from);
        let string = |key: &str| {
            object
                .get(key)
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned()
        };
        Some(Self {
            api_id,
            call_mode,
            operation_command,
            req_id: string("req_id"),
            operation_id: string("operation_id"),
        })
    }

    /// Parse a header from its JSON string.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        Self::from_json(&serde_json::from_str(raw.trim()).ok()?)
    }
}

/// RPC response header (C++ `ResponseHeader`): `{"status": <int>}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResponseHeader {
    pub status: i64,
}

impl Default for ResponseHeader {
    fn default() -> Self {
        Self {
            status: i64::from(RPC_STATUS_CODE_INVALID),
        }
    }
}

impl ResponseHeader {
    #[must_use]
    pub fn new(status: impl Into<i64>) -> Self {
        Self {
            status: status.into(),
        }
    }

    #[must_use]
    pub fn to_json(&self) -> Value {
        serde_json::json!({ "status": self.status })
    }

    #[must_use]
    pub fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }

    /// Parse a response header; `None` if `status` is missing or malformed.
    #[must_use]
    pub fn parse(raw: &str) -> Option<Self> {
        parse_status_from_header(raw).map(Self::new)
    }
}

pub(crate) fn json_i64(value: Option<&Value>) -> Option<i64> {
    match value? {
        Value::Number(n) => n.as_i64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

/// Tracks how many remote endpoints are currently matched, fed from RustDDS
/// status events (each event is delivered once, so the count must be kept).
#[derive(Debug, Default)]
pub(crate) struct MatchCount {
    current: AtomicI32,
    drain_lock: std::sync::Mutex<()>,
}

impl MatchCount {
    pub(crate) fn is_matched(&self) -> bool {
        self.current.load(Ordering::SeqCst) > 0
    }

    fn set(&self, count: i32) {
        self.current.store(count, Ordering::SeqCst);
    }

    pub(crate) fn drain_writer<T: Serialize>(&self, writer: &rustdds::no_key::DataWriter<T>) {
        let Ok(_guard) = self.drain_lock.lock() else {
            return;
        };
        while let Some(status) = writer.try_recv_status() {
            if let DataWriterStatus::PublicationMatched { current, .. } = status {
                self.set(current.count());
            }
        }
    }

    fn drain_reader_events<T: 'static>(&self, events: &std::sync::Mutex<DataReaderEventStream<T>>) {
        let Ok(mut events) = events.lock() else {
            return;
        };
        while let Some(Some(status)) = events.next().now_or_never() {
            self.apply_reader_status(&status);
        }
    }

    pub(crate) fn apply_reader_status(&self, status: &DataReaderStatus) {
        if let DataReaderStatus::SubscriptionMatched { current, .. } = status {
            self.set(current.count());
        }
    }
}

/// Poll `ready` every 10 ms until it returns true or `timeout` elapses.
pub(crate) async fn poll_until(timeout: Duration, mut ready: impl FnMut() -> bool) -> bool {
    const POLL_INTERVAL: Duration = Duration::from_millis(10);
    let deadline = Instant::now() + timeout;
    loop {
        if ready() {
            return true;
        }
        let now = Instant::now();
        if now >= deadline {
            return false;
        }
        tokio::time::sleep(POLL_INTERVAL.min(deadline - now)).await;
    }
}

#[derive(Debug, Clone)]
pub struct RpcClientOptions {
    pub domain_id: u16,
    pub default_timeout: Duration,
    pub startup_wait: Duration,
    pub service_topic: String,
    /// Optional robot-name suffix. When set (and non-empty), the RPC channel
    /// becomes `<service_topic>/<robot_name>`, matching the C++
    /// `Client::Init(robot_name)` overloads.
    pub robot_name: Option<String>,
}

impl Default for RpcClientOptions {
    fn default() -> Self {
        Self {
            domain_id: 0,
            // 5 s is a safe default for most commands. Mode changes are slow,
            // so change_mode passes its own longer timeout.
            default_timeout: Duration::from_secs(5),
            // Before the first RPC call, wait at most this long for the request
            // writer to match the service's reader. Returns as soon as it matches.
            startup_wait: Duration::from_millis(3000),
            service_topic: LOCO_API_TOPIC.to_owned(),
            robot_name: None,
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

    /// Target a named robot: the channel becomes `<service_topic>/<robot_name>`.
    /// An empty name selects the default (un-suffixed) channel.
    #[must_use]
    pub fn with_robot_name(mut self, robot_name: impl Into<String>) -> Self {
        let name = robot_name.into();
        self.robot_name = if name.is_empty() { None } else { Some(name) };
        self
    }

    /// The effective service channel, including the robot-name suffix.
    #[must_use]
    pub fn resolved_service_topic(&self) -> String {
        let base = normalize_service_topic(&self.service_topic);
        match self.robot_name.as_deref() {
            Some(name) if !name.is_empty() => format!("{base}/{name}"),
            _ => base,
        }
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
    response_events: std::sync::Mutex<DataReaderEventStream<RpcRespMsg>>,
    request_matches: MatchCount,
    response_matches: MatchCount,
    default_timeout: Duration,
    startup_wait: Duration,
    startup_wait_done: AtomicBool,
    service_topic: String,
}

#[derive(Debug, Deserialize, Default)]
struct EmptyResponse {}

fn parse_status_value(value: &Value) -> Option<i64> {
    json_i64(Some(value))
}

fn parse_status_from_header(raw_json: &str) -> Option<i64> {
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

pub(crate) fn normalize_service_topic(service_topic: &str) -> String {
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

pub(crate) fn preview_for_log(value: &str, max_chars: usize) -> String {
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

    // `..Default::default()` keeps this compiling if `DdsConfig` grows fields.
    #[allow(clippy::needless_update)]
    pub fn new(options: RpcClientOptions) -> Result<Self> {
        let node = DdsNode::new(super::DdsConfig {
            domain_id: options.domain_id,
            ..Default::default()
        })?;
        Self::with_node(&node, &options)
    }

    /// Create a client on an existing DDS node. `options.domain_id` is ignored.
    pub fn with_node(node: &DdsNode, options: &RpcClientOptions) -> Result<Self> {
        let service_topic = options.resolved_service_topic();
        let request_topic = rpc_request_topic(&service_topic);
        let response_topic = rpc_response_topic(&service_topic);
        let request_writer = node.publisher::<RpcReqMsg>(&request_topic)?;
        let response_stream = node
            .subscribe_reader::<RpcRespMsg>(&response_topic)?
            .async_sample_stream();
        let response_events = response_stream.async_event_stream();

        Ok(Self {
            node: node.clone(),
            request_writer: request_writer.into_inner(),
            response_stream: Mutex::new(response_stream),
            response_events: std::sync::Mutex::new(response_events),
            request_matches: MatchCount::default(),
            response_matches: MatchCount::default(),
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
        let response_body = self
            .send_request(RequestHeader::new(api_id), body.into(), timeout)
            .await?;

        decode_response_body(&response_body).map_err(|err| {
            RpcError::RequestFailed {
                status: 0,
                message: format!("Failed to deserialize response body: {err}"),
            }
            .into()
        })
    }

    /// Send a raw request (`api_id` + JSON body) and return the raw response
    /// body. Equivalent to the C++ `SendApiRequestWithResponse`.
    pub async fn send_api_request(&self, api_id: i32, body: impl Into<String>) -> Result<String> {
        self.send_request(RequestHeader::new(api_id), body.into(), None)
            .await
    }

    /// Send a request with an explicit header and wait for its response body.
    ///
    /// `header.req_id` is overwritten with the request UUID. A fire-and-forget
    /// header is rejected (use [`Self::fire_and_forget`]).
    pub async fn send_request_with_header(
        &self,
        header: RequestHeader,
        body: impl Into<String>,
        timeout: Option<Duration>,
    ) -> Result<String> {
        if header.call_mode == RpcCallMode::FireAndForget {
            return Err(RpcError::BadRequest(
                "fire-and-forget requests have no response; use fire_and_forget".to_owned(),
            )
            .into());
        }
        self.send_request(header, body.into(), timeout).await
    }

    /// Send a fire-and-forget request (C++ `SendApiRequestFireAndForget`).
    ///
    /// The header carries `call_mode = 2`; the service handles the request but
    /// publishes no response. This never touches the response stream lock, so
    /// it can be called at high rate (e.g. a 20 ms Move loop) concurrently with
    /// regular calls. Waits up to [`DEFAULT_ENDPOINT_MATCH_TIMEOUT`] for the
    /// service's request reader to be discovered (instant once matched).
    pub async fn fire_and_forget<ApiId>(&self, api_id: ApiId, body: impl Into<String>) -> Result<()>
    where
        ApiId: Into<i32>,
    {
        self.fire_and_forget_with_timeout(api_id, body, DEFAULT_ENDPOINT_MATCH_TIMEOUT)
            .await
    }

    /// [`Self::fire_and_forget`] with a raw `api_id`.
    pub async fn send_fire_and_forget(&self, api_id: i32, body: impl Into<String>) -> Result<()> {
        self.fire_and_forget(api_id, body).await
    }

    /// [`Self::fire_and_forget`] with serde-serialized parameters.
    pub async fn fire_and_forget_serialized<ApiId, P>(&self, api_id: ApiId, params: &P) -> Result<()>
    where
        ApiId: Into<i32>,
        P: Serialize,
    {
        self.fire_and_forget(api_id, serde_json::to_string(params)?)
            .await
    }

    /// [`Self::fire_and_forget`] waiting at most `endpoint_match_timeout` for
    /// the request endpoint to match. A zero timeout uses
    /// [`DEFAULT_WAIT_FOR_SERVICE_TIMEOUT`], like the C++ client.
    ///
    /// Fails with [`RpcError::RemoteTimeout`] (status 100) when the service is
    /// not discovered in time, and [`RpcError::InternalServerError`] (status
    /// 500) when the DDS write fails.
    pub async fn fire_and_forget_with_timeout<ApiId>(
        &self,
        api_id: ApiId,
        body: impl Into<String>,
        endpoint_match_timeout: Duration,
    ) -> Result<()>
    where
        ApiId: Into<i32>,
    {
        let timeout = if endpoint_match_timeout.is_zero() {
            DEFAULT_WAIT_FOR_SERVICE_TIMEOUT
        } else {
            endpoint_match_timeout
        };
        if !self.wait_for_service(timeout, false).await {
            return Err(RpcError::from_status_code(
                RPC_STATUS_CODE_TIMEOUT,
                "RPC endpoints are not matched".to_owned(),
            )
            .into());
        }

        let api_id = api_id.into();
        let request_id = Uuid::new_v4().to_string();
        let header = RequestHeader::fire_and_forget(api_id).with_req_id(request_id.clone());
        let body = body.into();
        tracing::trace!(
            target: "booster_sdk::rpc",
            service_topic = %self.service_topic,
            api_id,
            request_uuid = %request_id,
            body = %preview_for_log(&body, 300),
            "send fire-and-forget rpc request"
        );
        self.write_request(request_id, &header, body)
    }

    /// Service topic this client talks to (including any robot-name suffix).
    #[must_use]
    pub fn service_topic(&self) -> &str {
        &self.service_topic
    }

    /// Whether the service endpoints are currently matched, without waiting.
    ///
    /// The request writer must match a remote reader; with
    /// `require_response_path` the response reader must also match a remote
    /// writer.
    #[must_use]
    pub fn is_service_matched(&self, require_response_path: bool) -> bool {
        self.request_matches.drain_writer(&self.request_writer);
        if !self.request_matches.is_matched() {
            return false;
        }
        if require_response_path {
            self.response_matches
                .drain_reader_events(&self.response_events);
            return self.response_matches.is_matched();
        }
        true
    }

    /// Wait until the service endpoints are discovered (C++ `WaitForService`).
    ///
    /// Returns `true` once the request writer has matched the service's
    /// reader and, when `require_response_path` is set, the response reader
    /// has matched the service's writer. Returns `false` after `timeout`.
    /// C++ defaults: [`DEFAULT_WAIT_FOR_SERVICE_TIMEOUT`], `true`.
    pub async fn wait_for_service(&self, timeout: Duration, require_response_path: bool) -> bool {
        let start = Instant::now();
        let matched = poll_until(timeout, || self.is_service_matched(require_response_path)).await;
        if matched {
            tracing::debug!(
                target: "booster_sdk::rpc",
                service_topic = %self.service_topic,
                elapsed_ms = start.elapsed().as_millis(),
                require_response_path,
                "rpc service matched"
            );
        } else {
            tracing::debug!(
                target: "booster_sdk::rpc",
                service_topic = %self.service_topic,
                timeout_ms = timeout.as_millis(),
                require_response_path,
                req_matched = self.request_matches.is_matched(),
                resp_matched = self.response_matches.is_matched(),
                "rpc wait_for_service timed out"
            );
        }
        matched
    }

    fn write_request(&self, request_id: String, header: &RequestHeader, body: String) -> Result<()> {
        let request = RpcReqMsg {
            uuid: request_id,
            header: header.to_json_string(),
            body,
        };
        self.request_writer.write(request, None).map_err(|err| {
            RpcError::InternalServerError(format!("Failed to publish RPC request: {err}")).into()
        })
    }

    /// Send a request with a typed header and return the raw response body.
    ///
    /// The header's `req_id` is set to the request UUID (as the C++ client
    /// does). Non-zero response statuses are returned as errors; responses
    /// with status -1 (no status yet) are skipped, like the C++ client.
    pub(crate) async fn send_request(
        &self,
        mut header: RequestHeader,
        body: String,
        timeout: Option<Duration>,
    ) -> Result<String> {
        if self.startup_wait > Duration::from_millis(0)
            && !self.startup_wait_done.swap(true, Ordering::SeqCst)
            && !self.wait_for_service(self.startup_wait, false).await
        {
            // Requests are volatile, so a request sent before the match is
            // silently lost. Responses are transient-local, so the response
            // side needs no wait.
            tracing::warn!(
                target: "booster_sdk::rpc",
                service_topic = %self.service_topic,
                startup_wait_ms = self.startup_wait.as_millis(),
                "rpc service not discovered within startup wait, sending anyway"
            );
        }

        // Single-flight per client: one response stream consumer at a time.
        let mut response_stream = self.response_stream.lock().await;

        let api_id = header.api_id;
        let request_id = Uuid::new_v4().to_string();
        header.req_id.clone_from(&request_id);
        let service_topic = self.service_topic.clone();

        tracing::debug!(
            target: "booster_sdk::rpc",
            service_topic = %service_topic,
            api_id,
            request_uuid = %request_id,
            header = %preview_for_log(&header.to_json_string(), 200),
            body = %preview_for_log(&body, 300),
            "send rpc request"
        );

        self.write_request(request_id.clone(), &header, body)?;

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

            if status_code == i64::from(RPC_STATUS_CODE_INVALID) {
                // C++ treats status -1 as "no response yet" and keeps waiting.
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
                let code = i32::try_from(status_code).unwrap_or(i32::MAX);
                return Err(RpcError::from_status_code(code, message).into());
            }

            return Ok(response.body);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

    #[test]
    fn request_response_header_json() {
        let header = RequestHeader::new(2001).with_req_id("u-1");
        assert_eq!(
            header.to_json(),
            json!({ "api_id": 2001, "call_mode": 0, "req_id": "u-1" })
        );
        // Without req_id only the mandatory fields remain.
        assert_eq!(
            RequestHeader::new(7).to_json(),
            json!({ "api_id": 7, "call_mode": 0 })
        );
    }

    #[test]
    fn fire_and_forget_header_json() {
        let header = RequestHeader::fire_and_forget(2001).with_req_id("u-2");
        assert_eq!(
            header.to_json(),
            json!({ "api_id": 2001, "call_mode": 2, "req_id": "u-2" })
        );
    }

    #[test]
    fn operation_header_json() {
        let start = RequestHeader::operation(1100, RpcOperationCommand::Start, "op")
            .with_req_id("u-3");
        assert_eq!(
            start.to_json(),
            json!({
                "api_id": 1100,
                "call_mode": 1,
                "operation_command": 1,
                "operation_id": "op",
                "req_id": "u-3",
            })
        );
        let cancel = RequestHeader::operation(1100, RpcOperationCommand::Cancel, "op");
        assert_eq!(cancel.to_json()["operation_command"], 2);
    }

    #[test]
    fn request_header_round_trips() {
        for header in [
            RequestHeader::new(1),
            RequestHeader::fire_and_forget(2).with_req_id("r"),
            RequestHeader::operation(3, RpcOperationCommand::Cancel, "o").with_req_id("r"),
        ] {
            assert_eq!(RequestHeader::parse(&header.to_json_string()), Some(header));
        }
    }

    #[test]
    fn request_header_parse_defaults() {
        let header = RequestHeader::parse(r#"{"api_id":5}"#).unwrap();
        assert_eq!(header, RequestHeader::new(5));
        assert!(RequestHeader::parse(r#"{"call_mode":0}"#).is_none());
        let unknown = RequestHeader::parse(r#"{"api_id":5,"call_mode":9}"#).unwrap();
        assert_eq!(unknown.call_mode, RpcCallMode::RequestResponse);
    }

    #[test]
    fn response_header_json() {
        assert_eq!(ResponseHeader::default().status, -1);
        assert_eq!(ResponseHeader::new(0).to_json_string(), r#"{"status":0}"#);
        assert_eq!(ResponseHeader::parse(r#"{"status":409}"#), Some(ResponseHeader::new(409)));
    }
}
