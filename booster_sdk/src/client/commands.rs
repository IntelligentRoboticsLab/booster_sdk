//! Command parameter types with builders.
//!
//! This module provides ergonomic builder types for constructing robot control commands.

use crate::types::{Direction, Frame, GripperMode, Hand, Posture, Transform};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Move command parameters for robot locomotion
#[derive(Debug, Clone, Copy, TypedBuilder, Serialize, Deserialize)]
pub struct MoveCommand {
    /// Linear velocity in X direction (forward/backward) in m/s
    #[builder(default = 0.0)]
    pub vx: f32,

    /// Linear velocity in Y direction (left/right) in m/s
    #[builder(default = 0.0)]
    pub vy: f32,

    /// Angular velocity (yaw rate) in rad/s
    #[builder(default = 0.0)]
    pub vyaw: f32,
}

impl MoveCommand {
    /// Create a forward movement command
    #[must_use]
    pub fn forward(speed: f32) -> Self {
        Self {
            vx: speed,
            vy: 0.0,
            vyaw: 0.0,
        }
    }

    /// Create a backward movement command
    #[must_use]
    pub fn backward(speed: f32) -> Self {
        Self {
            vx: -speed,
            vy: 0.0,
            vyaw: 0.0,
        }
    }

    /// Create a turning command (positive = left, negative = right)
    #[must_use]
    pub fn turn(angular_speed: f32) -> Self {
        Self {
            vx: 0.0,
            vy: 0.0,
            vyaw: angular_speed,
        }
    }

    /// Stop all movement
    #[must_use]
    pub fn stop() -> Self {
        Self {
            vx: 0.0,
            vy: 0.0,
            vyaw: 0.0,
        }
    }
}

/// Head rotation command
#[derive(Debug, Clone, Copy, TypedBuilder, Serialize, Deserialize)]
pub struct HeadRotation {
    /// Pitch angle in radians (up/down)
    pub pitch: f32,

    /// Yaw angle in radians (left/right)
    pub yaw: f32,
}

/// Continuous head rotation with direction
#[derive(Debug, Clone, Copy, TypedBuilder, Serialize, Deserialize)]
pub struct HeadRotationContinuous {
    /// Pitch direction and speed
    pub pitch_direction: Direction,

    /// Yaw direction and speed
    pub yaw_direction: Direction,

    /// Speed multiplier (0.0 to 1.0)
    #[builder(default = 0.5)]
    pub speed: f32,
}

/// Hand end-effector pose command
#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
pub struct HandPoseCommand {
    /// Target hand
    pub hand: Hand,

    /// Target pose (position + orientation)
    pub pose: Posture,

    /// Movement duration in seconds
    #[builder(default = 1.0)]
    pub duration: f32,
}

/// Hand end-effector command with auxiliary waypoint
#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
pub struct HandPoseWithAuxCommand {
    /// Target hand
    pub hand: Hand,

    /// Target pose (position + orientation)
    pub pose: Posture,

    /// Auxiliary waypoint pose
    pub aux_pose: Posture,

    /// Movement duration in seconds
    #[builder(default = 1.0)]
    pub duration: f32,
}

/// Transform-based hand movement command
#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
pub struct HandTransformCommand {
    /// Target hand
    pub hand: Hand,

    /// Target transform (position + quaternion rotation)
    pub transform: Transform,

    /// Movement duration in seconds
    #[builder(default = 1.0)]
    pub duration: f32,
}

/// Gripper control command
#[derive(Debug, Clone, Copy, TypedBuilder, Serialize, Deserialize)]
pub struct GripperCommand {
    /// Target hand
    pub hand: Hand,

    /// Control mode (position or force)
    pub mode: GripperMode,

    /// Motion parameter value
    /// - Position mode: 0-1000 (0 = fully open, 1000 = fully closed)
    /// - Force mode: 50-1000 (grasping force)
    pub motion_param: u16,

    /// Movement speed (1-1000)
    #[builder(default = 500)]
    pub speed: u16,
}

impl GripperCommand {
    /// Create a command to open the gripper
    #[must_use]
    pub fn open(hand: Hand) -> Self {
        Self {
            hand,
            mode: GripperMode::Position,
            motion_param: 0,
            speed: 500,
        }
    }

    /// Create a command to close the gripper
    #[must_use]
    pub fn close(hand: Hand) -> Self {
        Self {
            hand,
            mode: GripperMode::Position,
            motion_param: 1000,
            speed: 500,
        }
    }

    /// Create a force-based grasp command
    #[must_use]
    pub fn grasp(hand: Hand, force: u16) -> Self {
        Self {
            hand,
            mode: GripperMode::Force,
            motion_param: force.clamp(50, 1000),
            speed: 500,
        }
    }

    /// Convert to DDS gripper control message.
    #[must_use]
    pub fn to_dds_control(&self) -> crate::dds::GripperControl {
        let (position, force) = match self.mode {
            GripperMode::Position => (self.motion_param as i32, 0),
            GripperMode::Force => (0, self.motion_param as i32),
        };

        crate::dds::GripperControl {
            hand_index: u8::from(self.hand),
            position,
            force,
            speed: self.speed as i32,
        }
    }
}

/// Per-finger control parameters for dexterous hand
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FingerControl {
    /// Target angle in radians
    pub angle: f32,

    /// Target force (0-1000)
    pub force: u16,

    /// Movement speed (1-1000)
    pub speed: u16,
}

impl FingerControl {
    /// Create a new finger control command
    #[must_use]
    pub fn new(angle: f32, force: u16, speed: u16) -> Self {
        Self {
            angle,
            force: force.clamp(0, 1000),
            speed: speed.clamp(1, 1000),
        }
    }
}

/// Dexterous hand control command (6 DOF per hand)
#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
pub struct DexterousHandCommand {
    /// Target hand
    pub hand: Hand,

    /// Thumb rotation control
    pub thumb_rotation: FingerControl,

    /// Thumb flexion control
    pub thumb: FingerControl,

    /// Index finger control
    pub index: FingerControl,

    /// Middle finger control
    pub middle: FingerControl,

    /// Ring finger control
    pub ring: FingerControl,

    /// Pinky finger control
    pub pinky: FingerControl,
}

impl DexterousHandCommand {
    /// Create a command to open all fingers
    #[must_use]
    pub fn open_all(hand: Hand) -> Self {
        let open_finger = FingerControl::new(0.0, 100, 500);
        Self {
            hand,
            thumb_rotation: open_finger,
            thumb: open_finger,
            index: open_finger,
            middle: open_finger,
            ring: open_finger,
            pinky: open_finger,
        }
    }

    /// Create a command to close all fingers
    #[must_use]
    pub fn close_all(hand: Hand) -> Self {
        let close_finger = FingerControl::new(1.5, 500, 500);
        Self {
            hand,
            thumb_rotation: close_finger,
            thumb: close_finger,
            index: close_finger,
            middle: close_finger,
            ring: close_finger,
            pinky: close_finger,
        }
    }

    /// Create a pinch grasp (thumb and index)
    #[must_use]
    pub fn pinch(hand: Hand) -> Self {
        let close = FingerControl::new(1.2, 400, 500);
        let open = FingerControl::new(0.0, 100, 500);
        Self {
            hand,
            thumb_rotation: close,
            thumb: close,
            index: close,
            middle: open,
            ring: open,
            pinky: open,
        }
    }
}

/// Frame transform query
#[derive(Debug, Clone, Copy, TypedBuilder, Serialize, Deserialize)]
pub struct FrameTransformQuery {
    /// Source frame
    pub source: Frame,

    /// Destination frame
    pub destination: Frame,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_f32_eq(actual: f32, expected: f32) {
        assert!((actual - expected).abs() <= f32::EPSILON);
    }

    #[test]
    fn test_move_command_builders() {
        let forward = MoveCommand::forward(0.5);
        assert_f32_eq(forward.vx, 0.5);
        assert_f32_eq(forward.vy, 0.0);

        let turn = MoveCommand::turn(0.3);
        assert_f32_eq(turn.vyaw, 0.3);

        let stop = MoveCommand::stop();
        assert_f32_eq(stop.vx, 0.0);
    }

    #[test]
    fn test_gripper_command_builders() {
        let open = GripperCommand::open(Hand::Left);
        assert_eq!(open.motion_param, 0);
        assert_eq!(open.mode, GripperMode::Position);

        let close = GripperCommand::close(Hand::Right);
        assert_eq!(close.motion_param, 1000);

        let grasp = GripperCommand::grasp(Hand::Left, 600);
        assert_eq!(grasp.mode, GripperMode::Force);
        assert_eq!(grasp.motion_param, 600);
    }

    #[test]
    fn test_dexterous_hand_presets() {
        let open = DexterousHandCommand::open_all(Hand::Left);
        assert_eq!(open.hand, Hand::Left);
        assert_f32_eq(open.thumb.angle, 0.0);

        let pinch = DexterousHandCommand::pinch(Hand::Right);
        assert_eq!(pinch.hand, Hand::Right);
        assert!(pinch.index.angle > 1.0);
        assert_f32_eq(pinch.middle.angle, 0.0);
    }

    #[test]
    fn test_finger_control_clamping() {
        let finger = FingerControl::new(1.0, 1500, 50);
        assert_eq!(finger.force, 1000); // Clamped to max
        assert_eq!(finger.speed, 50);
    }
}
