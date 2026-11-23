//! Robot-state DDS topics exposed through `BoosterClient.subscribe_*`:
//! state/event enums and read-only message classes.
//!
//! Message classes whose name clashes with an enum of the same name get an
//! `Msg` suffix: `TrainedTrajStatusMsg` (enum: `TrainedTrajStatus`) and
//! `RobocupBehaviorStatusMsg` (enum: `RobocupBehaviorStatus`).

use booster_sdk::{
    dds::{
        BatteryState, BinaryData, ButtonEventMsg, FallDownState, HandReplyData, HandReplyParam,
        HandTouchData, HandTouchParam, MotionState, ProneBodyControlStatus, RemoteControllerState,
        RobocupBehaviorStatus as RobocupBehaviorStatusMsg, RobotDdsBatteryStatus,
        RobotDdsImuStatus, RobotDdsJointStatus, RobotProcessStateMsg, RobotProcessStatus,
        RobotReplayTrajID, RobotStatesMsg, RobotStatusDdsMsg,
        TrainedTrajStatus as TrainedTrajStatusMsg,
    },
    types::{
        ButtonEventType, FallDownStateType, GetTrainedTrajStatusResponse, ProneBodyControlPosture,
        RemoteControllerEvent, RobocupBehaviorStatus, TrainedTrajStatus,
    },
};
use pyo3::{Bound, prelude::*, types::PyBytes, types::PyModule};

use super::booster::{PyAction, PyBodyControl, PyRobotMode};
use crate::macros::{py_int_enum, py_readonly_class};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

py_int_enum!(PyRemoteControllerEvent, "RemoteControllerEvent", RemoteControllerEvent, i32, {
    NONE => None,
    AXIS => Axis,
    HAT => Hat,
    BUTTON_DOWN => ButtonDown,
    BUTTON_UP => ButtonUp,
    REMOVE => Remove,
});

py_int_enum!(PyButtonEventType, "ButtonEventType", ButtonEventType, i32, {
    PRESS_DOWN => PressDown,
    PRESS_UP => PressUp,
    SINGLE_CLICK => SingleClick,
    DOUBLE_CLICK => DoubleClick,
    TRIPLE_CLICK => TripleClick,
    LONG_PRESS_START => LongPressStart,
    LONG_PRESS_HOLD => LongPressHold,
    LONG_PRESS_END => LongPressEnd,
});

py_int_enum!(PyFallDownStateType, "FallDownStateType", FallDownStateType, i32, {
    IS_READY => IsReady,
    IS_FALLING => IsFalling,
    HAS_FALLEN => HasFallen,
    IS_GETTING_UP => IsGettingUp,
});

py_int_enum!(PyProneBodyControlPosture, "ProneBodyControlPosture", ProneBodyControlPosture, i32, {
    UNKNOWN => Unknown,
    INACTIVE => Inactive,
    PUSH_UP => PushUp,
    LIE_DOWN => LieDown,
    SOCCER_LOCOMOTION => SoccerLocomotion,
    SOCCER_KICKING => SoccerKicking,
});

py_int_enum!(PyRobocupBehaviorStatus, "RobocupBehaviorStatus", RobocupBehaviorStatus, i32, {
    RUNNING => Running,
    SHOOTING => Shooting,
    PASSING => Passing,
});

py_int_enum!(PyTrainedTrajStatus, "TrainedTrajStatus", TrainedTrajStatus, i32, {
    IDLE => Idle,
    RUNNING => Running,
});

// ---------------------------------------------------------------------------
// rt/device_gateway
// ---------------------------------------------------------------------------

py_readonly_class!(PyRobotDdsJointStatus, "RobotDdsJointStatus", RobotDdsJointStatus, {
    name: String,
    index: i32,
    is_connected: bool,
    temperature: i32,
    is_limited: i32,
    status_code: i32,
    temperature_level: i32,
});

py_readonly_class!(PyRobotDdsImuStatus, "RobotDdsImuStatus", RobotDdsImuStatus, {
    name: String,
    index: i32,
    is_connected: bool,
    status_code: i32,
});

py_readonly_class!(PyRobotDdsBatteryStatus, "RobotDdsBatteryStatus", RobotDdsBatteryStatus, {
    name: String,
    temperature: f32,
    soc: f32,
    voltage: f32,
    status_code: i32,
    status_level: i32,
});

/// Convert a slice of Rust messages into their Python classes.
fn convert_vec<T: Clone, P: From<T>>(values: &[T]) -> Vec<P> {
    values.iter().cloned().map(P::from).collect()
}

py_readonly_class!(
    PyRobotStatusDdsMsg,
    "RobotStatusDdsMsg",
    RobotStatusDdsMsg,
    {},
    {
        #[getter]
        fn joint_vec(&self) -> Vec<PyRobotDdsJointStatus> {
            convert_vec(&self.0.joint_vec)
        }

        #[getter]
        fn imu_vec(&self) -> Vec<PyRobotDdsImuStatus> {
            convert_vec(&self.0.imu_vec)
        }

        #[getter]
        fn battery_vec(&self) -> Vec<PyRobotDdsBatteryStatus> {
            convert_vec(&self.0.battery_vec)
        }
    }
);

// ---------------------------------------------------------------------------
// Motion, battery, buttons, remote controller, processes, video
// ---------------------------------------------------------------------------

py_readonly_class!(PyMotionState, "MotionState", MotionState, {
    current_mode: i32,
    target_mode: i32,
    is_transitioning: bool,
});

py_readonly_class!(PyBatteryState, "BatteryState", BatteryState, {
    voltage: f32,
    current: f32,
    soc: f32,
    average_voltage: f32,
});

py_readonly_class!(PyButtonEventMsg, "ButtonEventMsg", ButtonEventMsg, {
    button: i32,
    event: PyButtonEventType,
});

py_readonly_class!(PyRemoteControllerState, "RemoteControllerState", RemoteControllerState, {
    event: u32,
    lx: f32,
    ly: f32,
    rx: f32,
    ry: f32,
    a: bool,
    b: bool,
    x: bool,
    y: bool,
    lb: bool,
    rb: bool,
    lt: bool,
    rt: bool,
    ls: bool,
    rs: bool,
    back: bool,
    start: bool,
    hat_c: bool,
    hat_u: bool,
    hat_d: bool,
    hat_l: bool,
    hat_r: bool,
    hat_lu: bool,
    hat_ld: bool,
    hat_ru: bool,
    hat_rd: bool,
    reserved: u8,
}, {
    /// Decoded `event`, or `None` if unknown.
    fn event_kind(&self) -> Option<PyRemoteControllerEvent> {
        self.0.event_kind().map(Into::into)
    }
});

py_readonly_class!(PyRobotProcessStatus, "RobotProcessStatus", RobotProcessStatus, {
    name: String,
    pid: i32,
    status: i32,
});

py_readonly_class!(
    PyRobotProcessStateMsg,
    "RobotProcessStateMsg",
    RobotProcessStateMsg,
    {},
    {
        #[getter]
        fn process_vec(&self) -> Vec<PyRobotProcessStatus> {
            convert_vec(&self.0.process_vec)
        }
    }
);

py_readonly_class!(PyBinaryData, "BinaryData", BinaryData, {
    timestamp: i64,
    sequence_num: u32,
    encoding: String,
}, {
    #[getter]
    fn data<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.data)
    }
});

// ---------------------------------------------------------------------------
// Fall-down, robot states, prone body, trajectories, soccer
// ---------------------------------------------------------------------------

py_readonly_class!(PyFallDownState, "FallDownState", FallDownState, {
    fall_down_state: PyFallDownStateType,
    is_recovery_available: bool,
});

py_readonly_class!(PyRobotStatesMsg, "RobotStatesMsg", RobotStatesMsg, {
    current_mode: i32,
    current_body_control: i32,
    current_actions: Vec<i32>,
}, {
    /// Decoded `current_mode`, or `None` if unknown.
    fn mode(&self) -> Option<PyRobotMode> {
        self.0.mode().map(Into::into)
    }

    /// Decoded `current_body_control`, or `None` if unknown.
    fn body_control(&self) -> Option<PyBodyControl> {
        self.0.body_control().map(Into::into)
    }

    /// Decoded `current_actions` (unknown values are skipped).
    fn actions(&self) -> Vec<PyAction> {
        self.0.actions().into_iter().map(Into::into).collect()
    }
});

py_readonly_class!(PyProneBodyControlStatus, "ProneBodyControlStatus", ProneBodyControlStatus, {
    posture: i32,
}, {
    /// Decoded `posture`, or `None` if unknown.
    fn posture_kind(&self) -> Option<PyProneBodyControlPosture> {
        self.0.posture_kind().map(Into::into)
    }
});

py_readonly_class!(PyRobotReplayTrajID, "RobotReplayTrajID", RobotReplayTrajID, {
    id: String,
});

py_readonly_class!(PyTrainedTrajStatusMsg, "TrainedTrajStatusMsg", TrainedTrajStatusMsg, {
    status: i32,
    traj_id: String,
}, {
    /// Decoded `status`, or `None` if unknown.
    fn status_kind(&self) -> Option<PyTrainedTrajStatus> {
        self.0.status_kind().map(Into::into)
    }
});

py_readonly_class!(PyRobocupBehaviorStatusMsg, "RobocupBehaviorStatusMsg", RobocupBehaviorStatusMsg, {
    status: i32,
}, {
    /// Decoded `status`, or `None` if unknown.
    fn status_kind(&self) -> Option<PyRobocupBehaviorStatus> {
        self.0.status_kind().map(Into::into)
    }
});

py_readonly_class!(
    PyGetTrainedTrajStatusResponse,
    "GetTrainedTrajStatusResponse",
    GetTrainedTrajStatusResponse,
    { status: i32, traj_id: String },
    {
        #[new]
        #[pyo3(signature = (status, traj_id=String::new()))]
        fn new(status: i32, traj_id: String) -> Self {
            Self(GetTrainedTrajStatusResponse { status, traj_id })
        }

        /// Whether a trained trajectory is currently running.
        fn is_running(&self) -> bool {
            self.0.is_running()
        }

        /// Decoded `status`, or `None` if unknown.
        fn status_enum(&self) -> Option<PyTrainedTrajStatus> {
            self.0.status_enum().map(Into::into)
        }
    }
);

// ---------------------------------------------------------------------------
// Dexterous hands
// ---------------------------------------------------------------------------

py_readonly_class!(PyHandReplyParam, "HandReplyParam", HandReplyParam, {
    angle: i32,
    force: i32,
    current: i32,
    error: i32,
    status: i32,
    temp: i32,
    seq: i32,
});

py_readonly_class!(PyHandReplyData, "HandReplyData", HandReplyData, {
    hand_index: i32,
    hand_type: i32,
}, {
    #[getter]
    fn hand_data(&self) -> Vec<PyHandReplyParam> {
        convert_vec(&self.0.hand_data)
    }
});

py_readonly_class!(PyHandTouchParam, "HandTouchParam", HandTouchParam, {}, {
    #[getter]
    fn finger_one<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.finger_one)
    }

    #[getter]
    fn finger_two<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.finger_two)
    }

    #[getter]
    fn finger_three<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.finger_three)
    }

    #[getter]
    fn finger_four<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.finger_four)
    }

    #[getter]
    fn finger_five<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.finger_five)
    }

    #[getter]
    fn finger_palm<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.finger_palm)
    }
});

py_readonly_class!(PyHandTouchData, "HandTouchData", HandTouchData, {
    touch_data: PyHandTouchParam,
    hand_index: i32,
    hand_type: i32,
});

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRemoteControllerEvent>()?;
    m.add_class::<PyButtonEventType>()?;
    m.add_class::<PyFallDownStateType>()?;
    m.add_class::<PyProneBodyControlPosture>()?;
    m.add_class::<PyRobocupBehaviorStatus>()?;
    m.add_class::<PyTrainedTrajStatus>()?;
    m.add_class::<PyRobotDdsJointStatus>()?;
    m.add_class::<PyRobotDdsImuStatus>()?;
    m.add_class::<PyRobotDdsBatteryStatus>()?;
    m.add_class::<PyRobotStatusDdsMsg>()?;
    m.add_class::<PyMotionState>()?;
    m.add_class::<PyBatteryState>()?;
    m.add_class::<PyButtonEventMsg>()?;
    m.add_class::<PyRemoteControllerState>()?;
    m.add_class::<PyRobotProcessStatus>()?;
    m.add_class::<PyRobotProcessStateMsg>()?;
    m.add_class::<PyBinaryData>()?;
    m.add_class::<PyFallDownState>()?;
    m.add_class::<PyRobotStatesMsg>()?;
    m.add_class::<PyProneBodyControlStatus>()?;
    m.add_class::<PyRobotReplayTrajID>()?;
    m.add_class::<PyTrainedTrajStatusMsg>()?;
    m.add_class::<PyRobocupBehaviorStatusMsg>()?;
    m.add_class::<PyGetTrainedTrajStatusResponse>()?;
    m.add_class::<PyHandReplyParam>()?;
    m.add_class::<PyHandReplyData>()?;
    m.add_class::<PyHandTouchParam>()?;
    m.add_class::<PyHandTouchData>()?;
    Ok(())
}
