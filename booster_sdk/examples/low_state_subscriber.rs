//! Subscribe to low-level state feedback over Zenoh.
//!
//! This mirrors the C++ `low_level_subscriber.cpp` example and expects the
//! `zenoh-bridge-dds` process to bridge FastDDS topics into Zenoh. By default it
//! listens on domain `0`; override with the `BOOSTER_DOMAIN_ID` environment
//! variable.

use booster_sdk::types::LowState;
use zenoh::handlers::FifoChannel;

const DOMAIN_ENV: &str = "BOOSTER_DOMAIN_ID";
const TOPIC_SUFFIX: &str = "rt/low_state";

fn resolve_domain_id() -> u16 {
    std::env::var(DOMAIN_ENV)
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(0)
}

fn format_key_expr(domain_id: u16) -> String {
    format!("domain{domain_id}/{TOPIC_SUFFIX}")
}

fn log_low_state(state: &LowState) {
    println!("Received low_state sample");
    println!("  serial motor count: {}", state.motor_state_serial.len());
    println!(
        "  parallel motor count: {}",
        state.motor_state_parallel.len()
    );

    let imu = state.imu_state;
    println!(
        "  imu: rpy=({:.3}, {:.3}, {:.3}), gyro=({:.3}, {:.3}, {:.3}), acc=({:.3}, {:.3}, {:.3})",
        imu.rpy[0],
        imu.rpy[1],
        imu.rpy[2],
        imu.gyro[0],
        imu.gyro[1],
        imu.gyro[2],
        imu.acc[0],
        imu.acc[1],
        imu.acc[2],
    );

    for (index, motor) in state.motor_state_serial.iter().enumerate() {
        println!(
            "  serial motor {index}: dq={:.4}, ddq={:.4}, tau_est={:.4}, temp={}C, lost={}",
            motor.dq, motor.ddq, motor.tau_est, motor.temperature, motor.lost
        );
    }

    for (index, motor) in state.motor_state_parallel.iter().enumerate() {
        println!(
            "  parallel motor {index}: dq={:.4}, ddq={:.4}, tau_est={:.4}, temp={}C, lost={}",
            motor.dq, motor.ddq, motor.tau_est, motor.temperature, motor.lost
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let domain_id = resolve_domain_id();
    let key_expr = format_key_expr(domain_id);
    tracing::info!("Subscribing to '{key_expr}' (domain: {domain_id})");
    tracing::info!("Make sure zenoh-bridge-dds is running to bridge DDS topics");

    let session = zenoh::open(zenoh::Config::default()).await?;
    let subscriber = session
        .declare_subscriber(&key_expr)
        .with(FifoChannel::default())
        .await?;

    tracing::info!("Waiting for LowState samples...");

    loop {
        let sample = subscriber.recv_async().await?;
        let payload = sample.payload().to_bytes();

        match LowState::from_cdr_le(payload.as_ref()) {
            Ok(state) => log_low_state(&state),
            Err(err) => tracing::warn!("Failed to decode LowState sample: {err}"),
        }
    }
}
