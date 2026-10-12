# Column / rectangular select

Date: 2026-10-12  
Version: 0.3.244

## What works

- **Alt+drag** (Option+drag on macOS) builds a rectangular selection.
- Each line in the block gets a range at the same columns (char indices clamp to line length).
- **Virtual space past EOL** for Alt+drag: typing pads short lines with spaces to the target column; copy pads missing columns with spaces in the clipboard. Carets still draw at EOL until padded.
- **Typing**, **Backspace**, **Delete**, **Paste**, **Enter**, and **Tab** apply to every multi-caret / rect range (one undo). Successive typing at the same carets coalesces into one undo unit (1s window), like plain typing.
- **Copy** / **Cut** with two or more ranges joins line slices with newlines (rect copy includes virtual-space spaces).
- **Multi-select next/all** carets also receive typing (same path). Multi-select Next is remappable (`shortcut_multi_select_next`, default Cmd+Alt+N; Shift skips current).
- **Column Editor** (`IDM_EDIT_COLUMNMODE`) still inserts clipboard text or `0,1,2…` at the caret column.

## Differs from Notepad++ / Scintilla

- Virtual space is pad-on-type / pad-on-copy for rect selections; no free caret motion into empty columns, and no click-to-place caret past EOL.
- No click-to-add extra carets beyond multi-select commands and Alt+drag.
- Arrow keys clear multi-carets (they do not move all carets together).
- Paste always inserts the same clipboard string at each caret (no per-line column paste split beyond what you copied).

## Code

- Buffer: `TextBuffer::rect_ranges` — `crates/buffer/src/lib.rs`
- Document: `set_rect_selection`, `insert_multi`, `delete_*_multi` — `crates/doc/src/lib.rs`
- UI: Alt+drag + input path — `crates/app/src/ui.rs`
