# npp-rs overnight autoresearch

Karpathy-style ratchet for **npp-rs** (Rust / egui editor), not LLM `train.py`.
You program research by editing **this file**. The agent executes the loop. Humans own the strategy.

Inspired by [karpathy/autoresearch](https://github.com/karpathy/autoresearch) and [mac-stats `docs/autoresearch/`](https://github.com/raro42/mac-stats/tree/main/docs/autoresearch). This track improves **shipped editor behavior**.

## Goal (non-negotiable)

**Every overnight window must strengthen the product.** Quiet ticks are failure mode, not success.
A night that only appends “Quiet tick” is a **loss**.

Issue work still wins. If `agents/tasks/` has a live `FEAT-`, `WIP-`, or `TEST-` file, the main loop codes that first. Autoresearch runs when the queue is idle (or `AGENT_AUTORESEARCH_FORCE=1`).

## Window

- **Active:** 20:00–06:00 local only (host clock).
- **Harness:** `./agents/npp-cursor-loop.sh` step **009** must spawn `cursor-agent` (not print-only).
- **Quiet daytime:** do not run 009 unless forced.
- Finish the current experiment (keep or discard) before the next tick.
- **No dirty leftovers:** commit + push keep work. Daily git flush (step 008) is the backstop.
- Public repo: no home paths, secrets, or emails in commits or GitHub comments.

## Nightly minimum

Inside each 20:00–06:00 window:

1. **At least one** real experiment must land a row in the local `results.tsv` (`keep` or `discard`).
2. Docs-only or clippy nits do **not** satisfy the minimum by themselves.
3. Cap: at most **one** quiet tick per night. After that, pick from [standing_backlog.md](standing_backlog.md).

Results file (untracked, XDG state): `npp-rs/autoresearch/results.tsv` under `$XDG_STATE_HOME` (default: user state dir). Do not commit it.

## Immutable vs editable

**Do not weaken:**

- `scripts/autoresearch_ratchet.py` keep/discard gate
- `./scripts/ci-local.sh` as the push gate
- Secrets / `.env`
- Force-push to `main`

**Prefer one surface per experiment:** one command family under `crates/app/src/commands/`, one editor behavior, or one honest gap from `docs/whats-missing.md`.

## Metric (binary keep/discard)

Primary gate:

```bash
python3 scripts/autoresearch_ratchet.py verify
```

**Keep** only if:

1. `verify` exits 0,
2. The change addresses a real candidate (Idea priority),
3. You can state the user-facing fitness in one line.

Then run `./scripts/ci-local.sh` before push. Bump `[workspace.package] version` and `docs/changelog.md` when behavior ships.

**Discard** (reset) if verify fails, the change is filler, or you cannot name the improvement.

```bash
START_SHA=$(git rev-parse HEAD)
# … experiment …
python3 scripts/autoresearch_ratchet.py discard --start-sha "$START_SHA" --description "…"
# or on success (after commit):
python3 scripts/autoresearch_ratchet.py keep --description "…"
```

Reset only runs when `NPP_AUTORESEARCH_ALLOW_RESET=1` (the loop sets this for step 009).

## Experiment loop (each 009 tick)

1. Read this file, `docs/autoresearch/standing_backlog.md`, `agents/workspace/lessons.md`, `docs/whats-missing.md`, `docs/next-gaps.md`.
2. Skim `logs/panic.log` via `python3 scripts/scan_panic_log.py` (do not paste home paths).
3. Pick fuel in Idea priority. Do not default to quiet when the standing backlog has work.
4. Record `START_SHA=$(git rev-parse HEAD)`. Implement the smallest change that could help.
5. `python3 scripts/autoresearch_ratchet.py verify`.
6. **Fail** → `discard --start-sha …`. Stop this tick.
7. **Pass** → `./scripts/ci-local.sh`, commit, `keep --description …`, version + changelog when shipping, `git push origin HEAD`.
8. Append a one-line outcome to `agents/workspace/todo.md` (no private paths).

## Idea priority

1. Open GitHub issues / live `FEAT-` `WIP-` `TEST-` (the issue pipeline, not 009)
2. Panic / log signatures (`scripts/scan_panic_log.py`)
3. Standing backlog top item
4. Honest partials in `docs/whats-missing.md` (make one behavior less shallow)
5. Gaps in `docs/gap-analysis-vs-npp.md` that fit a small batch
6. Skip as the night’s only win: docs-only churn, version bumps without a fix

## Simplicity

A tiny gain that adds ugly complexity → discard. Deleting code with equal or better verify → keep.

## Output

- One experiment per tick max. At least one keep or discard per overnight window.
- `results.tsv` rows for every keep / discard / crash.
- Do not ask the human whether to continue overnight.
