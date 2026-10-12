# Column / rectangular select

Date: 2026-10-12  
Version: 0.3.246

## What works

- **Alt+drag** (Option+drag on macOS) builds a rectangular selection.
- Each line in the block gets a range at the same columns (char indices clamp to line length).
- **Virtual space past EOL** for Alt+drag: mouse hit-test can target columns past every line’s EOL (space width); carets and the selection wash draw at those columns; typing pads short lines with spaces to the target column; copy pads missing columns with spaces in the clipboard.
- **Plain (non-Alt) free caret past EOL**: click or Right past EOL keeps a virtual column (`Document::virtual_caret_col`); caret paint uses it; typing / paste / Tab pad with spaces; Backspace / Left shrink virtual space without eating the newline; Up/Down keep the preferred column when the next line is shorter.
- **Typing**, **Backspace**, **Delete**, **Paste**, **Enter**, and **Tab** apply to every multi-caret / rect range (one undo). Successive typing at the same carets coalesces into one undo unit (1s window), like plain typing.
- **Copy** / **Cut** with two or more ranges joins line slices with newlines (rect copy includes virtual-space spaces).
- **Multi-select next/all** carets also receive typing (same path). Multi-select Next is remappable (`shortcut_multi_select_next`, default Cmd+Alt+N; Shift skips current).
- **Column Editor** (`IDM_EDIT_COLUMNMODE`) still inserts clipboard text or `0,1,2…` at the caret column.

## Differs from Notepad++ / Scintilla

- Plain Shift+drag selection still clamps to the line body (virtual extend for stream selection is later).
- No click-to-add extra carets beyond multi-select commands and Alt+drag.
- Arrow keys clear multi-carets (they do not move all carets together).
- Paste always inserts the same clipboard string at each caret (no per-line column paste split beyond what you copied).

## Code

- Buffer: `TextBuffer::rect_ranges` / `rect_ranges_from_cols` — `crates/buffer/src/lib.rs`
- Document: `set_rect_selection` / `set_rect_selection_cols`, `set_caret_at_line_col`, `insert_plain`, `move_char`, `insert_multi`, `delete_*_multi` — `crates/doc/src/lib.rs`
- Hit-test / paint: `col_from_x_virtual`, `x_for_col` — `crates/app/src/ui_paint.rs`; Alt+drag + plain click path — `crates/app/src/ui.rs`
