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
