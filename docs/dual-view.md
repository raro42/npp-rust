# Dual view

Date: 2026-08-29

## Behaviour

- View → Move / Switch / Clone to Other View opens a right-hand pane.
- Each pane is a real editor for its tab (type, delete, clipboard, caret, selection).
- Click a pane to focus it for keyboard input.
- The focused pane owns Cmd/Ctrl+Z / Y undo-redo and select-all shortcuts.
- Menu **Edit** commands that change text also use the focused pane tab (`focused_edit_tab`), not only the tab-bar active document.
- Sync H/V scroll and zoom sync still share line scroll / font size.
- Compare mode colours both panes and re-diffs after edits (~200 ms debounce).

## Limits

- The other pane paints plain text (no syntax colours yet).

## Code

- `EditorPane` + `focused_pane` + `dispatch_menu_cmd` — `crates/app/src/ui.rs`
- `mark_text_changed_at` / `undo_at` / `redo_at` — `crates/app/src/editor.rs`
