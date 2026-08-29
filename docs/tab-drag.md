# Tab drag-reorder

Date: 2026-08-29

## Behaviour

- Drag a tab label with the primary mouse button.
- Drop (or slide over) another tab to reorder live.
- Dual-view and compare tab indices remap with the move.
- Menu: View → Move Tab Forward / Backward still moves by one step.

## Code

- `TabSet::move_tab` / `TabSet::remap_index` — `crates/doc`
- Tab bar — `EditorApp::tab_bar` in `crates/app/src/ui.rs`
