# Parallel stub sweep (8 agents)

Date: 2026-08-29

Agent loop paused so these workers do not fight `cursor-agent`.

| # | Domain | Owns | Focus |
|---|--------|------|--------|
| 1 | Search | `commands/search.rs` | Find in files / char range / change history |
| 2 | View | `commands/view.rs` | Sync scroll, LTR/RTL, panels (no Doc Map UI) |
| 3 | Edit | `commands/edit.rs` | Column mode, call tips |
| 4 | File | `commands/file.rs` | Close all but pinned |
| 5 | Encoding | `commands/format.rs` | ANSI / UTF-8 labels |
| 6 | Help | `commands/help.rs` | Command line arguments |
| 7 | Settings/Run | `commands/misc.rs` | Shortcut mapper, import, Run |
| 8 | UI | `ui.rs` (+ thin flags) | Doc Map, Function List, Char Panel |

Rule: one agent per domain file (`docs/agent-parallel.md`).
