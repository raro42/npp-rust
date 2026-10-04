# Unattended loop

Date: 2026-10-03

On Linux the durable path is a **systemd user unit** (same idea as backoffice). Mac still uses Terminal + `start-unattended.command`.

## Linux (preferred)

Needs `cursor-agent` on `PATH`, `gh` authenticated, git able to push `origin/main`.

```bash
python3 scripts/install_npp_rs_units.py
systemctl --user status npp-rs-agent-loop.service
journalctl --user -u npp-rs-agent-loop.service -f
```

The watchdog timer restarts the loop if the unit is **enabled** and down. Disable the unit to stop for good:

```bash
systemctl --user disable --now npp-rs-agent-loop.service
```

Linger should be on so the loop survives logout (`loginctl show-user -p Linger`).

## Mac / one-off Terminal

1. Leave the machine on.
2. `gh auth status` works; `cursor-agent` on `PATH`; git can push `main`.
3. Double-click `agents/start-unattended.command`, or:

```bash
export PATH="$HOME/.local/bin:$PATH"
export AGENT_USE_CURSOR=1 AGENT_LOOP_SLEEP_MINUTES=5
./agents/start-unattended.command
```

Do not rely on a Cursor chat shell for `nohup`. That session often dies and takes the loop with it.

Force replace a live loop:

```bash
AGENT_LOOP_FORCE_RESTART=1 ./agents/start-unattended.command
```

## Check progress

```bash
./agents/npp-cursor-loop.sh status
systemctl --user is-active npp-rs-agent-loop.service
git log origin/main -5 --oneline
python3 scripts/autoresearch_ratchet.py status
```

## Expectation

Small commits on `main`, CI gates via `./scripts/ci-local.sh`, overnight keep/discard when the issue queue is idle. You will not get a full Notepad++ clone in one night.
