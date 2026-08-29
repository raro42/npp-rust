# Help menu — Debug Info and logs

Date: 2026-08-29

## Debug Info (`?` → Debug Info…)

Opens a **read-only editor tab** named `Debug Info`.

Shows:

- App version, OS, and arch
- Whether `logs/panic.log` exists (path relative to process cwd)
- Settings path and log-tail preference

It does **not** send data anywhere.

## Open npp-rust Logs (`?` → Open npp-rust Logs)

Opens every `*.log` under `logs/` relative to the process cwd.

Typical file: `logs/panic.log` (panic hook in `crates/app/src/main.rs`).

If none exist, the status bar says so.

## Related

- [tail-log.md](tail-log.md) — follow a log file as it grows
