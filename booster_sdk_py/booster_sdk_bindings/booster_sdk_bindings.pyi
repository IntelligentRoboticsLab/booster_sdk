"""Type stubs for booster_sdk_bindings."""

class BoosterSdkError(Exception):
    """Exception raised by the Booster SDK."""

    ...

class RobotMode:
    """Robot operating mode."""

    DAMPING: RobotMode
    PREPARE: RobotMode
    WALKING: RobotMode
    CUSTOM: RobotMode
    SOCCER: RobotMode

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class Hand:
    """Robot hand identifier."""

    LEFT: Hand
    RIGHT: Hand

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class GripperMode:
    """Gripper control mode."""

    POSITION: GripperMode
    FORCE: GripperMode

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class GripperCommand:
    """Gripper control command."""

    def __init__(
        self,
        hand: Hand,
        mode: GripperMode,
        motion_param: int,
        speed: int | None = ...,
    ) -> None: ...
    @staticmethod
    def open(hand: Hand) -> GripperCommand: ...
    @staticmethod
    def close(hand: Hand) -> GripperCommand: ...
    @staticmethod
    def grasp(hand: Hand, force: int) -> GripperCommand: ...
    def __repr__(self) -> str: ...

class BoosterClient:
    """High-level robot client."""

    def __init__(self) -> None: ...
    def wait_for_discovery(self, timeout_secs: float) -> None: ...
    def change_mode(self, mode: RobotMode) -> None: ...
    def move_robot(self, vx: float, vy: float, vyaw: float) -> None: ...
    def publish_gripper_command(self, command: GripperCommand) -> None: ...
    def publish_gripper(
        self,
        hand: Hand,
        mode: GripperMode,
        motion_param: int,
        speed: int | None = ...,
    ) -> None: ...
