//! DDS runtime helpers for creating publishers and subscriptions.

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::mpsc;

use rustdds::{DomainParticipant, Publisher, QosPolicyBuilder, Subscriber};

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
        let participant = DomainParticipant::new(config.domain_id)
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
        std::thread::spawn(move || {
            let mut reader = reader;
            loop {
                match reader.take_next_sample() {
                    Ok(Some(sample)) => {
                        if sender.blocking_send(sample.into_value()).is_err() {
                            break;
                        }
                    }
                    Ok(None) => std::thread::sleep(std::time::Duration::from_millis(5)),
                    Err(_) => std::thread::sleep(std::time::Duration::from_millis(10)),
                }
            }
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
