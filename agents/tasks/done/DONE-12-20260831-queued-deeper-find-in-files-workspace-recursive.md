# P1: Deeper Find in Files (workspace recursive)

## GitHub Issues
- **Issue:** https://github.com/raro42/npp-rust/issues/12
- **12**

## Problem / goal
See GitHub issue #12 and docs/gap-analysis-vs-npp.md / docs/next-gaps.md.

## High-level instructions for coder
- Keep the change small and on branch `main`.
- Run `./scripts/ci-local.sh` before push.
- Bump version + changelog when user-visible.
- Do not close the issue from the coder step.

## Privacy
- No home paths or secrets in commits or comments.

## Progress
- **2026-08-31 (coder):** Recursive Find in Files on `workspace_root`; include/exclude globs on Find bar; skip hidden dirs / symlinks / binary / huge files; caps on depth, files, matches. Version **0.3.10** (commit `0ab1b6a`). `./scripts/ci-local.sh` green. Handoff → TEST (issue left open).

## Tester (2026-08-31)
- Verified: `find_in_files_scan` + caps in `crates/app/src/search_util.rs`; command uses `workspace_root` in `crates/app/src/commands/search.rs`; include/exclude settings in `crates/app/src/recent.rs` + Find bar in `crates/app/src/ui.rs`; unit test `recursive_scan_skips_target_and_respects_include`; version **0.3.10**; commit `0ab1b6a`.
- `./scripts/ci-local.sh` — **PASS** (fmt, clippy `-D warnings`, workspace tests, release build).
- Result: **PASS** → `DONE-`; leave issue #12 open for handoff.

## Handoff (2026-08-31)
- User-facing notes already in `docs/changelog.md` under **[0.3.10]** (not Unreleased).
- Task goal met: recursive Find in Files on workspace root, caps, include/exclude globs, clickable results (v0.3.10, `0ab1b6a`).
- Close issue #12 with `agent:done`.

Handoff: complete
