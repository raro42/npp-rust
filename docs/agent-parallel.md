# Parallel agents — file ownership

Date: 2026-08-29

Menu command code lives under `crates/app/src/commands/`. **One agent owns one domain file** to avoid merge fights.

| Domain file | Owns |
|-------------|------|
| `commands/file.rs` | File menu (`IDM_FILE_*`) |
| `commands/edit.rs` | Edit menu (`IDM_EDIT_*`) |
| `commands/search.rs` | Search (`IDM_SEARCH_*`, `IDM_FOCUS_ON_*`) |
| `commands/view.rs` | View (`IDM_VIEW_*`) |
| `commands/format.rs` | Encoding / EOL (`IDM_FORMAT_*`) |
| `commands/lang.rs` | Language (`IDM_LANG_*`) |
| `commands/misc.rs` | Tools / Window / Settings |
| `commands/help.rs` | Help (`IDM_ABOUT`, …) |
| `commands/common.rs` | Shared helpers only |
| `commands/mod.rs` | Types, `dispatch` router, `is_implemented` list |

## Rules

1. Do not edit another agent’s domain file unless you coordinate first.
2. Put new shared helpers in `common.rs`.
3. When you implement a command, add its id to `is_implemented` in `mod.rs` (or the next agent cannot see teal).
4. Prefer small commits per domain.
5. Inventory: `docs/menu-todo.md`.

## UI / editor

| File | Owns |
|------|------|
| `ui.rs` | egui shell, menus, tabs, dialogs, input |
| `ui_paint.rs` | Viewport text metrics + highlighted line paint (hot path) |
| `editor.rs` | `EditorState`, tabs I/O |
| `doc` / `buffer` crates | document model |

Avoid two agents editing `ui.rs` at once. Prefer paint changes in `ui_paint.rs`.

## Do not undo

Keep menu commands under `crates/app/src/commands/*.rs`. Do not fold them back into one file.
