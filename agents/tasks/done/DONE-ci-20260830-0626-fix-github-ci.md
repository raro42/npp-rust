# Fix failing GitHub CI

## Goal
Make `.github/workflows/ci.yml` green on `main`.

## Local gates (must pass before push)
- `./scripts/ci-local.sh`
- or: `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

## Recent failures (sanitized)
- branch `main` run `33296611755`: Require local fmt/clippy before push so CI stays green.
  - inspect: `gh run view 33296611755 --log-failed` (redact private paths)
- branch `dev` run `33296610777`: Require local fmt/clippy before push so CI stays green.
  - inspect: `gh run view 33296610777 --log-failed` (redact private paths)
- branch `main` run `33296558068`: Fix rustfmt so CI format check passes.
  - inspect: `gh run view 33296558068 --log-failed` (redact private paths)
- branch `dev` run `33296556931`: Fix rustfmt so CI format check passes.
  - inspect: `gh run view 33296556931 --log-failed` (redact private paths)
- branch `main` run `33271870588`: Docs: refresh next-gaps after v0.3.2 partial P1.
  - inspect: `gh run view 33271870588 --log-failed` (redact private paths)

## Steps
1. Reproduce with `./scripts/ci-local.sh` on branch `main`.
2. Fix fmt/clippy/tests.
3. Commit and push `main`.
4. Confirm a new CI run succeeds.

## Privacy
- No home paths, secrets, or emails in commits or task notes.

Created: 2026-08-30T06:26:47Z

Handoff: complete
Note: CI on main is green; stale watch task retired so P1 themes can run.
