//! `MoveController` / `ArmController` (C++ `b1::MoveController` /
//! `b1::ArmController`).
//!
//! The motion loops run with the GIL released and check for Python signals
//! every 100 ms: Ctrl-C cancels the running call (a `MoveController` then
//! sends a zero-velocity command) and raises `KeyboardInterrupt`.

use std::sync::Arc;

use booster_sdk::{
    client::{
        controllers::{ArmController, ArmJoint, MoveController},
        loco::BoosterClient,
    },
    types::JOINT_CNT,
};
use pyo3::{
    Bound,
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
    types::{PyModule, PyTuple},
};

use super::booster::PyBoosterClient;
use crate::{
    macros::py_int_enum,
    options::rpc_options,
    runtime::{get_tokio_runtime, wait_for_future_interruptible},
    to_py_err,
};

py_int_enum!(PyArmJoint, "ArmJoint", ArmJoint, i32, {
    LEFT_PITCH => LeftPitch,
    LEFT_ROLL => LeftRoll,
    LEFT_YAW => LeftYaw,
    LEFT_ELBOW => LeftElbow,
    RIGHT_PITCH => RightPitch,
    RIGHT_ROLL => RightRoll,
    RIGHT_YAW => RightYaw,
    RIGHT_ELBOW => RightElbow,
});

/// Use `client` if given, else create one from the keyword options.
fn resolve_client(
    client: Option<PyRef<'_, PyBoosterClient>>,
    robot_name: Option<String>,
    network: Option<&Bound<'_, PyAny>>,
    domain_id: Option<u16>,
) -> PyResult<Arc<BoosterClient>> {
    if let Some(client) = client {
        if robot_name.is_some() || network.is_some() || domain_id.is_some() {
            return Err(PyValueError::new_err(
                "pass either client or robot_name/network/domain_id, not both",
            ));
        }
        return Ok(Arc::clone(&client.client));
    }
    let options = rpc_options(None, robot_name, network, domain_id)?;
    BoosterClient::with_options(options)
        .map(Arc::new)
        .map_err(to_py_err)
}

fn closed_error(name: &str) -> PyErr {
    PyRuntimeError::new_err(format!("{name} is closed"))
}

// ---------------------------------------------------------------------------
// MoveController
// ---------------------------------------------------------------------------

/// Odometry-based point-to-point locomotion (C++ `b1::MoveController`).
#[pyclass(module = "booster_sdk_bindings", name = "MoveController")]
pub struct PyMoveController {
    inner: Option<MoveController>,
}

impl PyMoveController {
    fn controller(&self) -> PyResult<&MoveController> {
        self.inner
            .as_ref()
            .ok_or_else(|| closed_error("MoveController"))
    }
}

/// Run a motion future; on Ctrl-C (or another signal error) send a stop.
fn run_motion(
    py: Python<'_>,
    controller: &MoveController,
    motion: impl std::future::Future<Output = ()> + Send,
) -> PyResult<()> {
    let result = wait_for_future_interruptible(py, motion);
    if result.is_err() {
        py.detach(|| get_tokio_runtime().block_on(controller.stop()));
    }
    result
}

#[pymethods]
impl PyMoveController {
    /// Create a controller (blocks for ~2-7 s like the C++ constructor).
    #[new]
    #[pyo3(signature = (*, client=None, robot_name=None, network=None, domain_id=None))]
    fn new(
        py: Python<'_>,
        client: Option<PyRef<'_, PyBoosterClient>>,
        robot_name: Option<String>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let client = resolve_client(client, robot_name, network, domain_id)?;
        let controller = wait_for_future_interruptible(py, MoveController::with_client(client))?
            .map_err(to_py_err)?;
        Ok(Self {
            inner: Some(controller),
        })
    }

    /// The underlying locomotion client.
    #[getter]
    fn client(&self) -> PyResult<PyBoosterClient> {
        Ok(PyBoosterClient {
            client: Arc::clone(self.controller()?.client()),
        })
    }

    /// Latest odometry pose `(x, y, theta)`.
    fn pose(&self) -> PyResult<(f32, f32, f32)> {
        Ok(self.controller()?.pose())
    }

    /// Walk to the odometry-frame target `(x, y)` with speed limit `vel` (m/s).
    fn move_to_target(&self, py: Python<'_>, x: f32, y: f32, vel: f32) -> PyResult<()> {
        let c = self.controller()?;
        run_motion(py, c, c.move_to_target(x, y, vel))
    }

    /// Walk `(dx, dy)` relative to the current odometry position.
    fn move_to_relative(&self, py: Python<'_>, dx: f32, dy: f32, vel: f32) -> PyResult<()> {
        let c = self.controller()?;
        run_motion(py, c, c.move_to_relative(dx, dy, vel))
    }

    /// Turn in place by `|angle|` rad at `|vel|` rad/s (a positive angle turns
    /// with negative yaw rate, as in the C++ SDK).
    fn turn_around(&self, py: Python<'_>, angle: f32, vel: f32) -> PyResult<()> {
        let c = self.controller()?;
        run_motion(py, c, c.turn_around(angle, vel))
    }

    /// Command zero velocity.
    fn stop(&self, py: Python<'_>) -> PyResult<()> {
        let controller = self.controller()?;
        py.detach(|| get_tokio_runtime().block_on(controller.stop()));
        Ok(())
    }

    /// Stop the robot and close the odometry subscription (idempotent).
    fn close(&mut self, py: Python<'_>) {
        if let Some(mut controller) = self.inner.take() {
            py.detach(|| get_tokio_runtime().block_on(controller.close()));
        }
    }

    /// Whether `close()` has been called.
    #[getter]
    fn closed(&self) -> bool {
        self.inner.is_none()
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (*_args))]
    fn __exit__(&mut self, py: Python<'_>, _args: &Bound<'_, PyTuple>) {
        self.close(py);
    }
}

impl Drop for PyMoveController {
    fn drop(&mut self) {
        if let Some(controller) = self.inner.take() {
            // The core `Drop` sends a best-effort stop on the current runtime.
            let _guard = get_tokio_runtime().enter();
            drop(controller);
        }
    }
}

// ---------------------------------------------------------------------------
// ArmController
// ---------------------------------------------------------------------------

/// Interpolated arm-joint control via low-level commands (C++ `b1::ArmController`).
#[pyclass(module = "booster_sdk_bindings", name = "ArmController")]
pub struct PyArmController {
    inner: Option<ArmController>,
}

impl PyArmController {
    fn controller(&self) -> PyResult<&ArmController> {
        self.inner
            .as_ref()
            .ok_or_else(|| closed_error("ArmController"))
    }

    fn controller_mut(&mut self) -> PyResult<&mut ArmController> {
        self.inner
            .as_mut()
            .ok_or_else(|| closed_error("ArmController"))
    }
}

#[pymethods]
impl PyArmController {
    /// Create a controller and enable upper-body custom control (blocks
    /// ~2.5-7.5 s like the C++ constructor).
    #[new]
    #[pyo3(signature = (*, joint_count=JOINT_CNT, client=None, robot_name=None, network=None, domain_id=None))]
    fn new(
        py: Python<'_>,
        joint_count: usize,
        client: Option<PyRef<'_, PyBoosterClient>>,
        robot_name: Option<String>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let client = resolve_client(client, robot_name, network, domain_id)?;
        let controller =
            wait_for_future_interruptible(py, ArmController::with_client(client, joint_count))?
                .map_err(to_py_err)?;
        Ok(Self {
            inner: Some(controller),
        })
    }

    /// The underlying locomotion client.
    #[getter]
    fn client(&self) -> PyResult<PyBoosterClient> {
        Ok(PyBoosterClient {
            client: Arc::clone(self.controller()?.client()),
        })
    }

    /// Latest measured joint positions (parallel layout).
    fn joint_positions(&self) -> PyResult<Vec<f32>> {
        Ok(self.controller()?.joint_positions())
    }

    /// Queue a move of `joint` (an `ArmJoint` or index 2-9) to `target_rad`
    /// over `duration_ms` (min 50 ms). Returns `self` for chaining; nothing
    /// moves until `finish()`.
    fn control_arm(
        mut slf: PyRefMut<'_, Self>,
        joint: i32,
        target_rad: f32,
        duration_ms: f32,
    ) -> PyResult<PyRefMut<'_, Self>> {
        if ArmJoint::try_from(joint).is_err() {
            return Err(PyValueError::new_err(format!(
                "invalid arm joint index {joint} (expected 2-9)"
            )));
        }
        slf.controller_mut()?
            .control_arm_index(joint, target_rad, duration_ms);
        Ok(slf)
    }

    /// Execute the queued moves (100 Hz linear interpolation). Ctrl-C stops
    /// publishing and raises `KeyboardInterrupt`; the queue is kept.
    fn finish(&mut self, py: Python<'_>) -> PyResult<()> {
        let controller = self.controller_mut()?;
        wait_for_future_interruptible(py, controller.finish())?.map_err(to_py_err)
    }

    /// Disable upper-body custom control and close the channels (idempotent).
    fn close(&mut self, py: Python<'_>) {
        if let Some(mut controller) = self.inner.take() {
            py.detach(|| get_tokio_runtime().block_on(controller.close()));
        }
    }

    /// Whether `close()` has been called.
    #[getter]
    fn closed(&self) -> bool {
        self.inner.is_none()
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (*_args))]
    fn __exit__(&mut self, py: Python<'_>, _args: &Bound<'_, PyTuple>) {
        self.close(py);
    }
}

impl Drop for PyArmController {
    fn drop(&mut self) {
        if let Some(controller) = self.inner.take() {
            // The core `Drop` disables upper-body custom control on the
            // current runtime (best effort).
            let _guard = get_tokio_runtime().enter();
            drop(controller);
        }
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyArmJoint>()?;
    m.add_class::<PyMoveController>()?;
    m.add_class::<PyArmController>()?;
    Ok(())
}
