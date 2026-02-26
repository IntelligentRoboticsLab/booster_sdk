//! Quick X5 camera RPC connectivity check.
//!
//! Run with:
//! `cargo run -p booster_sdk --example x5_camera_rpc_check`
//!
//! Optional env vars:
//! - `BOOSTER_DOMAIN_ID` (default: `0`)
//! - `BOOSTER_TIMEOUT_MS` (default: `3000`)
//! - `BOOSTER_STARTUP_WAIT_MS` (default: `7000`)
//! - `BOOSTER_RETRIES` (default: `3`)
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
    let startup_wait_ms = std::env::var("BOOSTER_STARTUP_WAIT_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(7000);
    let retries = std::env::var("BOOSTER_RETRIES")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(3);

    println!(
        "X5 RPC check: domain_id={}, timeout_ms={}, startup_wait_ms={}, retries={}, topic={}",
        domain_id, timeout_ms, startup_wait_ms, retries, X5_CAMERA_CONTROL_API_TOPIC
    );

    let client = X5CameraClient::with_options(RpcClientOptions {
        domain_id,
        default_timeout: Duration::from_millis(timeout_ms),
        service_topic: X5_CAMERA_CONTROL_API_TOPIC.to_owned(),
    })?;

    // Mimic "waiting for service to become available" before first call.
    tokio::time::sleep(Duration::from_millis(startup_wait_ms)).await;

    let total_started = Instant::now();
    let mut last_err = None;
    for attempt in 1..=retries {
        let started = Instant::now();
        match client.get_status().await {
            Ok(resp) => {
                let elapsed = started.elapsed();
                let total_elapsed = total_started.elapsed();
                println!(
                    "OK: attempt={} status={} status_enum={:?} elapsed_ms={} total_elapsed_ms={}",
                    attempt,
                    resp.status,
                    resp.status_enum(),
                    elapsed.as_millis(),
                    total_elapsed.as_millis()
                );
                return Ok(());
            }
            Err(err) => {
                let elapsed = started.elapsed();
                eprintln!("attempt {} failed: {} (elapsed_ms={})", attempt, err, elapsed.as_millis());
                last_err = Some(err.to_string());
                if attempt < retries {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
    }

    eprintln!(
        "ERR: {}",
        last_err.unwrap_or_else(|| "unknown error".to_string())
    );
    eprintln!("total_elapsed_ms={}", total_started.elapsed().as_millis());
    std::process::exit(2);
}
