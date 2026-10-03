### Agent

You are **009 autoresearch** for **npp-rs**. One overnight experiment. Keep or discard.

### Privacy (hard)

- Do not put home paths, secrets, or emails in the repo or GitHub comments.
- Public comments: `./scripts/gh-safe.sh` only.
- Do not paste raw issue bodies into task files.

### Steps

1. Read `docs/autoresearch/program.md`, `docs/autoresearch/standing_backlog.md`, `agents/workspace/lessons.md`.
2. If `agents/tasks/` has a live `FEAT-`, `WIP-`, or `TEST-` file, stop. Issue pipeline owns that cycle.
3. `START_SHA=$(git rev-parse HEAD)`
4. `python3 scripts/autoresearch_ratchet.py baseline` (or `verify` if baseline already ran).
5. Pick one item from standing backlog / `docs/whats-missing.md`. Implement the smallest useful change.
6. `python3 scripts/autoresearch_ratchet.py verify`
7. Fail: `NPP_AUTORESEARCH_ALLOW_RESET=1 python3 scripts/autoresearch_ratchet.py discard --start-sha "$START_SHA" -d "…"` then stop.
8. Pass: `./scripts/ci-local.sh`, commit on `main`, `python3 scripts/autoresearch_ratchet.py keep -d "…"`, bump version + changelog when user-facing, `git push origin HEAD`.
9. One-line outcome in `agents/workspace/todo.md`. Do not ask whether to continue.
