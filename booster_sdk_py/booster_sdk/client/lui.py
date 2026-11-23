"""LUI speech client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

LuiClient = bindings.LuiClient
BoosterSdkError = bindings.BoosterSdkError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
LuiApiId = bindings.LuiApiId
TOPIC_LUI_ASR_CHUNK = bindings.TOPIC_LUI_ASR_CHUNK
LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS = bindings.LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS
LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS = bindings.LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS
LUI_MAX_RECOGNIZE_AUDIO_BYTES = bindings.LUI_MAX_RECOGNIZE_AUDIO_BYTES
LuiTtsConfig = bindings.LuiTtsConfig
LuiTtsParameter = bindings.LuiTtsParameter
LuiSynthesizeSpeechRequest = bindings.LuiSynthesizeSpeechRequest
LuiSynthesizeSpeechResponse = bindings.LuiSynthesizeSpeechResponse
LuiRecognizeAudioRequest = bindings.LuiRecognizeAudioRequest
LuiRecognizeAudioResponse = bindings.LuiRecognizeAudioResponse
AsrChunk = bindings.AsrChunk
AsrUtterance = bindings.AsrUtterance
Subscription = bindings.Subscription

__all__ = [
    "LuiClient",
    "BoosterSdkError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "LuiApiId",
    "TOPIC_LUI_ASR_CHUNK",
    "LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS",
    "LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS",
    "LUI_MAX_RECOGNIZE_AUDIO_BYTES",
    "LuiTtsConfig",
    "LuiTtsParameter",
    "LuiSynthesizeSpeechRequest",
    "LuiSynthesizeSpeechResponse",
    "LuiRecognizeAudioRequest",
    "LuiRecognizeAudioResponse",
    "AsrChunk",
    "AsrUtterance",
    "Subscription",
]
