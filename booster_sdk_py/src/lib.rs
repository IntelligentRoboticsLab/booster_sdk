mod client;
mod runtime;

use booster_sdk::{client::ai::BOOSTER_ROBOT_USER_ID, types::BoosterError};
use pyo3::{exceptions::PyException, prelude::*, types::PyModule};

pyo3::create_exception!(booster_sdk_bindings, BoosterSdkError, PyException);

pub(crate) fn to_py_err(err: BoosterError) -> PyErr {
    BoosterSdkError::new_err(err.to_string())
}

#[pymodule]
fn booster_sdk_bindings(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("BoosterSdkError", py.get_type::<BoosterSdkError>())?;
    m.add("BOOSTER_ROBOT_USER_ID", BOOSTER_ROBOT_USER_ID)?;

    client::register_classes(m)?;

    Ok(())
}
