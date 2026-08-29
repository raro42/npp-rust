# Issue #3 architecture finish (parallel)

Date: 2026-08-29  
Issue: https://github.com/raro42/npp-rust/issues/3

## Already done (do not redo)

- Dirty-close Save / Don’t Save / Cancel
- Commands split under `commands/`
- `ui_paint.rs` extract
- Path-keyed pending loads (DocumentId is owned by **issue #4 Agent A** — reuse, do not fork)

## Owned by issue #4 agents (do not duplicate)

| Item | Owner |
|------|--------|
| DocumentId + pending load by id | #4 Agent A |
| Read-only mutation gate | #4 Agent B |
| Saved revision / reload | #4 Agent C |
| Bookmark remap | #4 Agent D |
| Tail worker thread | #4 Agent E |

## Parallel ownership for #3 (only edit your rows)

| Agent | Owns | Deliverable |
|-------|------|-------------|
| F | `crates/fs` save path | **Done** — atomic save (temp sibling + `sync_all` + rename); no auto parent dirs; tests; std Windows replace |
| G | `crates/buffer` undo | **Done** — `with_transaction`; typing coalesce (kind/adjacency/time); indent/replace/join tests |
| H | `crates/fs` + `editor` open | **Done** — no `from_utf8_lossy` on open/tail; Windows-1252 + encoding metadata; status; tests |
| I | `.github/workflows/ci.yml` (+ toolchain if needed) | **Done** — `fmt --check` + `clippy -D warnings` on OS matrix; `rust-toolchain.toml` (stable + rustfmt/clippy) |
| J | `crates/highlight` + `editor` refresh | **Done** — linear byte→char; viewport window; `docs/highlight-viewport.md` |

Pull/rebase often. Small commits. Push `origin/dev`. Bump patch when user-visible. Update this checklist when done.

## Out of scope tonight (leave open on issue)

- Full `editor-core` / `editor-io` crate rename split (too large to finish safely beside #4)
- Complete public-field Document sealed API
- Full property/fuzz suite
