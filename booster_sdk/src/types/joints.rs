//! Low-level joint layouts for Booster robots (mirrors `b1_api_const.hpp`).
//!
//! The joint arrays used by [`LowCmd`](crate::dds::LowCmd) and
//! [`LowState`](crate::dds::LowState) are model-specific. Always pair a joint
//! index enum with the matching joint-count constant; sending a command with
//! the wrong layout can address the wrong actuator or cause the robot to
//! reject the message.
//!
//! | Robot model | Joint enum | Joint count |
//! | --- | --- | --- |
//! | K1 | [`JointIndexK1`] | [`JOINT_CNT_K1`] (22) |
//! | T1 | [`JointIndex`] | [`JOINT_CNT`] (23) |
//! | T1 with 7-DoF arms | [`JointIndexWith7DofArm`] | [`JOINT_CNT_7DOF_ARM`] (29) |

/// Number of joints in the T1 layout ([`JointIndex`]).
pub const JOINT_CNT: usize = 23;

/// Number of joints in the K1 layout ([`JointIndexK1`]).
pub const JOINT_CNT_K1: usize = 22;

/// Number of joints in the T1 7-DoF-arm layout ([`JointIndexWith7DofArm`]).
pub const JOINT_CNT_7DOF_ARM: usize = 29;

crate::api_id_enum! {
    /// Joint indices for the 23-joint T1 layout.
    pub JointIndex {
        HeadYaw = 0,
        HeadPitch = 1,
        LeftShoulderPitch = 2,
        LeftShoulderRoll = 3,
        LeftElbowPitch = 4,
        LeftElbowYaw = 5,
        RightShoulderPitch = 6,
        RightShoulderRoll = 7,
        RightElbowPitch = 8,
        RightElbowYaw = 9,
        Waist = 10,
        LeftHipPitch = 11,
        LeftHipRoll = 12,
        LeftHipYaw = 13,
        LeftKneePitch = 14,
        CrankUpLeft = 15,
        CrankDownLeft = 16,
        RightHipPitch = 17,
        RightHipRoll = 18,
        RightHipYaw = 19,
        RightKneePitch = 20,
        CrankUpRight = 21,
        CrankDownRight = 22,
    }
}

crate::api_id_enum! {
    /// Joint indices for the 22-joint K1 layout (no waist joint).
    pub JointIndexK1 {
        HeadYaw = 0,
        HeadPitch = 1,
        LeftShoulderPitch = 2,
        LeftShoulderRoll = 3,
        LeftElbowPitch = 4,
        LeftElbowYaw = 5,
        RightShoulderPitch = 6,
        RightShoulderRoll = 7,
        RightElbowPitch = 8,
        RightElbowYaw = 9,
        LeftHipPitch = 10,
        LeftHipRoll = 11,
        LeftHipYaw = 12,
        LeftKneePitch = 13,
        CrankUpLeft = 14,
        CrankDownLeft = 15,
        RightHipPitch = 16,
        RightHipRoll = 17,
        RightHipYaw = 18,
        RightKneePitch = 19,
        CrankUpRight = 20,
        CrankDownRight = 21,
    }
}

crate::api_id_enum! {
    /// Joint indices for the 29-joint T1 layout with two 7-DoF arms.
    pub JointIndexWith7DofArm {
        HeadYaw = 0,
        HeadPitch = 1,
        LeftShoulderPitch = 2,
        LeftShoulderRoll = 3,
        LeftElbowPitch = 4,
        LeftElbowYaw = 5,
        LeftWristPitch = 6,
        LeftWristYaw = 7,
        LeftHandRoll = 8,
        RightShoulderPitch = 9,
        RightShoulderRoll = 10,
        RightElbowPitch = 11,
        RightElbowYaw = 12,
        RightWristPitch = 13,
        RightWristYaw = 14,
        RightHandRoll = 15,
        Waist = 16,
        LeftHipPitch = 17,
        LeftHipRoll = 18,
        LeftHipYaw = 19,
        LeftKneePitch = 20,
        CrankUpLeft = 21,
        CrankDownLeft = 22,
        RightHipPitch = 23,
        RightHipRoll = 24,
        RightHipYaw = 25,
        RightKneePitch = 26,
        CrankUpRight = 27,
        CrankDownRight = 28,
    }
}

macro_rules! impl_joint_index {
    ($($name:ident),+) => {
        $(
            impl $name {
                /// Position of this joint in the motor command/state arrays.
                #[must_use]
                #[allow(clippy::cast_sign_loss)]
                pub fn index(self) -> usize {
                    i32::from(self) as usize
                }
            }

            impl From<$name> for usize {
                fn from(value: $name) -> Self {
                    value.index()
                }
            }
        )+
    };
}

impl_joint_index!(JointIndex, JointIndexK1, JointIndexWith7DofArm);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joint_counts_match_last_index() {
        assert_eq!(JointIndex::CrankDownRight.index() + 1, JOINT_CNT);
        assert_eq!(JointIndexK1::CrankDownRight.index() + 1, JOINT_CNT_K1);
        assert_eq!(
            JointIndexWith7DofArm::CrankDownRight.index() + 1,
            JOINT_CNT_7DOF_ARM
        );
    }
}
