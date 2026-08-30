# Agent notes (npp-rs)

- Branch: **`dev`** for ongoing work; `main` for stable.
- Version + push + daily release: `docs/release.md`
- Agent loop: `docs/agent-loop.md`
- Pipeline: **005 CI watch → 001 pickup → 002 coder → 003 tester → 004 handoff**
- CI daily watch: `python3 scripts/ci-watch.py` / loop step `005`
- Local CI gates: `./scripts/ci-local.sh`
- Public-repo privacy: `docs/security-public-repo.md`
- Never post private data to GitHub. Use `./scripts/gh-safe.sh`.
- Issue tasks: `python3 agents/issue_checker.py`
- Disk hygiene: `./scripts/daily-clean.sh`
- Long run: `agents/start-unattended.command` (Terminal, not IDE shell)
