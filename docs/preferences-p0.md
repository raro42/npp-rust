# Preferences and P0 polish (v0.3.1)

Date: 2026-08-29  
Issue: https://github.com/raro42/npp-rust/issues/7

## Settings (`npp-rs/settings.json`)

New keys:

| Key | Meaning |
|-----|---------|
| `gutter_extra` | Extra gutter pixels (0–40) |
| `caret_blink` | Blink caret |
| `default_eol` | `lf` / `crlf` for Enter |
| `recent_max` | Recent file cap (5–40); load and save both use this cap |
| `restore_session` | Reopen session on launch |
| `find_match_case` / `find_whole_word` / `find_in_selection` / `find_wrap` / `find_regex` | Find options |
| `find_query` / `replace_with` | Last find/replace strings |
| `compare_ignore_ws` | Compare ignores whitespace runs (also View menu toggle) |
| `compare_ignore_case` | Compare ignores letter case (also View menu toggle) |
| `compare_ignore_blank` | Compare skips blank lines in LCS (also View menu toggle) |
| `compare_hide_equal` | Compare hides Equal (unchanged) lines in both panes (also View menu toggle) |
| `compare_hide_equal_context` | Equal lines kept visible on each side of a change when hide-equal is on (`0`–`10`, default `3`) |
| `backup_on_save` | Copy on-disk file into `npp-rs/backup/` before overwrite |
| `autosave_interval_secs` | Autosave dirty named tabs (`0` = off; else 15–900) |
| `show_fold_margin` | Gutter fold markers (`−` / `+`); default on |
| `show_whitespace` | Show space/tab glyphs (View + Preferences); default off |
| `show_eol` | Show end-of-line marks; default off |
| `show_npc` | Show non-printing / control characters; default off |
| `show_indent_guide` | Vertical indent guides; default off |
| `shortcut_word_wrap` | Word-wrap toggle chord (`Alt+Z` default; e.g. `Ctrl+W`) |
| `shortcut_find_next` | Find-next chord (`F3` default; Shift flips to find previous) |
| `shortcut_find_next_global` | Global find-next chord (`Cmd+G` default; Shift flips to find previous) |
| `shortcut_next_bookmark` | Next-bookmark chord (`F2` default; Shift flips to previous) |
| `shortcut_toggle_bookmark` | Toggle-bookmark chord (`Cmd+F2` default) |
| `shortcut_next_diff` | Next-compare-diff chord (`F7` default; Shift flips to previous) |
| `shortcut_first_diff` | First-compare-diff chord (`Cmd+F7` default; Shift flips to last) |
| `shortcut_next_hidden_equal` | Next-hidden-equal chord (`Alt+F7` default; Shift flips to previous) |
| `shortcut_first_hidden_equal` | First-hidden-equal chord (`Cmd+Alt+F7` default; Shift flips to last) |
| `shortcut_apply_compare_hunk` | Apply-compare-hunk-from chord (`Cmd+Alt+Left` default; opposite Left/Right applies to other; Shift applies all) |
| `shortcut_hide_equal_context` | Hide-equal-context increase chord (`Alt+]` default; opposite `[` / `]` decreases) |
| `shortcut_goto_line` | Go-to-line chord (`Cmd+L` default; Cmd/Ctrl+Shift+L delete line stays hard-wired) |
| `shortcut_duplicate_line` | Duplicate-line chord (`Cmd+D` default) |
| `shortcut_delete_line` | Delete-line chord (`Cmd+Shift+L` default) |
| `shortcut_indent` | Indent-lines chord (`Cmd+]` default) |
| `shortcut_outdent` | Outdent-lines chord (`Cmd+[` default) |
| `shortcut_format_document` | Format-document chord (`Cmd+Shift+I` default) |
| `shortcut_close_tab` | Close-tab chord (`Cmd+W` default) |
| `shortcut_new` | New-file chord (`Cmd+N` default) |
| `shortcut_open` | Open-file chord (`Cmd+O` default) |
| `shortcut_save` | Save chord (`Cmd+S` default) |
| `shortcut_save_as` | Save-as chord (`Cmd+Shift+S` default) |
| `shortcut_find` | Find-bar chord (`Cmd+F` default) |
| `shortcut_replace` | Replace-bar chord (`Cmd+H` default) |
| `shortcut_replace_alt` | Alternate replace-bar chord (`Cmd+Shift+F` default) |
| `shortcut_select_all` | Select-all chord (`Cmd+A` default) |
| `shortcut_undo` | Undo chord (`Cmd+Z` default; Shift flips to redo) |
| `shortcut_redo` | Redo alternate chord (`Cmd+Y` default; Shift+undo also redo) |
| `shortcut_zoom_in` | Zoom-in chord (`Cmd+=` default) |
| `shortcut_zoom_out` | Zoom-out chord (`Cmd+-` default) |
| `shortcut_zoom_restore` | Zoom-restore chord (`Cmd+0` default; mouse wheel stays hard-wired) |
| `shortcut_toggle_log_tail` | Toggle log-tail follow chord (`Cmd+Shift+T` default) |
| `shortcut_reload` | Reload-from-disk chord (`Cmd+R` default) |

Unknown extra keys are kept on load/save (not dropped).

See also: `docs/autosave-backup.md`, `docs/folding.md`.

## Session

File: `npp-rs/session.txt` (config dir). One path per line; optional `@folds h1,h2,…` after a path restores folded fold-header lines.  
Saved on quit when restore is on. Menu Save/Load Session uses the same file.

## Find

Bar shows Case / Word / Sel / Wrap / Re toggles and live match count. Opening Find with a multi-line selection arms Sel. Next/Prev status is `n/total`. Wrap off reports passed end/beginning of file or selection. Re uses a linear-time regex engine. With Re on, Replace expands `$n` / `${n}` / `$&` / `$$` and `\1` (also `\n` `\t` `\r`).
