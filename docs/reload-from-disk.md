# Reload from disk

Date: 2026-08-29

## Behaviour

File → Reload from Disk reads the active tab path again and replaces the buffer.

- Untitled: status only
- Clean: replace immediately; undo resets; dirty clears
- Dirty: Save / Don't Save / Cancel (same style as close)

`open_path` still reuses tabs by path for Open / Recent. Reload does not use that path.

## Saved revision

`TextBuffer::edit_generation` bumps on each new undo unit. `Document::saved_generation` stores that value on save. Undo/redo that returns to the saved generation clears dirty.

## Code

- `EditorState::request_reload` / `reload_from_disk` — `editor.rs`
- Menu — `commands/file.rs`
- UI — `unsaved_reload_window` in `ui.rs`
