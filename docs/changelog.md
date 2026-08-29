# Changelog

## [Unreleased]

## [0.2.10] — 2026-08-29

### Encoding honesty

- Open/tail never insert U+FFFD via `from_utf8_lossy`; invalid UTF-8 uses Windows-1252 with clear status
- Tests cover invalid byte sequences (`docs/encoding.md`)

## [0.2.9] — 2026-08-29

### Undo

- One user command is one undo unit (`with_transaction` for multi-edit helpers)
- Typing coalesce: same kind (insert), adjacent caret, within 1s
- Notes: `docs/undo-transactions.md`

## [0.2.8] — 2026-08-29

### Highlight

- Byte→char conversion is one forward pass (no per-span full rescans)
- Refresh uses a viewport-oriented window (still capped at 512 KiB)
- Notes: `docs/highlight-viewport.md`

## [0.2.7] — 2026-08-29

### Bookmarks

- Bookmarks (and other line marks) shift when inserts or deletes change line structure
- Buffer records `LineStructureEdit`; the editor prefers that over the snap heuristic

## [0.2.6] — 2026-08-29

### Stability (issue #4)

- Log tail: disk reads and rotate reload run on a background worker; the UI only applies `TailMsg` (dirty/suspend policy unchanged)

### Tabs / open

- Async large-file load binds to a stable `DocumentId` (not tab index)
- Pending load apply drops when the id is gone or no longer the loading placeholder

## [0.2.5] — 2026-08-29

### Save

- Atomic save: write a sibling temp file, `sync_all`, then rename over the target (std rename replaces on Windows)
- Save no longer creates missing parent directories

## [0.2.4] — 2026-08-29

### CLI

- `-V` / `--version`, `-n` / `--line`, `-ro` / `--read-only`, `--` end of options
- Help → Command Line Arguments documents the same flags

### Preferences follow-up

- View → Word wrap writes `settings.json`
- Edit → Indent Tab uses Preferences tab width

## [0.2.3] — 2026-08-29

### Dual view

- Menu Edit (and Format) commands use the focused pane tab, not only the tab-bar active document

### Encoding

- Format → ANSI: save writes Windows-1252 (lossy); UTF-8 / UTF-8-BOM set per-tab save encoding
- Open files keep the detected encoding for later save (`docs/encoding.md`)

### Compare

- Re-diff line tags after edits (~200 ms debounce) while Compare is on

### Change history

- Amber gutter ticks for unsaved edits; green after save (promote, not clear)
- Line-index remap on insert/delete (`LineEditSnap` / `prepare_edit`)

## [0.2.2] — 2026-08-29

### Release checkpoint

- About tagline: “a Notepad++ inspired editor, rebuilt for fun”
- Overnight gap loop started on issue #6 (`docs/overnight-gaps.md`)

## [0.2.1] — 2026-08-29

### Compare fix

- Move to Other View no longer undoes itself (removed extra Switch)
- Compare pins left/right panes to the compared pair so colours stay visible
- Switch in compare mode swaps both sides and their tags
- Clearer compare how-to in `docs/compare.md`

## [0.2.0] — 2026-08-29

Large feature batch after 0.1.2 (menu parity, dual view, compare, UX).

### Dual view edit

- Other view pane is writable (type, delete, clipboard, caret, selection)
- Click a pane to focus keyboard input; undo shortcuts follow the focused pane
- Sync scroll / zoom sync / compare washes unchanged

### Built-in compare

- View → Compare with Other View: 2-way side-by-side colours (red delete / green insert) + sync scroll
- In-process line LCS (`diff.rs`); Clear Compare to exit; max 3000 lines/side

### Menu parity (issue #1)

- File: `IDM_PINTAB` toggles active-tab pin; Close All but Pinned reports keep/closed counts
- Fold / hide lines: hidden lines leave the viewport
- Style-mark washes and bookmark ticks in the gutter
- Search: style mark, jump, clear, and copy-styled helpers
- Edit Cut / Copy / Paste use the session clipboard
- Placeholder / status-only menus cleared; Coming Soon stubs gone (`docs/menu-todo.md`)

### Editor UX

- Drag tabs on the tab bar to reorder (live slide; dual/compare indices remap)
- Prompt before closing unsaved tabs (Save / Don't Save / Cancel), including Close All variants, Exit, and window close
- Keep the last editor line clear of the status bar (status panel before editor + bottom padding)
- Help (?) → Changelog opens `docs/changelog.md` on GitHub
- Status bar shows `v{version}` and git short hash
- Preferences: log-tail, font size, line numbers
- CLI: open path args + `-h` / `--help`

### Earlier on `dev` (after 0.1.2)

- Command split by domain for parallel agents (`docs/agent-parallel.md`)
- Log open dialog, Help → Debug Info / Open Logs, dirty-safe tail stability

## [0.1.2] — 2026-08-28

### Menu parity (issue #1)

- Encoding menu: all `IDM_FORMAT_*` acknowledged (UTF-8 in memory)
- Edit: sort lines, remove dups, blank below, sentence case, split, Mac EOL

Ready ~337 / stub ~237 — see `docs/menu-todo.md`.

## [0.1.1] — 2026-08-28

### Progress on menu parity ([#1](https://github.com/raro42/npp-rust/issues/1))

- Inventory: `docs/menu-todo.md`
- File: Save All / Copy As / Rename / close variants / open folder & viewer / shell
- Edit: case, join/move lines, datetime, path copy helpers
- Search: set-and-find, Go to Line
- View: zoom, always-on-top, tab switch

## [0.1.0] — 2026-08-28

First public release of **npp-rs**.

### Features

- Multi-tab editor (UTF-8), open / save / recent files
- Find and replace
- Undo / redo, indent, line ops, word select
- Tree-sitter highlight (Rust, C/C++, Python, SQL, Markdown, JSON, …)
- Full Notepad++-style menu tree; ready items tinted teal; stubs show Coming Soon
- In-process plugins and format helpers
- Agent loop + public-repo privacy gates (`agents/`, `scripts/gh-safe.sh`)

### Builds

GitHub Actions release binaries for Linux, Windows, and macOS.
