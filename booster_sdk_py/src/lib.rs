mod client;
mod errors;
mod macros;
mod options;
mod runtime;
mod subscription;

use std::sync::OnceLock;
use std::time::Duration;

use booster_sdk::client::ai::BOOSTER_ROBOT_USER_ID;
use pyo3::{exceptions::PyValueError, prelude::*, types::PyModule};
use tracing_subscriber::{EnvFilter, fmt};

pub(crate) use errors::to_py_err;

pub(crate) fn startup_wait_from_seconds(
    startup_wait_sec: Option<f64>,
) -> PyResult<Option<Duration>> {
    let Some(seconds) = startup_wait_sec else {
        return Ok(None);
    };

    if !seconds.is_finite() {
        return Err(PyValueError::new_err("startup_wait_sec must be finite"));
    }
    if seconds < 0.0 {
        return Err(PyValueError::new_err("startup_wait_sec must be >= 0"));
    }

    Ok(Some(Duration::from_secs_f64(seconds)))
}

pub(crate) fn json_value_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<Py<PyAny>> {
    PyModule::import(py, "json")?
        .call_method1("loads", (value.to_string(),))
        .map(Bound::unbind)
}

/// Convert a JSON-compatible Python object to a `serde_json::Value`
/// (through `json.dumps`).
pub(crate) fn py_to_json_value(value: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    let text: String = PyModule::import(value.py(), "json")?
        .call_method1("dumps", (value,))?
        .extract()?;
    serde_json::from_str(&text).map_err(|err| PyValueError::new_err(err.to_string()))
}

fn rpc_debug_enabled() -> bool {
    std::env::var("BOOSTER_RPC_DEBUG")
        .map(|value| {
            let value = value.trim();
            value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case("yes")
                || value.eq_ignore_ascii_case("on")
        })
        .unwrap_or(false)
}

fn init_tracing_for_python() {
    static INIT: OnceLock<()> = OnceLock::new();
    INIT.get_or_init(|| {
        if !rpc_debug_enabled() {
            return;
        }

        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("booster_sdk::rpc=debug"));

        let _ = fmt()
            .with_env_filter(env_filter)
            .with_target(true)
            .with_thread_ids(true)
            .with_writer(std::io::stderr)
            .try_init();
    });
}

#[pymodule]
fn booster_sdk_bindings(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    init_tracing_for_python();

    errors::register(py, m)?;
    m.add("BOOSTER_ROBOT_USER_ID", BOOSTER_ROBOT_USER_ID)?;

    client::register_classes(m)?;
    subscription::register(m)?;

    Ok(())
}
