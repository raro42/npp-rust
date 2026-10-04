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
| `backup_on_save` | Copy on-disk file into `npp-rs/backup/` before overwrite |
| `autosave_interval_secs` | Autosave dirty named tabs (`0` = off; else 15–900) |
| `show_fold_margin` | Gutter fold markers (`−` / `+`); default on |
| `show_whitespace` | Show space/tab glyphs (View + Preferences); default off |
| `show_eol` | Show end-of-line marks; default off |
| `show_npc` | Show non-printing / control characters; default off |
| `show_indent_guide` | Vertical indent guides; default off |
| `shortcut_word_wrap` | Word-wrap toggle chord (`Alt+Z` default; e.g. `Ctrl+W`) |

Unknown extra keys are kept on load/save (not dropped).

See also: `docs/autosave-backup.md`, `docs/folding.md`.

## Session

File: `npp-rs/session.txt` (config dir). One path per line.  
Saved on quit when restore is on. Menu Save/Load Session uses the same file.

## Find

Bar shows Case / Word / Sel / Wrap / Re toggles and live match count. Opening Find with a multi-line selection arms Sel. Next/Prev status is `n/total`. Wrap off reports passed end/beginning of file or selection. Re uses a linear-time regex engine. With Re on, Replace expands `$n` / `${n}` / `$&` / `$$` and `\1` (also `\n` `\t` `\r`).
