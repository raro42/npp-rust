#!/usr/bin/env python3
"""Daily CI watch for the npp-rs agent loop.

Looks at recent GitHub Actions CI failures on main/dev.
At most once per UTC day (unless --force), writes a FEAT task when CI is red
and no CI fix task is already open.

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
STAMP = STATEDIR / "ci-watch.stamp"
GH_REPO = os.environ.get("NPP_GH_REPO", "raro42/npp-rust")


def utc_today() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%d")


def utc_stamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y%m%d-%H%M")


def run_gh(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["gh", *args],
        cwd=REPO,
        text=True,
        capture_output=True,
        check=False,
    )


def already_checked_today() -> bool:
    if not STAMP.is_file():
        return False
    try:
        return STAMP.read_text(encoding="utf-8").strip()[:10] == utc_today()
    except OSError:
        return False


def write_stamp() -> None:
    STATEDIR.mkdir(parents=True, exist_ok=True)
    STAMP.write_text(
        f"{datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')}\n",
        encoding="utf-8",
    )


def open_ci_tasks() -> list[Path]:
    out: list[Path] = []
    for pattern in ("FEAT-*ci*.md", "WIP-*ci*.md", "TEST-*ci*.md", "FEAT-ci-*.md", "WIP-ci-*.md"):
        out.extend(TASKDIR.glob(pattern))
    # Also match FEAT-0-…-fix-ci style
    for p in TASKDIR.glob("*.md"):
        name = p.name.lower()
        if any(name.startswith(pref) for pref in ("feat-", "wip-", "test-")) and "ci" in name:
            if p not in out:
                out.append(p)
    return out


def list_failed_ci() -> list[dict]:
    proc = run_gh(
        [
            "run",
            "list",
            "--repo",
            GH_REPO,
            "--workflow",
            "ci.yml",
            "--limit",
            "8",
            "--json",
            "databaseId,conclusion,headBranch,displayTitle,url,createdAt,status",
        ]
    )
    if proc.returncode != 0:
        print(f"ci-watch: gh failed: {proc.stderr.strip()}", file=sys.stderr)
        return []
    try:
        rows = json.loads(proc.stdout or "[]")
    except json.JSONDecodeError:
        return []
    failed = []
    for row in rows:
        if row.get("conclusion") != "failure":
            continue
        branch = row.get("headBranch") or ""
        if branch not in ("main", "dev"):
            continue
        failed.append(row)
    return failed


def write_feat(failures: list[dict]) -> Path:
    TASKDIR.mkdir(parents=True, exist_ok=True)
    stamp = utc_stamp()
    path = TASKDIR / f"FEAT-ci-{stamp}-fix-github-ci.md"
    lines = [
        "# Fix failing GitHub CI",
        "",
        "## Goal",
        "Make `.github/workflows/ci.yml` green on `dev` and `main`.",
        "",
        "## Local gates (must pass before push)",
        "- `./scripts/ci-local.sh`",
        "- or: `cargo fmt --all -- --check`",
        "- `cargo clippy --workspace --all-targets -- -D warnings`",
        "- `cargo test --workspace`",
        "",
        "## Recent failures (sanitized)",
    ]
    for row in failures[:5]:
        title = (row.get("displayTitle") or "CI").replace("\n", " ")[:120]
        branch = row.get("headBranch") or "?"
        rid = row.get("databaseId")
        lines.append(f"- branch `{branch}` run `{rid}`: {title}")
        lines.append(f"  - inspect: `gh run view {rid} --log-failed` (redact private paths)")
    lines.extend(
        [
            "",
            "## Steps",
            "1. Reproduce with `./scripts/ci-local.sh` on branch `dev`.",
            "2. Fix fmt/clippy/tests.",
            "3. Commit, push `dev`, fast-forward `main` if tip should match.",
            "4. Confirm a new CI run succeeds.",
            "",
            "## Privacy",
            "- No home paths, secrets, or emails in commits or task notes.",
            "",
            f"Created: {datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')}",
            "",
        ]
    )
    path.write_text("\n".join(lines), encoding="utf-8")
    return path


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--force", action="store_true", help="Ignore daily stamp")
    ap.add_argument(
        "--check-only",
        action="store_true",
        help="Print status; do not write tasks or stamp",
    )
    args = ap.parse_args()

    if not args.force and not args.check_only and already_checked_today():
        print(f"ci-watch: already checked today ({utc_today()}); skip")
        return 0

    failures = list_failed_ci()
    if not failures:
        print("ci-watch: no recent CI failures on main/dev")
        if not args.check_only:
            write_stamp()
        return 0

    print(f"ci-watch: {len(failures)} recent failure(s) on main/dev")
    existing = open_ci_tasks()
    if existing:
        print(f"ci-watch: open CI task already exists: {existing[0].name}")
        if not args.check_only:
            write_stamp()
        return 0

    if args.check_only:
        return 1

    path = write_feat(failures)
    write_stamp()
    print(f"ci-watch: wrote {path.relative_to(REPO)}")
    return 2  # signal: new FEAT created


if __name__ == "__main__":
    sys.exit(main())
