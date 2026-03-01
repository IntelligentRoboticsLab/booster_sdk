//! High-level locomotion control example.
//!
//! Run with:
//! `cargo run --manifest-path examples/rust/locomotion/Cargo.toml`

use booster_sdk::client::loco::BoosterClient;
use booster_sdk::types::RobotMode;
use tokio::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    tracing::info!("Starting locomotion control example");

    let client = BoosterClient::new()?;

    tracing::info!("Changing to walking mode...");
    client.change_mode(RobotMode::Walking).await?;
    tracing::info!("Mode changed successfully");

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
