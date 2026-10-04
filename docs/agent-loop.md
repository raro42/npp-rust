# Agent loop (npp-rust)

Date: 2026-10-03  
Repo: [raro42/npp-rust](https://github.com/raro42/npp-rust)  
Branch: `main`

## Purpose

Pick up open GitHub issues, turn them into **sanitized** task files, code, **test**, then **handoff** (changelog + close). Also watch **CI**, **panic logs**, **repo quality**, and **dirty git**. Overnight, when the queue is idle, run **autoresearch** (keep or discard). Privacy-first for a public repo.

Inspired by backoffice (systemd user unit) and mac-stats agent-ops. See [agent-loop-mac-stats-inspiration.md](agent-loop-mac-stats-inspiration.md) and [autoresearch/program.md](autoresearch/program.md).

## Standing rules

See [agents/README.md](../agents/README.md): always test, watch CI, skim panic log, no dirty leftovers, read `agents/workspace/lessons.md`.

## Pipeline

| Step | Agent | Input | Output |
|------|-------|-------|--------|
| 005 | CI watch | GitHub Actions `ci.yml` | refresh `ci-status.md` every cycle; `FEAT-ci-…` when latest finished run is red |
| 006 | Log monitor | `logs/panic.log` | `FEAT-log-…` on new signature |
| 007 | Quality | repo layout | fix or FEAT (≤1×/UTC week) |
| 008 | Git flush | dirty tree | commit+push safe files (≤1×/UTC day) |
| 001 | Issue pickup | GitHub issues | `FEAT-*.md` |
| 002 | Coder | `FEAT-` / `WIP-` | code on `main` + `TEST-*.md` |
| 003 | Tester | `TEST-` | `done/DONE-*.md` or back to `WIP-` |
| 004 | Handoff | `DONE-` without `Handoff: complete` | changelog + issue closed |
| 009 | Autoresearch | idle queue, 20:00–06:00 local | one keep/discard experiment |

Each `once` / loop cycle:

`sync → 005 → 006 → 007 → 008 → 001 → 002 → 003 → 004 → 003 → 004 → 009`

Observability lines: `AGENT_LOOP_TICK` / `AGENT_LOOP_SLEEP`.  
**Locks:** `agents/state/loop.pid` (one loop) and `agents/state/cursor.pid` (one cursor-agent). See [agent-loop-lock.md](agent-loop-lock.md).

Issue work always wins. Step 009 skips when a live `FEAT-` / `WIP-` / `TEST-` / pending handoff exists.

## Run

Preferred (Linux, reboot-safe, same idea as backoffice):

```bash
python3 scripts/install_npp_rs_units.py
systemctl --user status npp-rs-agent-loop.service
```

Stop and stay stopped (watchdog will not start a disabled unit):

```bash
systemctl --user disable --now npp-rs-agent-loop.service
```

Manual:

```bash
./agents/npp-cursor-loop.sh status
./agents/npp-cursor-loop.sh once
./agents/npp-cursor-loop.sh loop
./agents/npp-cursor-loop.sh 009   # force one research tick
```

Mac / ad-hoc: [unattended-20h.md](unattended-20h.md) · `agents/start-unattended.command`.

### Poll pace

| Situation | Next cycle |
|-----------|------------|
| Live `FEAT`, `WIP`, `TEST`, or pending `DONE` handoff | `AGENT_LOOP_BUSY_SLEEP_SECONDS` (default 15) |
| Idle | `AGENT_LOOP_SLEEP_MINUTES` (default 15; systemd installer sets 5) |
| Autoresearch spawn | at most once per `AGENT_AUTORESEARCH_INTERVAL_SECONDS` (default 1200) |

### Env

| Env | Meaning |
|-----|---------|
| `AGENT_USE_CURSOR` | `1`/`0` (default: auto if `cursor-agent` on PATH) |
| `AGENT_CI_WATCH_FORCE=1` | Ignore daily CI stamp |
| `AGENT_QUALITY_FORCE=1` | Ignore weekly quality stamp |
| `AGENT_GIT_FLUSH_FORCE=1` | Ignore daily git-flush stamp |
| `AGENT_LOOP_SLEEP_MINUTES` | Idle sleep |
| `AGENT_LOOP_BUSY_SLEEP_SECONDS` | Sleep between cycles while the queue has work |
| `AGENT_AUTORESEARCH=0` | Disable overnight research |
| `AGENT_AUTORESEARCH_FORCE=1` | Run 009 outside the night window |
| `AGENT_GIT_SYNC=0` | Skip fetch/pull (default is sync when the tree is clean) |
| `AGENT_LOOP_FORCE_RESTART=1` | Allow start-unattended to replace a live loop |

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
