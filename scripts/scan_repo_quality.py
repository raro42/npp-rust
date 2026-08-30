#!/usr/bin/env python3
"""Light repo quality scan for npp-rs (inspired by mac-stats scan_repo_quality)."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

# Unexpected junk at repo root (not the usual cargo/docs layout).
FORBIDDEN_ROOT_GLOBS = (
    "*.tmp",
    "*.bak",
    "Untitled*",
    "Copy of *",
)

REQUIRED = (
    "Cargo.toml",
    "README.md",
    "LICENSE",
    "CONTRIBUTING.md",
    "scripts/ci-local.sh",
    "scripts/ci-watch.py",
    "agents/npp-cursor-loop.sh",
    ".github/workflows/ci.yml",
)


def main() -> int:
    fails: list[str] = []
    warns: list[str] = []

    for rel in REQUIRED:
        if not (ROOT / rel).exists():
            fails.append(f"missing required path: {rel}")

    for pattern in FORBIDDEN_ROOT_GLOBS:
        for p in ROOT.glob(pattern):
            if p.is_file():
                fails.append(f"forbidden root file: {p.name}")

    # Warn on large accidental binaries at root
    for p in ROOT.iterdir():
        if p.is_file() and p.suffix in {".dmg", ".app", ".exe", ".zip"} and p.stat().st_size > 1_000_000:
            warns.append(f"large binary at root: {p.name}")

    for w in warns:
        print(f"WARN {w}")
    for f in fails:
        print(f"FAIL {f}")

    if fails:
        print(f"scan_repo_quality: {len(fails)} fail(s)")
        return 1
    print("scan_repo_quality: OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
