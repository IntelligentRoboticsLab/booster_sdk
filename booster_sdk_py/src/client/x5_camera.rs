use std::sync::Arc;

use booster_sdk::client::x5_camera::X5CameraClient;
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{PyCameraSetMode, PyX5CameraGetStatusResponse, runtime::wait_for_future, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "X5CameraClient", unsendable)]
pub struct PyX5CameraClient {
    client: Arc<X5CameraClient>,
}

#[pymethods]
impl PyX5CameraClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            client: Arc::new(X5CameraClient::new().map_err(to_py_err)?),
        })
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
    m.add_class::<PyX5CameraClient>()?;
    Ok(())
}
