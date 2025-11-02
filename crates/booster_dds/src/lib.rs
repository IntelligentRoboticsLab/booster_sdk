//! Zenoh communication layer for the Booster Robotics SDK.
//!
//! This crate exposes the asynchronous RPC client used by higher-level
//! components to interact with Booster services.

pub mod rpc;

pub use rpc::*;
