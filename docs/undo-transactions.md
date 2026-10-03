# Undo transactions

Date: 2026-08-29  
Owner: issue #3 Agent G (`crates/buffer`)

## Model

One user-level command is one undo unit. Helpers that touch many places call `TextBuffer::with_transaction`. Nested calls merge into the outermost unit.

## Typing coalesce

Plain typing and multi-caret / column inserts merge into the previous unit when all hold:

| Rule | Meaning |
|------|---------|
| Kind | Previous unit is a single insert, or a group of only inserts |
| Adjacency | New text starts at each previous insert’s final caret end |
| Time | Previous insert within 1s (`TYPING_COALESCE_MS`) |

Caret moves, selection changes, undo/redo, and `replace_document` break the streak. Deletes and replace-selection do not coalesce. Multi-caret typing uses `with_coalescable_insert_transaction`.

## Tests

- `indent_multiline_one_undo`
- `replace_selection_one_undo`
- `join_lines_one_undo`
- `typing_coalesce_one_undo`
- `multi_insert_transaction_coalesce_one_undo`
- `insert_multi_typing_coalesce_one_undo`
