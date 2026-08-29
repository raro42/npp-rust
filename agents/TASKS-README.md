# Agent tasks (npp-rust)

## Queues

| Prefix | Meaning | Next agent |
|--------|---------|------------|
| `FEAT-` | Planned from a GitHub issue (sanitized summary only) | `002` coder |
| `WIP-` | Coding in progress | `002` coder |
| `TEST-` | Ready for verification | `003` tester |
| `DONE-` | Tests passed; lives under `done/` | `004` handoff |

## Pipeline (loop)

Each cycle (`./agents/npp-cursor-loop.sh once` or `loop`):

1. **001** — pick up issues → `FEAT-`
2. **004** — finish any pending handoff (`DONE-` without `Handoff: complete`)
3. **003** — test any `TEST-`
4. **002** — code oldest `FEAT-` / `WIP-` → leave as `TEST-` when the batch is ready
5. **003** again — catch a fresh `TEST-` from this cycle
6. **004** again — changelog + close issue when tests passed

Coder must **not** close issues or skip to `done/`. Tester must **not** close issues. Handoff updates `docs/changelog.md` and closes.

## Privacy (mandatory)

1. Task files must **never** contain raw issue bodies, secrets, home paths, or emails.
2. Create FEAT files only via `python3 agents/issue_checker.py` (or agent 001 following the same rules).
3. GitHub comments: `./scripts/gh-safe.sh` only.
4. See `docs/security-public-repo.md` and `.cursor/rules/public-repo-no-exfiltration.mdc`.

## Naming

`FEAT-<issue>-YYYYMMDD-HHMM-<slug>.md` (UTC). Same for `WIP-` / `TEST-` / `DONE-`.
