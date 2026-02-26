"""Vision client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

VisionClient = bindings.VisionClient
BoosterSdkError = bindings.BoosterSdkError
DetectResults = bindings.DetectResults

__all__ = ["VisionClient", "BoosterSdkError", "DetectResults"]
