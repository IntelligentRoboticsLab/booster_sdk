//! Device-description messages: cameras (`idl/camera/Camera.h`), hands
//! (`idl/b1/Hand.h`), IMUs (`idl/b1/ImuInfo.h`) and the robot kinematic model
//! (`idl/b1/RobotModel.h`).
//!
//! Field names match the IDL member names (without the `m_` prefix) and enums
//! (de)serialize from integers, so these types can be used both for DDS (CDR)
//! and for parsing the JSON device catalogs returned by the robot's RPC
//! services.

use serde::{Deserialize, Serialize};

use crate::types::{
    CameraConnectionState, CameraModel, CameraPixelFormat, CameraPos, DepthPixelFormat,
    GeometryType, HandModel, HandPos, ImuModel, JointType,
};

use super::ros::{Point, Vector3};

// ---------------------------------------------------------------------------
// Cameras
// ---------------------------------------------------------------------------

/// Topics published by a camera (`booster_interface::msg::CameraTopic`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CameraTopic {
    pub topics: Vec<String>,
}

/// Camera mounting pose relative to `parent_frame`
/// (`booster_interface::msg::CameraExtrinsics`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CameraExtrinsics {
    pub parent_frame: String,
    pub xyz: Point,
    pub rpy: Vector3,
}

/// Pinhole intrinsics (`booster_interface::msg::CameraIntrinsics`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct CameraIntrinsics {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
}

/// Color stream encoding (`booster_interface::msg::VideoStreamEncoding`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct VideoStreamEncoding {
    pub width: u32,
    pub height: u32,
    pub format: CameraPixelFormat,
    pub bitrate_bps: u32,
}

/// Depth stream encoding (`booster_interface::msg::DepthStreamEncoding`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DepthStreamEncoding {
    pub width: u32,
    pub height: u32,
    pub format: DepthPixelFormat,
}

/// Camera description (`booster_interface::msg::Camera`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Camera {
    pub name: String,
    pub frame_id: String,
    pub serial_number: String,
    pub pos: CameraPos,
    pub model: CameraModel,
    pub state: CameraConnectionState,
    pub topics: CameraTopic,
    pub depth_scale: f64,
    pub extrinsics: CameraExtrinsics,
    pub color_encoding: VideoStreamEncoding,
    pub depth_encoding: DepthStreamEncoding,
}

/// List of cameras (`booster_interface::msg::CameraList`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CameraList {
    pub cameras: Vec<Camera>,
}

// ---------------------------------------------------------------------------
// Hands and IMUs
// ---------------------------------------------------------------------------

/// Installed hand description (`booster_interface::msg::Hand`).
///
/// Not to be confused with [`crate::types::Hand`], which selects a side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Hand {
    pub model: HandModel,
    pub pos: HandPos,
}

/// List of installed hands (`booster_interface::msg::HandList`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HandList {
    pub hands: Vec<Hand>,
}

/// IMU description (`booster_interface::msg::ImuInfo`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImuInfo {
    pub index: i32,
    pub name: String,
    pub mount_position: String,
    pub model: ImuModel,
}

/// List of IMUs (`booster_interface::msg::ImuInfoList`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ImuInfoList {
    pub imus: Vec<ImuInfo>,
}

// ---------------------------------------------------------------------------
// Robot model (URDF-like)
// ---------------------------------------------------------------------------

/// Pose of a frame relative to its parent (`booster_interface::msg::Origin`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Origin {
    pub xyz: Point,
    pub rpy: Vector3,
}

/// Inertia tensor (`booster_interface::msg::Inertia`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Inertia {
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

/// Link inertial properties (`booster_interface::msg::Inertial`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Inertial {
    pub origin: Origin,
    pub mass: f64,
    pub inertia: Inertia,
}

/// Box collision shape (`booster_interface::msg::Box`; renamed to avoid
/// shadowing `std::boxed::Box`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct BoxShape {
    pub size: Vector3,
}

/// Cylinder collision shape (`booster_interface::msg::Cylinder`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Cylinder {
    pub radius: f64,
    pub length: f64,
}

/// Sphere collision shape (`booster_interface::msg::Sphere`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Sphere {
    pub radius: f64,
}

/// Mesh collision shape (`booster_interface::msg::Mesh`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Mesh {
    pub filename: String,
    pub scale: Vector3,
}

/// Collision geometry; only the member selected by `type` is meaningful
/// (`booster_interface::msg::Geometry`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Geometry {
    pub r#type: GeometryType,
    #[serde(rename = "box")]
    pub box_: BoxShape,
    pub cylinder: Cylinder,
    pub sphere: Sphere,
    pub mesh: Mesh,
}

/// Collision element (`booster_interface::msg::Collision`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Collision {
    pub origin: Origin,
    pub geometry: Geometry,
}

/// Robot link (`booster_interface::msg::Link`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub inertial: Inertial,
    pub collisions: Vec<Collision>,
}

/// Joint limits (`booster_interface::msg::JointLimit`).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct JointLimit {
    pub lower: f64,
    pub upper: f64,
    pub effort: f64,
    pub velocity: f64,
}

/// Robot joint (`booster_interface::msg::Joint`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Joint {
    pub name: String,
    pub r#type: JointType,
    pub parent_link: String,
    pub child_link: String,
    pub origin: Origin,
    pub axis: Vector3,
    pub limit: JointLimit,
}

/// Robot kinematic/dynamic model (`booster_interface::msg::RobotModel`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RobotModel {
    pub root_link: String,
    pub links: Vec<Link>,
    pub joints: Vec<Joint>,
}
