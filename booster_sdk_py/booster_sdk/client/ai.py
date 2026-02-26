"""AI client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

AiClient = bindings.AiClient
BoosterSdkError = bindings.BoosterSdkError
BOOSTER_ROBOT_USER_ID = bindings.BOOSTER_ROBOT_USER_ID
TtsConfig = bindings.TtsConfig
LlmConfig = bindings.LlmConfig
AsrConfig = bindings.AsrConfig
StartAiChatParameter = bindings.StartAiChatParameter
SpeakParameter = bindings.SpeakParameter

__all__ = [
    "AiClient",
    "BoosterSdkError",
    "BOOSTER_ROBOT_USER_ID",
    "TtsConfig",
    "LlmConfig",
    "AsrConfig",
    "StartAiChatParameter",
    "SpeakParameter",
]
