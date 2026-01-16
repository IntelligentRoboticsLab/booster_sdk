//! Low-level telemetry and command data structures.
//!
//! These types describe the raw control and feedback messages exchanged with the
//! Booster robot at the motor and sensor layer.

use super::motor::{MotorCommand, MotorState};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

impl LowState {
    /// Deserialize a little-endian CDR payload (as sent by FastDDS) into a [`LowState`].
    #[must_use]
    pub fn from_cdr_le(bytes: &[u8]) -> Result<Self, cdr_encoding::Error> {
        cdr_encoding::from_bytes::<Self, byteorder::LittleEndian>(bytes).map(|(msg, _)| msg)
    }
}

/// Command type for low-level motor control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum CommandType {
    /// Control parallel motors.
    Parallel = 0,

    /// Control serial motors.
    Serial = 1,
}

impl From<CommandType> for u32 {
    fn from(cmd_type: CommandType) -> Self {
        cmd_type as u32
    }
}

impl TryFrom<u32> for CommandType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CommandType::Parallel),
            1 => Ok(CommandType::Serial),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for CommandType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        CommandType::try_from(u32::from(value))
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
        assert_eq!(CommandType::try_from(0u32), Ok(CommandType::Parallel));
        assert_eq!(CommandType::try_from(1u32), Ok(CommandType::Serial));
        assert!(CommandType::try_from(2u32).is_err());

        assert_eq!(CommandType::try_from(0u8), Ok(CommandType::Parallel));
        assert_eq!(u32::from(CommandType::Parallel), 0);
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

    #[test]
    fn low_state_cdr_roundtrip() {
        use byteorder::LittleEndian;
        use cdr_encoding::to_vec;

        let mut state = LowState::default();
        state.imu_state.rpy = [1.0, 2.0, 3.0];
        state.motor_state_serial.push(MotorState {
            mode: MotorMode::Servo,
            q: 0.1,
            dq: 0.2,
            ddq: 0.3,
            tau_est: 0.4,
            temperature: 42,
            lost: 7,
            reserve: [9, 10],
        });

        let bytes = to_vec::<_, LittleEndian>(&state).expect("serialize");
        let decoded = LowState::from_cdr_le(&bytes).expect("deserialize");

        assert_eq!(decoded.imu_state.rpy, state.imu_state.rpy);
        assert_eq!(decoded.motor_state_serial.len(), 1);
        let motor = decoded.motor_state_serial[0];
        assert_eq!(motor.mode, MotorMode::Servo);
        assert!((motor.tau_est - 0.4).abs() < f32::EPSILON);
        assert_eq!(motor.temperature, 42);
        assert_eq!(motor.lost, 7);
        assert_eq!(motor.reserve, [9, 10]);
    }
}
