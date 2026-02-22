//! X5 camera control RPC client.

use serde::{Deserialize, Serialize};

use crate::dds::{RpcClient, RpcClientOptions, X5_CAMERA_CONTROL_API_TOPIC};
use crate::types::Result;

crate::api_id_enum! {
    /// X5 camera RPC API identifiers.
    X5CameraApiId {
        ChangeMode = 5001,
        GetStatus = 5002,
    }
}

crate::api_id_enum! {
    /// Requested X5 camera mode.
    CameraSetMode {
        CameraModeNormal = 0,
        CameraModeHighResolution = 1,
        CameraModeNormalEnable = 2,
        CameraModeHighResolutionEnable = 3,
    }
}

crate::api_id_enum! {
    /// Reported X5 camera status values.
    CameraControlStatus {
        CameraStatusNormal = 0,
        CameraStatusHighResolution = 1,
        CameraStatusError = 2,
        CameraStatusNull = 3,
    }
}

/// Parameters for camera mode changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeModeParameter {
    pub mode: i32,
}

/// Response payload for camera status requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetStatusResponse {
    pub status: i32,
}

impl GetStatusResponse {
    /// Convert the raw integer status into the enum form.
    #[must_use]
    pub fn status_enum(&self) -> Option<CameraControlStatus> {
        CameraControlStatus::try_from(self.status).ok()
    }
}

/// High-level RPC client for X5 camera control APIs.
pub struct X5CameraClient {
    rpc: RpcClient,
}

impl X5CameraClient {
    /// Create an X5 camera client with default options.
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(X5_CAMERA_CONTROL_API_TOPIC))
    }

    /// Create an X5 camera client with custom RPC options.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::for_topic(options, X5_CAMERA_CONTROL_API_TOPIC)?;
        Ok(Self { rpc })
    }

    /// Change the camera mode.
    pub async fn change_mode(&self, mode: CameraSetMode) -> Result<()> {
        let param = ChangeModeParameter {
            mode: i32::from(mode),
        };
        self.rpc
            .call_serialized(X5CameraApiId::ChangeMode, &param)
            .await
    }

    /// Read the current camera status.
    pub async fn get_status(&self) -> Result<GetStatusResponse> {
        self.rpc.call_response(X5CameraApiId::GetStatus, "").await
    }
}
