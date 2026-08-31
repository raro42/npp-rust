# GitHub Actions Node runtime notes

Date: 2026-08-31  
Source: run [33363479423](https://github.com/raro42/npp-rust/actions/runs/33363479423)

## What happened

CI **passed**. Annotations warned that `actions/checkout@v4` targeted Node 20 while runners force Node 24.

## Fixes

1. Bump to `checkout@v5` / artifact `@v5` (Node 24).
2. Cloud CI is **not** on every push: schedule **06:00 + 18:00 UTC** + `workflow_dispatch` (cost).
3. Agent loop **005** refreshes `agents/workspace/ci-status.md` every cycle and queues a FEAT when the latest finished run is red.

Local gate unchanged: pre-push `./scripts/ci-local.sh`.

Manual cloud run: `gh workflow run ci.yml --ref main`

See: [Deprecation of Node 20 on GitHub Actions runners](https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/).
