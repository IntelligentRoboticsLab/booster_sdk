//! RPC client for high-level API requests over DDS.

use serde::{Serialize, de::DeserializeOwned};
use std::time::{Duration, Instant};
use uuid::Uuid;

use rustdds::no_key::DataReader;
use std::sync::{Arc, Mutex};

use crate::types::{DdsError, Result, RpcError};

use super::DdsNode;
use super::messages::{RpcReqMsg, RpcRespMsg};
use super::topics::{loco_request_topic, loco_response_topic};

#[derive(Debug)]
pub struct RpcClientOptions {
    pub domain_id: u16,
    pub default_timeout: Duration,
}

impl Default for RpcClientOptions {
    fn default() -> Self {
        Self {
            domain_id: 0,
            // 5 s is a safe default for most commands. Mode changes are slow
            // (the controller responds -1/pending immediately and only sends
            // the final 0 after the physical transition, which can take 3-5 s)
            // so change_mode passes its own longer timeout.
            default_timeout: Duration::from_secs(5),
        }
    }
}

pub struct RpcClient {
    node: DdsNode,
    request_writer: rustdds::no_key::DataWriter<RpcReqMsg>,
    response_reader: Arc<Mutex<DataReader<RpcRespMsg>>>,
    default_timeout: Duration,
}

impl RpcClient {
    pub fn new(options: RpcClientOptions) -> Result<Self> {
        let node = DdsNode::new(super::DdsConfig {
            domain_id: options.domain_id,
        })?;

        let request_topic = loco_request_topic();
        let response_topic = loco_response_topic();
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

    /// Wait until the locomotion controller's DDS subscriber is discovered.
    ///
    /// DDS participant discovery is asynchronous and can take hundreds of
    /// milliseconds. Call this once after construction before making any RPC
    /// calls to avoid the first request being silently dropped.
    pub async fn wait_for_discovery(&self, timeout: Duration) -> Result<()> {
        use std::time::Instant;
        let deadline = Instant::now() + timeout;
        loop {
            if !self.request_writer.get_matched_subscriptions().is_empty() {
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err(crate::types::DdsError::InitializationFailed(
                    "Timed out waiting for DDS discovery: no matched subscriptions on LocoApiTopicReq".to_string(),
                )
                .into());
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    pub async fn call<P, R>(&self, api_id: i32, params: &P, timeout: Option<Duration>) -> Result<R>
    where
        P: Serialize,
        R: DeserializeOwned + Send + 'static,
    {
        let request_id = Uuid::new_v4().to_string();

        let body = serde_json::to_string(params).map_err(|e| {
            RpcError::BadRequest(format!("Failed to serialize request parameters: {e}"))
        })?;

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

                        if response.status_code == -1 {
                            continue;
                        }

                        if response.status_code != 0 {
                            return Err(RpcError::from_status_code(
                                response.status_code,
                                response.body,
                            )
                            .into());
                        }

                        let result: R = serde_json::from_str(&response.body).map_err(|err| {
                            RpcError::RequestFailed {
                                status: response.status_code,
                                message: format!("Failed to deserialize response: {err}"),
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
