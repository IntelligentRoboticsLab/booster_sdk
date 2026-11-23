//! Low-level state subscriber (port of `low_level_subscriber.cpp`).
//!
//! Subscribes to `rt/low_state` and prints the IMU and motor states.
//!
//! Run with:
//! `cargo run -p low_level [network]`
//!
//! `network` is optional: an interface name (`eth0`) or IP address
//! (`192.168.10.10`) to use instead of loopback, e.g. when running off-robot.
//! `BOOSTER_DOMAIN_ID` and `BOOSTER_DDS_NETWORK` are honoured as well.

use booster_sdk::dds::{DdsConfig, DdsNode, LowState, low_state_topic};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = DdsConfig::from_env();
    if let Some(network) = std::env::args().nth(1) {
        config = config.with_interface(network);
    }
    println!(
        "Listening on rt/low_state (domain {}, network {:?})",
        config.domain_id, config.network
    );

    let node = DdsNode::new(config)?;
    let mut subscription = node.subscribe::<LowState>(&low_state_topic(), 4)?;

    let mut count: u64 = 0;
    while let Some(state) = subscription.blocking_recv() {
        count += 1;
        // low_state is published at a high rate; print every 100th sample.
        if count % 100 != 1 {
            continue;
        }
        let imu = &state.imu_state;
        println!("Received message #{count}:");
        println!(
            "  serial motor count: {}, parallel motor count: {}",
            state.motor_state_serial.len(),
            state.motor_state_parallel.len()
        );
        println!(
            "  imu rpy: {:?}, gyro: {:?}, acc: {:?}",
            imu.rpy, imu.gyro, imu.acc
        );
        for (i, motor) in state.motor_state_serial.iter().enumerate() {
            println!(
                "  serial motor {i}: q={:.3} dq={:.3} ddq={:.3} tau_est={:.3}",
                motor.q, motor.dq, motor.ddq, motor.tau_est
            );
        }
        for (i, motor) in state.motor_state_parallel.iter().enumerate() {
            println!(
                "  parallel motor {i}: q={:.3} dq={:.3} ddq={:.3} tau_est={:.3}",
                motor.q, motor.dq, motor.ddq, motor.tau_est
            );
        }
    }

    Ok(())
}
