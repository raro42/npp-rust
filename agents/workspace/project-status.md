# Project status

Date: 2026-10-03  
Repo: `raro42/npp-rust`  
Branch: `main` (in sync with origin)

## Health

The editor is a working beta at **v0.3.12** in `Cargo.toml`. CI on `main` is green (scheduled runs through 2026-10-03). Product work stopped on 2026-08-31. Later commits only update the issue-review timestamp.

## Shipped

- Menu stubs are gone. Handlers exist for the export menu IDs.
- v0.3.0–v0.3.6: stability, preferences, themes, change history, UTF-16.
- v0.3.7–v0.3.12: hotkeys, file drop, column select, Find in Files, autosave, folding.
- Issues **#1–#13** are closed.

## Gaps

| Item | State |
|------|--------|
| Issue **#14** (folding) | Code and tests are in. The GitHub issue is still **open** (`agent:wip`). Handoff is unfinished. |
| GitHub Release | Latest tag is **v0.3.6**. Versions **v0.3.7–v0.3.12** are on `main` with no tag or release. |
| Backlog | No new open issues after #14. Later ideas (autocomplete, macros, 3-way compare, plugins, i18n) have no issues yet. |
| Docs | `docs/whats-missing.md` and `docs/next-gaps.md` still describe the 2026-08-31 checkpoint. |

## CI

See `agents/workspace/ci-status.md`. Latest success: run `37102836208` (2026-10-03).
