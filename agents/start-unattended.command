#!/bin/bash
cd "$(dirname "$0")/.."
export PATH="$HOME/.local/bin:/usr/local/bin:$PATH"
export AGENT_USE_CURSOR=1
export AGENT_LOOP_SLEEP_MINUTES=5
pkill -f 'npp-cursor-loop.sh loop' 2>/dev/null || true
caffeinate -dimsu -t 72000 >>/tmp/npp-caffeinate.log 2>&1 &
echo $! >/tmp/npp-caffeinate.pid
nohup ./agents/npp-cursor-loop.sh loop >>/tmp/npp-agent-loop.log 2>&1 &
echo $! >/tmp/npp-agent-loop.pid
echo "Loop pid $(cat /tmp/npp-agent-loop.pid). Log: /tmp/npp-agent-loop.log"
tail -f /tmp/npp-agent-loop.log
