"""Hand-eye calibration client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

HandEyeCalibClient = bindings.HandEyeCalibClient
BoosterSdkError = bindings.BoosterSdkError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
HandEyeCalibApiId = bindings.HandEyeCalibApiId
StartHandEyeCalibParameter = bindings.StartHandEyeCalibParameter
HandEyeCalibStatus = bindings.HandEyeCalibStatus
HandEyeCalibResult = bindings.HandEyeCalibResult
HandEyeCalibApplyResult = bindings.HandEyeCalibApplyResult

__all__ = [
    "HandEyeCalibClient",
    "BoosterSdkError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "HandEyeCalibApiId",
    "StartHandEyeCalibParameter",
    "HandEyeCalibStatus",
    "HandEyeCalibResult",
    "HandEyeCalibApplyResult",
]
