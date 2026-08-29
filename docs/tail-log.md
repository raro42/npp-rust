# Tail log files

Date: 2026-08-29

## Use

1. Open a `*.log` file on disk.
2. A small dialog asks whether to enable Monitoring (tail) now.
3. Check **Remember** to store `always` or `never` in `npp-rs/settings.json` (next to recent files).
4. Or toggle tail later via **status bar `tail` / `TAIL`**, **View → Monitoring (tail -f)**, or **⌘/Ctrl+⇧+T**.
5. Status bar shows teal **TAIL** when on, weak **tail** when off. Tab title shows `[tail]`.
6. New lines append automatically (~250–300 ms poll). View follows the end.
7. Choose the same menu item again to stop. Log rotate (file shrinks) reloads the buffer.

## Discover logs

- **? → Debug Info** opens a tab with relative paths: `logs/panic.log` and `npp-rs/settings.json`, plus the current `log_tail_on_open` preference.
- **? → Open npp-rust Logs** loads every `logs/*.log` under the process cwd.
- Status messages use file names / relative labels (no home absolute paths).

Demo file for a quick try: write to a path you open, then enable Tail.
