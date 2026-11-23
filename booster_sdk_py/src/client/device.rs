//! Typed device catalogs (`booster_sdk::types::device`).
//!
//! `Camera`/`CameraList`, `ImuInfo`/`ImuInfoList` and `HandInfo`/`HandList`
//! are full classes. The robot model (links/joints tree) is returned as a
//! plain `dict` mirroring the JSON wire form.
//!
//! `DeviceInfo` lives in `booster.rs`; the `*_from_device_info` module
//! functions here parse it into the typed catalogs.

use booster_sdk::types::{
    DeviceInfo,
    device::{
        Camera, CameraConnectionState, CameraExtrinsics, CameraList, CameraModel,
        CameraPixelFormat, CameraPos, DepthPixelFormat, DepthStreamEncoding, GeometryType, Hand,
        HandList, HandModel, HandPos, ImuInfo, ImuInfoList, ImuModel, JointType, RobotModel,
        Vector3, VideoStreamEncoding,
    },
};
use pyo3::{Bound, exceptions::PyValueError, prelude::*, types::PyModule};

use super::booster::PyDeviceInfo;
use crate::{
    json_value_to_py,
    macros::{py_int_enum, py_readonly_class},
    to_py_err,
};

py_int_enum!(PyCameraModel, "CameraModel", CameraModel, u32, {
    UNKNOWN => Unknown,
    REALSENSE_D455 => RealsenseD455,
    ZED2I => Zed2i,
    ORBBEC_GEMINI_335L => OrbbecGemini335l,
    BOOSTER_MIPI => BoosterMipi,
});
py_int_enum!(PyCameraConnectionState, "CameraConnectionState", CameraConnectionState, u32, {
    UNKNOWN => Unknown,
    CONNECTED => Connected,
    CONNECTION_LOST => ConnectionLost,
});
py_int_enum!(PyCameraPos, "CameraPos", CameraPos, u32, {
    UNKNOWN => Unknown,
    HEAD => Head,
    LEFT_HAND => LeftHand,
    RIGHT_HAND => RightHand,
    WAIST => Waist,
});
py_int_enum!(PyCameraPixelFormat, "CameraPixelFormat", CameraPixelFormat, u32, {
    UNKNOWN => Unknown,
    RGB8 => Rgb8,
    BGR8 => Bgr8,
    YUV422 => Yuv422,
    JPEG => Jpeg,
    NV12 => Nv12,
});
py_int_enum!(PyDepthPixelFormat, "DepthPixelFormat", DepthPixelFormat, u32, {
    UNKNOWN => Unknown,
    UINT16 => Uint16,
    FLOAT32 => Float32,
});
py_int_enum!(PyImuModel, "ImuModel", ImuModel, u32, {
    UNKNOWN => Unknown,
    HI13R4N => Hi13r4n,
});
py_int_enum!(PyHandModel, "HandModel", HandModel, u32, {
    UNKNOWN => Unknown,
    INSPIRE_HAND_RH56DFX => InspireHandRh56dfx,
    INSPIRE_GRIPPER_EG2_4X2 => InspireGripperEg2_4x2,
    INSPIRE_TOUCH_HAND_RH56DFTP => InspireTouchHandRh56dftp,
});
py_int_enum!(PyHandPos, "HandPos", HandPos, u32, {
    UNKNOWN => Unknown,
    LEFT => Left,
    RIGHT => Right,
});
py_int_enum!(PyGeometryType, "GeometryType", GeometryType, u32, {
    BOX => Box,
    CYLINDER => Cylinder,
    SPHERE => Sphere,
    MESH => Mesh,
});
py_int_enum!(PyJointType, "JointType", JointType, u32, {
    FIXED => Fixed,
    REVOLUTE => Revolute,
    CONTINUOUS => Continuous,
    PRISMATIC => Prismatic,
    PLANAR => Planar,
    FLOATING => Floating,
});

pub(crate) fn json_err(err: serde_json::Error) -> PyErr {
    PyValueError::new_err(err.to_string())
}

fn parse_json(json: &str) -> PyResult<serde_json::Value> {
    serde_json::from_str(json).map_err(json_err)
}

py_readonly_class!(PyVector3, "Vector3", Vector3, { x: f64, y: f64, z: f64 });

py_readonly_class!(PyCameraExtrinsics, "CameraExtrinsics", CameraExtrinsics, {
    parent_frame: String,
    xyz: PyVector3,
    rpy: PyVector3,
});

py_readonly_class!(PyVideoStreamEncoding, "VideoStreamEncoding", VideoStreamEncoding, {
    width: u32,
    height: u32,
    format: PyCameraPixelFormat,
    bitrate_bps: u32,
});

py_readonly_class!(PyDepthStreamEncoding, "DepthStreamEncoding", DepthStreamEncoding, {
    width: u32,
    height: u32,
    format: PyDepthPixelFormat,
});

py_readonly_class!(PyCamera, "Camera", Camera, {
    name: String,
    frame_id: String,
    serial_number: String,
    pos: PyCameraPos,
    model: PyCameraModel,
    state: PyCameraConnectionState,
    depth_scale: f64,
    extrinsics: PyCameraExtrinsics,
    color_encoding: PyVideoStreamEncoding,
    depth_encoding: PyDepthStreamEncoding,
}, {
    /// Topic names the camera publishes on.
    #[getter]
    fn topics(&self) -> Vec<String> {
        self.0.topics.topics.clone()
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(json_err)
    }

    /// Parse one camera RPC JSON object (C++ `CameraFromRpcJson`).
    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        Camera::from_rpc_json(&parse_json(json)?)
            .map(Self)
            .map_err(to_py_err)
    }
});

py_readonly_class!(PyCameraList, "CameraList", CameraList, {}, {
    #[getter]
    fn cameras(&self) -> Vec<PyCamera> {
        self.0.cameras.iter().cloned().map(PyCamera).collect()
    }

    fn __len__(&self) -> usize {
        self.0.cameras.len()
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(json_err)
    }

    /// Parse a camera-list RPC body: a bare array or `{"cameras": [...]}`
    /// (C++ `CameraListFromRpcJson`).
    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        CameraList::from_rpc_json(&parse_json(json)?)
            .map(Self)
            .map_err(to_py_err)
    }

    /// Parse a camera `DeviceInfo`; empty unless its kind is CAMERA.
    #[staticmethod]
    fn from_device_info(info: PyRef<'_, PyDeviceInfo>) -> PyResult<Self> {
        CameraList::from_device_info(&info.0)
            .map(Self)
            .map_err(to_py_err)
    }
});

py_readonly_class!(PyImuInfo, "ImuInfo", ImuInfo, {
    index: i32,
    name: String,
    mount_position: String,
    model: PyImuModel,
});

py_readonly_class!(PyImuInfoList, "ImuInfoList", ImuInfoList, {}, {
    #[getter]
    fn imus(&self) -> Vec<PyImuInfo> {
        self.0.imus.iter().cloned().map(PyImuInfo).collect()
    }

    fn __len__(&self) -> usize {
        self.0.imus.len()
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(json_err)
    }

    /// Parse a sensors `DeviceInfo`; empty unless its kind is SENSORS.
    #[staticmethod]
    fn from_device_info(info: PyRef<'_, PyDeviceInfo>) -> PyResult<Self> {
        ImuInfoList::from_device_info(&info.0)
            .map(Self)
            .map_err(to_py_err)
    }
});

py_readonly_class!(PyHandInfo, "HandInfo", Hand, {
    model: PyHandModel,
    pos: PyHandPos,
});

py_readonly_class!(PyHandList, "HandList", HandList, {}, {
    #[getter]
    fn hands(&self) -> Vec<PyHandInfo> {
        self.0.hands.iter().copied().map(PyHandInfo).collect()
    }

    fn __len__(&self) -> usize {
        self.0.hands.len()
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(json_err)
    }

    /// Parse a hands `DeviceInfo`; empty unless its kind is HANDS.
    #[staticmethod]
    fn from_device_info(info: PyRef<'_, PyDeviceInfo>) -> PyResult<Self> {
        HandList::from_device_info(&info.0)
            .map(Self)
            .map_err(to_py_err)
    }
});

/// Parse a camera `DeviceInfo` (C++ `CameraListFromDeviceInfo`).
#[pyfunction]
fn camera_list_from_device_info(info: PyRef<'_, PyDeviceInfo>) -> PyResult<PyCameraList> {
    PyCameraList::from_device_info(info)
}

/// Parse a sensors `DeviceInfo` (C++ `ImuInfoListFromDeviceInfo`).
#[pyfunction]
fn imu_info_list_from_device_info(info: PyRef<'_, PyDeviceInfo>) -> PyResult<PyImuInfoList> {
    PyImuInfoList::from_device_info(info)
}

/// Parse a hands `DeviceInfo` (C++ `HandListFromDeviceInfo`).
#[pyfunction]
fn hand_list_from_device_info(info: PyRef<'_, PyDeviceInfo>) -> PyResult<PyHandList> {
    PyHandList::from_device_info(info)
}

/// Parse a robot-model `DeviceInfo` (C++ `RobotModelFromDeviceInfo`) into a
/// dict with `root_link`, `links` and `joints` (enum fields as ints, see
/// `GeometryType` / `JointType`). Returns `None` unless the kind is
/// ROBOT_MODEL.
#[pyfunction]
fn robot_model_from_device_info(
    py: Python<'_>,
    info: PyRef<'_, PyDeviceInfo>,
) -> PyResult<Option<Py<PyAny>>> {
    let Some(model) = RobotModel::from_device_info(&info.0).map_err(to_py_err)? else {
        return Ok(None);
    };
    let value = serde_json::to_value(&model).map_err(json_err)?;
    json_value_to_py(py, &value).map(Some)
}

/// Parse a serialized `DeviceInfo` (C++ `DeviceInfo::FromJson`): either
/// `{"kind": <int>, "body": ...}` or a bare RPC body (kind UNKNOWN).
#[pyfunction]
fn device_info_from_json(json: &str) -> PyResult<PyDeviceInfo> {
    DeviceInfo::from_json(json)
        .map(Into::into)
        .map_err(to_py_err)
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCameraModel>()?;
    m.add_class::<PyCameraConnectionState>()?;
    m.add_class::<PyCameraPos>()?;
    m.add_class::<PyCameraPixelFormat>()?;
    m.add_class::<PyDepthPixelFormat>()?;
    m.add_class::<PyImuModel>()?;
    m.add_class::<PyHandModel>()?;
    m.add_class::<PyHandPos>()?;
    m.add_class::<PyGeometryType>()?;
    m.add_class::<PyJointType>()?;
    m.add_class::<PyVector3>()?;
    m.add_class::<PyCameraExtrinsics>()?;
    m.add_class::<PyVideoStreamEncoding>()?;
    m.add_class::<PyDepthStreamEncoding>()?;
    m.add_class::<PyCamera>()?;
    m.add_class::<PyCameraList>()?;
    m.add_class::<PyImuInfo>()?;
    m.add_class::<PyImuInfoList>()?;
    m.add_class::<PyHandInfo>()?;
    m.add_class::<PyHandList>()?;
    m.add_function(wrap_pyfunction!(camera_list_from_device_info, m)?)?;
    m.add_function(wrap_pyfunction!(imu_info_list_from_device_info, m)?)?;
    m.add_function(wrap_pyfunction!(hand_list_from_device_info, m)?)?;
    m.add_function(wrap_pyfunction!(robot_model_from_device_info, m)?)?;
    m.add_function(wrap_pyfunction!(device_info_from_json, m)?)?;
    Ok(())
}
