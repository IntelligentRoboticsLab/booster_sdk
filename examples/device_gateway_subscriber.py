"""Print joint temperatures from the device gateway (``rt/device_gateway``).

Set ``BOOSTER_DDS_NETWORK`` / ``BOOSTER_DOMAIN_ID`` (or pass ``network=`` /
``domain_id=`` to ``BoosterClient``) to reach the robot.
"""

from booster_sdk.client.booster import BoosterClient


def main() -> None:
    client = BoosterClient()
    print("Listening for rt/device_gateway... (Ctrl-C to stop)")
    with client.subscribe_device_gateway() as subscription:
        for msg in subscription:
            for joint in msg.joint_vec:
                print(f"{joint.name}: {joint.temperature}C")


if __name__ == "__main__":
    main()
