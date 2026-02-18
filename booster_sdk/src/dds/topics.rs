//! DDS topic specifications for Booster robot communication.

use rustdds::{Topic, TopicKind};

use crate::types::{DdsError, Result};

use super::qos::{qos_best_effort_keep_last, qos_reliable_keep_all, qos_reliable_keep_last};
use rustdds::QosPolicies;

#[derive(Debug, Clone)]
pub struct TopicSpec {
    pub name: &'static str,
    pub type_name: &'static str,
    pub qos: QosPolicies,
    pub kind: TopicKind,
}

impl TopicSpec {
    pub fn create_topic(&self, participant: &rustdds::DomainParticipant) -> Result<Topic> {
        participant
            .create_topic(
                self.name.to_string(),
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

pub fn loco_request_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/LocoApiTopicReq",
        type_name: TYPE_RPC_REQ,
        qos: qos_reliable_keep_last(10),
        kind: TopicKind::NoKey,
    }
}

pub fn loco_response_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/LocoApiTopicResp",
        type_name: TYPE_RPC_RESP,
        qos: qos_reliable_keep_last(10),
        kind: TopicKind::NoKey,
    }
}

pub fn device_gateway_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/device_gateway",
        type_name: TYPE_ROBOT_STATUS,
        qos: qos_best_effort_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn motion_state_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/motion_state",
        type_name: TYPE_MOTION_STATE,
        qos: qos_best_effort_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn battery_state_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/battery_state",
        type_name: TYPE_BATTERY_STATE,
        qos: qos_reliable_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn button_event_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/button_event",
        type_name: TYPE_BUTTON_EVENT,
        qos: qos_reliable_keep_all(),
        kind: TopicKind::NoKey,
    }
}

pub fn remote_controller_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/remote_controller_state",
        type_name: TYPE_REMOTE_CONTROLLER,
        qos: qos_best_effort_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn process_state_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/booster_process_state",
        type_name: TYPE_PROCESS_STATE,
        qos: qos_reliable_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn video_stream_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/booster/video_stream",
        type_name: TYPE_BINARY_DATA,
        qos: qos_best_effort_keep_last(1),
        kind: TopicKind::NoKey,
    }
}

pub fn gripper_control_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/gripper_control",
        type_name: TYPE_GRIPPER_CONTROL,
        qos: qos_reliable_keep_last(10),
        kind: TopicKind::NoKey,
    }
}

pub fn light_control_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/light_control",
        type_name: TYPE_LIGHT_CONTROL,
        qos: qos_reliable_keep_last(10),
        kind: TopicKind::NoKey,
    }
}

pub fn safe_mode_topic() -> TopicSpec {
    TopicSpec {
        name: "rt/enter_safe_mode",
        type_name: TYPE_SAFE_MODE,
        qos: qos_reliable_keep_all(),
        kind: TopicKind::NoKey,
    }
}
