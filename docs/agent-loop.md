# Agent loop (npp-rust)

Date: 2026-08-28  
Repo: [raro42/npp-rust](https://github.com/raro42/npp-rust)  
Branch: `dev`

## Purpose

Pick up open GitHub issues, turn them into **sanitized** task files, then (optionally) code and test. Modelled after the pos2 loop, but **privacy-first** for a public repo.

## Run

```bash
./agents/npp-cursor-loop.sh once    # one cycle (pickup + coder)
./agents/npp-cursor-loop.sh loop    # every AGENT_LOOP_SLEEP_MINUTES (default 15)
./agents/npp-cursor-loop.sh 001     # issue pickup only
```

For a long unattended session (commit + push, honest placeholders), see [unattended-20h.md](unattended-20h.md).

### Auto-coder (002)

When `cursor-agent` is on `PATH` (usually `~/.local/bin`), the loop **runs the coder by default**.

| Env | Meaning |
|-----|---------|
| unset | Auto: `1` if `cursor-agent` exists, else `0` |
| `AGENT_USE_CURSOR=1` | Force coder on |
| `AGENT_USE_CURSOR=0` | Pickup only (no edits) |

### GitHub labels (what you see on issues)

| Label | Meaning |
|-------|---------|
| `agent:planned` | FEAT task file created |
| `agent:wip` | Coder started (FEAT renamed to WIP) |
| `agent:done` | Task finished (when agents set it) |

Restart after changing the script:

```bash
pkill -f 'npp-cursor-loop.sh loop' || true
./agents/npp-cursor-loop.sh loop
```

Issue pickup only:

```bash
python3 agents/issue_checker.py
```

## Agents

| Id | File | Role |
|----|------|------|
| 001 | `agents/001-issue-reviewer.md` | Issues → `FEAT-*.md` |
| 002 | `agents/002-coder.md` | Implement |
| 003 | `agents/003-tester.md` | Verify |
| 040 | `agents/040-committer.md` | Commit when asked |

Tasks: `agents/TASKS-README.md`.

## Privacy gates (do not skip)

1. Cursor rule: `.cursor/rules/public-repo-no-exfiltration.mdc` (always on).
2. Scanner: `python3 scripts/redact_public_text.py`.
3. GitHub writes: `./scripts/gh-safe.sh` (blocks comments that look private).
4. Issue checker stores **summaries only** — never the raw issue body.

Details: `docs/security-public-repo.md`.
