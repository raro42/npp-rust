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
