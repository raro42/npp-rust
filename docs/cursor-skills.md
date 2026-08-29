# Cursor skills for this repo

Date: 2026-08-28

## Why skills live under `.cursor/` here

Cursor loads project skills and rules from **`.cursor/`** in the repo. Putting **rust-skills** here means:

- Every clone of npp-rust gets the same Rust guidance
- Agents do not depend on a one-off install under `~/.cursor/`
- Reviews stay consistent across machines

Trade-off: the tree is larger (~265 small markdown files). That is acceptable for this project.

## Installed in-repo

| What | Path |
|------|------|
| rust-skills (265 rules, MIT) | `.cursor/skills/rust-skills/` |
| Rule (applies to `**/*.rs`) | `.cursor/rules/rust-idioms.mdc` |
| npp-rs menu wiring | `.cursor/skills/npp-rs-menu-wiring/` |

Upstream: https://github.com/leonardomso/rust-skills (Leonardo Maldonado, MIT). See `.cursor/skills/rust-skills/LICENSE`.

## Update

```bash
git clone --depth 1 https://github.com/leonardomso/rust-skills.git /tmp/rust-skills
rm -rf .cursor/skills/rust-skills
mkdir -p .cursor/skills/rust-skills
cp /tmp/rust-skills/SKILL.md /tmp/rust-skills/LICENSE .cursor/skills/rust-skills/
cp -R /tmp/rust-skills/rules .cursor/skills/rust-skills/rules
```
