# P0: File drop + selection drag move/copy

## GitHub Issues
- **Issue:** https://github.com/raro42/npp-rust/issues/10
- **10**

## Problem / goal
See GitHub issue #10 and docs/gap-analysis-vs-npp.md / docs/next-gaps.md.

## High-level instructions for coder
- Keep the change small and on branch `main`.
- Run `./scripts/ci-local.sh` before push.
- Bump version + changelog when user-visible.
- Do not close the issue from the coder step.

## Privacy
- No home paths or secrets in commits or comments.

## Progress
- **Coder (v0.3.8):** File drop opens paths; selection drag move / Ctrl|Cmd+copy via `TextBuffer::drag_selection_to`. Docs: `docs/drag-drop.md`. Commit `4f79398`. `./scripts/ci-local.sh` OK. Hand off to tester — do not close #10.

## Tester (2026-08-31)
- Verified: `TextBuffer::drag_selection_to` + buffer unit tests in `crates/buffer/src/lib.rs`; `handle_file_drops` / selection drag UI in `crates/app/src/ui.rs`; docs `docs/drag-drop.md`; version **0.3.8**; commit `4f79398`.
- `./scripts/ci-local.sh` — **PASS** (fmt, clippy `-D warnings`, workspace tests incl. drag_selection_*, release build).
- Result: **PASS** → `DONE-`; leave issue #10 open for handoff.
