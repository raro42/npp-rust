# Change history depth (issue #8 P1)

## Goal
Make change-history marks closer to Scintilla / Notepad++ block marks. Ship on `main`.

## Scope (this batch)
1. Read current MVP: amber unsaved / green saved ticks, line remap, clear + next/prev search (`docs/whats-missing.md`, `crates/doc`, `ui_paint`).
2. Deepen behaviour in a useful way, for example:
   - clearer gutter block / margin paint (not only a thin tick)
   - better remap across undo/redo and large edits
   - optional “modified since save” vs “modified since open” clarity in status or docs
3. Keep Search → Clear Change History and jump-to-mark working.
4. Tests where practical. `./scripts/ci-local.sh` before push.
5. Bump version + `docs/changelog.md` when user-visible.

## Out of scope for this batch
- UTF-16 LE/BE
- Full stylers.xml theme parity
- Clipboard history panel

## References
- Issue: https://github.com/raro42/npp-rust/issues/8
- `docs/next-gaps.md` (P1 Change history)
- Prior themes handoff left this item next

## Privacy
No secrets or home paths in commits or issue comments.

## Progress
- Full-height gutter bars (joined SC_MARK_BAR-style) + soft line wash; dual-view pane marks
- Buffer undo/redo emits `LineStructureEdit`; editor skips caret re-stamp on undo/redo
- Status `CHG u/s`; Clear/jump name unsaved vs saved; Summary shows counts
- Version **0.3.5**; `./scripts/ci-local.sh` passed
- Handoff: ready for 003 tester (do not close #8 — UTF-16 / full Scintilla reverted markers remain)

## Tester (2026-08-31)
- Verified: `ui_paint` bar/join/wash; `editor` undo remap test; status `CHG u/s`; version **0.3.5** + changelog.
- `./scripts/ci-local.sh` — **PASS** (fmt, clippy `-D warnings`, workspace tests, release build).
- Result: **PASS** → `DONE-`; leave issue #8 open for handoff (UTF-16 / Scintilla reverted markers still open).

## Handoff (2026-08-31)
- User-facing notes already in `docs/changelog.md` under **[0.3.5]** (not Unreleased).
- `docs/next-gaps.md` already lists v0.3.5 change-history ship.
- Task goal (change-history depth this batch) met. Issue #8 stays open: UTF-16 + Scintilla reverted markers / full stylers.xml remain.
- Do not close #8 until remaining P1 items ship (or a new issue tracks them).
- Batch handoff finished; do not re-pick this DONE file.

Handoff: deferred
Handoff: complete
