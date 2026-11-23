use std::sync::Arc;

use booster_sdk::client::vision::{
    DetectResults, GetDetectionObjectParameter, StartVisionServiceParameter, VisionApiId,
    VisionClient,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{
    macros::{py_int_enum, py_param_class},
    options::rpc_options,
    runtime::wait_for_future,
    to_py_err,
};

py_int_enum!(PyVisionApiId, "VisionApiId", VisionApiId, i32, {
    START_VISION_SERVICE => StartVisionService,
    STOP_VISION_SERVICE => StopVisionService,
    GET_DETECTION_OBJECT => GetDetectionObject,
});

py_param_class!(
    PyStartVisionServiceParameter,
    "StartVisionServiceParameter",
    StartVisionServiceParameter,
    {
        enable_position: bool = false,
        enable_color: bool = false,
        enable_face_detection: bool = false,
    }
);

py_param_class!(
    PyGetDetectionObjectParameter,
    "GetDetectionObjectParameter",
    GetDetectionObjectParameter,
    { focus_ratio: f32 = 0.33 }
);

py_param_class!(PyDetectResults, "DetectResults", DetectResults, {
    xmin: i64 = 0,
    ymin: i64 = 0,
    xmax: i64 = 0,
    ymax: i64 = 0,
    position: Vec<f32> = Vec::new(),
    tag: String = String::new(),
    conf: f32 = 0.0,
    rgb_mean: Vec<i32> = Vec::new(),
});

#[pyclass(module = "booster_sdk_bindings", name = "VisionClient", unsendable)]
pub struct PyVisionClient {
    client: Arc<VisionClient>,
}

fn into_py_results(results: Vec<DetectResults>) -> Vec<PyDetectResults> {
    results.into_iter().map(Into::into).collect()
}

#[pymethods]
impl PyVisionClient {
    #[new]
    #[pyo3(signature = (startup_wait_sec=None, *, robot_name=None, network=None, domain_id=None))]
    fn new(
        startup_wait_sec: Option<f64>,
        robot_name: Option<String>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let options = rpc_options(startup_wait_sec, robot_name, network, domain_id)?;
        let client = VisionClient::with_options(options).map_err(to_py_err)?;
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

    #[pyo3(signature = (enable_position=false, enable_color=false, enable_face_detection=false))]
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
        .map(into_py_results)
        .map_err(to_py_err)
    }

    /// Detected objects; `focus_ratio` defaults to 0.33 (C++ default).
    #[pyo3(signature = (focus_ratio=None))]
    fn get_detection_object(
        &self,
        py: Python<'_>,
        focus_ratio: Option<f32>,
    ) -> PyResult<Vec<PyDetectResults>> {
        let client = Arc::clone(&self.client);
        let focus_ratio = focus_ratio.unwrap_or(GetDetectionObjectParameter::default().focus_ratio);
        wait_for_future(py, async move {
            client.get_detection_object_with_ratio(focus_ratio).await
        })
        .map(into_py_results)
        .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyVisionApiId>()?;
    m.add_class::<PyStartVisionServiceParameter>()?;
    m.add_class::<PyGetDetectionObjectParameter>()?;
    m.add_class::<PyDetectResults>()?;
    m.add_class::<PyVisionClient>()?;
    Ok(())
}
