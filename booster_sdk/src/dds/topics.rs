//! DDS topic specifications for Booster robot communication.

use rustdds::{QosPolicies, Topic, TopicKind};

use crate::types::{DdsError, Result};

use super::qos::{qos_best_effort_keep_last, qos_reliable_keep_all, qos_reliable_keep_last};

#[derive(Debug, Clone)]
pub struct TopicSpec {
    pub name: String,
    pub type_name: &'static str,
    pub qos: QosPolicies,
    pub kind: TopicKind,
}

impl TopicSpec {
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

pub const TYPE_RPC_REQ: &str = "booster_msgs::msg::dds_::RpcReqMsg_";
pub const TYPE_RPC_RESP: &str = "booster_msgs::msg::dds_::RpcRespMsg_";
pub const TYPE_ROBOT_STATUS: &str = "booster_interface::msg::dds_::RobotStatusDdsMsg_";
pub const TYPE_MOTION_STATE: &str = "booster::msg::MotionState";
pub const TYPE_BATTERY_STATE: &str = "booster_interface::msg::dds_::BatteryState_";
pub const TYPE_BUTTON_EVENT: &str = "booster_interface::msg::dds_::ButtonEventMsg_";
pub const TYPE_REMOTE_CONTROLLER: &str = "booster_interface::msg::dds_::RemoteControllerState_";
pub const TYPE_PROCESS_STATE: &str = "booster_interface::msg::dds_::RobotProcessStateMsg_";
pub const TYPE_BINARY_DATA: &str = "booster_msgs::msg::dds_::BinaryData_";
pub const TYPE_GRIPPER_CONTROL: &str = "booster_interface::msg::dds_::GripperControl_";
pub const TYPE_LIGHT_CONTROL: &str = "booster_interface::msg::dds_::LightControlMsg_";
pub const TYPE_SAFE_MODE: &str = "booster_msgs::msg::dds_::BinaryData_";
pub const TYPE_SUBTITLE: &str = "booster_interface::msg::dds_::Subtitle_";
pub const TYPE_ASR_CHUNK: &str = "booster_interface::msg::dds_::AsrChunk_";

pub const LOCO_API_TOPIC: &str = "rt/LocoApiTopic";
pub const AI_API_TOPIC: &str = "rt/AiApiTopic";
pub const LUI_API_TOPIC: &str = "rt/LuiApiTopic";
pub const LIGHT_CONTROL_API_TOPIC: &str = "rt/LightControlApiTopic";
pub const VISION_API_TOPIC: &str = "rt/VisionApiTopic";
pub const X5_CAMERA_CONTROL_API_TOPIC: &str = "rt/X5CameraControl";

pub fn rpc_request_topic(service_topic: &str) -> TopicSpec {
    TopicSpec {
        name: format!("{service_topic}Req"),
        type_name: TYPE_RPC_REQ,
        qos: qos_reliable_keep_last(10),
        kind: TopicKind::NoKey,
    }
}

pub fn rpc_response_topic(service_topic: &str) -> TopicSpec {
    TopicSpec {
        name: format!("{service_topic}Resp"),
        type_name: TYPE_RPC_RESP,
        qos: qos_reliable_keep_last(10),
        kind: TopicKind::NoKey,
    }
}

pub fn loco_request_topic() -> TopicSpec {
    rpc_request_topic(LOCO_API_TOPIC)
}

pub fn loco_response_topic() -> TopicSpec {
    rpc_response_topic(LOCO_API_TOPIC)
}

pub fn device_gateway_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/device_gateway".to_owned(),
        type_name: TYPE_ROBOT_STATUS,
        qos: qos_best_effort_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn motion_state_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/motion_state".to_owned(),
        type_name: TYPE_MOTION_STATE,
        qos: qos_best_effort_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn battery_state_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/battery_state".to_owned(),
        type_name: TYPE_BATTERY_STATE,
        qos: qos_reliable_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn button_event_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/button_event".to_owned(),
        type_name: TYPE_BUTTON_EVENT,
        qos: qos_reliable_keep_all(),
        kind: TopicKind::NoKey,
    }
}

pub fn remote_controller_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/remote_controller_state".to_owned(),
        type_name: TYPE_REMOTE_CONTROLLER,
        qos: qos_best_effort_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn process_state_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/booster_process_state".to_owned(),
        type_name: TYPE_PROCESS_STATE,
        qos: qos_reliable_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn video_stream_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/booster/video_stream".to_owned(),
        type_name: TYPE_BINARY_DATA,
        qos: qos_best_effort_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn gripper_control_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/gripper_control".to_owned(),
        type_name: TYPE_GRIPPER_CONTROL,
        qos: qos_reliable_keep_last(10),
        kind: TopicKind::NoKey,
    }
}

pub fn light_control_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/light_control".to_owned(),
        type_name: TYPE_LIGHT_CONTROL,
        qos: qos_reliable_keep_last(10),
        kind: TopicKind::NoKey,
    }
}

pub fn safe_mode_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/enter_safe_mode".to_owned(),
        type_name: TYPE_SAFE_MODE,
        qos: qos_reliable_keep_all(),
        kind: TopicKind::NoKey,
    }
}

pub fn ai_subtitle_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/ai_subtitle".to_owned(),
        type_name: TYPE_SUBTITLE,
        qos: qos_reliable_keep_last(16),
        kind: TopicKind::NoKey,
    }
}

pub fn lui_asr_chunk_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/lui_asr_chunk".to_owned(),
        type_name: TYPE_ASR_CHUNK,
        qos: qos_reliable_keep_last(16),
        kind: TopicKind::NoKey,
    }
}
