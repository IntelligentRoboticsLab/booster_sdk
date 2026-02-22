use std::sync::Arc;

use booster_sdk::client::vision::VisionClient;
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{PyDetectResults, runtime::wait_for_future, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "VisionClient", unsendable)]
pub struct PyVisionClient {
    client: Arc<VisionClient>,
}

#[pymethods]
impl PyVisionClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            client: Arc::new(VisionClient::new().map_err(to_py_err)?),
        })
    }

    fn start_vision_service(
        &self,
        py: Python<'_>,
        enable_position: bool,
        enable_color: bool,
        enable_face_detection: bool,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client
                .start_vision_service(enable_position, enable_color, enable_face_detection)
                .await
        })
        .map_err(to_py_err)
    }

    fn stop_vision_service(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_vision_service().await }).map_err(to_py_err)
    }

    fn get_detection_object_with_ratio(
        &self,
        py: Python<'_>,
        focus_ratio: f32,
    ) -> PyResult<Vec<PyDetectResults>> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.get_detection_object_with_ratio(focus_ratio).await
        })
        .map(|results| results.into_iter().map(Into::into).collect())
        .map_err(to_py_err)
    }

    fn get_detection_object(&self, py: Python<'_>) -> PyResult<Vec<PyDetectResults>> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_detection_object().await })
            .map(|results| results.into_iter().map(Into::into).collect())
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyVisionClient>()?;
    Ok(())
}
