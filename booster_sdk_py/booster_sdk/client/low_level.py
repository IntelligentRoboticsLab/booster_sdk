"""Low-level joint control (``rt/joint_ctrl`` / ``rt/low_state``) and kick commands.

Create publishers with :meth:`BoosterClient.low_cmd_publisher` /
:meth:`BoosterClient.kick_publisher` and subscribe with
:meth:`BoosterClient.subscribe_low_state`. Always pair a joint index enum with
the matching joint count (T1: ``JointIndex`` / ``JOINT_CNT``; K1:
``JointIndexK1`` / ``JOINT_CNT_K1``; T1 7-DoF arms: ``JointIndexWith7DofArm`` /
``JOINT_CNT_7DOF_ARM``)."""

from __future__ import annotations

import booster_sdk_bindings as bindings

JOINT_CNT = bindings.JOINT_CNT
JOINT_CNT_K1 = bindings.JOINT_CNT_K1
JOINT_CNT_7DOF_ARM = bindings.JOINT_CNT_7DOF_ARM
JointIndex = bindings.JointIndex
JointIndexK1 = bindings.JointIndexK1
JointIndexWith7DofArm = bindings.JointIndexWith7DofArm
CmdType = bindings.CmdType
MotorCmd = bindings.MotorCmd
LowCmd = bindings.LowCmd
LowCmdPublisher = bindings.LowCmdPublisher
MotorState = bindings.MotorState
ImuState = bindings.ImuState
LowState = bindings.LowState
Time = bindings.Time
Header = bindings.Header
Kick = bindings.Kick
KickPublisher = bindings.KickPublisher

__all__ = [
    "JOINT_CNT",
    "JOINT_CNT_K1",
    "JOINT_CNT_7DOF_ARM",
    "JointIndex",
    "JointIndexK1",
    "JointIndexWith7DofArm",
    "CmdType",
    "MotorCmd",
    "LowCmd",
    "LowCmdPublisher",
    "MotorState",
    "ImuState",
    "LowState",
    "Time",
    "Header",
    "Kick",
    "KickPublisher",
]
