# Agent tasks (npp-rust)

## Queues

| Prefix | Meaning | Next agent |
|--------|---------|------------|
| `FEAT-` | Planned from a GitHub issue (sanitized summary only) | `002` coder |
| `WIP-` | Work in progress | `002` / `003` |
| `TEST-` | Ready for verification | `003` tester |
| `DONE-` | Finished (move under `done/`) | `040` committer |

## Privacy (mandatory)

1. Task files must **never** contain raw issue bodies, secrets, home paths, or emails.
2. Create FEAT files only via `python3 agents/issue_checker.py` (or agent 001 following the same rules).
3. GitHub comments: `./scripts/gh-safe.sh` only.
4. See `docs/security-public-repo.md` and `.cursor/rules/public-repo-no-exfiltration.mdc`.

## Naming

`FEAT-<issue>-YYYYMMDD-HHMM-<slug>.md` (UTC).
