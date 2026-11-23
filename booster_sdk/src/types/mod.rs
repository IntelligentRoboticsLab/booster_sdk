//! Core domain types shared across the Booster Robotics SDK.

mod b1;
pub mod device;
mod error;
mod robot;

pub use b1::*;
pub use error::*;
pub use robot::*;
