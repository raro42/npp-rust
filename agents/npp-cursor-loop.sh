#!/usr/bin/env bash
# npp-rust agent loop — CI → logs → quality → flush → pickup → code → test → handoff.
# Run from repo root:
#   ./agents/npp-cursor-loop.sh once
#   ./agents/npp-cursor-loop.sh loop
#
# Env:
#   NPP_GH_REPO=raro42/npp-rust
#   AGENT_LOOP_SLEEP_MINUTES=15
#   AGENT_USE_CURSOR=1|0
#   AGENT_CI_WATCH_FORCE=1
#   AGENT_QUALITY_FORCE=1
#   AGENT_GIT_FLUSH_FORCE=1

set -euo pipefail

SCRIPTDIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPTDIR}/.." && pwd)"
TASKDIR="${SCRIPTDIR}/tasks"
DONEDIR="${TASKDIR}/done"
STATEDIR="${SCRIPTDIR}/state"
GH_REPO="${NPP_GH_REPO:-raro42/npp-rust}"
sleepminutes="${AGENT_LOOP_SLEEP_MINUTES:-15}"
sleepseconds=$((sleepminutes * 60))
AGENT_LOCK="${STATEDIR}/agent.pid"

export PATH="${HOME}/.local/bin:/usr/local/bin:${PATH}"
cd "$REPO_ROOT"

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

mkdir -p "$TASKDIR" "$DONEDIR" "${SCRIPTDIR}/001-issue-reviewer" "$STATEDIR" \
  "${SCRIPTDIR}/workspace" "${SCRIPTDIR}/006-log-monitor" "${SCRIPTDIR}/007-quality"

tick() {
  echo "AGENT_LOOP_TICK {\"at\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",\"msg\":\"$*\"}"
}

acquire_agent_lock() {
  if [[ -f "$AGENT_LOCK" ]]; then
    local old
    old="$(cat "$AGENT_LOCK" 2>/dev/null || true)"
    if [[ -n "$old" ]] && kill -0 "$old" 2>/dev/null; then
      echo "----- agent lock held by pid $old — skip cursor spawn"
      return 1
    fi
  fi
  echo $$ >"$AGENT_LOCK"
  return 0
}

release_agent_lock() {
  rm -f "$AGENT_LOCK"
}

run_cursor() {
  local role="$1"
  local prompt="$2"
  if [[ "${AGENT_USE_CURSOR}" != "1" ]] || ! command -v cursor-agent >/dev/null 2>&1; then
    echo "----- ${role}: cursor-agent off (AGENT_USE_CURSOR=${AGENT_USE_CURSOR})"
    return 0
  fi
  if ! acquire_agent_lock; then
    return 0
  fi
  echo "----- ${role}: starting cursor-agent"
  cursor-agent -p --force --trust --workspace "$REPO_ROOT" "$prompt" || {
    echo "----- ${role}: cursor-agent exited $?" >&2
  }
  release_agent_lock
  return 0
}

sync_main() {
  if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    git fetch origin 2>/dev/null || true
    if git show-ref --verify --quiet refs/heads/main; then
      git checkout main 2>/dev/null || true
      git pull --rebase --autostash origin main 2>/dev/null || true
    fi
  fi
}

issue_num_from_task() {
  local base="$1"
  echo "$base" | sed -E 's/^(FEAT|WIP|TEST|DONE)-([0-9]+)-.*/\2/'
}

stamp_day_file() {
  local name="$1"
  echo "$(date -u +%Y-%m-%d)" >"${STATEDIR}/${name}"
}

stamp_is_today() {
  local name="$1"
  local f="${STATEDIR}/${name}"
  [[ -f "$f" ]] && [[ "$(cat "$f" 2>/dev/null | head -c 10)" == "$(date -u +%Y-%m-%d)" ]]
}

stamp_week_file() {
  local name="$1"
  echo "$(date -u +%Y-W%V)" >"${STATEDIR}/${name}"
}

stamp_is_this_week() {
  local name="$1"
  local f="${STATEDIR}/${name}"
  [[ -f "$f" ]] && [[ "$(cat "$f" 2>/dev/null)" == "$(date -u +%Y-W%V)" ]]
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
  if [[ "$rc" -eq 2 ]]; then
    echo "----- 005: queued CI fix FEAT"
    run_cursor "005" \
      "Follow agents/005-ci-watcher.md and agents/002-coder.md. Read agents/workspace/lessons.md. Fix GitHub CI using ./scripts/ci-local.sh. Commit and push origin/main. Rename WIP to TEST when ready. Obey privacy rules."
  elif [[ "$rc" -eq 0 ]]; then
    echo "----- 005: CI watch OK or already stamped today"
  else
    echo "----- 005: CI red but task exists (or gh error)"
  fi
  return 0
}

step_006_log_monitor() {
  echo "===== 006 log monitor ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
  local rc=0
  set +e
  python3 "${REPO_ROOT}/scripts/scan_panic_log.py" --write-finding
  rc=$?
  set -e
  if [[ "$rc" -eq 2 ]]; then
    echo "----- 006: queued panic FEAT"
    run_cursor "006" \
      "Follow agents/006-log-monitor/PROMPT.md and agents/002-coder.md. A FEAT-log-*-panic.md was created. Investigate without putting home paths in commits. Run ./scripts/ci-local.sh before push."
  elif [[ "$rc" -eq 0 ]]; then
    echo "----- 006: no new panic signatures (or no log)"
  else
    echo "----- 006: new signatures seen (no write) or scan soft-fail"
  fi
  return 0
}

step_007_quality() {
  echo "===== 007 quality ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
  if [[ "${AGENT_QUALITY_FORCE:-0}" != "1" ]] && stamp_is_this_week "quality.stamp"; then
    echo "----- 007: already ran this UTC week"
    return 0
  fi
  local rc=0
  set +e
  python3 "${REPO_ROOT}/scripts/scan_repo_quality.py"
  rc=$?
  set -e
  stamp_week_file "quality.stamp"
  if [[ "$rc" -ne 0 ]]; then
    echo "----- 007: quality fails — spawn fixer"
    run_cursor "007" \
      "Follow agents/007-quality/PROMPT.md. Run python3 scripts/scan_repo_quality.py, fix fails, commit and push origin/main. Read agents/workspace/lessons.md. Obey privacy rules."
  else
    echo "----- 007: quality OK"
  fi
  return 0
}

step_008_git_flush() {
  echo "===== 008 git flush ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
  if [[ "${AGENT_GIT_FLUSH_FORCE:-0}" != "1" ]] && stamp_is_today "git-flush.stamp"; then
    echo "----- 008: already flushed today"
    return 0
  fi
  set +e
  python3 "${REPO_ROOT}/scripts/git_flush.py"
  set -e
  stamp_day_file "git-flush.stamp"
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
    "Follow agents/002-coder.md. Read agents/workspace/lessons.md and agents/workspace/todo.md first. Implement the oldest FEAT or WIP under agents/tasks/. Before push run ./scripts/ci-local.sh (fmt+clippy+test). Commit and push origin/main. Rename WIP- to TEST- when ready. Do not close the issue. Obey privacy rules."
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
    "Follow agents/003-tester.md. Read agents/workspace/lessons.md. Test the oldest TEST- task. Prefer ./scripts/ci-local.sh. On pass: DONE under agents/tasks/done/. On fail: back to WIP- with notes. Do not close the issue. Obey privacy rules."
}

step_004_handoff() {
  local task base issue_n
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
    "Follow agents/004-handoff.md. Review oldest DONE without Handoff: complete. Update docs/changelog.md, commit push origin/main, close issue with agent:done when met. Append Handoff: complete. Obey privacy rules."
}

run_once() {
  tick "cycle_start"
  echo "===== cycle start ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
  sync_main
  step_005_ci_watch
  step_006_log_monitor
  step_007_quality
  step_008_git_flush
  step_001_issues
  step_004_handoff
  step_003_tester
  step_002_coder
  step_003_tester
  step_004_handoff
  echo "===== cycle done ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
  tick "cycle_done"
}

cmd="${1:-once}"
case "$cmd" in
  once) run_once ;;
  loop)
    echo "===== npp loop start AGENT_USE_CURSOR=${AGENT_USE_CURSOR} sleep=${sleepminutes}m"
    while true; do
      run_once || true
      echo "AGENT_LOOP_SLEEP {\"minutes\":${sleepminutes}}"
      echo "----- sleep ${sleepminutes}m"
      sleep "$sleepseconds"
    done
    ;;
  001) sync_main; step_001_issues ;;
  002) sync_main; step_002_coder ;;
  003) sync_main; step_003_tester ;;
  004) sync_main; step_004_handoff ;;
  005) sync_main; step_005_ci_watch ;;
  006) sync_main; step_006_log_monitor ;;
  007) sync_main; step_007_quality ;;
  008) sync_main; AGENT_GIT_FLUSH_FORCE=1 step_008_git_flush ;;
  *)
    echo "usage: $0 [once|loop|001|002|003|004|005|006|007|008]" >&2
    exit 2
    ;;
esac
