## DDS Interoperability

The Rust SDK uses **Zenoh** for communication instead of DDS directly. To interoperate with the Booster (which uses FastDDS) or other DDS-based systems, you need to run `zenoh-bridge-dds`.

### Setting up the DDS Bridge

1. Install `zenoh-bridge-dds`:

   ```bash
   cargo install zenoh-bridge-dds
   ```

2. Run the bridge (in a separate terminal):

  ```bash
  zenoh-bridge-dds
  ```

This will automatically:

- Discover DDS participants on the network
- Create corresponding Zenoh publishers/subscribers
- Bridge messages bidirectionally between DDS and Zenoh

### Configuration

The bridge can be configured via config file:

```json
{
  // Listen on all network interfaces
  listen: {
    endpoints: ["udp/0.0.0.0:7447"]
  },

  // DDS configuration
  plugins: {
    dds: {
      domain: 0,  // DDS domain ID (must match Booster SDK)

      // Optional: limit to specific network interface
      interface: "eth0"
    }
  }
}
```

Save as `zenoh-bridge.json` and run:

```bash
zenoh-bridge-dds -c zenoh-bridge.json
```

### Topic Mapping

The Rust SDK prefixes all Zenoh resources with `domain{id}/`, e.g.:

- `domain0/rpc/loco/*` - Locomotion RPC calls exposed by this crate
- `domain0/rt/*` - Low-level realtime topics bridged from the C++ stack

The bridge automatically handles the mapping between Zenoh key expressions and DDS topic names.

### Network Architecture

```text
┌──────────────┐                ┌──────────────────┐                ┌───────────────┐
│  booster_sdk │◄───── Zenoh ───┤ zenoh-bridge-dds │◄───── DDS ─────┤  Booster Robot│
│    (Rust)    │                │                  │                │  (FastDDS)    │
└──────────────┘                └──────────────────┘                └───────────────┘
```

### Note

The bridge adds minimal latency (typically <1ms) and can run on the robot or on a separate machine on the same network.
