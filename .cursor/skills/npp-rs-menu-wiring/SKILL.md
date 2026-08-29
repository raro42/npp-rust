---
name: npp-rs-menu-wiring
description: >-
  Wire Notepad++ IDM_* menu commands in npp-rs. Use when implementing menu
  stubs, is_implemented, or Coming Soon vs Handled dispatch.
---

# npp-rs menu wiring

## Rules

1. Edit `crates/app/src/commands.rs` (`dispatch` + `is_implemented`) together.
2. Prefer a small real behavior over a fake “done” stub.
3. Teal menus = `is_implemented` true. Keep that list in sync.
4. Public repo: no private paths/secrets in status or commits.
5. Regenerate `docs/menu-todo.md` after a batch.

## Patterns

| Kind | Approach |
|------|----------|
| Open URL / browser | `open_url` |
| Hash tools | shell `shasum` / `md5` on selection or file |
| Tab order | mutate `TabSet` |
| Complex UI | keep Coming Soon |

## Do not

Duplicate rust-skills outside `.cursor/skills/rust-skills/` (that path is the in-repo source of truth).
