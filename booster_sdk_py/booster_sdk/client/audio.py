"""Audio service client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

AudioClient = bindings.AudioClient
BoosterSdkError = bindings.BoosterSdkError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
AUDIO_ERR_NOT_SUPPORTED = bindings.AUDIO_ERR_NOT_SUPPORTED
AUDIO_PROGRESS_TOPIC = bindings.AUDIO_PROGRESS_TOPIC
AUDIO_EVENT_TOPIC = bindings.AUDIO_EVENT_TOPIC
AUDIO_ERROR_TOPIC = bindings.AUDIO_ERROR_TOPIC
capture_frame_topic_name = bindings.capture_frame_topic_name
AudioPlayer = bindings.AudioPlayer
AudioRecorder = bindings.AudioRecorder
AudioLocalizer = bindings.AudioLocalizer
AudioCaptureStream = bindings.AudioCaptureStream
AudioSourceType = bindings.AudioSourceType
PlayerPriority = bindings.PlayerPriority
PlayerState = bindings.PlayerState
RecorderState = bindings.RecorderState
AudioCaptureStreamState = bindings.AudioCaptureStreamState
AudioDeviceQueryType = bindings.AudioDeviceQueryType
AudioDeviceDirection = bindings.AudioDeviceDirection
AudioDeviceTransport = bindings.AudioDeviceTransport
AudioDeviceBackendAffinity = bindings.AudioDeviceBackendAffinity
BluetoothScanState = bindings.BluetoothScanState
BluetoothDeviceState = bindings.BluetoothDeviceState
BluetoothMajorClass = bindings.BluetoothMajorClass
BluetoothAudioProfile = bindings.BluetoothAudioProfile
SessionType = bindings.SessionType
AudioErrorCode = bindings.AudioErrorCode
AudioEventCode = bindings.AudioEventCode
AudioErrorCategory = bindings.AudioErrorCategory
AudioErrorDetail = bindings.AudioErrorDetail
PcmFormat = bindings.PcmFormat
PlayerInitOptions = bindings.PlayerInitOptions
RecorderInitOptions = bindings.RecorderInitOptions
AudioCaptureStreamOptions = bindings.AudioCaptureStreamOptions
BluetoothScanOptions = bindings.BluetoothScanOptions
BluetoothConnectOptions = bindings.BluetoothConnectOptions
AudioError = bindings.AudioError
InitPlayerResponse = bindings.InitPlayerResponse
InitRecorderResponse = bindings.InitRecorderResponse
InitCaptureStreamResponse = bindings.InitCaptureStreamResponse
PlayerInfo = bindings.PlayerInfo
RecorderInfo = bindings.RecorderInfo
AudioCaptureStreamInfo = bindings.AudioCaptureStreamInfo
AudioDeviceInfo = bindings.AudioDeviceInfo
BluetoothDeviceInfo = bindings.BluetoothDeviceInfo
BluetoothConnectResult = bindings.BluetoothConnectResult
AudioTopicHeader = bindings.AudioTopicHeader
AudioProgressTopic = bindings.AudioProgressTopic
AudioEventTopic = bindings.AudioEventTopic
AudioErrorTopic = bindings.AudioErrorTopic
BluetoothEvent = bindings.BluetoothEvent
AudioCaptureFrame = bindings.AudioCaptureFrame
AudioCaptureFrameTopic = bindings.AudioCaptureFrameTopic
Subscription = bindings.Subscription

__all__ = [
    "AudioClient",
    "BoosterSdkError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "AUDIO_ERR_NOT_SUPPORTED",
    "AUDIO_PROGRESS_TOPIC",
    "AUDIO_EVENT_TOPIC",
    "AUDIO_ERROR_TOPIC",
    "capture_frame_topic_name",
    "AudioPlayer",
    "AudioRecorder",
    "AudioLocalizer",
    "AudioCaptureStream",
    "AudioSourceType",
    "PlayerPriority",
    "PlayerState",
    "RecorderState",
    "AudioCaptureStreamState",
    "AudioDeviceQueryType",
    "AudioDeviceDirection",
    "AudioDeviceTransport",
    "AudioDeviceBackendAffinity",
    "BluetoothScanState",
    "BluetoothDeviceState",
    "BluetoothMajorClass",
    "BluetoothAudioProfile",
    "SessionType",
    "AudioErrorCode",
    "AudioEventCode",
    "AudioErrorCategory",
    "AudioErrorDetail",
    "PcmFormat",
    "PlayerInitOptions",
    "RecorderInitOptions",
    "AudioCaptureStreamOptions",
    "BluetoothScanOptions",
    "BluetoothConnectOptions",
    "AudioError",
    "InitPlayerResponse",
    "InitRecorderResponse",
    "InitCaptureStreamResponse",
    "PlayerInfo",
    "RecorderInfo",
    "AudioCaptureStreamInfo",
    "AudioDeviceInfo",
    "BluetoothDeviceInfo",
    "BluetoothConnectResult",
    "AudioTopicHeader",
    "AudioProgressTopic",
    "AudioEventTopic",
    "AudioErrorTopic",
    "BluetoothEvent",
    "AudioCaptureFrame",
    "AudioCaptureFrameTopic",
    "Subscription",
]
