# Booster Robotics Rust SDK

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/IntelligentRoboticsLab/booster_sdk#license)
[![Crates.io](https://img.shields.io/crates/v/booster_sdk.svg)](https://crates.io/crates/booster_sdk)
[![Downloads](https://img.shields.io/crates/d/booster_sdk.svg)](https://crates.io/crates/booster_sdk)
[![Docs](https://docs.rs/booster_sdk/badge.svg)](https://docs.rs/booster_sdk/latest/booster_sdk/)
[![PyPI](https://img.shields.io/pypi/v/booster_sdk.svg)](https://pypi.org/project/booster-sdk/)

A Rust SDK for controlling Booster robots based on [Booster Robotics C++ SDK](https://github.com/BoosterRobotics/booster_robotics_sdk).

## 🚧 Project Status

This library is currently in early development. The core architecture and types are defined, but none of it has been tested on
an actual robot yet. The DDS transport layer is implemented using RustDDS.

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

Python bindings for the SDK are available using [PyO3](https://github.com/PyO3/pyo3). These bindings are very experimental!

### Requirements

- Python 3.10 or higher
- Rust toolchain (for building from source)

### Installation

The Python package can be built using pixi:

```bash
pixi run py-build-wheel
```

This will create a wheel file in `booster_sdk_py/dist/` that can be installed with `pip install booster_sdk_py/dist/*.whl`.

### Python API Example

Note: Python bindings are intentionally minimal and expose a subset of the Rust API.

```python
from booster_sdk import BoosterClient, GripperCommand, Hand, RobotMode

client = BoosterClient()

# Change to walking mode
client.change_mode(RobotMode.WALKING)

# Move forward
client.move_robot(0.5, 0.0, 0.0)

# Open right gripper
client.publish_gripper_command(GripperCommand.open(Hand.RIGHT))
```

The Python bindings currently cover basic mode changes, locomotion, and gripper control.

# DDS Setup

The Rust SDK communicates directly over DDS (RustDDS). Please refer to the [DDS Setup Guide](docs/dds_setup.md) for detailed instructions.

## Contributing

This SDK is currently in early development. Contributions are welcome! Please open issues or pull requests for bug fixes, features, or documentation improvements.
