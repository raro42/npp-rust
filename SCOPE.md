# npp-rs scope (honest)

**Date:** 2026-08-28

## What “full Notepad++ clone” means

Official Notepad++ is a large Win32 + Scintilla product (plugins DLL ABI, docking, column mode, macros, 100+ lexers, localization, updater, …). A **complete** clone is multi-year work for a team.

## What this project is

**npp-rs** — OS-agnostic Rust editor **inspired by** Notepad++, built to grow while other work runs.

Reference tree (read-only): [reference/notepad-plus-plus](reference/notepad-plus-plus) (shallow clone).

## Done toward a serious editor

- **Full Notepad++ main-menu tree** (574 items from `Notepad_plus.rc`) — wired commands work; others show a clear stub status
- Double-click word / triple-click line / drag select
- Tabs, Open Recent, Find / Replace
- Rope buffer, undo/redo, indent/outdent, duplicate/delete line
- Tree-sitter highlight: Rust, C, C++, Python, SQL, Markdown, JSON
- Format Document for Python / C++ / SQL / Markdown (+ trim/EOL plugins)
- In-process Plugins builtins (plus N++ Plugins menu entries)

## Tests

See [TESTING.md](TESTING.md). CI runs `cargo test --workspace` on push; build alone does not run tests.

## Still not Notepad++

- No N++ plugin ABI / Plugin Admin
- No dual view, docking, column editor, macros, fingerprint, full prefs
- No 100+ lexers / UDL parity
- Formatting is helpful, not clang-format / black / sqlfluff

## Principle

Ship **nice, working essentials** first. Grow features from the upstream reference when useful — do not fake “100% clone” in the About box.
