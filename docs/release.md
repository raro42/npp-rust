# Release process

Date: 2026-08-28  
Repo: [raro42/npp-rust](https://github.com/raro42/npp-rust)

## Cadence

| Habit | Rule |
|-------|------|
| Version | Semver in root `Cargo.toml`; tag `vX.Y.Z` |
| Push `main` | At least every 12 hours when work exists |
| GitHub Release | At least once per active day |
| Disk | `./scripts/daily-clean.sh` once per day |

## CI builds (Linux + Windows + macOS)

- **CI** (`.github/workflows/ci.yml`): test + release build on Ubuntu, Windows, macOS.
- **Release** (`.github/workflows/release.yml`): on tag `v*`, build binaries and attach them to a GitHub Release.

Yes — GitHub Actions compiles for **Linux** and **Windows** (and macOS).

## Ship a release

```bash
# 1. Bump version in Cargo.toml + docs/changelog.md
# 2. Commit on dev, merge to main
git checkout main && git merge --ff-only dev
git push origin main

# 3. Tag and push (starts release workflow)
git tag -a v0.1.0 -m "v0.1.0"
git push origin v0.1.0
```

Watch: Actions → Release. Assets appear on the GitHub Releases page.

## Local artifacts

Do not stockpile. Prefer CI assets. Clean with `./scripts/daily-clean.sh`.
