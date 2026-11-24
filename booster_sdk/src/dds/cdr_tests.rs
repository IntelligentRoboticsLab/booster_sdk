//! CDR layout and round-trip tests for the DDS message types.

use byteorder::LittleEndian;
use rustdds::cdr_encoding::{from_bytes, to_vec};
use serde::{Serialize, de::DeserializeOwned};

use super::*;
use crate::dds::{CameraModel, CameraPos, JointType};
use crate::types::{ButtonEventType, CmdType, FallDownStateType};

fn round_trip<T>(value: &T) -> (T, Vec<u8>)
where
    T: Serialize + DeserializeOwned,
{
    let bytes = to_vec::<T, LittleEndian>(value).expect("serialize");
    let (decoded, consumed) = from_bytes::<T, LittleEndian>(&bytes).expect("deserialize");
    assert_eq!(consumed, bytes.len());
    (decoded, bytes)
}

#[test]
fn motor_state_layout() {
    let state = MotorState {
        mode: 1,
        q: 0.5,
        dq: -1.0,
        ddq: 2.0,
        tau_est: 3.0,
        temperature: 42,
        lost: 7,
        reserve: [8, 9],
    };
    let (decoded, bytes) = round_trip(&state);
    assert_eq!(decoded, state);
    // u8 + pad(3) + 4*f32 + u8 + pad(3) + u32 + 2*u32
    assert_eq!(bytes.len(), 36);
    assert_eq!(bytes[0], 1);
    assert_eq!(&bytes[4..8], &0.5f32.to_le_bytes());
    assert_eq!(bytes[20], 42);
    assert_eq!(&bytes[24..28], &7u32.to_le_bytes());
    assert_eq!(&bytes[32..36], &9u32.to_le_bytes());
}

#[test]
fn low_state_round_trip() {
    let state = LowState {
        imu_state: ImuState {
            rpy: [0.1, 0.2, 0.3],
            gyro: [1.0, 2.0, 3.0],
            acc: [0.0, 0.0, 9.81],
        },
        motor_state_parallel: vec![MotorState::default(); 3],
        motor_state_serial: (0u8..23)
            .map(|i| MotorState {
                mode: i,
                q: f32::from(i),
                ..MotorState::default()
            })
            .collect(),
    };
    let (decoded, bytes) = round_trip(&state);
    assert_eq!(decoded, state);
    // imu (9 * f32, no length prefix) + 2 sequences (u32 length + 36 bytes each)
    assert_eq!(bytes.len(), 36 + 4 + 3 * 36 + 4 + 23 * 36);
    assert_eq!(&bytes[36..40], &3u32.to_le_bytes());
}

#[test]
fn low_cmd_layout() {
    let mut cmd = LowCmd::new(CmdType::Serial, 2);
    cmd.motor_cmd[1].kp = 40.0;
    let (decoded, bytes) = round_trip(&cmd);
    assert_eq!(decoded, cmd);
    // enum as u32, sequence length, 2 * (u8 + pad(3) + 6 * f32)
    assert_eq!(&bytes[0..4], &1u32.to_le_bytes());
    assert_eq!(&bytes[4..8], &2u32.to_le_bytes());
    assert_eq!(bytes.len(), 8 + 2 * 28);
}

#[test]
fn enum_fields_are_32_bit() {
    let msg = ButtonEventMsg {
        button: 3,
        event: ButtonEventType::LongPressEnd.into(),
    };
    let (decoded, bytes) = round_trip(&msg);
    assert_eq!(decoded, msg);
    assert_eq!(bytes, [3, 0, 0, 0, 7, 0, 0, 0]);

    let fall = FallDownState {
        fall_down_state: FallDownStateType::HasFallen.into(),
        is_recovery_available: true,
    };
    let (decoded, bytes) = round_trip(&fall);
    assert_eq!(decoded, fall);
    assert_eq!(bytes, [2, 0, 0, 0, 1]);
}

#[test]
fn unknown_enum_value_keeps_the_sample() {
    // A button event kind added by newer firmware (value 99).
    let bytes = [3, 0, 0, 0, 99, 0, 0, 0];
    let (msg, _) = from_bytes::<ButtonEventMsg, LittleEndian>(&bytes).expect("deserialize");
    assert_eq!(msg.button, 3);
    assert_eq!(msg.event.get(), None);
    assert_eq!(msg.event.raw(), 99);
}

#[test]
fn battery_state_layout() {
    let state = BatteryState {
        voltage: 48.0,
        current: 1.5,
        soc: 80.0,
        average_voltage: 47.5,
    };
    let (decoded, bytes) = round_trip(&state);
    assert_eq!(decoded, state);
    assert_eq!(bytes.len(), 16);
    assert_eq!(&bytes[12..16], &47.5f32.to_le_bytes());
}

#[test]
fn covariance_arrays_have_no_length_prefix() {
    let mut odom = Odometry {
        header: Header {
            stamp: Time { sec: 1, nanosec: 2 },
            frame_id: "odom".into(),
        },
        child_frame_id: "base_link".into(),
        ..Odometry::default()
    };
    odom.pose.covariance[35] = 1.0;
    odom.twist.covariance[0] = 2.0;
    let (decoded, _) = round_trip(&odom);
    assert_eq!(decoded, odom);

    let (_, bytes) = round_trip(&PoseWithCovariance::default());
    assert_eq!(bytes.len(), 7 * 8 + 36 * 8);
}

#[test]
fn tf_and_camera_info_round_trip() {
    let tf = TFMessage {
        transforms: vec![TransformStamped {
            child_frame_id: "head".into(),
            ..TransformStamped::default()
        }],
    };
    assert_eq!(round_trip(&tf).0, tf);

    let info = CameraInfo {
        d: vec![0.1, 0.2],
        p: [1.0; 12],
        ..CameraInfo::default()
    };
    assert_eq!(round_trip(&info).0, info);
}

#[test]
fn robot_model_round_trip() {
    let model = RobotModel {
        root_link: "trunk".into(),
        links: vec![Link {
            name: "trunk".into(),
            collisions: vec![Collision::default()],
            ..Link::default()
        }],
        joints: vec![Joint {
            name: "head_yaw".into(),
            r#type: JointType::Revolute.into(),
            ..Joint::default()
        }],
    };
    assert_eq!(round_trip(&model).0, model);
}

#[test]
fn device_structs_parse_json_with_integer_enums() {
    let json = r#"{
        "name": "head",
        "frame_id": "head_camera",
        "serial_number": "123",
        "pos": 1,
        "model": 4,
        "state": 1,
        "topics": {"topics": ["rt/camera/color"]},
        "depth_scale": 0.001,
        "extrinsics": {"parent_frame": "head", "xyz": {"x": 0.0, "y": 0.0, "z": 0.1}, "rpy": {"x": 0.0, "y": 0.0, "z": 0.0}},
        "color_encoding": {"width": 640, "height": 480, "format": 5, "bitrate_bps": 0},
        "depth_encoding": {"width": 0, "height": 0, "format": 0}
    }"#;
    let camera: Camera = serde_json::from_str(json).expect("camera json");
    assert_eq!(camera.pos, CameraPos::Head);
    assert_eq!(camera.model, CameraModel::BoosterMipi);
    assert_eq!(round_trip(&camera).0, camera);
}

#[test]
fn robot_status_round_trip() {
    let msg = RobotStatusDdsMsg {
        joint_vec: vec![RobotDdsJointStatus {
            name: "j".into(),
            is_limited: 1,
            ..RobotDdsJointStatus::default()
        }],
        imu_vec: vec![RobotDdsImuStatus::default()],
        battery_vec: vec![RobotDdsBatteryStatus {
            name: "battery".into(),
            voltage: 48.0,
            ..RobotDdsBatteryStatus::default()
        }],
    };
    assert_eq!(round_trip(&msg).0, msg);
}
