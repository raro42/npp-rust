# Real menu placeholders

## GitHub Issues
- **Issue:** https://github.com/raro42/npp-rust/issues/1
- **1**

## Problem / goal
Implement real behaviour for Placeholder / status-only items in docs/menu-todo.md.
Do not mark is_implemented for status-bar-only fakes.
Prefer batches: Search style jumps that already store style slots, View fold where cheap, Settings dialogs.
Commit and push to origin/dev after each batch. Update docs/menu-todo.md.

## Progress (2026-08-29)
- Paint fold/hide (`hidden_lines`), style-mark washes, bookmark gutter ticks.
- Search style mark / jump / clear / copy-styled; DEF find-mark jumps bookmarks.
- Edit Cut/Copy/Paste use the session clipboard.
- Placeholder count: **141 → 35** (`docs/menu-todo.md`).
- Commit: `b5913d5` — Paint fold and style marks; clear many menu placeholders.
- `cargo check -p app` pass; `ui_paint` unit tests pass.
- Hand off to tester as `TEST-`.

## Privacy
- Do not paste private paths into GitHub.
