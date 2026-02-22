use std::sync::Arc;

use booster_sdk::client::ai::AiClient;
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{PySpeakParameter, PyStartAiChatParameter, runtime::wait_for_future, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "AiClient", unsendable)]
pub struct PyAiClient {
    client: Arc<AiClient>,
}

#[pymethods]
impl PyAiClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            client: Arc::new(AiClient::new().map_err(to_py_err)?),
        })
    }

    fn start_ai_chat(&self, py: Python<'_>, param: PyStartAiChatParameter) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.into();
        wait_for_future(py, async move { client.start_ai_chat(&param).await }).map_err(to_py_err)
    }

    fn stop_ai_chat(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_ai_chat().await }).map_err(to_py_err)
    }

    fn speak(&self, py: Python<'_>, param: PySpeakParameter) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.into();
        wait_for_future(py, async move { client.speak(&param).await }).map_err(to_py_err)
    }

    fn start_face_tracking(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.start_face_tracking().await }).map_err(to_py_err)
    }

    fn stop_face_tracking(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_face_tracking().await }).map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAiClient>()?;
    Ok(())
}
