//! Robot-specific types and enums for the B1 robot.

use serde::{Deserialize, Serialize};

/// Robot operational mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
#[non_exhaustive]
pub enum RobotMode {
    /// Damping mode, motors are compliant
    Damping = 0,

    /// Prepare mode, standing pose
    Prepare = 1,

    /// Walking mode, active locomotion
    Walking = 2,

    /// Custom mode, user-defined behavior
    Custom = 3,

    /// Soccer mode
    Soccer = 4,
}

impl TryFrom<i32> for RobotMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RobotMode::Damping),
            1 => Ok(RobotMode::Prepare),
            2 => Ok(RobotMode::Walking),
            3 => Ok(RobotMode::Custom),
            4 => Ok(RobotMode::Soccer),
            _ => Err(()),
        }
    }
}

impl From<RobotMode> for i32 {
    fn from(mode: RobotMode) -> Self {
        mode as i32
    }
}

/// Hand selection (left or right)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(usize)]
pub enum Hand {
    Left = 0,
    Right = 1,
}

impl From<Hand> for usize {
    fn from(hand: Hand) -> Self {
        hand as usize
    }
}

impl From<Hand> for i32 {
    fn from(hand: Hand) -> Self {
        match hand {
            Hand::Left => 0,
            Hand::Right => 1,
        }
    }
}

impl From<Hand> for u8 {
    fn from(hand: Hand) -> Self {
        match hand {
            Hand::Left => 0,
            Hand::Right => 1,
        }
    }
}

impl TryFrom<usize> for Hand {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Hand::Left),
            1 => Ok(Hand::Right),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for Hand {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Hand::Left),
            1 => Ok(Hand::Right),
            _ => Err(()),
        }
    }
}

/// Gripper control mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum GripperMode {
    /// Position-based control
    Position = 0,

    /// Force-based control
    Force = 1,
}

impl From<GripperMode> for i32 {
    fn from(mode: GripperMode) -> Self {
        mode as i32
    }
}

impl TryFrom<i32> for GripperMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GripperMode::Position),
            1 => Ok(GripperMode::Force),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_mode_conversion() {
        assert_eq!(RobotMode::try_from(0), Ok(RobotMode::Damping));
        assert_eq!(RobotMode::try_from(2), Ok(RobotMode::Walking));
        assert_eq!(RobotMode::try_from(99), Err(()));

        assert_eq!(i32::from(RobotMode::Walking), 2);
    }

    #[test]
    fn test_gripper_mode_conversion() {
        assert_eq!(GripperMode::try_from(0), Ok(GripperMode::Position));
        assert_eq!(GripperMode::try_from(1), Ok(GripperMode::Force));
        assert_eq!(GripperMode::try_from(2), Err(()));
    }
}
