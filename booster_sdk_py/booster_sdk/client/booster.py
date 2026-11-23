"""Booster locomotion and control client bindings.

Topic message classes whose name clashes with an enum carry a ``Msg``
suffix (``TrainedTrajStatusMsg``, ``RobocupBehaviorStatusMsg``); ROS geometry
types are ``RosQuaternion`` / ``RosTransform`` (``Quaternion`` / ``Transform``
are the RPC types)."""

from __future__ import annotations

import booster_sdk_bindings as bindings

BoosterClient = bindings.BoosterClient
BoosterSdkError = bindings.BoosterSdkError
Odometer = bindings.Odometer
Subscription = bindings.Subscription
RobotMode = bindings.RobotMode
Hand = bindings.Hand
GripperMode = bindings.GripperMode
GripperCommand = bindings.GripperCommand
HandAction = bindings.HandAction
Frame = bindings.Frame
GripperControlMode = bindings.GripperControlMode
BoosterHandType = bindings.BoosterHandType
DanceId = bindings.DanceId
WholeBodyDanceId = bindings.WholeBodyDanceId
VisualKickVersion = bindings.VisualKickVersion
GaitType = bindings.GaitType
GetUpVersion = bindings.GetUpVersion
DeviceInfoKind = bindings.DeviceInfoKind
DeviceInfo = bindings.DeviceInfo
JointOrder = bindings.JointOrder
BodyControl = bindings.BodyControl
Action = bindings.Action
Position = bindings.Position
Orientation = bindings.Orientation
Posture = bindings.Posture
Quaternion = bindings.Quaternion
Transform = bindings.Transform
GripperMotionParameter = bindings.GripperMotionParameter
DexterousFingerParameter = bindings.DexterousFingerParameter
CustomModelParams = bindings.CustomModelParams
CustomModel = bindings.CustomModel
CustomTrainedTraj = bindings.CustomTrainedTraj
GetModeResponse = bindings.GetModeResponse
GetStatusResponse = bindings.GetStatusResponse
GetRobotInfoResponse = bindings.GetRobotInfoResponse
LoadCustomTrainedTrajResponse = bindings.LoadCustomTrainedTrajResponse
LocoApiId = bindings.LocoApiId
GetTrainedTrajStatusResponse = bindings.GetTrainedTrajStatusResponse
ApiOperation = bindings.ApiOperation
ApiOperationProgress = bindings.ApiOperationProgress
ApiOperationResult = bindings.ApiOperationResult
ApiOperationResultCode = bindings.ApiOperationResultCode
ApiOperationEventType = bindings.ApiOperationEventType
RemoteControllerEvent = bindings.RemoteControllerEvent
ButtonEventType = bindings.ButtonEventType
FallDownStateType = bindings.FallDownStateType
ProneBodyControlPosture = bindings.ProneBodyControlPosture
RobocupBehaviorStatus = bindings.RobocupBehaviorStatus
TrainedTrajStatus = bindings.TrainedTrajStatus
RobotDdsJointStatus = bindings.RobotDdsJointStatus
RobotDdsImuStatus = bindings.RobotDdsImuStatus
RobotDdsBatteryStatus = bindings.RobotDdsBatteryStatus
RobotStatusDdsMsg = bindings.RobotStatusDdsMsg
MotionState = bindings.MotionState
BatteryState = bindings.BatteryState
ButtonEventMsg = bindings.ButtonEventMsg
RemoteControllerState = bindings.RemoteControllerState
RobotProcessStatus = bindings.RobotProcessStatus
RobotProcessStateMsg = bindings.RobotProcessStateMsg
BinaryData = bindings.BinaryData
FallDownState = bindings.FallDownState
RobotStatesMsg = bindings.RobotStatesMsg
ProneBodyControlStatus = bindings.ProneBodyControlStatus
RobotReplayTrajID = bindings.RobotReplayTrajID
TrainedTrajStatusMsg = bindings.TrainedTrajStatusMsg
RobocupBehaviorStatusMsg = bindings.RobocupBehaviorStatusMsg
HandReplyParam = bindings.HandReplyParam
HandReplyData = bindings.HandReplyData
HandTouchParam = bindings.HandTouchParam
HandTouchData = bindings.HandTouchData
TFMessage = bindings.TFMessage
TransformStamped = bindings.TransformStamped
RosTransform = bindings.RosTransform
RosQuaternion = bindings.RosQuaternion
Header = bindings.Header
Time = bindings.Time
LowState = bindings.LowState
MotorState = bindings.MotorState
ImuState = bindings.ImuState
LowCmd = bindings.LowCmd
MotorCmd = bindings.MotorCmd
CmdType = bindings.CmdType
LowCmdPublisher = bindings.LowCmdPublisher
Kick = bindings.Kick
KickPublisher = bindings.KickPublisher

__all__ = [
    "BoosterClient",
    "BoosterSdkError",
    "Odometer",
    "Subscription",
    "RobotMode",
    "Hand",
    "GripperMode",
    "GripperCommand",
    "HandAction",
    "Frame",
    "GripperControlMode",
    "BoosterHandType",
    "DanceId",
    "WholeBodyDanceId",
    "VisualKickVersion",
    "GaitType",
    "GetUpVersion",
    "DeviceInfoKind",
    "DeviceInfo",
    "JointOrder",
    "BodyControl",
    "Action",
    "Position",
    "Orientation",
    "Posture",
    "Quaternion",
    "Transform",
    "GripperMotionParameter",
    "DexterousFingerParameter",
    "CustomModelParams",
    "CustomModel",
    "CustomTrainedTraj",
    "GetModeResponse",
    "GetStatusResponse",
    "GetRobotInfoResponse",
    "LoadCustomTrainedTrajResponse",
    "LocoApiId",
    "GetTrainedTrajStatusResponse",
    "ApiOperation",
    "ApiOperationProgress",
    "ApiOperationResult",
    "ApiOperationResultCode",
    "ApiOperationEventType",
    "RemoteControllerEvent",
    "ButtonEventType",
    "FallDownStateType",
    "ProneBodyControlPosture",
    "RobocupBehaviorStatus",
    "TrainedTrajStatus",
    "RobotDdsJointStatus",
    "RobotDdsImuStatus",
    "RobotDdsBatteryStatus",
    "RobotStatusDdsMsg",
    "MotionState",
    "BatteryState",
    "ButtonEventMsg",
    "RemoteControllerState",
    "RobotProcessStatus",
    "RobotProcessStateMsg",
    "BinaryData",
    "FallDownState",
    "RobotStatesMsg",
    "ProneBodyControlStatus",
    "RobotReplayTrajID",
    "TrainedTrajStatusMsg",
    "RobocupBehaviorStatusMsg",
    "HandReplyParam",
    "HandReplyData",
    "HandTouchParam",
    "HandTouchData",
    "TFMessage",
    "TransformStamped",
    "RosTransform",
    "RosQuaternion",
    "Header",
    "Time",
    "LowState",
    "MotorState",
    "ImuState",
    "LowCmd",
    "MotorCmd",
    "CmdType",
    "LowCmdPublisher",
    "Kick",
    "KickPublisher",
]
