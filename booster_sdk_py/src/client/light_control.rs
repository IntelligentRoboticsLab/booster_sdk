use std::sync::Arc;

use booster_sdk::client::light_control::{
    LightApiId, LightControlClient, SetLedLightColorParameter, SetLedLightColorsParameter,
};
use pyo3::{Bound, exceptions::PyValueError, prelude::*, types::PyModule};

use crate::{
    client::device::json_err,
    macros::{py_int_enum, py_param_class},
    options::rpc_options,
    runtime::wait_for_future,
    to_py_err,
};

py_int_enum!(PyLightApiId, "LightApiId", LightApiId, i32, {
    SET_LED_LIGHT_COLOR => SetLedLightColor,
    STOP_LED_LIGHT_CONTROL => StopLedLightControl,
    SET_LED_LIGHT_COLORS => SetLedLightColors,
});

py_param_class!(
    PySetLedLightColorParameter,
    "SetLEDLightColorParameter",
    SetLedLightColorParameter,
    { r: u8 = 0, g: u8 = 0, b: u8 = 0 },
    {
        /// Parse `"#RRGGBB"`; raises `ValueError` on malformed input.
        #[staticmethod]
        fn from_hex(color: &str) -> PyResult<Self> {
            SetLedLightColorParameter::from_hex(color)
                .map(Self::from_rust)
                .ok_or_else(|| PyValueError::new_err(format!("invalid hex color {color:?}")))
        }

        /// Parse `"#RRGGBB"`, falling back to black like the C++ SDK.
        #[staticmethod]
        fn from_hex_or_default(color: &str) -> Self {
            Self::from_rust(SetLedLightColorParameter::from_hex_or_default(color))
        }
    }
);

/// Color argument: a `SetLEDLightColorParameter`, an `(r, g, b)` tuple or a
/// `"#RRGGBB"` string.
fn extract_color(value: &Bound<'_, PyAny>) -> PyResult<SetLedLightColorParameter> {
    if let Ok(param) = value.extract::<PySetLedLightColorParameter>() {
        return Ok(param.to_rust());
    }
    if let Ok((r, g, b)) = value.extract::<(u8, u8, u8)>() {
        return Ok(SetLedLightColorParameter { r, g, b });
    }
    if let Ok(hex) = value.extract::<String>() {
        return SetLedLightColorParameter::from_hex(&hex)
            .ok_or_else(|| PyValueError::new_err(format!("invalid hex color {hex:?}")));
    }
    Err(PyValueError::new_err(
        "color must be a SetLEDLightColorParameter, an (r, g, b) tuple or a '#RRGGBB' string",
    ))
}

/// Multi-pixel LED payload (SDK 1.7+).
#[pyclass(
    module = "booster_sdk_bindings",
    name = "SetLEDLightColorsParameter",
    get_all,
    set_all
)]
#[derive(Clone, Default)]
pub struct PySetLedLightColorsParameter {
    colors: Vec<PySetLedLightColorParameter>,
}

impl PySetLedLightColorsParameter {
    fn to_rust(&self) -> SetLedLightColorsParameter {
        SetLedLightColorsParameter {
            colors: self
                .colors
                .iter()
                .map(PySetLedLightColorParameter::to_rust)
                .collect(),
        }
    }
}

#[pymethods]
impl PySetLedLightColorsParameter {
    /// `colors` items may be `SetLEDLightColorParameter`, `(r, g, b)` or
    /// `"#RRGGBB"`.
    #[new]
    #[pyo3(signature = (colors=Vec::new()))]
    fn new(colors: Vec<Bound<'_, PyAny>>) -> PyResult<Self> {
        Ok(Self {
            colors: colors
                .iter()
                .map(|color| extract_color(color).map(PySetLedLightColorParameter::from_rust))
                .collect::<PyResult<_>>()?,
        })
    }

    /// Serialize to the JSON body sent to the service.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.to_rust()).map_err(json_err)
    }

    /// Parse from a JSON body.
    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        let value: SetLedLightColorsParameter = serde_json::from_str(json).map_err(json_err)?;
        Ok(Self {
            colors: value
                .colors
                .into_iter()
                .map(PySetLedLightColorParameter::from_rust)
                .collect(),
        })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.to_rust())
    }
}

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
    #[pyo3(signature = (startup_wait_sec=None, *, robot_name=None, network=None, domain_id=None))]
    fn new(
        startup_wait_sec: Option<f64>,
        robot_name: Option<String>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let options = rpc_options(startup_wait_sec, robot_name, network, domain_id)?;
        let client = LightControlClient::with_options(options).map_err(to_py_err)?;
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

    fn set_led_light_color(&self, py: Python<'_>, r: u8, g: u8, b: u8) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.set_led_light_color(r, g, b).await })
            .map_err(to_py_err)
    }

    /// Set the color from a `SetLEDLightColorParameter`, `(r, g, b)` tuple or
    /// `"#RRGGBB"` string.
    fn set_led_light_color_param(&self, py: Python<'_>, color: &Bound<'_, PyAny>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = extract_color(color)?;
        wait_for_future(
            py,
            async move { client.set_led_light_color_param(&param).await },
        )
        .map_err(to_py_err)
    }

    /// Set per-pixel colors. Accepts a `SetLEDLightColorsParameter` or an
    /// iterable of `SetLEDLightColorParameter` / `(r, g, b)` / `"#RRGGBB"`.
    fn set_led_light_colors(&self, py: Python<'_>, colors: &Bound<'_, PyAny>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let colors: Vec<SetLedLightColorParameter> =
            if let Ok(param) = colors.extract::<PySetLedLightColorsParameter>() {
                param.to_rust().colors
            } else {
                colors
                    .try_iter()?
                    .map(|item| extract_color(&item?))
                    .collect::<PyResult<_>>()?
            };
        wait_for_future(
            py,
            async move { client.set_led_light_colors(&colors).await },
        )
        .map_err(to_py_err)
    }

    fn stop_led_light_control(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_led_light_control().await }).map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLightApiId>()?;
    m.add_class::<PySetLedLightColorParameter>()?;
    m.add_class::<PySetLedLightColorsParameter>()?;
    m.add_class::<PyLightControlClient>()?;
    Ok(())
}
