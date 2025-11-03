//! High-level locomotion control client.
//!
//! The `B1LocoClient` provides a high-level async API for controlling the B1 robot,
//! including locomotion, arm manipulation, head control, and predefined gestures.

use super::commands::{
    DexterousHandCommand, FrameTransformQuery, GripperCommand, HandPoseCommand,
    HandPoseWithAuxCommand, HandTransformCommand, HeadRotation, HeadRotationContinuous,
    MoveCommand,
};
use crate::{
    dds::{RpcClient, RpcClientOptions},
    types::{BoosterError, DanceId, Direction, Frame, Hand, Result, RobotMode, Transform},
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

#[derive(Deserialize)]
struct EmptyResponse {}

/// High-level locomotion and manipulation control client
pub struct B1LocoClient {
    rpc: Arc<RpcClient>,
}

impl B1LocoClient {
    /// Create a new `B1LocoClient` with default options.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying RPC client cannot be created.
    pub async fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::default()).await
    }

    /// Create a new `B1LocoClient` with a custom timeout.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying RPC client cannot be created.
    pub async fn with_timeout(timeout: Duration) -> Result<Self> {
        Self::with_options(RpcClientOptions {
            default_timeout: timeout,
            ..Default::default()
        })
        .await
    }

    /// Create a new client with fully customized RPC options.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying RPC client cannot be created.
    pub async fn with_options(options: RpcClientOptions) -> Result<Self> {
        let domain_id = options.domain_id;
        tracing::debug!("Initializing B1LocoClient (domain {})", domain_id);

        // Create RPC client for "loco" service
        let rpc = Arc::new(RpcClient::connect("loco", options).await?);

        Ok(Self { rpc })
    }

    /// Change robot operational mode
    ///
    /// # Arguments
    /// * `mode` - Target robot mode (Damping, Prepare, Walking, Custom)
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use booster_sdk::client::B1LocoClient;
    /// use booster_sdk::types::RobotMode;
    ///
    /// let client = B1LocoClient::new().await?;
    /// client.change_mode(RobotMode::Walking).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn change_mode(&self, mode: RobotMode) -> Result<()> {
        #[derive(Serialize)]
        struct Params {
            mode: i32,
        }

        tracing::debug!("Changing mode to: {:?}", mode);

        self.rpc
            .call::<Params, EmptyResponse>(
                "ChangeMode",
                &Params {
                    mode: i32::from(mode),
                },
                None,
            )
            .await?;

        Ok(())
    }

    /// Get current robot mode
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails or the response contains an unknown mode.
    pub async fn get_mode(&self) -> Result<RobotMode> {
        #[derive(Serialize)]
        struct Params {}

        #[derive(Deserialize)]
        struct Response {
            mode: i32,
        }

        tracing::debug!("Getting current mode");

        let response = self
            .rpc
            .call::<Params, Response>("GetMode", &Params {}, None)
            .await?;

        let mode = response.mode;

        RobotMode::try_from(mode).map_err(|()| BoosterError::Other(format!("Invalid mode: {mode}")))
    }

    /// Move the robot with velocity control
    ///
    /// # Arguments
    /// * `vx` - Forward/backward velocity (m/s)
    /// * `vy` - Left/right velocity (m/s)
    /// * `vyaw` - Yaw angular velocity (rad/s)
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use booster_sdk::client::B1LocoClient;
    ///
    /// let client = B1LocoClient::new().await?;
    /// // Move forward at 0.5 m/s
    /// client.move_robot(0.5, 0.0, 0.0).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn move_robot(&self, vx: f32, vy: f32, vyaw: f32) -> Result<()> {
        tracing::debug!("Moving robot: vx={}, vy={}, vyaw={}", vx, vy, vyaw);

        let cmd = MoveCommand { vx, vy, vyaw };

        self.rpc
            .call::<MoveCommand, EmptyResponse>("Move", &cmd, None)
            .await?;

        Ok(())
    }

    /// Move the robot using a `MoveCommand`
    ///
    /// # Errors
    ///
    /// Returns an error if forwarding the command fails.
    pub async fn move_with_command(&self, cmd: &MoveCommand) -> Result<()> {
        self.move_robot(cmd.vx, cmd.vy, cmd.vyaw).await
    }

    /// Make the robot lie down
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn lie_down(&self) -> Result<()> {
        #[derive(Serialize)]
        struct Params {}

        tracing::debug!("Commanding robot to lie down");

        self.rpc
            .call::<Params, EmptyResponse>("LieDown", &Params {}, None)
            .await?;

        Ok(())
    }

    /// Make the robot stand up from lying position
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn get_up(&self) -> Result<()> {
        #[derive(Serialize)]
        struct Params {}

        tracing::debug!("Commanding robot to get up");

        self.rpc
            .call::<Params, EmptyResponse>("GetUp", &Params {}, None)
            .await?;

        Ok(())
    }

    /// Rotate the robot's head to absolute angles
    ///
    /// # Arguments
    /// * `pitch` - Pitch angle in radians (up/down)
    /// * `yaw` - Yaw angle in radians (left/right)
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn rotate_head(&self, pitch: f32, yaw: f32) -> Result<()> {
        tracing::debug!("Rotating head: pitch={}, yaw={}", pitch, yaw);

        let cmd = HeadRotation { pitch, yaw };

        self.rpc
            .call::<HeadRotation, EmptyResponse>("RotateHead", &cmd, None)
            .await?;

        Ok(())
    }

    /// Rotate the robot's head continuously with direction control
    ///
    /// # Arguments
    /// * `pitch_direction` - Pitch direction (Positive, Stop, Negative)
    /// * `yaw_direction` - Yaw direction (Positive, Stop, Negative)
    /// * `speed` - Speed multiplier (0.0 to 1.0)
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn rotate_head_continuous(
        &self,
        pitch_direction: Direction,
        yaw_direction: Direction,
        speed: f32,
    ) -> Result<()> {
        tracing::debug!(
            "Rotating head continuously: pitch={:?}, yaw={:?}, speed={}",
            pitch_direction,
            yaw_direction,
            speed
        );

        let cmd = HeadRotationContinuous {
            pitch_direction,
            yaw_direction,
            speed,
        };

        self.rpc
            .call::<HeadRotationContinuous, EmptyResponse>("RotateHeadWithDirection", &cmd, None)
            .await?;

        Ok(())
    }

    /// Move hand end-effector to target pose
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn move_hand(&self, cmd: &HandPoseCommand) -> Result<()> {
        tracing::debug!("Moving hand: {:?}", cmd.hand);

        self.rpc
            .call::<HandPoseCommand, EmptyResponse>("MoveHandEndEffectorV2", cmd, None)
            .await?;

        Ok(())
    }

    /// Move hand end-effector with auxiliary waypoint
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn move_hand_with_aux(&self, cmd: &HandPoseWithAuxCommand) -> Result<()> {
        tracing::debug!("Moving hand with auxiliary waypoint: {:?}", cmd.hand);

        self.rpc
            .call::<HandPoseWithAuxCommand, EmptyResponse>("MoveHandEndEffectorWithAux", cmd, None)
            .await?;

        Ok(())
    }

    /// Move hand end-effector using transform
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn move_hand_transform(&self, cmd: &HandTransformCommand) -> Result<()> {
        tracing::debug!("Moving hand with transform: {:?}", cmd.hand);

        self.rpc
            .call::<HandTransformCommand, EmptyResponse>("MoveHandEndEffectorV2", cmd, None)
            .await?;

        Ok(())
    }

    /// Perform a waving gesture with the specified hand
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn wave_hand(&self, hand: Hand) -> Result<()> {
        #[derive(Serialize)]
        struct Params {
            hand: i32,
        }

        tracing::debug!("Waving hand: {:?}", hand);

        self.rpc
            .call::<Params, EmptyResponse>(
                "WaveHand",
                &Params {
                    hand: i32::from(hand),
                },
                None,
            )
            .await?;

        Ok(())
    }

    /// Perform a handshake gesture
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn handshake(&self) -> Result<()> {
        #[derive(Serialize)]
        struct Params {}

        tracing::debug!("Performing handshake");

        self.rpc
            .call::<Params, EmptyResponse>("Handshake", &Params {}, None)
            .await?;

        Ok(())
    }

    /// Control gripper
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn control_gripper(&self, cmd: &GripperCommand) -> Result<()> {
        tracing::debug!("Controlling gripper: {:?} {:?}", cmd.hand, cmd.mode);

        self.rpc
            .call::<GripperCommand, EmptyResponse>("ControlGripper", cmd, None)
            .await?;

        Ok(())
    }

    /// Control dexterous hand (per-finger control)
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn control_dexterous_hand(&self, cmd: &DexterousHandCommand) -> Result<()> {
        tracing::debug!("Controlling dexterous hand: {:?}", cmd.hand);

        self.rpc
            .call::<DexterousHandCommand, EmptyResponse>("ControlDexterousHand", cmd, None)
            .await?;

        Ok(())
    }

    /// Get transform between two coordinate frames
    ///
    /// # Arguments
    /// * `source` - Source frame
    /// * `destination` - Destination frame
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    ///
    /// # Returns
    /// Transform from source to destination frame
    pub async fn get_frame_transform(
        &self,
        source: Frame,
        destination: Frame,
    ) -> Result<Transform> {
        #[derive(Deserialize)]
        struct Response {
            transform: Transform,
        }

        tracing::debug!("Getting transform: {:?} -> {:?}", source, destination);

        let query = FrameTransformQuery {
            source,
            destination,
        };

        let response = self
            .rpc
            .call::<FrameTransformQuery, Response>("GetFrameTransform", &query, None)
            .await?;

        Ok(response.transform)
    }

    /// Perform a predefined dance routine
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn dance(&self, dance_id: DanceId) -> Result<()> {
        #[derive(Serialize)]
        struct Params {
            dance_id: i32,
        }

        tracing::debug!("Dancing: {:?}", dance_id);

        self.rpc
            .call::<Params, EmptyResponse>(
                "Dance",
                &Params {
                    dance_id: i32::from(dance_id),
                },
                None,
            )
            .await?;

        Ok(())
    }

    /// Stop all movement immediately
    ///
    /// # Errors
    ///
    /// Returns an error if the RPC request fails.
    pub async fn stop(&self) -> Result<()> {
        self.move_robot(0.0, 0.0, 0.0).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_f32_eq(actual: f32, expected: f32) {
        assert!((actual - expected).abs() <= f32::EPSILON);
    }

    #[test]
    fn test_move_command_conversion() {
        let cmd = MoveCommand::forward(0.5);
        assert_f32_eq(cmd.vx, 0.5);
        assert_f32_eq(cmd.vy, 0.0);
        assert_f32_eq(cmd.vyaw, 0.0);
    }
}
