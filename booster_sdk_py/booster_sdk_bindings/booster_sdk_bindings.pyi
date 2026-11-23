"""Type stubs for booster_sdk_bindings."""

from __future__ import annotations

from typing import Any, Iterator

BOOSTER_ROBOT_USER_ID: str

class BoosterSdkError(Exception):
    """Base class of every Booster SDK error.

    ``status_code`` is ``None`` here; :class:`RpcError` subclasses carry the
    C++ SDK RPC status code (see the ``RPC_STATUS_CODE_*`` constants).
    """

    status_code: int | None

class DdsError(BoosterSdkError):
    """DDS transport failure."""

class ValidationError(BoosterSdkError):
    """Client-side argument validation failure."""

class RpcError(BoosterSdkError):
    """RPC failure carrying the C++ status code."""

    status_code: int

class RpcTimeoutError(RpcError):
    """The RPC (or endpoint discovery) timed out locally or remotely (status 100)."""

class RpcStatusError(RpcError):
    """The service answered with an error status (see ``status_code``)."""

RPC_STATUS_CODE_INVALID: int
RPC_STATUS_CODE_SUCCESS: int
RPC_STATUS_CODE_TIMEOUT: int
RPC_STATUS_CODE_BAD_REQUEST: int
RPC_STATUS_CODE_CONFLICT: int
RPC_STATUS_CODE_REQUEST_TOO_FREQUENT: int
RPC_STATUS_CODE_INTERNAL_SERVER_ERROR: int
RPC_STATUS_CODE_SERVER_REFUSED: int
RPC_STATUS_CODE_STATE_TRANSITION_FAILED: int
RPC_STATUS_CODE_LOW_BATTERY: int

TOPIC_AI_SUBTITLE: str
TOPIC_LUI_ASR_CHUNK: str
LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS: int
LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS: int
LUI_MAX_RECOGNIZE_AUDIO_BYTES: int
AUDIO_ERR_NOT_SUPPORTED: int
AUDIO_PROGRESS_TOPIC: str
AUDIO_EVENT_TOPIC: str
AUDIO_ERROR_TOPIC: str

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
    BOXING_STYLE_KICK: WholeBodyDanceId
    ROUNDHOUSE_KICK: WholeBodyDanceId
    SHAN_HE_GU_REN_DANCE: WholeBodyDanceId
    GAI_GE_CHUN_FENG_DANCE: WholeBodyDanceId
    MICHAEL_DANCE_1_AND_2: WholeBodyDanceId
    BOW_AND_ARROW: WholeBodyDanceId
    CHARLESTON_DANCE: WholeBodyDanceId

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same dance id."""
        ...

class VisualKickVersion:
    """Visual-kick behavior version for :meth:`BoosterClient.visual_kick_with_version`."""

    V1: VisualKickVersion
    V2: VisualKickVersion

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same version."""
        ...

class GaitType:
    """Gait selectors for :meth:`BoosterClient.switch_gait`."""

    WHOLE_BODY_HUMANLIKE_GAIT: GaitType
    HALF_BODY_HUMANLIKE_GAIT: GaitType
    HALF_BODY_HUMANLIKE_GAIT_V2: GaitType
    WHOLE_BODY_HUMANLIKE_GAIT_V2: GaitType

    def __repr__(self) -> str:
        """Return a stable enum-style representation."""
        ...
    def __int__(self) -> int:
        """Return the raw integer value used by the RPC API."""
        ...
    def __eq__(self, other: object) -> bool:
        """Return ``True`` when both values represent the same gait."""
        ...

class GetUpVersion:
    """Get-up behavior implementation."""

    V1: GetUpVersion
    V2: GetUpVersion

    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class DeviceInfoKind:
    """Kind of catalog returned by a device-discovery RPC."""

    UNKNOWN: DeviceInfoKind
    SENSORS: DeviceInfoKind
    HANDS: DeviceInfoKind
    ROBOT_MODEL: DeviceInfoKind
    CAMERA: DeviceInfoKind

    def __int__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

class DeviceInfo:
    """JSON-backed device catalog returned by SDK 1.7."""

    @property
    def kind(self) -> DeviceInfoKind: ...
    @property
    def body(self) -> object: ...
    def __repr__(self) -> str: ...

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
    LION_DANCE_PREPARE_POSE: BodyControl
    VISUAL_KICK_V1: BodyControl

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

class AiApiId:
    """AI chat RPC API ids (for ``send_api_request``)."""

    START_AI_CHAT: AiApiId
    STOP_AI_CHAT: AiApiId
    SPEAK: AiApiId
    START_FACE_TRACKING: AiApiId
    STOP_FACE_TRACKING: AiApiId

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AiApiId:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class TtsConfig:
    """Text-to-speech configuration for AI chat."""

    voice_type: str
    ignore_bracket_text: list[int]

    def __init__(
        self, voice_type: str = "", ignore_bracket_text: list[int] = []
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> TtsConfig:
        """Parse from a JSON body."""
        ...

class LlmConfig:
    """Prompt configuration for the AI chat LLM."""

    system_prompt: str
    welcome_msg: str
    prompt_name: str

    def __init__(
        self, system_prompt: str = "", welcome_msg: str = "", prompt_name: str = ""
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> LlmConfig:
        """Parse from a JSON body."""
        ...

class AsrConfig:
    """Speech interruption settings (duration threshold in ms, keywords)."""

    interrupt_speech_duration: int
    interrupt_keywords: list[str]

    def __init__(
        self, interrupt_speech_duration: int = 0, interrupt_keywords: list[str] = []
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> AsrConfig:
        """Parse from a JSON body."""
        ...

class StartAiChatParameter:
    """Configuration for :meth:`AiClient.start_ai_chat`.

    The nested configs are shared objects: ``param.llm_config.prompt_name = "x"``
    updates ``param``. Omitted configs default to empty ones.
    """

    interrupt_mode: bool
    asr_config: AsrConfig
    llm_config: LlmConfig
    tts_config: TtsConfig
    enable_face_tracking: bool
    persona_id: str | None

    def __init__(
        self,
        interrupt_mode: bool = False,
        asr_config: AsrConfig | None = None,
        llm_config: LlmConfig | None = None,
        tts_config: TtsConfig | None = None,
        enable_face_tracking: bool = False,
        persona_id: str | None = None,
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> StartAiChatParameter:
        """Parse from a JSON body."""
        ...

class SpeakParameter:
    """Payload for :meth:`AiClient.speak`."""

    msg: str

    def __init__(self, msg: str = "") -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> SpeakParameter:
        """Parse from a JSON body."""
        ...

class Subtitle:
    """AI chat subtitle sample (``rt/ai_subtitle``)."""

    @property
    def magic_number(self) -> str: ...
    @property
    def text(self) -> str: ...
    @property
    def language(self) -> str: ...
    @property
    def user_id(self) -> str: ...
    @property
    def seq(self) -> int: ...
    @property
    def definite(self) -> bool: ...
    @property
    def paragraph(self) -> bool: ...
    @property
    def round_id(self) -> int: ...

class LuiApiId:
    """LUI speech RPC API ids (for ``send_api_request``)."""

    START_ASR: LuiApiId
    STOP_ASR: LuiApiId
    START_TTS: LuiApiId
    STOP_TTS: LuiApiId
    SEND_TTS_TEXT: LuiApiId
    SYNTHESIZE_SPEECH: LuiApiId
    RECOGNIZE_AUDIO_ONCE: LuiApiId
    START_AUDIO_RECOGNIZER: LuiApiId
    STOP_AUDIO_RECOGNIZER: LuiApiId
    RECOGNIZE_AUDIO_IN_SESSION: LuiApiId

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> LuiApiId:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class LuiTtsConfig:
    """TTS startup configuration for :class:`LuiClient`."""

    voice_type: str

    def __init__(self, voice_type: str = "") -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> LuiTtsConfig:
        """Parse from a JSON body."""
        ...

class LuiTtsParameter:
    """Payload for :meth:`LuiClient.send_tts_text`."""

    text: str

    def __init__(self, text: str = "") -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> LuiTtsParameter:
        """Parse from a JSON body."""
        ...

class AsrUtterance:
    """Utterance entry inside an :class:`AsrChunk`."""

    @property
    def text(self) -> str: ...
    @property
    def definite(self) -> bool: ...
    @property
    def start_time_ms(self) -> int: ...
    @property
    def end_time_ms(self) -> int: ...
    @property
    def speaker_id(self) -> str: ...
    @property
    def emotion(self) -> str: ...
    @property
    def gender(self) -> str: ...
    @property
    def lid_lang(self) -> str: ...
    @property
    def speech_rate(self) -> float: ...
    @property
    def volume_db(self) -> float: ...
    @property
    def additions_json(self) -> str: ...

class AsrChunk:
    """Standalone LUI ASR chunk (``rt/lui_asr_chunk``)."""

    @property
    def session_id(self) -> str: ...
    @property
    def client_id(self) -> str: ...
    @property
    def role(self) -> str: ...
    @property
    def text(self) -> str: ...
    @property
    def is_final(self) -> bool: ...
    @property
    def start_time_ms(self) -> int: ...
    @property
    def end_time_ms(self) -> int: ...
    @property
    def speaker_id(self) -> str: ...
    @property
    def emotion(self) -> str: ...
    @property
    def gender(self) -> str: ...
    @property
    def lid_lang(self) -> str: ...
    @property
    def speech_rate(self) -> float: ...
    @property
    def volume_db(self) -> float: ...
    @property
    def provider_logid(self) -> str: ...
    @property
    def raw_json(self) -> str: ...
    @property
    def utterances(self) -> list[AsrUtterance]: ...

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
        edition: str = ...,
        region: str = ...,
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

    @property
    def edition(self) -> str:
        """Robot edition string."""
        ...

    @property
    def region(self) -> str:
        """Robot region string."""
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

class VisionApiId:
    """Vision RPC API ids."""

    START_VISION_SERVICE: VisionApiId
    STOP_VISION_SERVICE: VisionApiId
    GET_DETECTION_OBJECT: VisionApiId

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> VisionApiId:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class StartVisionServiceParameter:
    """Body of the start-vision-service request."""

    enable_position: bool
    enable_color: bool
    enable_face_detection: bool

    def __init__(
        self,
        enable_position: bool = False,
        enable_color: bool = False,
        enable_face_detection: bool = False,
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> StartVisionServiceParameter:
        """Parse from a JSON body."""
        ...

class GetDetectionObjectParameter:
    """Body of the get-detection-object request."""

    focus_ratio: float

    def __init__(self, focus_ratio: float = 0.33) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> GetDetectionObjectParameter:
        """Parse from a JSON body."""
        ...

class DetectResults:
    """Single detection result produced by the vision service."""

    xmin: int
    ymin: int
    xmax: int
    ymax: int
    position: list[float]
    tag: str
    conf: float
    rgb_mean: list[int]

    def __init__(
        self,
        xmin: int = 0,
        ymin: int = 0,
        xmax: int = 0,
        ymax: int = 0,
        position: list[float] = [],
        tag: str = "",
        conf: float = 0.0,
        rgb_mean: list[int] = [],
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> DetectResults:
        """Parse from a JSON body."""
        ...

class X5CameraApiId:
    """X5 camera RPC API ids."""

    CHANGE_MODE: X5CameraApiId
    GET_STATUS: X5CameraApiId

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> X5CameraApiId:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class X5CameraChangeModeParameter:
    """Body of the X5 change-mode request (C++ ``ChangeModeParameter``); ``mode`` is a raw ``CameraSetMode`` value."""

    mode: int

    def __init__(self, mode: int = 0) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> X5CameraChangeModeParameter:
        """Parse from a JSON body."""
        ...

class Subscription:
    """A DDS topic subscription.

    Samples are returned as the topic's message class. ``recv`` blocks with the
    GIL released; iteration yields samples until the subscription is closed.
    """

    def recv(self, timeout: float | None = None) -> Any | None:
        """Wait for the next sample; ``None`` on timeout or once closed."""
        ...

    def try_recv(self) -> Any | None:
        """Return the next queued sample, or ``None`` if none is pending."""
        ...

    def latest(self) -> Any | None:
        """Drain the queue and return only the newest sample, or ``None``."""
        ...

    def matched_publications(self) -> int:
        """Number of remote publishers currently matched."""
        ...

    def close(self) -> None:
        """Stop receiving and release the DDS reader."""
        ...

    def is_closed(self) -> bool: ...
    def __iter__(self) -> Iterator[Any]: ...
    def __next__(self) -> Any: ...
    def __enter__(self) -> Subscription: ...
    def __exit__(self, *args: object) -> None: ...

class Odometer:
    """Planar odometry sample (``rt/odometer_state``)."""

    x: float
    y: float
    theta: float

class LocoApiId:
    """Locomotion RPC API ids (for :meth:`BoosterClient.send_api_request` / :meth:`BoosterClient.start_operation`)."""

    CHANGE_MODE: LocoApiId
    MOVE: LocoApiId
    ROTATE_HEAD: LocoApiId
    WAVE_HAND: LocoApiId
    ROTATE_HEAD_WITH_DIRECTION: LocoApiId
    LIE_DOWN: LocoApiId
    GET_UP: LocoApiId
    MOVE_HAND_END_EFFECTOR: LocoApiId
    CONTROL_GRIPPER: LocoApiId
    GET_FRAME_TRANSFORM: LocoApiId
    SWITCH_HAND_END_EFFECTOR_CONTROL_MODE: LocoApiId
    CONTROL_DEXTEROUS_HAND: LocoApiId
    HANDSHAKE: LocoApiId
    DANCE: LocoApiId
    GET_MODE: LocoApiId
    GET_STATUS: LocoApiId
    PUSH_UP: LocoApiId
    PLAY_SOUND: LocoApiId
    STOP_SOUND: LocoApiId
    GET_ROBOT_INFO: LocoApiId
    STOP_HAND_END_EFFECTOR: LocoApiId
    SHOOT: LocoApiId
    GET_UP_WITH_MODE: LocoApiId
    ZERO_TORQUE_DRAG: LocoApiId
    RECORD_TRAJECTORY: LocoApiId
    REPLAY_TRAJECTORY: LocoApiId
    WHOLE_BODY_DANCE: LocoApiId
    UPPER_BODY_CUSTOM_CONTROL: LocoApiId
    RESET_ODOMETRY: LocoApiId
    LOAD_CUSTOM_TRAINED_TRAJ: LocoApiId
    ACTIVATE_CUSTOM_TRAINED_TRAJ: LocoApiId
    UNLOAD_CUSTOM_TRAINED_TRAJ: LocoApiId
    ENTER_WBC_GAIT: LocoApiId
    EXIT_WBC_GAIT: LocoApiId
    MOVE_DUAL_HAND_END_EFFECTOR: LocoApiId
    VISUAL_KICK: LocoApiId
    LION_DANCE_PREPARE: LocoApiId
    LION_DANCE_START: LocoApiId
    LION_DANCE_MOVE: LocoApiId
    SWITCH_GAIT: LocoApiId
    ROTATE_HEAD_WITH_TIME: LocoApiId
    GET_SENSORS: LocoApiId
    GET_HANDS: LocoApiId
    GET_ROBOT_MODEL: LocoApiId
    GET_TRAINED_TRAJ_STATUS: LocoApiId
    HAND_ON_CHEST_GREETING: LocoApiId

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> LocoApiId:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class JointIndex:
    """Joint indices of the 23-joint T1 layout (:data:`JOINT_CNT`)."""

    HEAD_YAW: JointIndex
    HEAD_PITCH: JointIndex
    LEFT_SHOULDER_PITCH: JointIndex
    LEFT_SHOULDER_ROLL: JointIndex
    LEFT_ELBOW_PITCH: JointIndex
    LEFT_ELBOW_YAW: JointIndex
    RIGHT_SHOULDER_PITCH: JointIndex
    RIGHT_SHOULDER_ROLL: JointIndex
    RIGHT_ELBOW_PITCH: JointIndex
    RIGHT_ELBOW_YAW: JointIndex
    WAIST: JointIndex
    LEFT_HIP_PITCH: JointIndex
    LEFT_HIP_ROLL: JointIndex
    LEFT_HIP_YAW: JointIndex
    LEFT_KNEE_PITCH: JointIndex
    CRANK_UP_LEFT: JointIndex
    CRANK_DOWN_LEFT: JointIndex
    RIGHT_HIP_PITCH: JointIndex
    RIGHT_HIP_ROLL: JointIndex
    RIGHT_HIP_YAW: JointIndex
    RIGHT_KNEE_PITCH: JointIndex
    CRANK_UP_RIGHT: JointIndex
    CRANK_DOWN_RIGHT: JointIndex

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> JointIndex:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class JointIndexK1:
    """Joint indices of the 22-joint K1 layout (:data:`JOINT_CNT_K1`)."""

    HEAD_YAW: JointIndexK1
    HEAD_PITCH: JointIndexK1
    LEFT_SHOULDER_PITCH: JointIndexK1
    LEFT_SHOULDER_ROLL: JointIndexK1
    LEFT_ELBOW_PITCH: JointIndexK1
    LEFT_ELBOW_YAW: JointIndexK1
    RIGHT_SHOULDER_PITCH: JointIndexK1
    RIGHT_SHOULDER_ROLL: JointIndexK1
    RIGHT_ELBOW_PITCH: JointIndexK1
    RIGHT_ELBOW_YAW: JointIndexK1
    LEFT_HIP_PITCH: JointIndexK1
    LEFT_HIP_ROLL: JointIndexK1
    LEFT_HIP_YAW: JointIndexK1
    LEFT_KNEE_PITCH: JointIndexK1
    CRANK_UP_LEFT: JointIndexK1
    CRANK_DOWN_LEFT: JointIndexK1
    RIGHT_HIP_PITCH: JointIndexK1
    RIGHT_HIP_ROLL: JointIndexK1
    RIGHT_HIP_YAW: JointIndexK1
    RIGHT_KNEE_PITCH: JointIndexK1
    CRANK_UP_RIGHT: JointIndexK1
    CRANK_DOWN_RIGHT: JointIndexK1

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> JointIndexK1:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class JointIndexWith7DofArm:
    """Joint indices of the 29-joint T1 7-DoF-arm layout (:data:`JOINT_CNT_7DOF_ARM`)."""

    HEAD_YAW: JointIndexWith7DofArm
    HEAD_PITCH: JointIndexWith7DofArm
    LEFT_SHOULDER_PITCH: JointIndexWith7DofArm
    LEFT_SHOULDER_ROLL: JointIndexWith7DofArm
    LEFT_ELBOW_PITCH: JointIndexWith7DofArm
    LEFT_ELBOW_YAW: JointIndexWith7DofArm
    LEFT_WRIST_PITCH: JointIndexWith7DofArm
    LEFT_WRIST_YAW: JointIndexWith7DofArm
    LEFT_HAND_ROLL: JointIndexWith7DofArm
    RIGHT_SHOULDER_PITCH: JointIndexWith7DofArm
    RIGHT_SHOULDER_ROLL: JointIndexWith7DofArm
    RIGHT_ELBOW_PITCH: JointIndexWith7DofArm
    RIGHT_ELBOW_YAW: JointIndexWith7DofArm
    RIGHT_WRIST_PITCH: JointIndexWith7DofArm
    RIGHT_WRIST_YAW: JointIndexWith7DofArm
    RIGHT_HAND_ROLL: JointIndexWith7DofArm
    WAIST: JointIndexWith7DofArm
    LEFT_HIP_PITCH: JointIndexWith7DofArm
    LEFT_HIP_ROLL: JointIndexWith7DofArm
    LEFT_HIP_YAW: JointIndexWith7DofArm
    LEFT_KNEE_PITCH: JointIndexWith7DofArm
    CRANK_UP_LEFT: JointIndexWith7DofArm
    CRANK_DOWN_LEFT: JointIndexWith7DofArm
    RIGHT_HIP_PITCH: JointIndexWith7DofArm
    RIGHT_HIP_ROLL: JointIndexWith7DofArm
    RIGHT_HIP_YAW: JointIndexWith7DofArm
    RIGHT_KNEE_PITCH: JointIndexWith7DofArm
    CRANK_UP_RIGHT: JointIndexWith7DofArm
    CRANK_DOWN_RIGHT: JointIndexWith7DofArm

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> JointIndexWith7DofArm:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class CmdType:
    """Joint space of a :class:`LowCmd` (parallel = ankle cranks, serial = URDF joints)."""

    PARALLEL: CmdType
    SERIAL: CmdType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> CmdType:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class RemoteControllerEvent:
    """Linux input event type of :attr:`RemoteControllerState.event`."""

    NONE: RemoteControllerEvent
    AXIS: RemoteControllerEvent
    HAT: RemoteControllerEvent
    BUTTON_DOWN: RemoteControllerEvent
    BUTTON_UP: RemoteControllerEvent
    REMOVE: RemoteControllerEvent

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> RemoteControllerEvent:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class ButtonEventType:
    """Physical button event type."""

    PRESS_DOWN: ButtonEventType
    PRESS_UP: ButtonEventType
    SINGLE_CLICK: ButtonEventType
    DOUBLE_CLICK: ButtonEventType
    TRIPLE_CLICK: ButtonEventType
    LONG_PRESS_START: ButtonEventType
    LONG_PRESS_HOLD: ButtonEventType
    LONG_PRESS_END: ButtonEventType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> ButtonEventType:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class FallDownStateType:
    """Fall-detection state."""

    IS_READY: FallDownStateType
    IS_FALLING: FallDownStateType
    HAS_FALLEN: FallDownStateType
    IS_GETTING_UP: FallDownStateType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> FallDownStateType:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class ProneBodyControlPosture:
    """Posture reported on ``rt/prone_body_control_status``."""

    UNKNOWN: ProneBodyControlPosture
    INACTIVE: ProneBodyControlPosture
    PUSH_UP: ProneBodyControlPosture
    LIE_DOWN: ProneBodyControlPosture
    SOCCER_LOCOMOTION: ProneBodyControlPosture
    SOCCER_KICKING: ProneBodyControlPosture

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> ProneBodyControlPosture:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class RobocupBehaviorStatus:
    """Soccer behavior state (enum; the topic message is :class:`RobocupBehaviorStatusMsg`)."""

    RUNNING: RobocupBehaviorStatus
    SHOOTING: RobocupBehaviorStatus
    PASSING: RobocupBehaviorStatus

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> RobocupBehaviorStatus:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class TrainedTrajStatus:
    """Trained-trajectory status (enum; the topic message is :class:`TrainedTrajStatusMsg`)."""

    IDLE: TrainedTrajStatus
    RUNNING: TrainedTrajStatus

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> TrainedTrajStatus:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class ArmJoint:
    """Arm joints controllable by :class:`ArmController` (indices 2-9)."""

    LEFT_PITCH: ArmJoint
    LEFT_ROLL: ArmJoint
    LEFT_YAW: ArmJoint
    LEFT_ELBOW: ArmJoint
    RIGHT_PITCH: ArmJoint
    RIGHT_ROLL: ArmJoint
    RIGHT_YAW: ArmJoint
    RIGHT_ELBOW: ArmJoint

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> ArmJoint:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class ApiOperationEventType:
    """Kind of asynchronous-operation event."""

    UNKNOWN: ApiOperationEventType
    PROGRESS: ApiOperationEventType
    FINISHED: ApiOperationEventType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> ApiOperationEventType:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class ApiOperationResultCode:
    """Final outcome of an asynchronous operation."""

    UNKNOWN: ApiOperationResultCode
    SUCCEEDED: ApiOperationResultCode
    ABORTED: ApiOperationResultCode
    CANCELED: ApiOperationResultCode
    REJECTED: ApiOperationResultCode
    TIMEOUT: ApiOperationResultCode

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> ApiOperationResultCode:
        """Convert a raw value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

JOINT_CNT: int
"""Joints in the T1 layout (:class:`JointIndex`)."""
JOINT_CNT_K1: int
"""Joints in the K1 layout (:class:`JointIndexK1`)."""
JOINT_CNT_7DOF_ARM: int
"""Joints in the T1 7-DoF-arm layout (:class:`JointIndexWith7DofArm`)."""

# --- Robot-state topic messages (read-only; from BoosterClient.subscribe_*) ---

class RobotDdsJointStatus:
    """Joint health entry of :class:`RobotStatusDdsMsg`."""

    @property
    def name(self) -> str: ...
    @property
    def index(self) -> int: ...
    @property
    def is_connected(self) -> bool: ...
    @property
    def temperature(self) -> int: ...
    @property
    def is_limited(self) -> int: ...
    @property
    def status_code(self) -> int: ...
    @property
    def temperature_level(self) -> int: ...

class RobotDdsImuStatus:
    """IMU health entry of :class:`RobotStatusDdsMsg`."""

    @property
    def name(self) -> str: ...
    @property
    def index(self) -> int: ...
    @property
    def is_connected(self) -> bool: ...
    @property
    def status_code(self) -> int: ...

class RobotDdsBatteryStatus:
    """Battery health entry of :class:`RobotStatusDdsMsg`."""

    @property
    def name(self) -> str: ...
    @property
    def temperature(self) -> float: ...
    @property
    def soc(self) -> float: ...
    @property
    def voltage(self) -> float: ...
    @property
    def status_code(self) -> int: ...
    @property
    def status_level(self) -> int: ...

class RobotStatusDdsMsg:
    """Device health (``rt/device_gateway``)."""

    @property
    def joint_vec(self) -> list[RobotDdsJointStatus]: ...
    @property
    def imu_vec(self) -> list[RobotDdsImuStatus]: ...
    @property
    def battery_vec(self) -> list[RobotDdsBatteryStatus]: ...

class MotionState:
    """Motion state (raw :class:`RobotMode` values)."""

    @property
    def current_mode(self) -> int: ...
    @property
    def target_mode(self) -> int: ...
    @property
    def is_transitioning(self) -> bool: ...

class BatteryState:
    """Battery state."""

    @property
    def voltage(self) -> float: ...
    @property
    def current(self) -> float: ...
    @property
    def soc(self) -> float: ...
    @property
    def average_voltage(self) -> float: ...

class ButtonEventMsg:
    """Physical button event."""

    @property
    def button(self) -> int: ...
    @property
    def event(self) -> ButtonEventType: ...

class RemoteControllerState:
    """Remote controller (gamepad) state."""

    @property
    def event(self) -> int:
        """Raw :class:`RemoteControllerEvent` value."""
        ...
    @property
    def lx(self) -> float: ...
    @property
    def ly(self) -> float: ...
    @property
    def rx(self) -> float: ...
    @property
    def ry(self) -> float: ...
    @property
    def a(self) -> bool: ...
    @property
    def b(self) -> bool: ...
    @property
    def x(self) -> bool: ...
    @property
    def y(self) -> bool: ...
    @property
    def lb(self) -> bool: ...
    @property
    def rb(self) -> bool: ...
    @property
    def lt(self) -> bool: ...
    @property
    def rt(self) -> bool: ...
    @property
    def ls(self) -> bool: ...
    @property
    def rs(self) -> bool: ...
    @property
    def back(self) -> bool: ...
    @property
    def start(self) -> bool: ...
    @property
    def hat_c(self) -> bool: ...
    @property
    def hat_u(self) -> bool: ...
    @property
    def hat_d(self) -> bool: ...
    @property
    def hat_l(self) -> bool: ...
    @property
    def hat_r(self) -> bool: ...
    @property
    def hat_lu(self) -> bool: ...
    @property
    def hat_ld(self) -> bool: ...
    @property
    def hat_ru(self) -> bool: ...
    @property
    def hat_rd(self) -> bool: ...
    @property
    def reserved(self) -> int: ...
    def event_kind(self) -> RemoteControllerEvent | None:
        """Decoded ``event``, or ``None`` if unknown."""
        ...

class RobotProcessStatus:
    """Process state entry of :class:`RobotProcessStateMsg`."""

    @property
    def name(self) -> str: ...
    @property
    def pid(self) -> int: ...
    @property
    def status(self) -> int: ...

class RobotProcessStateMsg:
    """Robot process states."""

    @property
    def process_vec(self) -> list[RobotProcessStatus]: ...

class BinaryData:
    """Binary payload (video stream)."""

    @property
    def data(self) -> bytes: ...
    @property
    def timestamp(self) -> int: ...
    @property
    def sequence_num(self) -> int: ...
    @property
    def encoding(self) -> str: ...

class FallDownState:
    """Fall and recovery state (``rt/fall_down``)."""

    @property
    def fall_down_state(self) -> FallDownStateType: ...
    @property
    def is_recovery_available(self) -> bool: ...

class RobotStatesMsg:
    """Mode, body control and actions (``rt/robot_states``)."""

    @property
    def current_mode(self) -> int: ...
    @property
    def current_body_control(self) -> int: ...
    @property
    def current_actions(self) -> list[int]: ...
    def mode(self) -> RobotMode | None:
        """Decoded ``current_mode``, or ``None`` if unknown."""
        ...
    def body_control(self) -> BodyControl | None:
        """Decoded ``current_body_control``, or ``None`` if unknown."""
        ...
    def actions(self) -> list[Action]:
        """Decoded ``current_actions`` (unknown values are skipped)."""
        ...

class ProneBodyControlStatus:
    """Prone-body / visual-kick status."""

    @property
    def posture(self) -> int: ...
    def posture_kind(self) -> ProneBodyControlPosture | None:
        """Decoded ``posture``, or ``None`` if unknown."""
        ...

class RobotReplayTrajID:
    """Id of the recorded trajectory being replayed."""

    @property
    def id(self) -> str: ...

class TrainedTrajStatusMsg:
    """Trained-trajectory status message (``rt/trained_traj_status``).

    Named ``...Msg`` because :class:`TrainedTrajStatus` is the status enum.
    """

    @property
    def status(self) -> int: ...
    @property
    def traj_id(self) -> str: ...
    def status_kind(self) -> TrainedTrajStatus | None:
        """Decoded ``status``, or ``None`` if unknown."""
        ...

class RobocupBehaviorStatusMsg:
    """Soccer behavior status message (``rt/robocup_behavior_status``).

    Named ``...Msg`` because :class:`RobocupBehaviorStatus` is the status enum.
    """

    @property
    def status(self) -> int: ...
    def status_kind(self) -> RobocupBehaviorStatus | None:
        """Decoded ``status``, or ``None`` if unknown."""
        ...

class GetTrainedTrajStatusResponse:
    """Response of :meth:`BoosterClient.get_trained_traj_status`."""

    def __init__(self, status: int, traj_id: str = "") -> None: ...
    @property
    def status(self) -> int: ...
    @property
    def traj_id(self) -> str: ...
    def is_running(self) -> bool: ...
    def status_enum(self) -> TrainedTrajStatus | None:
        """Decoded ``status``, or ``None`` if unknown."""
        ...

class HandReplyParam:
    """Per-finger feedback of :class:`HandReplyData`."""

    @property
    def angle(self) -> int: ...
    @property
    def force(self) -> int: ...
    @property
    def current(self) -> int: ...
    @property
    def error(self) -> int: ...
    @property
    def status(self) -> int: ...
    @property
    def temp(self) -> int: ...
    @property
    def seq(self) -> int: ...

class HandReplyData:
    """Dexterous-hand actuator feedback (``rt/booster_hand_data``)."""

    @property
    def hand_data(self) -> list[HandReplyParam]: ...
    @property
    def hand_index(self) -> int:
        """0 = left, 1 = right (see :class:`Hand`)."""
        ...
    @property
    def hand_type(self) -> int:
        """Raw :class:`BoosterHandType` value."""
        ...

class HandTouchParam:
    """Raw touch-sensor arrays per finger."""

    @property
    def finger_one(self) -> bytes: ...
    @property
    def finger_two(self) -> bytes: ...
    @property
    def finger_three(self) -> bytes: ...
    @property
    def finger_four(self) -> bytes: ...
    @property
    def finger_five(self) -> bytes: ...
    @property
    def finger_palm(self) -> bytes: ...

class HandTouchData:
    """Hand touch-sensor data (``rt/booster_hand_touch_data``)."""

    @property
    def touch_data(self) -> HandTouchParam: ...
    @property
    def hand_index(self) -> int: ...
    @property
    def hand_type(self) -> int: ...

# --- Low-level control (rt/joint_ctrl, rt/low_state) ---

class MotorCmd:
    """Command for one motor.

    Torque = ``tau + kp * (q - q_measured) + kd * (dq - dq_measured)``.
    """

    mode: int
    q: float
    dq: float
    tau: float
    kp: float
    kd: float
    weight: float

    def __init__(
        self,
        mode: int = 0,
        q: float = 0.0,
        dq: float = 0.0,
        tau: float = 0.0,
        kp: float = 0.0,
        kd: float = 0.0,
        weight: float = 0.0,
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(json: str) -> MotorCmd: ...

class LowCmd:
    """Low-level joint command (``rt/joint_ctrl``).

    ``motor_cmd`` must have one entry per joint of the robot's layout
    (:data:`JOINT_CNT`, :data:`JOINT_CNT_K1` or :data:`JOINT_CNT_7DOF_ARM`).
    It is a regular list of :class:`MotorCmd` objects, so
    ``cmd.motor_cmd[i].q = 0.3`` edits the command in place.
    """

    cmd_type: CmdType
    motor_cmd: list[MotorCmd]

    def __init__(
        self,
        cmd_type: CmdType = ...,
        joint_count: int = 0,
        motor_cmd: list[MotorCmd] | None = None,
    ) -> None:
        """Create ``joint_count`` zeroed motor commands (or use ``motor_cmd``).

        ``cmd_type`` defaults to ``CmdType.PARALLEL``.
        """
        ...
    def __len__(self) -> int: ...

class LowCmdPublisher:
    """Publisher for :class:`LowCmd` on ``rt/joint_ctrl``."""

    def write(self, cmd: LowCmd) -> None: ...
    def matched_subscriptions(self) -> int:
        """Number of matched subscribers seen so far."""
        ...

class ImuState:
    """IMU sample of :class:`LowState`."""

    @property
    def rpy(self) -> list[float]:
        """Roll, pitch, yaw (rad)."""
        ...
    @property
    def gyro(self) -> list[float]:
        """Angular velocity (rad/s)."""
        ...
    @property
    def acc(self) -> list[float]:
        """Linear acceleration (m/s^2)."""
        ...

class MotorState:
    """State of one motor."""

    @property
    def mode(self) -> int: ...
    @property
    def q(self) -> float: ...
    @property
    def dq(self) -> float: ...
    @property
    def ddq(self) -> float: ...
    @property
    def tau_est(self) -> float: ...
    @property
    def temperature(self) -> int: ...
    @property
    def lost(self) -> int: ...
    @property
    def reserve(self) -> list[int]: ...

class LowState:
    """Low-level motor and IMU state (``rt/low_state``)."""

    @property
    def imu_state(self) -> ImuState: ...
    @property
    def motor_state_parallel(self) -> list[MotorState]: ...
    @property
    def motor_state_serial(self) -> list[MotorState]: ...

# --- ROS 2 messages (rt/kick_ball, rt/tf) ---

class Time:
    """``builtin_interfaces/Time``."""

    sec: int
    nanosec: int

    def __init__(self, sec: int = 0, nanosec: int = 0) -> None: ...
    @staticmethod
    def now() -> Time:
        """Current wall-clock time (UNIX epoch)."""
        ...
    def as_secs_f64(self) -> float: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(json: str) -> Time: ...

class Header:
    """``std_msgs/Header``. ``stamp`` is held by reference."""

    stamp: Time
    frame_id: str

    def __init__(self, stamp: Time | None = None, frame_id: str = "") -> None: ...

class Kick:
    """Kick reference command (``rt/kick_ball``, K1 / T1). ``header`` is held by reference."""

    header: Header
    x: float
    y: float
    dir: float
    goal_x: float
    goal_y: float
    robot_theta_to_field: float
    power: float

    def __init__(
        self,
        header: Header | None = None,
        x: float = 0.0,
        y: float = 0.0,
        dir: float = 0.0,
        goal_x: float = 0.0,
        goal_y: float = 0.0,
        robot_theta_to_field: float = 0.0,
        power: float = 0.0,
    ) -> None: ...

class KickPublisher:
    """Publisher for :class:`Kick` on ``rt/kick_ball``."""

    def write(self, kick: Kick) -> None: ...
    def matched_subscriptions(self) -> int:
        """Number of matched subscribers seen so far."""
        ...

class RosQuaternion:
    """``geometry_msgs/Quaternion`` (f64; :class:`Quaternion` is the RPC type)."""

    @property
    def x(self) -> float: ...
    @property
    def y(self) -> float: ...
    @property
    def z(self) -> float: ...
    @property
    def w(self) -> float: ...

class RosTransform:
    """``geometry_msgs/Transform`` (:class:`Transform` is the RPC type)."""

    @property
    def translation(self) -> Vector3: ...
    @property
    def rotation(self) -> RosQuaternion: ...

class TransformStamped:
    """``geometry_msgs/TransformStamped``."""

    @property
    def header(self) -> Header:
        """A copy of the header."""
        ...
    @property
    def child_frame_id(self) -> str: ...
    @property
    def transform(self) -> RosTransform: ...

class TFMessage:
    """Transform tree (``rt/tf``)."""

    @property
    def transforms(self) -> list[TransformStamped]: ...

# --- Asynchronous operations ---

class ApiOperationProgress:
    """Progress event of an :class:`ApiOperation`."""

    @property
    def event_type(self) -> ApiOperationEventType: ...
    @property
    def operation_id(self) -> str: ...
    @property
    def api_id(self) -> int: ...
    @property
    def status(self) -> int: ...
    @property
    def message(self) -> str: ...
    @property
    def body(self) -> str: ...

class ApiOperationResult:
    """Final result of an :class:`ApiOperation`."""

    @property
    def event_type(self) -> ApiOperationEventType: ...
    @property
    def code(self) -> ApiOperationResultCode: ...
    @property
    def operation_id(self) -> str: ...
    @property
    def api_id(self) -> int: ...
    @property
    def status(self) -> int: ...
    @property
    def message(self) -> str: ...
    @property
    def body(self) -> str: ...
    def is_success(self) -> bool:
        """``code == SUCCEEDED`` and ``status == 0``."""
        ...
    def body_or_raise(self) -> str:
        """Return ``body`` on success, else raise the matching :class:`RpcError`."""
        ...

class ApiOperation:
    """A started locomotion operation (:meth:`BoosterClient.start_operation`).

    ``next_event`` / ``wait`` block with the GIL released and honour Ctrl-C.
    Dropping the object stops event delivery but does not cancel the
    operation on the robot; call :meth:`cancel` for that.
    """

    @property
    def operation_id(self) -> str: ...
    @property
    def api_id(self) -> int: ...
    @property
    def done(self) -> bool:
        """Whether the final result has been received."""
        ...
    def next_event(
        self, timeout: float | None = None
    ) -> ApiOperationProgress | ApiOperationResult | None:
        """Next event; ``None`` on timeout or after the final result."""
        ...
    def wait(self, timeout: float | None = None) -> ApiOperationResult:
        """Wait for the final result, skipping progress events.

        Raises :class:`RpcTimeoutError` on timeout and :class:`RpcStatusError`
        if the result was already consumed (event stream closed).
        """
        ...
    def cancel(self) -> None:
        """Ask the service to cancel; the result (usually ``CANCELED``) still arrives."""
        ...

# --- Controllers ---

class MoveController:
    """Odometry-based point-to-point locomotion (C++ ``b1::MoveController``).

    Requires a walking mode. Construction blocks ~2-7 s: it waits up to 5 s
    for odometry (raising :class:`BoosterSdkError` if none arrives), then
    resets odometry. Motions block with the GIL released and have no overall
    timeout; Ctrl-C cancels the motion, sends a stop and raises
    ``KeyboardInterrupt``. Use as a context manager (or call :meth:`close`)
    to stop the robot when done.

    Pass either an existing ``client`` or ``robot_name``/``network``/``domain_id``.
    """

    def __init__(
        self,
        *,
        client: BoosterClient | None = None,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None: ...
    @property
    def client(self) -> BoosterClient: ...
    @property
    def closed(self) -> bool: ...
    def pose(self) -> tuple[float, float, float]:
        """Latest odometry pose ``(x, y, theta)``."""
        ...
    def move_to_target(self, x: float, y: float, vel: float) -> None:
        """Walk to odometry-frame ``(x, y)`` with speed limit ``vel`` (m/s)."""
        ...
    def move_to_relative(self, dx: float, dy: float, vel: float) -> None:
        """Walk ``(dx, dy)`` relative to the current position."""
        ...
    def turn_around(self, angle: float, vel: float) -> None:
        """Turn in place by ``|angle|`` rad at ``|vel|`` rad/s.

        As in the C++ SDK, a positive ``angle`` commands a negative yaw rate.
        """
        ...
    def stop(self) -> None:
        """Command zero velocity."""
        ...
    def close(self) -> None:
        """Stop and release the odometry subscription (idempotent)."""
        ...
    def __enter__(self) -> MoveController: ...
    def __exit__(self, *args: object) -> None: ...

class ArmController:
    """Interpolated arm-joint control via ``rt/joint_ctrl`` (C++ ``b1::ArmController``).

    Construction enables upper-body custom control and blocks ~2.5-7.5 s;
    :meth:`close` (or leaving the ``with`` block) disables it again.
    ``joint_count`` is the low-level layout size (``JOINT_CNT_K1`` on a K1).
    """

    def __init__(
        self,
        *,
        joint_count: int = ...,
        client: BoosterClient | None = None,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """``joint_count`` defaults to :data:`JOINT_CNT`."""
        ...
    @property
    def client(self) -> BoosterClient: ...
    @property
    def closed(self) -> bool: ...
    def joint_positions(self) -> list[float]:
        """Latest measured joint positions (parallel layout)."""
        ...
    def control_arm(
        self, joint: ArmJoint | int, target_rad: float, duration_ms: float
    ) -> ArmController:
        """Queue a move (min 50 ms); returns ``self`` for chaining.

        Raises ``ValueError`` for joints outside 2-9. Nothing moves until
        :meth:`finish`.
        """
        ...
    def finish(self) -> None:
        """Execute the queued moves at 100 Hz (GIL released).

        Ctrl-C stops publishing and raises ``KeyboardInterrupt`` (the queue is kept).
        """
        ...
    def close(self) -> None:
        """Disable upper-body custom control and close the channels (idempotent)."""
        ...
    def __enter__(self) -> ArmController: ...
    def __exit__(self, *args: object) -> None: ...

class BoosterClient:
    """High-level locomotion and body-control client for the robot."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a client.

        Args:
            startup_wait_sec: Optional one-time wait before the first RPC call.
                ``None`` uses SDK default startup wait.
            robot_name: Target ``rt/LocoApiTopic/<robot_name>`` (C++
                ``Init(robot_name)``).
            network: ``"localhost"``, ``"all"`` or interface names / IP
                addresses (a list or a comma-separated string).
            domain_id: DDS domain id.

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        """
        ...

    def subscribe_odometer(self) -> Subscription:
        """Subscribe to planar odometry; samples are :class:`Odometer`."""
        ...

    def move_command(self, vx: float, vy: float, vyaw: float) -> None:
        """Send a fire-and-forget velocity command (C++ ``MoveCommand``).

        Does not wait for a response (suited to control loops). Raises
        :class:`RpcTimeoutError` if the service has not been discovered yet.
        """
        ...

    def wait_for_service(
        self, timeout_sec: float, require_response_path: bool = False
    ) -> bool:
        """Wait until the locomotion service is discovered (C++ ``WaitForService``)."""
        ...

    def get_trained_traj_status(self) -> GetTrainedTrajStatusResponse:
        """Query trained-trajectory execution status."""
        ...

    def hand_on_chest_greeting(self, up: bool) -> None:
        """Raise (``up=True``) or lower the hand-on-chest greeting (T2 only)."""
        ...

    def send_api_request(self, api_id: LocoApiId | int, param: str = "") -> str:
        """Send a raw request and return the raw response body."""
        ...

    def start_operation(
        self,
        api_id: LocoApiId | int,
        param: str = "",
        start_timeout_sec: float = 1.0,
    ) -> ApiOperation:
        """Start a request as an asynchronous operation (SDK 1.8).

        Returns once the service accepted it; progress and the final result
        arrive through the returned :class:`ApiOperation`.
        """
        ...

    def cancel_operation(self, operation: ApiOperation) -> None:
        """Cancel an operation (same as ``operation.cancel()``)."""
        ...

    def publish_light_control(self, pixels: list[tuple[int, int, int]]) -> None:
        """Publish an LED strip command (one ``(r, g, b)`` tuple per LED)."""
        ...

    def enter_safe_mode(self, data: bytes = b"") -> None:
        """Publish a safe-mode message with a raw payload."""
        ...

    def low_cmd_publisher(self) -> LowCmdPublisher:
        """Create a publisher for low-level joint commands (``rt/joint_ctrl``).

        Only honoured in custom mode / upper-body custom control; the command's
        joint layout must match the robot model.
        """
        ...

    def kick_publisher(self) -> KickPublisher:
        """Create a publisher for kick reference commands (``rt/kick_ball``)."""
        ...

    def subscribe_device_gateway(self) -> Subscription:
        """Device health; samples are :class:`RobotStatusDdsMsg`."""
        ...

    def subscribe_motion_state(self) -> Subscription:
        """Samples are :class:`MotionState`."""
        ...

    def subscribe_battery_state(self) -> Subscription:
        """Samples are :class:`BatteryState`."""
        ...

    def subscribe_button_events(self) -> Subscription:
        """Samples are :class:`ButtonEventMsg`."""
        ...

    def subscribe_remote_controller(self) -> Subscription:
        """Samples are :class:`RemoteControllerState`."""
        ...

    def subscribe_process_state(self) -> Subscription:
        """Samples are :class:`RobotProcessStateMsg`."""
        ...

    def subscribe_video_stream(self) -> Subscription:
        """Samples are :class:`BinaryData`."""
        ...

    def subscribe_low_state(self) -> Subscription:
        """Motor and IMU state (``rt/low_state``); samples are :class:`LowState`."""
        ...

    def subscribe_fall_down(self) -> Subscription:
        """Samples are :class:`FallDownState`."""
        ...

    def subscribe_robot_states(self) -> Subscription:
        """Samples are :class:`RobotStatesMsg`."""
        ...

    def subscribe_prone_body_control_status(self) -> Subscription:
        """Samples are :class:`ProneBodyControlStatus`."""
        ...

    def subscribe_robot_replay_traj_id(self) -> Subscription:
        """Samples are :class:`RobotReplayTrajID`."""
        ...

    def subscribe_trained_traj_status(self) -> Subscription:
        """Samples are :class:`TrainedTrajStatusMsg`."""
        ...

    def subscribe_robocup_behavior_status(self) -> Subscription:
        """Samples are :class:`RobocupBehaviorStatusMsg` (K1 / T1)."""
        ...

    def subscribe_hand_data(self) -> Subscription:
        """Samples are :class:`HandReplyData` (needs a compatible hand)."""
        ...

    def subscribe_hand_touch_data(self) -> Subscription:
        """Samples are :class:`HandTouchData` (needs a touch hand)."""
        ...

    def subscribe_tf(self) -> Subscription:
        """Transform tree (``rt/tf``); samples are :class:`TFMessage`."""
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

    def rotate_head_with_time(self, pitch: float, yaw: float, time_millis: int) -> None:
        """Rotate head to target angles within the requested duration."""
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

    def get_up(self, version: GetUpVersion | None = ...) -> None:
        """Command robot to stand up."""
        ...

    def get_up_with_mode(
        self, mode: RobotMode, version: GetUpVersion | None = ...
    ) -> None:
        """Stand up and transition into specified mode."""
        ...

    def get_sensors(self) -> DeviceInfo:
        """Fetch the robot's static IMU sensor catalog."""
        ...

    def get_hands(self) -> DeviceInfo:
        """Fetch the robot's hand end-effector catalog."""
        ...

    def get_robot_model(self) -> DeviceInfo:
        """Fetch the URDF-derived robot model."""
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

    def reset_odometry_to(self, x: float, y: float, theta: float) -> None:
        """Reset odometry to a target pose (meters, radians)."""
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
        """Start or stop a visual kick (side-foot kick) using v1.6 V2 behavior."""
        ...

    def visual_kick_with_version(self, start: bool, version: VisualKickVersion) -> None:
        """Start or stop visual kick using a specific v1.6 behavior version."""
        ...

    def lion_dance_prepare(self, start: bool) -> None:
        """Start or stop lion-dance preparation."""
        ...

    def lion_dance_start(self, dance_idx: int) -> None:
        """Start lion dance routine by dance index."""
        ...

    def lion_dance_move(self, start: bool) -> None:
        """Start or stop lion-dance movement."""
        ...

    def switch_gait(self, gait_type: GaitType) -> None:
        """Switch between supported humanlike gait controllers."""
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

class AudioSourceType:
    """Audio player source type."""

    PCM_FILE: AudioSourceType
    WAV_FILE: AudioSourceType
    PCM_STREAM: AudioSourceType
    MP3_FILE: AudioSourceType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioSourceType:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class PlayerPriority:
    """Player priority."""

    LOW: PlayerPriority
    MEDIUM: PlayerPriority
    HIGH: PlayerPriority

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> PlayerPriority:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class PlayerState:
    """Player state."""

    IDLE: PlayerState
    READY: PlayerState
    PLAYING: PlayerState
    PAUSED: PlayerState
    STOPPED: PlayerState
    COMPLETED: PlayerState
    ERROR: PlayerState

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> PlayerState:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class RecorderState:
    """Recorder state."""

    IDLE: RecorderState
    READY: RecorderState
    RECORDING: RecorderState
    PAUSED: RecorderState
    STOPPED: RecorderState
    ERROR: RecorderState

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> RecorderState:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioCaptureStreamState:
    """Capture stream state."""

    IDLE: AudioCaptureStreamState
    READY: AudioCaptureStreamState
    STREAMING: AudioCaptureStreamState
    PAUSED: AudioCaptureStreamState
    STOPPED: AudioCaptureStreamState
    ERROR: AudioCaptureStreamState

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioCaptureStreamState:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioDeviceQueryType:
    """Direction used when querying audio devices."""

    INPUTS: AudioDeviceQueryType
    OUTPUTS: AudioDeviceQueryType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioDeviceQueryType:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioDeviceDirection:
    """Audio device direction."""

    INPUT: AudioDeviceDirection
    OUTPUT: AudioDeviceDirection

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioDeviceDirection:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioDeviceTransport:
    """Audio device transport."""

    BUILTIN: AudioDeviceTransport
    USB: AudioDeviceTransport
    BLUETOOTH: AudioDeviceTransport
    VIRTUAL: AudioDeviceTransport
    UNKNOWN: AudioDeviceTransport

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioDeviceTransport:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioDeviceBackendAffinity:
    """Backend responsible for an audio device."""

    PULSE: AudioDeviceBackendAffinity
    BOOSTER_AEC_ARRAY: AudioDeviceBackendAffinity
    ALSA_DIAGNOSTIC: AudioDeviceBackendAffinity

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioDeviceBackendAffinity:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class BluetoothScanState:
    """Bluetooth scanning state."""

    IDLE: BluetoothScanState
    SCANNING: BluetoothScanState

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> BluetoothScanState:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class BluetoothDeviceState:
    """Bluetooth device state."""

    UNKNOWN: BluetoothDeviceState
    PAIRED: BluetoothDeviceState
    CONNECTING: BluetoothDeviceState
    CONNECTED: BluetoothDeviceState
    DISCONNECTING: BluetoothDeviceState
    FAILED: BluetoothDeviceState

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> BluetoothDeviceState:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class BluetoothMajorClass:
    """Bluetooth major device class."""

    AUDIO: BluetoothMajorClass
    PERIPHERAL: BluetoothMajorClass
    PHONE: BluetoothMajorClass
    COMPUTER: BluetoothMajorClass
    OTHER: BluetoothMajorClass

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> BluetoothMajorClass:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class BluetoothAudioProfile:
    """Bluetooth audio profile."""

    NONE: BluetoothAudioProfile
    A2DP_SINK: BluetoothAudioProfile
    A2DP_SOURCE: BluetoothAudioProfile
    HFP_HEADSET: BluetoothAudioProfile
    HFP_AG: BluetoothAudioProfile

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> BluetoothAudioProfile:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class SessionType:
    """Kind of audio session a topic message refers to."""

    PLAYER: SessionType
    RECORDER: SessionType
    LOCALIZER: SessionType
    CAPTURE_STREAM: SessionType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> SessionType:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioErrorCode:
    """Service-level audio error code."""

    SERVICE_DEGRADED: AudioErrorCode
    INVALID_STATE: AudioErrorCode
    DEVICE_BUSY: AudioErrorCode
    NOT_SUPPORTED: AudioErrorCode
    LOCALIZATION_UNAVAILABLE: AudioErrorCode
    ALSA_DEVICE_NOT_FOUND: AudioErrorCode
    ALGO_INIT_FAILED: AudioErrorCode
    ALGO_PROCESS_FAILED: AudioErrorCode
    SYSTEM_VOLUME_CTRL_FAILED: AudioErrorCode
    SYSTEM_MUTE_CTRL_FAILED: AudioErrorCode
    MEDIA_DECODE_FAILED: AudioErrorCode
    MEDIA_ENCODE_FAILED: AudioErrorCode
    MEDIA_FORMAT_UNSUPPORTED: AudioErrorCode

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioErrorCode:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioEventCode:
    """Event code carried by :class:`AudioEventTopic`."""

    PLAYER_INIT_SUCCEEDED: AudioEventCode
    PLAYER_START_SUCCEEDED: AudioEventCode
    PLAYER_PAUSE_SUCCEEDED: AudioEventCode
    PLAYER_STOP_SUCCEEDED: AudioEventCode
    PLAYER_RESET_SUCCEEDED: AudioEventCode
    PLAYER_DESTROY_SUCCEEDED: AudioEventCode
    RECORDER_INIT_SUCCEEDED: AudioEventCode
    RECORDER_START_SUCCEEDED: AudioEventCode
    RECORDER_PAUSE_SUCCEEDED: AudioEventCode
    RECORDER_STOP_SUCCEEDED: AudioEventCode
    RECORDER_DESTROY_SUCCEEDED: AudioEventCode
    PLAYER_TERMINAL_STATE_CHANGED: AudioEventCode
    SYSTEM_VOLUME_SET_SUCCEEDED: AudioEventCode
    SYSTEM_MUTE_SET_SUCCEEDED: AudioEventCode
    CAPTURE_STREAM_INIT_SUCCEEDED: AudioEventCode
    CAPTURE_STREAM_START_SUCCEEDED: AudioEventCode
    CAPTURE_STREAM_PAUSE_SUCCEEDED: AudioEventCode
    CAPTURE_STREAM_STOP_SUCCEEDED: AudioEventCode
    CAPTURE_STREAM_DESTROY_SUCCEEDED: AudioEventCode

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioEventCode:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioErrorCategory:
    """Error category carried by :class:`AudioErrorTopic`."""

    RPC_ERROR: AudioErrorCategory
    PLAYBACK_ERROR: AudioErrorCategory
    RECORD_ERROR: AudioErrorCategory
    CAPTURE_STREAM_RUNTIME_ERROR: AudioErrorCategory
    LOCALIZATION_ERROR: AudioErrorCategory
    SYSTEM_VOLUME_ERROR: AudioErrorCategory
    SYSTEM_MUTE_ERROR: AudioErrorCategory
    INTERNAL_ERROR: AudioErrorCategory

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioErrorCategory:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class AudioErrorDetail:
    """Operation an :class:`AudioErrorTopic` refers to."""

    NONE: AudioErrorDetail
    INIT_PLAYER: AudioErrorDetail
    START_PLAYER: AudioErrorDetail
    PAUSE_PLAYER: AudioErrorDetail
    STOP_PLAYER: AudioErrorDetail
    RESET_PLAYER: AudioErrorDetail
    DESTROY_PLAYER: AudioErrorDetail
    GET_PLAYER_INFO: AudioErrorDetail
    SEND_PCM_DATA: AudioErrorDetail
    INIT_RECORDER: AudioErrorDetail
    START_RECORDER: AudioErrorDetail
    PAUSE_RECORDER: AudioErrorDetail
    STOP_RECORDER: AudioErrorDetail
    DESTROY_RECORDER: AudioErrorDetail
    GET_RECORDER_INFO: AudioErrorDetail
    GET_DOA_ANGLE: AudioErrorDetail
    SET_SYSTEM_VOLUME: AudioErrorDetail
    GET_SYSTEM_VOLUME: AudioErrorDetail
    SET_SYSTEM_MUTE: AudioErrorDetail
    GET_SYSTEM_MUTE: AudioErrorDetail
    INIT_CAPTURE_STREAM: AudioErrorDetail
    START_CAPTURE_STREAM: AudioErrorDetail
    PAUSE_CAPTURE_STREAM: AudioErrorDetail
    STOP_CAPTURE_STREAM: AudioErrorDetail
    DESTROY_CAPTURE_STREAM: AudioErrorDetail
    GET_CAPTURE_STREAM_INFO: AudioErrorDetail

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> AudioErrorDetail:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class PcmFormat:
    """PCM format descriptor."""

    sample_rate_hz: int
    channels: int
    bits_per_sample: int

    def __init__(
        self, sample_rate_hz: int = 16000, channels: int = 1, bits_per_sample: int = 16
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> PcmFormat:
        """Parse from a JSON body."""
        ...

class PlayerInitOptions:
    """Player initialization options (C++ defaults)."""

    source_type: AudioSourceType
    source_uri: str
    sample_rate_hz: int
    channels: int
    bits_per_sample: int
    priority: PlayerPriority

    def __init__(
        self,
        source_type: AudioSourceType = AudioSourceType.PCM_FILE,
        source_uri: str = "",
        sample_rate_hz: int = 16000,
        channels: int = 1,
        bits_per_sample: int = 16,
        priority: PlayerPriority = PlayerPriority.MEDIUM,
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> PlayerInitOptions:
        """Parse from a JSON body."""
        ...
    @staticmethod
    def pcm_stream() -> PlayerInitOptions:
        """Options for a PCM stream player fed through ``send_pcm_data``."""
        ...

class RecorderInitOptions:
    """Recorder initialization options (C++ defaults)."""

    output_path: str
    sample_rate_hz: int
    channels: int
    bits_per_sample: int

    def __init__(
        self,
        output_path: str = "",
        sample_rate_hz: int = 16000,
        channels: int = 1,
        bits_per_sample: int = 16,
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> RecorderInitOptions:
        """Parse from a JSON body."""
        ...

class AudioCaptureStreamOptions:
    """Capture stream options. ``requested_raw_format`` defaults to 16 kHz, 3 channels, 16-bit;
    reading it returns a copy, so assign a whole ``PcmFormat`` to change it.
    """

    enable_raw_pcm: bool
    enable_naec_pcm: bool
    requested_raw_format: PcmFormat

    def __init__(
        self,
        enable_raw_pcm: bool = True,
        enable_naec_pcm: bool = False,
        requested_raw_format: PcmFormat = ...,
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> AudioCaptureStreamOptions:
        """Parse from a JSON body."""
        ...

class BluetoothScanOptions:
    """Bluetooth discovery options."""

    timeout_ms: int
    audio_only: bool

    def __init__(self, timeout_ms: int = 30000, audio_only: bool = True) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> BluetoothScanOptions:
        """Parse from a JSON body."""
        ...

class BluetoothConnectOptions:
    """Bluetooth connect options."""

    address: str
    auto_pair: bool
    make_default: bool
    preferred_profile: BluetoothAudioProfile
    timeout_ms: int

    def __init__(
        self,
        address: str = "",
        auto_pair: bool = True,
        make_default: bool = True,
        preferred_profile: BluetoothAudioProfile = BluetoothAudioProfile.NONE,
        timeout_ms: int = 15000,
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> BluetoothConnectOptions:
        """Parse from a JSON body."""
        ...

class AudioError:
    """Detailed audio error (C++ ``AudioError``)."""

    ret_code: int
    ret_msg: str
    error_category: int
    error_detail: int

    def __init__(
        self,
        ret_code: int = 0,
        ret_msg: str = "",
        error_category: int = 0,
        error_detail: int = 0,
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> AudioError:
        """Parse from a JSON body."""
        ...
    def category_enum(self) -> AudioErrorCategory | None: ...
    def detail_enum(self) -> AudioErrorDetail | None: ...

class InitPlayerResponse:
    """Response of ``init_player``."""

    @property
    def ret_code(self) -> int: ...
    @property
    def ret_msg(self) -> str: ...
    @property
    def session_id(self) -> int: ...

class InitRecorderResponse:
    """Response of ``init_recorder``."""

    @property
    def ret_code(self) -> int: ...
    @property
    def ret_msg(self) -> str: ...
    @property
    def session_id(self) -> int: ...

class InitCaptureStreamResponse:
    """Response of ``init_capture_stream``."""

    @property
    def ret_code(self) -> int: ...
    @property
    def ret_msg(self) -> str: ...
    @property
    def session_id(self) -> int: ...
    @property
    def data_topic_name(self) -> str: ...

class PlayerInfo:
    """Player session info."""

    @property
    def state(self) -> int: ...
    @property
    def played_bytes(self) -> int: ...
    @property
    def total_bytes(self) -> int: ...
    @property
    def volume(self) -> float: ...
    def state_enum(self) -> PlayerState | None: ...

class RecorderInfo:
    """Recorder session info."""

    @property
    def state(self) -> int: ...
    @property
    def captured_bytes(self) -> int: ...
    def state_enum(self) -> RecorderState | None: ...

class AudioCaptureStreamInfo:
    """Capture stream session info."""

    @property
    def state(self) -> int: ...
    @property
    def raw_enabled(self) -> bool: ...
    @property
    def naec_enabled(self) -> bool: ...
    @property
    def actual_raw_format(self) -> PcmFormat: ...
    @property
    def actual_naec_format(self) -> PcmFormat: ...
    @property
    def published_frames(self) -> int: ...
    @property
    def dropped_frames(self) -> int: ...
    def state_enum(self) -> AudioCaptureStreamState | None: ...

class AudioDeviceInfo:
    """Audio input or output device descriptor."""

    @property
    def device_id(self) -> str: ...
    @property
    def display_name(self) -> str: ...
    @property
    def direction(self) -> AudioDeviceDirection: ...
    @property
    def transport(self) -> AudioDeviceTransport: ...
    @property
    def backend_affinity(self) -> AudioDeviceBackendAffinity: ...
    @property
    def is_available(self) -> bool: ...
    @property
    def is_system_default(self) -> bool: ...
    @property
    def supports_input(self) -> bool: ...
    @property
    def supports_output(self) -> bool: ...
    @property
    def provider_name(self) -> str: ...
    @property
    def native_id(self) -> str: ...
    @property
    def metadata_json(self) -> str: ...

class BluetoothDeviceInfo:
    """Bluetooth device descriptor."""

    @property
    def address(self) -> str: ...
    @property
    def name(self) -> str: ...
    @property
    def rssi(self) -> int: ...
    @property
    def state(self) -> BluetoothDeviceState: ...
    @property
    def major_class(self) -> BluetoothMajorClass: ...
    @property
    def paired(self) -> bool: ...
    @property
    def trusted(self) -> bool: ...
    @property
    def connected(self) -> bool: ...
    @property
    def is_audio_sink(self) -> bool: ...
    @property
    def is_audio_source(self) -> bool: ...
    @property
    def is_hfp_capable(self) -> bool: ...
    @property
    def connected_profiles(self) -> list[BluetoothAudioProfile]: ...
    @property
    def linked_pulse_sink_id(self) -> str: ...
    @property
    def linked_pulse_source_id(self) -> str: ...
    @property
    def last_seen_ms(self) -> int: ...

class BluetoothConnectResult:
    """Result of ``connect_bluetooth_device``."""

    @property
    def device(self) -> BluetoothDeviceInfo: ...
    @property
    def pulse_sink_id(self) -> str: ...
    @property
    def pulse_source_id(self) -> str: ...
    @property
    def active_profile(self) -> BluetoothAudioProfile: ...
    @property
    def pulse_endpoint_ready(self) -> bool: ...
    @property
    def default_sink_applied(self) -> bool: ...
    @property
    def default_source_applied(self) -> bool: ...
    @property
    def default_route_error_code(self) -> int: ...
    @property
    def default_route_error_msg(self) -> str: ...

class AudioTopicHeader:
    """Common header of audio topic messages."""

    @property
    def client_id(self) -> str: ...
    @property
    def request_id(self) -> str: ...
    @property
    def session_type(self) -> int: ...
    @property
    def session_id(self) -> int: ...
    @property
    def timestamp_ms(self) -> int: ...
    def session_type_enum(self) -> SessionType | None: ...

class AudioProgressTopic:
    """Playback/recording progress (``rt/booster/audio/progress``)."""

    @property
    def header(self) -> AudioTopicHeader: ...
    @property
    def position_bytes(self) -> int: ...
    @property
    def duration_bytes(self) -> int: ...
    @property
    def state(self) -> int: ...

class AudioEventTopic:
    """Session lifecycle event (``rt/booster/audio/events``)."""

    @property
    def header(self) -> AudioTopicHeader: ...
    @property
    def event_code(self) -> int: ...
    @property
    def state(self) -> int: ...
    @property
    def detail(self) -> str: ...
    def event_code_enum(self) -> AudioEventCode | None: ...

class AudioErrorTopic:
    """Service error (``rt/booster/audio/errors``)."""

    @property
    def header(self) -> AudioTopicHeader: ...
    @property
    def ret_code(self) -> int: ...
    @property
    def error_category(self) -> int: ...
    @property
    def error_detail(self) -> int: ...
    @property
    def ret_msg(self) -> str: ...
    def error_category_enum(self) -> AudioErrorCategory | None: ...
    def error_detail_enum(self) -> AudioErrorDetail | None: ...
    def to_audio_error(self) -> AudioError: ...

class BluetoothEvent:
    """Bluetooth lifecycle event."""

    @property
    def timestamp_ms(self) -> int: ...
    @property
    def event_type(self) -> str: ...
    @property
    def address(self) -> str: ...
    @property
    def payload_json(self) -> str: ...
    @property
    def error_code(self) -> int: ...

class AudioCaptureFrame:
    """Captured frame with optional raw and NAEC PCM (C++ ``AudioCaptureFrame``)."""

    @property
    def frame_seq(self) -> int: ...
    @property
    def timestamp_ms(self) -> int: ...
    @property
    def raw_valid(self) -> bool: ...
    @property
    def raw_format(self) -> PcmFormat: ...
    @property
    def raw_frame_samples_per_channel(self) -> int: ...
    @property
    def raw_pcm(self) -> list[int]: ...
    @property
    def naec_valid(self) -> bool: ...
    @property
    def naec_format(self) -> PcmFormat: ...
    @property
    def naec_frame_samples_per_channel(self) -> int: ...
    @property
    def naec_pcm(self) -> list[int]: ...
    def raw_pcm_bytes(self) -> bytes:
        """Raw PCM as little-endian 16-bit bytes."""
        ...
    def naec_pcm_bytes(self) -> bytes:
        """NAEC PCM as little-endian 16-bit bytes."""
        ...

class AudioCaptureFrameTopic:
    """PCM frame sample from a capture stream's data topic."""

    @property
    def header(self) -> AudioTopicHeader: ...
    @property
    def frame_seq(self) -> int: ...
    @property
    def raw_valid(self) -> bool: ...
    @property
    def raw_sample_rate_hz(self) -> int: ...
    @property
    def raw_channels(self) -> int: ...
    @property
    def raw_bits_per_sample(self) -> int: ...
    @property
    def raw_frame_samples_per_channel(self) -> int: ...
    @property
    def raw_pcm(self) -> list[int]: ...
    @property
    def naec_valid(self) -> bool: ...
    @property
    def naec_sample_rate_hz(self) -> int: ...
    @property
    def naec_channels(self) -> int: ...
    @property
    def naec_bits_per_sample(self) -> int: ...
    @property
    def naec_frame_samples_per_channel(self) -> int: ...
    @property
    def naec_pcm(self) -> list[int]: ...
    def raw_pcm_bytes(self) -> bytes:
        """Raw PCM as little-endian 16-bit bytes."""
        ...
    def naec_pcm_bytes(self) -> bytes:
        """NAEC PCM as little-endian 16-bit bytes."""
        ...
    def to_frame(self) -> AudioCaptureFrame: ...

def capture_frame_topic_name(data_topic_name: str) -> str:
    """DDS topic name for a capture stream ``data_topic_name``."""
    ...

class AudioPlayer:
    """One playback session (C++ ``AudioPlayer``); create with
    :meth:`AudioClient.create_player`. Dropping it does not destroy the
    service-side session; call :meth:`destroy`."""

    @property
    def session_id(self) -> int | None:
        """Service session id, or ``None`` before ``init`` / after ``destroy``."""
        ...
    def start(self) -> None: ...
    def pause(self) -> None: ...
    def stop(self) -> None: ...
    def destroy(self) -> None: ...
    def set_preferred_device(self, device_id: str) -> None:
        """Select the endpoint by ``AudioDeviceInfo.device_id``."""
        ...
    def owns(self, header: AudioTopicHeader) -> bool:
        """Whether a topic sample belongs to this session."""
        ...
    def apply_event(self, event: AudioEventTopic) -> bool:
        """Update the cached state from an event; returns whether it matched."""
        ...
    def init(self, options: PlayerInitOptions) -> int:
        """Initialize the session; returns the new session id."""
        ...
    def reset(self) -> None: ...
    def release(self) -> None: ...
    def set_volume(self, volume: float) -> None: ...
    def get_info(self) -> PlayerInfo: ...
    def push_pcm_stream(self, data: bytes) -> None: ...
    def cached_info(self) -> PlayerInfo: ...
    def cached_state(self) -> PlayerState: ...
    def apply_progress(self, progress: AudioProgressTopic) -> bool: ...

class AudioRecorder:
    """One recording session (C++ ``AudioRecorder``)."""

    @property
    def session_id(self) -> int | None:
        """Service session id, or ``None`` before ``init`` / after ``destroy``."""
        ...
    def start(self) -> None: ...
    def pause(self) -> None: ...
    def stop(self) -> None: ...
    def destroy(self) -> None: ...
    def set_preferred_device(self, device_id: str) -> None:
        """Select the endpoint by ``AudioDeviceInfo.device_id``."""
        ...
    def owns(self, header: AudioTopicHeader) -> bool:
        """Whether a topic sample belongs to this session."""
        ...
    def apply_event(self, event: AudioEventTopic) -> bool:
        """Update the cached state from an event; returns whether it matched."""
        ...
    def init(self, options: RecorderInitOptions) -> int: ...
    def release(self) -> None: ...
    def get_info(self) -> RecorderInfo: ...
    def cached_info(self) -> RecorderInfo: ...
    def cached_state(self) -> RecorderState: ...
    def apply_progress(self, progress: AudioProgressTopic) -> bool: ...

class AudioLocalizer:
    """Direction-of-arrival localizer (C++ ``AudioLocalizer``)."""

    def get_doa_angle(self) -> int:
        """Current DOA angle in degrees."""
        ...

class AudioCaptureStream:
    """Raw/NAEC microphone PCM stream (C++ ``AudioCaptureStream``)."""

    @property
    def session_id(self) -> int | None:
        """Service session id, or ``None`` before ``init`` / after ``destroy``."""
        ...
    def start(self) -> None: ...
    def pause(self) -> None: ...
    def stop(self) -> None: ...
    def destroy(self) -> None: ...
    def set_preferred_device(self, device_id: str) -> None:
        """Select the endpoint by ``AudioDeviceInfo.device_id``."""
        ...
    def owns(self, header: AudioTopicHeader) -> bool:
        """Whether a topic sample belongs to this session."""
        ...
    def apply_event(self, event: AudioEventTopic) -> bool:
        """Update the cached state from an event; returns whether it matched."""
        ...
    @property
    def data_topic_name(self) -> str: ...
    def init(
        self, options: AudioCaptureStreamOptions | None = None
    ) -> InitCaptureStreamResponse: ...
    def get_info(self) -> AudioCaptureStreamInfo: ...
    def cached_info(self) -> AudioCaptureStreamInfo: ...
    def cached_state(self) -> AudioCaptureStreamState: ...
    def subscribe_frames(self) -> Subscription:
        """Yields :class:`AudioCaptureFrameTopic`. Subscribe to this stream's PCM frames (requires a prior ``init``)."""
        ...

class AudioClient:
    """Client for the audio service (C++ ``AudioManager``)."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a audio client (no ``robot_name``: audio channels are not namespaced).

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        ``network`` is ``"localhost"``, ``"all"`` or interface names / IP
        addresses (a list or a comma-separated string).
        """
        ...
    def init(self) -> str:
        """Register with the audio service and return the client id."""
        ...
    def client_id(self) -> str | None: ...
    def is_initialized(self) -> bool: ...
    def shutdown(self) -> None:
        """Drop the cached client id and RPC channels; re-registers lazily."""
        ...
    def generate_request_id(self) -> str: ...
    def init_player(self, options: PlayerInitOptions) -> InitPlayerResponse: ...
    def start_player(self, session_id: int) -> None: ...
    def pause_player(self, session_id: int) -> None: ...
    def stop_player(self, session_id: int) -> None: ...
    def reset_player(self, session_id: int) -> None: ...
    def destroy_player(self, session_id: int) -> None: ...
    def set_player_volume(self, session_id: int, volume: float) -> None: ...
    def get_player_info(self, session_id: int) -> PlayerInfo: ...
    def send_pcm_data(self, session_id: int, pcm_bytes: bytes) -> None: ...
    def init_recorder(self, options: RecorderInitOptions) -> InitRecorderResponse: ...
    def start_recorder(self, session_id: int) -> None: ...
    def pause_recorder(self, session_id: int) -> None: ...
    def stop_recorder(self, session_id: int) -> None: ...
    def destroy_recorder(self, session_id: int) -> None: ...
    def get_recorder_info(self, session_id: int) -> RecorderInfo: ...
    def get_doa_angle(self) -> int: ...
    def set_system_volume(self, volume: float) -> None: ...
    def get_system_volume(self) -> float: ...
    def set_system_mute(self, mute: bool) -> None: ...
    def get_system_mute(self) -> bool: ...
    def init_capture_stream(
        self, options: AudioCaptureStreamOptions | None = None
    ) -> InitCaptureStreamResponse: ...
    def start_capture_stream(self, session_id: int) -> None: ...
    def pause_capture_stream(self, session_id: int) -> None: ...
    def stop_capture_stream(self, session_id: int) -> None: ...
    def destroy_capture_stream(self, session_id: int) -> None: ...
    def get_capture_stream_info(self, session_id: int) -> AudioCaptureStreamInfo: ...
    def get_devices(
        self, query_type: AudioDeviceQueryType
    ) -> list[AudioDeviceInfo]: ...
    def set_preferred_device(self, session_id: int, device_id: str) -> None:
        """Select the endpoint (``AudioDeviceInfo.device_id``) of a session."""
        ...
    def start_bluetooth_scan(
        self, options: BluetoothScanOptions | None = None
    ) -> None: ...
    def stop_bluetooth_scan(self) -> None: ...
    def get_bluetooth_devices(
        self, include_unpaired: bool = True
    ) -> list[BluetoothDeviceInfo]: ...
    def connect_bluetooth_device(
        self, options: BluetoothConnectOptions
    ) -> BluetoothConnectResult: ...
    def disconnect_bluetooth_device(self, address: str) -> None: ...
    def forget_bluetooth_device(self, address: str) -> None: ...
    def create_player(self) -> AudioPlayer: ...
    def attach_player(self, session_id: int) -> AudioPlayer: ...
    def create_recorder(self) -> AudioRecorder: ...
    def attach_recorder(self, session_id: int) -> AudioRecorder: ...
    def create_localizer(self) -> AudioLocalizer: ...
    def create_capture_stream(self) -> AudioCaptureStream: ...
    def attach_capture_stream(
        self, session_id: int, data_topic_name: str
    ) -> AudioCaptureStream: ...
    def subscribe_bluetooth_events(self) -> Subscription:
        """Subscription yielding :class:`BluetoothEvent` samples."""
        ...
    def subscribe_progress(self) -> Subscription:
        """Subscription yielding :class:`AudioProgressTopic` samples."""
        ...
    def subscribe_events(self) -> Subscription:
        """Subscription yielding :class:`AudioEventTopic` samples."""
        ...
    def subscribe_errors(self) -> Subscription:
        """Subscription yielding :class:`AudioErrorTopic` samples."""
        ...
    def subscribe_capture_frames(self, data_topic_name: str) -> Subscription:
        """Subscription yielding :class:`AudioCaptureFrameTopic` samples."""
        ...

class AiClient:
    """Client for AI chat and speech features."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a AI client.

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        ``network`` is ``"localhost"``, ``"all"`` or interface names / IP
        addresses (a list or a comma-separated string).
        ``robot_name`` targets ``<topic>/<robot_name>`` (C++ ``Init(robot_name)``).
        """
        ...
    def send_api_request(self, api_id: int, param: str) -> None:
        """Send a raw request (``api_id`` may be an ``*ApiId`` value); the response body is discarded."""
        ...
    def send_api_request_with_response(self, api_id: int, param: str) -> str:
        """Send a raw request and return the response body."""
        ...
    def start_ai_chat(self, param: StartAiChatParameter) -> None: ...
    def stop_ai_chat(self) -> None: ...
    def speak(self, param: SpeakParameter) -> None: ...
    def start_face_tracking(self) -> None: ...
    def stop_face_tracking(self) -> None: ...
    def subscribe_subtitle(self) -> Subscription:
        """Yields :class:`Subtitle`. Subscribe to AI chat subtitles (``rt/ai_subtitle``)."""
        ...

class LuiSynthesizeSpeechRequest:
    """Request for :meth:`LuiClient.synthesize_speech`."""

    text: str
    voice_type: str
    speed: float
    playback: bool

    def __init__(
        self,
        text: str = "",
        voice_type: str = "default",
        speed: float = 1.0,
        playback: bool = False,
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> LuiSynthesizeSpeechRequest:
        """Parse from a JSON body."""
        ...

class LuiSynthesizeSpeechResponse:
    """Synthesized audio."""

    @property
    def audio_base64(self) -> str: ...
    @property
    def sample_rate_hz(self) -> int: ...
    @property
    def channels(self) -> int: ...
    @property
    def bits_per_sample(self) -> int: ...
    @property
    def format(self) -> str: ...

class LuiRecognizeAudioRequest:
    """Audio recognition request for the LUI recognize APIs."""

    input_type: str
    file_path: str
    audio_base64: str
    sample_rate_hz: int
    channels: int
    bits_per_sample: int
    format: str

    def __init__(
        self,
        input_type: str = "",
        file_path: str = "",
        audio_base64: str = "",
        sample_rate_hz: int = 16000,
        channels: int = 1,
        bits_per_sample: int = 16,
        format: str = "pcm_s16le",
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> LuiRecognizeAudioRequest:
        """Parse from a JSON body."""
        ...
    @staticmethod
    def from_file(file_path: str) -> LuiRecognizeAudioRequest:
        """Recognize a ``.wav`` / ``.mp3`` file on the robot."""
        ...
    @staticmethod
    def from_pcm_base64(
        audio_base64: str,
        sample_rate_hz: int = 16000,
        channels: int = 1,
        bits_per_sample: int = 16,
        format: str = "pcm_s16le",
    ) -> LuiRecognizeAudioRequest:
        """Recognize base64 raw PCM (the service expects 16 kHz mono ``pcm_s16le``)."""
        ...

class LuiRecognizeAudioResponse:
    """Recognition result."""

    @property
    def text(self) -> str: ...

class LuiClient:
    """Client for LUI ASR/TTS. Sessions are owned by the client instance."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a LUI client.

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        ``network`` is ``"localhost"``, ``"all"`` or interface names / IP
        addresses (a list or a comma-separated string).
        ``robot_name`` targets ``<topic>/<robot_name>`` (C++ ``Init(robot_name)``).
        """
        ...
    def send_api_request(self, api_id: int, param: str) -> None:
        """Send a raw request (``api_id`` may be an ``*ApiId`` value); the response body is discarded."""
        ...
    def send_api_request_with_response(self, api_id: int, param: str) -> str:
        """Send a raw request and return the response body."""
        ...
    @staticmethod
    def is_asr_session_active(session_id: str, client_id: str) -> bool:
        """Whether this process has an active ASR session ``session_id`` owned by ``client_id``."""
        ...
    def start_asr(self) -> None: ...
    def stop_asr(self) -> None: ...
    def start_tts(self, config: LuiTtsConfig) -> None: ...
    def stop_tts(self) -> None: ...
    def send_tts_text(self, param: LuiTtsParameter) -> None: ...
    @property
    def client_id(self) -> str: ...
    @property
    def current_asr_session_id(self) -> str | None: ...
    @property
    def current_audio_recognizer_session_id(self) -> str | None: ...
    @property
    def current_tts_session_id(self) -> str | None: ...
    def synthesize_speech(
        self, req: LuiSynthesizeSpeechRequest
    ) -> LuiSynthesizeSpeechResponse: ...
    def recognize_audio_once(
        self, req: LuiRecognizeAudioRequest
    ) -> LuiRecognizeAudioResponse: ...
    def start_audio_recognizer(self) -> None: ...
    def stop_audio_recognizer(self) -> None: ...
    def recognize_audio_in_session(
        self, req: LuiRecognizeAudioRequest
    ) -> LuiRecognizeAudioResponse: ...
    def subscribe_asr_chunk(self) -> Subscription:
        """Yields :class:`AsrChunk`. Subscribe to standalone ASR chunks (``rt/lui_asr_chunk``)."""
        ...

class LightApiId:
    """Light control RPC API ids."""

    SET_LED_LIGHT_COLOR: LightApiId
    STOP_LED_LIGHT_CONTROL: LightApiId
    SET_LED_LIGHT_COLORS: LightApiId

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> LightApiId:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class SetLEDLightColorParameter:
    """One LED color."""

    r: int
    g: int
    b: int

    def __init__(self, r: int = 0, g: int = 0, b: int = 0) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> SetLEDLightColorParameter:
        """Parse from a JSON body."""
        ...
    @staticmethod
    def from_hex(color: str) -> SetLEDLightColorParameter:
        """Parse ``"#RRGGBB"``; raises ``ValueError`` on malformed input."""
        ...
    @staticmethod
    def from_hex_or_default(color: str) -> SetLEDLightColorParameter:
        """Parse ``"#RRGGBB"``, falling back to black like the C++ SDK."""
        ...

ColorLike = SetLEDLightColorParameter | tuple[int, int, int] | str

class SetLEDLightColorsParameter:
    """Multi-pixel LED payload (SDK 1.7+)."""

    colors: list[SetLEDLightColorParameter]

    def __init__(self, colors: list[ColorLike] = []) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(json: str) -> SetLEDLightColorsParameter: ...

class LightControlClient:
    """Client for the LED light control service."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a light control client.

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        ``network`` is ``"localhost"``, ``"all"`` or interface names / IP
        addresses (a list or a comma-separated string).
        ``robot_name`` targets ``<topic>/<robot_name>`` (C++ ``Init(robot_name)``).
        """
        ...
    def send_api_request(self, api_id: int, param: str) -> None:
        """Send a raw request (``api_id`` may be an ``*ApiId`` value); the response body is discarded."""
        ...
    def send_api_request_with_response(self, api_id: int, param: str) -> str:
        """Send a raw request and return the response body."""
        ...
    def set_led_light_color(self, r: int, g: int, b: int) -> None: ...
    def set_led_light_color_param(self, color: ColorLike) -> None:
        """Set the color from a parameter, an ``(r, g, b)`` tuple or ``"#RRGGBB"``."""
        ...
    def set_led_light_colors(
        self, colors: SetLEDLightColorsParameter | list[ColorLike]
    ) -> None:
        """Set per-pixel colors."""
        ...
    def stop_led_light_control(self) -> None: ...

class CameraModel:
    """Camera hardware model."""

    UNKNOWN: CameraModel
    REALSENSE_D455: CameraModel
    ZED2I: CameraModel
    ORBBEC_GEMINI_335L: CameraModel
    BOOSTER_MIPI: CameraModel

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> CameraModel:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class CameraConnectionState:
    """Camera connection state."""

    UNKNOWN: CameraConnectionState
    CONNECTED: CameraConnectionState
    CONNECTION_LOST: CameraConnectionState

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> CameraConnectionState:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class CameraPos:
    """Camera mount position."""

    UNKNOWN: CameraPos
    HEAD: CameraPos
    LEFT_HAND: CameraPos
    RIGHT_HAND: CameraPos
    WAIST: CameraPos

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> CameraPos:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class CameraPixelFormat:
    """Color stream pixel format."""

    UNKNOWN: CameraPixelFormat
    RGB8: CameraPixelFormat
    BGR8: CameraPixelFormat
    YUV422: CameraPixelFormat
    JPEG: CameraPixelFormat
    NV12: CameraPixelFormat

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> CameraPixelFormat:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class DepthPixelFormat:
    """Depth stream pixel format."""

    UNKNOWN: DepthPixelFormat
    UINT16: DepthPixelFormat
    FLOAT32: DepthPixelFormat

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> DepthPixelFormat:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class ImuModel:
    """IMU hardware model."""

    UNKNOWN: ImuModel
    HI13R4N: ImuModel

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> ImuModel:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class HandModel:
    """Hand / gripper hardware model."""

    UNKNOWN: HandModel
    INSPIRE_HAND_RH56DFX: HandModel
    INSPIRE_GRIPPER_EG2_4X2: HandModel
    INSPIRE_TOUCH_HAND_RH56DFTP: HandModel

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> HandModel:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class HandPos:
    """Hand mount side."""

    UNKNOWN: HandPos
    LEFT: HandPos
    RIGHT: HandPos

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> HandPos:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class GeometryType:
    """Collision geometry type (``type`` in robot-model dicts)."""

    BOX: GeometryType
    CYLINDER: GeometryType
    SPHERE: GeometryType
    MESH: GeometryType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> GeometryType:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class JointType:
    """Joint type (``type`` in robot-model dicts)."""

    FIXED: JointType
    REVOLUTE: JointType
    CONTINUOUS: JointType
    PRISMATIC: JointType
    PLANAR: JointType
    FLOATING: JointType

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> JointType:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class Vector3:
    """3D vector."""

    @property
    def x(self) -> float: ...
    @property
    def y(self) -> float: ...
    @property
    def z(self) -> float: ...

class CameraExtrinsics:
    """Camera pose relative to a parent frame."""

    @property
    def parent_frame(self) -> str: ...
    @property
    def xyz(self) -> Vector3: ...
    @property
    def rpy(self) -> Vector3: ...

class VideoStreamEncoding:
    """Color stream encoding."""

    @property
    def width(self) -> int: ...
    @property
    def height(self) -> int: ...
    @property
    def format(self) -> CameraPixelFormat: ...
    @property
    def bitrate_bps(self) -> int: ...

class DepthStreamEncoding:
    """Depth stream encoding."""

    @property
    def width(self) -> int: ...
    @property
    def height(self) -> int: ...
    @property
    def format(self) -> DepthPixelFormat: ...

class Camera:
    """One camera from the camera catalog."""

    @property
    def name(self) -> str: ...
    @property
    def frame_id(self) -> str: ...
    @property
    def serial_number(self) -> str: ...
    @property
    def pos(self) -> CameraPos: ...
    @property
    def model(self) -> CameraModel: ...
    @property
    def state(self) -> CameraConnectionState: ...
    @property
    def topics(self) -> list[str]: ...
    @property
    def depth_scale(self) -> float: ...
    @property
    def extrinsics(self) -> CameraExtrinsics: ...
    @property
    def color_encoding(self) -> VideoStreamEncoding: ...
    @property
    def depth_encoding(self) -> DepthStreamEncoding: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(json: str) -> Camera: ...

class CameraList:
    """Camera catalog."""

    @property
    def cameras(self) -> list[Camera]: ...
    def to_json(self) -> str: ...
    def __len__(self) -> int: ...
    @staticmethod
    def from_json(json: str) -> CameraList:
        """Parse a bare array or ``{"cameras": [...]}``."""
        ...
    @staticmethod
    def from_device_info(info: DeviceInfo) -> CameraList: ...

class ImuInfo:
    """One IMU from the sensor catalog."""

    @property
    def index(self) -> int: ...
    @property
    def name(self) -> str: ...
    @property
    def mount_position(self) -> str: ...
    @property
    def model(self) -> ImuModel: ...

class ImuInfoList:
    """IMU catalog."""

    @property
    def imus(self) -> list[ImuInfo]: ...
    def to_json(self) -> str: ...
    def __len__(self) -> int: ...
    @staticmethod
    def from_device_info(info: DeviceInfo) -> ImuInfoList: ...

class HandInfo:
    """One hand / gripper from the hand catalog (C++ ``booster_interface::msg::Hand``;
    named ``HandInfo`` because ``Hand`` is the LEFT/RIGHT enum)."""

    @property
    def model(self) -> HandModel: ...
    @property
    def pos(self) -> HandPos: ...

class HandList:
    """Hand catalog."""

    @property
    def hands(self) -> list[HandInfo]: ...
    def to_json(self) -> str: ...
    def __len__(self) -> int: ...
    @staticmethod
    def from_device_info(info: DeviceInfo) -> HandList: ...

def camera_list_from_device_info(info: DeviceInfo) -> CameraList:
    """Parse a CAMERA ``DeviceInfo`` (empty for other kinds)."""
    ...

def imu_info_list_from_device_info(info: DeviceInfo) -> ImuInfoList:
    """Parse a SENSORS ``DeviceInfo`` (empty for other kinds)."""
    ...

def hand_list_from_device_info(info: DeviceInfo) -> HandList:
    """Parse a HANDS ``DeviceInfo`` (empty for other kinds)."""
    ...

def robot_model_from_device_info(info: DeviceInfo) -> dict[str, object] | None:
    """Parse a ROBOT_MODEL ``DeviceInfo`` into a dict (``root_link``, ``links``,
    ``joints``; enum fields as ints, see ``GeometryType`` / ``JointType``).
    Returns ``None`` for other kinds."""
    ...

def device_info_from_json(json: str) -> DeviceInfo:
    """Parse ``{"kind": int, "body": ...}`` or a bare body (kind UNKNOWN)."""
    ...

class CameraApiId:
    """Camera catalog RPC API ids."""

    GET_CAMERAS: CameraApiId

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> CameraApiId:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class CameraClient:
    """Client for the camera catalog service."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a camera client.

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        ``network`` is ``"localhost"``, ``"all"`` or interface names / IP
        addresses (a list or a comma-separated string).
        ``robot_name`` targets ``<topic>/<robot_name>`` (C++ ``Init(robot_name)``).
        """
        ...
    def send_api_request(self, api_id: int, param: str) -> None:
        """Send a raw request (``api_id`` may be an ``*ApiId`` value); the response body is discarded."""
        ...
    def send_api_request_with_response(self, api_id: int, param: str) -> str:
        """Send a raw request and return the response body."""
        ...
    def get_cameras(self) -> DeviceInfo:
        """Raw camera catalog as ``DeviceInfo`` (kind CAMERA)."""
        ...
    def get_camera_list(self) -> CameraList:
        """Typed camera catalog (C++ ``GetCameraList``)."""
        ...

class HandEyeCalibApiId:
    """Hand-eye calibration RPC API ids."""

    START_CALIBRATION: HandEyeCalibApiId
    STOP_CALIBRATION: HandEyeCalibApiId
    GET_STATUS: HandEyeCalibApiId
    GET_RESULT: HandEyeCalibApiId
    APPLY_RESULT: HandEyeCalibApiId

    @property
    def name(self) -> str: ...
    @staticmethod
    def from_int(value: int) -> HandEyeCalibApiId:
        """Convert a raw wire value; raises ``ValueError`` if unknown."""
        ...
    def __int__(self) -> int: ...
    def __index__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class StartHandEyeCalibParameter:
    """Parameters for :meth:`HandEyeCalibClient.start_calibration`."""

    publish_feedback: bool
    square_size_m: float | None

    def __init__(
        self, publish_feedback: bool = True, square_size_m: float | None = None
    ) -> None: ...
    def to_json(self) -> str:
        """Serialize to the JSON body sent to the service."""
        ...
    @staticmethod
    def from_json(json: str) -> StartHandEyeCalibParameter:
        """Parse from a JSON body."""
        ...

class HandEyeCalibStatus:
    """Calibration job status."""

    @property
    def status(self) -> str: ...
    @property
    def job_id(self) -> str: ...
    @property
    def stage(self) -> str: ...
    @property
    def started_at(self) -> str: ...
    @property
    def finished_at(self) -> str: ...
    @property
    def progress(self) -> float: ...
    @property
    def stage2_done(self) -> int: ...
    @property
    def stage2_total(self) -> int: ...
    @property
    def stage3_done(self) -> int: ...
    @property
    def stage3_total(self) -> int: ...
    @property
    def error(self) -> object | None: ...
    def __init__(
        self,
        status: str = "",
        job_id: str = "",
        stage: str = "",
        started_at: str = "",
        finished_at: str = "",
        progress: float = 0.0,
        stage2_done: int = 0,
        stage2_total: int = 0,
        stage3_done: int = 0,
        stage3_total: int = 0,
        error: object | None = None,
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(json: str) -> HandEyeCalibStatus: ...

class HandEyeCalibResult:
    """Calibration result."""

    @property
    def status(self) -> str: ...
    @property
    def job_id(self) -> str: ...
    @property
    def summary(self) -> str: ...
    @property
    def result(self) -> object | None: ...
    @property
    def reprojection_error_px(self) -> float: ...
    @property
    def progress(self) -> float: ...
    @property
    def stage2_done(self) -> int: ...
    @property
    def stage2_total(self) -> int: ...
    @property
    def stage3_done(self) -> int: ...
    @property
    def stage3_total(self) -> int: ...
    @property
    def error(self) -> object | None: ...
    def __init__(
        self,
        status: str = "",
        job_id: str = "",
        summary: str = "",
        result: object | None = None,
        reprojection_error_px: float = 0.0,
        progress: float = 0.0,
        stage2_done: int = 0,
        stage2_total: int = 0,
        stage3_done: int = 0,
        stage3_total: int = 0,
        error: object | None = None,
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(json: str) -> HandEyeCalibResult: ...

class HandEyeCalibApplyResult:
    """Result of applying a calibration."""

    @property
    def status(self) -> str: ...
    @property
    def applied_path(self) -> str: ...
    @property
    def error(self) -> object | None: ...
    def __init__(
        self, status: str = "", applied_path: str = "", error: object | None = None
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(json: str) -> HandEyeCalibApplyResult: ...

class HandEyeCalibClient:
    """Client for the hand-eye calibration service."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a hand-eye calibration client.

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        ``network`` is ``"localhost"``, ``"all"`` or interface names / IP
        addresses (a list or a comma-separated string).
        ``robot_name`` targets ``<topic>/<robot_name>`` (C++ ``Init(robot_name)``).
        """
        ...
    def send_api_request(self, api_id: int, param: str) -> None:
        """Send a raw request (``api_id`` may be an ``*ApiId`` value); the response body is discarded."""
        ...
    def send_api_request_with_response(self, api_id: int, param: str) -> str:
        """Send a raw request and return the response body."""
        ...
    def start_calibration(
        self, param: StartHandEyeCalibParameter | None = None
    ) -> None: ...
    def stop_calibration(self) -> None: ...
    def get_status(self) -> HandEyeCalibStatus: ...
    def get_result(self) -> HandEyeCalibResult: ...
    def apply_result(self) -> HandEyeCalibApplyResult: ...

class VisionClient:
    """Client for the vision detection service."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        robot_name: str | None = None,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a vision client.

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        ``network`` is ``"localhost"``, ``"all"`` or interface names / IP
        addresses (a list or a comma-separated string).
        ``robot_name`` targets ``<topic>/<robot_name>`` (C++ ``Init(robot_name)``).
        """
        ...
    def send_api_request(self, api_id: int, param: str) -> None:
        """Send a raw request (``api_id`` may be an ``*ApiId`` value); the response body is discarded."""
        ...
    def send_api_request_with_response(self, api_id: int, param: str) -> str:
        """Send a raw request and return the response body."""
        ...
    def start_vision_service(
        self,
        enable_position: bool = False,
        enable_color: bool = False,
        enable_face_detection: bool = False,
    ) -> None: ...
    def stop_vision_service(self) -> None: ...
    def get_detection_object(
        self, focus_ratio: float | None = None
    ) -> list[DetectResults]:
        """Detected objects; ``focus_ratio`` defaults to 0.33 (C++ default)."""
        ...
    def get_detection_object_with_ratio(
        self, focus_ratio: float
    ) -> list[DetectResults]: ...

class X5CameraClient:
    """Client for the X5 camera control service (no robot-name suffix, like C++)."""

    def __init__(
        self,
        startup_wait_sec: float | None = None,
        *,
        network: str | list[str] | None = None,
        domain_id: int | None = None,
    ) -> None:
        """Create a X5 camera client.

        Domain id and network default to ``BOOSTER_DOMAIN_ID`` /
        ``BOOSTER_DDS_NETWORK`` (domain 0, loopback only when unset).
        ``network`` is ``"localhost"``, ``"all"`` or interface names / IP
        addresses (a list or a comma-separated string).
        """
        ...
    def send_api_request(self, api_id: int, param: str) -> None:
        """Send a raw request (``api_id`` may be an ``*ApiId`` value); the response body is discarded."""
        ...
    def send_api_request_with_response(self, api_id: int, param: str) -> str:
        """Send a raw request and return the response body."""
        ...
    def change_mode(self, mode: CameraSetMode) -> None: ...
    def get_status(self) -> X5CameraGetStatusResponse: ...
