mod ai;
mod audio;
mod booster;
mod camera;
mod handeye_calib;
mod light_control;
mod lui;
mod vision;
mod x5_camera;

use pyo3::{Bound, PyResult, types::PyModule};

pub(crate) fn register_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    booster::register(m)?;
    camera::register(m)?;
    handeye_calib::register(m)?;
    ai::register(m)?;
    audio::register(m)?;
    lui::register(m)?;
    light_control::register(m)?;
    vision::register(m)?;
    x5_camera::register(m)?;
    Ok(())
}
