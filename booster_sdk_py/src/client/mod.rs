mod ai;
mod audio;
mod booster;
mod camera;
mod controllers;
mod device;
mod handeye_calib;
mod light_control;
mod low_level;
mod lui;
mod operation;
mod robot_state;
mod vision;
mod x5_camera;

use pyo3::{Bound, PyResult, types::PyModule};

pub(crate) fn register_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    booster::register(m)?;
    robot_state::register(m)?;
    low_level::register(m)?;
    operation::register(m)?;
    controllers::register(m)?;
    camera::register(m)?;
    device::register(m)?;
    handeye_calib::register(m)?;
    ai::register(m)?;
    audio::register(m)?;
    lui::register(m)?;
    light_control::register(m)?;
    vision::register(m)?;
    x5_camera::register(m)?;
    Ok(())
}
