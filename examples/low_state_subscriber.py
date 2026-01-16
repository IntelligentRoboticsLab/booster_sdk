"""Subscribe to low-level state over Zenoh and decode FastDDS CDR frames.

Requirements:
- `booster_sdk` Python package installed (uses the LowState bindings).
- `zenoh` Python package installed.
- `zenoh-bridge-dds` running so DDS topic `rt/low_state` is bridged into Zenoh.

Domain defaults to 0; override with BOOSTER_DOMAIN_ID env var.
"""

from __future__ import annotations

import os
import sys
import time

import zenoh

from booster_sdk import LowState


def resolve_domain_id() -> int:
    try:
        return int(os.environ.get("BOOSTER_DOMAIN_ID", "0"))
    except ValueError:
        return 0


def format_key_expr(domain_id: int) -> str:
    return f"domain{domain_id}/rt/low_state"


def log_low_state(state: LowState) -> None:
    print("Received low_state sample")
    print(f"  serial motor count: {len(state.motor_state_serial)}")
    print(f"  parallel motor count: {len(state.motor_state_parallel)}")

    imu = state.imu_state
    print(
        "  imu: rpy=({:.3}, {:.3}, {:.3}), gyro=({:.3}, {:.3}, {:.3}), acc=({:.3}, {:.3}, {:.3})".format(
            imu.rpy[0],
            imu.rpy[1],
            imu.rpy[2],
            imu.gyro[0],
            imu.gyro[1],
            imu.gyro[2],
            imu.acc[0],
            imu.acc[1],
            imu.acc[2],
        )
    )

    for i, motor in enumerate(state.motor_state_serial):
        print(
            f"  serial motor {i}: dq={motor.dq:.4f}, ddq={motor.ddq:.4f}, tau_est={motor.tau_est:.4f}, temp={motor.temperature}C, lost={motor.lost}"
        )
    for i, motor in enumerate(state.motor_state_parallel):
        print(
            f"  parallel motor {i}: dq={motor.dq:.4f}, ddq={motor.ddq:.4f}, tau_est={motor.tau_est:.4f}, temp={motor.temperature}C, lost={motor.lost}"
        )


def main() -> int:
    domain_id = resolve_domain_id()
    key_expr = format_key_expr(domain_id)
    print(f"Subscribing to '{key_expr}' (domain {domain_id})")
    print("Ensure zenoh-bridge-dds is running to bridge DDS topics")

    session = zenoh.open({})
    subscriber = session.declare_subscriber(key_expr).res()

    try:
        while True:
            sample = subscriber.recv()
            payload = sample.payload.to_bytes()
            try:
                state = LowState.from_cdr(payload)
                log_low_state(state)
            except Exception as exc:  # noqa: BLE001
                print(f"Failed to decode LowState: {exc}")
            time.sleep(0.01)
    except KeyboardInterrupt:
        print("Exiting...")
    finally:
        subscriber.undeclare()
        session.close()

    return 0


if __name__ == "__main__":
    sys.exit(main())
