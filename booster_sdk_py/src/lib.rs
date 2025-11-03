mod runtime;

use crate::runtime::wait_for_future;
use std::{future::Future, sync::Arc, time::Duration};

use ::booster_sdk::{
    client::{
        B1LocoClient, DexterousHandCommand, FingerControl, GripperCommand, HandPoseCommand,
        HandPoseWithAuxCommand, HandTransformCommand, MoveCommand,
    },
    types::{
        BoosterError, DanceId, Direction, Frame, GripperMode, Hand, Position, Posture, Quaternion,
        RobotMode, Transform,
    },
};
use pyo3::{
    Bound,
    exceptions::{PyException, PyValueError},
    prelude::*,
    types::{PyAny, PyModule, PyType},
};

pyo3::create_exception!(booster_sdk_bindings, BoosterSdkError, PyException);

type Any<'py> = Bound<'py, PyAny>;

fn to_py_err(err: BoosterError) -> PyErr {
    BoosterSdkError::new_err(err.to_string())
}

// Python wrapper types for enums

#[pyclass(module = "booster_sdk_bindings", name = "RobotMode", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyRobotMode(RobotMode);

#[pymethods]
impl PyRobotMode {
    #[classattr]
    const DAMPING: Self = Self(RobotMode::Damping);
    #[classattr]
    const PREPARE: Self = Self(RobotMode::Prepare);
    #[classattr]
    const WALKING: Self = Self(RobotMode::Walking);
    #[classattr]
    const CUSTOM: Self = Self(RobotMode::Custom);

    fn __repr__(&self) -> String {
        match self.0 {
            RobotMode::Damping => "RobotMode.DAMPING".to_string(),
            RobotMode::Prepare => "RobotMode.PREPARE".to_string(),
            RobotMode::Walking => "RobotMode.WALKING".to_string(),
            RobotMode::Custom => "RobotMode.CUSTOM".to_string(),
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

#[pyclass(module = "booster_sdk_bindings", name = "Direction", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyDirection(Direction);

#[pymethods]
impl PyDirection {
    #[classattr]
    const POSITIVE: Self = Self(Direction::Positive);
    #[classattr]
    const STOP: Self = Self(Direction::Stop);
    #[classattr]
    const NEGATIVE: Self = Self(Direction::Negative);

    fn __repr__(&self) -> String {
        match self.0 {
            Direction::Positive => "Direction.POSITIVE".to_string(),
            Direction::Stop => "Direction.STOP".to_string(),
            Direction::Negative => "Direction.NEGATIVE".to_string(),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyDirection> for Direction {
    fn from(py_dir: PyDirection) -> Self {
        py_dir.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Frame", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyFrame(Frame);

#[pymethods]
impl PyFrame {
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
        format!("Frame.{}", self.0.as_str().to_uppercase())
    }

    fn __str__(&self) -> &'static str {
        self.0.as_str()
    }
}

impl From<PyFrame> for Frame {
    fn from(py_frame: PyFrame) -> Self {
        py_frame.0
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

    fn __repr__(&self) -> String {
        match self.0 {
            DanceId::NewYear => "DanceId.NEW_YEAR".to_string(),
            DanceId::Nezha => "DanceId.NEZHA".to_string(),
            DanceId::TowardsFuture => "DanceId.TOWARDS_FUTURE".to_string(),
            _ => format!("DanceId({})", i32::from(self.0)),
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyDanceId> for DanceId {
    fn from(py_id: PyDanceId) -> Self {
        py_id.0
    }
}

// Python wrapper types for data structures

#[pyclass(module = "booster_sdk_bindings", name = "Position")]
#[derive(Clone, Copy)]
pub struct PyPosition(Position);

#[pymethods]
impl PyPosition {
    #[new]
    #[pyo3(signature = (x, y, z))]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Position::new(x, y, z))
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

    fn __repr__(&self) -> String {
        format!("Position(x={}, y={}, z={})", self.0.x, self.0.y, self.0.z)
    }

    fn __str__(&self) -> String {
        format!("({}, {}, {})", self.0.x, self.0.y, self.0.z)
    }

    fn __getitem__(&self, idx: usize) -> PyResult<f32> {
        match idx {
            0 => Ok(self.0.x),
            1 => Ok(self.0.y),
            2 => Ok(self.0.z),
            _ => Err(PyValueError::new_err("index out of range, expected 0-2")),
        }
    }
}

impl From<PyPosition> for Position {
    fn from(py_pos: PyPosition) -> Self {
        py_pos.0
    }
}

impl From<Position> for PyPosition {
    fn from(pos: Position) -> Self {
        Self(pos)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Quaternion")]
#[derive(Clone, Copy)]
pub struct PyQuaternion(Quaternion);

#[pymethods]
impl PyQuaternion {
    #[new]
    #[pyo3(signature = (x, y, z, w))]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Quaternion::from_xyzw(x, y, z, w))
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

    fn __str__(&self) -> String {
        format!("({}, {}, {}, {})", self.0.x, self.0.y, self.0.z, self.0.w)
    }

    fn __getitem__(&self, idx: usize) -> PyResult<f32> {
        match idx {
            0 => Ok(self.0.x),
            1 => Ok(self.0.y),
            2 => Ok(self.0.z),
            3 => Ok(self.0.w),
            _ => Err(PyValueError::new_err("index out of range, expected 0-3")),
        }
    }
}

impl From<PyQuaternion> for Quaternion {
    fn from(py_quat: PyQuaternion) -> Self {
        py_quat.0
    }
}

impl From<Quaternion> for PyQuaternion {
    fn from(quat: Quaternion) -> Self {
        Self(quat)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Transform")]
#[derive(Clone, Copy)]
pub struct PyTransform(Transform);

#[pymethods]
impl PyTransform {
    #[new]
    #[pyo3(signature = (position, rotation))]
    fn new(position: PyPosition, rotation: PyQuaternion) -> Self {
        Self(Transform::new(position.into(), rotation.into()))
    }

    #[getter]
    fn position(&self) -> PyPosition {
        PyPosition(self.0.position)
    }

    #[getter]
    fn rotation(&self) -> PyQuaternion {
        PyQuaternion(self.0.rotation)
    }

    fn __repr__(&self) -> String {
        format!(
            "Transform(position=Position(x={}, y={}, z={}), rotation=Quaternion(x={}, y={}, z={}, w={}))",
            self.0.position.x,
            self.0.position.y,
            self.0.position.z,
            self.0.rotation.x,
            self.0.rotation.y,
            self.0.rotation.z,
            self.0.rotation.w
        )
    }
}

impl From<PyTransform> for Transform {
    fn from(py_transform: PyTransform) -> Self {
        py_transform.0
    }
}

impl From<Transform> for PyTransform {
    fn from(transform: Transform) -> Self {
        Self(transform)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "Posture")]
#[derive(Clone, Copy)]
pub struct PyPosture(Posture);

#[pymethods]
impl PyPosture {
    #[new]
    #[pyo3(signature = (position, orientation))]
    fn new(position: PyPosition, orientation: PyPosition) -> Self {
        Self(Posture::new(position.into(), orientation.into()))
    }

    #[getter]
    fn position(&self) -> PyPosition {
        PyPosition(self.0.position)
    }

    #[getter]
    fn orientation(&self) -> PyPosition {
        PyPosition(self.0.orientation)
    }

    fn __repr__(&self) -> String {
        format!(
            "Posture(position=Position(x={}, y={}, z={}), orientation=Position(x={}, y={}, z={}))",
            self.0.position.x,
            self.0.position.y,
            self.0.position.z,
            self.0.orientation.x,
            self.0.orientation.y,
            self.0.orientation.z
        )
    }
}

impl From<PyPosture> for Posture {
    fn from(py_posture: PyPosture) -> Self {
        py_posture.0
    }
}

impl From<Posture> for PyPosture {
    fn from(posture: Posture) -> Self {
        Self(posture)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "FingerControl")]
#[derive(Clone, Copy)]
pub struct PyFingerControl(FingerControl);

#[pymethods]
impl PyFingerControl {
    #[new]
    #[pyo3(signature = (angle, force, speed))]
    fn new(angle: f32, force: u16, speed: u16) -> Self {
        Self(FingerControl::new(angle, force, speed))
    }

    #[getter]
    fn angle(&self) -> f32 {
        self.0.angle
    }

    #[getter]
    fn force(&self) -> u16 {
        self.0.force
    }

    #[getter]
    fn speed(&self) -> u16 {
        self.0.speed
    }

    fn __repr__(&self) -> String {
        format!(
            "FingerControl(angle={}, force={}, speed={})",
            self.0.angle, self.0.force, self.0.speed
        )
    }
}

impl From<PyFingerControl> for FingerControl {
    fn from(py_fc: PyFingerControl) -> Self {
        py_fc.0
    }
}

impl From<FingerControl> for PyFingerControl {
    fn from(fc: FingerControl) -> Self {
        Self(fc)
    }
}

fn as_lowercase(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[pyclass(module = "booster_sdk_bindings", name = "B1LocoClient", unsendable)]
pub struct PyB1LocoClient {
    inner: Arc<B1LocoClient>,
}

impl PyB1LocoClient {
    fn block_on<F, T>(&self, py: Python<'_>, fut: F) -> PyResult<T>
    where
        F: Future<Output = ::booster_sdk::types::Result<T>> + Send + 'static,
        T: Send + 'static,
    {
        wait_for_future(py, fut).map_err(to_py_err)
    }
}

#[pymethods]
impl PyB1LocoClient {
    #[new]
    fn new(py: Python<'_>) -> PyResult<Self> {
        let client = wait_for_future(py, B1LocoClient::new()).map_err(to_py_err)?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    #[classmethod]
    fn with_timeout(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        timeout_seconds: f64,
    ) -> PyResult<Self> {
        if timeout_seconds <= 0.0 {
            return Err(PyValueError::new_err("timeout must be positive"));
        }

        let duration = Duration::from_secs_f64(timeout_seconds);
        let client =
            wait_for_future(py, B1LocoClient::with_timeout(duration)).map_err(to_py_err)?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    fn change_mode(&self, py: Python<'_>, mode: PyRobotMode) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.change_mode(mode.into()).await })?;
        Ok(())
    }

    fn get_mode(&self, py: Python<'_>) -> PyResult<i32> {
        let client = Arc::clone(&self.inner);
        let mode = self.block_on(py, async move { client.get_mode().await })?;
        Ok(i32::from(mode))
    }

    #[pyo3(signature = (vx, vy, vyaw))]
    fn move_robot(&self, py: Python<'_>, vx: f32, vy: f32, vyaw: f32) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.move_robot(vx, vy, vyaw).await })?;
        Ok(())
    }

    #[pyo3(signature = (vx=0.0, vy=0.0, vyaw=0.0))]
    fn move_with_command(&self, py: Python<'_>, vx: f32, vy: f32, vyaw: f32) -> PyResult<()> {
        let command = MoveCommand { vx, vy, vyaw };
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.move_with_command(&command).await })?;
        Ok(())
    }

    fn lie_down(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.lie_down().await })?;
        Ok(())
    }

    fn get_up(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.get_up().await })?;
        Ok(())
    }

    #[pyo3(signature = (pitch, yaw))]
    fn rotate_head(&self, py: Python<'_>, pitch: f32, yaw: f32) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.rotate_head(pitch, yaw).await })?;
        Ok(())
    }

    fn rotate_head_continuous(
        &self,
        py: Python<'_>,
        pitch_direction: PyDirection,
        yaw_direction: PyDirection,
        speed: f32,
    ) -> PyResult<()> {
        if !(0.0..=1.0).contains(&speed) {
            return Err(PyValueError::new_err("speed must be between 0.0 and 1.0"));
        }

        let client = Arc::clone(&self.inner);
        self.block_on(py, async move {
            client
                .rotate_head_continuous(pitch_direction.into(), yaw_direction.into(), speed)
                .await
        })?;
        Ok(())
    }

    #[pyo3(signature = (hand, position, orientation, duration = 1.0))]
    fn move_hand(
        &self,
        py: Python<'_>,
        hand: PyHand,
        position: PyPosition,
        orientation: PyPosition,
        duration: f32,
    ) -> PyResult<()> {
        if duration <= 0.0 {
            return Err(PyValueError::new_err("duration must be positive"));
        }

        let pose = Posture::new(position.into(), orientation.into());
        let command = HandPoseCommand::builder()
            .hand(hand.into())
            .pose(pose)
            .duration(duration)
            .build();
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.move_hand(&command).await })?;
        Ok(())
    }

    #[pyo3(signature = (hand, position, orientation, aux_position, aux_orientation, duration = 1.0))]
    #[allow(clippy::too_many_arguments)]
    fn move_hand_with_aux(
        &self,
        py: Python<'_>,
        hand: PyHand,
        position: PyPosition,
        orientation: PyPosition,
        aux_position: PyPosition,
        aux_orientation: PyPosition,
        duration: f32,
    ) -> PyResult<()> {
        if duration <= 0.0 {
            return Err(PyValueError::new_err("duration must be positive"));
        }

        let pose = Posture::new(position.into(), orientation.into());
        let aux_pose = Posture::new(aux_position.into(), aux_orientation.into());
        let command = HandPoseWithAuxCommand::builder()
            .hand(hand.into())
            .pose(pose)
            .aux_pose(aux_pose)
            .duration(duration)
            .build();
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.move_hand_with_aux(&command).await })?;
        Ok(())
    }

    #[pyo3(signature = (hand, transform, duration = 1.0))]
    fn move_hand_transform(
        &self,
        py: Python<'_>,
        hand: PyHand,
        transform: PyTransform,
        duration: f32,
    ) -> PyResult<()> {
        if duration <= 0.0 {
            return Err(PyValueError::new_err("duration must be positive"));
        }

        let command = HandTransformCommand::builder()
            .hand(hand.into())
            .transform(transform.into())
            .duration(duration)
            .build();
        let client = Arc::clone(&self.inner);
        self.block_on(
            py,
            async move { client.move_hand_transform(&command).await },
        )?;
        Ok(())
    }

    fn wave_hand(&self, py: Python<'_>, hand: PyHand) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.wave_hand(hand.into()).await })?;
        Ok(())
    }

    fn handshake(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.handshake().await })?;
        Ok(())
    }

    #[pyo3(signature = (hand, mode, motion_param, speed = 500))]
    fn control_gripper(
        &self,
        py: Python<'_>,
        hand: PyHand,
        mode: PyGripperMode,
        motion_param: u16,
        speed: u16,
    ) -> PyResult<()> {
        if !(1..=1000).contains(&speed) {
            return Err(PyValueError::new_err("speed must be between 1 and 1000"));
        }

        let command = GripperCommand::builder()
            .hand(hand.into())
            .mode(mode.into())
            .motion_param(motion_param)
            .speed(speed)
            .build();
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.control_gripper(&command).await })?;
        Ok(())
    }

    #[pyo3(signature = (hand, *, preset=None, thumb_rotation=None, thumb=None, index=None, middle=None, ring=None, pinky=None))]
    #[allow(clippy::too_many_arguments)]
    fn control_dexterous_hand(
        &self,
        py: Python<'_>,
        hand: PyHand,
        preset: Option<&Any<'_>>,
        thumb_rotation: Option<PyFingerControl>,
        thumb: Option<PyFingerControl>,
        index: Option<PyFingerControl>,
        middle: Option<PyFingerControl>,
        ring: Option<PyFingerControl>,
        pinky: Option<PyFingerControl>,
    ) -> PyResult<()> {
        let command = if let Some(preset) = preset {
            match as_lowercase(preset.extract::<&str>()?).as_str() {
                "open_all" | "open" => DexterousHandCommand::open_all(hand.into()),
                "close_all" | "close" => DexterousHandCommand::close_all(hand.into()),
                "pinch" => DexterousHandCommand::pinch(hand.into()),
                other => {
                    return Err(PyValueError::new_err(format!(
                        "unknown preset '{other}'; choose from 'open_all', 'close_all', or 'pinch'"
                    )));
                }
            }
        } else {
            let thumb_rotation = thumb_rotation.ok_or_else(|| {
                PyValueError::new_err("thumb_rotation must be provided when preset is not used")
            })?;
            let thumb = thumb.ok_or_else(|| {
                PyValueError::new_err("thumb must be provided when preset is not used")
            })?;
            let index = index.ok_or_else(|| {
                PyValueError::new_err("index must be provided when preset is not used")
            })?;
            let middle = middle.ok_or_else(|| {
                PyValueError::new_err("middle must be provided when preset is not used")
            })?;
            let ring = ring.ok_or_else(|| {
                PyValueError::new_err("ring must be provided when preset is not used")
            })?;
            let pinky = pinky.ok_or_else(|| {
                PyValueError::new_err("pinky must be provided when preset is not used")
            })?;

            DexterousHandCommand::builder()
                .hand(hand.into())
                .thumb_rotation(thumb_rotation.into())
                .thumb(thumb.into())
                .index(index.into())
                .middle(middle.into())
                .ring(ring.into())
                .pinky(pinky.into())
                .build()
        };

        let client = Arc::clone(&self.inner);
        self.block_on(
            py,
            async move { client.control_dexterous_hand(&command).await },
        )?;
        Ok(())
    }

    fn get_frame_transform(
        &self,
        py: Python<'_>,
        source: PyFrame,
        destination: PyFrame,
    ) -> PyResult<PyTransform> {
        let client = Arc::clone(&self.inner);
        let transform = self.block_on(py, async move {
            client
                .get_frame_transform(source.into(), destination.into())
                .await
        })?;
        Ok(PyTransform::from(transform))
    }

    fn dance(&self, py: Python<'_>, dance_id: PyDanceId) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.dance(dance_id.into()).await })?;
        Ok(())
    }

    fn stop(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        self.block_on(py, async move { client.stop().await })?;
        Ok(())
    }
}

#[pymodule]
fn booster_sdk_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyB1LocoClient>()?;
    m.add_class::<PyRobotMode>()?;
    m.add_class::<PyHand>()?;
    m.add_class::<PyDirection>()?;
    m.add_class::<PyFrame>()?;
    m.add_class::<PyGripperMode>()?;
    m.add_class::<PyDanceId>()?;
    m.add_class::<PyPosition>()?;
    m.add_class::<PyQuaternion>()?;
    m.add_class::<PyTransform>()?;
    m.add_class::<PyPosture>()?;
    m.add_class::<PyFingerControl>()?;
    m.add("BoosterSdkError", m.py().get_type::<BoosterSdkError>())?;
    Ok(())
}
