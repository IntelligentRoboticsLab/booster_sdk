//! Low-level telemetry and command data structures.
//!
//! These types describe the raw control and feedback messages exchanged with the
//! Booster robot at the motor and sensor layer.

use super::motor::{MotorCommand, MotorState};
use serde::{Deserialize, Serialize};

/// IMU (Inertial Measurement Unit) state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ImuState {
    /// Roll, pitch, yaw (radians).
    pub rpy: [f32; 3],

    /// Angular velocity (rad/s), [x, y, z].
    pub gyro: [f32; 3],

    /// Linear acceleration (m/s^2), [x, y, z].
    pub acc: [f32; 3],
}

impl Default for ImuState {
    fn default() -> Self {
        Self {
            rpy: [0.0; 3],
            gyro: [0.0; 3],
            acc: [0.0; 3],
        }
    }
}

/// Low-level state message containing IMU data and motor feedback.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LowState {
    /// IMU data.
    pub imu_state: ImuState,

    /// Parallel motor states (e.g., leg motors).
    pub motor_state_parallel: Vec<MotorState>,

    /// Serial motor states (e.g., arm motors).
    pub motor_state_serial: Vec<MotorState>,
}

/// Command type for low-level motor control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CommandType {
    /// Control parallel motors.
    Parallel = 0,

    /// Control serial motors.
    Serial = 1,
}

impl From<CommandType> for u8 {
    fn from(cmd_type: CommandType) -> Self {
        cmd_type as u8
    }
}

impl TryFrom<u8> for CommandType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CommandType::Parallel),
            1 => Ok(CommandType::Serial),
            _ => Err(()),
        }
    }
}

/// Low-level command message containing raw motor commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowCommand {
    /// Command category (parallel or serial motors).
    pub cmd_type: CommandType,

    /// Motor command payload.
    pub motor_cmd: Vec<MotorCommand>,
}

impl Default for LowCommand {
    fn default() -> Self {
        Self {
            cmd_type: CommandType::Parallel,
            motor_cmd: Vec::new(),
        }
    }
}

/// Odometry data from the robot.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Odometry {
    /// X position (meters).
    pub x: f32,

    /// Y position (meters).
    pub y: f32,

    /// Orientation angle (radians).
    pub theta: f32,
}

/// Gripper/hand feedback data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HandData {
    /// Hand index (0 = left, 1 = right).
    pub hand_index: u8,

    /// Current gripper position (0-1000).
    pub position: u16,

    /// Current gripper force.
    pub force: f32,

    /// Status flags.
    pub status: u8,
}

impl Default for HandData {
    fn default() -> Self {
        Self {
            hand_index: 0,
            position: 0,
            force: 0.0,
            status: 0,
        }
    }
}

/// Fall detection event information.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct FallEvent {
    /// Timestamp of fall detection.
    pub timestamp: u64,

    /// Fall detected flag.
    pub detected: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_type_roundtrip() {
        assert_eq!(CommandType::try_from(0u8), Ok(CommandType::Parallel));
        assert_eq!(CommandType::try_from(1u8), Ok(CommandType::Serial));
        assert!(CommandType::try_from(2u8).is_err());

        assert_eq!(u8::from(CommandType::Parallel), 0);
    }

    #[test]
    fn low_command_defaults() {
        let cmd = LowCommand::default();
        assert_eq!(cmd.cmd_type, CommandType::Parallel);
        assert!(cmd.motor_cmd.is_empty());
    }

    #[test]
    fn low_state_defaults() {
        let state = LowState::default();
        assert_eq!(state.motor_state_parallel.len(), 0);
        assert_eq!(state.motor_state_serial.len(), 0);
        assert!(
            state
                .imu_state
                .rpy
                .iter()
                .all(|value| value.abs() < f32::EPSILON)
        );
    }
}
