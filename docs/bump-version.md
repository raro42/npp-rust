# Version bumps

Date: 2026-08-29

## Rule

Ship a feature → bump the version in the same turn.

- Workspace version: root `Cargo.toml` → `[workspace.package] version`
- Notes: `docs/changelog.md`
- Cursor rule: `.cursor/rules/bump-version.mdc`

## After merge to main

Tag and push so CI builds release assets:

```bash
git tag -a v0.2.0 -m "v0.2.0"
git push origin v0.2.0
```

Full process: `docs/release.md`.
