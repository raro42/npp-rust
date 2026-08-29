# Issue #4 stability finish (parallel)

Date: 2026-08-29  
Issue: https://github.com/raro42/npp-rust/issues/4

## Already done (do not redo)

- Dirty-close Save / Don’t Save / Cancel
- Path-keyed pending loads (upgraded to DocumentId — Agent A)
- Tail refuse/suspend when dirty (MVP)
- Command split under `commands/`
- Partial UTF-8 tail carry
- **Agent E:** Tail reads on worker (`TailChannel` / `poll_tail_async`); GUI `poll_tail` applies `TailMsg` only

## Parallel ownership (only edit your rows)

| Agent | Owns | Deliverable |
|-------|------|-------------|
| A | `crates/doc` DocumentId; `editor.rs` PendingLoad / apply / close | Stable id on Document + pending load by id+path; tests |
| B | `crates/buffer` + thin `doc` edit gate; `commands/edit.rs` / `format.rs` / `misc.rs` mutate paths | Read-only cannot mutate via menu; buffer API returns Err |
| C | `editor.rs` reload + `dirty`/`saved_generation` on Document | Reload replaces content with confirm if dirty; undo-to-saved clears dirty |
| D | `buffer` edit hooks + `Document.bookmarks` | **Done** — `LineStructureEdit` on buffer; bookmarks remap on insert/delete; tests |
| E | `crates/fs` + `editor.rs` poll_tail | **Done** — `TailChannel` / `poll_tail_async`; UI applies `TailMsg` only |

Commit and push each batch. Bump patch version when user-visible. Update this file’s checklist when done.

## Checklist

### Agent A — DocumentId pending loads

- [x] `DocumentId` (u64) on `Document`; assign on create via `TabSet::alloc_id`
- [x] `TabSet::{index_of_id,get_by_id,get_mut_by_id}`
- [x] `PendingLoad` stores `document_id` + `path`
- [x] `apply_open_result` / poll fail: apply only if id exists and is still loading for that path
- [x] Tab close cancels pending by id/path
- [x] Unit tests: reorder then apply; close then apply; not-loading then apply
- [x] Patch version + changelog

### Agent B — read-only edit gate

- [ ] (other agent)

### Agent C — reload / dirty generation

- [ ] (other agent)

### Agent D — bookmarks shift

- [x] `LineStructureEdit` recorded in buffer `delete_range` / `apply_insert`
- [x] `Document` consumes via `take_line_structure_edit` in `mark_text_changed`
- [x] Snap delete heuristic removes bookmark on deleted line
- [x] Unit tests: insert shift, delete-above shift, delete mark line, hook path
- [x] Patch version + changelog

### Agent E — tail worker

- [x] `TailChannel` / `TailMsg` / `TailOutcome` in `crates/fs`
- [x] `poll_tail_async` worker; rotate reload on worker
- [x] `EditorState::poll_tail` schedules + `drain_tail_msgs` / `apply_tail_msg` only (no sync disk on UI)
- [x] Dirty/suspend policy kept
- [x] Unit tests for async append
- [x] Patch version + changelog

## Agent E leftover limits

- `enable_tail_follow` still calls `file_size` on the GUI thread (one-shot user action).
- One worker thread per in-flight path (not a shared pool).
- Append still capped at `TAIL_MAX_CHUNK` (1 MiB); rotate reload reads the whole file on the worker.

