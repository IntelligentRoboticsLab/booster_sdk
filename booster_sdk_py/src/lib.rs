mod runtime;

use crate::runtime::wait_for_future;
use std::sync::Arc;

use ::booster_sdk::{
    client::{BoosterClient, GripperCommand},
    types::{BoosterError, GripperMode, Hand, RobotMode},
};
use pyo3::{exceptions::PyException, prelude::*, types::PyModule};

pyo3::create_exception!(booster_sdk_bindings, BoosterSdkError, PyException);

fn to_py_err(err: BoosterError) -> PyErr {
    BoosterSdkError::new_err(err.to_string())
}

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
    #[classattr]
    const SOCCER: Self = Self(RobotMode::Soccer);

    fn __repr__(&self) -> String {
        match self.0 {
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

    fn wait_for_discovery(&self, py: Python<'_>, timeout_secs: f64) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let timeout = std::time::Duration::from_secs_f64(timeout_secs);
        wait_for_future(py, async move { client.wait_for_discovery(timeout).await })
            .map_err(to_py_err)
    }

    fn change_mode(&self, py: Python<'_>, mode: PyRobotMode) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.change_mode(mode.into()).await }).map_err(to_py_err)
    }

    fn move_robot(&self, py: Python<'_>, vx: f32, vy: f32, vyaw: f32) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.move_robot(vx, vy, vyaw).await }).map_err(to_py_err)
    }

    fn publish_gripper_command(&self, command: PyGripperCommand) -> PyResult<()> {
        self.client
            .publish_gripper_command(&command.0)
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

#[pymodule]
fn booster_sdk_bindings(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("BoosterSdkError", py.get_type::<BoosterSdkError>())?;
    m.add_class::<PyBoosterClient>()?;
    m.add_class::<PyRobotMode>()?;
    m.add_class::<PyHand>()?;
    m.add_class::<PyGripperMode>()?;
    m.add_class::<PyGripperCommand>()?;
    Ok(())
}
