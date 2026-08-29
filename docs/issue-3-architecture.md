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
| F | `crates/fs` save path | Atomic save: temp sibling + flush + rename; tests; Windows-safe as far as std allows |
| G | `crates/buffer` undo | Transactional undo (one user command = one undo); typing coalesce policy; tests |
| H | `crates/fs` + `editor` open | Invalid UTF-8: no silent lossy permanent corruption — detect/reject or keep encoding metadata; honest status |
| I | `.github/workflows/ci.yml` (+ toolchain if needed) | `fmt --check`, `clippy -D warnings` (fix warnings first if needed in small commits), keep OS matrix |
| J | `crates/highlight` + `editor` refresh | Viewport-oriented highlight / avoid per-span quadratic `chars().count`; tests or bench note |

Pull/rebase often. Small commits. Push `origin/dev`. Bump patch when user-visible. Update this checklist when done.

## Out of scope tonight (leave open on issue)

- Full `editor-core` / `editor-io` crate rename split (too large to finish safely beside #4)
- Complete public-field Document sealed API
- Full property/fuzz suite
