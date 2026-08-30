# Agent notes (npp-rs)

- Branch: **`main`** only (no `dev` branch).
- Version + push + daily release: `docs/release.md`
- Agent loop: `docs/agent-loop.md`
- Inspiration map: `docs/agent-loop-mac-stats-inspiration.md`
- Standing rules: `agents/README.md`
- Pipeline: **005 CI → 006 logs → 007 quality → 008 git flush → 001 pickup → 002 coder → 003 tester → 004 handoff**
- Local CI: `./scripts/ci-local.sh`
- Privacy: `docs/security-public-repo.md` · `./scripts/gh-safe.sh`
- Issue tasks: `python3 agents/issue_checker.py`
- Disk hygiene: `./scripts/daily-clean.sh`
- Long run: `agents/start-unattended.command`
