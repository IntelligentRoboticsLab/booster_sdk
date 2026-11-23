//! Low-level control and robot-state messages (`booster_interface::msg`
//! IDLs from `include/booster/idl/b1`).
//!
//! Field order and widths match the C++ IDL headers exactly, since CDR
//! encoding is positional.

use serde::{Deserialize, Serialize};

use crate::types::{
    Action, BodyControl, CmdType, FallDownStateType, ProneBodyControlPosture, RobotMode,
};

use super::ros::Header;

// ---------------------------------------------------------------------------
// rt/joint_ctrl, rt/low_state
// ---------------------------------------------------------------------------

/// Command for a single motor (`booster_interface::msg::MotorCmd`).
///
/// The commanded torque is
/// `tau + kp * (q - q_measured) + kd * (dq - dq_measured)`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct MotorCmd {
    pub mode: u8,
    /// Target position (rad).
    pub q: f32,
    /// Target velocity (rad/s).
    pub dq: f32,
    /// Feed-forward torque (Nm).
    pub tau: f32,
    /// Position gain.
    pub kp: f32,
    /// Velocity gain.
    pub kd: f32,
    /// Blend weight of this command (used by some upper-body controllers).
    pub weight: f32,
}

/// Low-level joint command published on `rt/joint_ctrl`
/// (`booster_interface::msg::LowCmd`).
///
/// `motor_cmd` must contain one entry per joint in the robot's layout, see
/// [`JOINT_CNT`](crate::types::JOINT_CNT) and friends.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LowCmd {
    pub cmd_type: CmdType,
    pub motor_cmd: Vec<MotorCmd>,
}

impl LowCmd {
    /// Create a command with `joint_count` zeroed motor commands.
    #[must_use]
    pub fn new(cmd_type: CmdType, joint_count: usize) -> Self {
        Self {
            cmd_type,
            motor_cmd: vec![MotorCmd::default(); joint_count],
        }
    }
}

/// IMU sample embedded in [`LowState`] (`booster_interface::msg::ImuState`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct ImuState {
    /// Roll, pitch, yaw (rad).
    pub rpy: [f32; 3],
    /// Angular velocity (rad/s).
    pub gyro: [f32; 3],
    /// Linear acceleration (m/s^2).
    pub acc: [f32; 3],
}

/// State of a single motor (`booster_interface::msg::MotorState`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct MotorState {
    pub mode: u8,
    /// Position (rad).
    pub q: f32,
    /// Velocity (rad/s).
    pub dq: f32,
    /// Acceleration (rad/s^2).
    pub ddq: f32,
    /// Estimated torque (Nm).
    pub tau_est: f32,
    /// Temperature (degrees Celsius).
    pub temperature: u8,
    /// Lost-packet counter.
    pub lost: u32,
    pub reserve: [u32; 2],
}

/// Low-level robot state published on `rt/low_state`
/// (`booster_interface::msg::LowState`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LowState {
    pub imu_state: ImuState,
    pub motor_state_parallel: Vec<MotorState>,
    pub motor_state_serial: Vec<MotorState>,
}

// ---------------------------------------------------------------------------
// Robot status topics
// ---------------------------------------------------------------------------

/// Planar odometry published on `rt/odometer_state`
/// (`booster_interface::msg::Odometer`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Odometer {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

/// Fall-detection state published on `rt/fall_down`
/// (`booster_interface::msg::FallDownState`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FallDownState {
    pub fall_down_state: FallDownStateType,
    pub is_recovery_available: bool,
}

/// Current mode, body control and actions published on `rt/robot_states`
/// (`booster_interface::msg::RobotStatesMsg`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RobotStatesMsg {
    /// Raw [`RobotMode`] value.
    pub current_mode: i32,
    /// Raw [`BodyControl`] value.
    pub current_body_control: i32,
    /// Raw [`Action`] values.
    pub current_actions: Vec<i32>,
}

impl RobotStatesMsg {
    /// Decoded robot mode, if known.
    #[must_use]
    pub fn mode(&self) -> Option<RobotMode> {
        RobotMode::try_from(self.current_mode).ok()
    }

    /// Decoded body controller, if known.
    #[must_use]
    pub fn body_control(&self) -> Option<BodyControl> {
        BodyControl::try_from(self.current_body_control).ok()
    }

    /// Decoded actions (unknown values are skipped).
    #[must_use]
    pub fn actions(&self) -> Vec<Action> {
        self.current_actions
            .iter()
            .filter_map(|value| Action::try_from(*value).ok())
            .collect()
    }
}

/// Prone-body controller status published on `rt/prone_body_control_status`
/// (`booster_interface::msg::ProneBodyControlStatus`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProneBodyControlStatus {
    /// Raw [`ProneBodyControlPosture`] value.
    pub posture: i32,
}

impl ProneBodyControlStatus {
    /// Decoded posture, if known.
    #[must_use]
    pub fn posture_kind(&self) -> Option<ProneBodyControlPosture> {
        ProneBodyControlPosture::try_from(self.posture).ok()
    }
}

/// Soccer behavior status published on `rt/robocup_behavior_status`
/// (`booster_interface::msg::RobocupBehaviorStatus`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RobocupBehaviorStatus {
    /// Raw [`crate::types::RobocupBehaviorStatus`] value.
    pub status: i32,
}

impl RobocupBehaviorStatus {
    /// Decoded status, if known.
    #[must_use]
    pub fn status_kind(&self) -> Option<crate::types::RobocupBehaviorStatus> {
        crate::types::RobocupBehaviorStatus::try_from(self.status).ok()
    }
}

/// Identifier of the recorded trajectory being replayed, published on
/// `rt/robot_replay_traj_id` (`booster_interface::msg::RobotReplayTrajID`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RobotReplayTrajID {
    pub id: String,
}

/// Trained-trajectory execution status published on `rt/trained_traj_status`
/// (`booster_interface::msg::TrainedTrajStatus`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TrainedTrajStatus {
    /// Raw [`crate::types::TrainedTrajStatus`] value.
    pub status: i32,
    pub traj_id: String,
}

impl TrainedTrajStatus {
    /// Decoded status, if known.
    #[must_use]
    pub fn status_kind(&self) -> Option<crate::types::TrainedTrajStatus> {
        crate::types::TrainedTrajStatus::try_from(self.status).ok()
    }
}

/// Visual/soccer kick reference command published on `rt/kick_ball`
/// (`brain::msg::Kick`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Kick {
    pub header: Header,
    pub x: f64,
    pub y: f64,
    pub dir: f64,
    pub goal_x: f64,
    pub goal_y: f64,
    pub robot_theta_to_field: f64,
    pub power: f64,
}

// ---------------------------------------------------------------------------
// Dexterous hands
// ---------------------------------------------------------------------------

/// Per-finger feedback (`booster_interface::msg::HandReplyParam`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HandReplyParam {
    pub angle: i32,
    pub force: i32,
    pub current: i32,
    pub error: i32,
    pub status: i32,
    pub temp: i32,
    pub seq: i32,
}

/// Hand actuator feedback published on `rt/booster_hand_data`
/// (`booster_interface::msg::HandReplyData`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HandReplyData {
    pub hand_data: Vec<HandReplyParam>,
    /// See [`crate::types::Hand`] (0 = left, 1 = right).
    pub hand_index: i32,
    /// See [`crate::types::BoosterHandType`].
    pub hand_type: i32,
}

/// Raw touch-sensor arrays per finger (`booster_interface::msg::HandTouchParam`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HandTouchParam {
    pub finger_one: Vec<u8>,
    pub finger_two: Vec<u8>,
    pub finger_three: Vec<u8>,
    pub finger_four: Vec<u8>,
    pub finger_five: Vec<u8>,
    pub finger_palm: Vec<u8>,
}

/// Hand touch-sensor data published on `rt/booster_hand_touch_data`
/// (`booster_interface::msg::HandTouchData`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HandTouchData {
    pub touch_data: HandTouchParam,
    /// See [`crate::types::Hand`] (0 = left, 1 = right).
    pub hand_index: i32,
    /// See [`crate::types::BoosterHandType`].
    pub hand_type: i32,
}
