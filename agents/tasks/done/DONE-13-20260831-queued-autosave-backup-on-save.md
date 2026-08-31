# P1: Autosave / backup-on-save

## GitHub Issues
- **Issue:** https://github.com/raro42/npp-rust/issues/13
- **13**

## Problem / goal
See GitHub issue #13 and docs/gap-analysis-vs-npp.md / docs/next-gaps.md.

## High-level instructions for coder
- Keep the change small and on branch `main`.
- Run `./scripts/ci-local.sh` before push.
- Bump version + changelog when user-visible.
- Do not close the issue from the coder step.

## Privacy
- No home paths or secrets in commits or comments.

## Progress
- **2026-08-31 (coder):** Preferences backup-on-save (`npp-rs/backup/` path layout) + autosave interval for dirty named tabs. Settings keys in `npp-rs/settings.json`. Version **0.3.11** (commit `0cf60c3`). Docs: `docs/autosave-backup.md`. `./scripts/ci-local.sh` green. Handoff → TEST (issue left open).

## Tester (2026-08-31)
- Verified prefs + `crates/app/src/backup.rs` + `tick_autosave` + settings keys + `docs/autosave-backup.md` (v0.3.11 / `0cf60c3`).
- `./scripts/ci-local.sh` — **PASS** (fmt, clippy, 119 tests, release).
- Result: **PASS** → `DONE-`; leave issue #13 open for handoff.

## Handoff (2026-08-31)
- User-facing notes already in `docs/changelog.md` under **[0.3.11]** (not Unreleased).
- Task goal met: Preferences backup-on-save + autosave interval, settings keys, docs (v0.3.11, `0cf60c3`).
- Close issue #13 with `agent:done`.

Handoff: complete
