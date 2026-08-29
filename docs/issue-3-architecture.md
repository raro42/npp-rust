# Issue #3 architecture finish

Date: 2026-08-29  
Issue: https://github.com/raro42/npp-rust/issues/3  
Status: **In-scope items done** (2026-08-29). Close issue; out-of-scope follow-ups stay in `docs/whats-missing.md`.

## Shipped

| Item | Status |
|------|--------|
| Atomic save | Done |
| Transactional undo | Done |
| Honest UTF-8 / Windows-1252 open+tail | Done |
| CI fmt + clippy | Done |
| Highlight viewport / linear byte→char | Done |
| Dirty-close, command split, ui_paint | Done earlier |

DocumentId, read-only gate, reload, bookmarks, tail worker: shipped under issue #4.

## Explicitly out of scope (not blockers for closing #3)

- Full `editor-core` / `editor-io` crate rename
- Fully sealed Document public fields
- Property / fuzz suite
