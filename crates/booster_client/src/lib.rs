//! High-level robot control client for the Booster Robotics SDK.
//!
//! This crate provides ergonomic async APIs for controlling the B1 robot.

pub mod commands;
pub mod loco_client;

pub use commands::*;
pub use loco_client::*;
