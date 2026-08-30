#!/usr/bin/env python3
"""Commit + push pending safe work (daily loop backstop).

Inspired by mac-stats scripts/overnight_git_flush.py.
Skips secrets. Does not force-push. Exits 0 when clean or flush succeeded.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

SECRET_HINTS = (
    ".env",
    "credentials",
    "secret",
    "id_rsa",
    "id_ed25519",
    ".pem",
    "token.json",
)


def run(args: list[str], check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, cwd=ROOT, text=True, capture_output=True, check=check)


def is_secret_path(path: str) -> bool:
    low = path.lower()
    return any(h in low for h in SECRET_HINTS)


def porcelain_paths() -> list[str]:
    out = run(["git", "status", "--porcelain"]).stdout.splitlines()
    paths: list[str] = []
    for ln in out:
        if not ln.strip():
            continue
        # status XY then path; handle renames "R  a -> b"
        path = ln[3:].strip()
        if " -> " in path:
            path = path.split(" -> ", 1)[1].strip()
        paths.append(path)
    return paths


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    paths = porcelain_paths()
    if not paths:
        print("git_flush: clean")
        return 0

    safe = [p for p in paths if not is_secret_path(p)]
    skipped = [p for p in paths if is_secret_path(p)]
    if skipped:
        print("git_flush: skip secrets:")
        for p in skipped:
            print(f"  - {p}")

    if not safe:
        print("git_flush: nothing safe to commit")
        return 0

    print("git_flush: would commit:")
    for p in safe:
        print(f"  - {p}")
    if args.dry_run:
        return 0

    run(["git", "add", "--", *safe], check=False)
    msg = "Agent flush: commit pending safe work from the loop backstop."
    c = run(["git", "commit", "-m", msg], check=False)
    if c.returncode != 0:
        print(c.stdout)
        print(c.stderr, file=sys.stderr)
        # Nothing new staged
        return 0
    p = run(["git", "push", "origin", "HEAD"], check=False)
    if p.returncode != 0:
        print(p.stderr, file=sys.stderr)
        return 1
    print("git_flush: pushed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
