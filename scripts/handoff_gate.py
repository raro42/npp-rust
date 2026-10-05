#!/usr/bin/env python3
"""Local DONE-task handoff state. GitHub close is done by the loop after this.

Handoff is finished only when the task is not deferred AND either:
- there is no numeric GitHub issue, and the file has ``Handoff: complete``, or
- the file has ``GitHub: closed`` (written after a successful close).

A file may say ``Handoff: complete`` while the issue is still open. That is
unfinished; the loop must close GitHub and stamp ``GitHub: closed``.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

RE_DEFERRED = re.compile(r"(?m)^[\s-]*Handoff:\s*deferred\b")
RE_COMPLETE = re.compile(r"(?m)^[\s-]*Handoff:\s*complete\b")
RE_GH_CLOSED = re.compile(r"(?m)^[\s-]*GitHub:\s*closed\b")
RE_ISSUE_NUM = re.compile(r"^(?:FEAT|WIP|TEST|DONE)-(\d+)-")


def parse_text(text: str) -> dict[str, bool]:
    return {
        "deferred": bool(RE_DEFERRED.search(text)),
        "complete": bool(RE_COMPLETE.search(text)),
        "github_closed": bool(RE_GH_CLOSED.search(text)),
    }


def issue_number_from_name(name: str) -> int | None:
    m = RE_ISSUE_NUM.match(name)
    if not m:
        return None
    return int(m.group(1))


def needs_github_query(text: str, has_issue: bool) -> bool:
    """True when the loop must ask GitHub (issue still open?)."""
    p = parse_text(text)
    if p["deferred"] or p["github_closed"]:
        return False
    return has_issue


def locally_pending(text: str, has_issue: bool) -> bool:
    """Pending without talking to GitHub.

    If there is a numeric issue and no ``GitHub: closed``, treat as pending
    (caller confirms with ``gh``). Non-issue DONE files pending until complete.
    """
    p = parse_text(text)
    if p["deferred"] or p["github_closed"]:
        return False
    if has_issue:
        return True
    return not p["complete"]


def awaiting_handoff(text: str, has_issue: bool, issue_open: bool | None) -> bool:
    p = parse_text(text)
    if p["deferred"] or p["github_closed"]:
        return False
    if has_issue:
        if issue_open is True:
            return True
        if issue_open is False:
            return False
        return True
    return not p["complete"]


def stamp_github_closed(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    if RE_GH_CLOSED.search(text):
        return
    if not text.endswith("\n"):
        text += "\n"
    if not RE_COMPLETE.search(text):
        text += "Handoff: complete\n"
    text += "GitHub: closed\n"
    path.write_text(text, encoding="utf-8")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=("pending", "needs-gh", "stamp-closed"))
    ap.add_argument("path", type=Path)
    args = ap.parse_args()
    path: Path = args.path
    name = path.name
    n = issue_number_from_name(name)
    has_issue = n is not None
    text = path.read_text(encoding="utf-8") if path.is_file() else ""
    if args.cmd == "pending":
        return 0 if locally_pending(text, has_issue) else 1
    if args.cmd == "needs-gh":
        return 0 if needs_github_query(text, has_issue) else 1
    stamp_github_closed(path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
