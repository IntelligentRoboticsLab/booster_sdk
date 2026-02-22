"""High-level client classes for the Booster SDK."""

from __future__ import annotations

import booster_sdk_bindings as bindings

BoosterClient = bindings.BoosterClient
AiClient = bindings.AiClient
LuiClient = bindings.LuiClient
LightControlClient = bindings.LightControlClient
VisionClient = bindings.VisionClient
X5CameraClient = bindings.X5CameraClient

__all__ = [
    "BoosterClient",
    "AiClient",
    "LuiClient",
    "LightControlClient",
    "VisionClient",
    "X5CameraClient",
]
