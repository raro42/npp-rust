# Highlight cost (Agent J / issue #3)

Date: 2026-08-29

## Problem

1. Each highlight span did `source[..byte].chars().count()`.
2. That rescanned from the start for every span (quadratic on long files).
3. Refresh always used the first 512 KiB, even when the viewport was far below.

## Approach

### Byte → char (one pass)

`crates/highlight` collects tree-sitter byte ranges, then converts them with a monotonic `ByteCharCursor`. Cost is linear in source length plus span count.

Spans stay in **char** offsets. The paint path (`ui_paint::paint_line_text`) already uses absolute char indices.

### Viewport window

`EditorState::refresh_highlight_if_needed(view_first_line)`:

| Case | Behavior |
|------|----------|
| File fits in 512 KiB from line 0 | Whole file; cover `(0, line_count)` |
| Large file | Window around `view_first_line` (±64 lines, ≥96 ahead, ≤512 KiB) |
| Scroll still inside cover (inner margin) | No recompute |
| Scroll leaves cover | New pass; spans get `start_char` base so offsets stay absolute |

UI passes `scroll_line` from `ui.rs`.

## Leftover limits

- Window is line-based from primary scroll only (not dual-view other pane).
- Hidden-fold display rows vs buffer lines: cover uses buffer line indices via `scroll_line` on the visible row list (same as paint).
- Tree-sitter still parses the window string each dirty pass (no incremental parse).
- Secondary dual-view pane does not keep its own highlight cache.
