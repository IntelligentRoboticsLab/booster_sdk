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
}

impl TryFrom<i32> for RobotMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RobotMode::Damping),
            1 => Ok(RobotMode::Prepare),
            2 => Ok(RobotMode::Walking),
            3 => Ok(RobotMode::Custom),
            _ => Err(()),
        }
    }
}

impl From<RobotMode> for i32 {
    fn from(mode: RobotMode) -> Self {
        mode as i32
    }
}

/// Coordinate frames on the robot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Frame {
    /// Robot body center frame
    Body,

    /// Head frame
    Head,

    /// Left hand frame
    LeftHand,

    /// Right hand frame
    RightHand,

    /// Left foot frame
    LeftFoot,

    /// Right foot frame
    RightFoot,
}

impl Frame {
    /// Convert to string representation used in API
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Frame::Body => "body",
            Frame::Head => "head",
            Frame::LeftHand => "left_hand",
            Frame::RightHand => "right_hand",
            Frame::LeftFoot => "left_foot",
            Frame::RightFoot => "right_foot",
        }
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

/// Direction for continuous movement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum Direction {
    /// Positive direction
    Positive = 1,
    /// No movement
    Stop = 0,
    /// Negative direction
    Negative = -1,
}

impl From<Direction> for i32 {
    fn from(direction: Direction) -> Self {
        direction as i32
    }
}

impl TryFrom<i32> for Direction {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Direction::Positive),
            0 => Ok(Direction::Stop),
            -1 => Ok(Direction::Negative),
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

/// Predefined dance routines
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
#[non_exhaustive]
pub enum DanceId {
    NewYear = 0,

    Nezha = 1,

    TowardsFuture = 2,
}

impl From<DanceId> for i32 {
    fn from(dance_id: DanceId) -> Self {
        dance_id as i32
    }
}

impl TryFrom<i32> for DanceId {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DanceId::NewYear),
            1 => Ok(DanceId::Nezha),
            2 => Ok(DanceId::TowardsFuture),
            _ => Err(()),
        }
    }
}

/// Joint indices for B1 robot with 4-DOF arms (23 joints total)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum JointB1 {
    // Head (2 DOF)
    HeadYaw = 0,
    HeadPitch = 1,

    // Left arm (4 DOF)
    LeftShoulderPitch = 2,
    LeftShoulderRoll = 3,
    LeftElbowPitch = 4,
    LeftElbowYaw = 5,

    // Right arm (4 DOF)
    RightShoulderPitch = 6,
    RightShoulderRoll = 7,
    RightElbowPitch = 8,
    RightElbowYaw = 9,

    // Waist (1 DOF)
    Waist = 10,

    // Left leg (6 DOF)
    LeftHipPitch = 11,
    LeftHipRoll = 12,
    LeftHipYaw = 13,
    LeftKneePitch = 14,
    LeftCrankUp = 15,
    LeftCrankDown = 16,

    // Right leg (6 DOF)
    RightHipPitch = 17,
    RightHipRoll = 18,
    RightHipYaw = 19,
    RightKneePitch = 20,
    RightCrankUp = 21,
    RightCrankDown = 22,
}

impl JointB1 {
    pub const COUNT: usize = 23;
}

impl From<JointB1> for usize {
    fn from(joint: JointB1) -> Self {
        joint as usize
    }
}

impl TryFrom<usize> for JointB1 {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(JointB1::HeadYaw),
            1 => Ok(JointB1::HeadPitch),
            2 => Ok(JointB1::LeftShoulderPitch),
            3 => Ok(JointB1::LeftShoulderRoll),
            4 => Ok(JointB1::LeftElbowPitch),
            5 => Ok(JointB1::LeftElbowYaw),
            6 => Ok(JointB1::RightShoulderPitch),
            7 => Ok(JointB1::RightShoulderRoll),
            8 => Ok(JointB1::RightElbowPitch),
            9 => Ok(JointB1::RightElbowYaw),
            10 => Ok(JointB1::Waist),
            11 => Ok(JointB1::LeftHipPitch),
            12 => Ok(JointB1::LeftHipRoll),
            13 => Ok(JointB1::LeftHipYaw),
            14 => Ok(JointB1::LeftKneePitch),
            15 => Ok(JointB1::LeftCrankUp),
            16 => Ok(JointB1::LeftCrankDown),
            17 => Ok(JointB1::RightHipPitch),
            18 => Ok(JointB1::RightHipRoll),
            19 => Ok(JointB1::RightHipYaw),
            20 => Ok(JointB1::RightKneePitch),
            21 => Ok(JointB1::RightCrankUp),
            22 => Ok(JointB1::RightCrankDown),
            _ => Err(()),
        }
    }
}

/// Joint indices for B1 robot with 7-DOF arms (29 joints total)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum JointB1_7DOF {
    // Head (2 DOF)
    HeadYaw = 0,
    HeadPitch = 1,

    // Left arm (7 DOF)
    LeftShoulderPitch = 2,
    LeftShoulderRoll = 3,
    LeftElbowPitch = 4,
    LeftElbowYaw = 5,
    LeftWristPitch = 6,
    LeftWristYaw = 7,
    LeftHandRoll = 8,

    // Right arm (7 DOF)
    RightShoulderPitch = 9,
    RightShoulderRoll = 10,
    RightElbowPitch = 11,
    RightElbowYaw = 12,
    RightWristPitch = 13,
    RightWristYaw = 14,
    RightHandRoll = 15,

    // Waist (1 DOF)
    Waist = 16,

    // Left leg (6 DOF)
    LeftHipPitch = 17,
    LeftHipRoll = 18,
    LeftHipYaw = 19,
    LeftKneePitch = 20,
    LeftCrankUp = 21,
    LeftCrankDown = 22,

    // Right leg (6 DOF)
    RightHipPitch = 23,
    RightHipRoll = 24,
    RightHipYaw = 25,
    RightKneePitch = 26,
    RightCrankUp = 27,
    RightCrankDown = 28,
}

impl JointB1_7DOF {
    pub const COUNT: usize = 29;
}

impl From<JointB1_7DOF> for usize {
    fn from(joint: JointB1_7DOF) -> Self {
        joint as usize
    }
}

impl TryFrom<usize> for JointB1_7DOF {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(JointB1_7DOF::HeadYaw),
            1 => Ok(JointB1_7DOF::HeadPitch),
            2 => Ok(JointB1_7DOF::LeftShoulderPitch),
            3 => Ok(JointB1_7DOF::LeftShoulderRoll),
            4 => Ok(JointB1_7DOF::LeftElbowPitch),
            5 => Ok(JointB1_7DOF::LeftElbowYaw),
            6 => Ok(JointB1_7DOF::LeftWristPitch),
            7 => Ok(JointB1_7DOF::LeftWristYaw),
            8 => Ok(JointB1_7DOF::LeftHandRoll),
            9 => Ok(JointB1_7DOF::RightShoulderPitch),
            10 => Ok(JointB1_7DOF::RightShoulderRoll),
            11 => Ok(JointB1_7DOF::RightElbowPitch),
            12 => Ok(JointB1_7DOF::RightElbowYaw),
            13 => Ok(JointB1_7DOF::RightWristPitch),
            14 => Ok(JointB1_7DOF::RightWristYaw),
            15 => Ok(JointB1_7DOF::RightHandRoll),
            16 => Ok(JointB1_7DOF::Waist),
            17 => Ok(JointB1_7DOF::LeftHipPitch),
            18 => Ok(JointB1_7DOF::LeftHipRoll),
            19 => Ok(JointB1_7DOF::LeftHipYaw),
            20 => Ok(JointB1_7DOF::LeftKneePitch),
            21 => Ok(JointB1_7DOF::LeftCrankUp),
            22 => Ok(JointB1_7DOF::LeftCrankDown),
            23 => Ok(JointB1_7DOF::RightHipPitch),
            24 => Ok(JointB1_7DOF::RightHipRoll),
            25 => Ok(JointB1_7DOF::RightHipYaw),
            26 => Ok(JointB1_7DOF::RightKneePitch),
            27 => Ok(JointB1_7DOF::RightCrankUp),
            28 => Ok(JointB1_7DOF::RightCrankDown),
            _ => Err(()),
        }
    }
}

/// Dexterous hand finger indices (6 DOF per hand)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Finger {
    ThumbRotation = 0,
    Thumb = 1,
    Index = 2,
    Middle = 3,
    Ring = 4,
    Pinky = 5,
}

impl Finger {
    pub const COUNT: usize = 6;
}

impl From<Finger> for usize {
    fn from(finger: Finger) -> Self {
        finger as usize
    }
}

impl TryFrom<usize> for Finger {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Finger::ThumbRotation),
            1 => Ok(Finger::Thumb),
            2 => Ok(Finger::Index),
            3 => Ok(Finger::Middle),
            4 => Ok(Finger::Ring),
            5 => Ok(Finger::Pinky),
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
    fn test_joint_counts() {
        assert_eq!(JointB1::COUNT, 23);
        assert_eq!(JointB1_7DOF::COUNT, 29);
        assert_eq!(Finger::COUNT, 6);
    }

    #[test]
    fn test_joint_indexing() {
        assert_eq!(usize::from(JointB1::HeadYaw), 0);
        assert_eq!(usize::from(JointB1::Waist), 10);
        assert_eq!(usize::from(JointB1::RightCrankDown), 22);

        assert_eq!(JointB1::try_from(0), Ok(JointB1::HeadYaw));
        assert!(JointB1::try_from(23).is_err());
    }
}
