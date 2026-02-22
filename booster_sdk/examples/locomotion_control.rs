//! High-level locomotion control example
//!
//! This example demonstrates basic locomotion control using the `BoosterClient`.
//!
//! Run with: cargo run --example `locomotion_control`

use booster_sdk::client::loco::BoosterClient;
use booster_sdk::types::RobotMode;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    tracing::info!("Starting locomotion control example");

    // Create client
    let client = BoosterClient::new()?;

    // Change to walking mode
    tracing::info!("Changing to walking mode...");
    client.change_mode(RobotMode::Walking).await?;
    tracing::info!("Mode changed successfully");

    // Wait a moment for mode transition
    tokio::time::sleep(Duration::from_secs(2)).await;

    tracing::info!("Moving forward at 0.5 m/s for 3 seconds");
    client.move_robot(0.5, 0.0, 0.0).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    tracing::info!("Stopping");
    client.move_robot(0.0, 0.0, 0.0).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    tracing::info!("Turning left for 2 seconds");
    client.move_robot(0.0, 0.0, 0.6).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    tracing::info!("Stopping");
    client.move_robot(0.0, 0.0, 0.0).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    tracing::info!("Example completed successfully");

    Ok(())
}
