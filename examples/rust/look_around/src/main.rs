//! Motion state subscription example.
//!
//! Run with:
//! `cargo run -p look_around`

use booster_sdk::client::loco::BoosterClient;
use tokio::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::new("off,booster_sdk=debug");
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    tracing::info!("Starting motion state subscription example");

    let client = BoosterClient::new()?;
    let mut motion_state = client.subscribe_motion_state()?;

    let timeout = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            _ = &mut timeout => {
                tracing::info!("Finished listening for motion state updates");
                break;
            }
            sample = motion_state.recv() => {
                if let Some(state) = sample {
                    tracing::info!(
                        "Motion state: current={}, target={}, transitioning={}",
                        state.current_mode,
                        state.target_mode,
                        state.is_transitioning
                    );
                }
            }
        }
    }

    Ok(())
}
