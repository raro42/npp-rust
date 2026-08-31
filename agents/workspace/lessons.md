# Lessons

Patterns after operator corrections. Keep short. One lesson per bullet.

- Prefer `./scripts/ci-local.sh` over `cargo check` alone — CI runs fmt + clippy `-D warnings`.
- Install git hooks once: `./scripts/install-git-hooks.sh` (pre-push blocks bad pushes).
- Day-to-day branch is **`main`** only (no long-lived `dev`).
- Do not claim “Coming Soon stubs remain” — they are cleared; depth gaps live in `docs/whats-missing.md`.
- Parallel agents on the same files without worktrees cause lost work; prefer sequential commits on `main`.
- Windows-only `#[cfg(windows)]` is not clippy’d on macOS — avoid needless `return` in those blocks.
- DONE handoff lines may be `- Handoff: complete`; loop grep must accept that or 004 blocks forever.
- Start the loop with `agents/start-unattended.command` (Terminal) so it survives; macOS has no `setsid`.
- Cloud CI is 2×/day only; trust pre-push `ci-local` and loop 005 (`agents/workspace/ci-status.md`).
- Never start a second agent loop. Use `./agents/npp-cursor-loop.sh status`. Force restart only with `AGENT_LOOP_FORCE_RESTART=1`.
