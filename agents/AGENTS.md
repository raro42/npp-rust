# Agent notes (npp-rs)

- Branch: **`dev`** for ongoing work; `main` for stable.
- Version + push + daily release: `docs/release.md`
- Agent loop: `docs/agent-loop.md`
- Pipeline: **001 pickup → 002 coder → 003 tester → 004 handoff** (changelog + close)
- Public-repo privacy: `docs/security-public-repo.md`
- Never post private data to GitHub. Use `./scripts/gh-safe.sh`.
- Issue tasks: `python3 agents/issue_checker.py`
- Disk hygiene: `./scripts/daily-clean.sh`
- Long run: `agents/start-unattended.command` (Terminal, not IDE shell)
