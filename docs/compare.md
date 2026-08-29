# Built-in 2-way compare

Date: 2026-08-29

## Why not only Linux `diff`?

npp-rs targets macOS, Linux, and Windows. System `diff` is not always present. The UI still needs parsed line tags for colours and sync scroll. This build uses an **in-process LCS** (`crates/app/src/diff.rs`).

## How to choose the two files

Compare uses **two open tabs**:

| Pane | Source |
|------|--------|
| Left (main) | Active tab when you start Compare |
| Right (Other view) | Other-view tab |

### Example: `dummy.log` vs `dummy.log.2`

1. Open both files.
2. Click **`dummy.log.2`**.
3. **View → Move to Other View** (puts that file on the right; leaves the other file active).
4. Click **`dummy.log`** if it is not already the active tab.
5. **View → Compare with Other View**.

Or with only those two tabs: select `dummy.log`, then Compare — the other tab becomes the right side.

Status line shows: `Compare “dummy.log” | “dummy.log.2” (−N +M)`.

**View → Clear Compare** removes colours.

While compare is on, panes stay pinned to that pair (tab clicks do not swap the left file away).

## Limits

- Both panes stay editable. Line tags refresh after edits (~200 ms debounce).
- MVP max: **3000 lines** per side.
- No gap rows for inserts (line numbers stay per-file; sync is by scroll line).
- Start Compare turns on sync H + V scroll.
- Preferences: **Ignore whitespace differences** collapses whitespace runs before LCS.

## Not in MVP

- 3-way merge
- Char-level inline diff
- Shelling out to system `diff`

## Code

- `compare_stale` + `refresh_compare_if_stale` — `crates/app/src/editor.rs`, `crates/app/src/ui.rs`
- Line LCS — `crates/app/src/diff.rs`
