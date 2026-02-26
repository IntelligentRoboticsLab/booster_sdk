"""LUI speech client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

LuiClient = bindings.LuiClient
BoosterSdkError = bindings.BoosterSdkError
LuiTtsConfig = bindings.LuiTtsConfig
LuiTtsParameter = bindings.LuiTtsParameter

__all__ = [
    "LuiClient",
    "BoosterSdkError",
    "LuiTtsConfig",
    "LuiTtsParameter",
]
