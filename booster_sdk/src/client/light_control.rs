//! LED light control RPC client.

use serde::{Deserialize, Serialize};

use crate::dds::{LIGHT_CONTROL_API_TOPIC, RpcClient, RpcClientOptions};
use crate::types::Result;

crate::api_id_enum! {
    /// LED light control RPC API identifiers.
    LightApiId {
        SetLedLightColor = 2000,
        StopLedLightControl = 2001,
    }
}

/// RGB color payload for LED control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetLedLightColorParameter {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl SetLedLightColorParameter {
    /// Parse a `#RRGGBB` color string.
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
    /// Create a light control client with default options.
    pub fn new() -> Result<Self> {
        Self::with_options(RpcClientOptions::for_service(LIGHT_CONTROL_API_TOPIC))
    }

    /// Create a light control client with custom RPC options.
    pub fn with_options(options: RpcClientOptions) -> Result<Self> {
        let rpc = RpcClient::for_topic(options, LIGHT_CONTROL_API_TOPIC)?;
        Ok(Self { rpc })
    }

    /// Set LED light color from RGB values.
    pub async fn set_led_light_color(&self, r: u8, g: u8, b: u8) -> Result<()> {
        self.set_led_light_color_param(&SetLedLightColorParameter { r, g, b })
            .await
    }

    /// Set LED light color using a parameter struct.
    pub async fn set_led_light_color_param(&self, param: &SetLedLightColorParameter) -> Result<()> {
        self.rpc
            .call_serialized(LightApiId::SetLedLightColor, param)
            .await
    }

    /// Stop LED light control.
    pub async fn stop_led_light_control(&self) -> Result<()> {
        self.rpc
            .call_void(LightApiId::StopLedLightControl, "")
            .await
    }
}
