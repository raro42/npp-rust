# Column / rectangular select

Date: 2026-10-12  
Version: 0.3.251

## What works

- **Alt+drag** (Option+drag on macOS) builds a rectangular selection.
- Each line in the block gets a range at the same columns (char indices clamp to line length).
- **Virtual space past EOL** for Alt+drag: mouse hit-test can target columns past every line’s EOL (space width); carets and the selection wash draw at those columns; typing pads short lines with spaces to the target column; copy pads missing columns with spaces in the clipboard.
- **Plain (non-Alt) free caret past EOL**: click or Right past EOL keeps a virtual column (`Document::virtual_caret_col`); caret paint uses it; typing / paste / Tab pad with spaces; Backspace / Left shrink virtual space without eating the newline; Up/Down keep the preferred column when the next line is shorter.
- **Stream select into virtual space**: Shift+click / drag (and plain drag) past EOL extends the selection wash and caret via `set_stream_selection_to_line_col`; Shift+Right / Shift+Left grow or shrink virtual space at EOL the same way. Buffer selection still clamps to the line body; copy/cut use real characters only.
- **Ctrl/Cmd+click add carets**: adds a zero-width caret at the click (seeds the previous caret/selection); click the same caret again to remove it. Columns past EOL clamp to the line body for click-added carets.
- **Arrow keys move all multi-carets** (plain Left/Right/Up/Down, no modifiers). Non-empty ranges collapse to an edge first; rect selections keep `rect_cols` (including virtual space).
- **Shift+arrows extend stream multi-carets** (Shift+Left/Right/Up/Down). Each range keeps its anchor and moves the caret end; typing replaces the extended ranges. Word-jump / document jumps still clear multi-carets.
- **Shift+arrows extend rect selections** (Shift+Left/Right/Up/Down). Keeps the Alt+drag anchor corner (`rect_anchor`) and moves the opposite caret corner (columns may be past EOL). Collapsing to one line becomes a stream selection of that column span.
- **Typing**, **Backspace**, **Delete**, **Paste**, **Enter**, and **Tab** apply to every multi-caret / rect range (one undo). Successive typing at the same carets coalesces into one undo unit (1s window), like plain typing.
- **Copy** / **Cut** with two or more ranges joins line slices with newlines (rect copy includes virtual-space spaces).
- **Multi-select next/all** carets also receive typing (same path). Multi-select Next is remappable (`shortcut_multi_select_next`, default Cmd+Alt+N; Shift skips current).
- **Column Editor** (`IDM_EDIT_COLUMNMODE`) still inserts clipboard text or `0,1,2…` at the caret column.

## Differs from Notepad++ / Scintilla

- Stream virtual select is caret-end only (no separate virtual anchor column); empty EOL→virtual-only selections stay caret paint without a wash.
- Rect Shift+extend tracks one anchor corner; after a plain arrow move the anchor resets to the top/start corner of the new block.
- Paste always inserts the same clipboard string at each caret (no per-line column paste split beyond what you copied).

## Code

- Buffer: `TextBuffer::rect_ranges` / `rect_ranges_from_cols`, `anchor_and_caret` — `crates/buffer/src/lib.rs`
- Document: `set_rect_selection` / `set_rect_selection_cols`, `rect_anchor`, `set_caret_at_line_col`, `add_caret_at_line_col`, `set_stream_selection_to_line_col`, `insert_plain`, `move_char`, `move_multi_char`, `move_multi_line`, `extend_multi_char`, `extend_multi_line`, `insert_multi`, `delete_*_multi` — `crates/doc/src/lib.rs`
- Hit-test / paint: `col_from_x_virtual`, `x_for_col` — `crates/app/src/ui_paint.rs`; Alt+drag + Ctrl/Cmd+click add caret + plain click / stream select path — `crates/app/src/ui.rs`
