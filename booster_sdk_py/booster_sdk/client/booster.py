"""Booster locomotion and control client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

BoosterClient = bindings.BoosterClient
BoosterSdkError = bindings.BoosterSdkError
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

__all__ = [
    "BoosterClient",
    "BoosterSdkError",
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
]
