//! DDS runtime helpers for creating publishers and subscriptions.
//!
//! # Lifetimes
//!
//! Every [`DdsPublisher`], [`DdsSubscription`] and [`DdsCallbackSubscription`]
//! owns its DDS endpoint. Dropping it closes the endpoint: the data writer is
//! deleted immediately, and a subscription's background reader thread stops
//! within a few milliseconds and deletes its data reader. Remote participants
//! are notified through regular DDS discovery.

use serde::{Serialize, de::DeserializeOwned};
use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread::JoinHandle,
};
use tokio::sync::{Notify, mpsc};

use futures::StreamExt;
use rustdds::{
    DataReaderStatus, DataWriterStatus, DomainParticipant, DomainParticipantBuilder, Publisher,
    QosPolicyBuilder, StatusEvented, Subscriber, no_key::DataReader,
};

use crate::types::{DdsError, Result};

use super::topics::TopicSpec;

/// Environment variable read by [`DdsConfig::from_env`] for the domain id.
pub const ENV_DOMAIN_ID: &str = "BOOSTER_DOMAIN_ID";
/// Environment variable read by [`DdsConfig::from_env`] for the network
/// selection (see [`DdsNetwork::from_str`]).
pub const ENV_DDS_NETWORK: &str = "BOOSTER_DDS_NETWORK";

/// Which local network interfaces DDS uses for discovery and data.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum DdsNetwork {
    /// Loopback only (`127.0.0.1`). This is the default: the SDK normally runs
    /// on the robot itself, and restricting discovery to loopback avoids
    /// flooding the robot's networks with discovery traffic.
    #[default]
    Localhost,
    /// All interfaces (no filter). Discovery is advertised on every interface.
    All,
    /// Only the interfaces with these IP addresses.
    Addresses(Vec<IpAddr>),
    /// Only the addresses of the named interfaces (e.g. `eth0`, `en0`).
    /// Entries that are IP literals are used as-is. Resolved when the
    /// [`DdsNode`] is created.
    Interfaces(Vec<String>),
}

impl DdsNetwork {
    /// Restrict DDS to the interface with this name or IP address, mirroring
    /// the C++ `ChannelFactory::Init(domain_id, network_interface)` argument.
    pub fn interface(name_or_ip: impl AsRef<str>) -> Self {
        let value = name_or_ip.as_ref().trim();
        match value.parse::<IpAddr>() {
            Ok(ip) => Self::Addresses(vec![ip]),
            Err(_) => Self::Interfaces(vec![value.to_owned()]),
        }
    }

    /// Resolve to the IP filter passed to RustDDS (`None` = no filter).
    pub fn resolve(&self) -> Result<Option<Vec<IpAddr>>> {
        match self {
            Self::Localhost => Ok(Some(vec![IpAddr::V4(Ipv4Addr::LOCALHOST)])),
            Self::All => Ok(None),
            Self::Addresses(addrs) => Ok(Some(addrs.clone())),
            Self::Interfaces(names) => {
                let interfaces = if_addrs::get_if_addrs().map_err(|err| {
                    DdsError::InitializationFailed(format!(
                        "failed to list network interfaces: {err}"
                    ))
                })?;
                let mut addrs = Vec::new();
                for name in names {
                    if let Ok(ip) = name.parse::<IpAddr>() {
                        addrs.push(ip);
                        continue;
                    }
                    let before = addrs.len();
                    addrs.extend(
                        interfaces
                            .iter()
                            .filter(|iface| &iface.name == name)
                            .map(if_addrs::Interface::ip),
                    );
                    if addrs.len() == before {
                        return Err(DdsError::InitializationFailed(format!(
                            "network interface '{name}' not found or has no address"
                        ))
                        .into());
                    }
                }
                Ok(Some(addrs))
            }
        }
    }
}

impl FromStr for DdsNetwork {
    type Err = std::convert::Infallible;

    /// Parse a network selection:
    /// - `""`, `localhost`, `lo`, `loopback` → [`DdsNetwork::Localhost`]
    /// - `all`, `*`, `any` → [`DdsNetwork::All`]
    /// - otherwise a comma-separated list of IP addresses and/or interface
    ///   names, e.g. `192.168.10.10` or `eth0,wlan0`.
    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let value = value.trim();
        match value.to_ascii_lowercase().as_str() {
            "" | "localhost" | "lo" | "loopback" => return Ok(Self::Localhost),
            "all" | "*" | "any" => return Ok(Self::All),
            _ => {}
        }

        let entries: Vec<&str> = value
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if entries.iter().all(|entry| entry.parse::<IpAddr>().is_ok()) {
            Ok(Self::Addresses(
                entries.iter().filter_map(|e| e.parse().ok()).collect(),
            ))
        } else {
            Ok(Self::Interfaces(
                entries.into_iter().map(str::to_owned).collect(),
            ))
        }
    }
}

/// Configuration of a [`DdsNode`].
#[derive(Default, Debug, Clone)]
pub struct DdsConfig {
    /// DDS domain id (default `0`).
    pub domain_id: u16,
    /// Network interfaces to use (default: loopback only).
    pub network: DdsNetwork,
}

impl DdsConfig {
    /// Config for `domain_id` with the default (loopback-only) network.
    #[must_use]
    pub fn new(domain_id: u16) -> Self {
        Self {
            domain_id,
            ..Self::default()
        }
    }

    /// Set the network selection.
    #[must_use]
    pub fn with_network(mut self, network: DdsNetwork) -> Self {
        self.network = network;
        self
    }

    /// Restrict DDS to one interface name or IP address (like the C++
    /// `ChannelFactory::Init(domain_id, network_interface)`).
    #[must_use]
    pub fn with_interface(self, name_or_ip: impl AsRef<str>) -> Self {
        self.with_network(DdsNetwork::interface(name_or_ip))
    }

    /// Build a config from the environment:
    /// - `BOOSTER_DOMAIN_ID` (default `0`)
    /// - `BOOSTER_DDS_NETWORK` (default loopback; see [`DdsNetwork::from_str`])
    ///
    /// Invalid values fall back to the defaults.
    #[must_use]
    pub fn from_env() -> Self {
        let domain_id = std::env::var(ENV_DOMAIN_ID)
            .ok()
            .and_then(|value| value.trim().parse().ok())
            .unwrap_or_default();
        let network = std::env::var(ENV_DDS_NETWORK)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or_default();
        Self { domain_id, network }
    }
}

#[derive(Clone)]
pub struct DdsNode {
    participant: DomainParticipant,
    publisher: Publisher,
    subscriber: Subscriber,
}

impl DdsNode {
    pub fn new(config: DdsConfig) -> Result<Self> {
        let mut builder = DomainParticipantBuilder::new(config.domain_id);
        if let Some(addrs) = config.network.resolve()? {
            builder = builder.with_only_networks(addrs);
        }
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

    /// The underlying RustDDS participant.
    pub fn participant(&self) -> &DomainParticipant {
        &self.participant
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
                topic: spec.name.clone(),
                reason: err.to_string(),
            })?;
        Ok(DdsPublisher {
            writer,
            matched: AtomicUsize::new(0),
        })
    }

    pub fn subscribe_reader<T>(&self, spec: &TopicSpec) -> Result<DataReader<T>>
    where
        T: DeserializeOwned + 'static,
    {
        let topic = spec.create_topic(&self.participant)?;
        self.subscriber
            .create_datareader_no_key_cdr::<T>(&topic, Some(spec.qos.clone()))
            .map_err(|err| DdsError::SubscriberCreationFailed {
                topic: spec.name.clone(),
                reason: err.to_string(),
            })
            .map_err(Into::into)
    }

    /// Subscribe to a topic; samples are delivered through a bounded channel
    /// of size `buffer` (the reader thread blocks when it is full).
    pub fn subscribe<T>(&self, spec: &TopicSpec, buffer: usize) -> Result<DdsSubscription<T>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let reader = self.subscribe_reader::<T>(spec)?;
        let (sender, receiver) = mpsc::channel(buffer.max(1));
        let handle = spawn_reader_thread(&spec.name, reader, move |sample| {
            sender.blocking_send(sample).is_ok()
        })?;
        Ok(DdsSubscription { receiver, handle })
    }

    /// Subscribe to a topic and invoke `callback` for every sample on a
    /// dedicated background thread (like the C++ `ChannelSubscriber` handler).
    ///
    /// The callback runs until the returned handle is dropped or
    /// [`DdsCallbackSubscription::close`] is called.
    pub fn subscribe_callback<T, F>(
        &self,
        spec: &TopicSpec,
        mut callback: F,
    ) -> Result<DdsCallbackSubscription>
    where
        T: DeserializeOwned + Send + 'static,
        F: FnMut(T) + Send + 'static,
    {
        let reader = self.subscribe_reader::<T>(spec)?;
        let handle = spawn_reader_thread(&spec.name, reader, move |sample| {
            callback(sample);
            true
        })?;
        Ok(DdsCallbackSubscription { handle })
    }
}

/// State shared between a subscription handle and its reader thread.
struct ReaderHandle {
    stop: Arc<AtomicBool>,
    wake: Arc<Notify>,
    matched: Arc<AtomicUsize>,
    thread: Option<JoinHandle<()>>,
}

impl ReaderHandle {
    fn matched(&self) -> usize {
        self.matched.load(Ordering::Relaxed)
    }

    fn is_closed(&self) -> bool {
        self.thread.as_ref().is_none_or(JoinHandle::is_finished)
    }

    fn signal_stop(&self) {
        self.stop.store(true, Ordering::Release);
        // `notify_one` stores a permit, so the wakeup is not lost if the
        // reader thread is between its stop check and its next await.
        self.wake.notify_one();
    }

    fn close(&mut self) {
        self.signal_stop();
        if let Some(thread) = self.thread.take()
            && thread.thread().id() != std::thread::current().id()
        {
            let _ = thread.join();
        }
    }
}

impl Drop for ReaderHandle {
    fn drop(&mut self) {
        // Do not join here: the handle may be dropped from inside the reader
        // callback or while the thread is blocked on a full channel. The
        // thread observes the flag (or the closed channel) and exits on its
        // own, dropping the DataReader.
        self.signal_stop();
    }
}

fn spawn_reader_thread<T, F>(
    topic: &str,
    reader: DataReader<T>,
    mut on_sample: F,
) -> Result<ReaderHandle>
where
    T: DeserializeOwned + Send + 'static,
    F: FnMut(T) -> bool + Send + 'static,
{
    let stop = Arc::new(AtomicBool::new(false));
    let wake = Arc::new(Notify::new());
    let matched = Arc::new(AtomicUsize::new(0));
    let thread_stop = Arc::clone(&stop);
    let thread_wake = Arc::clone(&wake);
    let thread_matched = Arc::clone(&matched);
    let topic_name = topic.to_owned();

    // Dedicated thread so this works with or without a tokio runtime (the
    // Python bindings call it outside one). The thread sleeps until a sample
    // or status event arrives, or until the handle signals stop.
    let thread = std::thread::Builder::new()
        .name(format!("dds-reader:{topic}"))
        .spawn(move || {
            let mut samples = reader.async_sample_stream();
            let mut events = samples.async_event_stream();
            futures::executor::block_on(async move {
                loop {
                    if thread_stop.load(Ordering::Acquire) {
                        break;
                    }
                    tokio::select! {
                        biased;
                        () = thread_wake.notified() => {}
                        Some(status) = events.next() => {
                            if let DataReaderStatus::SubscriptionMatched { current, .. } = status {
                                thread_matched.store(
                                    usize::try_from(current.count()).unwrap_or(0),
                                    Ordering::Relaxed,
                                );
                            }
                        }
                        sample = samples.next() => match sample {
                            Some(Ok(sample)) => {
                                if !on_sample(sample.into_value()) {
                                    break;
                                }
                            }
                            Some(Err(err)) => {
                                tracing::debug!(topic = %topic_name, %err, "dds read error");
                            }
                            None => break,
                        },
                    }
                }
            });
        })
        .map_err(|err| DdsError::SubscriberCreationFailed {
            topic: topic.to_owned(),
            reason: err.to_string(),
        })?;

    Ok(ReaderHandle {
        stop,
        wake,
        matched,
        thread: Some(thread),
    })
}

/// Typed DDS data writer. Dropping it deletes the writer.
pub struct DdsPublisher<T: Serialize> {
    writer: rustdds::no_key::DataWriter<T>,
    matched: AtomicUsize,
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

    /// Number of remote/local readers currently matched with this writer.
    ///
    /// Derived from `PublicationMatched` status events (RustDDS does not
    /// implement the direct query), so it reflects events seen so far.
    pub fn matched_subscriptions(&self) -> usize {
        while let Some(status) = self.writer.try_recv_status() {
            if let DataWriterStatus::PublicationMatched { current, .. } = status {
                self.matched.store(
                    usize::try_from(current.count()).unwrap_or(0),
                    Ordering::Relaxed,
                );
            }
        }
        self.matched.load(Ordering::Relaxed)
    }

    pub fn into_inner(self) -> rustdds::no_key::DataWriter<T> {
        self.writer
    }
}

/// Channel-based subscription created by [`DdsNode::subscribe`].
///
/// Dropping it stops the background reader thread and deletes the reader.
pub struct DdsSubscription<T> {
    receiver: mpsc::Receiver<T>,
    handle: ReaderHandle,
}

impl<T> DdsSubscription<T> {
    /// Wait for the next sample. Returns `None` once the subscription is
    /// closed and all buffered samples were received.
    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }

    /// Return a buffered sample without waiting, if any.
    pub fn try_recv(&mut self) -> Option<T> {
        self.receiver.try_recv().ok()
    }

    /// Blocking variant of [`recv`](Self::recv) for synchronous code.
    ///
    /// # Panics
    /// Panics when called from within an asynchronous execution context.
    pub fn blocking_recv(&mut self) -> Option<T> {
        self.receiver.blocking_recv()
    }

    /// Drain all currently buffered samples and return the newest one.
    pub fn latest(&mut self) -> Option<T> {
        let mut latest = None;
        while let Ok(sample) = self.receiver.try_recv() {
            latest = Some(sample);
        }
        latest
    }

    /// Number of writers currently matched with this subscription, as
    /// reported by `SubscriptionMatched` status events.
    pub fn matched_publications(&self) -> usize {
        self.handle.matched()
    }

    /// Stop receiving. Buffered samples can still be read with
    /// [`try_recv`](Self::try_recv)/[`recv`](Self::recv).
    pub fn close(&mut self) {
        self.receiver.close();
        self.handle.signal_stop();
    }
}

/// Callback-based subscription created by [`DdsNode::subscribe_callback`].
///
/// Dropping it stops the background thread (without waiting for it) and
/// deletes the reader.
pub struct DdsCallbackSubscription {
    handle: ReaderHandle,
}

impl DdsCallbackSubscription {
    /// Number of writers currently matched with this subscription.
    pub fn matched_publications(&self) -> usize {
        self.handle.matched()
    }

    /// Whether the background thread has exited.
    pub fn is_closed(&self) -> bool {
        self.handle.is_closed()
    }

    /// Stop the callback thread and wait for it to exit (unless called from
    /// within the callback itself).
    pub fn close(mut self) {
        self.handle.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_network() {
        assert_eq!("".parse::<DdsNetwork>().unwrap(), DdsNetwork::Localhost);
        assert_eq!("All".parse::<DdsNetwork>().unwrap(), DdsNetwork::All);
        assert_eq!(
            "192.168.10.10, 10.0.0.2".parse::<DdsNetwork>().unwrap(),
            DdsNetwork::Addresses(vec![
                "192.168.10.10".parse().unwrap(),
                "10.0.0.2".parse().unwrap()
            ])
        );
        assert_eq!(
            "eth0".parse::<DdsNetwork>().unwrap(),
            DdsNetwork::Interfaces(vec!["eth0".to_owned()])
        );
        assert_eq!(
            DdsNetwork::interface("127.0.0.1"),
            DdsNetwork::Addresses(vec![IpAddr::V4(Ipv4Addr::LOCALHOST)])
        );
    }

    #[test]
    fn resolve_mixed_and_missing() {
        let network = "127.0.0.1,lo0".parse::<DdsNetwork>().unwrap();
        // Either lo0 exists (macOS) or not (Linux); IP literal must survive.
        if let Ok(Some(addrs)) = network.resolve() {
            assert!(addrs.contains(&IpAddr::V4(Ipv4Addr::LOCALHOST)));
        }
        assert!(
            DdsNetwork::Interfaces(vec!["definitely-not-an-iface0".into()])
                .resolve()
                .is_err()
        );
    }

    #[test]
    fn default_config_is_localhost() {
        let config = DdsConfig::default();
        assert_eq!(config.domain_id, 0);
        assert_eq!(
            config.network.resolve().unwrap(),
            Some(vec![IpAddr::V4(Ipv4Addr::LOCALHOST)])
        );
    }
}
