//! DDS runtime helpers for creating publishers and subscriptions.

use futures::StreamExt;
use serde::{Serialize, de::DeserializeOwned};
use std::net::{IpAddr, Ipv4Addr};
use tokio::sync::mpsc;

use rustdds::{
    DomainParticipant, DomainParticipantBuilder, Publisher, QosPolicyBuilder, Subscriber,
};

use crate::types::{DdsError, Result};

use super::topics::TopicSpec;

#[derive(Default, Debug, Clone)]
pub struct DdsConfig {
    pub domain_id: u16,
}

#[derive(Clone)]
pub struct DdsNode {
    participant: DomainParticipant,
    publisher: Publisher,
    subscriber: Subscriber,
}

impl DdsNode {
    pub fn new(config: DdsConfig) -> Result<Self> {
        let builder = DomainParticipantBuilder::new(config.domain_id)
            .with_only_networks([IpAddr::V4(Ipv4Addr::LOCALHOST)]);
        let participant = builder
            .build()
            .map_err(|err| DdsError::InitializationFailed(err.to_string()))?;
        let qos = QosPolicyBuilder::new().build();
        let publisher = participant
            .create_publisher(&qos)
            .map_err(|err| DdsError::InitializationFailed(err.to_string()))?;
        let subscriber = participant
            .create_subscriber(&qos)
            .map_err(|err| DdsError::InitializationFailed(err.to_string()))?;

        Ok(Self {
            participant,
            publisher,
            subscriber,
        })
    }

    pub fn publisher<T>(&self, spec: &TopicSpec) -> Result<DdsPublisher<T>>
    where
        T: Serialize,
    {
        let topic = spec.create_topic(&self.participant)?;
        let writer = self
            .publisher
            .create_datawriter_no_key_cdr::<T>(&topic, Some(spec.qos.clone()))
            .map_err(|err| DdsError::PublisherCreationFailed {
                topic: spec.name.to_string(),
                reason: err.to_string(),
            })?;
        Ok(DdsPublisher { writer })
    }

    pub fn subscribe_reader<T>(&self, spec: &TopicSpec) -> Result<rustdds::no_key::DataReader<T>>
    where
        T: DeserializeOwned + 'static,
    {
        let topic = spec.create_topic(&self.participant)?;
        self.subscriber
            .create_datareader_no_key_cdr::<T>(&topic, Some(spec.qos.clone()))
            .map_err(|err| DdsError::SubscriberCreationFailed {
                topic: spec.name.to_string(),
                reason: err.to_string(),
            })
            .map_err(Into::into)
    }

    pub fn subscribe<T>(&self, spec: &TopicSpec, buffer: usize) -> Result<DdsSubscription<T>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let topic = spec.create_topic(&self.participant)?;
        let reader = self
            .subscriber
            .create_datareader_no_key_cdr::<T>(&topic, Some(spec.qos.clone()))
            .map_err(|err| DdsError::SubscriberCreationFailed {
                topic: spec.name.to_string(),
                reason: err.to_string(),
            })?;

        let (sender, receiver) = mpsc::channel(buffer);
        let topic_name = spec.name.to_string();
        // Dedicated thread so this works with or without a tokio runtime (the
        // Python bindings call it outside one). The thread sleeps until a sample
        // arrives or the subscription is dropped.
        std::thread::spawn(move || {
            let mut stream = reader.async_sample_stream();
            futures::executor::block_on(async move {
                loop {
                    let sample = tokio::select! {
                        biased;
                        () = sender.closed() => break,
                        sample = stream.next() => sample,
                    };
                    match sample {
                        Some(Ok(sample)) => {
                            if sender.send(sample.into_value()).await.is_err() {
                                break;
                            }
                        }
                        Some(Err(err)) => {
                            tracing::debug!(topic = %topic_name, %err, "dds read error");
                        }
                        None => break,
                    }
                }
            });
        });

        Ok(DdsSubscription { receiver })
    }
}

pub struct DdsPublisher<T: Serialize> {
    writer: rustdds::no_key::DataWriter<T>,
}

impl<T> DdsPublisher<T>
where
    T: Serialize,
{
    pub fn write(&self, message: T) -> Result<()> {
        self.writer
            .write(message, None)
            .map_err(|err| DdsError::PublishFailed(err.to_string()).into())
    }

    pub fn into_inner(self) -> rustdds::no_key::DataWriter<T> {
        self.writer
    }
}

pub struct DdsSubscription<T> {
    receiver: mpsc::Receiver<T>,
}

impl<T> DdsSubscription<T> {
    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}
