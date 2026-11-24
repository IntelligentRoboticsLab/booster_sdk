//! DDS message types matching the Booster robot topics.
//!
//! CDR encoding is positional, so the field order and widths of every struct
//! here mirror the corresponding C++ IDL header exactly. Additional message
//! families live in [`super::low_level`], [`super::ros`] and
//! [`super::devices`].

use serde::{Deserialize, Serialize};

use crate::types::{ButtonEventType, EnumValue, RemoteControllerEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcReqMsg {
    pub uuid: String,
    pub header: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRespMsg {
    pub uuid: String,
    pub header: String,
    pub body: String,
}

/// Joint health entry of [`RobotStatusDdsMsg`]
/// (`booster_interface::msg::RobotDdsJointStatus`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RobotDdsJointStatus {
    pub name: String,
    pub index: i32,
    pub is_connected: bool,
    pub temperature: i32,
    pub is_limited: i32,
    pub status_code: i32,
    pub temperature_level: i32,
}

/// IMU health entry of [`RobotStatusDdsMsg`]
/// (`booster_interface::msg::RobotDdsImuStatus`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RobotDdsImuStatus {
    pub name: String,
    pub index: i32,
    pub is_connected: bool,
    pub status_code: i32,
}

/// Battery health entry of [`RobotStatusDdsMsg`]
/// (`booster_interface::msg::RobotDdsBatteryStatus`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RobotDdsBatteryStatus {
    pub name: String,
    pub temperature: f32,
    pub soc: f32,
    pub voltage: f32,
    pub status_code: i32,
    pub status_level: i32,
}

/// Device health published on `rt/device_gateway`
/// (`booster_interface::msg::RobotStatusDdsMsg`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RobotStatusDdsMsg {
    pub joint_vec: Vec<RobotDdsJointStatus>,
    pub imu_vec: Vec<RobotDdsImuStatus>,
    pub battery_vec: Vec<RobotDdsBatteryStatus>,
}

/// Motion state (`booster::msg::MotionState`).
///
/// This type is not part of the public C++ IDL headers; the layout was
/// recovered from the compiled C++ SDK (serialization order: `current_mode`,
/// `target_mode`, `is_transitioning`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MotionState {
    pub current_mode: i32,
    pub target_mode: i32,
    pub is_transitioning: bool,
}

/// Battery state (`booster_interface::msg::BatteryState`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct BatteryState {
    /// Battery voltage (V).
    pub voltage: f32,
    /// Battery current (A).
    pub current: f32,
    /// State of charge.
    pub soc: f32,
    /// Averaged battery voltage (V).
    pub average_voltage: f32,
}

/// Physical button event (`booster_interface::msg::ButtonEventMsg`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ButtonEventMsg {
    /// Button identifier.
    pub button: i32,
    /// Button event kind; use `.get()` to decode.
    pub event: EnumValue<ButtonEventType>,
}

/// Remote controller (gamepad) state
/// (`booster_interface::msg::RemoteControllerState`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoteControllerState {
    /// Raw [`RemoteControllerEvent`] value (Linux input event type).
    pub event: u32,
    pub lx: f32,
    pub ly: f32,
    pub rx: f32,
    pub ry: f32,
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub lb: bool,
    pub rb: bool,
    pub lt: bool,
    pub rt: bool,
    pub ls: bool,
    pub rs: bool,
    pub back: bool,
    pub start: bool,
    pub hat_c: bool,
    pub hat_u: bool,
    pub hat_d: bool,
    pub hat_l: bool,
    pub hat_r: bool,
    pub hat_lu: bool,
    pub hat_ld: bool,
    pub hat_ru: bool,
    pub hat_rd: bool,
    pub reserved: u8,
}

impl RemoteControllerState {
    /// Decoded event type, if known.
    #[must_use]
    pub fn event_kind(&self) -> Option<RemoteControllerEvent> {
        i32::try_from(self.event)
            .ok()
            .and_then(|value| RemoteControllerEvent::try_from(value).ok())
    }
}

/// Process state entry (`booster_interface::msg::RobotProcessState`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RobotProcessStatus {
    pub name: String,
    pub pid: i32,
    pub status: i32,
}

/// Name used by the C++ IDL for [`RobotProcessStatus`].
pub type RobotProcessState = RobotProcessStatus;

/// Process states (`booster_interface::msg::RobotProcessStateMsg`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RobotProcessStateMsg {
    pub process_vec: Vec<RobotProcessStatus>,
}

/// Binary payload (`booster_msgs::msg::BinaryData`).
///
/// Not part of the public C++ IDL headers; the layout could not be verified
/// against the C++ SDK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryData {
    pub data: Vec<u8>,
    pub timestamp: i64,
    pub sequence_num: u32,
    pub encoding: String,
}

/// Gripper command published on `rt/gripper_control`.
///
/// **Note:** the C++ SDK 1.8 has no DDS gripper-control topic or
/// `GripperControl` type; grippers are controlled through the loco RPC API
/// (`ControlGripper`). The C++ library does contain an unpublished
/// `booster_msgs::msg::HandDdsMsg` / `HandCommand` / `HandParam` family, but no
/// topic name for it. This type is kept for backwards compatibility and may
/// not be understood by the robot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GripperControl {
    pub hand_index: u8,
    pub position: i32,
    pub force: i32,
    pub speed: i32,
}

/// Single LED color (`booster_interface::msg::LightPixel`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LightPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// LED strip command (`booster_interface::msg::LightControlMsg`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LightControlMsg {
    pub pixels: Vec<LightPixel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeMode {
    /// Raw payload for safe mode (schema not documented in DDS reference).
    pub data: Vec<u8>,
}

/// Bluetooth lifecycle event published by the SDK 1.7 audio service
/// (`booster_msgs::audio::BluetoothEventTopic`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BluetoothEvent {
    pub timestamp_ms: i64,
    pub event_type: String,
    pub address: String,
    pub payload_json: String,
    pub error_code: i32,
}
