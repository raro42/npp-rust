# Lessons

Patterns after operator corrections. Keep short. One lesson per bullet.

- Prefer `./scripts/ci-local.sh` over `cargo check` alone — CI runs fmt + clippy `-D warnings`.
- Install git hooks once: `./scripts/install-git-hooks.sh` (pre-push blocks bad pushes).
- Day-to-day branch is **`main`** only (no long-lived `dev`).
- Do not claim “Coming Soon stubs remain” — they are cleared; depth gaps live in `docs/whats-missing.md`.
- Parallel agents on the same files without worktrees cause lost work; prefer sequential commits on `main`.
- Windows-only `#[cfg(windows)]` is not clippy’d on macOS — avoid needless `return` in those blocks.
