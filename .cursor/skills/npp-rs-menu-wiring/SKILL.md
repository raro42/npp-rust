---
name: npp-rs-menu-wiring
description: >-
  Wire Notepad++ IDM_* menu commands in npp-rs. Use when implementing menu
  stubs, is_implemented, or Coming Soon vs Handled dispatch.
---

# npp-rs menu wiring

## Rules

1. Edit the **domain file** under `crates/app/src/commands/` (see `docs/agent-parallel.md`).
2. Keep `is_implemented` in `commands/mod.rs` in sync (teal labels).
3. Prefer a small real behavior over a fake “done” stub.
4. Public repo: no private paths/secrets in status or commits.
5. Regenerate `docs/menu-todo.md` after a batch.

## Domain map

| File | Area |
|------|------|
| `file.rs` | File |
| `edit.rs` | Edit |
| `search.rs` | Search |
| `view.rs` | View |
| `format.rs` | Encoding / EOL |
| `common.rs` | Shared helpers |

## Patterns

| Kind | Approach |
|------|----------|
| Open URL / browser | `common::open_url` |
| Hash tools | shell `shasum` / `md5` on selection or file |
| Tab order | mutate `TabSet` |
| Complex UI | keep Coming Soon |

## Do not

Duplicate rust-skills outside `.cursor/skills/rust-skills/` (that path is the in-repo source of truth).
