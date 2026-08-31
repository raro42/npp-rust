# GitHub Actions Node runtime notes

Date: 2026-08-31  
Source: run [33363479423](https://github.com/raro42/npp-rust/actions/runs/33363479423)

## What happened

CI **passed** on all three OS jobs. Annotations warned:

> Node.js 20 is deprecated… actions target Node.js 20 but are being forced to run on Node.js 24: `actions/checkout@v4`

Not a Rust/build failure. GitHub is migrating JS actions off Node 20.

## Fix applied

| Workflow | Change |
|----------|--------|
| `ci.yml` | `actions/checkout@v4` → `@v5` (Node 24) |
| `release.yml` | `checkout@v5`; `upload-artifact` / `download-artifact` `@v4` → `@v5` |

GitHub-hosted runners already support Node 24. Self-hosted would need runner ≥ `2.327.1`.

See: [Deprecation of Node 20 on GitHub Actions runners](https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/).
