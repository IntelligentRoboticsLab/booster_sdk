"""Light control client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

LightControlClient = bindings.LightControlClient
BoosterSdkError = bindings.BoosterSdkError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
LightApiId = bindings.LightApiId
SetLEDLightColorParameter = bindings.SetLEDLightColorParameter
SetLEDLightColorsParameter = bindings.SetLEDLightColorsParameter

__all__ = [
    "LightControlClient",
    "BoosterSdkError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "LightApiId",
    "SetLEDLightColorParameter",
    "SetLEDLightColorsParameter",
]
