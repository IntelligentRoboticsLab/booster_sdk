//! Gripper control example
//!
//! This example demonstrates how to control the robot's grippers.
//!
//! Run with: cargo run --example `gripper_control`

use booster_sdk::client::{BoosterClient, commands::GripperCommand};
use booster_sdk::types::{Hand, RobotMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    tracing::info!("Starting gripper control example");

    let client = BoosterClient::new()?;

    // Ensure robot is in correct mode
    tracing::info!("Preparing robot...");
    client.change_mode(RobotMode::Walking).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Open both grippers
    tracing::info!("Opening both grippers");
    client.publish_gripper_command(&GripperCommand::open(Hand::Left))?;
    client.publish_gripper_command(&GripperCommand::open(Hand::Right))?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Close left gripper
    tracing::info!("Closing left gripper");
    client.publish_gripper_command(&GripperCommand::close(Hand::Left))?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Open left gripper again
    tracing::info!("Opening left gripper");
    client.publish_gripper_command(&GripperCommand::open(Hand::Left))?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Force-based grasp with right hand
    tracing::info!("Grasping with right hand (force control)");
    client.publish_gripper_command(&GripperCommand::grasp(Hand::Right, 600))?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Release
    tracing::info!("Releasing grasp");
    client.publish_gripper_command(&GripperCommand::open(Hand::Right))?;

    tracing::info!("Example completed successfully");

    Ok(())
}
