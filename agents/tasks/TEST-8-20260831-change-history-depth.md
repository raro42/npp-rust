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
