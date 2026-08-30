#!/usr/bin/env bash
# npp-rust agent loop — CI watch → pickup → coder → tester → handoff.
# Run from repo root:
#   ./agents/npp-cursor-loop.sh once
#   ./agents/npp-cursor-loop.sh loop
#
# Env:
#   NPP_GH_REPO=raro42/npp-rust
#   AGENT_LOOP_SLEEP_MINUTES=15
#   AGENT_USE_CURSOR=1|0   # default: 1 when cursor-agent is on PATH
#   AGENT_CI_WATCH_FORCE=1 # ignore daily CI-watch stamp

set -euo pipefail

SCRIPTDIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPTDIR}/.." && pwd)"
TASKDIR="${SCRIPTDIR}/tasks"
DONEDIR="${TASKDIR}/done"
GH_REPO="${NPP_GH_REPO:-raro42/npp-rust}"
sleepminutes="${AGENT_LOOP_SLEEP_MINUTES:-15}"
sleepseconds=$((sleepminutes * 60))

# cursor-agent often lives in ~/.local/bin
export PATH="${HOME}/.local/bin:/usr/local/bin:${PATH}"

cd "$REPO_ROOT"

# Enable agents when the CLI is present, unless the user set AGENT_USE_CURSOR=0.
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

mkdir -p "$TASKDIR" "$DONEDIR" "${SCRIPTDIR}/001-issue-reviewer" "${SCRIPTDIR}/state"

run_cursor() {
  local role="$1"
  local prompt="$2"
  if [[ "${AGENT_USE_CURSOR}" != "1" ]] || ! command -v cursor-agent >/dev/null 2>&1; then
    echo "----- ${role}: cursor-agent off (AGENT_USE_CURSOR=${AGENT_USE_CURSOR})"
    return 0
  fi
  echo "----- ${role}: starting cursor-agent"
  # Keep going even if the agent exits non-zero.
  cursor-agent -p --force --trust --workspace "$REPO_ROOT" "$prompt" || {
    echo "----- ${role}: cursor-agent exited $?" >&2
    return 0
  }
}

sync_dev() {
  if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    git fetch origin 2>/dev/null || true
    if git show-ref --verify --quiet refs/heads/dev; then
      git checkout dev 2>/dev/null || true
      git pull --rebase --autostash origin dev 2>/dev/null || true
    fi
  fi
}

issue_num_from_task() {
  local base="$1"
  echo "$base" | sed -E 's/^(FEAT|WIP|TEST|DONE)-([0-9]+)-.*/\2/'
}

step_005_ci_watch() {
  echo "===== 005 CI watch ($(date -u +%Y-%m-%dT%H:%M:%SZ)) repo=$GH_REPO"
  local rc=0
  set +e
  if [[ "${AGENT_CI_WATCH_FORCE:-0}" == "1" ]]; then
    python3 "${REPO_ROOT}/scripts/ci-watch.py" --force
  else
    python3 "${REPO_ROOT}/scripts/ci-watch.py"
  fi
  rc=$?
  set -e
  # 2 = new FEAT created → kick coder prompt toward CI fix in this cycle.
  if [[ "$rc" -eq 2 ]]; then
    echo "----- 005: queued CI fix FEAT; coder will pick it up"
    run_cursor "005" \
      "Follow agents/005-ci-watcher.md and agents/002-coder.md. There is a new FEAT-ci-*-fix-github-ci.md under agents/tasks/. Fix GitHub CI (fmt/clippy/tests). Run ./scripts/ci-local.sh before push. Commit and push to origin/dev; fast-forward main if appropriate. Rename WIP to TEST when ready. Obey privacy rules."
  elif [[ "$rc" -eq 0 ]]; then
    echo "----- 005: CI watch OK or already stamped today"
  else
    echo "----- 005: CI still red but a fix task already exists (or gh error)"
  fi
  return 0
}

step_001_issues() {
  echo "===== 001 issue pickup ($(date -u +%Y-%m-%dT%H:%M:%SZ)) repo=$GH_REPO"
  NPP_GH_REPO="$GH_REPO" python3 "${SCRIPTDIR}/issue_checker.py"
}

step_002_coder() {
  local feat wip task base issue_n
  feat="$(ls -1 "$TASKDIR"/FEAT-*.md 2>/dev/null | head -n 1 || true)"
  wip="$(ls -1 "$TASKDIR"/WIP-*.md 2>/dev/null | head -n 1 || true)"
  task="${feat:-$wip}"
  if [[ -z "$task" ]]; then
    echo "----- 002: no FEAT or WIP tasks"
    return 0
  fi

  base="$(basename "$task")"
  if [[ "$base" == FEAT-* ]]; then
    local wip_name="${base/FEAT-/WIP-}"
    mv "$task" "$TASKDIR/$wip_name"
    task="$TASKDIR/$wip_name"
    base="$wip_name"
    echo "----- 002: promoted to $(basename "$task")"
  fi
  issue_n="$(issue_num_from_task "$base")"
  if [[ -n "$issue_n" ]] && [[ "$issue_n" =~ ^[0-9]+$ ]]; then
    echo "----- 002: GitHub issue #${issue_n} → agent:wip"
    gh issue edit "$issue_n" --repo "$GH_REPO" --add-label "agent:wip" 2>/dev/null || true
    gh issue edit "$issue_n" --repo "$GH_REPO" --remove-label "agent:planned" 2>/dev/null || true
    ./scripts/gh-safe.sh issue comment "$issue_n" --body "Agent 002: coding (\`$(basename "$task")\`). Will hand to tester as TEST- when the batch is ready." 2>/dev/null || true
  fi

  echo "----- 002: pending $(basename "$task")"
  run_cursor "002" \
    "Follow agents/002-coder.md. Implement the oldest FEAT or WIP under agents/tasks/. Prefer real behaviour for Placeholder items in docs/menu-todo.md (not status-only fakes). Before push: cargo fmt --all, cargo clippy --workspace --all-targets -- -D warnings, cargo check -p app. Commit and push to origin/dev. When the batch is ready, rename WIP- to TEST- (do not close the issue, do not move to done/). Obey .cursor/rules/public-repo-no-exfiltration.mdc. Never post private data."
}

step_003_tester() {
  local task base issue_n
  task="$(ls -1 "$TASKDIR"/TEST-*.md 2>/dev/null | head -n 1 || true)"
  if [[ -z "$task" ]]; then
    echo "----- 003: no TEST tasks"
    return 0
  fi
  base="$(basename "$task")"
  issue_n="$(issue_num_from_task "$base")"
  echo "----- 003: testing $(basename "$task")"
  if [[ -n "$issue_n" ]] && [[ "$issue_n" =~ ^[0-9]+$ ]]; then
    ./scripts/gh-safe.sh issue comment "$issue_n" --body "Agent 003: testing (\`$(basename "$task")\`)." 2>/dev/null || true
  fi
  run_cursor "003" \
    "Follow agents/003-tester.md. Test the oldest TEST- task under agents/tasks/. Run cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, and cargo test --workspace (or ./scripts/ci-local.sh). On pass: rename to DONE- and move under agents/tasks/done/. On fail: rename back to WIP- with notes. Do not close the GitHub issue (handoff does that). Obey privacy rules. Commit and push any task-file updates."
}

step_004_handoff() {
  local task base issue_n
  # Oldest DONE without handoff complete marker.
  task=""
  local f
  for f in $(ls -1 "$DONEDIR"/DONE-*.md 2>/dev/null || true); do
    if ! grep -q '^Handoff: complete' "$f" 2>/dev/null; then
      task="$f"
      break
    fi
  done
  if [[ -z "$task" ]]; then
    echo "----- 004: no DONE tasks awaiting handoff"
    return 0
  fi
  base="$(basename "$task")"
  issue_n="$(issue_num_from_task "$base")"
  echo "----- 004: handoff $(basename "$task")"
  if [[ -n "$issue_n" ]] && [[ "$issue_n" =~ ^[0-9]+$ ]]; then
    ./scripts/gh-safe.sh issue comment "$issue_n" --body "Agent 004: handoff review + changelog (\`$(basename "$task")\`)." 2>/dev/null || true
  fi
  run_cursor "004" \
    "Follow agents/004-handoff.md. Review the oldest agents/tasks/done/DONE-*.md without 'Handoff: complete'. Update docs/changelog.md [Unreleased], commit and push, then close the GitHub issue with agent:done if the goal is met. Append 'Handoff: complete' to the task file. Obey privacy rules."
}

run_once() {
  echo "===== cycle start ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
  sync_dev
  step_005_ci_watch
  step_001_issues
  # Prefer finishing the pipeline: handoff → test → code (so work does not pile up untested).
  step_004_handoff
  step_003_tester
  step_002_coder
  # If coder just created a TEST-, test and hand off in the same cycle.
  step_003_tester
  step_004_handoff
  echo "===== cycle done ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
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
  002) sync_dev; step_002_coder ;;
  003) sync_dev; step_003_tester ;;
  004) sync_dev; step_004_handoff ;;
  005) sync_dev; step_005_ci_watch ;;
  *)
    echo "usage: $0 [once|loop|001|002|003|004|005]" >&2
    exit 2
    ;;
esac
