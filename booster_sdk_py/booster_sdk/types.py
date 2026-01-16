"""Type definitions and conversion utilities for the Booster SDK"""

from __future__ import annotations

from typing import Any, Sequence, Union

import booster_sdk_bindings as bindings

# Re-export all types from bindings
from booster_sdk_bindings import (
    BoosterSdkError,
    DanceId,
    Direction,
    FingerControl,
    Frame,
    GripperMode,
    Hand,
    ImuState,
    LowState,
    MotorMode,
    MotorState,
    Position,
    Posture,
    Quaternion,
    RobotMode,
    Transform,
)

__all__ = [
    # Exported types
    "BoosterSdkError",
    "DanceId",
    "Direction",
    "FingerControl",
    "ImuState",
    "MotorState",
    "LowState",
    "MotorMode",
    "Frame",
    "GripperMode",
    "Hand",
    "Position",
    "Posture",
    "Quaternion",
    "RobotMode",
    "Transform",
    # Type aliases
    "PositionLike",
    "QuaternionLike",
    "TransformLike",
    "PostureLike",
    "FingerControlLike",
    # Conversion functions
    "to_position",
    "to_quaternion",
    "to_transform",
    "to_posture",
    "to_finger_control",
]

# Type aliases for flexible input
PositionLike = Union[
    Position,
    Sequence[float],  # [x, y, z] or (x, y, z)
    dict[str, float],  # {"x": ..., "y": ..., "z": ...}
    Any,  # Object with x, y, z attributes (e.g., numpy array, dataclass)
]

QuaternionLike = Union[
    Quaternion,
    Sequence[float],  # [x, y, z, w] or (x, y, z, w)
    dict[str, float],  # {"x": ..., "y": ..., "z": ..., "w": ...}
    Any,  # Object with x, y, z, w attributes
]

TransformLike = Union[
    Transform,
    tuple[PositionLike, QuaternionLike],  # ((x, y, z), (qx, qy, qz, qw))
    dict[str, Any],  # {"position": ..., "rotation": ...}
    Any,  # Object with position and rotation attributes
]

PostureLike = Union[
    Posture,
    dict[str, Any],  # {"position": ..., "orientation": ...}
    Any,  # Object with position and orientation attributes
]

FingerControlLike = Union[
    FingerControl,
    Sequence[Union[float, int]],  # (angle, force, speed)
    dict[str, Union[float, int]],  # {"angle": ..., "force": ..., "speed": ...}
    Any,  # Object with angle, force, speed attributes
]


def to_position(value: PositionLike) -> Position:
    """Convert various input types to Position

    Args:
        value: Position-like value (Position, sequence of 3 floats, dict with x/y/z, or object with x/y/z attributes)

    Returns:
        Position object

    Raises:
        ValueError: If the input cannot be converted to Position
        TypeError: If the input type is not supported

    Examples:
        >>> to_position([1.0, 2.0, 3.0])
        Position(x=1.0, y=2.0, z=3.0)
        >>> to_position({"x": 1.0, "y": 2.0, "z": 3.0})
        Position(x=1.0, y=2.0, z=3.0)
    """
    if isinstance(value, Position):
        return value

    # Try sequence [x, y, z]
    if isinstance(value, (list, tuple)):
        if len(value) == 3:
            return Position(float(value[0]), float(value[1]), float(value[2]))
        raise ValueError(f"Position sequence must have 3 elements, got {len(value)}")

    # Try numpy array
    try:
        import numpy as np

        if isinstance(value, np.ndarray):
            if value.shape == (3,):
                return Position(float(value[0]), float(value[1]), float(value[2]))
            raise ValueError(f"Position array must have shape (3,), got {value.shape}")
    except ImportError:
        pass

    # Try dict
    if isinstance(value, dict):
        try:
            return Position(float(value["x"]), float(value["y"]), float(value["z"]))
        except KeyError as e:
            raise ValueError("Position dict must contain 'x', 'y', and 'z' keys") from e

    # Try object with attributes
    if hasattr(value, "x") and hasattr(value, "y") and hasattr(value, "z"):
        return Position(float(value.x), float(value.y), float(value.z))

    raise TypeError(
        f"Cannot convert {type(value).__name__} to Position. "
        "Expected Position, sequence of 3 floats, dict with x/y/z, or object with x/y/z attributes"
    )


def to_quaternion(value: QuaternionLike) -> Quaternion:
    """Convert various input types to Quaternion

    Args:
        value: Quaternion-like value (Quaternion, sequence of 4 floats, dict with x/y/z/w, or object with x/y/z/w attributes)

    Returns:
        Quaternion object

    Raises:
        ValueError: If the input cannot be converted to Quaternion
        TypeError: If the input type is not supported

    Examples:
        >>> to_quaternion([0.0, 0.0, 0.0, 1.0])
        Quaternion(x=0.0, y=0.0, z=0.0, w=1.0)
        >>> to_quaternion({"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0})
        Quaternion(x=0.0, y=0.0, z=0.0, w=1.0)
    """
    if isinstance(value, Quaternion):
        return value

    # Try sequence [x, y, z, w]
    if isinstance(value, (list, tuple)):
        if len(value) == 4:
            return Quaternion(
                float(value[0]), float(value[1]), float(value[2]), float(value[3])
            )
        raise ValueError(f"Quaternion sequence must have 4 elements, got {len(value)}")

    # Try numpy array
    try:
        import numpy as np

        if isinstance(value, np.ndarray):
            if value.shape == (4,):
                return Quaternion(
                    float(value[0]), float(value[1]), float(value[2]), float(value[3])
                )
            raise ValueError(
                f"Quaternion array must have shape (4,), got {value.shape}"
            )
    except ImportError:
        pass

    # Try dict
    if isinstance(value, dict):
        try:
            return Quaternion(
                float(value["x"]),
                float(value["y"]),
                float(value["z"]),
                float(value["w"]),
            )
        except KeyError as e:
            raise ValueError(
                "Quaternion dict must contain 'x', 'y', 'z', and 'w' keys"
            ) from e

    # Try object with attributes
    if (
        hasattr(value, "x")
        and hasattr(value, "y")
        and hasattr(value, "z")
        and hasattr(value, "w")
    ):
        return Quaternion(
            float(value.x), float(value.y), float(value.z), float(value.w)
        )

    raise TypeError(
        f"Cannot convert {type(value).__name__} to Quaternion. "
        "Expected Quaternion, sequence of 4 floats, dict with x/y/z/w, or object with x/y/z/w attributes"
    )


def to_transform(value: TransformLike) -> Transform:
    """Convert various input types to Transform

    Args:
        value: Transform-like value (Transform, tuple of (position, rotation), dict with position/rotation, or object with position/rotation attributes)

    Returns:
        Transform object

    Raises:
        ValueError: If the input cannot be converted to Transform
        TypeError: If the input type is not supported

    Examples:
        >>> to_transform(([1.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0]))
        Transform(position=Position(...), rotation=Quaternion(...))
        >>> to_transform({"position": [1.0, 2.0, 3.0], "rotation": [0.0, 0.0, 0.0, 1.0]})
        Transform(position=Position(...), rotation=Quaternion(...))
    """
    if isinstance(value, Transform):
        return value

    # Try tuple ((x, y, z), (qx, qy, qz, qw))
    if isinstance(value, tuple) and len(value) == 2:
        return Transform(to_position(value[0]), to_quaternion(value[1]))

    # Try dict {"position": ..., "rotation": ...}
    if isinstance(value, dict):
        if "position" in value and "rotation" in value:
            return Transform(
                to_position(value["position"]), to_quaternion(value["rotation"])
            )
        raise ValueError("Transform dict must contain 'position' and 'rotation' keys")

    # Try object with attributes
    if hasattr(value, "position") and hasattr(value, "rotation"):
        return Transform(to_position(value.position), to_quaternion(value.rotation))

    raise TypeError(
        f"Cannot convert {type(value).__name__} to Transform. "
        "Expected Transform, tuple of (position, rotation), dict with position/rotation, "
        "or object with position/rotation attributes"
    )


def to_posture(value: PostureLike) -> Posture:
    """Convert various input types to Posture

    Args:
        value: Posture-like value (Posture, dict with position/orientation, or object with position/orientation attributes)

    Returns:
        Posture object

    Raises:
        ValueError: If the input cannot be converted to Posture
        TypeError: If the input type is not supported

    Examples:
        >>> to_posture({"position": [1.0, 2.0, 3.0], "orientation": [0.0, 0.0, 0.0]})
        Posture(position=Position(...), orientation=Position(...))
    """
    if isinstance(value, Posture):
        return value

    # Try dict {"position": ..., "orientation": ...}
    if isinstance(value, dict):
        if "position" in value and "orientation" in value:
            return Posture(
                to_position(value["position"]), to_position(value["orientation"])
            )
        raise ValueError("Posture dict must contain 'position' and 'orientation' keys")

    # Try object with attributes
    if hasattr(value, "position") and hasattr(value, "orientation"):
        return Posture(to_position(value.position), to_position(value.orientation))

    raise TypeError(
        f"Cannot convert {type(value).__name__} to Posture. "
        "Expected Posture, dict with position/orientation, or object with position/orientation attributes"
    )


def to_finger_control(value: FingerControlLike) -> FingerControl:
    """Convert various input types to FingerControl

    Args:
        value: FingerControl-like value (FingerControl, tuple of (angle, force, speed), dict with angle/force/speed, or object with angle/force/speed attributes)

    Returns:
        FingerControl object

    Raises:
        ValueError: If the input cannot be converted to FingerControl
        TypeError: If the input type is not supported

    Examples:
        >>> to_finger_control((0.5, 100, 200))
        FingerControl(angle=0.5, force=100, speed=200)
        >>> to_finger_control({"angle": 0.5, "force": 100, "speed": 200})
        FingerControl(angle=0.5, force=100, speed=200)
    """
    if isinstance(value, FingerControl):
        return value

    # Try tuple (angle, force, speed)
    if isinstance(value, (list, tuple)):
        if len(value) == 3:
            return FingerControl(float(value[0]), int(value[1]), int(value[2]))
        raise ValueError(
            f"FingerControl sequence must have 3 elements, got {len(value)}"
        )

    # Try dict
    if isinstance(value, dict):
        try:
            return FingerControl(
                float(value["angle"]), int(value["force"]), int(value["speed"])
            )
        except KeyError as e:
            raise ValueError(
                "FingerControl dict must contain 'angle', 'force', and 'speed' keys"
            ) from e

    # Try object with attributes
    if hasattr(value, "angle") and hasattr(value, "force") and hasattr(value, "speed"):
        return FingerControl(float(value.angle), int(value.force), int(value.speed))

    raise TypeError(
        f"Cannot convert {type(value).__name__} to FingerControl. "
        "Expected FingerControl, tuple of (angle, force, speed), dict with angle/force/speed, "
        "or object with angle/force/speed attributes"
    )
