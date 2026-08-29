# Dual view

Date: 2026-08-29

## Behaviour

- View → Move / Switch / Clone to Other View opens a right-hand pane.
- Each pane is a real editor for its tab (type, delete, clipboard, caret, selection).
- Click a pane to focus it for keyboard input.
- The focused pane owns Cmd/Ctrl+Z / Y undo-redo and select-all shortcuts.
- Sync H/V scroll and zoom sync still share line scroll / font size.
- Compare mode still colours both panes.

## Limits

- Menu Edit commands still target the **active** (tab-bar) document, not the other pane.
- The other pane paints plain text (no syntax colours yet).
- Compare tags do not refresh when you edit after Compare.

## Code

- `EditorPane` + `focused_pane` — `crates/app/src/ui.rs`
- `mark_text_changed_at` / `undo_at` / `redo_at` — `crates/app/src/editor.rs`
