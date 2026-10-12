# Dual view

Date: 2026-08-29

## Behaviour

- View → Move / Switch / Clone to Other View opens a right-hand pane.
- Each pane is a real editor for its tab (type, delete, clipboard, caret, selection).
- Click a pane to focus it for keyboard input.
- The focused pane owns Cmd/Ctrl+Z / Y undo-redo and select-all shortcuts.
- Menu **Edit** commands that change text also use the focused pane tab (`focused_edit_tab`), not only the tab-bar active document.
- Sync H/V scroll and zoom sync still share line scroll / font size. Menu / shortcut toggles flip the live pane flags (so session-restored sync turns off on the first click). Zoom Sync also persists in `npp-rs/settings.json` (`zoom_sync`).
- Compare mode colours both panes and re-diffs after edits (~200 ms debounce).

## Session

- With **Preferences → Restore session** on, the other-pane tab is saved as `@other` (alias `@otherview`) under its path in `npp-rs/session.txt`. Relaunch re-opens dual view with that tab on the right (primary stays `@active`). Untitled other panes are not persisted.
- When the other pane had keyboard focus, session also writes `@focus` (alias `@otherfocus`) on that entry. Relaunch focuses the other pane after the split opens (primary focus stays the default when `@focus` is absent).
- Synchronize Vertical / Horizontal Scrolling flags are saved as `@syncv` / `@synch` (aliases `@syncvert` / `@synchoriz`) on the active entry. Relaunch restores those flags when the split opens.
- When Compare was on, session also writes `@compare` (alias `@cmp`) on the active entry. Relaunch restarts Compare against the `@other` pane after the split opens.
- A ⌘/Ctrl-click marked Compare partner (tab `⇄`) is saved as `@partner` (alias `@cmppartner`) under that tab’s path. Relaunch re-marks it even when dual view / Compare were off.

## Limits

- The other pane keeps its own viewport highlight cache (same 512 KiB window rules as the primary pane).

## Code

- `EditorPane` + `focused_pane` + `dispatch_menu_cmd` — `crates/app/src/ui.rs`
- `mark_text_changed_at` / `undo_at` / `redo_at` — `crates/app/src/editor.rs`
