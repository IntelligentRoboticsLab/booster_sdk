"""Subscribe to device gateway state over DDS with CycloneDDS.

Requirements:
- `pip install cyclonedds`
"""

from dataclasses import dataclass
from typing import List

from cyclonedds.domain import DomainParticipant
from cyclonedds.sub import DataReader
from cyclonedds.topic import Topic


@dataclass
class RobotDdsJointStatus:
    name: str
    index: int
    is_connected: bool
    temperature: int
    is_limited: bool
    status_code: int
    temperature_level: int


@dataclass
class RobotDdsImuStatus:
    name: str
    index: int
    is_connected: bool
    status_code: int


@dataclass
class RobotDdsBatteryStatus:
    name: str
    index: int
    soc: float
    status_code: int
    soc_level: int


@dataclass
class RobotStatusDdsMsg:
    joint_vec: List[RobotDdsJointStatus]
    imu_vec: List[RobotDdsImuStatus]
    battery_vec: List[RobotDdsBatteryStatus]


def main() -> None:
    participant = DomainParticipant(0)
    topic = Topic(participant, "rt/device_gateway", RobotStatusDdsMsg)
    reader = DataReader(participant, topic)

    print("Listening for rt/device_gateway...")
    while True:
        samples = reader.take(10)
        for sample in samples:
            if not sample.data:
                continue
            msg = sample.data
            for joint in msg.joint_vec:
                print(f"{joint.name}: {joint.temperature}C")


if __name__ == "__main__":
    main()
