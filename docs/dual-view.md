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

## Session

- With **Preferences → Restore session** on, the other-pane tab is saved as `@other` (alias `@otherview`) under its path in `npp-rs/session.txt`. Relaunch re-opens dual view with that tab on the right (primary stays `@active`). Untitled other panes are not persisted.
- When the other pane had keyboard focus, session also writes `@focus` (alias `@otherfocus`) on that entry. Relaunch focuses the other pane after the split opens (primary focus stays the default when `@focus` is absent).

## Limits

- The other pane keeps its own viewport highlight cache (same 512 KiB window rules as the primary pane).
- Session restore does not yet keep sync-scroll flags.

## Code

- `EditorPane` + `focused_pane` + `dispatch_menu_cmd` — `crates/app/src/ui.rs`
- `mark_text_changed_at` / `undo_at` / `redo_at` — `crates/app/src/editor.rs`
