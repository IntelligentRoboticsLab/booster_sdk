//! DDS topic specifications for Booster robot communication.
//!
//! # QoS conventions
//!
//! The C++ SDK creates channels with the Fast DDS defaults unless `reliable`
//! is requested: data writers default to *reliable* and data readers to
//! *best effort*. A best-effort reader matches both reliable and best-effort
//! writers, so state topics that are only read by SDK users use best-effort
//! QoS here. Command topics that the SDK publishes (`rt/joint_ctrl`,
//! `rt/kick_ball`) use reliable QoS, which matches any robot-side reader.
//! Use [`TopicSpec::with_qos`], [`TopicSpec::reliable`] or
//! [`TopicSpec::best_effort`] to override.

use rustdds::{
    Duration, QosPolicies, QosPolicyBuilder, Topic, TopicKind,
    policy::{Durability, History, Reliability},
};

use crate::types::{DdsError, Result};

use super::qos::{
    qos_best_effort_keep_last, qos_reliable_keep_all, qos_reliable_keep_last,
    qos_reliable_transient_local_keep_last,
};

/// Name, DDS type name and QoS of a topic.
#[derive(Debug, Clone)]
pub struct TopicSpec {
    pub name: String,
    pub type_name: &'static str,
    pub qos: QosPolicies,
    pub kind: TopicKind,
}

impl TopicSpec {
    /// Create a spec for a keyless topic with best-effort, keep-last(1) QoS.
    ///
    /// `type_name` must be the exact DDS type name used by the robot, e.g.
    /// [`TYPE_LOW_STATE`].
    pub fn new(name: impl Into<String>, type_name: &'static str) -> Self {
        Self {
            name: name.into(),
            type_name,
            qos: qos_best_effort_keep_last(1),
            kind: TopicKind::NoKey,
        }
    }

    /// Replace the QoS policies.
    #[must_use]
    pub fn with_qos(mut self, qos: QosPolicies) -> Self {
        self.qos = qos;
        self
    }

    /// Use reliable delivery (keeps the configured history).
    #[must_use]
    pub fn reliable(self) -> Self {
        self.modify_qos(QosPolicyBuilder::new().reliability(Reliability::Reliable {
            max_blocking_time: Duration::from_millis(100),
        }))
    }

    /// Use best-effort delivery (keeps the configured history).
    #[must_use]
    pub fn best_effort(self) -> Self {
        self.modify_qos(QosPolicyBuilder::new().reliability(Reliability::BestEffort))
    }

    /// Keep the last `depth` samples.
    #[must_use]
    pub fn keep_last(self, depth: i32) -> Self {
        self.modify_qos(QosPolicyBuilder::new().history(History::KeepLast { depth }))
    }

    /// Keep all samples.
    #[must_use]
    pub fn keep_all(self) -> Self {
        self.modify_qos(QosPolicyBuilder::new().history(History::KeepAll))
    }

    /// Use transient-local durability (late joiners receive cached samples).
    #[must_use]
    pub fn transient_local(self) -> Self {
        self.modify_qos(QosPolicyBuilder::new().durability(Durability::TransientLocal))
    }

    fn modify_qos(mut self, builder: QosPolicyBuilder) -> Self {
        self.qos = self.qos.modify_by(&builder.build());
        self
    }

    pub fn create_topic(&self, participant: &rustdds::DomainParticipant) -> Result<Topic> {
        participant
            .create_topic(
                self.name.clone(),
                self.type_name.to_string(),
                &self.qos,
                self.kind,
            )
            .map_err(|err| DdsError::InitializationFailed(err.to_string()).into())
    }
}

// ---------------------------------------------------------------------------
// DDS type names (as registered by the C++ SDK)
// ---------------------------------------------------------------------------

pub const TYPE_RPC_REQ: &str = "booster_msgs::msg::dds_::RpcReqMsg_";
pub const TYPE_RPC_RESP: &str = "booster_msgs::msg::dds_::RpcRespMsg_";
pub const TYPE_ROBOT_STATUS: &str = "booster_interface::msg::dds_::RobotStatusDdsMsg_";
pub const TYPE_MOTION_STATE: &str = "booster::msg::dds_::MotionState_";
pub const TYPE_BATTERY_STATE: &str = "booster_interface::msg::dds_::BatteryState_";
pub const TYPE_BUTTON_EVENT: &str = "booster_interface::msg::dds_::ButtonEventMsg_";
pub const TYPE_REMOTE_CONTROLLER: &str = "booster_interface::msg::dds_::RemoteControllerState_";
pub const TYPE_PROCESS_STATE: &str = "booster_interface::msg::dds_::RobotProcessStateMsg_";
pub const TYPE_BINARY_DATA: &str = "booster_msgs::msg::dds_::BinaryData_";
/// No such type exists in the C++ SDK 1.8; see [`super::GripperControl`].
pub const TYPE_GRIPPER_CONTROL: &str = "booster_interface::msg::dds_::GripperControl_";
pub const TYPE_LIGHT_CONTROL: &str = "booster_interface::msg::dds_::LightControlMsg_";
pub const TYPE_SAFE_MODE: &str = "booster_msgs::msg::dds_::BinaryData_";
pub const TYPE_SUBTITLE: &str = "booster_interface::msg::dds_::Subtitle_";
pub const TYPE_ASR_CHUNK: &str = "booster_interface::msg::dds_::AsrChunk_";
pub const TYPE_BLUETOOTH_EVENT: &str = "booster_msgs::audio::dds_::BluetoothEventTopic_";

// Low-level control and robot state (booster_interface::msg).
pub const TYPE_LOW_CMD: &str = "booster_interface::msg::dds_::LowCmd_";
pub const TYPE_MOTOR_CMD: &str = "booster_interface::msg::dds_::MotorCmd_";
pub const TYPE_LOW_STATE: &str = "booster_interface::msg::dds_::LowState_";
pub const TYPE_MOTOR_STATE: &str = "booster_interface::msg::dds_::MotorState_";
pub const TYPE_IMU_STATE: &str = "booster_interface::msg::dds_::ImuState_";
pub const TYPE_ODOMETER: &str = "booster_interface::msg::dds_::Odometer_";
pub const TYPE_FALL_DOWN_STATE: &str = "booster_interface::msg::dds_::FallDownState_";
pub const TYPE_HAND_REPLY_DATA: &str = "booster_interface::msg::dds_::HandReplyData_";
pub const TYPE_HAND_REPLY_PARAM: &str = "booster_interface::msg::dds_::HandReplyParam_";
pub const TYPE_HAND_TOUCH_DATA: &str = "booster_interface::msg::dds_::HandTouchData_";
pub const TYPE_HAND_TOUCH_PARAM: &str = "booster_interface::msg::dds_::HandTouchParam_";
pub const TYPE_ROBOT_STATES: &str = "booster_interface::msg::dds_::RobotStatesMsg_";
pub const TYPE_PRONE_BODY_CONTROL_STATUS: &str =
    "booster_interface::msg::dds_::ProneBodyControlStatus_";
pub const TYPE_ROBOCUP_BEHAVIOR_STATUS: &str =
    "booster_interface::msg::dds_::RobocupBehaviorStatus_";
pub const TYPE_ROBOT_REPLAY_TRAJ_ID: &str = "booster_interface::msg::dds_::RobotReplayTrajID_";
pub const TYPE_TRAINED_TRAJ_STATUS: &str = "booster_interface::msg::dds_::TrainedTrajStatus_";
pub const TYPE_LIGHT_PIXEL: &str = "booster_interface::msg::dds_::LightPixel_";
pub const TYPE_ROBOT_DDS_JOINT_STATUS: &str = "booster_interface::msg::dds_::RobotDdsJointStatus_";
pub const TYPE_ROBOT_DDS_IMU_STATUS: &str = "booster_interface::msg::dds_::RobotDdsImuStatus_";
pub const TYPE_ROBOT_DDS_BATTERY_STATUS: &str =
    "booster_interface::msg::dds_::RobotDdsBatteryStatus_";
pub const TYPE_ROBOT_PROCESS_STATE: &str = "booster_interface::msg::dds_::RobotProcessState_";
pub const TYPE_KICK: &str = "brain::msg::dds_::Kick_";

// Device descriptions (booster_interface::msg).
pub const TYPE_CAMERA: &str = "booster_interface::msg::dds_::Camera_";
pub const TYPE_CAMERA_LIST: &str = "booster_interface::msg::dds_::CameraList_";
pub const TYPE_CAMERA_TOPIC: &str = "booster_interface::msg::dds_::CameraTopic_";
pub const TYPE_CAMERA_INTRINSICS: &str = "booster_interface::msg::dds_::CameraIntrinsics_";
pub const TYPE_CAMERA_EXTRINSICS: &str = "booster_interface::msg::dds_::CameraExtrinsics_";
pub const TYPE_VIDEO_STREAM_ENCODING: &str = "booster_interface::msg::dds_::VideoStreamEncoding_";
pub const TYPE_DEPTH_STREAM_ENCODING: &str = "booster_interface::msg::dds_::DepthStreamEncoding_";
pub const TYPE_HAND: &str = "booster_interface::msg::dds_::Hand_";
pub const TYPE_HAND_LIST: &str = "booster_interface::msg::dds_::HandList_";
pub const TYPE_IMU_INFO: &str = "booster_interface::msg::dds_::ImuInfo_";
pub const TYPE_IMU_INFO_LIST: &str = "booster_interface::msg::dds_::ImuInfoList_";
pub const TYPE_ROBOT_MODEL: &str = "booster_interface::msg::dds_::RobotModel_";
pub const TYPE_ORIGIN: &str = "booster_interface::msg::dds_::Origin_";
pub const TYPE_INERTIA: &str = "booster_interface::msg::dds_::Inertia_";
pub const TYPE_INERTIAL: &str = "booster_interface::msg::dds_::Inertial_";
pub const TYPE_BOX: &str = "booster_interface::msg::dds_::Box_";
pub const TYPE_CYLINDER: &str = "booster_interface::msg::dds_::Cylinder_";
pub const TYPE_SPHERE: &str = "booster_interface::msg::dds_::Sphere_";
pub const TYPE_MESH: &str = "booster_interface::msg::dds_::Mesh_";
pub const TYPE_GEOMETRY: &str = "booster_interface::msg::dds_::Geometry_";
pub const TYPE_COLLISION: &str = "booster_interface::msg::dds_::Collision_";
pub const TYPE_LINK: &str = "booster_interface::msg::dds_::Link_";
pub const TYPE_JOINT_LIMIT: &str = "booster_interface::msg::dds_::JointLimit_";
pub const TYPE_JOINT: &str = "booster_interface::msg::dds_::Joint_";

// ROS 2 compatible types.
pub const TYPE_TIME: &str = "builtin_interfaces::msg::dds_::Time_";
pub const TYPE_HEADER: &str = "std_msgs::msg::dds_::Header_";
pub const TYPE_POINT: &str = "geometry_msgs::msg::dds_::Point_";
pub const TYPE_VECTOR3: &str = "geometry_msgs::msg::dds_::Vector3_";
pub const TYPE_QUATERNION: &str = "geometry_msgs::msg::dds_::Quaternion_";
pub const TYPE_POSE: &str = "geometry_msgs::msg::dds_::Pose_";
pub const TYPE_POSE_STAMPED: &str = "geometry_msgs::msg::dds_::PoseStamped_";
pub const TYPE_POSE_WITH_COVARIANCE: &str = "geometry_msgs::msg::dds_::PoseWithCovariance_";
pub const TYPE_TRANSFORM: &str = "geometry_msgs::msg::dds_::Transform_";
pub const TYPE_TRANSFORM_STAMPED: &str = "geometry_msgs::msg::dds_::TransformStamped_";
pub const TYPE_TWIST: &str = "geometry_msgs::msg::dds_::Twist_";
pub const TYPE_TWIST_WITH_COVARIANCE: &str = "geometry_msgs::msg::dds_::TwistWithCovariance_";
pub const TYPE_ODOMETRY: &str = "nav_msgs::msg::dds_::Odometry_";
pub const TYPE_IMU: &str = "sensor_msgs::msg::dds_::Imu_";
pub const TYPE_JOINT_STATE: &str = "sensor_msgs::msg::dds_::JointState_";
pub const TYPE_IMAGE: &str = "sensor_msgs::msg::dds_::Image_";
pub const TYPE_COMPRESSED_IMAGE: &str = "sensor_msgs::msg::dds_::CompressedImage_";
pub const TYPE_CAMERA_INFO: &str = "sensor_msgs::msg::dds_::CameraInfo_";
pub const TYPE_REGION_OF_INTEREST: &str = "sensor_msgs::msg::dds_::RegionOfInterest_";
pub const TYPE_TF_MESSAGE: &str = "tf2_msgs::msg::dds_::TFMessage_";

// ---------------------------------------------------------------------------
// RPC service topics
// ---------------------------------------------------------------------------

pub const LOCO_API_TOPIC: &str = "rt/LocoApiTopic";
pub const AI_API_TOPIC: &str = "rt/AiApiTopic";
pub const LUI_API_TOPIC: &str = "rt/LuiApiTopic";
pub const LIGHT_CONTROL_API_TOPIC: &str = "rt/LightControlApiTopic";
pub const VISION_API_TOPIC: &str = "rt/VisionApiTopic";
pub const X5_CAMERA_CONTROL_API_TOPIC: &str = "rt/X5CameraControl";
pub const CAMERA_API_TOPIC: &str = "rt/CameraApiTopic";
pub const HAND_EYE_CALIB_API_TOPIC: &str = "rt/HandEyeCalibApiTopic";

pub const LOCO_API_OPERATION_EVENT_TOPIC: &str = "rt/LocoApiOperationEvent";
pub const LUI_API_OPERATION_EVENT_TOPIC: &str = "rt/LuiApiOperationEvent";

// ---------------------------------------------------------------------------
// Data topic names
// ---------------------------------------------------------------------------

pub const DEVICE_GATEWAY_TOPIC: &str = "rt/device_gateway";
pub const MOTION_STATE_TOPIC: &str = "rt/motion_state";
pub const BATTERY_STATE_TOPIC: &str = "rt/battery_state";
pub const BUTTON_EVENT_TOPIC: &str = "rt/button_event";
pub const REMOTE_CONTROLLER_STATE_TOPIC: &str = "rt/remote_controller_state";
pub const PROCESS_STATE_TOPIC: &str = "rt/booster_process_state";
pub const VIDEO_STREAM_TOPIC: &str = "rt/booster/video_stream";
pub const GRIPPER_CONTROL_TOPIC: &str = "rt/gripper_control";
pub const LIGHT_CONTROL_TOPIC: &str = "rt/light_control";
pub const SAFE_MODE_TOPIC: &str = "rt/enter_safe_mode";
pub const AI_SUBTITLE_TOPIC: &str = "rt/ai_subtitle";
pub const LUI_ASR_CHUNK_TOPIC: &str = "rt/lui_asr_chunk";
pub const AUDIO_BLUETOOTH_EVENT_TOPIC: &str = "rt/booster/audio/bluetooth_events";

/// Low-level joint-command topic (`kTopicJointCtrl`).
pub const JOINT_CTRL_TOPIC: &str = "rt/joint_ctrl";
/// Low-level motor and IMU state topic (`kTopicLowState`).
pub const LOW_STATE_TOPIC: &str = "rt/low_state";
/// Fall and recovery state topic (`kTopicFallDown`).
pub const FALL_DOWN_TOPIC: &str = "rt/fall_down";
/// Planar odometry topic (`kTopicOdometerState`).
pub const ODOMETER_STATE_TOPIC: &str = "rt/odometer_state";
/// Hand actuator-data topic (`kTopicBoosterHandData`).
pub const BOOSTER_HAND_DATA_TOPIC: &str = "rt/booster_hand_data";
/// Hand touch-sensor topic (`kTopicHandTouchData`).
pub const HAND_TOUCH_DATA_TOPIC: &str = "rt/booster_hand_touch_data";
/// Transform-tree topic (`kTopicTF`).
pub const TF_TOPIC: &str = "rt/tf";
/// Mode, body-control and action state topic (`kTopicRobotStates`).
pub const ROBOT_STATES_TOPIC: &str = "rt/robot_states";
/// Prone-body control status topic (`kTopicProneBodyControlStatus`).
pub const PRONE_BODY_CONTROL_STATUS_TOPIC: &str = "rt/prone_body_control_status";
/// Replayed trajectory id topic (`kTopicRobotReplayTrajID`).
pub const ROBOT_REPLAY_TRAJ_ID_TOPIC: &str = "rt/robot_replay_traj_id";
/// Trained-trajectory status topic (`kTopicTrainedTrajStatus`).
pub const TRAINED_TRAJ_STATUS_TOPIC: &str = "rt/trained_traj_status";
/// Soccer behavior status topic (`kTopicRobocupBehaviorStatus`).
pub const ROBOCUP_BEHAVIOR_STATUS_TOPIC: &str = "rt/robocup_behavior_status";
/// Kick reference-command topic (`kTopicKickReference`).
pub const KICK_BALL_TOPIC: &str = "rt/kick_ball";
/// ROS-bridge odometry topic (`kTopicRosOdometer`).
pub const ROS_ODOM_TOPIC: &str = "rt/odom";
/// ROS-bridge IMU topic (`kTopicRosImu`).
pub const ROS_IMU_TOPIC: &str = "rt/imu/data";
/// ROS-bridge joint-state topic (`kTopicRosJointStates`).
pub const ROS_JOINT_STATES_TOPIC: &str = "rt/joint_states";

// ---------------------------------------------------------------------------
// RPC topic specs
// ---------------------------------------------------------------------------

pub fn rpc_request_topic(service_topic: &str) -> TopicSpec {
    TopicSpec::new(format!("{service_topic}Req"), TYPE_RPC_REQ).with_qos(qos_reliable_keep_last(10))
}

pub fn rpc_response_topic(service_topic: &str) -> TopicSpec {
    TopicSpec::new(format!("{service_topic}Resp"), TYPE_RPC_RESP)
        .with_qos(qos_reliable_transient_local_keep_last(10))
}

pub fn loco_request_topic() -> TopicSpec {
    rpc_request_topic(LOCO_API_TOPIC)
}

pub fn loco_response_topic() -> TopicSpec {
    rpc_response_topic(LOCO_API_TOPIC)
}

// ---------------------------------------------------------------------------
// Existing data topic specs
// ---------------------------------------------------------------------------

pub fn device_gateway_topic() -> TopicSpec {
    TopicSpec::new(DEVICE_GATEWAY_TOPIC, TYPE_ROBOT_STATUS)
}

pub fn motion_state_topic() -> TopicSpec {
    TopicSpec::new(MOTION_STATE_TOPIC, TYPE_MOTION_STATE)
}

pub fn battery_state_topic() -> TopicSpec {
    TopicSpec::new(BATTERY_STATE_TOPIC, TYPE_BATTERY_STATE).with_qos(qos_reliable_keep_last(1))
}

pub fn button_event_topic() -> TopicSpec {
    TopicSpec::new(BUTTON_EVENT_TOPIC, TYPE_BUTTON_EVENT).with_qos(qos_reliable_keep_all())
}

pub fn remote_controller_topic() -> TopicSpec {
    TopicSpec::new(REMOTE_CONTROLLER_STATE_TOPIC, TYPE_REMOTE_CONTROLLER)
}

pub fn process_state_topic() -> TopicSpec {
    TopicSpec::new(PROCESS_STATE_TOPIC, TYPE_PROCESS_STATE).with_qos(qos_reliable_keep_last(1))
}

pub fn video_stream_topic() -> TopicSpec {
    TopicSpec::new(VIDEO_STREAM_TOPIC, TYPE_BINARY_DATA)
}

/// See [`super::GripperControl`]: the C++ SDK 1.8 has no equivalent topic.
pub fn gripper_control_topic() -> TopicSpec {
    TopicSpec::new(GRIPPER_CONTROL_TOPIC, TYPE_GRIPPER_CONTROL).with_qos(qos_reliable_keep_last(10))
}

pub fn light_control_topic() -> TopicSpec {
    TopicSpec::new(LIGHT_CONTROL_TOPIC, TYPE_LIGHT_CONTROL).with_qos(qos_reliable_keep_last(10))
}

pub fn safe_mode_topic() -> TopicSpec {
    TopicSpec::new(SAFE_MODE_TOPIC, TYPE_SAFE_MODE).with_qos(qos_reliable_keep_all())
}

pub fn ai_subtitle_topic() -> TopicSpec {
    TopicSpec::new(AI_SUBTITLE_TOPIC, TYPE_SUBTITLE).with_qos(qos_reliable_keep_last(16))
}

pub fn lui_asr_chunk_topic() -> TopicSpec {
    TopicSpec::new(LUI_ASR_CHUNK_TOPIC, TYPE_ASR_CHUNK).with_qos(qos_reliable_keep_last(16))
}

pub fn audio_bluetooth_event_topic() -> TopicSpec {
    TopicSpec::new(AUDIO_BLUETOOTH_EVENT_TOPIC, TYPE_BLUETOOTH_EVENT)
        .with_qos(qos_reliable_keep_last(16))
}

// ---------------------------------------------------------------------------
// b1_api_const.hpp topics
// ---------------------------------------------------------------------------

/// `rt/joint_ctrl` carrying [`LowCmd`](super::LowCmd). Reliable keep-last(1),
/// matching the C++ SDK's default (reliable) data writer.
pub fn joint_ctrl_topic() -> TopicSpec {
    TopicSpec::new(JOINT_CTRL_TOPIC, TYPE_LOW_CMD).reliable()
}

/// `rt/low_state` carrying [`LowState`](super::LowState) (high rate).
pub fn low_state_topic() -> TopicSpec {
    TopicSpec::new(LOW_STATE_TOPIC, TYPE_LOW_STATE)
}

/// `rt/fall_down` carrying [`FallDownState`](super::FallDownState).
pub fn fall_down_topic() -> TopicSpec {
    TopicSpec::new(FALL_DOWN_TOPIC, TYPE_FALL_DOWN_STATE).keep_last(10)
}

/// `rt/odometer_state` carrying [`Odometer`](super::Odometer).
pub fn odometer_state_topic() -> TopicSpec {
    TopicSpec::new(ODOMETER_STATE_TOPIC, TYPE_ODOMETER)
}

/// `rt/booster_hand_data` carrying [`HandReplyData`](super::HandReplyData).
pub fn booster_hand_data_topic() -> TopicSpec {
    TopicSpec::new(BOOSTER_HAND_DATA_TOPIC, TYPE_HAND_REPLY_DATA).keep_last(4)
}

/// `rt/booster_hand_touch_data` carrying [`HandTouchData`](super::HandTouchData).
pub fn hand_touch_data_topic() -> TopicSpec {
    TopicSpec::new(HAND_TOUCH_DATA_TOPIC, TYPE_HAND_TOUCH_DATA).keep_last(4)
}

/// `rt/tf` carrying [`TFMessage`](super::TFMessage).
pub fn tf_topic() -> TopicSpec {
    TopicSpec::new(TF_TOPIC, TYPE_TF_MESSAGE).keep_last(10)
}

/// `rt/robot_states` carrying [`RobotStatesMsg`](super::RobotStatesMsg).
pub fn robot_states_topic() -> TopicSpec {
    TopicSpec::new(ROBOT_STATES_TOPIC, TYPE_ROBOT_STATES).keep_last(10)
}

/// `rt/prone_body_control_status` carrying
/// [`ProneBodyControlStatus`](super::ProneBodyControlStatus).
pub fn prone_body_control_status_topic() -> TopicSpec {
    TopicSpec::new(
        PRONE_BODY_CONTROL_STATUS_TOPIC,
        TYPE_PRONE_BODY_CONTROL_STATUS,
    )
    .keep_last(10)
}

/// `rt/robot_replay_traj_id` carrying [`RobotReplayTrajID`](super::RobotReplayTrajID).
pub fn robot_replay_traj_id_topic() -> TopicSpec {
    TopicSpec::new(ROBOT_REPLAY_TRAJ_ID_TOPIC, TYPE_ROBOT_REPLAY_TRAJ_ID).keep_last(10)
}

/// `rt/trained_traj_status` carrying [`TrainedTrajStatus`](super::TrainedTrajStatus).
pub fn trained_traj_status_topic() -> TopicSpec {
    TopicSpec::new(TRAINED_TRAJ_STATUS_TOPIC, TYPE_TRAINED_TRAJ_STATUS).keep_last(10)
}

/// `rt/robocup_behavior_status` carrying
/// [`RobocupBehaviorStatus`](super::RobocupBehaviorStatus).
pub fn robocup_behavior_status_topic() -> TopicSpec {
    TopicSpec::new(ROBOCUP_BEHAVIOR_STATUS_TOPIC, TYPE_ROBOCUP_BEHAVIOR_STATUS).keep_last(10)
}

/// `rt/kick_ball` carrying [`Kick`](super::Kick). Reliable, as it is a
/// command topic.
pub fn kick_ball_topic() -> TopicSpec {
    TopicSpec::new(KICK_BALL_TOPIC, TYPE_KICK)
        .reliable()
        .keep_last(10)
}

/// `rt/odom` carrying [`Odometry`](super::Odometry) (requires the ROS bridge).
pub fn ros_odom_topic() -> TopicSpec {
    TopicSpec::new(ROS_ODOM_TOPIC, TYPE_ODOMETRY)
}

/// `rt/imu/data` carrying [`Imu`](super::Imu) (requires the ROS bridge).
pub fn ros_imu_topic() -> TopicSpec {
    TopicSpec::new(ROS_IMU_TOPIC, TYPE_IMU)
}

/// `rt/joint_states` carrying [`JointState`](super::JointState) (requires the
/// ROS bridge).
pub fn ros_joint_states_topic() -> TopicSpec {
    TopicSpec::new(ROS_JOINT_STATES_TOPIC, TYPE_JOINT_STATE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_modifies_qos() {
        let spec = TopicSpec::new("rt/test", TYPE_LOW_STATE);
        assert!(!spec.qos.is_reliable());
        let spec = spec.reliable().keep_last(5);
        assert_eq!(spec.qos, qos_reliable_keep_last(5));
        assert!(!spec.best_effort().qos.is_reliable());
    }

    #[test]
    fn existing_specs_keep_names() {
        assert_eq!(device_gateway_topic().name, "rt/device_gateway");
        assert_eq!(rpc_request_topic(LOCO_API_TOPIC).name, "rt/LocoApiTopicReq");
        assert_eq!(
            rpc_response_topic(LOCO_API_TOPIC).name,
            "rt/LocoApiTopicResp"
        );
        assert_eq!(joint_ctrl_topic().type_name, TYPE_LOW_CMD);
    }
}
