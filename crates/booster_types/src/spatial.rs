//! Spatial data types powered by `glam`.
//!
//! This module re-exports `glam` primitives for representing 3D poses and
//! provides thin wrappers that keep the previous posture/transform shapes.

use glam::{EulerRot, Quat, Vec3};
use serde::{Deserialize, Serialize};

/// 3D position expressed as a glam vector.
pub type Position = Vec3;

/// 3D orientation expressed as Euler angles (roll, pitch, yaw) in radians.
pub type Orientation = Vec3;

/// Quaternion rotation using glam's implementation.
pub type Quaternion = Quat;

/// Combined position and Euler-angle orientation (pose).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Posture {
    #[serde(with = "serde_vec3_xyz")]
    pub position: Vec3,

    /// Orientation stored as roll/pitch/yaw (extrinsic XYZ) in radians.
    #[serde(with = "serde_vec3_xyz")]
    pub orientation: Vec3,
}

impl Posture {
    /// Create a new posture from position and orientation vectors.
    #[inline]
    #[must_use]
    pub const fn new(position: Vec3, orientation: Vec3) -> Self {
        Self {
            position,
            orientation,
        }
    }

    /// Identity posture at the origin with zero rotation.
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        orientation: Vec3::ZERO,
    };
}

/// 3D transformation (translation + quaternion rotation).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    /// Translation vector.
    #[serde(with = "serde_vec3_xyz")]
    pub position: Vec3,

    /// Rotation as a quaternion.
    #[serde(with = "serde_quat_xyzw")]
    pub rotation: Quat,
}

impl Transform {
    /// Create a new transform from position and rotation.
    #[inline]
    #[must_use]
    pub const fn new(position: Vec3, rotation: Quat) -> Self {
        Self { position, rotation }
    }

    /// Identity transform with zero translation and unit quaternion.
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };

    /// Create a transform from a posture by converting Euler angles to a quaternion.
    #[inline]
    #[must_use]
    pub fn from_posture(posture: &Posture) -> Self {
        Self {
            position: posture.position,
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                posture.orientation.x,
                posture.orientation.y,
                posture.orientation.z,
            ),
        }
    }

    /// Convert this transform back to a posture (Euler angles).
    #[inline]
    #[must_use]
    pub fn to_posture(&self) -> Posture {
        let (roll, pitch, yaw) = self.rotation.to_euler(EulerRot::XYZ);
        Posture {
            position: self.position,
            orientation: Vec3::new(roll, pitch, yaw),
        }
    }

    /// Compute the inverse transform.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        let inverse_rotation = self.rotation.conjugate();

        // Apply the inverse rotation to the negated translation.
        let inverse_translation = inverse_rotation * -self.position;
        Self {
            position: inverse_translation,
            rotation: inverse_rotation,
        }
    }

    /// Transform a point expressed in this transform's local frame.
    #[inline]
    #[must_use]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.rotation * point + self.position
    }
}

mod serde_vec3_xyz {
    use glam::Vec3;
    use serde::ser::SerializeStruct;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Vec3, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Vec3", 3)?;
        state.serialize_field("x", &value.x)?;
        state.serialize_field("y", &value.y)?;
        state.serialize_field("z", &value.z)?;
        state.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec3, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            x: f32,
            y: f32,
            z: f32,
        }

        let Helper { x, y, z } = Helper::deserialize(deserializer)?;
        Ok(Vec3::new(x, y, z))
    }
}

mod serde_quat_xyzw {
    use glam::Quat;
    use serde::ser::SerializeStruct;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Quat, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Quat", 4)?;
        state.serialize_field("x", &value.x)?;
        state.serialize_field("y", &value.y)?;
        state.serialize_field("z", &value.z)?;
        state.serialize_field("w", &value.w)?;
        state.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Quat, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            x: f32,
            y: f32,
            z: f32,
            w: f32,
        }

        let Helper { x, y, z, w } = Helper::deserialize(deserializer)?;
        Ok(Quat::from_xyzw(x, y, z, w))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Position::ZERO;
        let p2 = Position::new(3.0, 4.0, 0.0);
        let dist = p1.distance(p2);
        assert!((dist - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_quaternion_euler_conversion() {
        let orientation = Orientation::new(0.1, 0.2, 0.3);
        let transform = Transform::from_posture(&Posture::new(Position::ZERO, orientation));
        let posture = transform.to_posture();

        assert!((orientation.x - posture.orientation.x).abs() < 0.001);
        assert!((orientation.y - posture.orientation.y).abs() < 0.001);
        assert!((orientation.z - posture.orientation.z).abs() < 0.001);
    }

    #[test]
    fn test_transform_inverse() {
        let transform = Transform::new(
            Position::new(1.0, 2.0, 3.0),
            Quaternion::from_euler(EulerRot::XYZ, 0.2, -0.1, 0.4),
        );

        let inv = transform.inverse();
        let point = Position::new(0.5, -0.3, 1.2);

        let transformed = transform.transform_point(point);
        let recovered = inv.transform_point(transformed);

        assert!((recovered - point).length() < 1e-4);
    }
}
