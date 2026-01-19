//! Subscribe to low-level state feedback over DDS.
//!
//! This mirrors the C++ `low_level_subscriber.cpp` example. By default it
//! listens on domain `0`; override with the `BOOSTER_DOMAIN_ID` environment
//! variable.

use booster_sdk::types::LowState;
use rustdds::{
    DomainParticipant, QosPolicyBuilder, TopicKind, policy::History, policy::Reliability,
};

const DOMAIN_ENV: &str = "BOOSTER_DOMAIN_ID";
const TOPIC_NAME: &str = "rt/low_state";
const TYPE_NAME: &str = "booster::msg::LowState";

fn resolve_domain_id() -> u16 {
    std::env::var(DOMAIN_ENV)
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(0)
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

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let domain_id = resolve_domain_id();
    tracing::info!("Subscribing to '{TOPIC_NAME}' (domain: {domain_id})");

    let participant = DomainParticipant::new(domain_id)?;
    let qos = QosPolicyBuilder::new()
        .reliability(Reliability::BestEffort)
        .history(History::KeepLast { depth: 1 })
        .build();
    let subscriber = participant.create_subscriber(&qos)?;
    let topic = participant.create_topic(
        TOPIC_NAME.to_string(),
        TYPE_NAME.to_string(),
        &qos,
        TopicKind::NoKey,
    )?;
    let mut reader = subscriber.create_datareader_no_key_cdr::<LowState>(&topic, Some(qos))?;

    tracing::info!("Waiting for LowState samples...");

    loop {
        match reader.take_next_sample() {
            Ok(Some(sample)) => log_low_state(sample.value()),
            Ok(None) => std::thread::sleep(std::time::Duration::from_millis(5)),
            Err(err) => tracing::warn!("Failed to read LowState sample: {err}"),
        }
    }
}
