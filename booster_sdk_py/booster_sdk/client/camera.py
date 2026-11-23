"""Camera catalog client bindings and typed device catalogs."""

from __future__ import annotations

import booster_sdk_bindings as bindings

CameraClient = bindings.CameraClient
BoosterSdkError = bindings.BoosterSdkError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
CameraApiId = bindings.CameraApiId
DeviceInfo = bindings.DeviceInfo
DeviceInfoKind = bindings.DeviceInfoKind
CameraModel = bindings.CameraModel
CameraConnectionState = bindings.CameraConnectionState
CameraPos = bindings.CameraPos
CameraPixelFormat = bindings.CameraPixelFormat
DepthPixelFormat = bindings.DepthPixelFormat
ImuModel = bindings.ImuModel
HandModel = bindings.HandModel
HandPos = bindings.HandPos
GeometryType = bindings.GeometryType
JointType = bindings.JointType
Vector3 = bindings.Vector3
CameraExtrinsics = bindings.CameraExtrinsics
VideoStreamEncoding = bindings.VideoStreamEncoding
DepthStreamEncoding = bindings.DepthStreamEncoding
Camera = bindings.Camera
CameraList = bindings.CameraList
ImuInfo = bindings.ImuInfo
ImuInfoList = bindings.ImuInfoList
HandInfo = bindings.HandInfo
HandList = bindings.HandList
camera_list_from_device_info = bindings.camera_list_from_device_info
imu_info_list_from_device_info = bindings.imu_info_list_from_device_info
hand_list_from_device_info = bindings.hand_list_from_device_info
robot_model_from_device_info = bindings.robot_model_from_device_info
device_info_from_json = bindings.device_info_from_json

__all__ = [
    "CameraClient",
    "BoosterSdkError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "CameraApiId",
    "DeviceInfo",
    "DeviceInfoKind",
    "CameraModel",
    "CameraConnectionState",
    "CameraPos",
    "CameraPixelFormat",
    "DepthPixelFormat",
    "ImuModel",
    "HandModel",
    "HandPos",
    "GeometryType",
    "JointType",
    "Vector3",
    "CameraExtrinsics",
    "VideoStreamEncoding",
    "DepthStreamEncoding",
    "Camera",
    "CameraList",
    "ImuInfo",
    "ImuInfoList",
    "HandInfo",
    "HandList",
    "camera_list_from_device_info",
    "imu_info_list_from_device_info",
    "hand_list_from_device_info",
    "robot_model_from_device_info",
    "device_info_from_json",
]
