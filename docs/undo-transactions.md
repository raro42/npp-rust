# Undo transactions

Date: 2026-08-29  
Owner: issue #3 Agent G (`crates/buffer`)

## Model

One user-level command is one undo unit. Helpers that touch many places call `TextBuffer::with_transaction`. Nested calls merge into the outermost unit.

## Typing coalesce

Plain typing merges into the previous unit when all hold:

| Rule | Meaning |
|------|---------|
| Kind | Previous unit is a single insert |
| Adjacency | New text starts at `last_insert_end` |
| Time | Previous insert within 1s (`TYPING_COALESCE_MS`) |

Caret moves, selection changes, undo/redo, `replace_document`, and starting a transaction break the streak.

## Tests

- `indent_multiline_one_undo`
- `replace_selection_one_undo`
- `join_lines_one_undo`
- `typing_coalesce_one_undo`
