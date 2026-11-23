"""X5 camera client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

X5CameraClient = bindings.X5CameraClient
BoosterSdkError = bindings.BoosterSdkError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
X5CameraApiId = bindings.X5CameraApiId
X5CameraChangeModeParameter = bindings.X5CameraChangeModeParameter
CameraSetMode = bindings.CameraSetMode
CameraControlStatus = bindings.CameraControlStatus
X5CameraGetStatusResponse = bindings.X5CameraGetStatusResponse

__all__ = [
    "X5CameraClient",
    "BoosterSdkError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "X5CameraApiId",
    "X5CameraChangeModeParameter",
    "CameraSetMode",
    "CameraControlStatus",
    "X5CameraGetStatusResponse",
]
