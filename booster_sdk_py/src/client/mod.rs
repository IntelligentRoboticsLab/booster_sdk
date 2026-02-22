mod ai;
mod booster;
mod light_control;
mod lui;
mod vision;
mod x5_camera;

use pyo3::{Bound, PyResult, types::PyModule};

pub(crate) fn register_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    booster::register(m)?;
    ai::register(m)?;
    lui::register(m)?;
    light_control::register(m)?;
    vision::register(m)?;
    x5_camera::register(m)?;
    Ok(())
}
