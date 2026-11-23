//! State and event enums carried by Booster DDS topics.
//!
//! These mirror the enums in `robot_shared.hpp`, `b1_api_const.hpp` and the
//! IDL headers. IDL enums are 32-bit on the wire, so they (de)serialize as
//! `i32` (which is CDR-compatible with the C++ `uint32_t` enums) and from plain
//! integers in JSON.

crate::api_id_enum! {
    /// Linux input event types emitted by a remote controller device
    /// (`RemoteControllerState::event`).
    #[derive(Default)]
    pub RemoteControllerEvent {
        /// No event.
        #[default]
        None = 0,
        /// Analog-axis motion.
        Axis = 0x600,
        /// Directional-hat position change.
        Hat = 0x602,
        /// Button pressed.
        ButtonDown = 0x603,
        /// Button released.
        ButtonUp = 0x604,
        /// Controller device removed.
        Remove = 0x606,
    }
}

crate::api_id_enum! {
    /// Physical button event type (`ButtonEventMsg::event`).
    #[derive(Default)]
    pub ButtonEventType {
        #[default]
        PressDown = 0,
        PressUp = 1,
        SingleClick = 2,
        DoubleClick = 3,
        TripleClick = 4,
        LongPressStart = 5,
        LongPressHold = 6,
        LongPressEnd = 7,
    }
}

crate::api_id_enum! {
    /// Fall-detection state (`FallDownState::fall_down_state`).
    #[derive(Default)]
    pub FallDownStateType {
        #[default]
        IsReady = 0,
        IsFalling = 1,
        HasFallen = 2,
        IsGettingUp = 3,
    }
}

crate::api_id_enum! {
    /// Motor command layout of a [`LowCmd`](crate::dds::LowCmd).
    #[derive(Default)]
    pub CmdType {
        /// Commands are expressed in parallel (ankle-crank) joint space.
        #[default]
        Parallel = 0,
        /// Commands are expressed in serial (URDF) joint space.
        Serial = 1,
    }
}

crate::api_id_enum! {
    /// Posture reported on `rt/prone_body_control_status`.
    #[derive(Default)]
    pub ProneBodyControlPosture {
        #[default]
        Unknown = 0,
        Inactive = 1,
        PushUp = 2,
        LieDown = 3,
        SoccerLocomotion = 4,
        SoccerKicking = 5,
    }
}

crate::api_id_enum! {
    /// Soccer behavior state reported on `rt/robocup_behavior_status`.
    #[derive(Default)]
    pub RobocupBehaviorStatus {
        /// Behavior execution is in progress.
        #[default]
        Running = 0,
        /// The robot is executing a shot.
        Shooting = 1,
        /// The robot is executing a pass.
        Passing = 2,
    }
}

crate::api_id_enum! {
    /// Trained-trajectory execution status reported on `rt/trained_traj_status`.
    #[derive(Default)]
    pub TrainedTrajStatus {
        #[default]
        Idle = 0,
        Running = 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_controller_event_values() {
        assert_eq!(
            RemoteControllerEvent::try_from(0x603),
            Ok(RemoteControllerEvent::ButtonDown)
        );
        assert_eq!(i32::from(RemoteControllerEvent::Remove), 0x606);
    }
}
