# File drop and selection drag

Date: 2026-08-31

## File drop

- Drop one or more files onto the window to open each in a tab.
- Folders and non-files are skipped (status line reports counts).
- Uses egui `raw.dropped_files` / `hovered_files`.

## Selection drag

- Drag inside an existing selection to **move** the text.
- Hold **Ctrl** (Windows/Linux) or **Cmd** (macOS) while dragging to **copy**.
- Drop inside the selection is a no-op for move (same buffer).
- One undo unit via `TextBuffer::drag_selection_to` (same buffer).
- Orange drop caret shows the insert point.
- Works in the primary pane and the dual-view secondary pane.
- **Cross-pane (dual view):** drag from one pane onto the other tab to move (or Ctrl/Cmd-copy) into that buffer. Each side gets its own undo unit.
- **Drop into Find:** with the Find bar open, drag a selection onto the Find field to set the query (first line, max 200 chars). Does not move or copy buffer text. Orange highlight on the Find field while hovering.

## Code

- Buffer: `crates/buffer/src/lib.rs` — `drag_selection_to` (same buffer)
- UI: `crates/app/src/ui.rs` — `handle_file_drops`, `SelTextDrag` (`tab` + `drop_tab` + `drop_find`), `finish_sel_text_drag`, `find_query_from_drag_text`
