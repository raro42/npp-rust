# P0: Column/rect select + multi-caret typing

## GitHub Issues
- **Issue:** https://github.com/raro42/npp-rust/issues/11
- **11**

## Problem / goal
See GitHub issue #11 and docs/gap-analysis-vs-npp.md / docs/next-gaps.md.

## High-level instructions for coder
- Keep the change small and on branch `main`.
- Run `./scripts/ci-local.sh` before push.
- Bump version + changelog when user-visible.
- Do not close the issue from the coder step.

## Privacy
- No home paths or secrets in commits or comments.

## Progress
- **Coder (v0.3.9):** Alt+drag rect select → `multi_sels`; typing/Backspace/Delete/Paste/Enter/Tab via `insert_multi` / `delete_*_multi`. Column Editor kept. Docs: `docs/column-mode.md`. Commit `569747b`. `./scripts/ci-local.sh` OK. Hand off to tester — do not close #11.

## Tester (2026-08-31)
- Verified: Alt+drag → `set_rect_selection` / `multi_sels` in `crates/app/src/ui.rs` + `crates/doc/src/lib.rs`; `insert_multi` / `delete_*_multi`; buffer `rect_ranges` tests; docs `docs/column-mode.md`; version **0.3.9**; commit `569747b`.
- `./scripts/ci-local.sh` — **PASS** (fmt, clippy `-D warnings`, workspace tests incl. rect_ranges_*, insert_multi_*, delete_backward_multi_*, release build).
- Result: **PASS** → `DONE-`; leave issue #11 open for handoff.
