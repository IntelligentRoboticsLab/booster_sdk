"""Booster SDK Python package.

SDK errors and RPC status codes are re-exported here; clients live in
:mod:`booster_sdk.client`."""

from __future__ import annotations

import booster_sdk_bindings as bindings

BoosterSdkError = bindings.BoosterSdkError
DdsError = bindings.DdsError
ValidationError = bindings.ValidationError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
RPC_STATUS_CODE_INVALID = bindings.RPC_STATUS_CODE_INVALID
RPC_STATUS_CODE_SUCCESS = bindings.RPC_STATUS_CODE_SUCCESS
RPC_STATUS_CODE_TIMEOUT = bindings.RPC_STATUS_CODE_TIMEOUT
RPC_STATUS_CODE_BAD_REQUEST = bindings.RPC_STATUS_CODE_BAD_REQUEST
RPC_STATUS_CODE_CONFLICT = bindings.RPC_STATUS_CODE_CONFLICT
RPC_STATUS_CODE_REQUEST_TOO_FREQUENT = bindings.RPC_STATUS_CODE_REQUEST_TOO_FREQUENT
RPC_STATUS_CODE_INTERNAL_SERVER_ERROR = bindings.RPC_STATUS_CODE_INTERNAL_SERVER_ERROR
RPC_STATUS_CODE_SERVER_REFUSED = bindings.RPC_STATUS_CODE_SERVER_REFUSED
RPC_STATUS_CODE_STATE_TRANSITION_FAILED = (
    bindings.RPC_STATUS_CODE_STATE_TRANSITION_FAILED
)
RPC_STATUS_CODE_LOW_BATTERY = bindings.RPC_STATUS_CODE_LOW_BATTERY

__all__ = [
    "client",
    "BoosterSdkError",
    "DdsError",
    "ValidationError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "RPC_STATUS_CODE_INVALID",
    "RPC_STATUS_CODE_SUCCESS",
    "RPC_STATUS_CODE_TIMEOUT",
    "RPC_STATUS_CODE_BAD_REQUEST",
    "RPC_STATUS_CODE_CONFLICT",
    "RPC_STATUS_CODE_REQUEST_TOO_FREQUENT",
    "RPC_STATUS_CODE_INTERNAL_SERVER_ERROR",
    "RPC_STATUS_CODE_SERVER_REFUSED",
    "RPC_STATUS_CODE_STATE_TRANSITION_FAILED",
    "RPC_STATUS_CODE_LOW_BATTERY",
]
