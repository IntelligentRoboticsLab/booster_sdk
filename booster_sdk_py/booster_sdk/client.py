"""High-level client for controlling the Booster robot."""

from __future__ import annotations

import booster_sdk_bindings as bindings

from .types import GripperCommand, GripperMode, Hand, RobotMode

__all__ = ["BoosterClient"]


class BoosterClient:
    """Minimal Python wrapper for the BoosterClient bindings."""

    def __init__(self) -> None:
        self._inner = bindings.BoosterClient()

    def wait_for_discovery(self, timeout_secs: float = 2.0) -> None:
        """Give DDS participant discovery time to complete.

        Call this once after construction and before the first ``change_mode``
        or ``move_robot`` call.  Without it, the first request may be silently
        dropped because DDS discovery (SPDP) takes a few hundred milliseconds.

        A duration of 1–2 seconds is typical for a local robot network.
        """
        self._inner.wait_for_discovery(timeout_secs)

    def change_mode(self, mode: RobotMode) -> None:
        self._inner.change_mode(mode)

    def move_robot(self, vx: float, vy: float, vyaw: float) -> None:
        self._inner.move_robot(vx, vy, vyaw)

    def publish_gripper_command(self, command: GripperCommand) -> None:
        self._inner.publish_gripper_command(command)

    def publish_gripper(
        self,
        hand: Hand,
        mode: GripperMode,
        motion_param: int,
        speed: int | None = None,
    ) -> None:
        self._inner.publish_gripper(hand, mode, motion_param, speed)
