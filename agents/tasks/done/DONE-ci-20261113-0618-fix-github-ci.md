# Fix failing GitHub CI

## Goal
Make `.github/workflows/ci.yml` green on `main`.

## Local gates (must pass before push)
- `./scripts/ci-local.sh`

## Recent failures (sanitized)
- run `43377241030`: CI
  - inspect: `gh run view 43377241030 --log-failed` (redact private paths)

## Steps
1. Reproduce with `./scripts/ci-local.sh` on `main`.
2. Fix fmt/clippy/tests.
3. Commit and push `main`.
4. Trigger CI if needed: `gh workflow run ci.yml --ref main`
5. Confirm green: `gh run list --workflow=ci.yml --limit 3`

## Privacy
- No home paths, secrets, or emails in commits or task notes.

Created: 2026-11-13T06:18:03Z

## Progress
- Cause: `clippy::needless_bool` in `crates/app/src/ui.rs` (4 sites) failed Clippy on ubuntu/macos/windows for run `43377241030`.
- Fix: collapse `if … { return false; } true` into direct bool expressions.
- Local: `./scripts/ci-local.sh` OK.
- Commit `8af7e799` on `main`; dispatched run `43378591376`.

## Tester (2026-11-13)
- Verified commit `8af7e799`: four `needless_bool` sites in `crates/app/src/ui.rs` collapsed to direct bool expressions.
- `./scripts/ci-local.sh` passed (fmt, clippy `-D warnings`, workspace tests, release build).
- Cloud: run `43378591376` success (ubuntu/macos/windows).
- No GitHub issue on this CI watch task (skip `gh-safe` comment).
- Result: **pass**. Moving to `agents/tasks/done/DONE-ci-…`. Leave open for handoff.

## Handoff (2026-11-13)
- Goal met: `ci.yml` on `main` green again (run `43378591376`, ubuntu/macos/windows). Clippy `needless_bool` in `crates/app/src/ui.rs` fixed in commit `8af7e799`.
- No user-facing product change; left `docs/changelog.md` **[Unreleased]** empty.
- No GitHub issue on this CI watch task (nothing to close via `gh-safe`).

Handoff: complete
GitHub: closed
