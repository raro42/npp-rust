# File drop and selection drag

Date: 2026-08-31

## File drop

- Drop one or more files onto the window to open each in a tab.
- Folders and non-files are skipped (status line reports counts).
- Uses egui `raw.dropped_files` / `hovered_files`.

## Selection drag

- Drag inside an existing selection to **move** the text.
- Hold **Ctrl** (Windows/Linux) or **Cmd** (macOS) while dragging to **copy**.
- Drop inside the selection is a no-op for move.
- One undo unit via `TextBuffer::drag_selection_to`.
- Orange drop caret shows the insert point.
- Works in the primary pane and the dual-view secondary pane (same buffer only).

## Code

- Buffer: `crates/buffer/src/lib.rs` — `drag_selection_to`
- UI: `crates/app/src/ui.rs` — `handle_file_drops`, `SelTextDrag`, `finish_sel_text_drag`
