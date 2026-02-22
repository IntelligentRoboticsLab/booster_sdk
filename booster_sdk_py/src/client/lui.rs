use std::sync::Arc;

use booster_sdk::client::ai::LuiClient;
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{PyLuiTtsConfig, PyLuiTtsParameter, runtime::wait_for_future, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "LuiClient", unsendable)]
pub struct PyLuiClient {
    client: Arc<LuiClient>,
}

#[pymethods]
impl PyLuiClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            client: Arc::new(LuiClient::new().map_err(to_py_err)?),
        })
    }

    fn start_asr(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.start_asr().await }).map_err(to_py_err)
    }

    fn stop_asr(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_asr().await }).map_err(to_py_err)
    }

    fn start_tts(&self, py: Python<'_>, config: PyLuiTtsConfig) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let config = config.into();
        wait_for_future(py, async move { client.start_tts(&config).await }).map_err(to_py_err)
    }

    fn stop_tts(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_tts().await }).map_err(to_py_err)
    }

    fn send_tts_text(&self, py: Python<'_>, param: PyLuiTtsParameter) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.into();
        wait_for_future(py, async move { client.send_tts_text(&param).await }).map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLuiClient>()?;
    Ok(())
}
