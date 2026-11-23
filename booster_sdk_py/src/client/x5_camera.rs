use std::sync::Arc;

use booster_sdk::client::x5_camera::{
    CameraControlStatus, CameraSetMode, ChangeModeParameter,
    GetStatusResponse as X5CameraStatusResponse, X5CameraApiId, X5CameraClient,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{
    macros::{py_int_enum, py_param_class},
    options::rpc_options,
    runtime::wait_for_future,
    to_py_err,
};

py_int_enum!(PyX5CameraApiId, "X5CameraApiId", X5CameraApiId, i32, {
    CHANGE_MODE => ChangeMode,
    GET_STATUS => GetStatus,
});

py_param_class!(
    PyX5CameraChangeModeParameter,
    "X5CameraChangeModeParameter",
    ChangeModeParameter,
    { mode: i32 = 0 }
);

#[pyclass(module = "booster_sdk_bindings", name = "CameraSetMode", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyCameraSetMode(CameraSetMode);

#[pymethods]
impl PyCameraSetMode {
    #[classattr]
    const CAMERA_MODE_NORMAL: Self = Self(CameraSetMode::CameraModeNormal);
    #[classattr]
    const CAMERA_MODE_HIGH_RESOLUTION: Self = Self(CameraSetMode::CameraModeHighResolution);
    #[classattr]
    const CAMERA_MODE_NORMAL_ENABLE: Self = Self(CameraSetMode::CameraModeNormalEnable);
    #[classattr]
    const CAMERA_MODE_HIGH_RESOLUTION_ENABLE: Self =
        Self(CameraSetMode::CameraModeHighResolutionEnable);

    fn __repr__(&self) -> String {
        match self.0 {
            CameraSetMode::CameraModeNormal => "CameraSetMode.CAMERA_MODE_NORMAL".to_string(),
            CameraSetMode::CameraModeHighResolution => {
                "CameraSetMode.CAMERA_MODE_HIGH_RESOLUTION".to_string()
            }
            CameraSetMode::CameraModeNormalEnable => {
                "CameraSetMode.CAMERA_MODE_NORMAL_ENABLE".to_string()
            }
            CameraSetMode::CameraModeHighResolutionEnable => {
                "CameraSetMode.CAMERA_MODE_HIGH_RESOLUTION_ENABLE".to_string()
            }
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<PyCameraSetMode> for CameraSetMode {
    fn from(value: PyCameraSetMode) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "CameraControlStatus", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyCameraControlStatus(CameraControlStatus);

#[pymethods]
impl PyCameraControlStatus {
    #[classattr]
    const CAMERA_STATUS_NORMAL: Self = Self(CameraControlStatus::CameraStatusNormal);
    #[classattr]
    const CAMERA_STATUS_HIGH_RESOLUTION: Self =
        Self(CameraControlStatus::CameraStatusHighResolution);
    #[classattr]
    const CAMERA_STATUS_ERROR: Self = Self(CameraControlStatus::CameraStatusError);
    #[classattr]
    const CAMERA_STATUS_NULL: Self = Self(CameraControlStatus::CameraStatusNull);

    fn __repr__(&self) -> String {
        match self.0 {
            CameraControlStatus::CameraStatusNormal => {
                "CameraControlStatus.CAMERA_STATUS_NORMAL".to_string()
            }
            CameraControlStatus::CameraStatusHighResolution => {
                "CameraControlStatus.CAMERA_STATUS_HIGH_RESOLUTION".to_string()
            }
            CameraControlStatus::CameraStatusError => {
                "CameraControlStatus.CAMERA_STATUS_ERROR".to_string()
            }
            CameraControlStatus::CameraStatusNull => {
                "CameraControlStatus.CAMERA_STATUS_NULL".to_string()
            }
        }
    }

    fn __int__(&self) -> i32 {
        i32::from(self.0)
    }
}

impl From<CameraControlStatus> for PyCameraControlStatus {
    fn from(value: CameraControlStatus) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "X5CameraGetStatusResponse")]
#[derive(Clone)]
pub struct PyX5CameraGetStatusResponse(X5CameraStatusResponse);

#[pymethods]
impl PyX5CameraGetStatusResponse {
    #[new]
    fn new(status: i32) -> Self {
        Self(X5CameraStatusResponse { status })
    }

    #[getter]
    fn status(&self) -> i32 {
        self.0.status
    }

    fn status_enum(&self) -> Option<PyCameraControlStatus> {
        self.0.status_enum().map(Into::into)
    }

    fn __repr__(&self) -> String {
        format!("X5CameraGetStatusResponse(status={})", self.0.status)
    }
}

impl From<X5CameraStatusResponse> for PyX5CameraGetStatusResponse {
    fn from(value: X5CameraStatusResponse) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "X5CameraClient", unsendable)]
pub struct PyX5CameraClient {
    client: Arc<X5CameraClient>,
}

#[pymethods]
impl PyX5CameraClient {
    /// The X5 camera service has no robot-name channel suffix (like C++).
    #[new]
    #[pyo3(signature = (startup_wait_sec=None, *, network=None, domain_id=None))]
    fn new(
        startup_wait_sec: Option<f64>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let options = rpc_options(startup_wait_sec, None, network, domain_id)?;
        let client = X5CameraClient::with_options(options).map_err(to_py_err)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Send a raw request; the response body is discarded.
    fn send_api_request(&self, py: Python<'_>, api_id: i32, param: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.send_api_request(api_id, &param).await },
        )
        .map_err(to_py_err)
    }

    /// Send a raw request and return the response body.
    fn send_api_request_with_response(
        &self,
        py: Python<'_>,
        api_id: i32,
        param: String,
    ) -> PyResult<String> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.send_api_request_with_response(api_id, &param).await
        })
        .map_err(to_py_err)
    }

    fn change_mode(&self, py: Python<'_>, mode: PyCameraSetMode) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.change_mode(mode.into()).await }).map_err(to_py_err)
    }

    fn get_status(&self, py: Python<'_>) -> PyResult<PyX5CameraGetStatusResponse> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_status().await })
            .map(Into::into)
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyX5CameraApiId>()?;
    m.add_class::<PyX5CameraChangeModeParameter>()?;
    m.add_class::<PyCameraSetMode>()?;
    m.add_class::<PyCameraControlStatus>()?;
    m.add_class::<PyX5CameraGetStatusResponse>()?;
    m.add_class::<PyX5CameraClient>()?;
    Ok(())
}
