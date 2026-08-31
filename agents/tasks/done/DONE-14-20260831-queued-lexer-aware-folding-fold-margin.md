# P1: Lexer-aware folding + fold margin

## GitHub Issues
- **Issue:** https://github.com/raro42/npp-rust/issues/14
- **14**

## Problem / goal
See GitHub issue #14 and docs/gap-analysis-vs-npp.md / docs/next-gaps.md.

## High-level instructions for coder
- Keep the change small and on branch `main`.
- Run `./scripts/ci-local.sh` before push.
- Bump version + changelog when user-visible.
- Do not close the issue from the coder step.

## Privacy
- No home paths or secrets in commits or comments.

## Progress
- **2026-08-31 (coder):** Fold margin (`−`/`+` click toggle). Brace folds for Rust/C-like; indent folds for Python/others. View fold commands use the same regions. Pref `show_fold_margin`. Version **0.3.12** (commit `51c0ed5`). Docs: `docs/folding.md`. `./scripts/ci-local.sh` green. Handoff → TEST (issue left open).

## Tester (2026-08-31)
- Verified `crates/app/src/fold.rs` (brace + indent), fold margin paint/click in `ui.rs` / `ui_paint.rs`, pref `show_fold_margin`, View fold commands, `docs/folding.md` (v0.3.12 / `51c0ed5`).
- `./scripts/ci-local.sh` — **PASS** (fmt, clippy `-D warnings`, workspace tests incl. fold::*, release build).
- Result: **PASS** → `DONE-`; leave issue #14 open for handoff.

## Handoff (2026-08-31)
- User-facing notes already in `docs/changelog.md` under **[0.3.12]** (not Unreleased).
- Task goal met: lexer-aware folding + fold margin (v0.3.12, `51c0ed5`).
- Close issue #14 with `agent:done`.

Handoff: complete
