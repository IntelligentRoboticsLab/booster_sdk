//! ROS 2 compatible message types (`builtin_interfaces`, `std_msgs`,
//! `geometry_msgs`, `nav_msgs`, `sensor_msgs`, `tf2_msgs`).
//!
//! Field order and types follow the IDL headers shipped with the Booster C++
//! SDK, which in turn match the standard ROS 2 message definitions, so these
//! are CDR-compatible with ROS 2 nodes as well.

use serde::{Deserialize, Serialize};

use super::serde_array;

// ---------------------------------------------------------------------------
// builtin_interfaces / std_msgs
// ---------------------------------------------------------------------------

/// `builtin_interfaces::msg::Time`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Time {
    pub sec: i32,
    pub nanosec: u32,
}

impl Time {
    /// Current wall-clock time (UNIX epoch based).
    #[must_use]
    pub fn now() -> Self {
        let elapsed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        Self {
            sec: i32::try_from(elapsed.as_secs()).unwrap_or(i32::MAX),
            nanosec: elapsed.subsec_nanos(),
        }
    }

    /// Time as floating-point seconds.
    #[must_use]
    pub fn as_secs_f64(&self) -> f64 {
        f64::from(self.sec) + f64::from(self.nanosec) * 1e-9
    }
}

/// `std_msgs::msg::Header`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Header {
    pub stamp: Time,
    pub frame_id: String,
}

// ---------------------------------------------------------------------------
// geometry_msgs
// ---------------------------------------------------------------------------

/// `geometry_msgs::msg::Point`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// `geometry_msgs::msg::Vector3`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// `geometry_msgs::msg::Quaternion`. Defaults to the identity rotation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

/// `geometry_msgs::msg::Pose`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Pose {
    pub position: Point,
    pub orientation: Quaternion,
}

/// `geometry_msgs::msg::PoseStamped`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PoseStamped {
    pub header: Header,
    pub pose: Pose,
}

/// `geometry_msgs::msg::PoseWithCovariance` (row-major 6x6 covariance).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PoseWithCovariance {
    pub pose: Pose,
    #[serde(with = "serde_array")]
    pub covariance: [f64; 36],
}

impl Default for PoseWithCovariance {
    fn default() -> Self {
        Self {
            pose: Pose::default(),
            covariance: [0.0; 36],
        }
    }
}

/// `geometry_msgs::msg::Transform`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

/// `geometry_msgs::msg::TransformStamped`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TransformStamped {
    pub header: Header,
    pub child_frame_id: String,
    pub transform: Transform,
}

/// `geometry_msgs::msg::Twist`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}

/// `geometry_msgs::msg::TwistWithCovariance` (row-major 6x6 covariance).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TwistWithCovariance {
    pub twist: Twist,
    #[serde(with = "serde_array")]
    pub covariance: [f64; 36],
}

impl Default for TwistWithCovariance {
    fn default() -> Self {
        Self {
            twist: Twist::default(),
            covariance: [0.0; 36],
        }
    }
}

// ---------------------------------------------------------------------------
// nav_msgs / tf2_msgs
// ---------------------------------------------------------------------------

/// `nav_msgs::msg::Odometry` (published on `rt/odom` by the ROS bridge).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Odometry {
    pub header: Header,
    pub child_frame_id: String,
    pub pose: PoseWithCovariance,
    pub twist: TwistWithCovariance,
}

/// `tf2_msgs::msg::TFMessage` (published on `rt/tf`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TFMessage {
    pub transforms: Vec<TransformStamped>,
}

// ---------------------------------------------------------------------------
// sensor_msgs
// ---------------------------------------------------------------------------

/// `sensor_msgs::msg::Imu` (published on `rt/imu/data` by the ROS bridge).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Imu {
    pub header: Header,
    pub orientation: Quaternion,
    pub orientation_covariance: [f64; 9],
    pub angular_velocity: Vector3,
    pub angular_velocity_covariance: [f64; 9],
    pub linear_acceleration: Vector3,
    pub linear_acceleration_covariance: [f64; 9],
}

/// `sensor_msgs::msg::JointState` (published on `rt/joint_states` by the ROS
/// bridge).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JointState {
    pub header: Header,
    pub name: Vec<String>,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub effort: Vec<f64>,
}

/// `sensor_msgs::msg::Image`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Image {
    pub header: Header,
    pub height: u32,
    pub width: u32,
    pub encoding: String,
    pub is_bigendian: u8,
    pub step: u32,
    pub data: Vec<u8>,
}

/// `sensor_msgs::msg::CompressedImage`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompressedImage {
    pub header: Header,
    pub format: String,
    pub data: Vec<u8>,
}

/// `sensor_msgs::msg::RegionOfInterest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RegionOfInterest {
    pub x_offset: u32,
    pub y_offset: u32,
    pub height: u32,
    pub width: u32,
    pub do_rectify: bool,
}

/// `sensor_msgs::msg::CameraInfo`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CameraInfo {
    pub header: Header,
    pub height: u32,
    pub width: u32,
    pub distortion_model: String,
    pub d: Vec<f64>,
    pub k: [f64; 9],
    pub r: [f64; 9],
    pub p: [f64; 12],
    pub binning_x: u32,
    pub binning_y: u32,
    pub roi: RegionOfInterest,
}
