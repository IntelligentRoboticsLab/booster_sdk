//! Odometry-based point-to-point walking (port of the C++ `MoveController`).
//!
//! The robot must already be in walking mode. Walks 0.5 m forward, turns
//! around, and walks back.
//!
//! Run with: `cargo run -p move_controller`
//! (`BOOSTER_DOMAIN_ID` / `BOOSTER_DDS_NETWORK` select the DDS domain/network.)

use booster_sdk::client::MoveController;
use booster_sdk::dds::RpcClientOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut controller = MoveController::with_options(RpcClientOptions::from_env()).await?;
    println!("start pose: {:?}", controller.pose());

    controller.move_to_relative(0.5, 0.0, 0.3).await;
    println!("after forward: {:?}", controller.pose());

    controller.turn_around(std::f32::consts::PI, 0.5).await;
    controller.move_to_target(0.0, 0.0, 0.3).await;
    println!("back home: {:?}", controller.pose());

    controller.close().await;
    Ok(())
}
