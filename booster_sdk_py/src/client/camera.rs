use std::sync::Arc;

use booster_sdk::client::camera::CameraClient;
use pyo3::{Bound, prelude::*, types::PyModule};

use super::booster::PyDeviceInfo;
use crate::{runtime::wait_for_future, startup_wait_from_seconds, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "CameraClient", unsendable)]
pub struct PyCameraClient {
    client: Arc<CameraClient>,
}

#[pymethods]
impl PyCameraClient {
    #[new]
    #[pyo3(signature = (startup_wait_sec=None))]
    fn new(startup_wait_sec: Option<f64>) -> PyResult<Self> {
        let startup_wait = startup_wait_from_seconds(startup_wait_sec)?;
        let client = match startup_wait {
            Some(wait) => CameraClient::with_startup_wait(wait),
            None => CameraClient::new(),
        }
        .map_err(to_py_err)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    fn get_cameras(&self, py: Python<'_>) -> PyResult<PyDeviceInfo> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_cameras().await })
            .map(Into::into)
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCameraClient>()?;
    Ok(())
}
