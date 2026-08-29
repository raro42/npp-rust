# Issue #5 — log open dialog

Date: 2026-08-29

## Status

**Implemented on `dev`.** Closed when housekeeping finished.

## What went wrong (why it looked unfinished)

1. Code landed (dialog + Remember → `npp-rs/settings.json` + Preferences).
2. Agent wrote “implemented (not committed)” into `WIP-5` and left the GitHub issue **open** with `agent:wip`.
3. The agent loop died before issue close: IDE/`nohup` sessions were torn down, and `cursor-agent` runs are long — cycles looked “stuck” at WIP labels.
4. Fake “menu stubs 0” work stole attention; nobody finished issue hygiene.

**Fix applied 2026-08-29:** unit tests, issue closed, task moved to `agents/tasks/done/`, docs updated.

## Behaviour

- Open `*.log` → **Follow this log?** (Ask / Always / Never via Remember).
- Settings → Preferences edits the same preference.
- Tests: `opening_log_*` in `crates/app/src/editor.rs`.
