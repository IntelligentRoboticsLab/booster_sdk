//! High-level B1 locomotion client built on DDS RPC and topic I/O.

use crate::dds::{
    BatteryState, BinaryData, ButtonEventMsg, DdsNode, DdsPublisher, DdsSubscription,
    GripperControl, LightControlMsg, MotionState, RemoteControllerState, RobotProcessStateMsg,
    RobotStatusDdsMsg, RpcClient, RpcClientOptions, SafeMode, battery_state_topic,
    button_event_topic, device_gateway_topic, gripper_control_topic, light_control_topic,
    motion_state_topic, process_state_topic, remote_controller_topic, safe_mode_topic,
    video_stream_topic,
};
use crate::types::{
    BoosterHandType, CustomTrainedTraj, DanceId, DexterousFingerParameter, Frame, GetModeResponse,
    GetRobotInfoResponse, GetStatusResponse, GripperControlMode, GripperMode,
    GripperMotionParameter, Hand, HandAction, HandIndex, LoadCustomTrainedTrajResponse, LocoApiId,
    Result, RobotMode, Transform, WholeBodyDanceId,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use typed_builder::TypedBuilder;

// The controller may send an intermediate pending status (-1) before the
// final success response. Mode transitions (especially PREPARE) can take
// several seconds.
const CHANGE_MODE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// High-level client for B1 locomotion control and telemetry.
pub struct BoosterClient {
    rpc: RpcClient,
    gripper_publisher: DdsPublisher<GripperControl>,
    light_publisher: DdsPublisher<LightControlMsg>,
    safe_mode_publisher: DdsPublisher<SafeMode>,
}

impl BoosterClient {
    /// Create a locomotion client with default options.
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::default())
    }

    /// Create a locomotion client with custom RPC options.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::new(options)?;
        let node = rpc.node().clone();
        let gripper_publisher = node.publisher::<GripperControl>(&gripper_control_topic())?;
        let light_publisher = node.publisher::<LightControlMsg>(&light_control_topic())?;
        let safe_mode_publisher = node.publisher::<SafeMode>(&safe_mode_topic())?;

        Ok(Self {
            rpc,
            gripper_publisher,
            light_publisher,
            safe_mode_publisher,
        })
    }

    /// Access the underlying DDS node.
    pub fn node(&self) -> &DdsNode {
        self.rpc.node()
    }

    /// Change the robot mode.
    pub async fn change_mode(&self, mode: RobotMode) -> Result<()> {
        let param = json!({ "mode": i32::from(mode) }).to_string();
        self.rpc
            .call_void_with_timeout(LocoApiId::ChangeMode, param, Some(CHANGE_MODE_TIMEOUT))
            .await
    }

    /// Get the current robot mode.
    pub async fn get_mode(&self) -> Result<GetModeResponse> {
        self.rpc.call_response(LocoApiId::GetMode, "").await
    }

    /// Get the current robot status.
    pub async fn get_status(&self) -> Result<GetStatusResponse> {
        self.rpc.call_response(LocoApiId::GetStatus, "").await
    }

    /// Get robot identity and version information.
    pub async fn get_robot_info(&self) -> Result<GetRobotInfoResponse> {
        self.rpc.call_response(LocoApiId::GetRobotInfo, "").await
    }

    /// Move the robot base in body frame.
    pub async fn move_robot(&self, vx: f32, vy: f32, vyaw: f32) -> Result<()> {
        let param = json!({ "vx": vx, "vy": vy, "vyaw": vyaw }).to_string();
        self.rpc.call_void(LocoApiId::Move, param).await
    }

    /// Rotate the head to absolute pitch/yaw angles.
    pub async fn rotate_head(&self, pitch: f32, yaw: f32) -> Result<()> {
        let param = json!({ "pitch": pitch, "yaw": yaw }).to_string();
        self.rpc.call_void(LocoApiId::RotateHead, param).await
    }

    /// Trigger a right-hand wave action.
    pub async fn wave_hand(&self, action: HandAction) -> Result<()> {
        let param = json!({
            "hand_index": i32::from(HandIndex::Right),
            "hand_action": i32::from(action),
        })
        .to_string();
        self.rpc.call_void(LocoApiId::WaveHand, param).await
    }

    /// Rotate the head with direction steps.
    pub async fn rotate_head_with_direction(
        &self,
        pitch_direction: i32,
        yaw_direction: i32,
    ) -> Result<()> {
        let param = json!({
            "pitch_direction": pitch_direction,
            "yaw_direction": yaw_direction,
        })
        .to_string();
        self.rpc
            .call_void(LocoApiId::RotateHeadWithDirection, param)
            .await
    }

    /// Command the robot to lie down.
    pub async fn lie_down(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::LieDown, "").await
    }

    /// Command the robot to get up.
    pub async fn get_up(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::GetUp, "").await
    }

    /// Command the robot to get up into a specific mode.
    pub async fn get_up_with_mode(&self, mode: RobotMode) -> Result<()> {
        let param = json!({ "mode": i32::from(mode) }).to_string();
        self.rpc.call_void(LocoApiId::GetUpWithMode, param).await
    }

    /// Trigger a shoot action.
    pub async fn shoot(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::Shoot, "").await
    }

    /// Trigger a push-up action.
    pub async fn push_up(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::PushUp, "").await
    }

    /// Move a hand end effector with auxiliary posture input.
    pub async fn move_hand_end_effector_with_aux(
        &self,
        target_posture: &crate::types::Posture,
        aux_posture: &crate::types::Posture,
        time_millis: i32,
        hand_index: HandIndex,
    ) -> Result<()> {
        let param = json!({
            "target_posture": target_posture,
            "aux_posture": aux_posture,
            "time_millis": time_millis,
            "hand_index": i32::from(hand_index),
            "has_aux": true,
            "new_version": false,
        })
        .to_string();
        self.rpc
            .call_void(LocoApiId::MoveHandEndEffector, param)
            .await
    }

    /// Move a hand end effector.
    pub async fn move_hand_end_effector(
        &self,
        target_posture: &crate::types::Posture,
        time_millis: i32,
        hand_index: HandIndex,
    ) -> Result<()> {
        let param = json!({
            "target_posture": target_posture,
            "time_millis": time_millis,
            "hand_index": i32::from(hand_index),
            "has_aux": false,
            "new_version": false,
        })
        .to_string();
        self.rpc
            .call_void(LocoApiId::MoveHandEndEffector, param)
            .await
    }

    /// Move a hand end effector using the v2 behavior flag.
    pub async fn move_hand_end_effector_v2(
        &self,
        target_posture: &crate::types::Posture,
        time_millis: i32,
        hand_index: HandIndex,
    ) -> Result<()> {
        let param = json!({
            "target_posture": target_posture,
            "time_millis": time_millis,
            "hand_index": i32::from(hand_index),
            "has_aux": false,
            "new_version": true,
        })
        .to_string();
        self.rpc
            .call_void(LocoApiId::MoveHandEndEffector, param)
            .await
    }

    /// Stop hand end-effector motion.
    pub async fn stop_hand_end_effector(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::StopHandEndEffector, "").await
    }

    /// Control a gripper.
    pub async fn control_gripper(
        &self,
        motion_param: GripperMotionParameter,
        mode: GripperControlMode,
        hand_index: HandIndex,
    ) -> Result<()> {
        let param = json!({
            "motion_param": motion_param,
            "mode": i32::from(mode),
            "hand_index": i32::from(hand_index),
        })
        .to_string();
        self.rpc.call_void(LocoApiId::ControlGripper, param).await
    }

    /// Query the transform from `src` frame to `dst` frame.
    pub async fn get_frame_transform(&self, src: Frame, dst: Frame) -> Result<Transform> {
        let param = json!({
            "src": i32::from(src),
            "dst": i32::from(dst),
        })
        .to_string();
        self.rpc
            .call_response(LocoApiId::GetFrameTransform, param)
            .await
    }

    /// Enable or disable hand end-effector control mode.
    pub async fn switch_hand_end_effector_control_mode(&self, switch_on: bool) -> Result<()> {
        let param = json!({ "switch_on": switch_on }).to_string();
        self.rpc
            .call_void(LocoApiId::SwitchHandEndEffectorControlMode, param)
            .await
    }

    /// Trigger a handshake action.
    pub async fn handshake(&self, action: HandAction) -> Result<()> {
        let param = json!({ "hand_action": i32::from(action) }).to_string();
        self.rpc.call_void(LocoApiId::Handshake, param).await
    }

    /// Control a dexterous hand with explicit hand type.
    pub async fn control_dexterous_hand(
        &self,
        finger_params: &[DexterousFingerParameter],
        hand_index: HandIndex,
        hand_type: BoosterHandType,
    ) -> Result<()> {
        let param = json!({
            "finger_params": finger_params,
            "hand_index": i32::from(hand_index),
            "hand_type": i32::from(hand_type),
        })
        .to_string();
        self.rpc
            .call_void(LocoApiId::ControlDexterousHand, param)
            .await
    }

    /// Control a dexterous hand using the default hand type.
    pub async fn control_dexterous_hand_default(
        &self,
        finger_params: &[DexterousFingerParameter],
        hand_index: HandIndex,
    ) -> Result<()> {
        self.control_dexterous_hand(finger_params, hand_index, BoosterHandType::InspireHand)
            .await
    }

    /// Trigger an upper-body dance or gesture action.
    pub async fn dance(&self, dance_id: DanceId) -> Result<()> {
        let param = json!({ "dance_id": i32::from(dance_id) }).to_string();
        self.rpc.call_void(LocoApiId::Dance, param).await
    }

    /// Play a sound file on the robot.
    pub async fn play_sound(&self, sound_file_path: impl Into<String>) -> Result<()> {
        let param = json!({ "sound_file_path": sound_file_path.into() }).to_string();
        self.rpc.call_void(LocoApiId::PlaySound, param).await
    }

    /// Stop active sound playback.
    pub async fn stop_sound(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::StopSound, "").await
    }

    /// Enable or disable zero-torque drag mode.
    pub async fn zero_torque_drag(&self, active: bool) -> Result<()> {
        let param = json!({ "enable": active }).to_string();
        self.rpc.call_void(LocoApiId::ZeroTorqueDrag, param).await
    }

    /// Start or stop trajectory recording.
    pub async fn record_trajectory(&self, active: bool) -> Result<()> {
        let param = json!({ "enable": active }).to_string();
        self.rpc.call_void(LocoApiId::RecordTrajectory, param).await
    }

    /// Replay a recorded trajectory from file.
    pub async fn replay_trajectory(&self, traj_file_path: impl Into<String>) -> Result<()> {
        let param = json!({ "traj_file_path": traj_file_path.into() }).to_string();
        self.rpc.call_void(LocoApiId::ReplayTrajectory, param).await
    }

    /// Trigger a whole-body dance action.
    pub async fn whole_body_dance(&self, dance_id: WholeBodyDanceId) -> Result<()> {
        let param = json!({ "dance_id": i32::from(dance_id) }).to_string();
        self.rpc.call_void(LocoApiId::WholeBodyDance, param).await
    }

    /// Enable or disable upper-body custom control.
    pub async fn upper_body_custom_control(&self, start: bool) -> Result<()> {
        let param = json!({ "start": start }).to_string();
        self.rpc
            .call_void(LocoApiId::UpperBodyCustomControl, param)
            .await
    }

    /// Reset odometry state.
    pub async fn reset_odometry(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::ResetOdometry, "").await
    }

    /// Load a custom trained trajectory.
    pub async fn load_custom_trained_traj(
        &self,
        traj: &CustomTrainedTraj,
    ) -> Result<LoadCustomTrainedTrajResponse> {
        self.rpc
            .call_serialized_response(LocoApiId::LoadCustomTrainedTraj, traj)
            .await
    }

    /// Activate a loaded custom trained trajectory by id.
    pub async fn activate_custom_trained_traj(&self, tid: impl Into<String>) -> Result<()> {
        let param = json!({ "tid": tid.into() }).to_string();
        self.rpc
            .call_void(LocoApiId::ActivateCustomTrainedTraj, param)
            .await
    }

    /// Unload a custom trained trajectory by id.
    pub async fn unload_custom_trained_traj(&self, tid: impl Into<String>) -> Result<()> {
        let param = json!({ "tid": tid.into() }).to_string();
        self.rpc
            .call_void(LocoApiId::UnloadCustomTrainedTraj, param)
            .await
    }

    /// Enter WBC gait mode.
    pub async fn enter_wbc_gait(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::EnterWbcGait, "").await
    }

    /// Exit WBC gait mode.
    pub async fn exit_wbc_gait(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::ExitWbcGait, "").await
    }

    /// Publish a raw gripper control topic message.
    pub fn publish_gripper(&self, control: GripperControl) -> Result<()> {
        self.gripper_publisher.write(control)
    }

    /// Publish a high-level gripper command.
    pub fn publish_gripper_command(&self, command: &GripperCommand) -> Result<()> {
        self.gripper_publisher.write(command.to_dds_control())
    }

    /// Publish a light control topic message.
    pub fn publish_light_control(&self, message: LightControlMsg) -> Result<()> {
        self.light_publisher.write(message)
    }

    /// Publish a safe mode topic message.
    pub fn enter_safe_mode(&self, message: SafeMode) -> Result<()> {
        self.safe_mode_publisher.write(message)
    }

    /// Subscribe to device gateway robot status messages.
    pub fn subscribe_device_gateway(&self) -> Result<DdsSubscription<RobotStatusDdsMsg>> {
        self.rpc.node().subscribe(&device_gateway_topic(), 32)
    }

    /// Subscribe to motion state messages.
    pub fn subscribe_motion_state(&self) -> Result<DdsSubscription<MotionState>> {
        self.rpc.node().subscribe(&motion_state_topic(), 16)
    }

    /// Subscribe to battery state messages.
    pub fn subscribe_battery_state(&self) -> Result<DdsSubscription<BatteryState>> {
        self.rpc.node().subscribe(&battery_state_topic(), 8)
    }

    /// Subscribe to button event messages.
    pub fn subscribe_button_events(&self) -> Result<DdsSubscription<ButtonEventMsg>> {
        self.rpc.node().subscribe(&button_event_topic(), 32)
    }

    /// Subscribe to remote controller state messages.
    pub fn subscribe_remote_controller(&self) -> Result<DdsSubscription<RemoteControllerState>> {
        self.rpc.node().subscribe(&remote_controller_topic(), 32)
    }

    /// Subscribe to robot process state messages.
    pub fn subscribe_process_state(&self) -> Result<DdsSubscription<RobotProcessStateMsg>> {
        self.rpc.node().subscribe(&process_state_topic(), 8)
    }

    /// Subscribe to video stream messages.
    pub fn subscribe_video_stream(&self) -> Result<DdsSubscription<BinaryData>> {
        self.rpc.node().subscribe(&video_stream_topic(), 4)
    }
}

/// Gripper control command
#[derive(Debug, Clone, Copy, TypedBuilder, Serialize, Deserialize)]
pub struct GripperCommand {
    /// Target hand
    pub hand: Hand,

    /// Control mode (position or force)
    pub mode: GripperMode,

    /// Motion parameter value
    /// - Position mode: 0-1000 (0 = fully open, 1000 = fully closed)
    /// - Force mode: 50-1000 (grasping force)
    pub motion_param: u16,

    /// Movement speed (1-1000)
    #[builder(default = 500)]
    pub speed: u16,
}

impl GripperCommand {
    /// Create a command to open the gripper
    #[must_use]
    pub fn open(hand: Hand) -> Self {
        Self {
            hand,
            mode: GripperMode::Position,
            motion_param: 0,
            speed: 500,
        }
    }

    /// Create a command to close the gripper
    #[must_use]
    pub fn close(hand: Hand) -> Self {
        Self {
            hand,
            mode: GripperMode::Position,
            motion_param: 1000,
            speed: 500,
        }
    }

    /// Create a force-based grasp command
    #[must_use]
    pub fn grasp(hand: Hand, force: u16) -> Self {
        Self {
            hand,
            mode: GripperMode::Force,
            motion_param: force.clamp(50, 1000),
            speed: 500,
        }
    }

    /// Convert to DDS gripper control message.
    #[must_use]
    pub fn to_dds_control(&self) -> crate::dds::GripperControl {
        let (position, force) = match self.mode {
            GripperMode::Position => (self.motion_param as i32, 0),
            GripperMode::Force => (0, self.motion_param as i32),
        };

        crate::dds::GripperControl {
            hand_index: u8::from(self.hand),
            position,
            force,
            speed: self.speed as i32,
        }
    }
}
