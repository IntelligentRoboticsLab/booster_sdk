"""Light control client bindings."""

from __future__ import annotations

import booster_sdk_bindings as bindings

LightControlClient = bindings.LightControlClient
BoosterSdkError = bindings.BoosterSdkError

__all__ = ["LightControlClient", "BoosterSdkError"]
