#!/usr/bin/env python3
"""Portable timeout wrapper for diff-prod scripts."""

from __future__ import annotations

import subprocess
import sys


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: timeout.py <seconds> <command> [args...]", file=sys.stderr)
        return 2
    try:
        seconds = float(sys.argv[1])
    except ValueError:
        print(f"invalid timeout seconds: {sys.argv[1]}", file=sys.stderr)
        return 2
    try:
        return subprocess.run(sys.argv[2:], timeout=seconds).returncode
    except subprocess.TimeoutExpired:
        return 124


if __name__ == "__main__":
    raise SystemExit(main())
