//! X5 camera control RPC client.

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::dds::{RpcClient, RpcClientOptions, X5_CAMERA_CONTROL_API_TOPIC};
use crate::types::Result;

use super::util::{EmptyResponse, serialize_param};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum X5CameraApiId {
    ChangeMode = 5001,
    GetStatus = 5002,
}

impl From<X5CameraApiId> for i32 {
    fn from(value: X5CameraApiId) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for X5CameraApiId {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            5001 => Ok(Self::ChangeMode),
            5002 => Ok(Self::GetStatus),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum CameraSetMode {
    CameraModeNormal = 0,
    CameraModeHighResolution = 1,
    CameraModeNormalEnable = 2,
    CameraModeHighResolutionEnable = 3,
}

impl From<CameraSetMode> for i32 {
    fn from(value: CameraSetMode) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for CameraSetMode {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CameraModeNormal),
            1 => Ok(Self::CameraModeHighResolution),
            2 => Ok(Self::CameraModeNormalEnable),
            3 => Ok(Self::CameraModeHighResolutionEnable),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum CameraControlStatus {
    CameraStatusNormal = 0,
    CameraStatusHighResolution = 1,
    CameraStatusError = 2,
    CameraStatusNull = 3,
}

impl From<CameraControlStatus> for i32 {
    fn from(value: CameraControlStatus) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for CameraControlStatus {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CameraStatusNormal),
            1 => Ok(Self::CameraStatusHighResolution),
            2 => Ok(Self::CameraStatusError),
            3 => Ok(Self::CameraStatusNull),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeModeParameter {
    pub mode: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetStatusResponse {
    pub status: i32,
}

impl GetStatusResponse {
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
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(X5_CAMERA_CONTROL_API_TOPIC))
    }

    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::new(options.with_service_topic(X5_CAMERA_CONTROL_API_TOPIC))?;
        Ok(Self { rpc })
    }

    pub async fn send_api_request(&self, api_id: X5CameraApiId, param: &str) -> Result<()> {
        self.rpc
            .call_with_body::<EmptyResponse>(i32::from(api_id), param.to_owned(), None)
            .await?;
        Ok(())
    }

    pub async fn send_api_request_with_response<R>(
        &self,
        api_id: X5CameraApiId,
        param: &str,
    ) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        self.rpc
            .call_with_body(i32::from(api_id), param.to_owned(), None)
            .await
    }

    pub async fn change_mode(&self, mode: CameraSetMode) -> Result<()> {
        let param = ChangeModeParameter {
            mode: i32::from(mode),
        };
        self.send_api_request(X5CameraApiId::ChangeMode, &serialize_param(&param)?)
            .await
    }

    pub async fn get_status(&self) -> Result<GetStatusResponse> {
        self.send_api_request_with_response(X5CameraApiId::GetStatus, "")
            .await
    }
}
