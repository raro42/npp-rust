# P0 feature map (code pointers)

Date: 2026-08-29  
Source: codebase scan for Preferences, Find/Replace, Session, Compare.

## 1. Preferences / AppSettings

| Item | Location |
|------|----------|
| Struct | `crates/app/src/recent.rs` — `AppSettings`, `LogTailOnOpen` |
| Load/save | `AppSettings::load()`, `AppSettings::save()` |
| Disk path | `{config}/npp-rs/settings.json` via `settings_store_path()`; label `SETTINGS_REL` |
| UI | `crates/app/src/ui.rs` — `EditorApp::preferences_window` |
| Menu | `IDM_SETTING_PREFERENCE` → `commands/misc.rs` sets `UiFlags.show_preferences` |
| Init | `EditorState::new()` calls `AppSettings::load()` |

**Fields today:** `log_tail_on_open`, `font_size`, `show_line_numbers`, `tab_width`, `word_wrap`, `status_show_lang`, `status_show_chars`, `theme_id`.

**Not persisted:** dual-view sync flags, find options, session restore opt-in, compare options.

## 2. Find / Replace

| Item | Location |
|------|----------|
| Query state | `EditorState.find_query`, `find_open` |
| Replace UI string | `EditorApp.replace_with`, `show_replace` (not in settings) |
| Bar UI | `EditorApp::find_replace_bar` |
| Commands | `commands/search.rs` — `IDM_SEARCH_FIND` / `REPLACE` / next/prev |
| API | `EditorState::find_next/prev`, `replace_next`, `replace_all` |
| Buffer search | `TextBuffer::find_next/prev(query, from, wrap)` — case-sensitive substring |
| Match-count UI | Status only (`Find: match at {s}`); no live count in bar |
| Replace All | `text.replace` + `replace_document` (one undo Replace unit) |
| Case/word options | Only in Edit multi-select helpers (`find_all_matches` in `commands/edit.rs`) |

## 3. Session

| Item | Location |
|------|----------|
| Menu | `IDM_FILE_SAVESESSION` / `LOADSESSION` in `commands/file.rs` |
| Format | CWD file `npp-rs-session.txt` — one absolute path per line |
| Save | Paths of tabs that have `Document.path`; untitled skipped |
| Load | `open_path` for each existing line; no auto-launch |
| Launch | `CliOptions.paths` only (`main.rs` / `EditorApp::open_argv_paths`) |

## 4. Compare / dual view

| Item | Location |
|------|----------|
| Diff engine | `crates/app/src/diff.rs` — `diff_line_tags`, `LineKind`, `MAX_COMPARE_LINES=3000` |
| UI state | `EditorApp` compare_* + `sync_scroll_h/v` |
| Menu | `IDM_VIEW_COMPARE` / `CLEARCOMPARE` / `SYNSCROLLH` / `SYNSCROLLV` |
| Atomics | `commands/view.rs` — `SYNC_SCROLL_H/V`, `ZOOM_SYNC` (process session, not settings.json) |
| Ignore whitespace | **None** — exact line string equality after EOL trim only |
| Re-diff | `compare_stale` + `refresh_compare_if_stale` (~200 ms) |

## Related docs

- `docs/next-gaps.md` (P0 backlog)
- `docs/compare.md`, `docs/dual-view.md`, `docs/undo-transactions.md`
