//! Camera catalog RPC client introduced with Booster SDK 1.7.

use std::time::Duration;

use crate::{
    dds::{CAMERA_API_TOPIC, RpcClient, RpcClientOptions},
    types::{DeviceInfo, DeviceInfoKind, Result, device::CameraList},
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

    /// Target a named robot: requests go to `rt/CameraApiTopic/<robot_name>`
    /// (C++ `Init(robot_name)`). An empty name selects the default topic.
    pub fn with_robot_name(robot_name: impl Into<String>) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(CAMERA_API_TOPIC).with_robot_name(robot_name),
        )
    }

    pub fn with_startup_wait(startup_wait: Duration) -> Result<Self> {
        Self::with_options(
            RpcClientOptions::for_service(CAMERA_API_TOPIC).with_startup_wait(startup_wait),
        )
    }

    /// Create a client with custom RPC options. The service topic is always
    /// the camera topic; `options.robot_name` is honored.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        Ok(Self {
            rpc: RpcClient::for_topic(options, CAMERA_API_TOPIC)?,
        })
    }

    /// Send a raw request and discard the response body (C++ `SendApiRequest`).
    pub async fn send_api_request(&self, api_id: impl Into<i32>, param: &str) -> Result<()> {
        self.rpc.send_api_request(api_id.into(), param).await?;
        Ok(())
    }

    /// Send a raw request and return the response body
    /// (C++ `SendApiRequestWithResponse`).
    pub async fn send_api_request_with_response(
        &self,
        api_id: impl Into<i32>,
        param: &str,
    ) -> Result<String> {
        self.rpc.send_api_request(api_id.into(), param).await
    }

    /// Query the camera catalog. The response body contains a `cameras` array.
    pub async fn get_cameras(&self) -> Result<DeviceInfo> {
        let body = self.rpc.call_response(CameraApiId::GetCameras, "").await?;
        Ok(DeviceInfo::new(DeviceInfoKind::Camera, body))
    }

    /// Query the camera catalog and parse it into typed cameras
    /// (C++ `GetCameras` followed by `CameraListFromDeviceInfo`).
    pub async fn get_camera_list(&self) -> Result<CameraList> {
        CameraList::from_device_info(&self.get_cameras().await?)
    }
}
