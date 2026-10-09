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
- Encoding: UTF-8 / BOM / ANSI / UTF-16 LE·BE (BOM or no-BOM detect)
- In-process plugin builtins (not N++ DLL ABI)

## Still not Notepad++

- Larger hard-wired shortcut set; word wrap + find next + find next (global) + next bookmark + toggle bookmark + compare start/clear + swap compare sides + compare to saved + copy / open compare diff + copy / open compare summary + copy / open compare hunk + word jump + next difference + first difference + next hidden equal + first hidden equal + apply compare hunk + hide unchanged lines + hide-equal context + go to line + duplicate line + delete line + indent + outdent + format document + close tab + next / prev tab + new + open + reload + save + save as + save all + print + Shift+Tab outdent + fold / unfold all + fold / unfold current + matching brace + move line + toggle comment + find + close find/replace + replace + replace (alternate) + select all + undo + redo + zoom in + zoom out + zoom restore + toggle log tail remappable (`shortcut_word_wrap`, `shortcut_find_next`, `shortcut_find_next_global`, `shortcut_next_bookmark`, `shortcut_toggle_bookmark`, `shortcut_compare`, `shortcut_swap_compare`, `shortcut_compare_to_saved`, `shortcut_copy_compare_diff`, `shortcut_copy_compare_summary`, `shortcut_copy_compare_hunk`, `shortcut_word_jump`, `shortcut_next_diff`, `shortcut_first_diff`, `shortcut_next_hidden_equal`, `shortcut_first_hidden_equal`, `shortcut_apply_compare_hunk`, `shortcut_hide_equal`, `shortcut_hide_equal_context`, `shortcut_goto_line`, `shortcut_duplicate_line`, `shortcut_delete_line`, `shortcut_indent`, `shortcut_outdent`, `shortcut_shift_tab_outdent`, `shortcut_fold_all`, `shortcut_fold_current`, `shortcut_matching_brace`, `shortcut_move_line`, `shortcut_toggle_comment`, `shortcut_format_document`, `shortcut_close_tab`, `shortcut_next_tab`, `shortcut_new`, `shortcut_open`, `shortcut_reload`, `shortcut_save`, `shortcut_save_as`, `shortcut_save_all`, `shortcut_print`, `shortcut_find`, `shortcut_close_find`, `shortcut_replace`, `shortcut_replace_alt`, `shortcut_select_all`, `shortcut_undo`, `shortcut_redo`, `shortcut_zoom_in`, `shortcut_zoom_out`, `shortcut_zoom_restore`, `shortcut_toggle_log_tail`); no full `shortcuts.xml` yet
- File drop + selection drag move/copy (v0.3.8); Alt+rect column select + multi-caret typing (v0.3.9; no virtual space)
- No N++ plugin ABI / Plugin Admin install
- No UDL, hex editor, FTP/cloud, UI localization
- Autosave / backup: MVP in Preferences (v0.3.11); not full N++ snapshot sessions
- Macros record menu IDs only; folding: margin + brace/indent MVP (v0.3.12)

## Principle

Ship **nice, working essentials** first. Grow features from the upstream reference when useful — do not fake “100% clone” in the About box.

## App crate layout

- Menu commands live under `crates/app/src/commands/`. See [agent-parallel.md](agent-parallel.md).
- Viewport paint hot path: `ui_paint.rs`; shell/UI: `ui.rs`.
