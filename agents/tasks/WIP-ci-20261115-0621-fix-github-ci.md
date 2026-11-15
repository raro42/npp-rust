# Fix failing GitHub CI

## Goal
Make `.github/workflows/ci.yml` green on `main`.

## Local gates (must pass before push)
- `./scripts/ci-local.sh`

## Recent failures (sanitized)
- run `43684352554`: CI
  - inspect: `gh run view 43684352554 --log-failed` (redact private paths)
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

Created: 2026-11-15T06:21:35Z

## Progress
- Root cause: Windows `cargo fmt` stack-overflows on large `crates/app/src/ui.rs` (rustfmt main thread ~1 MiB stack). Ubuntu + macOS were green on run `43684352554`.
- `ci.yml` Windows skip could not be pushed (OAuth token lacks `workflow` scope). `rustfmt.toml` `ignore` is nightly-only on stable.
- Fix: split Function List classifiers into `crates/app/src/func_like.rs` + `func_like_line_tests.rs` so each file stays under the Windows rustfmt stack limit.
