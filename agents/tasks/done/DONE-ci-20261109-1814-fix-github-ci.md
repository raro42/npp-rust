# Fix failing GitHub CI

## Goal
Make `.github/workflows/ci.yml` green on `main`.

## Local gates (must pass before push)
- `./scripts/ci-local.sh`

## Recent failures (sanitized)
- run `42671695599`: CI
  - inspect: `gh run view 42671695599 --log-failed` (redact private paths)

## Steps
1. Reproduce with `./scripts/ci-local.sh` on `main`.
2. Fix fmt/clippy/tests.
3. Commit and push `main`.
4. Trigger CI if needed: `gh workflow run ci.yml --ref main`
5. Confirm green: `gh run list --workflow=ci.yml --limit 3`

## Privacy
- No home paths, secrets, or emails in commits or task notes.

Created: 2026-11-09T18:14:18Z

## Progress
- Local `./scripts/ci-local.sh`: green (fmt, clippy -D warnings, tests, release build).
- Failed run `42671695599` (schedule): no jobs created; GitHub UI showed Internal server error (startup failure), same head SHA that passed the morning schedule.
- No Rust/workflow YAML change required; re-dispatching `ci.yml` on `main` to confirm cloud green.
- Handoff: ready for tester after cloud re-run.


## Progress (update)
- Commit `f376f69` pushed; dispatched run `42672908737`.
- Ubuntu + macOS green; Windows matrix leg never started (Internal server error).
- githubstatus.com: Actions **major_outage** / Partial System Outage.
- Blocker: GitHub Actions platform outage (not local gates). Re-ran `42672908737`; waiting for recovery.
- Left as WIP until a full 3-OS cloud run is green.


## Progress (green)
- Re-run `42672908737` completed success: ubuntu, macos, windows all green.
- Root cause was GitHub Actions outage (incident with Actions/Checks API), not project code.
- Handoff: TEST ready for tester.

## Tester
- Verified claim: latest finished `ci.yml` run `42672908737` success (ubuntu/macos/windows); `agents/workspace/ci-status.md` GREEN. Prior schedule fail `42671695599` was Actions startup outage, not project code.
- `./scripts/ci-local.sh` passed (fmt, clippy `-D warnings`, workspace tests, release build).
- No GitHub issue on this CI watch task (skip `gh-safe` comment).
- Result: **pass**. Moving to `agents/tasks/done/DONE-ci-…`. Leave open for handoff.

## Handoff (2026-11-09)
- Goal met: `ci.yml` on `main` green again (run `42672908737`, ubuntu/macos/windows). Failure was GitHub Actions outage, not project code. Commit `7b5b65b`.
- No user-facing product change; left `docs/changelog.md` **[Unreleased]** empty.
- No GitHub issue on this CI watch task (nothing to close via `gh-safe`).

Handoff: complete
GitHub: closed
