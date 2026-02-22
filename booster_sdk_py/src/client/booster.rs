use std::sync::Arc;

use booster_sdk::{
    client::loco::{BoosterClient, GripperCommand},
    types::{
        Action, BodyControl, BoosterHandType, CustomModel, CustomModelParams, CustomTrainedTraj,
        DanceId, DexterousFingerParameter, Frame, GetModeResponse, GetRobotInfoResponse,
        GetStatusResponse, GripperControlMode, GripperMode, GripperMotionParameter, Hand,
        HandAction, JointOrder, LoadCustomTrainedTrajResponse, Orientation, Position, Posture,
        Quaternion, RobotMode, Transform, WholeBodyDanceId,
    },
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{runtime::wait_for_future, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "RobotMode", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyRobotMode(RobotMode);

#[pymethods]
impl PyRobotMode {
    #[classattr]
    const UNKNOWN: Self = Self(RobotMode::Unknown);
    #[classattr]
    const DAMPING: Self = Self(RobotMode::Damping);
    #[classattr]
    const PREPARE: Self = Self(RobotMode::Prepare);
    #[classattr]
    const WALKING: Self = Self(RobotMode::Walking);
    #[classattr]
    const CUSTOM: Self = Self(RobotMode::Custom);
    #[classattr]
    const SOCCER: Self = Self(RobotMode::Soccer);

    fn __repr__(&self) -> String {
        match self.0 {
            RobotMode::Unknown => "RobotMode.UNKNOWN".to_string(),
            RobotMode::Damping => "RobotMode.DAMPING".to_string(),
            RobotMode::Prepare => "RobotMode.PREPARE".to_string(),
            RobotMode::Walking => "RobotMode.WALKING".to_string(),
            RobotMode::Custom => "RobotMode.CUSTOM".to_string(),
            RobotMode::Soccer => "RobotMode.SOCCER".to_string(),
            _ => format!("RobotMode({})", i32::from(self.0)),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyRobotMode> for RobotMode {
    fn from(py_mode: PyRobotMode) -> Self {
        py_mode.0
    }
}

impl From<RobotMode> for PyRobotMode {
    fn from(mode: RobotMode) -> Self {
        Self(mode)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Hand", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyHand(Hand);

#[pymethods]
impl PyHand {
    #[classattr]
    const LEFT: Self = Self(Hand::Left);
    #[classattr]
    const RIGHT: Self = Self(Hand::Right);

    fn __repr__(&self) -> String {
        match self.0 {
            Hand::Left => "Hand.LEFT".to_string(),
            Hand::Right => "Hand.RIGHT".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyHand> for Hand {
    fn from(py_hand: PyHand) -> Self {
        py_hand.0
    }
}

impl From<Hand> for PyHand {
    fn from(hand: Hand) -> Self {
        Self(hand)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "GripperMode", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyGripperMode(GripperMode);

#[pymethods]
impl PyGripperMode {
    #[classattr]
    const POSITION: Self = Self(GripperMode::Position);
    #[classattr]
    const FORCE: Self = Self(GripperMode::Force);

    fn __repr__(&self) -> String {
        match self.0 {
            GripperMode::Position => "GripperMode.POSITION".to_string(),
            GripperMode::Force => "GripperMode.FORCE".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyGripperMode> for GripperMode {
    fn from(py_mode: PyGripperMode) -> Self {
        py_mode.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "HandAction", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyHandAction(HandAction);

#[pymethods]
impl PyHandAction {
    #[classattr]
    const OPEN: Self = Self(HandAction::Open);
    #[classattr]
    const CLOSE: Self = Self(HandAction::Close);

    fn __repr__(&self) -> String {
        match self.0 {
            HandAction::Open => "HandAction.OPEN".to_string(),
            HandAction::Close => "HandAction.CLOSE".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyHandAction> for HandAction {
    fn from(py_action: PyHandAction) -> Self {
        py_action.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Frame", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyFrame(Frame);

#[pymethods]
impl PyFrame {
    #[classattr]
    const UNKNOWN: Self = Self(Frame::Unknown);
    #[classattr]
    const BODY: Self = Self(Frame::Body);
    #[classattr]
    const HEAD: Self = Self(Frame::Head);
    #[classattr]
    const LEFT_HAND: Self = Self(Frame::LeftHand);
    #[classattr]
    const RIGHT_HAND: Self = Self(Frame::RightHand);
    #[classattr]
    const LEFT_FOOT: Self = Self(Frame::LeftFoot);
    #[classattr]
    const RIGHT_FOOT: Self = Self(Frame::RightFoot);

    fn __repr__(&self) -> String {
        match self.0 {
            Frame::Unknown => "Frame.UNKNOWN".to_string(),
            Frame::Body => "Frame.BODY".to_string(),
            Frame::Head => "Frame.HEAD".to_string(),
            Frame::LeftHand => "Frame.LEFT_HAND".to_string(),
            Frame::RightHand => "Frame.RIGHT_HAND".to_string(),
            Frame::LeftFoot => "Frame.LEFT_FOOT".to_string(),
            Frame::RightFoot => "Frame.RIGHT_FOOT".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyFrame> for Frame {
    fn from(value: PyFrame) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "GripperControlMode", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyGripperControlMode(GripperControlMode);

#[pymethods]
impl PyGripperControlMode {
    #[classattr]
    const POSITION: Self = Self(GripperControlMode::Position);
    #[classattr]
    const FORCE: Self = Self(GripperControlMode::Force);

    fn __repr__(&self) -> String {
        match self.0 {
            GripperControlMode::Position => "GripperControlMode.POSITION".to_string(),
            GripperControlMode::Force => "GripperControlMode.FORCE".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyGripperControlMode> for GripperControlMode {
    fn from(value: PyGripperControlMode) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "BoosterHandType", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyBoosterHandType(BoosterHandType);

#[pymethods]
impl PyBoosterHandType {
    #[classattr]
    const INSPIRE_HAND: Self = Self(BoosterHandType::InspireHand);
    #[classattr]
    const INSPIRE_TOUCH_HAND: Self = Self(BoosterHandType::InspireTouchHand);
    #[classattr]
    const REVO_HAND: Self = Self(BoosterHandType::RevoHand);
    #[classattr]
    const UNKNOWN: Self = Self(BoosterHandType::Unknown);

    fn __repr__(&self) -> String {
        match self.0 {
            BoosterHandType::InspireHand => "BoosterHandType.INSPIRE_HAND".to_string(),
            BoosterHandType::InspireTouchHand => "BoosterHandType.INSPIRE_TOUCH_HAND".to_string(),
            BoosterHandType::RevoHand => "BoosterHandType.REVO_HAND".to_string(),
            BoosterHandType::Unknown => "BoosterHandType.UNKNOWN".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyBoosterHandType> for BoosterHandType {
    fn from(value: PyBoosterHandType) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "DanceId", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyDanceId(DanceId);

#[pymethods]
impl PyDanceId {
    #[classattr]
    const NEW_YEAR: Self = Self(DanceId::NewYear);
    #[classattr]
    const NEZHA: Self = Self(DanceId::Nezha);
    #[classattr]
    const TOWARDS_FUTURE: Self = Self(DanceId::TowardsFuture);
    #[classattr]
    const DABBING_GESTURE: Self = Self(DanceId::DabbingGesture);
    #[classattr]
    const ULTRAMAN_GESTURE: Self = Self(DanceId::UltramanGesture);
    #[classattr]
    const RESPECT_GESTURE: Self = Self(DanceId::RespectGesture);
    #[classattr]
    const CHEERING_GESTURE: Self = Self(DanceId::CheeringGesture);
    #[classattr]
    const LUCKY_CAT_GESTURE: Self = Self(DanceId::LuckyCatGesture);
    #[classattr]
    const STOP: Self = Self(DanceId::Stop);

    fn __repr__(&self) -> String {
        match self.0 {
            DanceId::NewYear => "DanceId.NEW_YEAR".to_string(),
            DanceId::Nezha => "DanceId.NEZHA".to_string(),
            DanceId::TowardsFuture => "DanceId.TOWARDS_FUTURE".to_string(),
            DanceId::DabbingGesture => "DanceId.DABBING_GESTURE".to_string(),
            DanceId::UltramanGesture => "DanceId.ULTRAMAN_GESTURE".to_string(),
            DanceId::RespectGesture => "DanceId.RESPECT_GESTURE".to_string(),
            DanceId::CheeringGesture => "DanceId.CHEERING_GESTURE".to_string(),
            DanceId::LuckyCatGesture => "DanceId.LUCKY_CAT_GESTURE".to_string(),
            DanceId::Stop => "DanceId.STOP".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyDanceId> for DanceId {
    fn from(value: PyDanceId) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "WholeBodyDanceId", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyWholeBodyDanceId(WholeBodyDanceId);

#[pymethods]
impl PyWholeBodyDanceId {
    #[classattr]
    const ARBIC_DANCE: Self = Self(WholeBodyDanceId::ArbicDance);
    #[classattr]
    const MICHAEL_DANCE_1: Self = Self(WholeBodyDanceId::MichaelDance1);
    #[classattr]
    const MICHAEL_DANCE_2: Self = Self(WholeBodyDanceId::MichaelDance2);
    #[classattr]
    const MICHAEL_DANCE_3: Self = Self(WholeBodyDanceId::MichaelDance3);
    #[classattr]
    const MOON_WALK: Self = Self(WholeBodyDanceId::MoonWalk);
    #[classattr]
    const BOXING_STYLE_KICK: Self = Self(WholeBodyDanceId::BoxingStyleKick);
    #[classattr]
    const ROUNDHOUSE_KICK: Self = Self(WholeBodyDanceId::RoundhouseKick);

    fn __repr__(&self) -> String {
        match self.0 {
            WholeBodyDanceId::ArbicDance => "WholeBodyDanceId.ARBIC_DANCE".to_string(),
            WholeBodyDanceId::MichaelDance1 => "WholeBodyDanceId.MICHAEL_DANCE_1".to_string(),
            WholeBodyDanceId::MichaelDance2 => "WholeBodyDanceId.MICHAEL_DANCE_2".to_string(),
            WholeBodyDanceId::MichaelDance3 => "WholeBodyDanceId.MICHAEL_DANCE_3".to_string(),
            WholeBodyDanceId::MoonWalk => "WholeBodyDanceId.MOON_WALK".to_string(),
            WholeBodyDanceId::BoxingStyleKick => "WholeBodyDanceId.BOXING_STYLE_KICK".to_string(),
            WholeBodyDanceId::RoundhouseKick => "WholeBodyDanceId.ROUNDHOUSE_KICK".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyWholeBodyDanceId> for WholeBodyDanceId {
    fn from(value: PyWholeBodyDanceId) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "JointOrder", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyJointOrder(JointOrder);

#[pymethods]
impl PyJointOrder {
    #[classattr]
    const MUJOCO: Self = Self(JointOrder::MuJoCo);
    #[classattr]
    const ISAAC_LAB: Self = Self(JointOrder::IsaacLab);

    fn __repr__(&self) -> String {
        match self.0 {
            JointOrder::MuJoCo => "JointOrder.MUJOCO".to_string(),
            JointOrder::IsaacLab => "JointOrder.ISAAC_LAB".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyJointOrder> for JointOrder {
    fn from(value: PyJointOrder) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "BodyControl", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyBodyControl(BodyControl);

#[pymethods]
impl PyBodyControl {
    #[classattr]
    const UNKNOWN: Self = Self(BodyControl::Unknown);
    #[classattr]
    const DAMPING: Self = Self(BodyControl::Damping);
    #[classattr]
    const PREPARE: Self = Self(BodyControl::Prepare);
    #[classattr]
    const HUMANLIKE_GAIT: Self = Self(BodyControl::HumanlikeGait);
    #[classattr]
    const PRONE_BODY: Self = Self(BodyControl::ProneBody);
    #[classattr]
    const SOCCER_GAIT: Self = Self(BodyControl::SoccerGait);
    #[classattr]
    const CUSTOM: Self = Self(BodyControl::Custom);
    #[classattr]
    const GET_UP: Self = Self(BodyControl::GetUp);
    #[classattr]
    const WHOLE_BODY_DANCE: Self = Self(BodyControl::WholeBodyDance);
    #[classattr]
    const SHOOT: Self = Self(BodyControl::Shoot);
    #[classattr]
    const INSIDE_FOOT: Self = Self(BodyControl::InsideFoot);
    #[classattr]
    const GOALIE: Self = Self(BodyControl::Goalie);
    #[classattr]
    const WBC_GAIT: Self = Self(BodyControl::WbcGait);

    fn __repr__(&self) -> String {
        match self.0 {
            BodyControl::Unknown => "BodyControl.UNKNOWN".to_string(),
            BodyControl::Damping => "BodyControl.DAMPING".to_string(),
            BodyControl::Prepare => "BodyControl.PREPARE".to_string(),
            BodyControl::HumanlikeGait => "BodyControl.HUMANLIKE_GAIT".to_string(),
            BodyControl::ProneBody => "BodyControl.PRONE_BODY".to_string(),
            BodyControl::SoccerGait => "BodyControl.SOCCER_GAIT".to_string(),
            BodyControl::Custom => "BodyControl.CUSTOM".to_string(),
            BodyControl::GetUp => "BodyControl.GET_UP".to_string(),
            BodyControl::WholeBodyDance => "BodyControl.WHOLE_BODY_DANCE".to_string(),
            BodyControl::Shoot => "BodyControl.SHOOT".to_string(),
            BodyControl::InsideFoot => "BodyControl.INSIDE_FOOT".to_string(),
            BodyControl::Goalie => "BodyControl.GOALIE".to_string(),
            BodyControl::WbcGait => "BodyControl.WBC_GAIT".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<BodyControl> for PyBodyControl {
    fn from(value: BodyControl) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Action", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyAction(Action);

#[pymethods]
impl PyAction {
    #[classattr]
    const UNKNOWN: Self = Self(Action::Unknown);
    #[classattr]
    const HAND_SHAKE: Self = Self(Action::HandShake);
    #[classattr]
    const HAND_WAVE: Self = Self(Action::HandWave);
    #[classattr]
    const HAND_CONTROL: Self = Self(Action::HandControl);
    #[classattr]
    const DANCE_NEW_YEAR: Self = Self(Action::DanceNewYear);
    #[classattr]
    const DANCE_NEZHA: Self = Self(Action::DanceNezha);
    #[classattr]
    const DANCE_TOWARDS_FUTURE: Self = Self(Action::DanceTowardsFuture);
    #[classattr]
    const GESTURE_DABBING: Self = Self(Action::GestureDabbing);
    #[classattr]
    const GESTURE_ULTRAMAN: Self = Self(Action::GestureUltraman);
    #[classattr]
    const GESTURE_RESPECT: Self = Self(Action::GestureRespect);
    #[classattr]
    const GESTURE_CHEER: Self = Self(Action::GestureCheer);
    #[classattr]
    const GESTURE_LUCKY_CAT: Self = Self(Action::GestureLuckyCat);
    #[classattr]
    const GESTURE_BOXING: Self = Self(Action::GestureBoxing);
    #[classattr]
    const ZERO_TORQUE_DRAG: Self = Self(Action::ZeroTorqueDrag);
    #[classattr]
    const RECORD_TRAJ: Self = Self(Action::RecordTraj);
    #[classattr]
    const RUN_RECORDED_TRAJ: Self = Self(Action::RunRecordedTraj);

    fn __repr__(&self) -> String {
        match self.0 {
            Action::Unknown => "Action.UNKNOWN".to_string(),
            Action::HandShake => "Action.HAND_SHAKE".to_string(),
            Action::HandWave => "Action.HAND_WAVE".to_string(),
            Action::HandControl => "Action.HAND_CONTROL".to_string(),
            Action::DanceNewYear => "Action.DANCE_NEW_YEAR".to_string(),
            Action::DanceNezha => "Action.DANCE_NEZHA".to_string(),
            Action::DanceTowardsFuture => "Action.DANCE_TOWARDS_FUTURE".to_string(),
            Action::GestureDabbing => "Action.GESTURE_DABBING".to_string(),
            Action::GestureUltraman => "Action.GESTURE_ULTRAMAN".to_string(),
            Action::GestureRespect => "Action.GESTURE_RESPECT".to_string(),
            Action::GestureCheer => "Action.GESTURE_CHEER".to_string(),
            Action::GestureLuckyCat => "Action.GESTURE_LUCKY_CAT".to_string(),
            Action::GestureBoxing => "Action.GESTURE_BOXING".to_string(),
            Action::ZeroTorqueDrag => "Action.ZERO_TORQUE_DRAG".to_string(),
            Action::RecordTraj => "Action.RECORD_TRAJ".to_string(),
            Action::RunRecordedTraj => "Action.RUN_RECORDED_TRAJ".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<Action> for PyAction {
    fn from(value: Action) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "GripperCommand")]
#[derive(Clone)]
pub struct PyGripperCommand(GripperCommand);

#[pymethods]
impl PyGripperCommand {
    #[new]
    fn new(hand: PyHand, mode: PyGripperMode, motion_param: u16, speed: Option<u16>) -> Self {
        Self(GripperCommand {
            hand: hand.into(),
            mode: mode.into(),
            motion_param,
            speed: speed.unwrap_or(500),
        })
    }

    #[staticmethod]
    fn open(hand: PyHand) -> Self {
        Self(GripperCommand::open(hand.into()))
    }

    #[staticmethod]
    fn close(hand: PyHand) -> Self {
        Self(GripperCommand::close(hand.into()))
    }

    #[staticmethod]
    fn grasp(hand: PyHand, force: u16) -> Self {
        Self(GripperCommand::grasp(hand.into(), force))
    }

    #[getter]
    fn hand(&self) -> PyHand {
        self.0.hand.into()
    }

    #[getter]
    fn mode(&self) -> PyGripperMode {
        PyGripperMode(self.0.mode)
    }

    #[getter]
    fn motion_param(&self) -> u16 {
        self.0.motion_param
    }

    #[getter]
    fn speed(&self) -> u16 {
        self.0.speed
    }

    fn __repr__(&self) -> String {
        format!(
            "GripperCommand(hand={}, mode={}, motion_param={}, speed={})",
            u8::from(self.0.hand),
            i32::from(self.0.mode),
            self.0.motion_param,
            self.0.speed
        )
    }
}

impl From<PyGripperCommand> for GripperCommand {
    fn from(value: PyGripperCommand) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Position")]
#[derive(Clone, Copy)]
pub struct PyPosition(Position);

#[pymethods]
impl PyPosition {
    #[new]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Position { x, y, z })
    }

    #[getter]
    fn x(&self) -> f32 {
        self.0.x
    }

    #[setter]
    fn set_x(&mut self, value: f32) {
        self.0.x = value;
    }

    #[getter]
    fn y(&self) -> f32 {
        self.0.y
    }

    #[setter]
    fn set_y(&mut self, value: f32) {
        self.0.y = value;
    }

    #[getter]
    fn z(&self) -> f32 {
        self.0.z
    }

    #[setter]
    fn set_z(&mut self, value: f32) {
        self.0.z = value;
    }

    fn __repr__(&self) -> String {
        format!("Position(x={}, y={}, z={})", self.0.x, self.0.y, self.0.z)
    }
}

impl From<PyPosition> for Position {
    fn from(value: PyPosition) -> Self {
        value.0
    }
}

impl From<Position> for PyPosition {
    fn from(value: Position) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Orientation")]
#[derive(Clone, Copy)]
pub struct PyOrientation(Orientation);

#[pymethods]
impl PyOrientation {
    #[new]
    fn new(roll: f32, pitch: f32, yaw: f32) -> Self {
        Self(Orientation { roll, pitch, yaw })
    }

    #[getter]
    fn roll(&self) -> f32 {
        self.0.roll
    }

    #[setter]
    fn set_roll(&mut self, value: f32) {
        self.0.roll = value;
    }

    #[getter]
    fn pitch(&self) -> f32 {
        self.0.pitch
    }

    #[setter]
    fn set_pitch(&mut self, value: f32) {
        self.0.pitch = value;
    }

    #[getter]
    fn yaw(&self) -> f32 {
        self.0.yaw
    }

    #[setter]
    fn set_yaw(&mut self, value: f32) {
        self.0.yaw = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "Orientation(roll={}, pitch={}, yaw={})",
            self.0.roll, self.0.pitch, self.0.yaw
        )
    }
}

impl From<PyOrientation> for Orientation {
    fn from(value: PyOrientation) -> Self {
        value.0
    }
}

impl From<Orientation> for PyOrientation {
    fn from(value: Orientation) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Posture")]
#[derive(Clone, Copy)]
pub struct PyPosture(Posture);

#[pymethods]
impl PyPosture {
    #[new]
    fn new(position: PyPosition, orientation: PyOrientation) -> Self {
        Self(Posture {
            position: position.into(),
            orientation: orientation.into(),
        })
    }

    #[getter]
    fn position(&self) -> PyPosition {
        self.0.position.into()
    }

    #[setter]
    fn set_position(&mut self, value: PyPosition) {
        self.0.position = value.into();
    }

    #[getter]
    fn orientation(&self) -> PyOrientation {
        self.0.orientation.into()
    }

    #[setter]
    fn set_orientation(&mut self, value: PyOrientation) {
        self.0.orientation = value.into();
    }

    fn __repr__(&self) -> String {
        format!(
            "Posture(position={}, orientation={})",
            self.position().__repr__(),
            self.orientation().__repr__()
        )
    }
}

impl From<PyPosture> for Posture {
    fn from(value: PyPosture) -> Self {
        value.0
    }
}

impl From<Posture> for PyPosture {
    fn from(value: Posture) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Quaternion")]
#[derive(Clone, Copy)]
pub struct PyQuaternion(Quaternion);

#[pymethods]
impl PyQuaternion {
    #[new]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Quaternion { x, y, z, w })
    }

    #[getter]
    fn x(&self) -> f32 {
        self.0.x
    }

    #[getter]
    fn y(&self) -> f32 {
        self.0.y
    }

    #[getter]
    fn z(&self) -> f32 {
        self.0.z
    }

    #[getter]
    fn w(&self) -> f32 {
        self.0.w
    }

    fn __repr__(&self) -> String {
        format!(
            "Quaternion(x={}, y={}, z={}, w={})",
            self.0.x, self.0.y, self.0.z, self.0.w
        )
    }
}

impl From<Quaternion> for PyQuaternion {
    fn from(value: Quaternion) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Transform")]
#[derive(Clone, Copy)]
pub struct PyTransform(Transform);

#[pymethods]
impl PyTransform {
    #[new]
    fn new(position: PyPosition, orientation: PyQuaternion) -> Self {
        Self(Transform {
            position: position.into(),
            orientation: orientation.0,
        })
    }

    #[getter]
    fn position(&self) -> PyPosition {
        self.0.position.into()
    }

    #[getter]
    fn orientation(&self) -> PyQuaternion {
        self.0.orientation.into()
    }

    fn __repr__(&self) -> String {
        format!(
            "Transform(position={}, orientation={})",
            self.position().__repr__(),
            self.orientation().__repr__()
        )
    }
}

impl From<Transform> for PyTransform {
    fn from(value: Transform) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "GripperMotionParameter")]
#[derive(Clone, Copy)]
pub struct PyGripperMotionParameter(GripperMotionParameter);

#[pymethods]
impl PyGripperMotionParameter {
    #[new]
    fn new(position: i32, force: i32, speed: i32) -> Self {
        Self(GripperMotionParameter {
            position,
            force,
            speed,
        })
    }

    #[getter]
    fn position(&self) -> i32 {
        self.0.position
    }

    #[getter]
    fn force(&self) -> i32 {
        self.0.force
    }

    #[getter]
    fn speed(&self) -> i32 {
        self.0.speed
    }

    fn __repr__(&self) -> String {
        format!(
            "GripperMotionParameter(position={}, force={}, speed={})",
            self.0.position, self.0.force, self.0.speed
        )
    }
}

impl From<PyGripperMotionParameter> for GripperMotionParameter {
    fn from(value: PyGripperMotionParameter) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "DexterousFingerParameter")]
#[derive(Clone, Copy)]
pub struct PyDexterousFingerParameter(DexterousFingerParameter);

#[pymethods]
impl PyDexterousFingerParameter {
    #[new]
    fn new(seq: i32, angle: i32, force: i32, speed: i32) -> Self {
        Self(DexterousFingerParameter {
            seq,
            angle,
            force,
            speed,
        })
    }

    #[getter]
    fn seq(&self) -> i32 {
        self.0.seq
    }

    #[getter]
    fn angle(&self) -> i32 {
        self.0.angle
    }

    #[getter]
    fn force(&self) -> i32 {
        self.0.force
    }

    #[getter]
    fn speed(&self) -> i32 {
        self.0.speed
    }

    fn __repr__(&self) -> String {
        format!(
            "DexterousFingerParameter(seq={}, angle={}, force={}, speed={})",
            self.0.seq, self.0.angle, self.0.force, self.0.speed
        )
    }
}

impl From<PyDexterousFingerParameter> for DexterousFingerParameter {
    fn from(value: PyDexterousFingerParameter) -> Self {
        value.0
    }
}

impl From<DexterousFingerParameter> for PyDexterousFingerParameter {
    fn from(value: DexterousFingerParameter) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "CustomModelParams")]
#[derive(Clone)]
pub struct PyCustomModelParams(CustomModelParams);

#[pymethods]
impl PyCustomModelParams {
    #[new]
    fn new(action_scale: Vec<f64>, kp: Vec<f64>, kd: Vec<f64>) -> Self {
        Self(CustomModelParams {
            action_scale,
            kp,
            kd,
        })
    }

    #[getter]
    fn action_scale(&self) -> Vec<f64> {
        self.0.action_scale.clone()
    }

    #[getter]
    fn kp(&self) -> Vec<f64> {
        self.0.kp.clone()
    }

    #[getter]
    fn kd(&self) -> Vec<f64> {
        self.0.kd.clone()
    }

    fn __repr__(&self) -> String {
        "CustomModelParams(...)".to_string()
    }
}

impl From<PyCustomModelParams> for CustomModelParams {
    fn from(value: PyCustomModelParams) -> Self {
        value.0
    }
}

impl From<CustomModelParams> for PyCustomModelParams {
    fn from(value: CustomModelParams) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "CustomModel")]
#[derive(Clone)]
pub struct PyCustomModel(CustomModel);

#[pymethods]
impl PyCustomModel {
    #[new]
    fn new(file_path: String, params: Vec<PyCustomModelParams>, joint_order: PyJointOrder) -> Self {
        Self(CustomModel {
            file_path,
            params: params.into_iter().map(Into::into).collect(),
            joint_order: joint_order.into(),
        })
    }

    #[getter]
    fn file_path(&self) -> String {
        self.0.file_path.clone()
    }

    #[getter]
    fn params(&self) -> Vec<PyCustomModelParams> {
        self.0.params.clone().into_iter().map(Into::into).collect()
    }

    #[getter]
    fn joint_order(&self) -> PyJointOrder {
        PyJointOrder(self.0.joint_order)
    }

    fn __repr__(&self) -> String {
        format!("CustomModel(file_path='{}', params=...)", self.0.file_path)
    }
}

impl From<PyCustomModel> for CustomModel {
    fn from(value: PyCustomModel) -> Self {
        value.0
    }
}

impl From<CustomModel> for PyCustomModel {
    fn from(value: CustomModel) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "CustomTrainedTraj")]
#[derive(Clone)]
pub struct PyCustomTrainedTraj(CustomTrainedTraj);

#[pymethods]
impl PyCustomTrainedTraj {
    #[new]
    fn new(traj_file_path: String, model: PyCustomModel) -> Self {
        Self(CustomTrainedTraj {
            traj_file_path,
            model: model.into(),
        })
    }

    #[getter]
    fn traj_file_path(&self) -> String {
        self.0.traj_file_path.clone()
    }

    #[getter]
    fn model(&self) -> PyCustomModel {
        self.0.model.clone().into()
    }

    fn __repr__(&self) -> String {
        format!(
            "CustomTrainedTraj(traj_file_path='{}', model=...)",
            self.0.traj_file_path
        )
    }
}

impl From<PyCustomTrainedTraj> for CustomTrainedTraj {
    fn from(value: PyCustomTrainedTraj) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "GetModeResponse")]
#[derive(Clone)]
pub struct PyGetModeResponse(GetModeResponse);

#[pymethods]
impl PyGetModeResponse {
    #[new]
    fn new(mode: i32) -> Self {
        Self(GetModeResponse { mode })
    }

    #[getter]
    fn mode(&self) -> i32 {
        self.0.mode
    }

    fn mode_enum(&self) -> Option<PyRobotMode> {
        self.0.mode_enum().map(Into::into)
    }

    fn __repr__(&self) -> String {
        format!("GetModeResponse(mode={})", self.0.mode)
    }
}

impl From<GetModeResponse> for PyGetModeResponse {
    fn from(value: GetModeResponse) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "GetStatusResponse")]
#[derive(Clone)]
pub struct PyGetStatusResponse(GetStatusResponse);

#[pymethods]
impl PyGetStatusResponse {
    #[new]
    fn new(current_mode: i32, current_body_control: i32, current_actions: Vec<i32>) -> Self {
        Self(GetStatusResponse {
            current_mode,
            current_body_control,
            current_actions,
        })
    }

    #[getter]
    fn current_mode(&self) -> i32 {
        self.0.current_mode
    }

    #[getter]
    fn current_body_control(&self) -> i32 {
        self.0.current_body_control
    }

    #[getter]
    fn current_actions(&self) -> Vec<i32> {
        self.0.current_actions.clone()
    }

    fn current_mode_enum(&self) -> Option<PyRobotMode> {
        self.0.current_mode_enum().map(Into::into)
    }

    fn current_body_control_enum(&self) -> Option<PyBodyControl> {
        self.0.current_body_control_enum().map(Into::into)
    }

    fn current_actions_enum(&self) -> Vec<PyAction> {
        self.0
            .current_actions_enum()
            .into_iter()
            .map(Into::into)
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "GetStatusResponse(current_mode={}, current_body_control={}, current_actions={:?})",
            self.0.current_mode, self.0.current_body_control, self.0.current_actions
        )
    }
}

impl From<GetStatusResponse> for PyGetStatusResponse {
    fn from(value: GetStatusResponse) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "GetRobotInfoResponse")]
#[derive(Clone)]
pub struct PyGetRobotInfoResponse(GetRobotInfoResponse);

#[pymethods]
impl PyGetRobotInfoResponse {
    #[new]
    fn new(
        name: String,
        nickname: String,
        version: String,
        model: String,
        serial_number: String,
    ) -> Self {
        Self(GetRobotInfoResponse {
            name,
            nickname,
            version,
            model,
            serial_number,
        })
    }

    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    fn nickname(&self) -> String {
        self.0.nickname.clone()
    }

    #[getter]
    fn version(&self) -> String {
        self.0.version.clone()
    }

    #[getter]
    fn model(&self) -> String {
        self.0.model.clone()
    }

    #[getter]
    fn serial_number(&self) -> String {
        self.0.serial_number.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "GetRobotInfoResponse(name='{}', nickname='{}', version='{}', model='{}', serial_number='{}')",
            self.0.name, self.0.nickname, self.0.version, self.0.model, self.0.serial_number
        )
    }
}

impl From<GetRobotInfoResponse> for PyGetRobotInfoResponse {
    fn from(value: GetRobotInfoResponse) -> Self {
        Self(value)
    }
}

#[pyclass(
    module = "booster_sdk_bindings",
    name = "LoadCustomTrainedTrajResponse"
)]
#[derive(Clone)]
pub struct PyLoadCustomTrainedTrajResponse(LoadCustomTrainedTrajResponse);

#[pymethods]
impl PyLoadCustomTrainedTrajResponse {
    #[new]
    fn new(tid: String) -> Self {
        Self(LoadCustomTrainedTrajResponse { tid })
    }

    #[getter]
    fn tid(&self) -> String {
        self.0.tid.clone()
    }

    fn __repr__(&self) -> String {
        format!("LoadCustomTrainedTrajResponse(tid='{}')", self.0.tid)
    }
}

impl From<LoadCustomTrainedTrajResponse> for PyLoadCustomTrainedTrajResponse {
    fn from(value: LoadCustomTrainedTrajResponse) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "BoosterClient", unsendable)]
pub struct PyBoosterClient {
    client: Arc<BoosterClient>,
}

#[pymethods]
impl PyBoosterClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            client: Arc::new(BoosterClient::new().map_err(to_py_err)?),
        })
    }

    fn change_mode(&self, py: Python<'_>, mode: PyRobotMode) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.change_mode(mode.into()).await }).map_err(to_py_err)
    }

    fn get_mode(&self, py: Python<'_>) -> PyResult<PyGetModeResponse> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_mode().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn get_status(&self, py: Python<'_>) -> PyResult<PyGetStatusResponse> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_status().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn get_robot_info(&self, py: Python<'_>) -> PyResult<PyGetRobotInfoResponse> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_robot_info().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn move_robot(&self, py: Python<'_>, vx: f32, vy: f32, vyaw: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.move_robot(vx, vy, vyaw).await }).map_err(to_py_err)
    }

    fn rotate_head(&self, py: Python<'_>, pitch: f32, yaw: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.rotate_head(pitch, yaw).await }).map_err(to_py_err)
    }

    fn wave_hand(&self, py: Python<'_>, action: PyHandAction) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.wave_hand(action.into()).await }).map_err(to_py_err)
    }

    fn rotate_head_with_direction(
        &self,
        py: Python<'_>,
        pitch_direction: i32,
        yaw_direction: i32,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client
                .rotate_head_with_direction(pitch_direction, yaw_direction)
                .await
        })
        .map_err(to_py_err)
    }

    fn lie_down(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.lie_down().await }).map_err(to_py_err)
    }

    fn get_up(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_up().await }).map_err(to_py_err)
    }

    fn get_up_with_mode(&self, py: Python<'_>, mode: PyRobotMode) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.get_up_with_mode(mode.into()).await },
        )
        .map_err(to_py_err)
    }

    fn shoot(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.shoot().await }).map_err(to_py_err)
    }

    fn push_up(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.push_up().await }).map_err(to_py_err)
    }

    fn move_hand_end_effector_with_aux(
        &self,
        py: Python<'_>,
        target_posture: PyPosture,
        aux_posture: PyPosture,
        time_millis: i32,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let target_posture: Posture = target_posture.into();
        let aux_posture: Posture = aux_posture.into();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client
                .move_hand_end_effector_with_aux(
                    &target_posture,
                    &aux_posture,
                    time_millis,
                    hand_index,
                )
                .await
        })
        .map_err(to_py_err)
    }

    fn move_hand_end_effector(
        &self,
        py: Python<'_>,
        target_posture: PyPosture,
        time_millis: i32,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let target_posture: Posture = target_posture.into();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client
                .move_hand_end_effector(&target_posture, time_millis, hand_index)
                .await
        })
        .map_err(to_py_err)
    }

    fn move_hand_end_effector_v2(
        &self,
        py: Python<'_>,
        target_posture: PyPosture,
        time_millis: i32,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let target_posture: Posture = target_posture.into();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client
                .move_hand_end_effector_v2(&target_posture, time_millis, hand_index)
                .await
        })
        .map_err(to_py_err)
    }

    fn stop_hand_end_effector(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_hand_end_effector().await }).map_err(to_py_err)
    }

    fn control_gripper(
        &self,
        py: Python<'_>,
        motion_param: PyGripperMotionParameter,
        mode: PyGripperControlMode,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let motion_param: GripperMotionParameter = motion_param.into();
        let mode: GripperControlMode = mode.into();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client.control_gripper(motion_param, mode, hand_index).await
        })
        .map_err(to_py_err)
    }

    fn get_frame_transform(
        &self,
        py: Python<'_>,
        src: PyFrame,
        dst: PyFrame,
    ) -> PyResult<PyTransform> {
        let client = Arc::clone(&self.client);
        let src: Frame = src.into();
        let dst: Frame = dst.into();
        wait_for_future(
            py,
            async move { client.get_frame_transform(src, dst).await },
        )
        .map(Into::into)
        .map_err(to_py_err)
    }

    fn switch_hand_end_effector_control_mode(
        &self,
        py: Python<'_>,
        switch_on: bool,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client
                .switch_hand_end_effector_control_mode(switch_on)
                .await
        })
        .map_err(to_py_err)
    }

    fn handshake(&self, py: Python<'_>, action: PyHandAction) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.handshake(action.into()).await }).map_err(to_py_err)
    }

    fn control_dexterous_hand(
        &self,
        py: Python<'_>,
        finger_params: Vec<PyDexterousFingerParameter>,
        hand_index: PyHand,
        hand_type: PyBoosterHandType,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let finger_params: Vec<DexterousFingerParameter> =
            finger_params.into_iter().map(Into::into).collect();
        let hand_index: Hand = hand_index.into();
        let hand_type: BoosterHandType = hand_type.into();
        wait_for_future(py, async move {
            client
                .control_dexterous_hand(&finger_params, hand_index, hand_type)
                .await
        })
        .map_err(to_py_err)
    }

    fn control_dexterous_hand_default(
        &self,
        py: Python<'_>,
        finger_params: Vec<PyDexterousFingerParameter>,
        hand_index: PyHand,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let finger_params: Vec<DexterousFingerParameter> =
            finger_params.into_iter().map(Into::into).collect();
        let hand_index: Hand = hand_index.into();
        wait_for_future(py, async move {
            client
                .control_dexterous_hand_default(&finger_params, hand_index)
                .await
        })
        .map_err(to_py_err)
    }

    fn dance(&self, py: Python<'_>, dance_id: PyDanceId) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.dance(dance_id.into()).await }).map_err(to_py_err)
    }

    fn play_sound(&self, py: Python<'_>, sound_file_path: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.play_sound(sound_file_path).await })
            .map_err(to_py_err)
    }

    fn stop_sound(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_sound().await }).map_err(to_py_err)
    }

    fn zero_torque_drag(&self, py: Python<'_>, active: bool) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.zero_torque_drag(active).await }).map_err(to_py_err)
    }

    fn record_trajectory(&self, py: Python<'_>, active: bool) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.record_trajectory(active).await })
            .map_err(to_py_err)
    }

    fn replay_trajectory(&self, py: Python<'_>, traj_file_path: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.replay_trajectory(traj_file_path).await },
        )
        .map_err(to_py_err)
    }

    fn whole_body_dance(&self, py: Python<'_>, dance_id: PyWholeBodyDanceId) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.whole_body_dance(dance_id.into()).await },
        )
        .map_err(to_py_err)
    }

    fn upper_body_custom_control(&self, py: Python<'_>, start: bool) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.upper_body_custom_control(start).await },
        )
        .map_err(to_py_err)
    }

    fn reset_odometry(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.reset_odometry().await }).map_err(to_py_err)
    }

    fn load_custom_trained_traj(
        &self,
        py: Python<'_>,
        traj: PyCustomTrainedTraj,
    ) -> PyResult<PyLoadCustomTrainedTrajResponse> {
        let client = Arc::clone(&self.client);
        let traj: CustomTrainedTraj = traj.into();
        wait_for_future(
            py,
            async move { client.load_custom_trained_traj(&traj).await },
        )
        .map(Into::into)
        .map_err(to_py_err)
    }

    fn activate_custom_trained_traj(&self, py: Python<'_>, tid: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.activate_custom_trained_traj(tid).await },
        )
        .map_err(to_py_err)
    }

    fn unload_custom_trained_traj(&self, py: Python<'_>, tid: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.unload_custom_trained_traj(tid).await },
        )
        .map_err(to_py_err)
    }

    fn enter_wbc_gait(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.enter_wbc_gait().await }).map_err(to_py_err)
    }

    fn exit_wbc_gait(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.exit_wbc_gait().await }).map_err(to_py_err)
    }

    fn publish_gripper_command(&self, command: PyGripperCommand) -> PyResult<()> {
        let command: GripperCommand = command.into();
        self.client
            .publish_gripper_command(&command)
            .map_err(to_py_err)
    }

    fn publish_gripper(
        &self,
        hand: PyHand,
        mode: PyGripperMode,
        motion_param: u16,
        speed: Option<u16>,
    ) -> PyResult<()> {
        let command = GripperCommand {
            hand: hand.into(),
            mode: mode.into(),
            motion_param,
            speed: speed.unwrap_or(500),
        };
        self.client
            .publish_gripper_command(&command)
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRobotMode>()?;
    m.add_class::<PyHand>()?;
    m.add_class::<PyGripperMode>()?;
    m.add_class::<PyHandAction>()?;
    m.add_class::<PyFrame>()?;
    m.add_class::<PyGripperControlMode>()?;
    m.add_class::<PyBoosterHandType>()?;
    m.add_class::<PyDanceId>()?;
    m.add_class::<PyWholeBodyDanceId>()?;
    m.add_class::<PyJointOrder>()?;
    m.add_class::<PyBodyControl>()?;
    m.add_class::<PyAction>()?;
    m.add_class::<PyGripperCommand>()?;
    m.add_class::<PyPosition>()?;
    m.add_class::<PyOrientation>()?;
    m.add_class::<PyPosture>()?;
    m.add_class::<PyQuaternion>()?;
    m.add_class::<PyTransform>()?;
    m.add_class::<PyGripperMotionParameter>()?;
    m.add_class::<PyDexterousFingerParameter>()?;
    m.add_class::<PyCustomModelParams>()?;
    m.add_class::<PyCustomModel>()?;
    m.add_class::<PyCustomTrainedTraj>()?;
    m.add_class::<PyGetModeResponse>()?;
    m.add_class::<PyGetStatusResponse>()?;
    m.add_class::<PyGetRobotInfoResponse>()?;
    m.add_class::<PyLoadCustomTrainedTrajResponse>()?;
    m.add_class::<PyBoosterClient>()?;
    Ok(())
}
