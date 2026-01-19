//! High-level Booster robot client built on DDS.

use crate::dds::{
    BatteryState, BinaryData, ButtonEventMsg, DdsNode, DdsPublisher, DdsSubscription,
    GripperControl, LightControlMsg, MotionState, RemoteControllerState, RobotProcessStateMsg,
    RobotStatusDdsMsg, RpcClient, RpcClientOptions, SafeMode, battery_state_topic,
    button_event_topic, device_gateway_topic, gripper_control_topic, light_control_topic,
    motion_state_topic, process_state_topic, remote_controller_topic, safe_mode_topic,
    video_stream_topic,
};
use crate::types::{Result, RobotMode};
use serde::{Deserialize, Serialize};

const CHANGE_MODE_API_ID: i32 = 2000;
const MOVE_API_ID: i32 = 2001;

#[derive(Deserialize)]
struct EmptyResponse {}

/// High-level client for Booster robot control and telemetry.
pub struct BoosterClient {
    rpc: RpcClient,
    node: DdsNode,
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
            node,
            gripper_publisher,
            light_publisher,
            safe_mode_publisher,
        })
    }

    pub async fn change_mode(&self, mode: RobotMode) -> Result<()> {
        #[derive(Serialize)]
        struct Params {
            mode: i32,
        }

        self.rpc
            .call::<Params, EmptyResponse>(
                CHANGE_MODE_API_ID,
                &Params {
                    mode: i32::from(mode),
                },
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn move_robot(&self, vx: f32, vy: f32, vyaw: f32) -> Result<()> {
        #[derive(Serialize)]
        struct Params {
            vx: f32,
            vy: f32,
            vyaw: f32,
        }

        self.rpc
            .call::<Params, EmptyResponse>(MOVE_API_ID, &Params { vx, vy, vyaw }, None)
            .await?;

        Ok(())
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
        self.node.subscribe(&device_gateway_topic(), 32)
    }

    pub fn subscribe_motion_state(&self) -> Result<DdsSubscription<MotionState>> {
        self.node.subscribe(&motion_state_topic(), 16)
    }

    pub fn subscribe_battery_state(&self) -> Result<DdsSubscription<BatteryState>> {
        self.node.subscribe(&battery_state_topic(), 8)
    }

    pub fn subscribe_button_events(&self) -> Result<DdsSubscription<ButtonEventMsg>> {
        self.node.subscribe(&button_event_topic(), 32)
    }

    pub fn subscribe_remote_controller(&self) -> Result<DdsSubscription<RemoteControllerState>> {
        self.node.subscribe(&remote_controller_topic(), 32)
    }

    pub fn subscribe_process_state(&self) -> Result<DdsSubscription<RobotProcessStateMsg>> {
        self.node.subscribe(&process_state_topic(), 8)
    }

    pub fn subscribe_video_stream(&self) -> Result<DdsSubscription<BinaryData>> {
        self.node.subscribe(&video_stream_topic(), 4)
    }
}
