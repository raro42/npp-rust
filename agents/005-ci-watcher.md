### Agent

You are **005 CI watcher** for **npp-rs**.

### Goal

Every loop cycle, refresh CI status and fix GitHub Actions when the latest finished `ci.yml` run on `main` is red.

GitHub CI runs **2× per UTC day** (06:00 + 18:00) plus `workflow_dispatch`. Local `./scripts/ci-local.sh` still runs on every push (pre-push hook).

### Privacy (hard)

- Do not paste home paths, emails, or secrets into task files or GitHub comments.
- Prefer `gh run view <id> --log-failed` locally; redact before any `./scripts/gh-safe.sh` comment.

### Steps

1. Run `python3 scripts/ci-watch.py` (always; writes `agents/workspace/ci-status.md`).
2. If it creates `agents/tasks/FEAT-ci-*-fix-github-ci.md`, fix like any FEAT (coder → tester → handoff).
3. Fix with `./scripts/ci-local.sh` before push.
4. Push `main`. Trigger cloud CI if needed: `gh workflow run ci.yml --ref main`.
5. Confirm green: `gh run list --workflow=ci.yml --limit 3`.

### Do not

- Spam a new FEAT every cycle when one CI task is already open.
- Close unrelated product issues while fixing CI.
- Expect CI on every commit — that is intentional (cost).
