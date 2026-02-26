mod client;
mod runtime;

use std::sync::OnceLock;

use booster_sdk::{client::ai::BOOSTER_ROBOT_USER_ID, types::BoosterError};
use pyo3::{exceptions::PyException, prelude::*, types::PyModule};
use tracing_subscriber::{EnvFilter, fmt};

pyo3::create_exception!(booster_sdk_bindings, BoosterSdkError, PyException);

pub(crate) fn to_py_err(err: BoosterError) -> PyErr {
    BoosterSdkError::new_err(err.to_string())
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

        let env_filter = std::env::var("RUST_LOG")
            .ok()
            .and_then(|value| EnvFilter::try_new(value).ok())
            .unwrap_or_else(|| EnvFilter::new("booster_sdk::rpc=debug"));

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

    m.add("BoosterSdkError", py.get_type::<BoosterSdkError>())?;
    m.add("BOOSTER_ROBOT_USER_ID", BOOSTER_ROBOT_USER_ID)?;

    client::register_classes(m)?;

    Ok(())
}
