# Parallel gap sweep

Date: 2026-08-29

Agent loop paused. Workers target gaps in `docs/whats-missing.md`.

| # | Focus | Owns (only) |
|---|--------|-------------|
| 1 | Pin UI + Preferences + dual-view MVP | `ui.rs`, `recent.rs`, `commands/view.rs` |
| 2 | Change-history marks | `doc`, `editor.rs`, `commands/search.rs`, `ui_paint.rs` |
| 3 | CLI path args | `main.rs`, `commands/help.rs` |
| 4 | Encoding load/save | `commands/format.rs`, `crates/fs` |
| 5 | RTL + richer call tips | `commands/edit.rs` |
| 6 | Pin from File menu (`IDM_PINTAB` + Close All but Pinned status) | `commands/file.rs` — done 2026-08-29 |
| 7 | Settings import / Plugin Admin | `commands/misc.rs`, `crates/plugins` |

Do not edit another row’s files. Commit and push each batch to `origin/main`.
