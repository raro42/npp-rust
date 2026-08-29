# Changelog

## [Unreleased]

### Built-in compare

- View → Compare with Other View: 2-way side-by-side colours (red delete / green insert) + sync scroll
- In-process line LCS (`diff.rs`); Clear Compare to exit; max 3000 lines/side

### Menu parity (issue #1)

- File: `IDM_PINTAB` toggles active-tab pin; Close All but Pinned reports keep/closed counts
- Fold / hide lines: hidden lines leave the viewport
- Style-mark washes and bookmark ticks in the gutter
- Search: style mark, jump, clear, and copy-styled helpers
- Edit Cut / Copy / Paste use the session clipboard
- Placeholder / status-only menus: 141 → 35 (`docs/menu-todo.md`)

### Editor UX

- Drag tabs on the tab bar to reorder (live slide; dual/compare indices remap)
- Prompt before closing unsaved tabs (Save / Don't Save / Cancel), including Close All variants, Exit, and window close
- Keep the last editor line clear of the status bar (status panel before editor + bottom padding)
- Help (?) → Changelog opens `docs/changelog.md` on GitHub

### Earlier on `dev` (after 0.1.2)

- Menu stubs largely cleared (Ready inventory in `docs/menu-todo.md`)
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
