"""High-level client for controlling the B1 robot"""

from __future__ import annotations

from typing import Optional

import booster_sdk_bindings as bindings

from .types import (
    DanceId,
    Direction,
    FingerControlLike,
    Frame,
    GripperMode,
    Hand,
    PositionLike,
    RobotMode,
    Transform,
    TransformLike,
    to_finger_control,
    to_position,
    to_transform,
)

__all__ = ["B1LocoClient"]


class B1LocoClient:
    """High-level client for controlling the B1 robot with flexible input types

    This wrapper accepts various input types (lists, tuples, numpy arrays, dicts, etc.)
    and automatically converts them to the appropriate bindings types.

    Examples:
        >>> client = B1LocoClient()
        >>> client.move_hand(Hand.LEFT, [1.0, 2.0, 3.0], [0.0, 0.0, 0.0])
        >>> client.move_hand_transform(Hand.RIGHT, ([0.5, 0.5, 0.5], [0, 0, 0, 1]))
    """

    def __init__(self, timeout_seconds: Optional[float] = None):
        """Create a new B1LocoClient

        Args:
            timeout_seconds: Optional connection timeout in seconds. If None, uses default timeout.

        Raises:
            ValueError: If timeout is not positive
            BoosterSdkError: If connection fails
        """
        if timeout_seconds is None:
            self._inner = bindings.B1LocoClient()
        else:
            self._inner = bindings.B1LocoClient.with_timeout(timeout_seconds)

    def change_mode(self, mode: RobotMode) -> None:
        """Change the robot's operating mode

        Args:
            mode: Target robot mode

        Raises:
            BoosterSdkError: If mode change fails
        """
        self._inner.change_mode(mode)

    def get_mode(self) -> int:
        """Get the current robot mode

        Returns:
            Current mode as integer

        Raises:
            BoosterSdkError: If query fails
        """
        return self._inner.get_mode()

    def move_robot(self, vx: float, vy: float, vyaw: float) -> None:
        """Move the robot with specified velocities

        Args:
            vx: Forward/backward velocity
            vy: Left/right velocity
            vyaw: Rotational velocity

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.move_robot(vx, vy, vyaw)

    def move_with_command(
        self, vx: float = 0.0, vy: float = 0.0, vyaw: float = 0.0
    ) -> None:
        """Move the robot with optional velocity parameters

        Args:
            vx: Forward/backward velocity (default: 0.0)
            vy: Left/right velocity (default: 0.0)
            vyaw: Rotational velocity (default: 0.0)

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.move_with_command(vx, vy, vyaw)

    def lie_down(self) -> None:
        """Command the robot to lie down

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.lie_down()

    def get_up(self) -> None:
        """Command the robot to get up

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.get_up()

    def rotate_head(self, pitch: float, yaw: float) -> None:
        """Rotate the robot's head to specified angles

        Args:
            pitch: Pitch angle
            yaw: Yaw angle

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.rotate_head(pitch, yaw)

    def rotate_head_continuous(
        self, pitch_direction: Direction, yaw_direction: Direction, speed: float
    ) -> None:
        """Rotate the robot's head continuously

        Args:
            pitch_direction: Direction for pitch rotation
            yaw_direction: Direction for yaw rotation
            speed: Speed factor (0.0 to 1.0)

        Raises:
            ValueError: If speed is not between 0.0 and 1.0
            BoosterSdkError: If command fails
        """
        self._inner.rotate_head_continuous(pitch_direction, yaw_direction, speed)

    def move_hand(
        self,
        hand: Hand,
        position: PositionLike,
        orientation: PositionLike,
        duration: float = 1.0,
    ) -> None:
        """Move a hand to specified pose

        Args:
            hand: Which hand to move
            position: Target position (Position, list/tuple of 3 floats, dict with x/y/z, etc.)
            orientation: Target orientation (Position, list/tuple of 3 floats, dict with x/y/z, etc.)
            duration: Duration of movement in seconds (default: 1.0)

        Raises:
            ValueError: If duration is not positive
            BoosterSdkError: If command fails
        """
        self._inner.move_hand(
            hand, to_position(position), to_position(orientation), duration
        )

    def move_hand_with_aux(
        self,
        hand: Hand,
        position: PositionLike,
        orientation: PositionLike,
        aux_position: PositionLike,
        aux_orientation: PositionLike,
        duration: float = 1.0,
    ) -> None:
        """Move a hand with auxiliary pose

        Args:
            hand: Which hand to move
            position: Target position (Position, list/tuple of 3 floats, dict with x/y/z, etc.)
            orientation: Target orientation (Position, list/tuple of 3 floats, dict with x/y/z, etc.)
            aux_position: Auxiliary position (Position, list/tuple of 3 floats, dict with x/y/z, etc.)
            aux_orientation: Auxiliary orientation (Position, list/tuple of 3 floats, dict with x/y/z, etc.)
            duration: Duration of movement in seconds (default: 1.0)

        Raises:
            ValueError: If duration is not positive
            BoosterSdkError: If command fails
        """
        self._inner.move_hand_with_aux(
            hand,
            to_position(position),
            to_position(orientation),
            to_position(aux_position),
            to_position(aux_orientation),
            duration,
        )

    def move_hand_transform(
        self, hand: Hand, transform: TransformLike, duration: float = 1.0
    ) -> None:
        """Move a hand using a transform

        Args:
            hand: Which hand to move
            transform: Target transform (Transform, tuple of (position, rotation), dict, etc.)
            duration: Duration of movement in seconds (default: 1.0)

        Raises:
            ValueError: If duration is not positive
            BoosterSdkError: If command fails
        """
        self._inner.move_hand_transform(hand, to_transform(transform), duration)

    def wave_hand(self, hand: Hand) -> None:
        """Make a hand perform a waving gesture

        Args:
            hand: Which hand to wave

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.wave_hand(hand)

    def handshake(self) -> None:
        """Perform a handshake gesture

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.handshake()

    def control_gripper(
        self, hand: Hand, mode: GripperMode, motion_param: int, speed: int = 500
    ) -> None:
        """Control a gripper

        Args:
            hand: Which hand's gripper to control
            mode: Control mode (position or force)
            motion_param: Motion parameter value
            speed: Speed parameter (1-1000, default: 500)

        Raises:
            ValueError: If speed is not between 1 and 1000
            BoosterSdkError: If command fails
        """
        self._inner.control_gripper(hand, mode, motion_param, speed)

    def control_dexterous_hand(
        self,
        hand: Hand,
        *,
        preset: Optional[str] = None,
        thumb_rotation: Optional[FingerControlLike] = None,
        thumb: Optional[FingerControlLike] = None,
        index: Optional[FingerControlLike] = None,
        middle: Optional[FingerControlLike] = None,
        ring: Optional[FingerControlLike] = None,
        pinky: Optional[FingerControlLike] = None,
    ) -> None:
        """Control a dexterous hand

        Either provide a preset name or individual finger controls.
        Finger controls can be FingerControl objects, tuples (angle, force, speed),
        dicts with angle/force/speed keys, or objects with those attributes.

        Args:
            hand: Which hand to control
            preset: Preset name ('open_all', 'close_all', or 'pinch')
            thumb_rotation: Thumb rotation control
            thumb: Thumb finger control
            index: Index finger control
            middle: Middle finger control
            ring: Ring finger control
            pinky: Pinky finger control

        Raises:
            ValueError: If preset is invalid or required finger controls are missing
            BoosterSdkError: If command fails
        """
        # Convert finger controls if provided
        converted_thumb_rotation = (
            to_finger_control(thumb_rotation) if thumb_rotation is not None else None
        )
        converted_thumb = to_finger_control(thumb) if thumb is not None else None
        converted_index = to_finger_control(index) if index is not None else None
        converted_middle = to_finger_control(middle) if middle is not None else None
        converted_ring = to_finger_control(ring) if ring is not None else None
        converted_pinky = to_finger_control(pinky) if pinky is not None else None

        self._inner.control_dexterous_hand(
            hand,
            preset=preset,
            thumb_rotation=converted_thumb_rotation,
            thumb=converted_thumb,
            index=converted_index,
            middle=converted_middle,
            ring=converted_ring,
            pinky=converted_pinky,
        )

    def get_frame_transform(self, source: Frame, destination: Frame) -> Transform:
        """Get the transform between two reference frames

        Args:
            source: Source reference frame
            destination: Destination reference frame

        Returns:
            Transform from source to destination

        Raises:
            BoosterSdkError: If query fails
        """
        return self._inner.get_frame_transform(source, destination)

    def dance(self, dance_id: DanceId) -> None:
        """Perform a predefined dance

        Args:
            dance_id: Which dance to perform

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.dance(dance_id)

    def stop(self) -> None:
        """Stop all robot motion

        Raises:
            BoosterSdkError: If command fails
        """
        self._inner.stop()
