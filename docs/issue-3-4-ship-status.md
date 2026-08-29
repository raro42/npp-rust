# Issue #3 / #4 ship status

Date: 2026-08-29

## Done on `dev`

| Item | Status | Notes |
|------|--------|-------|
| #4 D bookmarks | Done | `LineStructureEdit` + remap via `prepare_edit` |
| #4 E tail worker | Done | `TailChannel` / `poll_tail_async`; UI apply-only |
| #3 G undo transactions | Done | `with_transaction` + typing coalesce |
| #3 H UTF-8 honesty | Done | no `from_utf8_lossy` on open/tail; encoding notes |
| #3 I CI | Done | `fmt --check` + `clippy -D warnings` |
| #3 F / J | Done | atomic save; highlight viewport |

## 0.2.11 fix-up

Tip had broken TailMsg test syntax and a dead `workspace_root` field that failed `clippy -D warnings`. This release repairs CI gates and marks Agent E checklist done.
