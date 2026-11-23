"""Vision client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

VisionClient = bindings.VisionClient
BoosterSdkError = bindings.BoosterSdkError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
VisionApiId = bindings.VisionApiId
StartVisionServiceParameter = bindings.StartVisionServiceParameter
GetDetectionObjectParameter = bindings.GetDetectionObjectParameter
DetectResults = bindings.DetectResults

__all__ = [
    "VisionClient",
    "BoosterSdkError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "VisionApiId",
    "StartVisionServiceParameter",
    "GetDetectionObjectParameter",
    "DetectResults",
]
