# Booster Robotics Rust SDK

A Rust SDK for controlling Booster robots based on [Booster Robotics C++ SDK](https://github.com/BoosterRobotics/booster_robotics_sdk).

## 🚧 Project Status

This library is currently in early development. The core architecture and types are defined, but none of it has been tested on
an actual robot yet. Specifically, the `booster_dds` crate's Zenoh communication layer is untested.

## API Examples

### High-Level Locomotion Control

```rust
use booster_client::B1LocoClient;
use booster_types::{RobotMode, Hand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize and create client
    let client = B1LocoClient::new().await?;

    // Change to walking mode
    client.change_mode(RobotMode::Walking).await?;

    // Move forward
    client.move_robot(0.5, 0.0, 0.0).await?;

    // Wave hand
    client.wave_hand(Hand::Right).await?;

    // Lie down when done
    client.lie_down().await?;

    Ok(())
}
```

## Crate Overview

- `booster_types`: Shared data types and error definitions
- `booster_dds`: Low-level DDS communication layer using Zenoh
- `booster_client`: High-level client API for robot control

## Contributing

This SDK is currently in early development. Contributions are welcome! Please open issues or pull requests for bug fixes, features, or documentation improvements.
