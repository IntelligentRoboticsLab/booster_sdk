"""Convenience controllers ported from the C++ SDK (``MoveController``, ``ArmController``)."""

from __future__ import annotations

import booster_sdk_bindings as bindings

MoveController = bindings.MoveController
ArmController = bindings.ArmController
ArmJoint = bindings.ArmJoint

__all__ = [
    "MoveController",
    "ArmController",
    "ArmJoint",
]
