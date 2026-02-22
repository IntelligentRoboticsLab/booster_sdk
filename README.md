# Booster Robotics SDK

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/IntelligentRoboticsLab/booster_sdk#license)
[![Crates.io](https://img.shields.io/crates/v/booster_sdk.svg)](https://crates.io/crates/booster_sdk)
[![Downloads](https://img.shields.io/crates/d/booster_sdk.svg)](https://crates.io/crates/booster_sdk)
[![Docs](https://docs.rs/booster_sdk/badge.svg)](https://docs.rs/booster_sdk/latest/booster_sdk/)
[![PyPI](https://img.shields.io/pypi/v/booster_sdk.svg)](https://pypi.org/project/booster-sdk/)

This project is a Rust reimplementation of the original [Booster Robotics C++ SDK (`booster_robotics_sdk`)](https://github.com/BoosterRobotics/booster_robotics_sdk) for controlling Booster robots.

In addition to the Rust crate, this repository also provides Python bindings built on top of the Rust implementation.

## 🚧 Project Status

This library is currently in active development and has been tested on a real robot.

## API Examples

### High-Level Locomotion Control

```rust
use booster_sdk::client::BoosterClient;
use booster_sdk::types::{RobotMode, Hand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize and create client
    let client = BoosterClient::new()?;

    // Change to walking mode
    client.change_mode(RobotMode::Walking).await?;

    // Move forward
    client.move_robot(0.5, 0.0, 0.0).await?;

    // Wave hand
    // Publish gripper commands if needed (DDS topic-based control)
    client.publish_gripper_command(&booster_sdk::client::GripperCommand::open(Hand::Right))?;

    // Lie down when done
    client.lie_down().await?;

    Ok(())
}
```

## Experimental Python Bindings

Python wheels are available on [PyPI](https://pypi.org/project/booster-sdk/):

```bash
pip install booster-sdk
```

### Python API Example

Note: Python bindings are experimental.

```python
from booster_sdk.client.booster import BoosterClient
from booster_sdk.types import GripperCommand, Hand, RobotMode

client = BoosterClient()

# Change to walking mode
client.change_mode(RobotMode.WALKING)

# Move forward
client.move_robot(0.5, 0.0, 0.0)

# Open right gripper
client.publish_gripper_command(GripperCommand.open(Hand.RIGHT))
```

The Python bindings currently cover core control flows, including locomotion, gripper control, AI/LUI RPC calls, vision RPC calls, and X5 camera RPC calls.

## Contributing

This SDK is in active development. Contributions are welcome! Please open issues or pull requests for bug fixes, features, or documentation improvements.
