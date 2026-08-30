# Quality monitor (weekly)

Standing role: catch repo clutter and dead scaffolding before a human asks.

## Schedule

At most **once per UTC week** from the agent loop (step 007). Force with `AGENT_QUALITY_FORCE=1`.

## Entrypoints

| Item | Path |
|------|------|
| Scanner | `python3 scripts/scan_repo_quality.py` |
| Prompt | [PROMPT.md](PROMPT.md) |

## Expectation

Fix fails (or open a FEAT). Re-scan until exit 0 or only documented warns remain.
