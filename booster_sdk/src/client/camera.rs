//! Camera catalog RPC client introduced with Booster SDK 1.7.

use std::time::Duration;

use crate::{
    dds::{CAMERA_API_TOPIC, RpcClient, RpcClientOptions},
    types::{DeviceInfo, DeviceInfoKind, Result},
};

crate::api_id_enum! {
    /// Camera service RPC API identifiers.
    CameraApiId {
        GetCameras = 3100,
    }
}

/// Client for camera device discovery.
pub struct CameraClient {
    rpc: RpcClient,
}

impl CameraClient {
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(CAMERA_API_TOPIC))
    }

    pub fn with_startup_wait(startup_wait: Duration) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(CAMERA_API_TOPIC).with_startup_wait(startup_wait),
        )
    }

    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        Ok(Self {
            rpc: RpcClient::for_topic(options, CAMERA_API_TOPIC)?,
        })
    }

    /// Query the camera catalog. The response body contains a `cameras` array.
    pub async fn get_cameras(&self) -> Result<DeviceInfo> {
        let body = self.rpc.call_response(CameraApiId::GetCameras, "").await?;
        Ok(DeviceInfo::new(DeviceInfoKind::Camera, body))
    }
}
