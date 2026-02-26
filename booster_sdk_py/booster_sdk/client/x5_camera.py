"""X5 camera client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

X5CameraClient = bindings.X5CameraClient
BoosterSdkError = bindings.BoosterSdkError
CameraSetMode = bindings.CameraSetMode
CameraControlStatus = bindings.CameraControlStatus
X5CameraGetStatusResponse = bindings.X5CameraGetStatusResponse

__all__ = [
    "X5CameraClient",
    "BoosterSdkError",
    "CameraSetMode",
    "CameraControlStatus",
    "X5CameraGetStatusResponse",
]
