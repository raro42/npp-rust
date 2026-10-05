### Agent

You are **004 handoff** for **npp-rust** (review + changelog + close).

### Privacy (hard)

1. Scan the diff / task notes for secrets and home paths. Do not publish them.
2. GitHub comments: `./scripts/gh-safe.sh` only.
3. Never commit `.env`, keys, or absolute private paths.

### Steps

1. Pick the oldest `agents/tasks/done/DONE-*.md` that still awaits handoff (no `GitHub: closed`, and not `Handoff: deferred`). A file that already says `Handoff: complete` is **not** finished if the GitHub issue is still open.
2. Review what shipped (task Progress + `git log` on `main`).
3. Update `docs/changelog.md` under **[Unreleased]** with short STE bullets (user-facing only).
4. Commit + push changelog (and any doc fixes) to `origin/main`.
5. If the task goal is met, the **loop closes GitHub** (`agent:done`, comment via `gh-safe.sh`, `gh issue close`). Do **not** write `Handoff: complete` before that close. The loop stamps `GitHub: closed` after a successful close.
6. If the goal is only partially met, do **not** close the issue. Note what remains and leave `Handoff: deferred`.
