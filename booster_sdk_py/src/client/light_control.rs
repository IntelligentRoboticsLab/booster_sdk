use std::sync::Arc;

use booster_sdk::client::light_control::LightControlClient;
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{runtime::wait_for_future, to_py_err};

#[pyclass(
    module = "booster_sdk_bindings",
    name = "LightControlClient",
    unsendable
)]
pub struct PyLightControlClient {
    client: Arc<LightControlClient>,
}

#[pymethods]
impl PyLightControlClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            client: Arc::new(LightControlClient::new().map_err(to_py_err)?),
        })
    }

    fn set_led_light_color(&self, py: Python<'_>, r: u8, g: u8, b: u8) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.set_led_light_color(r, g, b).await })
            .map_err(to_py_err)
    }

    fn set_led_light_color_param(&self, py: Python<'_>, r: u8, g: u8, b: u8) -> PyResult<()> {
        self.set_led_light_color(py, r, g, b)
    }

    fn stop_led_light_control(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_led_light_control().await }).map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLightControlClient>()?;
    Ok(())
}
