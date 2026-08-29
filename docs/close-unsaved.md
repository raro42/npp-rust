# Close unsaved documents

Date: 2026-08-29

## Behaviour

Before the app closes a dirty tab, it asks the user:

- **Save** — save, then close
- **Don't Save** — close without save
- **Cancel** — keep the tab open; stop bulk close / quit

## Triggers

| Action | Path |
|--------|------|
| Tab × / middle-click / Cmd+W | `request_close_tab` |
| File → Close | `request_close_tab` |
| File → Close All / to left / right / but current / unchanged | `BulkClose` + prompts per dirty tab |
| File → Exit | `request_quit` |
| Window close (red ×) | Cancel OS close if dirty; then `request_quit` |

## Code

- State: `EditorState::pending_close`, `bulk_close`, `want_quit` in `editor.rs`
- UI: `unsaved_close_window` in `ui.rs`

Force-close without prompt stays for trash move and similar after the file is already gone.
