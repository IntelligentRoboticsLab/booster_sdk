//! Construction of [`RpcClientOptions`] from the keyword arguments shared by
//! the high-level client constructors.

use booster_sdk::dds::{DdsNetwork, RpcClientOptions};
use pyo3::{exceptions::PyTypeError, prelude::*, types::PyString};

use crate::startup_wait_from_seconds;

/// Parse the `network` keyword: `"localhost"`, `"all"`, a comma-separated
/// string, or a list of interface names / IP addresses (see
/// `DdsNetwork::from_str`).
pub(crate) fn parse_network(value: &Bound<'_, PyAny>) -> PyResult<DdsNetwork> {
    let spec = if let Ok(text) = value.cast::<PyString>() {
        text.to_str()?.to_owned()
    } else if let Ok(entries) = value.extract::<Vec<String>>() {
        if entries.is_empty() {
            return Err(PyTypeError::new_err(
                "network list must name at least one interface or address",
            ));
        }
        entries.join(",")
    } else {
        return Err(PyTypeError::new_err(
            "network must be 'localhost', 'all', or a str / list[str] of interface names or IP addresses",
        ));
    };
    // `DdsNetwork::from_str` is infallible.
    Ok(spec.parse().unwrap_or_default())
}

/// Build client options. The base comes from the environment
/// (`BOOSTER_DOMAIN_ID`, `BOOSTER_DDS_NETWORK`; loopback / domain 0 when
/// unset); explicit keyword arguments override it.
pub(crate) fn rpc_options(
    startup_wait_sec: Option<f64>,
    robot_name: Option<String>,
    network: Option<&Bound<'_, PyAny>>,
    domain_id: Option<u16>,
) -> PyResult<RpcClientOptions> {
    let mut options = RpcClientOptions::from_env();
    if let Some(wait) = startup_wait_from_seconds(startup_wait_sec)? {
        options = options.with_startup_wait(wait);
    }
    if let Some(robot_name) = robot_name {
        options = options.with_robot_name(robot_name);
    }
    if let Some(network) = network {
        options = options.with_network(parse_network(network)?);
    }
    if let Some(domain_id) = domain_id {
        options = options.with_domain_id(domain_id);
    }
    Ok(options)
}
