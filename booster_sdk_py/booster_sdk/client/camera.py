"""Camera catalog client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

CameraClient = bindings.CameraClient
DeviceInfo = bindings.DeviceInfo
DeviceInfoKind = bindings.DeviceInfoKind
BoosterSdkError = bindings.BoosterSdkError

__all__ = ["CameraClient", "DeviceInfo", "DeviceInfoKind", "BoosterSdkError"]
