//! Python exception hierarchy and SDK error conversion.
//!
//! ```text
//! BoosterSdkError                 (every SDK error; `.status_code` is None)
//! ├── DdsError                    DDS transport failures
//! ├── ValidationError             client-side argument validation
//! └── RpcError                    RPC failures; `.status_code` is an int
//!     ├── RpcTimeoutError         local or remote timeout (status 100)
//!     └── RpcStatusError          the service answered with an error status
//! ```

use booster_sdk::types::{
    BoosterError, RPC_STATUS_CODE_BAD_REQUEST, RPC_STATUS_CODE_CONFLICT,
    RPC_STATUS_CODE_INTERNAL_SERVER_ERROR, RPC_STATUS_CODE_INVALID, RPC_STATUS_CODE_LOW_BATTERY,
    RPC_STATUS_CODE_REQUEST_TOO_FREQUENT, RPC_STATUS_CODE_SERVER_REFUSED,
    RPC_STATUS_CODE_STATE_TRANSITION_FAILED, RPC_STATUS_CODE_SUCCESS, RPC_STATUS_CODE_TIMEOUT,
    RpcError as CoreRpcError,
};
use pyo3::{exceptions::PyException, prelude::*, types::PyModule};

pyo3::create_exception!(
    booster_sdk_bindings,
    BoosterSdkError,
    PyException,
    "Base class of all Booster SDK errors."
);
pyo3::create_exception!(
    booster_sdk_bindings,
    DdsError,
    BoosterSdkError,
    "DDS transport failure."
);
pyo3::create_exception!(
    booster_sdk_bindings,
    ValidationError,
    BoosterSdkError,
    "Client-side validation failure."
);
pyo3::create_exception!(
    booster_sdk_bindings,
    RpcError,
    BoosterSdkError,
    "RPC failure; `status_code` holds the C++ SDK status code."
);
pyo3::create_exception!(
    booster_sdk_bindings,
    RpcTimeoutError,
    RpcError,
    "The RPC (or endpoint discovery) timed out; `status_code` is 100."
);
pyo3::create_exception!(
    booster_sdk_bindings,
    RpcStatusError,
    RpcError,
    "The service answered with an error status (see `status_code`)."
);

/// Convert an SDK error into the matching Python exception, with
/// `status_code` set for RPC errors.
pub(crate) fn to_py_err(err: BoosterError) -> PyErr {
    let message = err.to_string();
    let (py_err, status_code) = match &err {
        BoosterError::Rpc(rpc) => {
            let code = rpc.status_code();
            let py_err = match rpc {
                CoreRpcError::Timeout { .. } | CoreRpcError::RemoteTimeout(_) => {
                    RpcTimeoutError::new_err(message)
                }
                _ => RpcStatusError::new_err(message),
            };
            (py_err, Some(code))
        }
        BoosterError::Timeout { .. } => (
            RpcTimeoutError::new_err(message),
            Some(RPC_STATUS_CODE_TIMEOUT),
        ),
        BoosterError::Dds(_) => (DdsError::new_err(message), None),
        BoosterError::Validation(_) => (ValidationError::new_err(message), None),
        _ => (BoosterSdkError::new_err(message), None),
    };

    if let Some(code) = status_code {
        Python::attach(|py| {
            // Best effort: the exception is still meaningful without it.
            let _ = py_err.value(py).setattr("status_code", code);
        });
    }
    py_err
}

pub(crate) fn register(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let base = py.get_type::<BoosterSdkError>();
    base.setattr("status_code", py.None())?;
    m.add("BoosterSdkError", base)?;
    m.add("DdsError", py.get_type::<DdsError>())?;
    m.add("ValidationError", py.get_type::<ValidationError>())?;
    m.add("RpcError", py.get_type::<RpcError>())?;
    m.add("RpcTimeoutError", py.get_type::<RpcTimeoutError>())?;
    m.add("RpcStatusError", py.get_type::<RpcStatusError>())?;

    m.add("RPC_STATUS_CODE_INVALID", RPC_STATUS_CODE_INVALID)?;
    m.add("RPC_STATUS_CODE_SUCCESS", RPC_STATUS_CODE_SUCCESS)?;
    m.add("RPC_STATUS_CODE_TIMEOUT", RPC_STATUS_CODE_TIMEOUT)?;
    m.add("RPC_STATUS_CODE_BAD_REQUEST", RPC_STATUS_CODE_BAD_REQUEST)?;
    m.add("RPC_STATUS_CODE_CONFLICT", RPC_STATUS_CODE_CONFLICT)?;
    m.add(
        "RPC_STATUS_CODE_REQUEST_TOO_FREQUENT",
        RPC_STATUS_CODE_REQUEST_TOO_FREQUENT,
    )?;
    m.add(
        "RPC_STATUS_CODE_INTERNAL_SERVER_ERROR",
        RPC_STATUS_CODE_INTERNAL_SERVER_ERROR,
    )?;
    m.add(
        "RPC_STATUS_CODE_SERVER_REFUSED",
        RPC_STATUS_CODE_SERVER_REFUSED,
    )?;
    m.add(
        "RPC_STATUS_CODE_STATE_TRANSITION_FAILED",
        RPC_STATUS_CODE_STATE_TRANSITION_FAILED,
    )?;
    m.add("RPC_STATUS_CODE_LOW_BATTERY", RPC_STATUS_CODE_LOW_BATTERY)?;
    Ok(())
}
