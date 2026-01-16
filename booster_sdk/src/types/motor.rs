//! Motor command and state data structures.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Motor control mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MotorMode {
    /// Servo mode (position/velocity control)
    Servo = 0,

    /// Damping mode (low stiffness)
    Damping = 1,
}

impl From<MotorMode> for u8 {
    fn from(mode: MotorMode) -> Self {
        mode as u8
    }
}

impl TryFrom<u8> for MotorMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MotorMode::Servo),
            1 => Ok(MotorMode::Damping),
            _ => Err(()),
        }
    }
}

/// Motor command for a single joint
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotorCommand {
    /// Control mode
    pub mode: MotorMode,

    /// Target position (radians)
    pub q: f32,

    /// Target velocity (rad/s)
    pub dq: f32,

    /// Feedforward torque (Nm)
    pub tau: f32,

    /// Position gain
    pub kp: f32,

    /// Velocity gain
    pub kd: f32,

    /// Weight/blending factor (0.0 to 1.0)
    pub weight: f32,
}

impl Default for MotorCommand {
    fn default() -> Self {
        Self {
            mode: MotorMode::Damping,
            q: 0.0,
            dq: 0.0,
            tau: 0.0,
            kp: 0.0,
            kd: 0.0,
            weight: 0.0,
        }
    }
}

impl MotorCommand {
    /// Create a new motor command with servo mode
    #[must_use]
    pub fn servo(q: f32, dq: f32, kp: f32, kd: f32) -> Self {
        Self {
            mode: MotorMode::Servo,
            q,
            dq,
            tau: 0.0,
            kp,
            kd,
            weight: 1.0,
        }
    }

    /// Create a damping mode command
    #[must_use]
    pub fn damping() -> Self {
        Self {
            mode: MotorMode::Damping,
            ..Default::default()
        }
    }

    /// Set feedforward torque
    #[must_use]
    pub fn with_torque(mut self, tau: f32) -> Self {
        self.tau = tau;
        self
    }

    /// Set blending weight
    #[must_use]
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }
}

/// Motor state feedback for a single joint
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotorState {
    /// Current mode
    pub mode: MotorMode,
    /// Current position (radians)
    pub q: f32,
    /// Current velocity (rad/s)
    pub dq: f32,
    /// Current acceleration (rad/s²)
    pub ddq: f32,
    /// Estimated torque (N·m)
    pub tau_est: f32,
    /// Motor temperature (°C)
    pub temperature: u8,
    /// Packet loss counter from firmware
    pub lost: u32,
    /// Reserved field for future diagnostics
    pub reserve: [u32; 2],
}

impl Default for MotorState {
    fn default() -> Self {
        Self {
            mode: MotorMode::Damping,
            q: 0.0,
            dq: 0.0,
            ddq: 0.0,
            tau_est: 0.0,
            temperature: 0,
            lost: 0,
            reserve: [0; 2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_f32_eq(actual: f32, expected: f32) {
        assert!((actual - expected).abs() <= f32::EPSILON);
    }

    #[test]
    fn test_motor_command_builder() {
        let cmd = MotorCommand::servo(1.0, 0.5, 10.0, 2.0)
            .with_torque(0.1)
            .with_weight(0.8);

        assert_eq!(cmd.mode, MotorMode::Servo);
        assert_f32_eq(cmd.q, 1.0);
        assert_f32_eq(cmd.dq, 0.5);
        assert_f32_eq(cmd.kp, 10.0);
        assert_f32_eq(cmd.kd, 2.0);
        assert_f32_eq(cmd.tau, 0.1);
        assert_f32_eq(cmd.weight, 0.8);
    }

    #[test]
    fn test_motor_mode_conversion() {
        assert_eq!(MotorMode::try_from(0u8), Ok(MotorMode::Servo));
        assert_eq!(MotorMode::try_from(1u8), Ok(MotorMode::Damping));
        assert_eq!(MotorMode::try_from(2u8), Err(()));

        assert_eq!(u8::from(MotorMode::Servo), 0);
    }

    #[test]
    fn test_weight_clamping() {
        let cmd = MotorCommand::servo(0.0, 0.0, 1.0, 1.0).with_weight(1.5);
        assert_f32_eq(cmd.weight, 1.0_f32);

        let cmd = MotorCommand::servo(0.0, 0.0, 1.0, 1.0).with_weight(-0.5);
        assert_f32_eq(cmd.weight, 0.0_f32);
    }
}
