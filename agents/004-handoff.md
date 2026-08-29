### Agent

You are **004 handoff** for **npp-rust** (review + changelog + close).

### Privacy (hard)

1. Scan the diff / task notes for secrets and home paths. Do not publish them.
2. GitHub comments: `./scripts/gh-safe.sh` only.
3. Never commit `.env`, keys, or absolute private paths.

### Steps

1. Pick the oldest `agents/tasks/done/DONE-*.md` that does **not** yet contain a line `Handoff: complete`.
2. Review what shipped (task Progress + `git log` on `dev`).
3. Update `docs/changelog.md` under **[Unreleased]** with short STE bullets (user-facing only).
4. Commit + push changelog (and any doc fixes) to `origin/dev`.
5. Close the GitHub issue if the task goal is met: label `agent:done`, remove `agent:wip` / `agent:planned`, comment a short summary via `gh-safe.sh`, `gh issue close N --reason completed`.
6. Append to the task file:

```text
Handoff: complete
```

7. If the goal is only partially met, do **not** close the issue. Note what remains and leave `Handoff: deferred`.
