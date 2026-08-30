# Agent loop (npp-rust)

Date: 2026-08-30  
Repo: [raro42/npp-rust](https://github.com/raro42/npp-rust)  
Branch: `main`

## Purpose

Pick up open GitHub issues, turn them into **sanitized** task files, code, **test**, then **handoff** (changelog + close). Also watch **CI**, **panic logs**, **repo quality**, and **dirty git** so the operator does not have to babysit. Privacy-first for a public repo.

Inspired by mac-stats agent-ops — see [agent-loop-mac-stats-inspiration.md](agent-loop-mac-stats-inspiration.md).

## Standing rules

See [agents/README.md](../agents/README.md): always test, watch CI, skim panic log, no dirty leftovers, read `agents/workspace/lessons.md`.

## Pipeline

| Step | Agent | Input | Output |
|------|-------|-------|--------|
| 005 | CI watch | GitHub Actions `ci.yml` | `FEAT-ci-…` when red (≤1×/UTC day) |
| 006 | Log monitor | `logs/panic.log` | `FEAT-log-…` on new signature |
| 007 | Quality | repo layout | fix or FEAT (≤1×/UTC week) |
| 008 | Git flush | dirty tree | commit+push safe files (≤1×/UTC day) |
| 001 | Issue pickup | GitHub issues | `FEAT-*.md` |
| 002 | Coder | `FEAT-` / `WIP-` | code on `main` + `TEST-*.md` |
| 003 | Tester | `TEST-` | `done/DONE-*.md` or back to `WIP-` |
| 004 | Handoff | `DONE-` without `Handoff: complete` | changelog + issue closed |

Each `once` / loop cycle:

`sync → 005 → 006 → 007 → 008 → 001 → 004 → 003 → 002 → 003 → 004`

Observability lines: `AGENT_LOOP_TICK` / `AGENT_LOOP_SLEEP`. Cursor spawns use `agents/state/agent.pid` so two agents do not overlap.

## Run

```bash
./agents/npp-cursor-loop.sh once
./agents/npp-cursor-loop.sh loop
./agents/npp-cursor-loop.sh 005   # CI
./agents/npp-cursor-loop.sh 006   # panic log
./agents/npp-cursor-loop.sh 007   # quality
./agents/npp-cursor-loop.sh 008   # git flush (forced)
```

Unattended: [unattended-20h.md](unattended-20h.md) · `agents/start-unattended.command`.

### Env

| Env | Meaning |
|-----|---------|
| `AGENT_USE_CURSOR` | `1`/`0` (default: auto if `cursor-agent` on PATH) |
| `AGENT_CI_WATCH_FORCE=1` | Ignore daily CI stamp |
| `AGENT_QUALITY_FORCE=1` | Ignore weekly quality stamp |
| `AGENT_GIT_FLUSH_FORCE=1` | Ignore daily git-flush stamp |
| `AGENT_LOOP_SLEEP_MINUTES` | Loop sleep (default 15) |

### GitHub labels

| Label | Meaning |
|-------|---------|
| `agent:planned` | FEAT task file created |
| `agent:wip` | Coder / tester in progress |
| `agent:done` | Handoff finished |

## Privacy gates

1. `.cursor/rules/public-repo-no-exfiltration.mdc`
2. `python3 scripts/redact_public_text.py`
3. `./scripts/gh-safe.sh`
4. Issue checker stores **summaries only**

Details: `docs/security-public-repo.md`.
