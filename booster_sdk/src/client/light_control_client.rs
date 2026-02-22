//! LED light control RPC client.

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::dds::{LIGHT_CONTROL_API_TOPIC, RpcClient, RpcClientOptions};
use crate::types::Result;

use super::util::{EmptyResponse, serialize_param};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum LightApiId {
    SetLedLightColor = 2000,
    StopLedLightControl = 2001,
}

impl From<LightApiId> for i32 {
    fn from(value: LightApiId) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for LightApiId {
    type Error = &'static str;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            2000 => Ok(Self::SetLedLightColor),
            2001 => Ok(Self::StopLedLightControl),
            _ => Err("invalid value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetLedLightColorParameter {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl SetLedLightColorParameter {
    #[must_use]
    pub fn from_hex(color: &str) -> Option<Self> {
        let color = color.trim();
        if color.len() != 7 || !color.starts_with('#') {
            return None;
        }

        let r = u8::from_str_radix(&color[1..3], 16).ok()?;
        let g = u8::from_str_radix(&color[3..5], 16).ok()?;
        let b = u8::from_str_radix(&color[5..7], 16).ok()?;

        Some(Self { r, g, b })
    }
}

/// High-level RPC client for LED light control APIs.
pub struct LightControlClient {
    rpc: RpcClient,
}

impl LightControlClient {
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(LIGHT_CONTROL_API_TOPIC))
    }

    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::new(options.with_service_topic(LIGHT_CONTROL_API_TOPIC))?;
        Ok(Self { rpc })
    }

    pub async fn send_api_request(&self, api_id: LightApiId, param: &str) -> Result<()> {
        self.rpc
            .call_with_body::<EmptyResponse>(i32::from(api_id), param.to_owned(), None)
            .await?;
        Ok(())
    }

    pub async fn send_api_request_with_response<R>(
        &self,
        api_id: LightApiId,
        param: &str,
    ) -> Result<R>
    where
        R: DeserializeOwned + Send + 'static,
    {
        self.rpc
            .call_with_body(i32::from(api_id), param.to_owned(), None)
            .await
    }

    pub async fn set_led_light_color(&self, r: u8, g: u8, b: u8) -> Result<()> {
        let param = SetLedLightColorParameter { r, g, b };
        self.send_api_request(LightApiId::SetLedLightColor, &serialize_param(&param)?)
            .await
    }

    pub async fn set_led_light_color_param(&self, param: &SetLedLightColorParameter) -> Result<()> {
        self.send_api_request(LightApiId::SetLedLightColor, &serialize_param(param)?)
            .await
    }

    pub async fn stop_led_light_control(&self) -> Result<()> {
        self.send_api_request(LightApiId::StopLedLightControl, "")
            .await
    }
}
