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

## Installation

Python wheels are available on [PyPI](https://pypi.org/project/booster-sdk/):

```bash
pip install booster-sdk
```

## API Example

```python
from booster_sdk.client.booster import BoosterClient, RobotMode

client = BoosterClient()

# Change to walking mode
client.change_mode(RobotMode.WALKING)

# Move forward
client.move_robot(0.5, 0.0, 0.0)

```

The Rust crate and Python bindings support the Booster 1.7 SDK, including
locomotion, robot/device catalog discovery, gripper and LED control, AI/LUI and
vision RPCs, camera discovery, hand-eye calibration, and audio
device/Bluetooth management.

SDK 1.7 additions include:

- timed head rotation and selectable v1/v2 get-up behavior;
- sensor, hand, robot-model, and camera catalog queries;
- multi-pixel LED updates;
- audio device enumeration and Bluetooth scan/connect/disconnect/forget APIs;
- hand-eye calibration start, status, result, and apply APIs.

## Contributing

This SDK is in active development. Contributions are welcome! Please open issues or pull requests for bug fixes, features, or documentation improvements.
