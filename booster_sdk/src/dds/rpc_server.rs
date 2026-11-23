//! RPC server for DDS-backed SDK services (C++ `RpcServer`).
//!
//! Subscribes to `<channel>Req`, hands each decoded request to an
//! [`RpcHandler`] and publishes the returned response on `<channel>Resp`
//! (`uuid` echoes the request, header is `{"status": <int>}`). Requests sent
//! with [`RpcCallMode::FireAndForget`] are handled but not answered.

use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::thread::JoinHandle;

use futures::StreamExt;
use tokio::sync::oneshot;

use crate::types::{
    DdsError, RPC_STATUS_CODE_BAD_REQUEST, RPC_STATUS_CODE_INTERNAL_SERVER_ERROR,
    RPC_STATUS_CODE_SUCCESS, Result,
};

use super::messages::{RpcReqMsg, RpcRespMsg};
use super::node::DdsNode;
use super::rpc::{RequestHeader, ResponseHeader, RpcCallMode, normalize_service_topic, preview_for_log};
use super::topics::{rpc_request_topic, rpc_response_topic};

/// A decoded RPC request (C++ `Request` plus the correlation uuid).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcRequest {
    /// Request uuid from `RpcReqMsg::uuid`; responses echo it.
    pub uuid: String,
    pub header: RequestHeader,
    pub body: String,
}

impl RpcRequest {
    #[must_use]
    pub fn api_id(&self) -> i64 {
        self.header.api_id
    }
}

/// An RPC response (C++ `Response`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcResponse {
    pub header: ResponseHeader,
    pub body: String,
}

impl RpcResponse {
    #[must_use]
    pub fn new(status: impl Into<i64>, body: impl Into<String>) -> Self {
        Self {
            header: ResponseHeader::new(status),
            body: body.into(),
        }
    }

    /// Successful response with a body.
    #[must_use]
    pub fn ok(body: impl Into<String>) -> Self {
        Self::new(RPC_STATUS_CODE_SUCCESS, body)
    }

    /// Successful response with an empty body (C++ `Response::OkResponse`).
    #[must_use]
    pub fn ok_empty() -> Self {
        Self::ok(String::new())
    }

    /// Error response with `status` and a message body.
    #[must_use]
    pub fn error(status: impl Into<i64>, message: impl Into<String>) -> Self {
        Self::new(status, message)
    }

    #[must_use]
    pub fn status(&self) -> i64 {
        self.header.status
    }
}

/// Handles one decoded request and returns the response to publish
/// (C++ `RpcServer::HandleRequest`).
///
/// Runs on the server's dispatch thread; a panic is turned into a 500
/// response, like an exception in the C++ server.
pub trait RpcHandler: Send + Sync + 'static {
    fn handle_request(&self, request: &RpcRequest) -> RpcResponse;
}

impl<F> RpcHandler for F
where
    F: Fn(&RpcRequest) -> RpcResponse + Send + Sync + 'static,
{
    fn handle_request(&self, request: &RpcRequest) -> RpcResponse {
        self(request)
    }
}

/// Cloneable handle to publish responses on a server's response topic.
#[derive(Clone)]
pub struct RpcResponder {
    writer: Arc<rustdds::no_key::DataWriter<RpcRespMsg>>,
}

impl RpcResponder {
    /// Publish `response` for the request `uuid` (C++ `SendResponse`).
    pub fn send_response(&self, uuid: &str, response: &RpcResponse) -> Result<()> {
        let msg = RpcRespMsg {
            uuid: uuid.to_owned(),
            header: response.header.to_json_string(),
            body: response.body.clone(),
        };
        self.writer
            .write(msg, None)
            .map_err(|err| DdsError::PublishFailed(err.to_string()).into())
    }
}

/// DDS RPC server for one service channel.
///
/// Stops when dropped.
pub struct RpcServer {
    responder: RpcResponder,
    service_topic: String,
    stop: Option<oneshot::Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl RpcServer {
    /// Serve `service_topic` (e.g. `rt/LocoApiTopic`; a `Req`/`Resp` suffix is
    /// stripped) on `node` with `handler`.
    pub fn new(node: &DdsNode, service_topic: &str, handler: impl RpcHandler) -> Result<Self> {
        Self::with_shared_handler(node, service_topic, Arc::new(handler))
    }

    /// Like [`Self::new`] with a shared handler.
    pub fn with_shared_handler(
        node: &DdsNode,
        service_topic: &str,
        handler: Arc<dyn RpcHandler>,
    ) -> Result<Self> {
        let service_topic = normalize_service_topic(service_topic);
        let writer = node
            .publisher::<RpcRespMsg>(&rpc_response_topic(&service_topic))?
            .into_inner();
        let reader = node.subscribe_reader::<RpcReqMsg>(&rpc_request_topic(&service_topic))?;
        let responder = RpcResponder {
            writer: Arc::new(writer),
        };

        let (stop_tx, stop_rx) = oneshot::channel::<()>();
        let thread_responder = responder.clone();
        let topic = service_topic.clone();
        let thread = std::thread::Builder::new()
            .name("booster-rpc-server".to_owned())
            .spawn(move || {
                let mut stream = reader.async_sample_stream();
                futures::executor::block_on(async move {
                    let mut stop_rx = stop_rx;
                    loop {
                        let sample = tokio::select! {
                            biased;
                            _ = &mut stop_rx => break,
                            sample = stream.next() => sample,
                        };
                        match sample {
                            Some(Ok(sample)) => {
                                handle_message(
                                    &topic,
                                    handler.as_ref(),
                                    &thread_responder,
                                    sample.into_value(),
                                );
                            }
                            Some(Err(err)) => {
                                tracing::debug!(
                                    target: "booster_sdk::rpc",
                                    service_topic = %topic,
                                    %err,
                                    "rpc server read error"
                                );
                            }
                            None => break,
                        }
                    }
                });
            })
            .map_err(|err| DdsError::InitializationFailed(err.to_string()))?;

        Ok(Self {
            responder,
            service_topic,
            stop: Some(stop_tx),
            thread: Some(thread),
        })
    }

    /// Service channel served (without `Req`/`Resp` suffix).
    #[must_use]
    pub fn service_topic(&self) -> &str {
        &self.service_topic
    }

    /// Publish a response for request `uuid`.
    pub fn send_response(&self, uuid: &str, response: &RpcResponse) -> Result<()> {
        self.responder.send_response(uuid, response)
    }

    /// Cloneable response publisher, e.g. for answering from another thread.
    #[must_use]
    pub fn responder(&self) -> RpcResponder {
        self.responder.clone()
    }

    /// Stop serving requests (C++ `Stop`). Idempotent.
    pub fn stop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(thread) = self.thread.take()
            && thread.thread().id() != std::thread::current().id()
        {
            let _ = thread.join();
        }
    }
}

impl Drop for RpcServer {
    fn drop(&mut self) {
        self.stop();
    }
}

fn handle_message(
    topic: &str,
    handler: &dyn RpcHandler,
    responder: &RpcResponder,
    msg: RpcReqMsg,
) {
    let Some(header) = RequestHeader::parse(&msg.header) else {
        tracing::warn!(
            target: "booster_sdk::rpc",
            service_topic = %topic,
            uuid = %msg.uuid,
            header = %preview_for_log(&msg.header, 200),
            "failed to parse rpc request header"
        );
        let response = RpcResponse::error(
            RPC_STATUS_CODE_BAD_REQUEST,
            "Failed to parse RPC request header",
        );
        send_or_log(topic, responder, &msg.uuid, &response);
        return;
    };

    let fire_and_forget = header.call_mode == RpcCallMode::FireAndForget;
    let request = RpcRequest {
        uuid: msg.uuid,
        header,
        body: msg.body,
    };
    let response = std::panic::catch_unwind(AssertUnwindSafe(|| handler.handle_request(&request)))
        .unwrap_or_else(|_| {
            tracing::warn!(
                target: "booster_sdk::rpc",
                service_topic = %topic,
                uuid = %request.uuid,
                "rpc handler panicked"
            );
            RpcResponse::error(
                RPC_STATUS_CODE_INTERNAL_SERVER_ERROR,
                "RPC handler threw an exception",
            )
        });

    if !fire_and_forget {
        send_or_log(topic, responder, &request.uuid, &response);
    }
}

fn send_or_log(topic: &str, responder: &RpcResponder, uuid: &str, response: &RpcResponse) {
    if let Err(err) = responder.send_response(uuid, response) {
        tracing::warn!(
            target: "booster_sdk::rpc",
            service_topic = %topic,
            uuid,
            %err,
            "failed to publish rpc response"
        );
    }
}
