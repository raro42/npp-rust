#!/usr/bin/env python3
"""CI watch for the npp-rs agent loop.

Runs every loop cycle (cheap `gh` calls). Always refreshes
`agents/workspace/ci-status.md` so humans need not babysit Actions.

When the latest finished CI on main is red and no CI FEAT/WIP/TEST exists,
writes `agents/tasks/FEAT-ci-*-fix-github-ci.md`.

GitHub CI itself is scheduled 2×/day (+ workflow_dispatch) — see ci.yml.
Local gate: pre-push `./scripts/ci-local.sh`.

Privacy: task text is repo-relative only. No home paths.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
AGENTS = REPO / "agents"
TASKDIR = AGENTS / "tasks"
STATEDIR = AGENTS / "state"
WORKDIR = AGENTS / "workspace"
STATUS = WORKDIR / "ci-status.md"
STAMP = STATEDIR / "ci-watch.stamp"
GH_REPO = os.environ.get("NPP_GH_REPO", "raro42/npp-rust")


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


def utc_stamp() -> str:
    return utc_now().strftime("%Y%m%d-%H%M")


def run_gh(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["gh", *args],
        cwd=REPO,
        text=True,
        capture_output=True,
        check=False,
    )


def write_stamp() -> None:
    STATEDIR.mkdir(parents=True, exist_ok=True)
    STAMP.write_text(utc_now().strftime("%Y-%m-%dT%H:%M:%SZ") + "\n", encoding="utf-8")


def open_ci_tasks() -> list[Path]:
    out: list[Path] = []
    for p in TASKDIR.glob("*.md"):
        name = p.name.lower()
        if any(name.startswith(pref) for pref in ("feat-", "wip-", "test-")) and "ci" in name:
            out.append(p)
    return out


def list_ci_runs(limit: int = 6) -> list[dict]:
    proc = run_gh(
        [
            "run",
            "list",
            "--repo",
            GH_REPO,
            "--workflow",
            "ci.yml",
            "--limit",
            str(limit),
            "--json",
            "databaseId,conclusion,headBranch,displayTitle,url,createdAt,status,event",
        ]
    )
    if proc.returncode != 0:
        print(f"ci-watch: gh failed: {proc.stderr.strip()}", file=sys.stderr)
        return []
    try:
        return json.loads(proc.stdout or "[]")
    except json.JSONDecodeError:
        return []


def write_status(runs: list[dict], note: str) -> None:
    WORKDIR.mkdir(parents=True, exist_ok=True)
    lines = [
        "# CI status (agent watch)",
        "",
        f"Updated: {utc_now().strftime('%Y-%m-%dT%H:%M:%SZ')}",
        f"Repo: `{GH_REPO}`",
        f"Workflow: `ci.yml` (schedule 06:00 + 18:00 UTC, `workflow_dispatch`)",
        "",
        note,
        "",
        "| When (UTC) | Event | Status | Conclusion | Title | Run |",
        "|------------|-------|--------|------------|-------|-----|",
    ]
    for row in runs[:6]:
        created = (row.get("createdAt") or "")[:19].replace("T", " ")
        event = row.get("event") or "?"
        status = row.get("status") or "?"
        conclusion = row.get("conclusion") or "—"
        title = (row.get("displayTitle") or "CI").replace("|", "/").replace("\n", " ")[:60]
        rid = row.get("databaseId")
        url = row.get("url") or ""
        link = f"[`{rid}`]({url})" if rid and url else str(rid)
        lines.append(
            f"| {created} | {event} | {status} | {conclusion} | {title} | {link} |"
        )
    lines.extend(
        [
            "",
            "## Who watches",
            "",
            "- Agent loop step **005** runs `scripts/ci-watch.py` every cycle.",
            "- On **failure**: creates `FEAT-ci-…` and spawns a fixer (if no CI task open).",
            "- Local: `pre-push` still runs `./scripts/ci-local.sh` (free; no GitHub minutes).",
            "",
            "Manual: `gh workflow run ci.yml --ref main`",
            "",
        ]
    )
    STATUS.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_feat(failures: list[dict]) -> Path:
    TASKDIR.mkdir(parents=True, exist_ok=True)
    path = TASKDIR / f"FEAT-ci-{utc_stamp()}-fix-github-ci.md"
    lines = [
        "# Fix failing GitHub CI",
        "",
        "## Goal",
        "Make `.github/workflows/ci.yml` green on `main`.",
        "",
        "## Local gates (must pass before push)",
        "- `./scripts/ci-local.sh`",
        "",
        "## Recent failures (sanitized)",
    ]
    for row in failures[:5]:
        title = (row.get("displayTitle") or "CI").replace("\n", " ")[:120]
        rid = row.get("databaseId")
        lines.append(f"- run `{rid}`: {title}")
        lines.append(f"  - inspect: `gh run view {rid} --log-failed` (redact private paths)")
    lines.extend(
        [
            "",
            "## Steps",
            "1. Reproduce with `./scripts/ci-local.sh` on `main`.",
            "2. Fix fmt/clippy/tests.",
            "3. Commit and push `main`.",
            "4. Trigger CI if needed: `gh workflow run ci.yml --ref main`",
            "5. Confirm green: `gh run list --workflow=ci.yml --limit 3`",
            "",
            "## Privacy",
            "- No home paths, secrets, or emails in commits or task notes.",
            "",
            f"Created: {utc_now().strftime('%Y-%m-%dT%H:%M:%SZ')}",
            "",
        ]
    )
    path.write_text("\n".join(lines), encoding="utf-8")
    return path


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--check-only",
        action="store_true",
        help="Refresh status file; do not write FEAT tasks",
    )
    ap.add_argument(
        "--force",
        action="store_true",
        help="Ignored (kept for loop compatibility); watch always runs",
    )
    args = ap.parse_args()

    runs = list_ci_runs()
    if not runs:
        write_status([], "Note: no CI runs returned (gh auth or empty history).")
        write_stamp()
        print("ci-watch: no runs listed")
        return 0

    # Prefer latest finished run on main for health; ignore in_progress for "red".
    finished = [
        r
        for r in runs
        if (r.get("headBranch") or "main") in ("main", "")
        and r.get("status") == "completed"
    ]
    in_flight = [r for r in runs if r.get("status") == "in_progress"]
    failures = [r for r in finished if r.get("conclusion") == "failure"]

    if in_flight:
        note = f"In progress: {len(in_flight)} run(s). Watcher will re-check next cycle."
    elif failures and finished and finished[0].get("conclusion") == "failure":
        note = f"**RED** — latest finished run failed (`{finished[0].get('databaseId')}`)."
    elif finished and finished[0].get("conclusion") == "success":
        note = f"**GREEN** — latest finished run `{finished[0].get('databaseId')}` succeeded."
    else:
        note = "No clear finished success/failure on main yet."

    write_status(runs, note)
    write_stamp()
    print(f"ci-watch: {note}")

    # Only queue FEAT when the *latest* finished run is a failure.
    latest_failed = bool(finished) and finished[0].get("conclusion") == "failure"
    if not latest_failed:
        return 0

    existing = open_ci_tasks()
    if existing:
        print(f"ci-watch: open CI task already exists: {existing[0].name}")
        return 1

    if args.check_only:
        return 1

    path = write_feat(failures[:3] or finished[:1])
    print(f"ci-watch: wrote {path.relative_to(REPO)}")
    return 2


if __name__ == "__main__":
    sys.exit(main())
