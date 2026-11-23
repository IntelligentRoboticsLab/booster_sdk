//! Enums from the device-description IDLs (`Hand.h`, `ImuInfo.h`,
//! `RobotModel.h`, `camera/Camera.h`).
//!
//! All of them are 32-bit IDL enums; they (de)serialize as plain integers
//! (CDR-compatible with the C++ `uint32_t` enums, and parseable from JSON
//! device catalogs).

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
