#!/usr/bin/env python3
"""Install reboot-safe systemd user units for the npp-rs agent loop.

Writes units under the user systemd directory (not the repo).
Does not copy tokens into the repo. Relies on `gh` already authenticated.
"""

from __future__ import annotations

import getpass
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UNIT_DIR = Path.home() / ".config" / "systemd" / "user"
STATE_DIR = Path.home() / ".local" / "state" / "npp-rs"
HOME = Path.home()
PYTHON = sys.executable

PATH_VALUE = (
    f"{HOME}/.cargo/bin:"
    f"{HOME}/.local/bin:"
    f"{HOME}/.local/share/mise/installs/gh/latest/gh_2.100.0_linux_amd64/bin:"
    f"/usr/local/bin:/usr/bin"
)

AGENT_SERVICE = f"""[Unit]
Description=npp-rs GitHub issue + autoresearch agent loop
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
WorkingDirectory={ROOT}
Environment=HOME={HOME}
Environment=PATH={PATH_VALUE}
Environment=AGENT_USE_CURSOR=1
Environment=AGENT_LOOP_SLEEP_MINUTES=5
Environment=AGENT_LOOP_BUSY_SLEEP_SECONDS=15
Environment=AGENT_AUTORESEARCH=1
Environment=AGENT_GIT_SYNC=1
ExecStart=/usr/bin/bash {ROOT}/agents/npp-cursor-loop.sh loop
Restart=always
RestartSec=20
KillMode=mixed
TimeoutStopSec=60

[Install]
WantedBy=default.target
"""

WATCHDOG_SERVICE = f"""[Unit]
Description=npp-rs agent loop watchdog
After=network-online.target

[Service]
Type=oneshot
WorkingDirectory={ROOT}
Environment=HOME={HOME}
Environment=PATH={PATH_VALUE}
ExecStart={PYTHON} {ROOT}/scripts/npp_rs_watchdog.py
"""

WATCHDOG_TIMER = """[Unit]
Description=Run npp-rs watchdog every 5 minutes

[Timer]
OnBootSec=2min
OnUnitActiveSec=5min
AccuracySec=30s
Persistent=true
Unit=npp-rs-watchdog.service

[Install]
WantedBy=timers.target
"""


def run(cmd: list[str]) -> None:
    print("+", " ".join(cmd), flush=True)
    subprocess.run(cmd, check=True)


def main() -> int:
    UNIT_DIR.mkdir(parents=True, exist_ok=True)
    STATE_DIR.mkdir(parents=True, exist_ok=True)
    (UNIT_DIR / "npp-rs-agent-loop.service").write_text(AGENT_SERVICE, encoding="utf-8")
    (UNIT_DIR / "npp-rs-watchdog.service").write_text(WATCHDOG_SERVICE, encoding="utf-8")
    (UNIT_DIR / "npp-rs-watchdog.timer").write_text(WATCHDOG_TIMER, encoding="utf-8")
    print(f"wrote units in {UNIT_DIR}")

    user = os.environ.get("USER") or getpass.getuser()
    linger = subprocess.run(
        ["loginctl", "show-user", user, "-p", "Linger"],
        text=True,
        capture_output=True,
        check=False,
    ).stdout
    if "Linger=yes" not in linger:
        print("NOTE: user linger is off. Loop may stop at logout.")
        print("Enable with: loginctl enable-linger")

    run(["systemctl", "--user", "daemon-reload"])
    run(["systemctl", "--user", "enable", "--now", "npp-rs-agent-loop.service"])
    run(["systemctl", "--user", "enable", "--now", "npp-rs-watchdog.timer"])
    run(["systemctl", "--user", "start", "npp-rs-watchdog.service"])

    print("--- status ---")
    subprocess.run(
        ["systemctl", "--user", "status", "npp-rs-agent-loop.service", "--no-pager"],
        check=False,
    )
    subprocess.run(
        ["systemctl", "--user", "list-timers", "npp-rs-*.timer", "--no-pager"],
        check=False,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
