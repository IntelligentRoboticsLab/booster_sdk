use std::sync::Arc;

use booster_sdk::client::handeye_calib::{
    HandEyeCalibApplyResult, HandEyeCalibClient, HandEyeCalibResult, HandEyeCalibStatus,
    StartHandEyeCalibParameter,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{json_value_to_py, runtime::wait_for_future, startup_wait_from_seconds, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "StartHandEyeCalibParameter")]
#[derive(Clone)]
pub struct PyStartHandEyeCalibParameter(StartHandEyeCalibParameter);

#[pymethods]
impl PyStartHandEyeCalibParameter {
    #[new]
    #[pyo3(signature = (publish_feedback=true, square_size_m=None))]
    fn new(publish_feedback: bool, square_size_m: Option<f64>) -> Self {
        Self(StartHandEyeCalibParameter {
            publish_feedback,
            square_size_m,
        })
    }

    #[getter]
    fn publish_feedback(&self) -> bool {
        self.0.publish_feedback
    }

    #[getter]
    fn square_size_m(&self) -> Option<f64> {
        self.0.square_size_m
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "HandEyeCalibStatus")]
#[derive(Clone)]
pub struct PyHandEyeCalibStatus(HandEyeCalibStatus);

#[pymethods]
impl PyHandEyeCalibStatus {
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
        self.0
            .error
            .as_ref()
            .map(|value| json_value_to_py(py, value))
            .transpose()
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
        self.0
            .result
            .as_ref()
            .map(|value| json_value_to_py(py, value))
            .transpose()
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
        self.0
            .error
            .as_ref()
            .map(|value| json_value_to_py(py, value))
            .transpose()
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
        self.0
            .error
            .as_ref()
            .map(|value| json_value_to_py(py, value))
            .transpose()
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
    #[pyo3(signature = (startup_wait_sec=None))]
    fn new(startup_wait_sec: Option<f64>) -> PyResult<Self> {
        let startup_wait = startup_wait_from_seconds(startup_wait_sec)?;
        let client = match startup_wait {
            Some(wait) => HandEyeCalibClient::with_startup_wait(wait),
            None => HandEyeCalibClient::new(),
        }
        .map_err(to_py_err)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    fn start_calibration(
        &self,
        py: Python<'_>,
        param: PyStartHandEyeCalibParameter,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.0;
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
    m.add_class::<PyStartHandEyeCalibParameter>()?;
    m.add_class::<PyHandEyeCalibStatus>()?;
    m.add_class::<PyHandEyeCalibResult>()?;
    m.add_class::<PyHandEyeCalibApplyResult>()?;
    m.add_class::<PyHandEyeCalibClient>()?;
    Ok(())
}
