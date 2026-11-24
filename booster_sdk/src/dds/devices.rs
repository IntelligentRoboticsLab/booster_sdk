//! Device-description messages: cameras (`idl/camera/Camera.h`), hands
//! (`idl/b1/Hand.h`), IMUs (`idl/b1/ImuInfo.h`) and the robot kinematic model
//! (`idl/b1/RobotModel.h`).
//!
//! Field names match the IDL member names (without the `m_` prefix) and enums
//! (de)serialize from integers (CDR-compatible with the C++ `uint32_t` enums).
//!
//! These are the DDS wire types. For tolerant parsing of the JSON device
//! catalogs returned by the RPC services (missing/null keys, unknown enum
//! values), use [`crate::types::device`] instead.

use serde::{Deserialize, Serialize};

use super::ros::{Point, Vector3};
use crate::types::EnumValue;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

crate::api_id_enum! {
    /// Hand hardware model (`Hand::model`).
    #[derive(Default)]
    pub HandModel {
        #[default]
        Unknown = 0,
        InspireHandRh56dfx = 1,
        InspireGripperEg24x2 = 2,
        InspireTouchHandRh56dftp = 3,
    }
}

crate::api_id_enum! {
    /// Hand mount side (`Hand::pos`).
    #[derive(Default)]
    pub HandPos {
        #[default]
        Unknown = 0,
        Left = 1,
        Right = 2,
    }
}

crate::api_id_enum! {
    /// IMU hardware model (`ImuInfo::model`).
    #[derive(Default)]
    pub ImuModel {
        #[default]
        Unknown = 0,
        Hi13r4n = 1,
    }
}

crate::api_id_enum! {
    /// Camera hardware model (`Camera::model`).
    #[derive(Default)]
    pub CameraModel {
        #[default]
        Unknown = 0,
        RealsenseD455 = 1,
        Zed2i = 2,
        OrbbecGemini335l = 3,
        BoosterMipi = 4,
    }
}

crate::api_id_enum! {
    /// Camera connection state (`Camera::state`).
    #[derive(Default)]
    pub CameraConnectionState {
        #[default]
        Unknown = 0,
        Connected = 1,
        ConnectionLost = 2,
    }
}

crate::api_id_enum! {
    /// Camera mount position (`Camera::pos`).
    #[derive(Default)]
    pub CameraPos {
        #[default]
        Unknown = 0,
        Head = 1,
        LeftHand = 2,
        RightHand = 3,
        Waist = 4,
    }
}

crate::api_id_enum! {
    /// Color stream pixel format (`VideoStreamEncoding::format`).
    #[derive(Default)]
    pub CameraPixelFormat {
        #[default]
        Unknown = 0,
        Rgb8 = 1,
        Bgr8 = 2,
        Yuv422 = 3,
        Jpeg = 4,
        Nv12 = 5,
    }
}

crate::api_id_enum! {
    /// Depth stream pixel format (`DepthStreamEncoding::format`).
    #[derive(Default)]
    pub DepthPixelFormat {
        #[default]
        Unknown = 0,
        Uint16 = 1,
        Float32 = 2,
    }
}

crate::api_id_enum! {
    /// Collision geometry kind (`Geometry::type`).
    #[derive(Default)]
    pub GeometryType {
        #[default]
        Box = 0,
        Cylinder = 1,
        Sphere = 2,
        Mesh = 3,
    }
}

crate::api_id_enum! {
    /// URDF-style joint kind (`Joint::type`).
    #[derive(Default)]
    pub JointType {
        #[default]
        Fixed = 0,
        Revolute = 1,
        Continuous = 2,
        Prismatic = 3,
        Planar = 4,
        Floating = 5,
    }
}

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
    pub format: EnumValue<CameraPixelFormat>,
    pub bitrate_bps: u32,
}

/// Depth stream encoding (`booster_interface::msg::DepthStreamEncoding`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DepthStreamEncoding {
    pub width: u32,
    pub height: u32,
    pub format: EnumValue<DepthPixelFormat>,
}

/// Camera description (`booster_interface::msg::Camera`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Camera {
    pub name: String,
    pub frame_id: String,
    pub serial_number: String,
    pub pos: EnumValue<CameraPos>,
    pub model: EnumValue<CameraModel>,
    pub state: EnumValue<CameraConnectionState>,
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
    pub model: EnumValue<HandModel>,
    pub pos: EnumValue<HandPos>,
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
    pub model: EnumValue<ImuModel>,
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
    pub r#type: EnumValue<GeometryType>,
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
    pub r#type: EnumValue<JointType>,
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
