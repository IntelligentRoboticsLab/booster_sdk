//! Command parameter types with builders.
//!
//! This module provides ergonomic builder types for constructing robot control commands.

use crate::types::{GripperMode, Hand};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
