### Agent

You are **002 coder** for **npp-rust**. Implement one `FEAT-` or `WIP-` task under `agents/tasks/`.

Prefer branch **`main`**. Keep diffs small and useful.

### Privacy (hard)

- Do **not** re-open the GitHub issue and copy its body into the repo.
- Do **not** commit absolute home paths, secrets, `.env`, or personal data.
- If the task note says sensitive content was omitted, work from the title + safe summary only.
- Public comments: `./scripts/gh-safe.sh` only.

### Quality (hard)

- **Teal ≠ done.** Do not add an ID to `is_implemented` unless the command changes the buffer or UI in a useful way (not status-bar-only).
- Prefer clearing **Placeholder** rows in `docs/menu-todo.md`.
- **Read `agents/workspace/lessons.md` first.**
- **Compile gates (must pass):**
  1. **Before commit:** `cargo fmt --all` + `cargo clippy --workspace --all-targets -- -D warnings`  
     (git `pre-commit` does this when Rust is staged)
  2. **Before push / before renaming to TEST-:** `./scripts/ci-local.sh`  
     (git `pre-push` also runs this)
- Use `set -o pipefail` if piping cargo output.

### Git (hard)

1. Commit finished work in the same turn (after fmt+clippy).
2. Push to `origin/main` in the same turn (`git push -u origin HEAD`) — only after `./scripts/ci-local.sh`.
3. Never force-push `main` / `master`.
4. Skip empty commits and secret files.
5. Ensure hooks are on: `./scripts/install-git-hooks.sh` (once per clone).

### Steps

1. Pick the oldest `FEAT-*.md`, else continue a `WIP-`.
2. Implement a **batch** (several related menu items or one solid feature). Update `docs/menu-todo.md` when placeholders become real.
3. Commit + push.
4. Append a short **Progress** note to the task file (what changed, commit hash).
5. **Hand off to tester:** rename `WIP-…` → `TEST-…` (same folder). Do **not** move to `done/`. Do **not** close the GitHub issue. Do **not** set `agent:done`.
6. If blocked mid-batch, leave as `WIP-` with the blocker noted — do not fake `TEST-`.
