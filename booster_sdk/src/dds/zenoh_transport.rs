//! Zenoh backend for [`DdsNode`](super::DdsNode).
//!
//! `zenoh-plugin-dds` forwards each DDS sample as its raw serialized payload (CDR encapsulation
//! header followed by the CDR body) on a key equal to the DDS topic name. This backend publishes
//! and decodes exactly that, so it interoperates with the bridge on a real robot as well as with
//! any Zenoh-native peer using the same convention.

use std::marker::PhantomData;

use byteorder::{BigEndian, LittleEndian};
use rustdds::cdr_encoding;
use rustdds::policy::Reliability;
use serde::{Serialize, de::DeserializeOwned};
use zenoh::Wait;
use zenoh::pubsub::{Publisher, Subscriber};
use zenoh::qos::CongestionControl;
use zenoh::sample::Sample;

use crate::types::{DdsError, Result};

use super::topics::TopicSpec;
use super::transport::ZenohConfig;

/// Representation identifiers from the DDS-XTypes encapsulation header.
const CDR_BE: [u8; 2] = [0x00, 0x00];
const CDR_LE: [u8; 2] = [0x00, 0x01];
const ENCAPSULATION_HEADER_LEN: usize = 4;

#[derive(Clone)]
pub(crate) struct ZenohNode {
    session: zenoh::Session,
    key_prefix: Option<String>,
}

impl ZenohNode {
    pub(crate) fn open(config: &ZenohConfig) -> Result<Self> {
        let session = zenoh::open(session_config(config)?)
            .wait()
            .map_err(|err| DdsError::InitializationFailed(err.to_string()))?;
        Ok(Self {
            session,
            key_prefix: config.key_prefix.clone(),
        })
    }

    fn key(&self, spec: &TopicSpec) -> String {
        match &self.key_prefix {
            Some(prefix) => format!("{}/{}", prefix.trim_end_matches('/'), spec.name),
            None => spec.name.clone(),
        }
    }

    pub(crate) fn publisher<T: Serialize>(&self, spec: &TopicSpec) -> Result<ZenohPublisher<T>> {
        // Reliable DDS writers block instead of dropping, so mirror that for RPC requests.
        let congestion_control = match spec.qos.reliability() {
            Some(Reliability::Reliable { .. }) => CongestionControl::Block,
            _ => CongestionControl::Drop,
        };
        let publisher = self
            .session
            .declare_publisher(self.key(spec))
            .congestion_control(congestion_control)
            .wait()
            .map_err(|err| DdsError::PublisherCreationFailed {
                topic: spec.name.clone(),
                reason: err.to_string(),
            })?;
        Ok(ZenohPublisher {
            publisher,
            message: PhantomData,
        })
    }

    /// Call `on_message` for every sample on the topic, from a Zenoh runtime thread.
    pub(crate) fn subscribe<T, F>(
        &self,
        spec: &TopicSpec,
        on_message: F,
    ) -> Result<ZenohSubscription>
    where
        T: DeserializeOwned,
        F: Fn(T) + Send + Sync + 'static,
    {
        let topic = spec.name.clone();
        let subscriber = self
            .session
            .declare_subscriber(self.key(spec))
            .callback(
                move |sample: Sample| match decode::<T>(&sample.payload().to_bytes()) {
                    Ok(message) => on_message(message),
                    Err(reason) => tracing::warn!(
                        target: "booster_sdk::zenoh",
                        topic = %topic,
                        reason = %reason,
                        "dropping undecodable sample"
                    ),
                },
            )
            .wait()
            .map_err(|err| DdsError::SubscriberCreationFailed {
                topic: spec.name.clone(),
                reason: err.to_string(),
            })?;
        Ok(ZenohSubscription {
            _subscriber: subscriber,
        })
    }
}

pub(crate) struct ZenohPublisher<T> {
    publisher: Publisher<'static>,
    message: PhantomData<fn(T)>,
}

impl<T: Serialize> ZenohPublisher<T> {
    pub(crate) fn write(&self, message: &T) -> Result<()> {
        self.publisher
            .put(encode(message)?)
            .wait()
            .map_err(|err| DdsError::PublishFailed(err.to_string()).into())
    }
}

/// Keeps a Zenoh subscription declared; dropping it undeclares the subscriber.
pub(crate) struct ZenohSubscription {
    _subscriber: Subscriber<()>,
}

fn session_config(config: &ZenohConfig) -> Result<zenoh::Config> {
    let mut session_config = zenoh::Config::default();
    if config.connect.is_empty() {
        return Ok(session_config);
    }

    let endpoints = serde_json::to_string(&config.connect)?;
    for (key, value) in [
        ("mode", r#""client""#),
        ("connect/endpoints", endpoints.as_str()),
        ("scouting/multicast/enabled", "false"),
    ] {
        session_config
            .insert_json5(key, value)
            .map_err(|err| DdsError::InitializationFailed(format!("zenoh config {key}: {err}")))?;
    }
    Ok(session_config)
}

fn encode<T: Serialize>(message: &T) -> Result<Vec<u8>> {
    let mut payload = vec![CDR_LE[0], CDR_LE[1], 0, 0];
    // The serializer counts alignment from its own start, i.e. from the body after the header.
    cdr_encoding::to_writer::<T, LittleEndian, _>(&mut payload, message)
        .map_err(|err| DdsError::PublishFailed(format!("CDR encoding failed: {err}")))?;
    Ok(payload)
}

fn decode<T: DeserializeOwned>(payload: &[u8]) -> std::result::Result<T, String> {
    if payload.len() < ENCAPSULATION_HEADER_LEN {
        return Err(format!(
            "payload of {} bytes is shorter than the CDR encapsulation header",
            payload.len()
        ));
    }
    let (header, body) = payload.split_at(ENCAPSULATION_HEADER_LEN);
    let decoded = match [header[0], header[1]] {
        CDR_LE => cdr_encoding::from_bytes::<T, LittleEndian>(body),
        CDR_BE => cdr_encoding::from_bytes::<T, BigEndian>(body),
        other => {
            return Err(format!(
                "unsupported representation identifier {other:02x?}"
            ));
        }
    };
    decoded
        .map(|(message, _)| message)
        .map_err(|err| err.to_string())
}
