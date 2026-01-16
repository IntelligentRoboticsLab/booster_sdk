"""Booster SDK - Python bindings for controlling the B1 robot"""

from __future__ import annotations

from .client import B1LocoClient
from .types import (
    BoosterSdkError,
    DanceId,
    Direction,
    FingerControl,
    FingerControlLike,
    Frame,
    GripperMode,
    Hand,
    ImuState,
    LowState,
    MotorMode,
    MotorState,
    Position,
    PositionLike,
    Posture,
    PostureLike,
    Quaternion,
    QuaternionLike,
    RobotMode,
    Transform,
    TransformLike,
    to_finger_control,
    to_position,
    to_posture,
    to_quaternion,
    to_transform,
)

__all__ = [
    # Client
    "B1LocoClient",
    # Core types
    "BoosterSdkError",
    "Position",
    "Quaternion",
    "Transform",
    "Posture",
    "FingerControl",
    "ImuState",
    "MotorState",
    "LowState",
    "MotorMode",
    # Enums
    "RobotMode",
    "Hand",
    "Direction",
    "Frame",
    "GripperMode",
    "DanceId",
    # Type aliases
    "PositionLike",
    "QuaternionLike",
    "TransformLike",
    "PostureLike",
    "FingerControlLike",
    # Conversion utilities
    "to_position",
    "to_quaternion",
    "to_transform",
    "to_posture",
    "to_finger_control",
]
