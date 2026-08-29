# Issue #4 stability finish (parallel)

Date: 2026-08-29  
Issue: https://github.com/raro42/npp-rust/issues/4

## Already done (do not redo)

- Dirty-close Save / Don’t Save / Cancel
- Path-keyed pending loads (upgrade to DocumentId still needed)
- Tail refuse/suspend when dirty (MVP)
- Command split under `commands/`
- Partial UTF-8 tail carry

## Parallel ownership (only edit your rows)

| Agent | Owns | Deliverable |
|-------|------|-------------|
| A | `crates/doc` DocumentId; `editor.rs` PendingLoad / apply / close | Stable id on Document + pending load by id+path; tests |
| B | `crates/buffer` + thin `doc` edit gate; `commands/edit.rs` / `format.rs` / `misc.rs` mutate paths | Read-only cannot mutate via menu; buffer API returns Err |
| C | `editor.rs` reload + `dirty`/`saved_generation` on Document | Reload replaces content with confirm if dirty; undo-to-saved clears dirty |
| D | `buffer` edit hooks + `Document.bookmarks` | Bookmarks shift on insert/delete lines |
| E | `crates/fs` + `editor.rs` poll_tail | Tail reads on worker thread; UI applies TailRead events |

Commit and push each batch. Bump patch version when user-visible. Update this file’s checklist when done.
