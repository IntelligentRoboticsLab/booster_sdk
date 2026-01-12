//! High-level look around example
//!
//! This example demonstrates basic look around behavior using the `B1LocoClient`.
//!
//! Run with: cargo run --example `look_around`

use booster_sdk::{client::B1LocoClient, types::RobotMode};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    tracing::info!("Starting look around example");

    // Create client with 2-second timeout
    let client = B1LocoClient::with_timeout(Duration::from_millis(2000)).await?;

    // Get current mode
    match client.get_mode().await {
        Ok(mode) => tracing::info!("Current mode: {:?}", mode),
        Err(e) => tracing::error!("Failed to get mode: {}", e),
    }

    // Rotate head
    tracing::info!("Looking around");

    client.change_mode(RobotMode::Damping).await?;

    // scan from left to right with a sine wave
    let steps = 4000;
    for i in 0..=steps {
        let angle = (i as f32 / steps as f32) * std::f32::consts::TAU;
        let head_yaw = (6.0 * angle).cos() * 1.0;
        let head_pitch = (6.0 * angle).sin() * 0.4;
        client.rotate_head(head_pitch, head_yaw).await?;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    // set head back to center
    tracing::info!("Centering head");
    client.rotate_head(0.0, 0.0).await?;

    Ok(())
}
