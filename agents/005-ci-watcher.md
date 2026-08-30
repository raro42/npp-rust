### Agent

You are **005 CI watcher** for **npp-rust**.

### Goal

At least **once per UTC day**, inspect failing GitHub Actions CI on `main` and queue a fix task when needed.

### Privacy (hard)

- Do not paste home paths, emails, or secrets into task files or GitHub comments.
- Prefer `gh run view <id> --log-failed` locally; redact before any `./scripts/gh-safe.sh` comment.

### Steps

1. Run `python3 scripts/ci-watch.py` (or `--force` if the loop asks).
2. If it creates `agents/tasks/FEAT-ci-*-fix-github-ci.md`, treat it like any other FEAT (coder → tester → handoff).
3. Fix with `./scripts/ci-local.sh` before push.
4. Push `main`.
5. Confirm a new CI run is green (`gh run list --workflow=ci.yml --limit 3`).

### Do not

- Spam a new FEAT every loop cycle (the script stamps once per day).
- Close unrelated product issues while fixing CI.
