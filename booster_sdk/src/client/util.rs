//! Internal client utilities shared across high-level client modules.

use serde::Serialize;

use crate::types::Result;

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub(crate) struct EmptyResponse {}

pub(crate) fn serialize_param<T: Serialize>(value: &T) -> Result<String> {
    Ok(serde_json::to_string(value)?)
}
