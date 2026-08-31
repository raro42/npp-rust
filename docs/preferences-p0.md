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
| `recent_max` | Recent file cap (5–40) |
| `restore_session` | Reopen session on launch |
| `find_match_case` / `find_whole_word` | Find options |
| `find_query` / `replace_with` | Last find/replace strings |
| `compare_ignore_ws` | Compare ignores whitespace runs |
| `backup_on_save` | Copy on-disk file into `npp-rs/backup/` before overwrite |
| `autosave_interval_secs` | Autosave dirty named tabs (`0` = off; else 15–900) |
| `show_fold_margin` | Gutter fold markers (`−` / `+`); default on |

See also: `docs/autosave-backup.md`, `docs/folding.md`.

## Session

File: `npp-rs/session.txt` (config dir). One path per line.  
Saved on quit when restore is on. Menu Save/Load Session uses the same file.

## Find

Bar shows Case / Word toggles and live match count. Next/Prev status is `n/total`.
