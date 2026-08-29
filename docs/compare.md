# Built-in 2-way compare

Date: 2026-08-29

## Why not only Linux `diff`?

npp-rs targets macOS, Linux, and Windows. System `diff` is not always present. The UI still needs parsed line tags for colours and sync scroll. This build uses an **in-process LCS** (`crates/app/src/diff.rs`).

## How to use

1. Open two tabs.
2. Put the second file in **Other View** (View → Move/Clone / dual view), or leave Compare to pick the other tab.
3. **View → Compare with Other View** — opens dual view, enables sync scroll, colours deletes (red) / inserts (green).
4. **View → Clear Compare** — removes colours.

Both panes stay editable. Compare colours do not refresh after you edit.

MVP limit: **3000 lines** per side.

## Not in MVP

- 3-way merge
- Char-level inline diff
- Shelling out to system `diff`
