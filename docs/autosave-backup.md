# Autosave and backup-on-save

Date: 2026-08-31  
Issue: https://github.com/raro42/npp-rust/issues/13

## Preferences

Open **Settings → Preferences…** → **Files**.

| Setting | Key in `npp-rs/settings.json` | Meaning |
|---------|-------------------------------|---------|
| Backup on save | `backup_on_save` | Before overwrite, copy the on-disk file into the config backup tree |
| Autosave interval (sec) | `autosave_interval_secs` | `0` = off; otherwise clamp to 15–900 seconds |

Defaults: both off (`false` / `0`).

## Backup layout

Copies go under the app config dir as `npp-rs/backup/…`, mirroring the source path
(drive / root markers stripped). Example labels in the UI use `npp-rs/backup` only —
never a machine home path.

One backup file per source path (next save overwrites that backup).

## Autosave behaviour

- Runs on a timer while the interval is non-zero.
- Saves dirty tabs that already have a path.
- Skips untitled tabs (no Save As dialog).
- Skips ANSI (Windows-1252) saves that would need a lossy confirm; those stay dirty.

## Out of scope

- Notepad++ verbose timestamped snapshot sessions
- Cloud sync
- Sibling `.bak` next to the source file
