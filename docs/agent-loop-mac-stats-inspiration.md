# Agent loop inspiration (from mac-stats)

Date: 2026-08-30  
Source: `~/projects/mac-stats/agents/` + overnight harness scripts

## What we took

| mac-stats idea | npp-rs adoption |
|----------------|-----------------|
| Standing rules: always test / always read logs | `agents/README.md` |
| Session `workspace/` (todo + lessons) | `agents/workspace/` |
| Log monitor (read-only scan) | `agents/006-log-monitor/` + `scripts/scan_panic_log.py` |
| Weekly quality / root clutter | `agents/007-quality/` + `scripts/scan_repo_quality.py` |
| Overnight git flush (no dirty leftovers) | `scripts/git_flush.py` + loop step 008 |
| Loop observability ticks | `AGENT_LOOP_TICK` / sleep notes in `npp-cursor-loop.sh` |
| Single-agent lock (no overlap) | `agents/state/agent.pid` |
| Daily CI watch | Already: step 005 |

## What we did **not** copy

- OpenClaw / Discord / Ollama tool loops
- Autoresearch ratchets and sibling harnesses
- Night-only window (20:00–06:00) — npp loop stays on-demand / unattended

## Standing expectation

Do not wait for the operator to ask why CI is red, panic.log grew, or the tree is dirty. The loop must notice.
