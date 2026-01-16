"""Type stubs for booster_sdk_bindings"""

from typing import Optional

class BoosterSdkError(Exception):
    """Exception raised by the Booster SDK"""

    ...

class RobotMode:
    """Robot operating mode"""

    DAMPING: RobotMode
    PREPARE: RobotMode
    WALKING: RobotMode
    CUSTOM: RobotMode

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class Hand:
    """Robot hand identifier"""

    LEFT: Hand
    RIGHT: Hand

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class Direction:
    """Direction for continuous movement"""

    POSITIVE: Direction
    STOP: Direction
    NEGATIVE: Direction

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class Frame:
    """Reference frame for transforms"""

    BODY: Frame
    HEAD: Frame
    LEFT_HAND: Frame
    RIGHT_HAND: Frame
    LEFT_FOOT: Frame
    RIGHT_FOOT: Frame

    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...

class GripperMode:
    """Gripper control mode"""

    POSITION: GripperMode
    FORCE: GripperMode

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class DanceId:
    """Predefined dance identifier"""

    NEW_YEAR: DanceId
    NEZHA: DanceId
    TOWARDS_FUTURE: DanceId

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class Position:
    """3D position with x, y, z coordinates"""

    def __init__(self, x: float, y: float, z: float) -> None:
        """Create a new Position

        Args:
            x: X coordinate
            y: Y coordinate
            z: Z coordinate
        """
        ...

    @property
    def x(self) -> float:
        """X coordinate"""
        ...

    @property
    def y(self) -> float:
        """Y coordinate"""
        ...

    @property
    def z(self) -> float:
        """Z coordinate"""
        ...

    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __getitem__(self, idx: int) -> float: ...

class Quaternion:
    """Rotation quaternion with x, y, z, w components"""

    def __init__(self, x: float, y: float, z: float, w: float) -> None:
        """Create a new Quaternion

        Args:
            x: X component
            y: Y component
            z: Z component
            w: W component (scalar part)
        """
        ...

    @property
    def x(self) -> float:
        """X component"""
        ...

    @property
    def y(self) -> float:
        """Y component"""
        ...

    @property
    def z(self) -> float:
        """Z component"""
        ...

    @property
    def w(self) -> float:
        """W component (scalar part)"""
        ...

    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __getitem__(self, idx: int) -> float: ...

class Transform:
    """3D transform with position and rotation"""

    def __init__(self, position: Position, rotation: Quaternion) -> None:
        """Create a new Transform

        Args:
            position: Position component
            rotation: Rotation component as quaternion
        """
        ...

    @property
    def position(self) -> Position:
        """Position component"""
        ...

    @property
    def rotation(self) -> Quaternion:
        """Rotation component"""
        ...

    def __repr__(self) -> str: ...

class Posture:
    """Robot posture with position and orientation"""

    def __init__(self, position: Position, orientation: Position) -> None:
        """Create a new Posture

        Args:
            position: Position in 3D space
            orientation: Orientation as Euler angles or similar representation
        """
        ...

    @property
    def position(self) -> Position:
        """Position component"""
        ...

    @property
    def orientation(self) -> Position:
        """Orientation component"""
        ...

    def __repr__(self) -> str: ...

class FingerControl:
    """Control parameters for a dexterous hand finger"""

    def __init__(self, angle: float, force: int, speed: int) -> None:
        """Create new FingerControl parameters

        Args:
            angle: Target angle for the finger
            force: Force parameter (0-65535)
            speed: Speed parameter (0-65535)
        """
        ...

    @property
    def angle(self) -> float:
        """Target angle"""
        ...

    @property
    def force(self) -> int:
        """Force parameter"""
        ...

    @property
    def speed(self) -> int:
        """Speed parameter"""
        ...

    def __repr__(self) -> str: ...

class MotorMode:
    """Motor control mode"""

    SERVO: MotorMode
    DAMPING: MotorMode

    def __repr__(self) -> str: ...
    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class ImuState:
    """Inertial measurement unit readings"""

    def __init__(
        self,
        rpy: Optional[list[float]] = ...,
        gyro: Optional[list[float]] = ...,
        acc: Optional[list[float]] = ...,
    ) -> None: ...

    @property
    def rpy(self) -> tuple[float, float, float]: ...
    @rpy.setter
    def rpy(self, values: list[float]) -> None: ...

    @property
    def gyro(self) -> tuple[float, float, float]: ...
    @gyro.setter
    def gyro(self, values: list[float]) -> None: ...

    @property
    def acc(self) -> tuple[float, float, float]: ...
    @acc.setter
    def acc(self, values: list[float]) -> None: ...

    def __repr__(self) -> str: ...

class MotorState:
    """Feedback state for a single motor"""

    def __init__(
        self,
        mode: MotorMode = ...,
        q: float = ...,
        dq: float = ...,
        ddq: float = ...,
        tau_est: float = ...,
        temperature: int = ...,
        lost: int = ...,
        reserve: Optional[list[int]] = ...,
    ) -> None: ...

    @property
    def mode(self) -> MotorMode: ...
    @mode.setter
    def mode(self, mode: MotorMode) -> None: ...

    @property
    def q(self) -> float: ...
    @q.setter
    def q(self, value: float) -> None: ...

    @property
    def dq(self) -> float: ...
    @dq.setter
    def dq(self, value: float) -> None: ...

    @property
    def ddq(self) -> float: ...
    @ddq.setter
    def ddq(self, value: float) -> None: ...

    @property
    def tau_est(self) -> float: ...
    @tau_est.setter
    def tau_est(self, value: float) -> None: ...

    @property
    def temperature(self) -> int: ...
    @temperature.setter
    def temperature(self, value: int) -> None: ...

    @property
    def lost(self) -> int: ...
    @lost.setter
    def lost(self, value: int) -> None: ...

    @property
    def reserve(self) -> tuple[int, int]: ...
    @reserve.setter
    def reserve(self, values: list[int]) -> None: ...

    def __repr__(self) -> str: ...

class LowState:
    """Low-level state containing IMU and motor feedback"""

    def __init__(
        self,
        imu_state: Optional[ImuState] = ...,
        motor_state_parallel: Optional[list[MotorState]] = ...,
        motor_state_serial: Optional[list[MotorState]] = ...,
    ) -> None: ...

    @classmethod
    def from_cdr(cls, data: bytes) -> LowState: ...
    def to_cdr(self) -> bytes: ...

    @property
    def imu_state(self) -> ImuState: ...
    @imu_state.setter
    def imu_state(self, imu: ImuState) -> None: ...

    @property
    def motor_state_parallel(self) -> list[MotorState]: ...
    @motor_state_parallel.setter
    def motor_state_parallel(self, motors: list[MotorState]) -> None: ...

    @property
    def motor_state_serial(self) -> list[MotorState]: ...
    @motor_state_serial.setter
    def motor_state_serial(self, motors: list[MotorState]) -> None: ...

    def __repr__(self) -> str: ...

class B1LocoClient:
    """Client for controlling the B1 robot"""

    def __init__(self) -> None:
        """Create a new B1LocoClient with default timeout"""
        ...

    @classmethod
    def with_timeout(cls, timeout_seconds: float) -> B1LocoClient:
        """Create a new B1LocoClient with custom timeout

        Args:
            timeout_seconds: Connection timeout in seconds (must be positive)

        Returns:
            New B1LocoClient instance

        Raises:
            ValueError: If timeout is not positive
            BoosterSdkError: If connection fails
        """
        ...

    def change_mode(self, mode: RobotMode) -> None:
        """Change the robot's operating mode

        Args:
            mode: Target robot mode

        Raises:
            BoosterSdkError: If mode change fails
        """
        ...

    def get_mode(self) -> int:
        """Get the current robot mode

        Returns:
            Current mode as integer

        Raises:
            BoosterSdkError: If query fails
        """
        ...

    def move_robot(self, vx: float, vy: float, vyaw: float) -> None:
        """Move the robot with specified velocities

        Args:
            vx: Forward/backward velocity
            vy: Left/right velocity
            vyaw: Rotational velocity

        Raises:
            BoosterSdkError: If command fails
        """
        ...

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
        ...

    def lie_down(self) -> None:
        """Command the robot to lie down

        Raises:
            BoosterSdkError: If command fails
        """
        ...

    def get_up(self) -> None:
        """Command the robot to get up

        Raises:
            BoosterSdkError: If command fails
        """
        ...

    def rotate_head(self, pitch: float, yaw: float) -> None:
        """Rotate the robot's head to specified angles

        Args:
            pitch: Pitch angle
            yaw: Yaw angle

        Raises:
            BoosterSdkError: If command fails
        """
        ...

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
        ...

    def move_hand(
        self,
        hand: Hand,
        position: Position,
        orientation: Position,
        duration: float = 1.0,
    ) -> None:
        """Move a hand to specified pose

        Args:
            hand: Which hand to move
            position: Target position
            orientation: Target orientation
            duration: Duration of movement in seconds (default: 1.0)

        Raises:
            ValueError: If duration is not positive
            BoosterSdkError: If command fails
        """
        ...

    def move_hand_with_aux(
        self,
        hand: Hand,
        position: Position,
        orientation: Position,
        aux_position: Position,
        aux_orientation: Position,
        duration: float = 1.0,
    ) -> None:
        """Move a hand with auxiliary pose

        Args:
            hand: Which hand to move
            position: Target position
            orientation: Target orientation
            aux_position: Auxiliary position
            aux_orientation: Auxiliary orientation
            duration: Duration of movement in seconds (default: 1.0)

        Raises:
            ValueError: If duration is not positive
            BoosterSdkError: If command fails
        """
        ...

    def move_hand_transform(
        self, hand: Hand, transform: Transform, duration: float = 1.0
    ) -> None:
        """Move a hand using a transform

        Args:
            hand: Which hand to move
            transform: Target transform
            duration: Duration of movement in seconds (default: 1.0)

        Raises:
            ValueError: If duration is not positive
            BoosterSdkError: If command fails
        """
        ...

    def wave_hand(self, hand: Hand) -> None:
        """Make a hand perform a waving gesture

        Args:
            hand: Which hand to wave

        Raises:
            BoosterSdkError: If command fails
        """
        ...

    def handshake(self) -> None:
        """Perform a handshake gesture

        Raises:
            BoosterSdkError: If command fails
        """
        ...

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
        ...

    def control_dexterous_hand(
        self,
        hand: Hand,
        *,
        preset: Optional[str] = None,
        thumb_rotation: Optional[FingerControl] = None,
        thumb: Optional[FingerControl] = None,
        index: Optional[FingerControl] = None,
        middle: Optional[FingerControl] = None,
        ring: Optional[FingerControl] = None,
        pinky: Optional[FingerControl] = None,
    ) -> None:
        """Control a dexterous hand

        Either provide a preset name or individual finger controls.

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
        ...

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
        ...

    def dance(self, dance_id: DanceId) -> None:
        """Perform a predefined dance

        Args:
            dance_id: Which dance to perform

        Raises:
            BoosterSdkError: If command fails
        """
        ...

    def stop(self) -> None:
        """Stop all robot motion

        Raises:
            BoosterSdkError: If command fails
        """
        ...
