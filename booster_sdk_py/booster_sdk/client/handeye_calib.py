"""Hand-eye calibration client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

HandEyeCalibClient = bindings.HandEyeCalibClient
StartHandEyeCalibParameter = bindings.StartHandEyeCalibParameter
HandEyeCalibStatus = bindings.HandEyeCalibStatus
HandEyeCalibResult = bindings.HandEyeCalibResult
HandEyeCalibApplyResult = bindings.HandEyeCalibApplyResult
BoosterSdkError = bindings.BoosterSdkError

__all__ = [
    "HandEyeCalibClient",
    "StartHandEyeCalibParameter",
    "HandEyeCalibStatus",
    "HandEyeCalibResult",
    "HandEyeCalibApplyResult",
    "BoosterSdkError",
]
