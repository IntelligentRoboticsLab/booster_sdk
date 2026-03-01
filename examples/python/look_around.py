"""Head look-around example.

Run with: python examples/python/look_around.py
"""

import logging
import time

from booster_sdk.client.booster import BoosterClient

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def main() -> None:
    logger.info("Starting look-around example")

    client = BoosterClient()

    # Sweep head left/right a few times, then return to center.
    positions = [
        (0.0, -0.7),
        (0.0, 0.7),
        (0.2, -0.4),
        (-0.2, 0.4),
        (0.0, 0.0),
    ]

    for pitch, yaw in positions:
        logger.info("Rotating head to pitch=%.2f yaw=%.2f", pitch, yaw)
        client.rotate_head(pitch, yaw)
        time.sleep(1.5)

    logger.info("Look-around example completed successfully")


if __name__ == "__main__":
    main()
