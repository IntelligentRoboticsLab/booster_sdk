"""AI chat client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

AiClient = bindings.AiClient
BoosterSdkError = bindings.BoosterSdkError
RpcError = bindings.RpcError
RpcTimeoutError = bindings.RpcTimeoutError
RpcStatusError = bindings.RpcStatusError
BOOSTER_ROBOT_USER_ID = bindings.BOOSTER_ROBOT_USER_ID
TOPIC_AI_SUBTITLE = bindings.TOPIC_AI_SUBTITLE
AiApiId = bindings.AiApiId
TtsConfig = bindings.TtsConfig
LlmConfig = bindings.LlmConfig
AsrConfig = bindings.AsrConfig
StartAiChatParameter = bindings.StartAiChatParameter
SpeakParameter = bindings.SpeakParameter
Subtitle = bindings.Subtitle
Subscription = bindings.Subscription

__all__ = [
    "AiClient",
    "BoosterSdkError",
    "RpcError",
    "RpcTimeoutError",
    "RpcStatusError",
    "BOOSTER_ROBOT_USER_ID",
    "TOPIC_AI_SUBTITLE",
    "AiApiId",
    "TtsConfig",
    "LlmConfig",
    "AsrConfig",
    "StartAiChatParameter",
    "SpeakParameter",
    "Subtitle",
    "Subscription",
]
