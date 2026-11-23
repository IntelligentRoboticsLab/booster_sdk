//! Typed views of the JSON device catalogs returned by the discovery RPCs.
//!
//! Ports the C++ SDK's `common/device_info_parser.hpp` and
//! `common/camera_info_parser.hpp`. The robot returns these catalogs as JSON
//! (see [`DeviceInfo`]); the types here mirror the corresponding IDL messages
//! (`booster_interface::msg::Camera`, `ImuInfo`, `Hand`, `RobotModel`, ...).
//!
//! Parsing is tolerant like the C++ parsers: missing keys and explicit `null`
//! values keep the field's default. Enum fields are encoded as integers on the
//! wire; values this SDK does not know map to the enum's default variant.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use super::{DeviceInfo, DeviceInfoKind, Result};

/// Declare a `u32` wire enum whose first variant is the default and which
/// deserializes unknown values to that default.
macro_rules! wire_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            $(#[$first_meta:meta])*
            $first:ident = $first_value:literal
            $(,
                $(#[$variant_meta:meta])*
                $variant:ident = $value:literal
            )* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        #[repr(u32)]
        pub enum $name {
            $(#[$first_meta])*
            #[default]
            $first = $first_value,
            $(
                $(#[$variant_meta])*
                $variant = $value,
            )*
        }

        impl From<$name> for u32 {
            fn from(value: $name) -> Self {
                value as u32
            }
        }

        impl TryFrom<u32> for $name {
            type Error = &'static str;

            fn try_from(value: u32) -> std::result::Result<Self, &'static str> {
                match value {
                    $first_value => Ok(Self::$first),
                    $($value => Ok(Self::$variant),)*
                    _ => Err("invalid value"),
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                serializer.serialize_u32(u32::from(*self))
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
                let value = Value::deserialize(deserializer)?;
                Ok(value
                    .as_u64()
                    .and_then(|v| u32::try_from(v).ok())
                    .and_then(|v| Self::try_from(v).ok())
                    .unwrap_or_default())
            }
        }
    };
}

wire_enum! {
    /// Camera hardware model.
    CameraModel {
        Unknown = 0,
        RealsenseD455 = 1,
        Zed2i = 2,
        OrbbecGemini335l = 3,
        BoosterMipi = 4,
    }
}

wire_enum! {
    /// Camera connection state.
    CameraConnectionState {
        Unknown = 0,
        Connected = 1,
        ConnectionLost = 2,
    }
}

wire_enum! {
    /// Camera mount position.
    CameraPos {
        Unknown = 0,
        Head = 1,
        LeftHand = 2,
        RightHand = 3,
        Waist = 4,
    }
}

wire_enum! {
    /// Pixel format of a color stream.
    CameraPixelFormat {
        Unknown = 0,
        Rgb8 = 1,
        Bgr8 = 2,
        Yuv422 = 3,
        Jpeg = 4,
        Nv12 = 5,
    }
}

wire_enum! {
    /// Pixel format of a depth stream.
    DepthPixelFormat {
        Unknown = 0,
        Uint16 = 1,
        Float32 = 2,
    }
}

wire_enum! {
    /// IMU hardware model.
    ImuModel {
        Unknown = 0,
        Hi13r4n = 1,
    }
}

wire_enum! {
    /// Hand / gripper hardware model.
    HandModel {
        Unknown = 0,
        InspireHandRh56dfx = 1,
        InspireGripperEg2_4x2 = 2,
        InspireTouchHandRh56dftp = 3,
    }
}

wire_enum! {
    /// Hand mount side.
    HandPos {
        Unknown = 0,
        Left = 1,
        Right = 2,
    }
}

wire_enum! {
    /// Collision geometry type.
    GeometryType {
        Box = 0,
        Cylinder = 1,
        Sphere = 2,
        Mesh = 3,
    }
}

wire_enum! {
    /// URDF-style joint type.
    JointType {
        Fixed = 0,
        Revolute = 1,
        Continuous = 2,
        Prismatic = 3,
        Planar = 4,
        Floating = 5,
    }
}

/// `geometry_msgs/Vector3` (also used for `geometry_msgs/Point`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// `geometry_msgs/Point`.
pub type Point = Vector3;

/// Topic names a camera publishes on.
///
/// The wire form is `{"topics": [...]}`; a bare array is accepted as well.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CameraTopic {
    pub topics: Vec<String>,
}

impl<'de> Deserialize<'de> for CameraTopic {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            List(Vec<String>),
            Object {
                #[serde(default)]
                topics: Option<Vec<String>>,
            },
        }
        Ok(match Repr::deserialize(deserializer)? {
            Repr::List(topics) => Self { topics },
            Repr::Object { topics } => Self {
                topics: topics.unwrap_or_default(),
            },
        })
    }
}

/// Camera pose relative to a parent frame.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CameraExtrinsics {
    pub parent_frame: String,
    pub xyz: Point,
    pub rpy: Vector3,
}

/// Pinhole camera intrinsics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CameraIntrinsics {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
}

/// Color stream encoding.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct VideoStreamEncoding {
    pub width: u32,
    pub height: u32,
    pub format: CameraPixelFormat,
    pub bitrate_bps: u32,
}

/// Depth stream encoding.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DepthStreamEncoding {
    pub width: u32,
    pub height: u32,
    pub format: DepthPixelFormat,
}

/// One camera from the camera catalog.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
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

impl Camera {
    /// Parse one camera RPC JSON object (C++ `CameraFromRpcJson`).
    ///
    /// A non-object value yields a default camera.
    pub fn from_rpc_json(json: &Value) -> Result<Self> {
        if !json.is_object() {
            return Ok(Self::default());
        }
        Ok(serde_json::from_value(strip_nulls(json.clone()))?)
    }
}

/// Camera catalog.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CameraList {
    pub cameras: Vec<Camera>,
}

impl CameraList {
    /// Parse a camera-list RPC body (C++ `CameraListFromRpcJson`).
    ///
    /// Accepts a bare array or `{"cameras": [...]}`; anything else yields an
    /// empty list.
    pub fn from_rpc_json(json: &Value) -> Result<Self> {
        let items = match json {
            Value::Array(items) => items,
            Value::Object(map) => match map.get("cameras") {
                Some(Value::Array(items)) => items,
                _ => return Ok(Self::default()),
            },
            _ => return Ok(Self::default()),
        };
        let cameras = items
            .iter()
            .map(Camera::from_rpc_json)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { cameras })
    }

    /// Parse a camera [`DeviceInfo`] (C++ `CameraListFromDeviceInfo`).
    ///
    /// Returns an empty list unless `info.kind` is [`DeviceInfoKind::Camera`].
    pub fn from_device_info(info: &DeviceInfo) -> Result<Self> {
        if info.kind != DeviceInfoKind::Camera {
            return Ok(Self::default());
        }
        Self::from_rpc_json(&info.body)
    }
}

/// One IMU from the sensor catalog.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ImuInfo {
    pub index: i32,
    pub name: String,
    pub mount_position: String,
    pub model: ImuModel,
}

/// IMU catalog.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ImuInfoList {
    pub imus: Vec<ImuInfo>,
}

impl ImuInfoList {
    /// Parse a sensors [`DeviceInfo`] (C++ `ImuInfoListFromDeviceInfo`).
    ///
    /// Returns an empty list unless `info.kind` is [`DeviceInfoKind::Sensors`].
    pub fn from_device_info(info: &DeviceInfo) -> Result<Self> {
        if info.kind != DeviceInfoKind::Sensors {
            return Ok(Self::default());
        }
        Ok(Self {
            imus: parse_list(&info.body, "imus")?,
        })
    }
}

/// One hand / gripper from the hand catalog.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Hand {
    pub model: HandModel,
    pub pos: HandPos,
}

/// Hand catalog.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct HandList {
    pub hands: Vec<Hand>,
}

impl HandList {
    /// Parse a hands [`DeviceInfo`] (C++ `HandListFromDeviceInfo`).
    ///
    /// Returns an empty list unless `info.kind` is [`DeviceInfoKind::Hands`].
    pub fn from_device_info(info: &DeviceInfo) -> Result<Self> {
        if info.kind != DeviceInfoKind::Hands {
            return Ok(Self::default());
        }
        Ok(Self {
            hands: parse_list(&info.body, "hands")?,
        })
    }
}

/// Pose of a frame relative to its parent.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Origin {
    pub xyz: Point,
    pub rpy: Vector3,
}

/// Inertia tensor.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Inertia {
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

/// Link inertial properties.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Inertial {
    pub origin: Origin,
    pub mass: f64,
    pub inertia: Inertia,
}

/// Box geometry.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BoxGeometry {
    pub size: Vector3,
}

/// Cylinder geometry.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Cylinder {
    pub radius: f64,
    pub length: f64,
}

/// Sphere geometry.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sphere {
    pub radius: f64,
}

/// Mesh geometry.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Mesh {
    pub filename: String,
    pub scale: Vector3,
}

/// Collision geometry; `geometry_type` selects which shape field applies.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: GeometryType,
    #[serde(rename = "box")]
    pub box_geometry: BoxGeometry,
    pub cylinder: Cylinder,
    pub sphere: Sphere,
    pub mesh: Mesh,
}

/// Collision element of a link.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Collision {
    pub origin: Origin,
    pub geometry: Geometry,
}

/// Robot link.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Link {
    pub name: String,
    pub inertial: Inertial,
    pub collisions: Vec<Collision>,
}

/// Joint limits.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JointLimit {
    pub lower: f64,
    pub upper: f64,
    pub effort: f64,
    pub velocity: f64,
}

/// Robot joint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Joint {
    pub name: String,
    #[serde(rename = "type")]
    pub joint_type: JointType,
    pub parent_link: String,
    pub child_link: String,
    pub origin: Origin,
    pub axis: Vector3,
    pub limit: JointLimit,
}

/// Kinematic / dynamic robot model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RobotModel {
    pub root_link: String,
    pub links: Vec<Link>,
    pub joints: Vec<Joint>,
}

impl RobotModel {
    /// Parse a robot-model RPC body. The model may be the body itself or be
    /// wrapped as `{"robot_model": {...}}`.
    pub fn from_rpc_json(json: &Value) -> Result<Self> {
        let model = match json.get("robot_model") {
            Some(inner @ Value::Object(_)) => inner,
            _ => json,
        };
        if !model.is_object() {
            return Ok(Self::default());
        }
        Ok(serde_json::from_value(strip_nulls(model.clone()))?)
    }

    /// Parse a robot-model [`DeviceInfo`] (C++ `RobotModelFromDeviceInfo`).
    ///
    /// Returns `None` unless `info.kind` is [`DeviceInfoKind::RobotModel`].
    pub fn from_device_info(info: &DeviceInfo) -> Result<Option<Self>> {
        if info.kind != DeviceInfoKind::RobotModel {
            return Ok(None);
        }
        Self::from_rpc_json(&info.body).map(Some)
    }
}

impl DeviceInfo {
    /// Parse a serialized device info (C++ `DeviceInfo::FromJson`).
    ///
    /// Accepts `{"kind": <int>, "body": ...}`, or a bare RPC body which is
    /// stored with kind [`DeviceInfoKind::Unknown`].
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(Self::from_value(serde_json::from_str(json)?))
    }

    /// Like [`Self::from_json`], from an already parsed value.
    #[must_use]
    pub fn from_value(mut value: Value) -> Self {
        if let Value::Object(map) = &mut value
            && map.contains_key("kind")
            && map.contains_key("body")
        {
            let kind = map
                .get("kind")
                .and_then(Value::as_i64)
                .and_then(|kind| i32::try_from(kind).ok())
                .and_then(|kind| DeviceInfoKind::try_from(kind).ok())
                .unwrap_or(DeviceInfoKind::Unknown);
            let body = map.remove("body").unwrap_or(Value::Null);
            return Self::new(kind, body);
        }
        Self::new(DeviceInfoKind::Unknown, value)
    }

    /// Serialize as `{"kind": <int>, "body": ...}` (C++ `DeviceInfo::ToJson`).
    #[must_use]
    pub fn to_json(&self) -> Value {
        serde_json::json!({ "kind": i32::from(self.kind), "body": self.body })
    }

    /// IMU list, empty unless this is a sensors catalog.
    pub fn imu_info_list(&self) -> Result<ImuInfoList> {
        ImuInfoList::from_device_info(self)
    }

    /// Hand list, empty unless this is a hands catalog.
    pub fn hand_list(&self) -> Result<HandList> {
        HandList::from_device_info(self)
    }

    /// Robot model, `None` unless this is a robot-model catalog.
    pub fn robot_model(&self) -> Result<Option<RobotModel>> {
        RobotModel::from_device_info(self)
    }

    /// Camera list, empty unless this is a camera catalog.
    pub fn camera_list(&self) -> Result<CameraList> {
        CameraList::from_device_info(self)
    }
}

/// Parse `body[key]` (or a bare array body) as a list of `T`.
fn parse_list<T: serde::de::DeserializeOwned>(body: &Value, key: &str) -> Result<Vec<T>> {
    let items = match body {
        Value::Array(items) => items,
        Value::Object(map) => match map.get(key) {
            Some(Value::Array(items)) => items,
            _ => return Ok(Vec::new()),
        },
        _ => return Ok(Vec::new()),
    };
    items
        .iter()
        .filter(|item| item.is_object())
        .map(|item| Ok(serde_json::from_value(strip_nulls(item.clone()))?))
        .collect()
}

/// Recursively drop object entries whose value is `null`, so `#[serde(default)]`
/// applies to them like the C++ parsers' `contains && !is_null` checks.
fn strip_nulls(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, strip_nulls(v)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(strip_nulls).collect()),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn camera_fixture() -> Value {
        json!({
            "name": "head_camera",
            "frame_id": "head_camera_link",
            "serial_number": "SN123",
            "pos": 1,
            "model": 3,
            "state": 1,
            "topics": {"topics": ["/camera/color", "/camera/depth"]},
            "depth_scale": 0.001,
            "extrinsics": {
                "parent_frame": "head",
                "xyz": {"x": 0.1, "y": 0.0, "z": 0.2},
                "rpy": {"x": 0.0, "y": 0.5, "z": 0.0}
            },
            "color_encoding": {"width": 1280, "height": 720, "format": 5, "bitrate_bps": 4000000},
            "depth_encoding": {"width": 640, "height": 480, "format": 1}
        })
    }

    #[test]
    fn parses_full_camera() {
        let camera = Camera::from_rpc_json(&camera_fixture()).unwrap();
        assert_eq!(camera.name, "head_camera");
        assert_eq!(camera.pos, CameraPos::Head);
        assert_eq!(camera.model, CameraModel::OrbbecGemini335l);
        assert_eq!(camera.state, CameraConnectionState::Connected);
        assert_eq!(camera.topics.topics.len(), 2);
        assert_eq!(camera.extrinsics.xyz.z, 0.2);
        assert_eq!(camera.extrinsics.rpy.y, 0.5);
        assert_eq!(camera.color_encoding.format, CameraPixelFormat::Nv12);
        assert_eq!(camera.color_encoding.bitrate_bps, 4_000_000);
        assert_eq!(camera.depth_encoding.format, DepthPixelFormat::Uint16);
    }

    #[test]
    fn camera_tolerates_missing_null_and_unknown_enum() {
        let camera = Camera::from_rpc_json(&json!({
            "name": "x",
            "serial_number": null,
            "model": 99,
            "topics": ["/a"]
        }))
        .unwrap();
        assert_eq!(camera.name, "x");
        assert_eq!(camera.serial_number, "");
        assert_eq!(camera.model, CameraModel::Unknown);
        assert_eq!(camera.topics.topics, vec!["/a".to_owned()]);
        assert_eq!(camera.color_encoding, VideoStreamEncoding::default());
    }

    #[test]
    fn camera_list_accepts_array_or_object() {
        let wrapped = CameraList::from_rpc_json(&json!({"cameras": [camera_fixture()]})).unwrap();
        let bare = CameraList::from_rpc_json(&json!([camera_fixture()])).unwrap();
        assert_eq!(wrapped, bare);
        assert_eq!(wrapped.cameras.len(), 1);
        assert!(
            CameraList::from_rpc_json(&json!({"foo": 1}))
                .unwrap()
                .cameras
                .is_empty()
        );
        assert!(
            CameraList::from_rpc_json(&json!("x"))
                .unwrap()
                .cameras
                .is_empty()
        );
    }

    #[test]
    fn device_info_kind_mismatch_yields_empty() {
        let info = DeviceInfo::new(
            DeviceInfoKind::Hands,
            json!({"cameras": [camera_fixture()]}),
        );
        assert!(info.camera_list().unwrap().cameras.is_empty());
        assert!(info.imu_info_list().unwrap().imus.is_empty());
        assert!(info.robot_model().unwrap().is_none());
        let info = DeviceInfo::new(
            DeviceInfoKind::Camera,
            json!({"cameras": [camera_fixture()]}),
        );
        assert_eq!(info.camera_list().unwrap().cameras.len(), 1);
    }

    #[test]
    fn parses_imus_and_hands() {
        let sensors = DeviceInfo::new(
            DeviceInfoKind::Sensors,
            json!({"imus": [{"index": 0, "name": "imu0", "mount_position": "torso", "model": 1}]}),
        );
        let imus = sensors.imu_info_list().unwrap().imus;
        assert_eq!(imus.len(), 1);
        assert_eq!(imus[0].model, ImuModel::Hi13r4n);
        assert_eq!(imus[0].mount_position, "torso");

        let hands = DeviceInfo::new(
            DeviceInfoKind::Hands,
            json!({"hands": [{"model": 2, "pos": 1}, {"model": 1, "pos": 2}]}),
        );
        let hands = hands.hand_list().unwrap().hands;
        assert_eq!(hands[0].model, HandModel::InspireGripperEg2_4x2);
        assert_eq!(hands[0].pos, HandPos::Left);
        assert_eq!(hands[1].pos, HandPos::Right);
    }

    #[test]
    fn parses_robot_model_wrapped_or_bare() {
        let model = json!({
            "root_link": "base",
            "links": [{
                "name": "base",
                "inertial": {"origin": {"xyz": {"x": 0, "y": 0, "z": 0.1}}, "mass": 3.5,
                              "inertia": {"ixx": 1.0, "iyy": 2.0, "izz": 3.0}},
                "collisions": [{"geometry": {"type": 1, "cylinder": {"radius": 0.1, "length": 0.4}}}]
            }],
            "joints": [{
                "name": "j1", "type": 1, "parent_link": "base", "child_link": "l1",
                "axis": {"x": 0, "y": 0, "z": 1},
                "limit": {"lower": -1.0, "upper": 1.0, "effort": 10.0, "velocity": 2.0}
            }]
        });
        let bare = DeviceInfo::new(DeviceInfoKind::RobotModel, model.clone())
            .robot_model()
            .unwrap()
            .unwrap();
        let wrapped = DeviceInfo::new(DeviceInfoKind::RobotModel, json!({"robot_model": model}))
            .robot_model()
            .unwrap()
            .unwrap();
        assert_eq!(bare, wrapped);
        assert_eq!(bare.root_link, "base");
        assert_eq!(bare.links[0].inertial.mass, 3.5);
        assert_eq!(
            bare.links[0].collisions[0].geometry.geometry_type,
            GeometryType::Cylinder
        );
        assert_eq!(bare.joints[0].joint_type, JointType::Revolute);
        assert_eq!(bare.joints[0].limit.velocity, 2.0);
    }

    #[test]
    fn device_info_from_json_wrapped_or_bare() {
        let info = DeviceInfo::from_json(r#"{"kind": 3, "body": {"cameras": []}}"#).unwrap();
        assert_eq!(info.kind, DeviceInfoKind::Camera);
        assert_eq!(info.body, json!({"cameras": []}));
        assert_eq!(info.to_json(), json!({"kind": 3, "body": {"cameras": []}}));

        let info = DeviceInfo::from_json(r#"{"imus": []}"#).unwrap();
        assert_eq!(info.kind, DeviceInfoKind::Unknown);
        assert_eq!(info.body, json!({"imus": []}));
    }
}
