//! DDS message types matching the Booster robot topics.

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotDdsJointStatus {
    pub name: String,
    pub index: i32,
    pub is_connected: bool,
    pub temperature: i32,
    pub is_limited: bool,
    pub status_code: i32,
    pub temperature_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotDdsImuStatus {
    pub name: String,
    pub index: i32,
    pub is_connected: bool,
    pub status_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotDdsBatteryStatus {
    pub name: String,
    pub index: i32,
    pub soc: f32,
    pub status_code: i32,
    pub soc_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotStatusDdsMsg {
    pub joint_vec: Vec<RobotDdsJointStatus>,
    pub imu_vec: Vec<RobotDdsImuStatus>,
    pub battery_vec: Vec<RobotDdsBatteryStatus>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotionState {
    pub current_mode: i32,
    pub target_mode: i32,
    pub is_transitioning: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BatteryState {
    pub voltage: f32,
    pub current: f32,
    pub temperature: f32,
    pub soc: f32,
    pub health: i32,
    pub status_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonEventMsg {
    pub event_type: u8,
    pub button_id: u32,
    pub timestamp: i64,
    pub data: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RemoteControllerState {
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
    pub hat_pos: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotProcessStatus {
    pub name: String,
    pub index: i32,
    pub pid: i32,
    pub status: i32,
    pub status_level: i32,
    pub can_restart: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotProcessStateMsg {
    pub process_vec: Vec<RobotProcessStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryData {
    pub data: Vec<u8>,
    pub timestamp: i64,
    pub sequence_num: u32,
    pub encoding: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GripperControl {
    pub hand_index: u8,
    pub position: i32,
    pub force: i32,
    pub speed: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightControlMsg {
    pub pixels: Vec<LightPixel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeMode {
    /// Raw payload for safe mode (schema not documented in DDS reference).
    pub data: Vec<u8>,
}
