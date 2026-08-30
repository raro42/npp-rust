# Agent loop (npp-rust)

Date: 2026-08-29  
Repo: [raro42/npp-rust](https://github.com/raro42/npp-rust)  
Branch: `main`


## Purpose

Pick up open GitHub issues, turn them into **sanitized** task files, code, **test**, then **handoff** (changelog + close). Privacy-first for a public repo.

## Pipeline (what was missing before)

Earlier the loop only ran **001 + 002**. It never called the tester or handoff. Coders also skipped ahead to `done/` and closed issues. That is fixed.

| Step | Agent | Input | Output |
|------|-------|-------|--------|
| 005 | CI watch | GitHub Actions `ci.yml` | `FEAT-ci-*-fix-github-ci.md` when red (≤1×/UTC day) |
| 001 | issue pickup | GitHub issues | `FEAT-*.md` |
| 002 | coder | `FEAT-` / `WIP-` | code on `main` + `TEST-*.md` |
| 003 | tester | `TEST-` | `done/DONE-*.md` or back to `WIP-` |
| 004 | handoff | `DONE-` without `Handoff: complete` | changelog + issue closed |

Each `once` / loop cycle runs: sync → **005** → 001 → 004 → 003 → 002 → 003 → 004  

CI watch (`scripts/ci-watch.py`) stamps `agents/state/ci-watch.stamp` so it runs at most once per UTC day unless `AGENT_CI_WATCH_FORCE=1`.

## Run

```bash
./agents/npp-cursor-loop.sh once    # full cycle
./agents/npp-cursor-loop.sh loop    # every AGENT_LOOP_SLEEP_MINUTES (default 15)
./agents/npp-cursor-loop.sh 001     # pickup only
./agents/npp-cursor-loop.sh 002     # coder only
./agents/npp-cursor-loop.sh 003     # tester only
./agents/npp-cursor-loop.sh 004     # handoff only
./agents/npp-cursor-loop.sh 005     # CI watch only (respects daily stamp)
AGENT_CI_WATCH_FORCE=1 ./agents/npp-cursor-loop.sh 005  # force CI check
```

For a long unattended session, see [unattended-20h.md](unattended-20h.md). Start via Terminal: `agents/start-unattended.command`.

### Auto agents

When `cursor-agent` is on `PATH` (usually `~/.local/bin`), the loop **runs 002/003/004 by default**.

| Env | Meaning |
|-----|---------|
| unset | Auto: `1` if `cursor-agent` exists, else `0` |
| `AGENT_USE_CURSOR=1` | Force agents on |
| `AGENT_USE_CURSOR=0` | Pickup / CI stamp only (no edits) |
| `AGENT_CI_WATCH_FORCE=1` | Run CI watch even if already stamped today |

### GitHub labels

| Label | Meaning |
|-------|---------|
| `agent:planned` | FEAT task file created |
| `agent:wip` | Coder / tester in progress |
| `agent:done` | Handoff finished (issue usually closed) |

Restart after changing the script:

```bash
pkill -f 'npp-cursor-loop.sh loop' || true
open agents/start-unattended.command
```

## Agents

| Id | File | Role |
|----|------|------|
| 005 | `agents/005-ci-watcher.md` | Failing CI → `FEAT-ci-…` (daily) |
| 001 | `agents/001-issue-reviewer.md` | Issues → `FEAT-*.md` |
| 002 | `agents/002-coder.md` | Implement → `TEST-` |
| 003 | `agents/003-tester.md` | Verify → `DONE-` |
| 004 | `agents/004-handoff.md` | Changelog + close |
| 040 | `agents/040-committer.md` | Extra commit hygiene |

Tasks: `agents/TASKS-README.md`.

## Privacy gates (do not skip)

1. Cursor rule: `.cursor/rules/public-repo-no-exfiltration.mdc` (always on).
2. Scanner: `python3 scripts/redact_public_text.py`.
3. GitHub writes: `./scripts/gh-safe.sh` (blocks comments that look private).
4. Issue checker stores **summaries only** — never the raw issue body.

Details: `docs/security-public-repo.md`.
