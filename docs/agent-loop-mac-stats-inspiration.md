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
| Single-agent lock (no overlap) | `agents/state/loop.pid` + `cursor.pid` |
| Daily CI watch | Step 005 |
| Autoresearch keep/discard | `docs/autoresearch/` + `scripts/autoresearch_ratchet.py` + loop step 009 |
| Reboot-safe loop | systemd user unit (`scripts/install_npp_rs_units.py`), like backoffice |

## What we did **not** copy

- OpenClaw / Discord / Ollama tool loops
- mac-stats sibling harness watcher / digester / morning-surprise home files
- LaunchAgent plists (Linux uses systemd)

## Standing expectation

Do not wait for the operator to ask why CI is red, panic.log grew, or the tree is dirty. The loop must notice. Overnight, idle ticks must keep or discard, not only sleep.
