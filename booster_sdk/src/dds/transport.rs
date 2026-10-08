//! Transport selection for SDK clients.

use super::node::DdsConfig;

/// Environment variable that switches the default transport to Zenoh.
///
/// When set, clients built from default options connect as a Zenoh client to this endpoint
/// (e.g. `tcp/127.0.0.1:7447`) instead of joining DDS domain 0.
pub const ZENOH_ENDPOINT_ENV_VAR: &str = "BOOSTER_SDK_ZENOH_ENDPOINT";

/// Middleware used to reach the robot services.
#[derive(Debug, Clone)]
pub enum TransportConfig {
    /// Native DDS, as spoken by the robot firmware.
    Dds(DdsConfig),
    /// Zenoh, using the same CDR payloads on the keys `zenoh-plugin-dds` bridges DDS topics to.
    ///
    /// Requires the `zenoh` cargo feature; creating a client fails without it.
    Zenoh(ZenohConfig),
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

impl TransportConfig {
    /// Zenoh client to `$BOOSTER_SDK_ZENOH_ENDPOINT` when it is set, DDS on domain 0 otherwise.
    #[must_use]
    pub fn from_env() -> Self {
        match std::env::var(ZENOH_ENDPOINT_ENV_VAR) {
            Ok(endpoint) if !endpoint.trim().is_empty() => {
                Self::Zenoh(ZenohConfig::client(endpoint.trim()))
            }
            _ => Self::Dds(DdsConfig::default()),
        }
    }
}

/// Zenoh session settings.
#[derive(Debug, Clone, Default)]
pub struct ZenohConfig {
    /// Endpoints to connect to. With none, the session runs in peer mode with Zenoh's default
    /// discovery; otherwise it is a client of these endpoints with multicast scouting disabled.
    pub connect: Vec<String>,
    /// Prefix prepended to every key, matching the `scope` option of `zenoh-plugin-dds`.
    pub key_prefix: Option<String>,
}

impl ZenohConfig {
    /// Connect as a client to a single endpoint, such as a router at `tcp/127.0.0.1:7447`.
    #[must_use]
    pub fn client(endpoint: impl Into<String>) -> Self {
        Self::default().with_endpoint(endpoint)
    }

    #[must_use]
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.connect.push(endpoint.into());
        self
    }

    #[must_use]
    pub fn with_key_prefix(mut self, key_prefix: impl Into<String>) -> Self {
        self.key_prefix = Some(key_prefix.into());
        self
    }
}
