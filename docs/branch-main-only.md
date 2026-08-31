# Branch policy: main only

Date: 2026-08-30

Day-to-day work is on **`main`**. The long-lived **`dev`** branch is removed.

## Why

CI ran on every push to both `main` and `dev`. That doubled compile time on GitHub Actions.

## Workflow

1. Commit on `main`.
2. `git push origin main`.
3. Tag releases from `main` (`vX.Y.Z`).

Agent loop syncs `main`. Cloud CI runs **twice per UTC day** (+ manual dispatch), not on every push. Local `ci-local` still runs on push via hooks.
