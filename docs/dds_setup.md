## DDS Setup

The Rust SDK communicates directly over DDS using RustDDS. The Booster robot uses
FastDDS, so you only need to ensure the DDS network settings match.

### Network Selection

By default the SDK only uses the **loopback interface** (`127.0.0.1`). The SDK is
normally run on the robot itself, and restricting discovery to loopback avoids
flooding the robot's networks with DDS discovery packets.

To talk to a robot from another machine (or to use a specific interface on the
robot), configure the network on the `DdsConfig`:

```rust
use booster_sdk::dds::{DdsConfig, DdsNetwork, DdsNode};

// Like the C++ `ChannelFactory::Init(0, "eth0")`: interface name or IP address.
let node = DdsNode::new(DdsConfig::new(0).with_interface("eth0"))?;

// Explicit IP addresses of local interfaces.
let config = DdsConfig::new(0)
    .with_network(DdsNetwork::Addresses(vec!["192.168.10.10".parse()?]));

// All interfaces (no filter; discovery is advertised everywhere).
let config = DdsConfig::new(0).with_network(DdsNetwork::All);
```

`DdsConfig::from_env()` reads the configuration from the environment:

| Variable | Meaning | Default |
| --- | --- | --- |
| `BOOSTER_DOMAIN_ID` | DDS domain id | `0` |
| `BOOSTER_DDS_NETWORK` | `localhost`, `all`, or a comma-separated list of interface names and/or IP addresses (e.g. `eth0` or `192.168.10.10`) | `localhost` |

The high-level clients (`BoosterClient`, `AiClient`, ...) take the same
setting through `RpcClientOptions`:

```rust
use booster_sdk::client::BoosterClient;
use booster_sdk::dds::{DdsNetwork, RpcClientOptions};

// Explicit interface:
let options = RpcClientOptions::default()
    .with_network(DdsNetwork::Interfaces(vec!["eth0".into()]));
let client = BoosterClient::with_options(options)?;

// Or from BOOSTER_DOMAIN_ID / BOOSTER_DDS_NETWORK:
let client = BoosterClient::with_options(RpcClientOptions::from_env())?;
```

### Network Requirements

When using a non-loopback network:

- **Same Subnet:** DDS participants should be on the same network (192.168.10.x recommended).
- **Multicast Enabled:** UDP multicast must be allowed.
- **Open Ports:** UDP 7400-7500 should be accessible.
- **Low Latency:** Wired connection recommended.

### Domain ID

The default DDS domain ID is `0`. You can override the domain ID when creating a
client or by setting `BOOSTER_DOMAIN_ID` in examples.

### Topics and QoS

Topic helpers live in `booster_sdk::dds` (e.g. `low_state_topic()`,
`joint_ctrl_topic()`, `fall_down_topic()`, `tf_topic()`). Custom topics can be
built with `TopicSpec::new(name, type_name)` and adjusted with `.reliable()`,
`.best_effort()`, `.keep_last(n)`, `.transient_local()` or `.with_qos(..)`.

State topics default to best-effort readers, which match both reliable and
best-effort writers. Command topics the SDK publishes (`rt/joint_ctrl`,
`rt/kick_ball`) use reliable writers, matching the C++ SDK defaults.

Dropping a `DdsPublisher`, `DdsSubscription` or `DdsCallbackSubscription`
closes the underlying DDS writer/reader.

### Troubleshooting

- If no data appears, confirm multicast is enabled on the interface and that the
  correct network is selected (the default is loopback only).
- Ensure firewalls allow UDP traffic on the DDS port range.
- Verify the robot and your machine share the same domain ID.
- `DdsPublisher::matched_subscriptions()` and
  `DdsSubscription::matched_publications()` show whether discovery succeeded.
