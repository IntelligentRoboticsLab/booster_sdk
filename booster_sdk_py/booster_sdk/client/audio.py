"""Audio service client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

AudioClient = bindings.AudioClient
AudioSourceType = bindings.AudioSourceType
PlayerPriority = bindings.PlayerPriority
PlayerState = bindings.PlayerState
RecorderState = bindings.RecorderState
AudioCaptureStreamState = bindings.AudioCaptureStreamState
PcmFormat = bindings.PcmFormat
PlayerInitOptions = bindings.PlayerInitOptions
RecorderInitOptions = bindings.RecorderInitOptions
AudioCaptureStreamOptions = bindings.AudioCaptureStreamOptions
InitPlayerResponse = bindings.InitPlayerResponse
InitRecorderResponse = bindings.InitRecorderResponse
InitCaptureStreamResponse = bindings.InitCaptureStreamResponse
PlayerInfo = bindings.PlayerInfo
RecorderInfo = bindings.RecorderInfo
AudioCaptureStreamInfo = bindings.AudioCaptureStreamInfo
AudioDeviceQueryType = bindings.AudioDeviceQueryType
AudioDeviceDirection = bindings.AudioDeviceDirection
AudioDeviceTransport = bindings.AudioDeviceTransport
AudioDeviceBackendAffinity = bindings.AudioDeviceBackendAffinity
AudioDeviceInfo = bindings.AudioDeviceInfo
BluetoothDeviceState = bindings.BluetoothDeviceState
BluetoothMajorClass = bindings.BluetoothMajorClass
BluetoothAudioProfile = bindings.BluetoothAudioProfile
BluetoothDeviceInfo = bindings.BluetoothDeviceInfo
BluetoothScanOptions = bindings.BluetoothScanOptions
BluetoothConnectOptions = bindings.BluetoothConnectOptions
BluetoothConnectResult = bindings.BluetoothConnectResult

__all__ = [
    "AudioClient",
    "AudioSourceType",
    "PlayerPriority",
    "PlayerState",
    "RecorderState",
    "AudioCaptureStreamState",
    "PcmFormat",
    "PlayerInitOptions",
    "RecorderInitOptions",
    "AudioCaptureStreamOptions",
    "InitPlayerResponse",
    "InitRecorderResponse",
    "InitCaptureStreamResponse",
    "PlayerInfo",
    "RecorderInfo",
    "AudioCaptureStreamInfo",
    "AudioDeviceQueryType",
    "AudioDeviceDirection",
    "AudioDeviceTransport",
    "AudioDeviceBackendAffinity",
    "AudioDeviceInfo",
    "BluetoothDeviceState",
    "BluetoothMajorClass",
    "BluetoothAudioProfile",
    "BluetoothDeviceInfo",
    "BluetoothScanOptions",
    "BluetoothConnectOptions",
    "BluetoothConnectResult",
]
