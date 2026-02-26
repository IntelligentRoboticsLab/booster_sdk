use std::sync::Arc;

use booster_sdk::client::light_control::LightControlClient;
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{runtime::wait_for_future, startup_wait_from_seconds, to_py_err};

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
    #[pyo3(signature = (startup_wait_sec=None))]
    fn new(startup_wait_sec: Option<f64>) -> PyResult<Self> {
        let startup_wait = startup_wait_from_seconds(startup_wait_sec)?;
        let client = match startup_wait {
            Some(wait) => LightControlClient::with_startup_wait(wait),
            None => LightControlClient::new(),
        }
        .map_err(to_py_err)?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    fn set_led_light_color(&self, py: Python<'_>, r: u8, g: u8, b: u8) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.set_led_light_color(r, g, b).await })
            .map_err(to_py_err)
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
