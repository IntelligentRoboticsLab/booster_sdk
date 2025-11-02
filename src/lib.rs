//! Booster Robotics SDK for Rust.
//!
//! This crate provides high-level, asynchronous control APIs for Booster robots,
//! alongside the transport layer abstractions and shared domain types that power
//! those APIs.
//!
//! ## Modules
//! - [`client`]: robot control interfaces such as [`client::B1LocoClient`]
//!   for locomotion, manipulation, and gesture commands.
//! - [`dds`]: Zenoh-based RPC transport primitives for communicating with Booster
//!   services.
//! - [`types`]: core data structures, error types, and helper utilities shared across
//!   the SDK.
//!
//! ## Getting Started
//!
//! Most applications will interact with the [`client`] module:
//!
//! ```no_run
//! use booster_sdk::client::B1LocoClient;
//! use booster_sdk::types::RobotMode;
//!
//! # async fn demo() -> booster_sdk::types::Result<()> {
//! let client = B1LocoClient::new().await?;
//! client.change_mode(RobotMode::Walking).await?;
//! client.move_robot(0.5, 0.0, 0.0).await?;
//! # Ok(())
//! # }
//! ```
//!
//! For advanced scenarios you can work directly with the [`dds`] module or compose
//! your own data pipelines using the types re-exported from [`types`].

pub mod client;
pub mod dds;
pub mod types;
