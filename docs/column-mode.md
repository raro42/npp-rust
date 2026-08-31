# Column / rectangular select

Date: 2026-08-31  
Version: 0.3.9

## What works

- **Alt+drag** (Option+drag on macOS) builds a rectangular selection.
- Each line in the block gets a range at the same columns (clamped to line length).
- **Typing**, **Backspace**, **Delete**, **Paste**, **Enter**, and **Tab** apply to every multi-caret / rect range (one undo).
- **Copy** / **Cut** with two or more ranges joins line slices with newlines.
- **Multi-select next/all** carets also receive typing (same path).
- **Column Editor** (`IDM_EDIT_COLUMNMODE`) still inserts clipboard text or `0,1,2…` at the caret column.

## Differs from Notepad++ / Scintilla

- No **virtual space** past the end of a short line (carets clamp to EOL).
- No click-to-add extra carets beyond multi-select commands and Alt+drag.
- Arrow keys clear multi-carets (they do not move all carets together).
- Paste always inserts the same clipboard string at each caret (no per-line column paste split beyond what you copied).

## Code

- Buffer: `TextBuffer::rect_ranges` — `crates/buffer/src/lib.rs`
- Document: `set_rect_selection`, `insert_multi`, `delete_*_multi` — `crates/doc/src/lib.rs`
- UI: Alt+drag + input path — `crates/app/src/ui.rs`
