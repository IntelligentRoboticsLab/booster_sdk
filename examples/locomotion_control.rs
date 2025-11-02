//! High-level locomotion control example
//!
//! This example demonstrates basic locomotion control using the `B1LocoClient`.
//!
//! Run with: cargo run --example `locomotion_control`

use booster_sdk::client::{B1LocoClient, commands::MoveCommand};
use booster_sdk::types::{Hand, RobotMode};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    tracing::info!("Starting locomotion control example");

    // Create client with 2-second timeout
    let client = B1LocoClient::with_timeout(Duration::from_millis(2000)).await?;

    // Get current mode
    match client.get_mode().await {
        Ok(mode) => tracing::info!("Current mode: {:?}", mode),
        Err(e) => tracing::error!("Failed to get mode: {}", e),
    }

    // Change to walking mode
    tracing::info!("Changing to walking mode...");
    client.change_mode(RobotMode::Walking).await?;
    tracing::info!("Mode changed successfully");

    // Wait a moment for mode transition
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Move forward
    tracing::info!("Moving forward at 0.3 m/s for 3 seconds");
    client.move_robot(0.3, 0.0, 0.0).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Stop
    tracing::info!("Stopping");
    client.stop().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Turn in place
    tracing::info!("Turning left for 2 seconds");
    let turn_cmd = MoveCommand::turn(0.5);
    client.move_with_command(&turn_cmd).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Stop again
    tracing::info!("Stopping");
    client.stop().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Wave hand
    tracing::info!("Waving right hand");
    client.wave_hand(Hand::Right).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Rotate head
    tracing::info!("Looking around");
    client.rotate_head(0.2, 0.5).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    client.rotate_head(0.0, 0.0).await?; // Center head

    // Lie down
    tracing::info!("Lying down");
    client.lie_down().await?;

    tracing::info!("Example completed successfully");

    Ok(())
}
