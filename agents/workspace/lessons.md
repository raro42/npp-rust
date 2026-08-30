# Lessons

Patterns after operator corrections. Keep short. One lesson per bullet.

- Prefer `./scripts/ci-local.sh` over `cargo check` alone — CI runs fmt + clippy `-D warnings`.
- Day-to-day branch is **`main`** only (no long-lived `dev`).
- Do not claim “Coming Soon stubs remain” — they are cleared; depth gaps live in `docs/whats-missing.md`.
- Parallel agents on the same files without worktrees cause lost work; prefer sequential commits on `main`.
