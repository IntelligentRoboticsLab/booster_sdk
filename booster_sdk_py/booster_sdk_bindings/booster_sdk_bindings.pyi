"""Type stubs for booster_sdk_bindings."""

from __future__ import annotations

BOOSTER_ROBOT_USER_ID: str

class BoosterSdkError(Exception):
    """Exception raised by the Booster SDK."""

    ...

class RobotMode:
    """Operational robot modes used by locomotion APIs.

    Use these values with :meth:`BoosterClient.change_mode` and
    :meth:`BoosterClient.get_up_with_mode`. The integer representation is the
    raw RPC value returned by the robot service.
    """

    UNKNOWN: RobotMode
    DAMPING: RobotMode
    PREPARE: RobotMode
    WALKING: RobotMode
    CUSTOM: RobotMode
    SOCCER: RobotMode

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same mode."""
        ...

class Hand:
    """Hand selector used by arm, gripper, and dexterous-hand APIs."""

    LEFT: Hand
    RIGHT: Hand

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same hand."""
        ...

class GripperMode:
    """Control mode for gripper commands.

    ``POSITION`` interprets ``motion_param`` as target opening.
    ``FORCE`` interprets ``motion_param`` as grasp force.
    """

    POSITION: GripperMode
    FORCE: GripperMode

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same mode."""
        ...

class HandAction:
    """Open/close hand action used by handshake and wave APIs."""

    OPEN: HandAction
    CLOSE: HandAction

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same action."""
        ...

class Frame:
    """Reference frame identifiers used for transform queries."""

    UNKNOWN: Frame
    BODY: Frame
    HEAD: Frame
    LEFT_HAND: Frame
    RIGHT_HAND: Frame
    LEFT_FOOT: Frame
    RIGHT_FOOT: Frame

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same frame."""
        ...

class GripperControlMode:
    """Mode for :meth:`BoosterClient.control_gripper`."""

    POSITION: GripperControlMode
    FORCE: GripperControlMode

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same mode."""
        ...

class BoosterHandType:
    """Dexterous hand hardware type used by low-level hand control APIs."""

    INSPIRE_HAND: BoosterHandType
    INSPIRE_TOUCH_HAND: BoosterHandType
    REVO_HAND: BoosterHandType
    UNKNOWN: BoosterHandType

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same hand type."""
        ...

class DanceId:
    """Upper-body dance and gesture identifiers for :meth:`BoosterClient.dance`."""

    NEW_YEAR: DanceId
    NEZHA: DanceId
    TOWARDS_FUTURE: DanceId
    DABBING_GESTURE: DanceId
    ULTRAMAN_GESTURE: DanceId
    RESPECT_GESTURE: DanceId
    CHEERING_GESTURE: DanceId
    LUCKY_CAT_GESTURE: DanceId
    STOP: DanceId

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same dance id."""
        ...

class WholeBodyDanceId:
    """Whole-body dance identifiers for :meth:`BoosterClient.whole_body_dance`."""

    ARBIC_DANCE: WholeBodyDanceId
    MICHAEL_DANCE_1: WholeBodyDanceId
    MICHAEL_DANCE_2: WholeBodyDanceId
    MICHAEL_DANCE_3: WholeBodyDanceId
    MOON_WALK: WholeBodyDanceId
    BOXING_STYLE_KICK: WholeBodyDanceId
    ROUNDHOUSE_KICK: WholeBodyDanceId

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same dance id."""
        ...

class JointOrder:
    """Joint indexing convention used by custom trajectory models."""

    MUJOCO: JointOrder
    ISAAC_LAB: JointOrder

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same joint order."""
        ...

class BodyControl:
    """High-level body-control state values reported by ``get_status()``."""

    UNKNOWN: BodyControl
    DAMPING: BodyControl
    PREPARE: BodyControl
    HUMANLIKE_GAIT: BodyControl
    PRONE_BODY: BodyControl
    SOCCER_GAIT: BodyControl
    CUSTOM: BodyControl
    GET_UP: BodyControl
    WHOLE_BODY_DANCE: BodyControl
    SHOOT: BodyControl
    INSIDE_FOOT: BodyControl
    GOALIE: BodyControl
    WBC_GAIT: BodyControl

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same body state."""
        ...

class Action:
    """High-level action identifiers reported by ``get_status()``."""

    UNKNOWN: Action
    HAND_SHAKE: Action
    HAND_WAVE: Action
    HAND_CONTROL: Action
    DANCE_NEW_YEAR: Action
    DANCE_NEZHA: Action
    DANCE_TOWARDS_FUTURE: Action
    GESTURE_DABBING: Action
    GESTURE_ULTRAMAN: Action
    GESTURE_RESPECT: Action
    GESTURE_CHEER: Action
    GESTURE_LUCKY_CAT: Action
    GESTURE_BOXING: Action
    ZERO_TORQUE_DRAG: Action
    RECORD_TRAJ: Action
    RUN_RECORDED_TRAJ: Action

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same action."""
        ...

class CameraSetMode:
    """Requested X5 camera mode for :meth:`X5CameraClient.change_mode`."""

    CAMERA_MODE_NORMAL: CameraSetMode
    CAMERA_MODE_HIGH_RESOLUTION: CameraSetMode
    CAMERA_MODE_NORMAL_ENABLE: CameraSetMode
    CAMERA_MODE_HIGH_RESOLUTION_ENABLE: CameraSetMode

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same mode."""
        ...

class CameraControlStatus:
    """Status values reported by :meth:`X5CameraClient.get_status`."""

    CAMERA_STATUS_NORMAL: CameraControlStatus
    CAMERA_STATUS_HIGH_RESOLUTION: CameraControlStatus
    CAMERA_STATUS_ERROR: CameraControlStatus
    CAMERA_STATUS_NULL: CameraControlStatus

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same status."""
        ...

class GripperCommand:
    """High-level gripper command payload used for topic publishing."""
    def __init__(
        self,
        hand: Hand,
        mode: GripperMode,
        motion_param: int,
        speed: int | None = ...,
    ) -> None:
        """Create a gripper command.

        Args:
            hand: Target hand.
            mode: Command mode (position or force).
            motion_param: Position or force value depending on ``mode``.
            speed: Motion speed (default: ``500``).
        """
        ...
    @staticmethod
    def open(hand: Hand) -> GripperCommand:
        """Build a position-mode command that fully opens the gripper."""
        ...
    @staticmethod
    def close(hand: Hand) -> GripperCommand:
        """Build a position-mode command that fully closes the gripper."""
        ...
    @staticmethod
    def grasp(hand: Hand, force: int) -> GripperCommand:
        """Build a force-mode grasp command.

        ``force`` is clamped to the SDK-supported range ``[50, 1000]``.
        """
        ...
    @property
    def hand(self) -> Hand:
        """Target hand."""
        ...
    @property
    def mode(self) -> GripperMode:
        """Command mode."""
        ...
    @property
    def motion_param(self) -> int:
        """Position/force parameter interpreted by ``mode``."""
        ...
    @property
    def speed(self) -> int:
        """Command speed value."""
        ...
    def __repr__(self) -> str:
        """Return a debug representation of this command."""
        ...

class Position:
    """Cartesian position in the robot coordinate system."""
    def __init__(self, x: float, y: float, z: float) -> None:
        """Create a position from ``x``, ``y``, and ``z`` coordinates."""
        ...
    @property
    def x(self) -> float:
        """X coordinate."""
        ...
    @x.setter
    def x(self, value: float) -> None:
        """Set the X coordinate."""
        ...
    @property
    def y(self) -> float:
        """Y coordinate."""
        ...
    @y.setter
    def y(self, value: float) -> None:
        """Set the Y coordinate."""
        ...
    @property
    def z(self) -> float:
        """Z coordinate."""
        ...
    @z.setter
    def z(self, value: float) -> None:
        """Set the Z coordinate."""
        ...

class Orientation:
    """Euler orientation (roll, pitch, yaw) in radians."""
    def __init__(self, roll: float, pitch: float, yaw: float) -> None:
        """Create an orientation from roll, pitch, and yaw angles."""
        ...
    @property
    def roll(self) -> float:
        """Roll angle in radians."""
        ...
    @roll.setter
    def roll(self, value: float) -> None:
        """Set roll angle in radians."""
        ...
    @property
    def pitch(self) -> float:
        """Pitch angle in radians."""
        ...
    @pitch.setter
    def pitch(self, value: float) -> None:
        """Set pitch angle in radians."""
        ...
    @property
    def yaw(self) -> float:
        """Yaw angle in radians."""
        ...
    @yaw.setter
    def yaw(self, value: float) -> None:
        """Set yaw angle in radians."""
        ...

class Posture:
    """Combined position and Euler orientation target."""
    def __init__(self, position: Position, orientation: Orientation) -> None:
        """Create a posture from ``position`` and ``orientation``."""
        ...
    @property
    def position(self) -> Position:
        """Position component."""
        ...
    @position.setter
    def position(self, value: Position) -> None:
        """Set the position component."""
        ...
    @property
    def orientation(self) -> Orientation:
        """Orientation component."""
        ...
    @orientation.setter
    def orientation(self, value: Orientation) -> None:
        """Set the orientation component."""
        ...

class Quaternion:
    """Quaternion orientation container."""
    def __init__(self, x: float, y: float, z: float, w: float) -> None:
        """Create a quaternion from ``x``, ``y``, ``z``, and ``w`` values."""
        ...
    @property
    def x(self) -> float:
        """Quaternion X component."""
        ...
    @property
    def y(self) -> float:
        """Quaternion Y component."""
        ...
    @property
    def z(self) -> float:
        """Quaternion Z component."""
        ...
    @property
    def w(self) -> float:
        """Quaternion W component."""
        ...

class Transform:
    """Rigid transform with translation and quaternion orientation."""
    def __init__(self, position: Position, orientation: Quaternion) -> None:
        """Create a transform."""
        ...
    @property
    def position(self) -> Position:
        """Translation component."""
        ...
    @property
    def orientation(self) -> Quaternion:
        """Quaternion orientation component."""
        ...

class GripperMotionParameter:
    """Parameter bundle for :meth:`BoosterClient.control_gripper`."""
    def __init__(self, position: int, force: int, speed: int) -> None:
        """Create gripper motion parameters."""
        ...
    @property
    def position(self) -> int:
        """Target position value."""
        ...
    @property
    def force(self) -> int:
        """Target force value."""
        ...
    @property
    def speed(self) -> int:
        """Motion speed value."""
        ...

class DexterousFingerParameter:
    """Low-level command for a single dexterous finger."""
    def __init__(self, seq: int, angle: int, force: int, speed: int) -> None:
        """Create one dexterous-finger command."""
        ...
    @property
    def seq(self) -> int:
        """Finger sequence/index."""
        ...
    @property
    def angle(self) -> int:
        """Target finger angle."""
        ...
    @property
    def force(self) -> int:
        """Force value for this finger."""
        ...
    @property
    def speed(self) -> int:
        """Speed value for this finger."""
        ...

class CustomModelParams:
    """Controller gains and scaling for a custom trajectory model."""
    def __init__(
        self, action_scale: list[float], kp: list[float], kd: list[float]
    ) -> None:
        """Create custom-model parameter vectors."""
        ...
    @property
    def action_scale(self) -> list[float]:
        """Per-joint action scaling factors."""
        ...
    @property
    def kp(self) -> list[float]:
        """Per-joint proportional gains."""
        ...
    @property
    def kd(self) -> list[float]:
        """Per-joint derivative gains."""
        ...

class CustomModel:
    """Metadata for a model used by custom trained trajectories."""
    def __init__(
        self,
        file_path: str,
        params: list[CustomModelParams],
        joint_order: JointOrder,
    ) -> None:
        """Create a custom model descriptor."""
        ...
    @property
    def file_path(self) -> str:
        """Path to the model file."""
        ...
    @property
    def params(self) -> list[CustomModelParams]:
        """Model parameter sets."""
        ...
    @property
    def joint_order(self) -> JointOrder:
        """Joint indexing convention expected by the model."""
        ...

class CustomTrainedTraj:
    """Payload for loading a custom trained trajectory."""
    def __init__(self, traj_file_path: str, model: CustomModel) -> None:
        """Create a trajectory payload for ``load_custom_trained_traj``."""
        ...
    @property
    def traj_file_path(self) -> str:
        """Path to the trajectory file."""
        ...
    @property
    def model(self) -> CustomModel:
        """Model metadata associated with this trajectory."""
        ...

class TtsConfig:
    """Text-to-speech configuration for AI chat."""
    def __init__(self, voice_type: str, ignore_bracket_text: list[int]) -> None:
        """Create TTS configuration."""
        ...
    @property
    def voice_type(self) -> str:
        """Voice profile identifier."""
        ...
    @property
    def ignore_bracket_text(self) -> list[int]:
        """Service-specific flags for bracket-text filtering."""
        ...

class LlmConfig:
    """Prompt configuration for the AI chat LLM."""
    def __init__(self, system_prompt: str, welcome_msg: str, prompt_name: str) -> None:
        """Create LLM prompt configuration."""
        ...
    @property
    def system_prompt(self) -> str:
        """System instruction prompt."""
        ...
    @property
    def welcome_msg(self) -> str:
        """Initial assistant greeting."""
        ...
    @property
    def prompt_name(self) -> str:
        """Prompt template/profile name."""
        ...

class AsrConfig:
    """Automatic speech recognition interruption settings."""
    def __init__(
        self, interrupt_speech_duration: int, interrupt_keywords: list[str]
    ) -> None:
        """Create ASR interruption configuration."""
        ...
    @property
    def interrupt_speech_duration(self) -> int:
        """Interruption speech duration threshold used by the service."""
        ...
    @property
    def interrupt_keywords(self) -> list[str]:
        """Keywords that trigger interruption behavior."""
        ...

class StartAiChatParameter:
    """Configuration payload for :meth:`AiClient.start_ai_chat`."""
    def __init__(
        self,
        interrupt_mode: bool,
        asr_config: AsrConfig,
        llm_config: LlmConfig,
        tts_config: TtsConfig,
        enable_face_tracking: bool,
    ) -> None:
        """Create AI chat startup parameters."""
        ...
    @property
    def interrupt_mode(self) -> bool:
        """Whether interruption mode is enabled."""
        ...
    @property
    def asr_config(self) -> AsrConfig:
        """ASR configuration."""
        ...
    @property
    def llm_config(self) -> LlmConfig:
        """LLM prompt configuration."""
        ...
    @property
    def tts_config(self) -> TtsConfig:
        """TTS configuration."""
        ...
    @property
    def enable_face_tracking(self) -> bool:
        """Whether AI face tracking should start with chat."""
        ...

class SpeakParameter:
    """Payload for :meth:`AiClient.speak`."""
    def __init__(self, msg: str) -> None:
        """Create a speech request with plain text content."""
        ...
    @property
    def msg(self) -> str:
        """Text that should be spoken by the AI service."""
        ...

class LuiTtsConfig:
    """TTS startup configuration for :class:`LuiClient`."""
    def __init__(self, voice_type: str) -> None:
        """Create LUI TTS configuration."""
        ...
    @property
    def voice_type(self) -> str:
        """Voice profile identifier."""
        ...

class LuiTtsParameter:
    """Payload for :meth:`LuiClient.send_tts_text`."""
    def __init__(self, text: str) -> None:
        """Create a TTS text payload."""
        ...
    @property
    def text(self) -> str:
        """Text to synthesize via the LUI service."""
        ...

class GetModeResponse:
    """Response payload returned by ``BoosterClient.get_mode()``."""

    def __init__(self, mode: int) -> None:
        """Create a mode response from raw integer mode."""
        ...

    @property
    def mode(self) -> int:
        """Raw robot mode value."""
        ...

    def mode_enum(self) -> RobotMode | None:
        """Mode converted to ``RobotMode`` when known, else ``None``."""
        ...

class GetStatusResponse:
    """Response payload returned by ``BoosterClient.get_status()``."""

    def __init__(
        self,
        current_mode: int,
        current_body_control: int,
        current_actions: list[int],
    ) -> None:
        """Create a status response from raw integer fields."""
        ...

    @property
    def current_mode(self) -> int:
        """Raw current mode value."""
        ...

    @property
    def current_body_control(self) -> int:
        """Raw current body-control state value."""
        ...

    @property
    def current_actions(self) -> list[int]:
        """Raw list of active action identifiers."""
        ...

    def current_mode_enum(self) -> RobotMode | None:
        """Current mode converted to ``RobotMode`` when known."""
        ...

    def current_body_control_enum(self) -> BodyControl | None:
        """Body control converted to ``BodyControl`` when known."""
        ...

    def current_actions_enum(self) -> list[Action]:
        """Active actions converted to known ``Action`` values."""
        ...

class GetRobotInfoResponse:
    """Robot identity and firmware metadata response."""

    def __init__(
        self,
        name: str,
        nickname: str,
        version: str,
        model: str,
        serial_number: str,
    ) -> None:
        """Create robot info payload."""
        ...

    @property
    def name(self) -> str:
        """Robot name."""
        ...

    @property
    def nickname(self) -> str:
        """User-configured nickname."""
        ...

    @property
    def version(self) -> str:
        """Software/firmware version string."""
        ...

    @property
    def model(self) -> str:
        """Robot model identifier."""
        ...

    @property
    def serial_number(self) -> str:
        """Hardware serial number."""
        ...

class LoadCustomTrainedTrajResponse:
    """Response for ``load_custom_trained_traj`` containing trajectory id."""

    def __init__(self, tid: str) -> None:
        """Create trajectory-load response."""
        ...

    @property
    def tid(self) -> str:
        """Loaded trajectory identifier used by activate/unload APIs."""
        ...

class X5CameraGetStatusResponse:
    """Response payload returned by ``X5CameraClient.get_status()``."""

    def __init__(self, status: int) -> None:
        """Create camera status response from raw status integer."""
        ...

    @property
    def status(self) -> int:
        """Raw camera status value."""
        ...

    def status_enum(self) -> CameraControlStatus | None:
        """Status converted to ``CameraControlStatus`` when known."""
        ...

class DetectResults:
    """Single detection result produced by the vision service."""

    def __init__(
        self,
        xmin: int,
        ymin: int,
        xmax: int,
        ymax: int,
        position: list[float],
        tag: str,
        conf: float,
        rgb_mean: list[int],
    ) -> None:
        """Create a detection record."""
        ...

    @property
    def xmin(self) -> int:
        """Left pixel coordinate of bounding box."""
        ...

    @property
    def ymin(self) -> int:
        """Top pixel coordinate of bounding box."""
        ...

    @property
    def xmax(self) -> int:
        """Right pixel coordinate of bounding box."""
        ...

    @property
    def ymax(self) -> int:
        """Bottom pixel coordinate of bounding box."""
        ...

    @property
    def position(self) -> list[float]:
        """Estimated position vector for the detection."""
        ...

    @property
    def tag(self) -> str:
        """Detected class/tag label."""
        ...

    @property
    def conf(self) -> float:
        """Detection confidence score."""
        ...

    @property
    def rgb_mean(self) -> list[int]:
        """Mean RGB values over the detection region."""
        ...

class BoosterClient:
    """High-level locomotion and body-control client for the robot."""

    def __init__(self, startup_wait_sec: float | None = ...) -> None:
        """Create a client.

        Args:
            startup_wait_sec: Optional one-time wait before the first RPC call.
                ``None`` uses SDK default startup wait.
        """
        ...

    def change_mode(self, mode: RobotMode) -> None:
        """Request robot mode transition."""
        ...

    def get_mode(self) -> GetModeResponse:
        """Fetch current robot mode."""
        ...

    def get_status(self) -> GetStatusResponse:
        """Fetch current robot status summary."""
        ...

    def get_robot_info(self) -> GetRobotInfoResponse:
        """Fetch robot identity/version metadata."""
        ...

    def move_robot(self, vx: float, vy: float, vyaw: float) -> None:
        """Command base motion in body frame."""
        ...

    def rotate_head(self, pitch: float, yaw: float) -> None:
        """Rotate head to target pitch/yaw angles."""
        ...

    def wave_hand(self, action: HandAction) -> None:
        """Trigger hand wave/open-close style action."""
        ...

    def rotate_head_with_direction(
        self, pitch_direction: int, yaw_direction: int
    ) -> None:
        """Rotate head using directional step commands."""
        ...

    def lie_down(self) -> None:
        """Command robot to lie down."""
        ...

    def get_up(self) -> None:
        """Command robot to stand up."""
        ...

    def get_up_with_mode(self, mode: RobotMode) -> None:
        """Stand up and transition into specified mode."""
        ...

    def shoot(self) -> None:
        """Trigger shoot action."""
        ...

    def push_up(self) -> None:
        """Trigger push-up action."""
        ...

    def move_hand_end_effector(
        self,
        target_posture: Posture,
        time_millis: int,
        hand_index: Hand,
    ) -> None:
        """Move a hand end effector to target posture."""
        ...

    def move_hand_end_effector_with_aux(
        self,
        target_posture: Posture,
        aux_posture: Posture,
        time_millis: int,
        hand_index: Hand,
    ) -> None:
        """Move end effector with auxiliary posture constraint."""
        ...

    def move_hand_end_effector_v2(
        self,
        target_posture: Posture,
        time_millis: int,
        hand_index: Hand,
    ) -> None:
        """Move end effector using v2 behavior flag."""
        ...

    def stop_hand_end_effector(self) -> None:
        """Stop active hand end-effector motion."""
        ...

    def control_gripper(
        self,
        motion_param: GripperMotionParameter,
        mode: GripperControlMode,
        hand_index: Hand,
    ) -> None:
        """Control gripper position/force."""
        ...

    def get_frame_transform(self, src: Frame, dst: Frame) -> Transform:
        """Query transform between two robot frames."""
        ...

    def switch_hand_end_effector_control_mode(self, switch_on: bool) -> None:
        """Enable or disable hand end-effector control mode."""
        ...

    def handshake(self, action: HandAction) -> None:
        """Trigger handshake action variant."""
        ...

    def control_dexterous_hand(
        self,
        finger_params: list[DexterousFingerParameter],
        hand_index: Hand,
        hand_type: BoosterHandType,
    ) -> None:
        """Send low-level dexterous-hand finger commands."""
        ...

    def control_dexterous_hand_default(
        self,
        finger_params: list[DexterousFingerParameter],
        hand_index: Hand,
    ) -> None:
        """Control dexterous hand using default hand type."""
        ...

    def dance(self, dance_id: DanceId) -> None:
        """Start predefined dance gesture."""
        ...

    def play_sound(self, sound_file_path: str) -> None:
        """Play a sound file on robot audio output."""
        ...

    def stop_sound(self) -> None:
        """Stop currently playing sound."""
        ...

    def zero_torque_drag(self, active: bool) -> None:
        """Enable/disable zero-torque drag mode."""
        ...

    def record_trajectory(self, active: bool) -> None:
        """Start or stop trajectory recording."""
        ...

    def replay_trajectory(self, traj_file_path: str) -> None:
        """Replay a recorded trajectory from path."""
        ...

    def whole_body_dance(self, dance_id: WholeBodyDanceId) -> None:
        """Start whole-body dance routine."""
        ...

    def upper_body_custom_control(self, start: bool) -> None:
        """Toggle upper-body custom control mode."""
        ...

    def reset_odometry(self) -> None:
        """Reset base odometry estimate."""
        ...

    def load_custom_trained_traj(
        self,
        traj: CustomTrainedTraj,
    ) -> LoadCustomTrainedTrajResponse:
        """Load a custom trained trajectory and return its id."""
        ...

    def activate_custom_trained_traj(self, tid: str) -> None:
        """Activate previously loaded custom trajectory by id."""
        ...

    def unload_custom_trained_traj(self, tid: str) -> None:
        """Unload previously loaded custom trajectory by id."""
        ...

    def enter_wbc_gait(self) -> None:
        """Enter WBC gait mode."""
        ...

    def exit_wbc_gait(self) -> None:
        """Exit WBC gait mode."""
        ...

    def move_dual_hand_end_effector(
        self,
        left_target_posture: Posture,
        right_target_posture: Posture,
        time_millis: int,
    ) -> None:
        """Move both hand end-effectors to target postures simultaneously."""
        ...

    def visual_kick(self, start: bool) -> None:
        """Start or stop a visual kick (side-foot kick)."""
        ...

    def publish_gripper_command(self, command: GripperCommand) -> None:
        """Publish low-level gripper command message."""
        ...

    def publish_gripper(
        self,
        hand: Hand,
        mode: GripperMode,
        motion_param: int,
        speed: int | None = ...,
    ) -> None:
        """Convenience wrapper for publishing gripper command fields."""
        ...

class AiClient:
    """Client for AI chat and speech features."""

    def __init__(self, startup_wait_sec: float | None = ...) -> None:
        """Create AI client.

        Args:
            startup_wait_sec: Optional one-time wait before first RPC call.
        """
        ...

    def start_ai_chat(self, param: StartAiChatParameter) -> None:
        """Start AI chat session with provided config."""
        ...

    def stop_ai_chat(self) -> None:
        """Stop active AI chat session."""
        ...

    def speak(self, param: SpeakParameter) -> None:
        """Send message for AI speech output."""
        ...

    def start_face_tracking(self) -> None:
        """Enable AI face tracking mode."""
        ...

    def stop_face_tracking(self) -> None:
        """Disable AI face tracking mode."""
        ...

class LuiClient:
    """Client for LUI ASR/TTS APIs."""

    def __init__(self, startup_wait_sec: float | None = ...) -> None:
        """Create LUI client.

        Args:
            startup_wait_sec: Optional one-time wait before first RPC call.
        """
        ...

    def start_asr(self) -> None:
        """Start ASR stream."""
        ...

    def stop_asr(self) -> None:
        """Stop ASR stream."""
        ...

    def start_tts(self, config: LuiTtsConfig) -> None:
        """Start TTS engine with given configuration."""
        ...

    def stop_tts(self) -> None:
        """Stop TTS engine."""
        ...

    def send_tts_text(self, param: LuiTtsParameter) -> None:
        """Send text payload for TTS synthesis."""
        ...

class LightControlClient:
    """Client for LED light control APIs."""

    def __init__(self, startup_wait_sec: float | None = ...) -> None:
        """Create light-control client.

        Args:
            startup_wait_sec: Optional one-time wait before first RPC call.
        """
        ...

    def set_led_light_color(self, r: int, g: int, b: int) -> None:
        """Set LED strip color using RGB values (0-255)."""
        ...

    def stop_led_light_control(self) -> None:
        """Stop active LED control program/effect."""
        ...

class VisionClient:
    """Client for vision inference APIs."""

    def __init__(self, startup_wait_sec: float | None = ...) -> None:
        """Create vision client.

        Args:
            startup_wait_sec: Optional one-time wait before first RPC call.
        """
        ...

    def start_vision_service(
        self,
        enable_position: bool,
        enable_color: bool,
        enable_face_detection: bool,
    ) -> None:
        """Start vision service with selected feature flags."""
        ...

    def stop_vision_service(self) -> None:
        """Stop vision service."""
        ...

    def get_detection_object_with_ratio(
        self, focus_ratio: float
    ) -> list[DetectResults]:
        """Get detected objects using custom focus ratio."""
        ...

    def get_detection_object(self) -> list[DetectResults]:
        """Get detected objects using default focus ratio."""
        ...

class X5CameraClient:
    """Client for X5 camera control and status APIs."""

    def __init__(self, startup_wait_sec: float | None = ...) -> None:
        """Create X5 camera client.

        Args:
            startup_wait_sec: Optional one-time wait before first RPC call.
        """
        ...

    def change_mode(self, mode: CameraSetMode) -> None:
        """Change camera mode."""
        ...

    def get_status(self) -> X5CameraGetStatusResponse:
        """Get current camera status."""
        ...
