//! B1 locomotion, kinematics, and high-level API types.

use serde::{Deserialize, Serialize};

use super::{Hand, RobotMode};

crate::api_id_enum! {
    pub enum LocoApiId {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum BodyControl {
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

impl From<BodyControl> for i32 {
    fn from(value: BodyControl) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for BodyControl {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Damping),
            2 => Ok(Self::Prepare),
            3 => Ok(Self::HumanlikeGait),
            4 => Ok(Self::ProneBody),
            5 => Ok(Self::SoccerGait),
            6 => Ok(Self::Custom),
            7 => Ok(Self::GetUp),
            8 => Ok(Self::WholeBodyDance),
            9 => Ok(Self::Shoot),
            10 => Ok(Self::InsideFoot),
            11 => Ok(Self::Goalie),
            12 => Ok(Self::WbcGait),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum Action {
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

impl From<Action> for i32 {
    fn from(value: Action) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for Action {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::HandShake),
            2 => Ok(Self::HandWave),
            3 => Ok(Self::HandControl),
            4 => Ok(Self::DanceNewYear),
            5 => Ok(Self::DanceNezha),
            6 => Ok(Self::DanceTowardsFuture),
            7 => Ok(Self::GestureDabbing),
            8 => Ok(Self::GestureUltraman),
            9 => Ok(Self::GestureRespect),
            10 => Ok(Self::GestureCheer),
            11 => Ok(Self::GestureLuckyCat),
            12 => Ok(Self::GestureBoxing),
            13 => Ok(Self::ZeroTorqueDrag),
            14 => Ok(Self::RecordTraj),
            15 => Ok(Self::RunRecordedTraj),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum Frame {
    Unknown = -1,
    Body = 0,
    Head = 1,
    LeftHand = 2,
    RightHand = 3,
    LeftFoot = 4,
    RightFoot = 5,
}

impl From<Frame> for i32 {
    fn from(value: Frame) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for Frame {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::Unknown),
            0 => Ok(Self::Body),
            1 => Ok(Self::Head),
            2 => Ok(Self::LeftHand),
            3 => Ok(Self::RightHand),
            4 => Ok(Self::LeftFoot),
            5 => Ok(Self::RightFoot),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum HandAction {
    Open = 0,
    Close = 1,
}

impl From<HandAction> for i32 {
    fn from(value: HandAction) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for HandAction {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Open),
            1 => Ok(Self::Close),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum BoosterHandType {
    InspireHand = 0,
    InspireTouchHand = 2,
    RevoHand = 3,
    Unknown = -1,
}

impl From<BoosterHandType> for i32 {
    fn from(value: BoosterHandType) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for BoosterHandType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::InspireHand),
            2 => Ok(Self::InspireTouchHand),
            3 => Ok(Self::RevoHand),
            -1 => Ok(Self::Unknown),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum DanceId {
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

impl From<DanceId> for i32 {
    fn from(value: DanceId) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for DanceId {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NewYear),
            1 => Ok(Self::Nezha),
            2 => Ok(Self::TowardsFuture),
            3 => Ok(Self::DabbingGesture),
            4 => Ok(Self::UltramanGesture),
            5 => Ok(Self::RespectGesture),
            6 => Ok(Self::CheeringGesture),
            7 => Ok(Self::LuckyCatGesture),
            1000 => Ok(Self::Stop),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum WholeBodyDanceId {
    ArbicDance = 0,
    MichaelDance1 = 1,
    MichaelDance2 = 2,
    MichaelDance3 = 3,
    MoonWalk = 4,
    BoxingStyleKick = 5,
    RoundhouseKick = 6,
}

impl From<WholeBodyDanceId> for i32 {
    fn from(value: WholeBodyDanceId) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for WholeBodyDanceId {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ArbicDance),
            1 => Ok(Self::MichaelDance1),
            2 => Ok(Self::MichaelDance2),
            3 => Ok(Self::MichaelDance3),
            4 => Ok(Self::MoonWalk),
            5 => Ok(Self::BoxingStyleKick),
            6 => Ok(Self::RoundhouseKick),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum JointOrder {
    MuJoCo = 0,
    IsaacLab = 1,
}

impl From<JointOrder> for i32 {
    fn from(value: JointOrder) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for JointOrder {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MuJoCo),
            1 => Ok(Self::IsaacLab),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum GripperControlMode {
    Position = 0,
    Force = 1,
}

impl From<GripperControlMode> for i32 {
    fn from(value: GripperControlMode) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for GripperControlMode {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Force),
            _ => Err("invalid value"),
        }
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
