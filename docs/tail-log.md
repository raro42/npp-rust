# Tail log files

Date: 2026-08-28

## Use

1. Open a log file on disk.
2. Toggle tail via **status bar `tail` / `TAIL`**, **View → Monitoring (tail -f)**, or **⌘/Ctrl+⇧+T**.
3. Status bar shows teal **TAIL** when on, weak **tail** when off. Tab title shows `[tail]`.
4. New lines append automatically (~250–300 ms poll). View follows the end.
5. Choose the same menu item again to stop. Log rotate (file shrinks) reloads the buffer.

Demo file for a quick try: write to a path you open, then enable Tail.
