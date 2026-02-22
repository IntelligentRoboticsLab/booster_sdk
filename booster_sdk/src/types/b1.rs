//! B1 locomotion, kinematics, and high-level API types.

use serde::{Deserialize, Serialize};

use super::{Hand, RobotMode};

crate::api_id_enum! {
    LocoApiId {
        ChangeMode = 2000,
        Move = 2001,
        RotateHead = 2004,
        WaveHand = 2005,
        RotateHeadWithDirection = 2006,
        LieDown = 2007,
        GetUp = 2008,
        MoveHandEndEffector = 2009,
        ControlGripper = 2010,
        GetFrameTransform = 2011,
        SwitchHandEndEffectorControlMode = 2012,
        ControlDexterousHand = 2013,
        Handshake = 2015,
        Dance = 2016,
        GetMode = 2017,
        GetStatus = 2018,
        PushUp = 2019,
        PlaySound = 2020,
        StopSound = 2021,
        GetRobotInfo = 2022,
        StopHandEndEffector = 2023,
        Shoot = 2024,
        GetUpWithMode = 2025,
        ZeroTorqueDrag = 2026,
        RecordTrajectory = 2027,
        ReplayTrajectory = 2028,
        WholeBodyDance = 2029,
        UpperBodyCustomControl = 2030,
        ResetOdometry = 2031,
        LoadCustomTrainedTraj = 2032,
        ActivateCustomTrainedTraj = 2033,
        UnloadCustomTrainedTraj = 2034,
        EnterWbcGait = 2035,
        ExitWbcGait = 2036,
    }
}

crate::api_id_enum! {
    BodyControl {
        Unknown = 0,
        Damping = 1,
        Prepare = 2,
        HumanlikeGait = 3,
        ProneBody = 4,
        SoccerGait = 5,
        Custom = 6,
        GetUp = 7,
        WholeBodyDance = 8,
        Shoot = 9,
        InsideFoot = 10,
        Goalie = 11,
        WbcGait = 12,
    }
}

crate::api_id_enum! {
    Action {
        Unknown = 0,
        HandShake = 1,
        HandWave = 2,
        HandControl = 3,
        DanceNewYear = 4,
        DanceNezha = 5,
        DanceTowardsFuture = 6,
        GestureDabbing = 7,
        GestureUltraman = 8,
        GestureRespect = 9,
        GestureCheer = 10,
        GestureLuckyCat = 11,
        GestureBoxing = 12,
        ZeroTorqueDrag = 13,
        RecordTraj = 14,
        RunRecordedTraj = 15,
    }
}

crate::api_id_enum! {
    Frame {
        Unknown = -1,
        Body = 0,
        Head = 1,
        LeftHand = 2,
        RightHand = 3,
        LeftFoot = 4,
        RightFoot = 5,
    }
}

crate::api_id_enum! {
    HandAction {
        Open = 0,
        Close = 1,
    }
}

crate::api_id_enum! {
    BoosterHandType {
        InspireHand = 0,
        InspireTouchHand = 2,
        RevoHand = 3,
        Unknown = -1,
    }
}

crate::api_id_enum! {
    DanceId {
        NewYear = 0,
        Nezha = 1,
        TowardsFuture = 2,
        DabbingGesture = 3,
        UltramanGesture = 4,
        RespectGesture = 5,
        CheeringGesture = 6,
        LuckyCatGesture = 7,
        Stop = 1000,
    }
}

crate::api_id_enum! {
    WholeBodyDanceId {
        ArbicDance = 0,
        MichaelDance1 = 1,
        MichaelDance2 = 2,
        MichaelDance3 = 3,
        MoonWalk = 4,
        BoxingStyleKick = 5,
        RoundhouseKick = 6,
    }
}

crate::api_id_enum! {
    JointOrder {
        MuJoCo = 0,
        IsaacLab = 1,
    }
}

crate::api_id_enum! {
    GripperControlMode {
        Position = 0,
        Force = 1,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Orientation {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Posture {
    pub position: Position,
    pub orientation: Orientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub position: Position,
    pub orientation: Quaternion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GripperMotionParameter {
    pub position: i32,
    pub force: i32,
    pub speed: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DexterousFingerParameter {
    pub seq: i32,
    pub angle: i32,
    pub force: i32,
    pub speed: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModeResponse {
    pub mode: i32,
}

impl GetModeResponse {
    #[must_use]
    pub fn mode_enum(&self) -> Option<RobotMode> {
        RobotMode::try_from(self.mode).ok()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStatusResponse {
    pub current_mode: i32,
    pub current_body_control: i32,
    pub current_actions: Vec<i32>,
}

impl GetStatusResponse {
    #[must_use]
    pub fn current_mode_enum(&self) -> Option<RobotMode> {
        RobotMode::try_from(self.current_mode).ok()
    }

    #[must_use]
    pub fn current_body_control_enum(&self) -> Option<BodyControl> {
        BodyControl::try_from(self.current_body_control).ok()
    }

    #[must_use]
    pub fn current_actions_enum(&self) -> Vec<Action> {
        self.current_actions
            .iter()
            .copied()
            .filter_map(|value| Action::try_from(value).ok())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetRobotInfoResponse {
    pub name: String,
    pub nickname: String,
    pub version: String,
    pub model: String,
    pub serial_number: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomModelParams {
    pub action_scale: Vec<f64>,
    pub kp: Vec<f64>,
    pub kd: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomModel {
    pub file_path: String,
    pub params: Vec<CustomModelParams>,
    pub joint_order: JointOrder,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomTrainedTraj {
    pub traj_file_path: String,
    pub model: CustomModel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadCustomTrainedTrajResponse {
    pub tid: String,
}

/// Convenience alias matching the C++ naming.
pub type HandIndex = Hand;
