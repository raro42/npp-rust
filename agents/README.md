# agents/

Agent-ops for **npp-rs**. Privacy-first (public repo).

## Standing rules (always)

1. **Always test** after a behaviour change: `./scripts/ci-local.sh` (or at least fmt + clippy + `cargo test --workspace`).
2. **Always watch CI** — step 005 / `scripts/ci-watch.py` (daily). Do not ignore a red `ci.yml` on `main`.
3. **Always skim local panic log** after a crash or odd run: `logs/panic.log` via [006-log-monitor/](006-log-monitor/).
4. **No dirty leftovers** — commit + push safe work; nightly backstop: `scripts/git_flush.py`.
5. **Read** [workspace/lessons.md](workspace/lessons.md) before coding; append when the operator corrects a pattern.

## Pipeline

| Step | Role | Entry |
|------|------|-------|
| 005 | CI watch | `005-ci-watcher.md` · `scripts/ci-watch.py` |
| 006 | Log scan | `006-log-monitor/` · `scripts/scan_panic_log.py` |
| 007 | Quality (weekly) | `007-quality/` · `scripts/scan_repo_quality.py` |
| 008 | Git flush (daily) | `scripts/git_flush.py` |
| 001 | Issue pickup | `001-issue-reviewer.md` · `issue_checker.py` |
| 002 | Coder | `002-coder.md` |
| 003 | Tester | `003-tester.md` |
| 004 | Handoff | `004-handoff.md` |

Loop: `./agents/npp-cursor-loop.sh once|loop` — see [docs/agent-loop.md](../docs/agent-loop.md).

## Layout

| Path | Purpose |
|------|---------|
| [tasks/](tasks/) | `FEAT-` / `WIP-` / `TEST-` queue; archive under `done/` |
| [workspace/](workspace/) | Session todo + lessons |
| [state/](state/) | Local stamps / agent lock (gitignored) |
| [006-log-monitor/](006-log-monitor/) | Panic / local log scan |
| [007-quality/](007-quality/) | Repo hygiene scan |

## Privacy

Never post home paths, secrets, or emails to GitHub. Use `./scripts/gh-safe.sh`. See `docs/security-public-repo.md`.
