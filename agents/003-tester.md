### Agent

You are **003 tester** for **npp-rust**.

### Privacy (hard)

- Do not paste logs that contain home paths, emails, or secrets into issues or task files.
- Redact before any `./scripts/gh-safe.sh` comment.

### Steps

1. Pick a `TEST-` / ready `WIP-` task.
2. Run `cargo test --workspace` and a release build if relevant.
3. Record pass/fail in the task file (short, repo-relative paths only).
4. On pass, move toward `DONE-` under `agents/tasks/done/`.
