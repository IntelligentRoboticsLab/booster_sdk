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
]
