//! Gripper control example
//!
//! This example demonstrates how to control the robot's grippers.
//!
//! Run with: cargo run --example `gripper_control`

use booster_client::{B1LocoClient, commands::GripperCommand};
use booster_types::{Hand, RobotMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    tracing::info!("Starting gripper control example");

    let client = B1LocoClient::new().await?;

    // Ensure robot is in correct mode
    tracing::info!("Preparing robot...");
    client.change_mode(RobotMode::Walking).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Open both grippers
    tracing::info!("Opening both grippers");
    client
        .control_gripper(&GripperCommand::open(Hand::Left))
        .await?;
    client
        .control_gripper(&GripperCommand::open(Hand::Right))
        .await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Close left gripper
    tracing::info!("Closing left gripper");
    client
        .control_gripper(&GripperCommand::close(Hand::Left))
        .await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Open left gripper again
    tracing::info!("Opening left gripper");
    client
        .control_gripper(&GripperCommand::open(Hand::Left))
        .await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Force-based grasp with right hand
    tracing::info!("Grasping with right hand (force control)");
    client
        .control_gripper(&GripperCommand::grasp(Hand::Right, 600))
        .await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Release
    tracing::info!("Releasing grasp");
    client
        .control_gripper(&GripperCommand::open(Hand::Right))
        .await?;

    tracing::info!("Example completed successfully");

    Ok(())
}
