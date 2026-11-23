//! Low-level joint control (`rt/joint_ctrl`, `rt/low_state`), kick commands
//! (`rt/kick_ball`), the transform tree (`rt/tf`) and the ROS 2 message
//! types they use.
//!
//! ROS geometry types are bound as `RosQuaternion` / `RosTransform` because
//! `Quaternion` / `Transform` are the (f32) RPC entity classes used by
//! `BoosterClient.get_frame_transform`. `Vector3` is shared with the camera
//! extrinsics.

use booster_sdk::{
    dds::{
        DdsPublisher, Header, ImuState, Kick, LowCmd, LowState, MotorCmd, MotorState, Quaternion,
        TFMessage, Time, Transform, TransformStamped,
    },
    types::{
        CmdType, JOINT_CNT, JOINT_CNT_7DOF_ARM, JOINT_CNT_K1, JointIndex, JointIndexK1,
        JointIndexWith7DofArm,
    },
};
use pyo3::{
    Bound,
    exceptions::PyTypeError,
    prelude::*,
    types::{PyList, PyModule},
};

use super::device::PyVector3;
use crate::{
    macros::{py_int_enum, py_param_class, py_readonly_class},
    to_py_err,
};

// ---------------------------------------------------------------------------
// Joint layouts
// ---------------------------------------------------------------------------

py_int_enum!(PyJointIndex, "JointIndex", JointIndex, i32, {
    HEAD_YAW => HeadYaw,
    HEAD_PITCH => HeadPitch,
    LEFT_SHOULDER_PITCH => LeftShoulderPitch,
    LEFT_SHOULDER_ROLL => LeftShoulderRoll,
    LEFT_ELBOW_PITCH => LeftElbowPitch,
    LEFT_ELBOW_YAW => LeftElbowYaw,
    RIGHT_SHOULDER_PITCH => RightShoulderPitch,
    RIGHT_SHOULDER_ROLL => RightShoulderRoll,
    RIGHT_ELBOW_PITCH => RightElbowPitch,
    RIGHT_ELBOW_YAW => RightElbowYaw,
    WAIST => Waist,
    LEFT_HIP_PITCH => LeftHipPitch,
    LEFT_HIP_ROLL => LeftHipRoll,
    LEFT_HIP_YAW => LeftHipYaw,
    LEFT_KNEE_PITCH => LeftKneePitch,
    CRANK_UP_LEFT => CrankUpLeft,
    CRANK_DOWN_LEFT => CrankDownLeft,
    RIGHT_HIP_PITCH => RightHipPitch,
    RIGHT_HIP_ROLL => RightHipRoll,
    RIGHT_HIP_YAW => RightHipYaw,
    RIGHT_KNEE_PITCH => RightKneePitch,
    CRANK_UP_RIGHT => CrankUpRight,
    CRANK_DOWN_RIGHT => CrankDownRight,
});

py_int_enum!(PyJointIndexK1, "JointIndexK1", JointIndexK1, i32, {
    HEAD_YAW => HeadYaw,
    HEAD_PITCH => HeadPitch,
    LEFT_SHOULDER_PITCH => LeftShoulderPitch,
    LEFT_SHOULDER_ROLL => LeftShoulderRoll,
    LEFT_ELBOW_PITCH => LeftElbowPitch,
    LEFT_ELBOW_YAW => LeftElbowYaw,
    RIGHT_SHOULDER_PITCH => RightShoulderPitch,
    RIGHT_SHOULDER_ROLL => RightShoulderRoll,
    RIGHT_ELBOW_PITCH => RightElbowPitch,
    RIGHT_ELBOW_YAW => RightElbowYaw,
    LEFT_HIP_PITCH => LeftHipPitch,
    LEFT_HIP_ROLL => LeftHipRoll,
    LEFT_HIP_YAW => LeftHipYaw,
    LEFT_KNEE_PITCH => LeftKneePitch,
    CRANK_UP_LEFT => CrankUpLeft,
    CRANK_DOWN_LEFT => CrankDownLeft,
    RIGHT_HIP_PITCH => RightHipPitch,
    RIGHT_HIP_ROLL => RightHipRoll,
    RIGHT_HIP_YAW => RightHipYaw,
    RIGHT_KNEE_PITCH => RightKneePitch,
    CRANK_UP_RIGHT => CrankUpRight,
    CRANK_DOWN_RIGHT => CrankDownRight,
});

py_int_enum!(PyJointIndexWith7DofArm, "JointIndexWith7DofArm", JointIndexWith7DofArm, i32, {
    HEAD_YAW => HeadYaw,
    HEAD_PITCH => HeadPitch,
    LEFT_SHOULDER_PITCH => LeftShoulderPitch,
    LEFT_SHOULDER_ROLL => LeftShoulderRoll,
    LEFT_ELBOW_PITCH => LeftElbowPitch,
    LEFT_ELBOW_YAW => LeftElbowYaw,
    LEFT_WRIST_PITCH => LeftWristPitch,
    LEFT_WRIST_YAW => LeftWristYaw,
    LEFT_HAND_ROLL => LeftHandRoll,
    RIGHT_SHOULDER_PITCH => RightShoulderPitch,
    RIGHT_SHOULDER_ROLL => RightShoulderRoll,
    RIGHT_ELBOW_PITCH => RightElbowPitch,
    RIGHT_ELBOW_YAW => RightElbowYaw,
    RIGHT_WRIST_PITCH => RightWristPitch,
    RIGHT_WRIST_YAW => RightWristYaw,
    RIGHT_HAND_ROLL => RightHandRoll,
    WAIST => Waist,
    LEFT_HIP_PITCH => LeftHipPitch,
    LEFT_HIP_ROLL => LeftHipRoll,
    LEFT_HIP_YAW => LeftHipYaw,
    LEFT_KNEE_PITCH => LeftKneePitch,
    CRANK_UP_LEFT => CrankUpLeft,
    CRANK_DOWN_LEFT => CrankDownLeft,
    RIGHT_HIP_PITCH => RightHipPitch,
    RIGHT_HIP_ROLL => RightHipRoll,
    RIGHT_HIP_YAW => RightHipYaw,
    RIGHT_KNEE_PITCH => RightKneePitch,
    CRANK_UP_RIGHT => CrankUpRight,
    CRANK_DOWN_RIGHT => CrankDownRight,
});

py_int_enum!(PyCmdType, "CmdType", CmdType, i32, {
    PARALLEL => Parallel,
    SERIAL => Serial,
});

// ---------------------------------------------------------------------------
// rt/joint_ctrl
// ---------------------------------------------------------------------------

py_param_class!(PyMotorCmd, "MotorCmd", MotorCmd, {
    mode: u8 = 0,
    q: f32 = 0.0,
    dq: f32 = 0.0,
    tau: f32 = 0.0,
    kp: f32 = 0.0,
    kd: f32 = 0.0,
    weight: f32 = 0.0,
});

/// Collect an iterable of `MotorCmd` into a fresh list (keeping the objects,
/// so later mutation through the list is visible).
fn motor_cmd_list<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>> {
    let list = PyList::empty(value.py());
    for item in value.try_iter()? {
        let item = item?;
        if !item.is_instance_of::<PyMotorCmd>() {
            return Err(PyTypeError::new_err(format!(
                "motor_cmd entries must be MotorCmd, got {}",
                item.get_type().name()?
            )));
        }
        list.append(item)?;
    }
    Ok(list)
}

/// Low-level joint command (`rt/joint_ctrl`).
///
/// `motor_cmd` is a regular list of `MotorCmd` objects, so
/// `cmd.motor_cmd[i].q = 0.3` updates the command in place.
#[pyclass(module = "booster_sdk_bindings", name = "LowCmd")]
pub struct PyLowCmd {
    #[pyo3(get, set)]
    cmd_type: PyCmdType,
    motor_cmd: Py<PyList>,
}

impl PyLowCmd {
    pub(crate) fn to_rust(&self, py: Python<'_>) -> PyResult<LowCmd> {
        let motor_cmd = self
            .motor_cmd
            .bind(py)
            .iter()
            .map(|item| Ok(item.cast::<PyMotorCmd>()?.borrow().to_rust()))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(LowCmd {
            cmd_type: self.cmd_type.into(),
            motor_cmd,
        })
    }
}

#[pymethods]
impl PyLowCmd {
    /// Create a command with `joint_count` zeroed motor commands, or with the
    /// given `motor_cmd` list.
    #[new]
    #[pyo3(signature = (cmd_type=PyCmdType(CmdType::Parallel), joint_count=0, motor_cmd=None))]
    fn new(
        py: Python<'_>,
        cmd_type: PyCmdType,
        joint_count: usize,
        motor_cmd: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let list = match motor_cmd {
            Some(value) => motor_cmd_list(value)?,
            None => {
                let list = PyList::empty(py);
                for _ in 0..joint_count {
                    list.append(Py::new(py, PyMotorCmd::default())?)?;
                }
                list
            }
        };
        Ok(Self {
            cmd_type,
            motor_cmd: list.unbind(),
        })
    }

    #[getter]
    fn motor_cmd(&self, py: Python<'_>) -> Py<PyList> {
        self.motor_cmd.clone_ref(py)
    }

    #[setter]
    fn set_motor_cmd(&mut self, value: &Bound<'_, PyAny>) -> PyResult<()> {
        self.motor_cmd = motor_cmd_list(value)?.unbind();
        Ok(())
    }

    fn __len__(&self, py: Python<'_>) -> usize {
        self.motor_cmd.bind(py).len()
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!(
            "LowCmd(cmd_type={}, joints={})",
            self.cmd_type.__repr__(),
            self.motor_cmd.bind(py).len()
        )
    }
}

/// Publisher for `LowCmd` on `rt/joint_ctrl` (see `BoosterClient.low_cmd_publisher`).
#[pyclass(module = "booster_sdk_bindings", name = "LowCmdPublisher")]
pub struct PyLowCmdPublisher(pub(crate) DdsPublisher<LowCmd>);

#[pymethods]
impl PyLowCmdPublisher {
    /// Publish one command.
    fn write(&self, py: Python<'_>, cmd: PyRef<'_, PyLowCmd>) -> PyResult<()> {
        let cmd = cmd.to_rust(py)?;
        self.0.write(cmd).map_err(to_py_err)
    }

    /// Number of matched subscribers seen so far.
    fn matched_subscriptions(&self) -> usize {
        self.0.matched_subscriptions()
    }
}

// ---------------------------------------------------------------------------
// rt/low_state
// ---------------------------------------------------------------------------

py_readonly_class!(PyImuState, "ImuState", ImuState, {
    rpy: Vec<f32>,
    gyro: Vec<f32>,
    acc: Vec<f32>,
});

py_readonly_class!(PyMotorState, "MotorState", MotorState, {
    mode: u8,
    q: f32,
    dq: f32,
    ddq: f32,
    tau_est: f32,
    temperature: u8,
    lost: u32,
    reserve: Vec<u32>,
});

py_readonly_class!(PyLowState, "LowState", LowState, {
    imu_state: PyImuState,
}, {
    #[getter]
    fn motor_state_parallel(&self) -> Vec<PyMotorState> {
        self.0.motor_state_parallel.iter().copied().map(Into::into).collect()
    }

    #[getter]
    fn motor_state_serial(&self) -> Vec<PyMotorState> {
        self.0.motor_state_serial.iter().copied().map(Into::into).collect()
    }
});

// ---------------------------------------------------------------------------
// builtin_interfaces / std_msgs
// ---------------------------------------------------------------------------

py_param_class!(PyTime, "Time", Time, {
    sec: i32 = 0,
    nanosec: u32 = 0,
}, {
    /// Current wall-clock time (UNIX epoch based).
    #[staticmethod]
    fn now() -> Self {
        Self::from_rust(Time::now())
    }

    /// Time as floating-point seconds.
    fn as_secs_f64(&self) -> f64 {
        self.to_rust().as_secs_f64()
    }
});

/// `std_msgs/Header`. `stamp` is held by reference, so
/// `header.stamp.sec = 1` updates this header.
#[pyclass(module = "booster_sdk_bindings", name = "Header")]
pub struct PyHeader {
    #[pyo3(get, set)]
    stamp: Py<PyTime>,
    #[pyo3(get, set)]
    frame_id: String,
}

impl PyHeader {
    pub(crate) fn from_rust(py: Python<'_>, value: Header) -> PyResult<Self> {
        Ok(Self {
            stamp: Py::new(py, PyTime::from_rust(value.stamp))?,
            frame_id: value.frame_id,
        })
    }

    pub(crate) fn to_rust(&self, py: Python<'_>) -> Header {
        Header {
            stamp: self.stamp.borrow(py).to_rust(),
            frame_id: self.frame_id.clone(),
        }
    }
}

#[pymethods]
impl PyHeader {
    #[new]
    #[pyo3(signature = (stamp=None, frame_id=String::new()))]
    fn new(py: Python<'_>, stamp: Option<Py<PyTime>>, frame_id: String) -> PyResult<Self> {
        let stamp = match stamp {
            Some(stamp) => stamp,
            None => Py::new(py, PyTime::default())?,
        };
        Ok(Self { stamp, frame_id })
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!("{:?}", self.to_rust(py))
    }
}

// ---------------------------------------------------------------------------
// rt/kick_ball
// ---------------------------------------------------------------------------

/// Visual/soccer kick reference command (`rt/kick_ball`, K1 / T1).
#[pyclass(module = "booster_sdk_bindings", name = "Kick")]
pub struct PyKick {
    #[pyo3(get, set)]
    header: Py<PyHeader>,
    #[pyo3(get, set)]
    x: f64,
    #[pyo3(get, set)]
    y: f64,
    #[pyo3(get, set)]
    dir: f64,
    #[pyo3(get, set)]
    goal_x: f64,
    #[pyo3(get, set)]
    goal_y: f64,
    #[pyo3(get, set)]
    robot_theta_to_field: f64,
    #[pyo3(get, set)]
    power: f64,
}

impl PyKick {
    fn to_rust(&self, py: Python<'_>) -> Kick {
        Kick {
            header: self.header.borrow(py).to_rust(py),
            x: self.x,
            y: self.y,
            dir: self.dir,
            goal_x: self.goal_x,
            goal_y: self.goal_y,
            robot_theta_to_field: self.robot_theta_to_field,
            power: self.power,
        }
    }
}

#[pymethods]
impl PyKick {
    #[new]
    #[pyo3(signature = (
        header=None, x=0.0, y=0.0, dir=0.0, goal_x=0.0, goal_y=0.0,
        robot_theta_to_field=0.0, power=0.0
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        header: Option<Py<PyHeader>>,
        x: f64,
        y: f64,
        dir: f64,
        goal_x: f64,
        goal_y: f64,
        robot_theta_to_field: f64,
        power: f64,
    ) -> PyResult<Self> {
        let header = match header {
            Some(header) => header,
            None => Py::new(py, PyHeader::from_rust(py, Header::default())?)?,
        };
        Ok(Self {
            header,
            x,
            y,
            dir,
            goal_x,
            goal_y,
            robot_theta_to_field,
            power,
        })
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!("{:?}", self.to_rust(py))
    }
}

/// Publisher for `Kick` on `rt/kick_ball` (see `BoosterClient.kick_publisher`).
#[pyclass(module = "booster_sdk_bindings", name = "KickPublisher")]
pub struct PyKickPublisher(pub(crate) DdsPublisher<Kick>);

#[pymethods]
impl PyKickPublisher {
    /// Publish one kick command.
    fn write(&self, py: Python<'_>, kick: PyRef<'_, PyKick>) -> PyResult<()> {
        let kick = kick.to_rust(py);
        self.0.write(kick).map_err(to_py_err)
    }

    /// Number of matched subscribers seen so far.
    fn matched_subscriptions(&self) -> usize {
        self.0.matched_subscriptions()
    }
}

// ---------------------------------------------------------------------------
// rt/tf
// ---------------------------------------------------------------------------

impl From<booster_sdk::dds::Vector3> for PyVector3 {
    fn from(value: booster_sdk::dds::Vector3) -> Self {
        Self(booster_sdk::types::device::Vector3 {
            x: value.x,
            y: value.y,
            z: value.z,
        })
    }
}

py_readonly_class!(PyRosQuaternion, "RosQuaternion", Quaternion, {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
});

py_readonly_class!(PyRosTransform, "RosTransform", Transform, {
    translation: PyVector3,
    rotation: PyRosQuaternion,
});

py_readonly_class!(PyTransformStamped, "TransformStamped", TransformStamped, {
    child_frame_id: String,
    transform: PyRosTransform,
}, {
    #[getter]
    fn header(&self, py: Python<'_>) -> PyResult<PyHeader> {
        PyHeader::from_rust(py, self.0.header.clone())
    }
});

py_readonly_class!(PyTFMessage, "TFMessage", TFMessage, {}, {
    #[getter]
    fn transforms(&self) -> Vec<PyTransformStamped> {
        self.0.transforms.iter().cloned().map(Into::into).collect()
    }
});

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("JOINT_CNT", JOINT_CNT)?;
    m.add("JOINT_CNT_K1", JOINT_CNT_K1)?;
    m.add("JOINT_CNT_7DOF_ARM", JOINT_CNT_7DOF_ARM)?;
    m.add_class::<PyJointIndex>()?;
    m.add_class::<PyJointIndexK1>()?;
    m.add_class::<PyJointIndexWith7DofArm>()?;
    m.add_class::<PyCmdType>()?;
    m.add_class::<PyMotorCmd>()?;
    m.add_class::<PyLowCmd>()?;
    m.add_class::<PyLowCmdPublisher>()?;
    m.add_class::<PyImuState>()?;
    m.add_class::<PyMotorState>()?;
    m.add_class::<PyLowState>()?;
    m.add_class::<PyTime>()?;
    m.add_class::<PyHeader>()?;
    m.add_class::<PyKick>()?;
    m.add_class::<PyKickPublisher>()?;
    m.add_class::<PyRosQuaternion>()?;
    m.add_class::<PyRosTransform>()?;
    m.add_class::<PyTransformStamped>()?;
    m.add_class::<PyTFMessage>()?;
    Ok(())
}
