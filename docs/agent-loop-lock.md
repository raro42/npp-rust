# Agent loop single-instance lock

Date: 2026-08-31

## Problem

`start-unattended.command` used to `pkill` and start a new loop every time. That created duplicate loops and overlapping `cursor-agent` runs.

## Locks (under `agents/state/`, gitignored)

| File | Role |
|------|------|
| `loop.pid` | **One** `npp-cursor-loop.sh loop` process. Written at loop start; cleared on exit. |
| `cursor.pid` | One `cursor-agent` spawn at a time (coder/tester/handoff). |

## Behaviour

- `./agents/npp-cursor-loop.sh loop` — acquires `loop.pid` or **exits** if another live loop holds it.
- `./agents/npp-cursor-loop.sh status` — prints running pid or “not running”.
- `agents/start-unattended.command` — if a healthy loop exists, **refuses** to start another. Does not kill by default.

## Force restart (explicit only)

```bash
AGENT_LOOP_FORCE_RESTART=1 open ./agents/start-unattended.command
# or:
AGENT_LOOP_FORCE_RESTART=1 ./agents/start-unattended.command
```

## Check

```bash
./agents/npp-cursor-loop.sh status
pgrep -fl npp-cursor-loop
tail -n 30 /tmp/npp-agent-loop.log
```
