//! High-level B1 locomotion client built on DDS RPC and topic I/O.

use crate::dds::RpcClient;
use crate::dds::{
    BatteryState, BinaryData, ButtonEventMsg, DdsNode, DdsPublisher, DdsSubscription,
    GripperControl, LightControlMsg, MotionState, RemoteControllerState, RobotProcessStateMsg,
    RobotStatusDdsMsg, RpcClientOptions, SafeMode, battery_state_topic, button_event_topic,
    device_gateway_topic, gripper_control_topic, light_control_topic, motion_state_topic,
    process_state_topic, remote_controller_topic, safe_mode_topic, video_stream_topic,
};
use crate::types::{
    BoosterHandType, CustomTrainedTraj, DanceId, DexterousFingerParameter, Frame, GetModeResponse,
    GetRobotInfoResponse, GetStatusResponse, GripperControlMode, GripperMotionParameter,
    HandAction, HandIndex, LoadCustomTrainedTrajResponse, LocoApiId, Result, RobotMode, Transform,
    WholeBodyDanceId,
};
use serde_json::json;

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
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::default())
    }

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

    pub fn node(&self) -> &DdsNode {
        self.rpc.node()
    }

    pub async fn change_mode(&self, mode: RobotMode) -> Result<()> {
        let param = json!({ "mode": i32::from(mode) }).to_string();
        self.rpc
            .call_void_with_timeout(LocoApiId::ChangeMode, param, Some(CHANGE_MODE_TIMEOUT))
            .await
    }

    pub async fn get_mode(&self) -> Result<GetModeResponse> {
        self.rpc.call_response(LocoApiId::GetMode, "").await
    }

    pub async fn get_status(&self) -> Result<GetStatusResponse> {
        self.rpc.call_response(LocoApiId::GetStatus, "").await
    }

    pub async fn get_robot_info(&self) -> Result<GetRobotInfoResponse> {
        self.rpc.call_response(LocoApiId::GetRobotInfo, "").await
    }

    pub async fn move_robot(&self, vx: f32, vy: f32, vyaw: f32) -> Result<()> {
        let param = json!({ "vx": vx, "vy": vy, "vyaw": vyaw }).to_string();
        self.rpc.call_void(LocoApiId::Move, param).await
    }

    pub async fn rotate_head(&self, pitch: f32, yaw: f32) -> Result<()> {
        let param = json!({ "pitch": pitch, "yaw": yaw }).to_string();
        self.rpc.call_void(LocoApiId::RotateHead, param).await
    }

    pub async fn wave_hand(&self, action: HandAction) -> Result<()> {
        let param = json!({
            "hand_index": i32::from(HandIndex::Right),
            "hand_action": i32::from(action),
        })
        .to_string();
        self.rpc.call_void(LocoApiId::WaveHand, param).await
    }

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

    pub async fn lie_down(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::LieDown, "").await
    }

    pub async fn get_up(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::GetUp, "").await
    }

    pub async fn get_up_with_mode(&self, mode: RobotMode) -> Result<()> {
        let param = json!({ "mode": i32::from(mode) }).to_string();
        self.rpc.call_void(LocoApiId::GetUpWithMode, param).await
    }

    pub async fn shoot(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::Shoot, "").await
    }

    pub async fn push_up(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::PushUp, "").await
    }

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

    pub async fn stop_hand_end_effector(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::StopHandEndEffector, "").await
    }

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

    pub async fn switch_hand_end_effector_control_mode(&self, switch_on: bool) -> Result<()> {
        let param = json!({ "switch_on": switch_on }).to_string();
        self.rpc
            .call_void(LocoApiId::SwitchHandEndEffectorControlMode, param)
            .await
    }

    pub async fn handshake(&self, action: HandAction) -> Result<()> {
        let param = json!({ "hand_action": i32::from(action) }).to_string();
        self.rpc.call_void(LocoApiId::Handshake, param).await
    }

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

    pub async fn control_dexterous_hand_default(
        &self,
        finger_params: &[DexterousFingerParameter],
        hand_index: HandIndex,
    ) -> Result<()> {
        self.control_dexterous_hand(finger_params, hand_index, BoosterHandType::InspireHand)
            .await
    }

    pub async fn dance(&self, dance_id: DanceId) -> Result<()> {
        let param = json!({ "dance_id": i32::from(dance_id) }).to_string();
        self.rpc.call_void(LocoApiId::Dance, param).await
    }

    pub async fn play_sound(&self, sound_file_path: impl Into<String>) -> Result<()> {
        let param = json!({ "sound_file_path": sound_file_path.into() }).to_string();
        self.rpc.call_void(LocoApiId::PlaySound, param).await
    }

    pub async fn stop_sound(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::StopSound, "").await
    }

    pub async fn zero_torque_drag(&self, active: bool) -> Result<()> {
        let param = json!({ "enable": active }).to_string();
        self.rpc.call_void(LocoApiId::ZeroTorqueDrag, param).await
    }

    pub async fn record_trajectory(&self, active: bool) -> Result<()> {
        let param = json!({ "enable": active }).to_string();
        self.rpc.call_void(LocoApiId::RecordTrajectory, param).await
    }

    pub async fn replay_trajectory(&self, traj_file_path: impl Into<String>) -> Result<()> {
        let param = json!({ "traj_file_path": traj_file_path.into() }).to_string();
        self.rpc.call_void(LocoApiId::ReplayTrajectory, param).await
    }

    pub async fn whole_body_dance(&self, dance_id: WholeBodyDanceId) -> Result<()> {
        let param = json!({ "dance_id": i32::from(dance_id) }).to_string();
        self.rpc.call_void(LocoApiId::WholeBodyDance, param).await
    }

    pub async fn upper_body_custom_control(&self, start: bool) -> Result<()> {
        let param = json!({ "start": start }).to_string();
        self.rpc
            .call_void(LocoApiId::UpperBodyCustomControl, param)
            .await
    }

    pub async fn reset_odometry(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::ResetOdometry, "").await
    }

    pub async fn load_custom_trained_traj(
        &self,
        traj: &CustomTrainedTraj,
    ) -> Result<LoadCustomTrainedTrajResponse> {
        self.rpc
            .call_serialized_response(LocoApiId::LoadCustomTrainedTraj, traj)
            .await
    }

    pub async fn activate_custom_trained_traj(&self, tid: impl Into<String>) -> Result<()> {
        let param = json!({ "tid": tid.into() }).to_string();
        self.rpc
            .call_void(LocoApiId::ActivateCustomTrainedTraj, param)
            .await
    }

    pub async fn unload_custom_trained_traj(&self, tid: impl Into<String>) -> Result<()> {
        let param = json!({ "tid": tid.into() }).to_string();
        self.rpc
            .call_void(LocoApiId::UnloadCustomTrainedTraj, param)
            .await
    }

    pub async fn enter_wbc_gait(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::EnterWbcGait, "").await
    }

    pub async fn exit_wbc_gait(&self) -> Result<()> {
        self.rpc.call_void(LocoApiId::ExitWbcGait, "").await
    }

    pub fn publish_gripper(&self, control: GripperControl) -> Result<()> {
        self.gripper_publisher.write(control)
    }

    pub fn publish_gripper_command(&self, command: &crate::client::GripperCommand) -> Result<()> {
        self.gripper_publisher.write(command.to_dds_control())
    }

    pub fn publish_light_control(&self, message: LightControlMsg) -> Result<()> {
        self.light_publisher.write(message)
    }

    pub fn enter_safe_mode(&self, message: SafeMode) -> Result<()> {
        self.safe_mode_publisher.write(message)
    }

    pub fn subscribe_device_gateway(&self) -> Result<DdsSubscription<RobotStatusDdsMsg>> {
        self.rpc.node().subscribe(&device_gateway_topic(), 32)
    }

    pub fn subscribe_motion_state(&self) -> Result<DdsSubscription<MotionState>> {
        self.rpc.node().subscribe(&motion_state_topic(), 16)
    }

    pub fn subscribe_battery_state(&self) -> Result<DdsSubscription<BatteryState>> {
        self.rpc.node().subscribe(&battery_state_topic(), 8)
    }

    pub fn subscribe_button_events(&self) -> Result<DdsSubscription<ButtonEventMsg>> {
        self.rpc.node().subscribe(&button_event_topic(), 32)
    }

    pub fn subscribe_remote_controller(&self) -> Result<DdsSubscription<RemoteControllerState>> {
        self.rpc.node().subscribe(&remote_controller_topic(), 32)
    }

    pub fn subscribe_process_state(&self) -> Result<DdsSubscription<RobotProcessStateMsg>> {
        self.rpc.node().subscribe(&process_state_topic(), 8)
    }

    pub fn subscribe_video_stream(&self) -> Result<DdsSubscription<BinaryData>> {
        self.rpc.node().subscribe(&video_stream_topic(), 4)
    }
}

/// Alias matching the C++ class naming.
pub type B1LocoClient = BoosterClient;
