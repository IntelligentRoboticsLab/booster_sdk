//! Transport nodes for creating publishers and subscriptions.
//!
//! A [`DdsNode`] speaks either native DDS or, with the `zenoh` feature, Zenoh on the keys
//! `zenoh-plugin-dds` bridges DDS topics to. Both carry the same CDR messages.

use serde::{Serialize, de::DeserializeOwned};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;

use futures::StreamExt;
use rustdds::no_key::DataReaderStream;
use rustdds::{
    DomainParticipant, DomainParticipantBuilder, Publisher, QosPolicyBuilder, Subscriber,
};

use crate::types::{DdsError, Result};

use super::topics::TopicSpec;
use super::transport::{TransportConfig, ZenohConfig};
#[cfg(feature = "zenoh")]
use super::zenoh_transport::{ZenohNode, ZenohPublisher, ZenohSubscription};

#[derive(Default, Debug, Clone)]
pub struct DdsConfig {
    pub domain_id: u16,
}

#[derive(Clone)]
pub struct DdsNode {
    backend: Backend,
}

#[derive(Clone)]
enum Backend {
    Dds(DdsBackend),
    #[cfg(feature = "zenoh")]
    Zenoh(ZenohNode),
}

#[derive(Clone)]
struct DdsBackend {
    participant: DomainParticipant,
    publisher: Publisher,
    subscriber: Subscriber,
}

impl DdsBackend {
    fn new(config: &DdsConfig) -> Result<Self> {
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

    fn reader<T>(&self, spec: &TopicSpec) -> Result<rustdds::no_key::DataReader<T>>
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
}

impl DdsNode {
    /// Join a DDS domain.
    pub fn new(config: DdsConfig) -> Result<Self> {
        Ok(Self {
            backend: Backend::Dds(DdsBackend::new(&config)?),
        })
    }

    /// Create a node on the given transport.
    pub fn with_transport(transport: &TransportConfig) -> Result<Self> {
        match transport {
            TransportConfig::Dds(config) => Self::new(config.clone()),
            TransportConfig::Zenoh(config) => Self::zenoh(config),
        }
    }

    /// Open a Zenoh session.
    ///
    /// Blocks while the session opens. Inside a Tokio runtime this needs the multi-threaded
    /// scheduler, because Zenoh panics on the current-thread one.
    #[cfg(feature = "zenoh")]
    pub fn zenoh(config: &ZenohConfig) -> Result<Self> {
        Ok(Self {
            backend: Backend::Zenoh(ZenohNode::open(config)?),
        })
    }

    #[cfg(not(feature = "zenoh"))]
    pub fn zenoh(_config: &ZenohConfig) -> Result<Self> {
        Err(DdsError::InitializationFailed(
            "booster_sdk was built without the `zenoh` feature".to_owned(),
        )
        .into())
    }

    pub fn publisher<T>(&self, spec: &TopicSpec) -> Result<DdsPublisher<T>>
    where
        T: Serialize,
    {
        let inner = match &self.backend {
            Backend::Dds(dds) => {
                let topic = spec.create_topic(&dds.participant)?;
                let writer = dds
                    .publisher
                    .create_datawriter_no_key_cdr::<T>(&topic, Some(spec.qos.clone()))
                    .map_err(|err| DdsError::PublisherCreationFailed {
                        topic: spec.name.to_string(),
                        reason: err.to_string(),
                    })?;
                PublisherInner::Dds(writer)
            }
            #[cfg(feature = "zenoh")]
            Backend::Zenoh(zenoh) => PublisherInner::Zenoh(zenoh.publisher(spec)?),
        };
        Ok(DdsPublisher { inner })
    }

    /// Raw DDS reader for the topic. Only available on the DDS transport.
    pub fn subscribe_reader<T>(&self, spec: &TopicSpec) -> Result<rustdds::no_key::DataReader<T>>
    where
        T: DeserializeOwned + 'static,
    {
        match &self.backend {
            Backend::Dds(dds) => dds.reader(spec),
            #[cfg(feature = "zenoh")]
            Backend::Zenoh(_) => Err(DdsError::SubscriberCreationFailed {
                topic: spec.name.to_string(),
                reason: "raw DDS readers are only available on the DDS transport".to_owned(),
            }
            .into()),
        }
    }

    pub fn subscribe<T>(&self, spec: &TopicSpec, buffer: usize) -> Result<DdsSubscription<T>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let (sender, receiver) = mpsc::channel(buffer);
        match &self.backend {
            Backend::Dds(dds) => {
                let reader = dds.reader::<T>(spec)?;
                std::thread::spawn(move || {
                    let mut reader = reader;
                    loop {
                        match reader.take_next_sample() {
                            Ok(Some(sample)) => {
                                if sender.blocking_send(sample.into_value()).is_err() {
                                    break;
                                }
                            }
                            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
                            Err(_) => std::thread::sleep(Duration::from_millis(10)),
                        }
                    }
                });
                Ok(DdsSubscription {
                    receiver,
                    _guard: SubscriptionGuard::Dds,
                })
            }
            #[cfg(feature = "zenoh")]
            Backend::Zenoh(zenoh) => {
                // Zenoh delivers on its runtime threads, which must not block, so a full buffer
                // drops the newest sample instead of applying backpressure.
                let subscription = zenoh.subscribe(spec, move |message: T| {
                    let _ = sender.try_send(message);
                })?;
                Ok(DdsSubscription {
                    receiver,
                    _guard: SubscriptionGuard::Zenoh {
                        _subscription: subscription,
                    },
                })
            }
        }
    }

    /// Async stream of samples, used for RPC responses.
    pub(crate) fn sample_stream<T>(&self, spec: &TopicSpec) -> Result<SampleStream<T>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        match &self.backend {
            Backend::Dds(dds) => Ok(SampleStream::Dds(
                dds.reader::<T>(spec)?.async_sample_stream(),
            )),
            #[cfg(feature = "zenoh")]
            Backend::Zenoh(zenoh) => {
                let (sender, receiver) = mpsc::unbounded_channel();
                let subscription = zenoh.subscribe(spec, move |message: T| {
                    let _ = sender.send(message);
                })?;
                Ok(SampleStream::Zenoh {
                    receiver,
                    _subscription: subscription,
                })
            }
        }
    }

    /// Call `on_message` for every sample until the returned guard is dropped.
    pub(crate) fn subscribe_callback<T, F>(
        &self,
        spec: &TopicSpec,
        on_message: F,
    ) -> Result<CallbackSubscription>
    where
        T: DeserializeOwned + Send + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        match &self.backend {
            Backend::Dds(dds) => {
                let mut reader = dds.reader::<T>(spec)?;
                let stop = Arc::new(AtomicBool::new(false));
                let thread_stop = Arc::clone(&stop);
                std::thread::spawn(move || {
                    while !thread_stop.load(Ordering::Relaxed) {
                        match reader.take_next_sample() {
                            Ok(Some(sample)) => on_message(sample.into_value()),
                            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
                            Err(_) => std::thread::sleep(Duration::from_millis(10)),
                        }
                    }
                });
                Ok(CallbackSubscription::Dds { stop })
            }
            #[cfg(feature = "zenoh")]
            Backend::Zenoh(zenoh) => Ok(CallbackSubscription::Zenoh {
                _subscription: zenoh.subscribe(spec, on_message)?,
            }),
        }
    }
}

pub struct DdsPublisher<T: Serialize> {
    inner: PublisherInner<T>,
}

enum PublisherInner<T: Serialize> {
    Dds(rustdds::no_key::DataWriter<T>),
    #[cfg(feature = "zenoh")]
    Zenoh(ZenohPublisher<T>),
}

impl<T> DdsPublisher<T>
where
    T: Serialize,
{
    pub fn write(&self, message: T) -> Result<()> {
        match &self.inner {
            PublisherInner::Dds(writer) => writer
                .write(message, None)
                .map_err(|err| DdsError::PublishFailed(err.to_string()).into()),
            #[cfg(feature = "zenoh")]
            PublisherInner::Zenoh(publisher) => publisher.write(&message),
        }
    }

    /// The underlying DDS writer, or `None` on the Zenoh transport.
    pub fn into_dds_writer(self) -> Option<rustdds::no_key::DataWriter<T>> {
        match self.inner {
            PublisherInner::Dds(writer) => Some(writer),
            #[cfg(feature = "zenoh")]
            PublisherInner::Zenoh(_) => None,
        }
    }
}

pub struct DdsSubscription<T> {
    receiver: mpsc::Receiver<T>,
    _guard: SubscriptionGuard,
}

/// Keeps the transport side of a subscription alive. The DDS reader thread stops on its own once
/// the receiver is dropped.
enum SubscriptionGuard {
    Dds,
    #[cfg(feature = "zenoh")]
    Zenoh {
        _subscription: ZenohSubscription,
    },
}

impl<T> DdsSubscription<T> {
    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}

pub(crate) enum SampleStream<T: DeserializeOwned + 'static> {
    Dds(DataReaderStream<T>),
    #[cfg(feature = "zenoh")]
    Zenoh {
        receiver: mpsc::UnboundedReceiver<T>,
        _subscription: ZenohSubscription,
    },
}

impl<T: DeserializeOwned + 'static> SampleStream<T> {
    /// Next sample, `Some(Err(_))` on a receive error, `None` once the stream has closed.
    pub(crate) async fn next(&mut self) -> Option<Result<T>> {
        match self {
            Self::Dds(stream) => stream.next().await.map(|sample| {
                sample
                    .map(|sample| sample.into_value())
                    .map_err(|err| DdsError::ReceiveFailed(err.to_string()).into())
            }),
            #[cfg(feature = "zenoh")]
            Self::Zenoh { receiver, .. } => receiver.recv().await.map(Ok),
        }
    }
}

pub(crate) enum CallbackSubscription {
    Dds {
        stop: Arc<AtomicBool>,
    },
    #[cfg(feature = "zenoh")]
    Zenoh {
        _subscription: ZenohSubscription,
    },
}

impl Drop for CallbackSubscription {
    fn drop(&mut self) {
        match self {
            Self::Dds { stop } => stop.store(true, Ordering::Relaxed),
            #[cfg(feature = "zenoh")]
            Self::Zenoh { .. } => {}
        }
    }
}
