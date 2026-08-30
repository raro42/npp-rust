#!/usr/bin/env python3
"""Scan logs/panic.log for recent panic signatures (read-only by default)."""

from __future__ import annotations

import argparse
import hashlib
import re
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LOG = ROOT / "logs" / "panic.log"
TASKDIR = ROOT / "agents" / "tasks"
STATEDIR = ROOT / "agents" / "state"
SEEN = STATEDIR / "panic-signatures.txt"


def utc_stamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y%m%d-%H%M")


def load_seen() -> set[str]:
    if not SEEN.is_file():
        return set()
    return {ln.strip() for ln in SEEN.read_text(encoding="utf-8").splitlines() if ln.strip()}


def save_seen(seen: set[str]) -> None:
    STATEDIR.mkdir(parents=True, exist_ok=True)
    SEEN.write_text("\n".join(sorted(seen)) + "\n", encoding="utf-8")


def extract_heads(text: str) -> list[str]:
    """Return short panic heads (first meaningful line after 'panic')."""
    heads: list[str] = []
    for block in re.split(r"\nnpp-rs panic", text):
        lines = [ln.strip() for ln in block.splitlines() if ln.strip()]
        if not lines:
            continue
        # Prefer a rust panic message line
        head = lines[0][:160]
        for ln in lines[:8]:
            if "panicked at" in ln or "panic" in ln.lower():
                head = ln[:160]
                break
        heads.append(head)
    return heads


def sig(head: str) -> str:
    return hashlib.sha256(head.encode("utf-8")).hexdigest()[:16]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--write-finding", action="store_true")
    args = ap.parse_args()

    if not LOG.is_file():
        print("scan_panic_log: no logs/panic.log")
        return 0

    text = LOG.read_text(encoding="utf-8", errors="replace")
    if not text.strip():
        print("scan_panic_log: empty log")
        return 0

    heads = extract_heads(text)
    if not heads:
        # Whole file as one blob when format differs
        heads = [text.strip().splitlines()[0][:160]]

    seen = load_seen()
    new: list[str] = []
    for h in heads[-20:]:
        s = sig(h)
        if s not in seen:
            new.append(h)
            seen.add(s)

    if not new:
        print("scan_panic_log: no new panic signatures")
        return 0

    print(f"scan_panic_log: {len(new)} new signature(s)")
    for h in new:
        print(f"  - {h}")

    if not args.write_finding:
        return 1

    TASKDIR.mkdir(parents=True, exist_ok=True)
    path = TASKDIR / f"FEAT-log-{utc_stamp()}-panic.md"
    body = "\n".join(
        [
            "# Investigate panic log signature",
            "",
            "## Goal",
            "Reproduce and fix the crash. Keep notes repo-relative only.",
            "",
            "## Signatures (sanitized)",
            *[f"- `{h}`" for h in new],
            "",
            "## Steps",
            "1. Read `logs/panic.log` locally (do not commit home paths).",
            "2. Add a regression test when practical.",
            "3. Run `./scripts/ci-local.sh`, commit, push `main`.",
            "",
            f"Created: {datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')}",
            "",
        ]
    )
    path.write_text(body, encoding="utf-8")
    save_seen(seen)
    print(f"scan_panic_log: wrote {path.relative_to(ROOT)}")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
