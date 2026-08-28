#!/usr/bin/env bash
# npp-rust agent loop — pick up GitHub issues with privacy gates.
# Run from repo root:
#   ./agents/npp-cursor-loop.sh once
#   ./agents/npp-cursor-loop.sh loop
#
# Env:
#   NPP_GH_REPO=raro42/npp-rust
#   AGENT_LOOP_SLEEP_MINUTES=15
#   AGENT_USE_CURSOR=1   # if cursor-agent is on PATH, run 002 after issue pickup

set -euo pipefail

SCRIPTDIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPTDIR}/.." && pwd)"
TASKDIR="${SCRIPTDIR}/tasks"
GH_REPO="${NPP_GH_REPO:-raro42/npp-rust}"
sleepminutes="${AGENT_LOOP_SLEEP_MINUTES:-15}"
sleepseconds=$((sleepminutes * 60))

cd "$REPO_ROOT"

ensure_gh_auth_env() {
  command -v gh >/dev/null 2>&1 || return 0
  if [[ -z "${GITHUB_TOKEN:-}${GH_TOKEN:-}" ]]; then
    return 0
  fi
  if gh api user -q .login >/dev/null 2>&1; then
    return 0
  fi
  echo "----- gh: invalid env token — unsetting so keyring can work" >&2
  unset GITHUB_TOKEN GH_TOKEN
}
ensure_gh_auth_env

mkdir -p "$TASKDIR" "$TASKDIR/done" "${SCRIPTDIR}/001-issue-reviewer"

sync_dev() {
  if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    git fetch origin 2>/dev/null || true
    if git show-ref --verify --quiet refs/heads/dev; then
      git checkout dev 2>/dev/null || true
      git pull --rebase --autostash origin dev 2>/dev/null || true
    fi
  fi
}

step_001_issues() {
  echo "===== 001 issue pickup ($(date -u +%Y-%m-%dT%H:%M:%SZ)) repo=$GH_REPO"
  NPP_GH_REPO="$GH_REPO" python3 "${SCRIPTDIR}/issue_checker.py"
}

step_002_coder_hint() {
  local feat
  feat="$(ls -1 "$TASKDIR"/FEAT-*.md 2>/dev/null | head -n 1 || true)"
  if [[ -z "$feat" ]]; then
    echo "----- 002: no FEAT tasks"
    return 0
  fi
  echo "----- 002: pending $(basename "$feat")"
  if [[ "${AGENT_USE_CURSOR:-0}" == "1" ]] && command -v cursor-agent >/dev/null 2>&1; then
    cursor-agent -p --force "Follow agents/002-coder.md. Implement the oldest FEAT task under agents/tasks/. Obey .cursor/rules/public-repo-no-exfiltration.mdc. Never post private data."
  else
    echo "----- 002: set AGENT_USE_CURSOR=1 and install cursor-agent to auto-run coder"
  fi
}

run_once() {
  sync_dev
  step_001_issues
  step_002_coder_hint
  echo "===== cycle done"
}

cmd="${1:-once}"
case "$cmd" in
  once) run_once ;;
  loop)
    while true; do
      run_once || true
      echo "----- sleep ${sleepminutes}m"
      sleep "$sleepseconds"
    done
    ;;
  001) sync_dev; step_001_issues ;;
  *)
    echo "usage: $0 [once|loop|001]" >&2
    exit 2
    ;;
esac
