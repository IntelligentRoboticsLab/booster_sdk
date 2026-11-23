use std::sync::Arc;

use booster_sdk::client::handeye_calib::{
    HandEyeCalibApiId, HandEyeCalibApplyResult, HandEyeCalibClient, HandEyeCalibResult,
    HandEyeCalibStatus, StartHandEyeCalibParameter,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{
    client::device::json_err,
    json_value_to_py,
    macros::{py_int_enum, py_param_class},
    options::rpc_options,
    py_to_json_value,
    runtime::wait_for_future,
    to_py_err,
};

py_int_enum!(PyHandEyeCalibApiId, "HandEyeCalibApiId", HandEyeCalibApiId, i32, {
    START_CALIBRATION => StartCalibration,
    STOP_CALIBRATION => StopCalibration,
    GET_STATUS => GetStatus,
    GET_RESULT => GetResult,
    APPLY_RESULT => ApplyResult,
});

py_param_class!(
    PyStartHandEyeCalibParameter,
    "StartHandEyeCalibParameter",
    StartHandEyeCalibParameter,
    {
        publish_feedback: bool = true,
        square_size_m: Option<f64> = None,
    }
);

fn optional_json(value: Option<&Bound<'_, PyAny>>) -> PyResult<Option<serde_json::Value>> {
    value
        .filter(|value| !value.is_none())
        .map(py_to_json_value)
        .transpose()
}

fn optional_py(py: Python<'_>, value: Option<&serde_json::Value>) -> PyResult<Option<Py<PyAny>>> {
    value.map(|value| json_value_to_py(py, value)).transpose()
}

#[pyclass(module = "booster_sdk_bindings", name = "HandEyeCalibStatus")]
#[derive(Clone)]
pub struct PyHandEyeCalibStatus(HandEyeCalibStatus);

#[pymethods]
impl PyHandEyeCalibStatus {
    #[new]
    #[pyo3(signature = (
        status=String::new(),
        job_id=String::new(),
        stage=String::new(),
        started_at=String::new(),
        finished_at=String::new(),
        progress=0.0,
        stage2_done=0,
        stage2_total=0,
        stage3_done=0,
        stage3_total=0,
        error=None,
    ))]
    #[expect(clippy::too_many_arguments)]
    fn new(
        status: String,
        job_id: String,
        stage: String,
        started_at: String,
        finished_at: String,
        progress: f64,
        stage2_done: i32,
        stage2_total: i32,
        stage3_done: i32,
        stage3_total: i32,
        error: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        Ok(Self(HandEyeCalibStatus {
            status,
            job_id,
            stage,
            started_at,
            finished_at,
            progress,
            stage2_done,
            stage2_total,
            stage3_done,
            stage3_total,
            error: optional_json(error)?,
        }))
    }

    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        serde_json::from_str(json).map(Self).map_err(json_err)
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(json_err)
    }

    #[getter]
    fn status(&self) -> String {
        self.0.status.clone()
    }
    #[getter]
    fn job_id(&self) -> String {
        self.0.job_id.clone()
    }
    #[getter]
    fn stage(&self) -> String {
        self.0.stage.clone()
    }
    #[getter]
    fn started_at(&self) -> String {
        self.0.started_at.clone()
    }
    #[getter]
    fn finished_at(&self) -> String {
        self.0.finished_at.clone()
    }
    #[getter]
    fn progress(&self) -> f64 {
        self.0.progress
    }
    #[getter]
    fn stage2_done(&self) -> i32 {
        self.0.stage2_done
    }
    #[getter]
    fn stage2_total(&self) -> i32 {
        self.0.stage2_total
    }
    #[getter]
    fn stage3_done(&self) -> i32 {
        self.0.stage3_done
    }
    #[getter]
    fn stage3_total(&self) -> i32 {
        self.0.stage3_total
    }
    #[getter]
    fn error(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_py(py, self.0.error.as_ref())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl From<HandEyeCalibStatus> for PyHandEyeCalibStatus {
    fn from(value: HandEyeCalibStatus) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "HandEyeCalibResult")]
#[derive(Clone)]
pub struct PyHandEyeCalibResult(HandEyeCalibResult);

#[pymethods]
impl PyHandEyeCalibResult {
    #[new]
    #[pyo3(signature = (
        status=String::new(),
        job_id=String::new(),
        summary=String::new(),
        result=None,
        reprojection_error_px=0.0,
        progress=0.0,
        stage2_done=0,
        stage2_total=0,
        stage3_done=0,
        stage3_total=0,
        error=None,
    ))]
    #[expect(clippy::too_many_arguments)]
    fn new(
        status: String,
        job_id: String,
        summary: String,
        result: Option<&Bound<'_, PyAny>>,
        reprojection_error_px: f64,
        progress: f64,
        stage2_done: i32,
        stage2_total: i32,
        stage3_done: i32,
        stage3_total: i32,
        error: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        Ok(Self(HandEyeCalibResult {
            status,
            job_id,
            summary,
            result: optional_json(result)?,
            reprojection_error_px,
            progress,
            stage2_done,
            stage2_total,
            stage3_done,
            stage3_total,
            error: optional_json(error)?,
        }))
    }

    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        serde_json::from_str(json).map(Self).map_err(json_err)
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(json_err)
    }

    #[getter]
    fn status(&self) -> String {
        self.0.status.clone()
    }
    #[getter]
    fn job_id(&self) -> String {
        self.0.job_id.clone()
    }
    #[getter]
    fn summary(&self) -> String {
        self.0.summary.clone()
    }
    #[getter]
    fn result(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_py(py, self.0.result.as_ref())
    }
    #[getter]
    fn reprojection_error_px(&self) -> f64 {
        self.0.reprojection_error_px
    }
    #[getter]
    fn progress(&self) -> f64 {
        self.0.progress
    }
    #[getter]
    fn stage2_done(&self) -> i32 {
        self.0.stage2_done
    }
    #[getter]
    fn stage2_total(&self) -> i32 {
        self.0.stage2_total
    }
    #[getter]
    fn stage3_done(&self) -> i32 {
        self.0.stage3_done
    }
    #[getter]
    fn stage3_total(&self) -> i32 {
        self.0.stage3_total
    }
    #[getter]
    fn error(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_py(py, self.0.error.as_ref())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl From<HandEyeCalibResult> for PyHandEyeCalibResult {
    fn from(value: HandEyeCalibResult) -> Self {
        Self(value)
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "HandEyeCalibApplyResult")]
#[derive(Clone)]
pub struct PyHandEyeCalibApplyResult(HandEyeCalibApplyResult);

#[pymethods]
impl PyHandEyeCalibApplyResult {
    #[new]
    #[pyo3(signature = (status=String::new(), applied_path=String::new(), error=None))]
    fn new(
        status: String,
        applied_path: String,
        error: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        Ok(Self(HandEyeCalibApplyResult {
            status,
            applied_path,
            error: optional_json(error)?,
        }))
    }

    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        serde_json::from_str(json).map(Self).map_err(json_err)
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(json_err)
    }

    #[getter]
    fn status(&self) -> String {
        self.0.status.clone()
    }
    #[getter]
    fn applied_path(&self) -> String {
        self.0.applied_path.clone()
    }
    #[getter]
    fn error(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        optional_py(py, self.0.error.as_ref())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl From<HandEyeCalibApplyResult> for PyHandEyeCalibApplyResult {
    fn from(value: HandEyeCalibApplyResult) -> Self {
        Self(value)
    }
}

#[pyclass(
    module = "booster_sdk_bindings",
    name = "HandEyeCalibClient",
    unsendable
)]
pub struct PyHandEyeCalibClient {
    client: Arc<HandEyeCalibClient>,
}

#[pymethods]
impl PyHandEyeCalibClient {
    #[new]
    #[pyo3(signature = (startup_wait_sec=None, *, robot_name=None, network=None, domain_id=None))]
    fn new(
        startup_wait_sec: Option<f64>,
        robot_name: Option<String>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let options = rpc_options(startup_wait_sec, robot_name, network, domain_id)?;
        let client = HandEyeCalibClient::with_options(options).map_err(to_py_err)?;
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

    /// Start calibration; `param` defaults to `StartHandEyeCalibParameter()`.
    #[pyo3(signature = (param=None))]
    fn start_calibration(
        &self,
        py: Python<'_>,
        param: Option<PyStartHandEyeCalibParameter>,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.unwrap_or_default().to_rust();
        wait_for_future(py, async move { client.start_calibration(&param).await })
            .map_err(to_py_err)
    }

    fn stop_calibration(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_calibration().await }).map_err(to_py_err)
    }

    fn get_status(&self, py: Python<'_>) -> PyResult<PyHandEyeCalibStatus> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_status().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn get_result(&self, py: Python<'_>) -> PyResult<PyHandEyeCalibResult> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.get_result().await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn apply_result(&self, py: Python<'_>) -> PyResult<PyHandEyeCalibApplyResult> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.apply_result().await })
            .map(Into::into)
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyHandEyeCalibApiId>()?;
    m.add_class::<PyStartHandEyeCalibParameter>()?;
    m.add_class::<PyHandEyeCalibStatus>()?;
    m.add_class::<PyHandEyeCalibResult>()?;
    m.add_class::<PyHandEyeCalibApplyResult>()?;
    m.add_class::<PyHandEyeCalibClient>()?;
    Ok(())
}
