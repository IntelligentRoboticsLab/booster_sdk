//! Quick X5 camera RPC connectivity check.
//!
//! Run with:
//! `cargo run -p booster_sdk --example x5_camera_rpc_check`
//!
//! Optional env vars:
//! - `BOOSTER_DOMAIN_ID` (default: `0`)
//! - `BOOSTER_TIMEOUT_MS` (default: `3000`)
//! - `BOOSTER_RPC_DEBUG` (`1`/`true` for verbose RPC logs)

use std::time::{Duration, Instant};

use booster_sdk::client::x5_camera::X5CameraClient;
use booster_sdk::dds::{RpcClientOptions, X5_CAMERA_CONTROL_API_TOPIC};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("error"));
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let domain_id = std::env::var("BOOSTER_DOMAIN_ID")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(0);
    let timeout_ms = std::env::var("BOOSTER_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(3000);

    println!(
        "X5 RPC check: domain_id={}, timeout_ms={}, topic={}",
        domain_id, timeout_ms, X5_CAMERA_CONTROL_API_TOPIC
    );

    let client = X5CameraClient::with_options(RpcClientOptions {
        domain_id,
        default_timeout: Duration::from_millis(timeout_ms),
        service_topic: X5_CAMERA_CONTROL_API_TOPIC.to_owned(),
    })?;

    // Give DDS discovery a moment to settle before the first RPC call.
    tokio::time::sleep(Duration::from_millis(500)).await;

    let started = Instant::now();
    match client.get_status().await {
        Ok(resp) => {
            let elapsed = started.elapsed();
            println!(
                "OK: status={} status_enum={:?} elapsed_ms={}",
                resp.status,
                resp.status_enum(),
                elapsed.as_millis()
            );
            Ok(())
        }
        Err(err) => {
            let elapsed = started.elapsed();
            eprintln!("ERR: {err}");
            eprintln!("elapsed_ms={}", elapsed.as_millis());
            std::process::exit(2);
        }
    }
}
