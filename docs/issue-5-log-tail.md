# Issue #5 — log open dialog

Date: 2026-08-29

## Status

**Implemented on `dev`.** Closed when housekeeping finished.

## What went wrong (why it looked unfinished)

1. Code landed (dialog + Remember → `npp-rs/settings.json` + Preferences).
2. Agent wrote “implemented (not committed)” into `WIP-5` and left the GitHub issue **open** with `agent:wip`.
3. The agent loop **died**, so nothing closed the issue or moved the task to `done/`.
4. Fake “menu stubs 0” work stole attention; nobody finished issue hygiene.

## Behaviour

- Open `*.log` → **Follow this log?** (Ask / Always / Never via Remember).
- Settings → Preferences edits the same preference.
- Tests: `opening_log_*` in `crates/app/src/editor.rs`.
