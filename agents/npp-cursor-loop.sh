#!/usr/bin/env bash
# npp-rust agent loop — pick up GitHub issues with privacy gates.
# Run from repo root:
#   ./agents/npp-cursor-loop.sh once
#   ./agents/npp-cursor-loop.sh loop
#
# Env:
#   NPP_GH_REPO=raro42/npp-rust
#   AGENT_LOOP_SLEEP_MINUTES=15
#   AGENT_USE_CURSOR=1|0   # default: 1 when cursor-agent is on PATH

set -euo pipefail

SCRIPTDIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPTDIR}/.." && pwd)"
TASKDIR="${SCRIPTDIR}/tasks"
GH_REPO="${NPP_GH_REPO:-raro42/npp-rust}"
sleepminutes="${AGENT_LOOP_SLEEP_MINUTES:-15}"
sleepseconds=$((sleepminutes * 60))

# cursor-agent often lives in ~/.local/bin
export PATH="${HOME}/.local/bin:/usr/local/bin:${PATH}"

cd "$REPO_ROOT"

# Enable auto-coder when the CLI is present, unless the user set AGENT_USE_CURSOR=0.
if [[ -z "${AGENT_USE_CURSOR+x}" ]]; then
  if command -v cursor-agent >/dev/null 2>&1; then
    AGENT_USE_CURSOR=1
  else
    AGENT_USE_CURSOR=0
  fi
fi

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

step_002_coder() {
  local feat wip
  feat="$(ls -1 "$TASKDIR"/FEAT-*.md 2>/dev/null | head -n 1 || true)"
  wip="$(ls -1 "$TASKDIR"/WIP-*.md 2>/dev/null | head -n 1 || true)"
  local task="${feat:-$wip}"
  if [[ -z "$task" ]]; then
    echo "----- 002: no FEAT or WIP tasks"
    return 0
  fi

  # Promote FEAT → WIP and mark GitHub issue as in progress.
  local base issue_n
  base="$(basename "$task")"
  if [[ "$base" == FEAT-* ]]; then
    local wip_name="${base/FEAT-/WIP-}"
    mv "$task" "$TASKDIR/$wip_name"
    task="$TASKDIR/$wip_name"
    base="$wip_name"
    echo "----- 002: promoted to $(basename "$task")"
  fi
  issue_n="$(echo "$base" | sed -E 's/^(FEAT|WIP|TEST)-([0-9]+)-.*/\2/')"
  if [[ -n "$issue_n" ]]; then
    echo "----- 002: GitHub issue #${issue_n} → agent:wip"
    gh issue edit "$issue_n" --repo "$GH_REPO" --add-label "agent:wip" 2>/dev/null || true
    gh issue edit "$issue_n" --repo "$GH_REPO" --remove-label "agent:planned" 2>/dev/null || true
    ./scripts/gh-safe.sh issue comment "$issue_n" --body "Agent 002: work in progress (\`$(basename "$task")\`)." 2>/dev/null || true
  fi

  echo "----- 002: pending $(basename "$task") (AGENT_USE_CURSOR=${AGENT_USE_CURSOR})"
  if [[ "${AGENT_USE_CURSOR}" == "1" ]] && command -v cursor-agent >/dev/null 2>&1; then
    echo "----- 002: starting cursor-agent coder"
    cursor-agent -p --force --trust --workspace "$REPO_ROOT" \
      "Follow agents/002-coder.md. Implement the oldest FEAT or WIP task under agents/tasks/. Prefer clearing menu stubs in docs/menu-todo.md for issue #1. Obey .cursor/rules/public-repo-no-exfiltration.mdc. Never post private data. Do not push unless the task says so."
  else
    echo "----- 002: coder off — set AGENT_USE_CURSOR=1 and ensure cursor-agent is on PATH"
  fi
}

run_once() {
  sync_dev
  step_001_issues
  step_002_coder
  echo "===== cycle done"
}

cmd="${1:-once}"
case "$cmd" in
  once) run_once ;;
  loop)
    echo "===== npp loop start AGENT_USE_CURSOR=${AGENT_USE_CURSOR} sleep=${sleepminutes}m"
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
