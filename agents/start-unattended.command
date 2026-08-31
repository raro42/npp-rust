#!/bin/bash
# Start the npp-rs agent loop in Terminal (single instance).
# Does NOT kill an existing healthy loop — use AGENT_LOOP_FORCE_RESTART=1 to replace.
cd "$(dirname "$0")/.."
export PATH="$HOME/.local/bin:/usr/local/bin:$PATH"
export AGENT_USE_CURSOR=1
export AGENT_LOOP_SLEEP_MINUTES=5

STATE_DIR="agents/state"
LOOP_LOCK="${STATE_DIR}/loop.pid"
mkdir -p "$STATE_DIR"

loop_alive() {
  local pid="$1"
  [[ -n "$pid" ]] || return 1
  kill -0 "$pid" 2>/dev/null || return 1
  local cmd
  cmd="$(ps -p "$pid" -o command= 2>/dev/null || true)"
  [[ "$cmd" == *npp-cursor-loop.sh* ]]
}

if [[ -f "$LOOP_LOCK" ]]; then
  old="$(tr -d ' \n' <"$LOOP_LOCK" 2>/dev/null || true)"
  if loop_alive "$old"; then
    if [[ "${AGENT_LOOP_FORCE_RESTART:-0}" == "1" ]]; then
      echo "Force restart: stopping loop pid $old"
      kill "$old" 2>/dev/null || true
      sleep 1
      kill -9 "$old" 2>/dev/null || true
      rm -f "$LOOP_LOCK"
    else
      echo "Loop already running (pid $old). Refusing duplicate."
      echo "Log: /tmp/npp-agent-loop.log"
      echo "Status: ./agents/npp-cursor-loop.sh status"
      echo "Force: AGENT_LOOP_FORCE_RESTART=1 open agents/start-unattended.command"
      tail -n 20 /tmp/npp-agent-loop.log 2>/dev/null || true
      exit 0
    fi
  else
    echo "Clearing stale loop lock (pid $old)"
    rm -f "$LOOP_LOCK"
  fi
fi

# Optional: keep Mac awake while unattended
if [[ ! -f /tmp/npp-caffeinate.pid ]] || ! kill -0 "$(cat /tmp/npp-caffeinate.pid 2>/dev/null)" 2>/dev/null; then
  caffeinate -dimsu -t 72000 >>/tmp/npp-caffeinate.log 2>&1 &
  echo $! >/tmp/npp-caffeinate.pid
fi

nohup ./agents/npp-cursor-loop.sh loop >>/tmp/npp-agent-loop.log 2>&1 &
echo $! >/tmp/npp-agent-loop.pid
sleep 1
if ./agents/npp-cursor-loop.sh status; then
  echo "Log: /tmp/npp-agent-loop.log"
else
  echo "WARNING: loop failed to acquire lock or exited early. See /tmp/npp-agent-loop.log" >&2
fi
tail -f /tmp/npp-agent-loop.log
