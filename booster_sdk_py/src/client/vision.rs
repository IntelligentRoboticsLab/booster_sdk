use std::sync::Arc;

use booster_sdk::client::vision::{DetectResults, VisionClient};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{runtime::wait_for_future, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "DetectResults")]
#[derive(Clone)]
pub struct PyDetectResults(DetectResults);

#[pymethods]
impl PyDetectResults {
    #[new]
    #[expect(clippy::too_many_arguments)]
    fn new(
        xmin: i64,
        ymin: i64,
        xmax: i64,
        ymax: i64,
        position: Vec<f32>,
        tag: String,
        conf: f32,
        rgb_mean: Vec<i32>,
    ) -> Self {
        Self(DetectResults {
            xmin,
            ymin,
            xmax,
            ymax,
            position,
            tag,
            conf,
            rgb_mean,
        })
    }

    #[getter]
    fn xmin(&self) -> i64 {
        self.0.xmin
    }

    #[getter]
    fn ymin(&self) -> i64 {
        self.0.ymin
    }

    #[getter]
    fn xmax(&self) -> i64 {
        self.0.xmax
    }

    #[getter]
    fn ymax(&self) -> i64 {
        self.0.ymax
    }

    #[getter]
    fn position(&self) -> Vec<f32> {
        self.0.position.clone()
    }

    #[getter]
    fn tag(&self) -> String {
        self.0.tag.clone()
    }

    #[getter]
    fn conf(&self) -> f32 {
        self.0.conf
    }

    #[getter]
    fn rgb_mean(&self) -> Vec<i32> {
        self.0.rgb_mean.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "DetectResults(tag='{}', conf={}, bbox=({}, {}, {}, {}))",
            self.0.tag, self.0.conf, self.0.xmin, self.0.ymin, self.0.xmax, self.0.ymax
        )
    }
}

impl From<DetectResults> for PyDetectResults {
    fn from(value: DetectResults) -> Self {
        Self(value)
    }
}

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
    m.add_class::<PyDetectResults>()?;
    m.add_class::<PyVisionClient>()?;
    Ok(())
}
