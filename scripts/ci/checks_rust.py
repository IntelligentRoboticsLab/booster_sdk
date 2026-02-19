#!/usr/bin/env python3
"""CI script for Rust checks: clippy and tests."""

import subprocess
import sys


def run(cmd: list[str]) -> bool:
    print(f"\n{'=' * 60}")
    print(f"Running: {' '.join(cmd)}")
    print(f"{'=' * 60}\n")
    result = subprocess.run(cmd)
    if result.returncode != 0:
        print(f"\n✗ Failed: {' '.join(cmd)}")
    else:
        print(f"\n✓ Passed: {' '.join(cmd)}")
    return result.returncode == 0


def main() -> int:
    steps = [
        ["cargo", "clippy", "--all-targets", "--all-features", "--", "-Dwarnings"],
        ["cargo", "test", "--all-targets", "--all-features"],
    ]

    failed = []
    for cmd in steps:
        if not run(cmd):
            failed.append(cmd)

    print(f"\n{'=' * 60}")
    if failed:
        print(f"✗ {len(failed)}/{len(steps)} checks failed:")
        for cmd in failed:
            print(f"  - {' '.join(cmd)}")
        return 1

    print(f"✓ All {len(steps)} checks passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
