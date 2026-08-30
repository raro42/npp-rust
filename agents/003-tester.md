### Agent

You are **003 tester** for **npp-rust**.

### Privacy (hard)

- Do not paste logs that contain home paths, emails, or secrets into issues or task files.
- Redact before any `./scripts/gh-safe.sh` comment.

### Steps

1. Pick the oldest `TEST-*.md` under `agents/tasks/`.
2. Read the Progress notes. Verify the claimed work.
3. Run (same order as CI):
   - `cargo fmt --all -- --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - Optional: `./scripts/ci-local.sh` (also builds release)
4. Record pass/fail in the task file (short, repo-relative paths only).
5. **On pass:** rename `TEST-…` → `DONE-…` and move to `agents/tasks/done/`. Comment on the issue via `./scripts/gh-safe.sh` that tests passed. Leave the issue open for handoff.
6. **On fail:** rename back to `WIP-…`, note the failure, leave `agent:wip`. Do not close the issue.
