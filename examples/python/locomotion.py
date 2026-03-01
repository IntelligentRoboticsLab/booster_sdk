"""High-level locomotion control example.

Run with: python examples/python/locomotion.py
"""

import logging
import time

from booster_sdk.client.booster import BoosterClient, RobotMode

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def main() -> None:
    logger.info("Starting locomotion control example")

    client = BoosterClient()

    logger.info("Changing to walking mode...")
    client.change_mode(RobotMode.WALKING)
    logger.info("Mode changed successfully")

    time.sleep(2)

    logger.info("Moving forward at 0.5 m/s for 3 seconds")
    client.move_robot(0.5, 0.0, 0.0)
    time.sleep(3)

    logger.info("Stopping")
    client.move_robot(0.0, 0.0, 0.0)
    time.sleep(1)

    logger.info("Turning left for 2 seconds")
    client.move_robot(0.0, 0.0, 0.6)
    time.sleep(2)

    logger.info("Stopping")
    client.move_robot(0.0, 0.0, 0.0)
    time.sleep(1)

    logger.info("Example completed successfully")


if __name__ == "__main__":
    main()
