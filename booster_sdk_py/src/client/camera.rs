use std::sync::Arc;

use booster_sdk::client::camera::{CameraApiId, CameraClient};
use pyo3::{Bound, prelude::*, types::PyModule};

use super::{booster::PyDeviceInfo, device::PyCameraList};
use crate::{macros::py_int_enum, options::rpc_options, runtime::wait_for_future, to_py_err};

py_int_enum!(PyCameraApiId, "CameraApiId", CameraApiId, i32, {
    GET_CAMERAS => GetCameras,
});

#[pyclass(module = "booster_sdk_bindings", name = "CameraClient", unsendable)]
pub struct PyCameraClient {
    client: Arc<CameraClient>,
}

#[pymethods]
impl PyCameraClient {
    #[new]
    #[pyo3(signature = (startup_wait_sec=None, *, robot_name=None, network=None, domain_id=None))]
    fn new(
        startup_wait_sec: Option<f64>,
        robot_name: Option<String>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let options = rpc_options(startup_wait_sec, robot_name, network, domain_id)?;
        let client = CameraClient::with_options(options).map_err(to_py_err)?;
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

    /// Typed camera catalog (C++ `GetCameraList`).
    fn get_camera_list(&self, py: Python<'_>) -> PyResult<PyCameraList> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_camera_list().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn get_cameras(&self, py: Python<'_>) -> PyResult<PyDeviceInfo> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_cameras().await })
            .map(Into::into)
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCameraApiId>()?;
    m.add_class::<PyCameraClient>()?;
    Ok(())
}
