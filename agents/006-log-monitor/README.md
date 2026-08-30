# agents/006-log-monitor/

Home for **reading local crash / panic logs**. Standing rule: skim after a crash or odd run.

## Source

- Prefer repo-relative: `logs/panic.log` (and `/tmp/npp-rs-panic.log` only as a local hint — never commit absolute paths into task files).
- **Read-only.** Do not truncate the log from this monitor.

## How to scan

```bash
python3 scripts/scan_panic_log.py
python3 scripts/scan_panic_log.py --write-finding
```

## Findings

When `--write-finding` sees a new signature, it writes `agents/tasks/FEAT-log-*-panic.md` (sanitized). Deduplicate the same panic head within 24 hours via `agents/state/`.
