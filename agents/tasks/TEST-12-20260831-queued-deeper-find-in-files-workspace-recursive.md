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
- **2026-08-31 (coder):** Recursive Find in Files on `workspace_root`; include/exclude globs on Find bar; skip hidden dirs / symlinks / binary / huge files; caps on depth, files, matches. Version **0.3.10** (commit `2801e03`). `./scripts/ci-local.sh` green. Handoff → TEST (issue left open).
