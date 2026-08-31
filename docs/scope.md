# npp-rs scope (honest)

**Date:** 2026-08-31

## What “full Notepad++ clone” means

Official Notepad++ is a large Win32 + Scintilla product (plugins DLL ABI, docking, column mode, macros, 80+ lexers, UDL, localization, updater, …). A **complete** clone is multi-year work for a team.

## What this project is

**npp-rs** — OS-agnostic Rust editor **inspired by** Notepad++, built to grow while other work runs.

Deep gap list (hotkeys, DnD, feature table): [gap-analysis-vs-npp.md](gap-analysis-vs-npp.md).

## Done toward a serious editor

- Full Notepad++-shaped main-menu tree (wired; teal ≠ full depth)
- Tabs, Open Recent, Find / Replace, bookmarks, change-history bars
- Rope buffer, undo/redo, dual view, 2-way compare
- Tree-sitter highlight subset; theme JSON + N++ XML subset
- Encoding: UTF-8 / BOM / ANSI / UTF-16 LE·BE (BOM)
- In-process plugin builtins (not N++ DLL ABI)

## Still not Notepad++

- Larger hard-wired shortcut set (no `shortcuts.xml` remap)
- File drop + selection drag move/copy (v0.3.8); Alt+rect column select + multi-caret typing (v0.3.9; no virtual space)
- No N++ plugin ABI / Plugin Admin install
- No UDL, hex editor, FTP/cloud, UI localization
- Autosave / backup: MVP in Preferences (v0.3.11); not full N++ snapshot sessions
- Macros record menu IDs only; folding is indent-hide MVP

## Principle

Ship **nice, working essentials** first. Grow features from the upstream reference when useful — do not fake “100% clone” in the About box.

## App crate layout

- Menu commands live under `crates/app/src/commands/`. See [agent-parallel.md](agent-parallel.md).
- Viewport paint hot path: `ui_paint.rs`; shell/UI: `ui.rs`.
