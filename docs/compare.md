# Built-in 2-way compare

Date: 2026-08-30

## Why not only Linux `diff`?

npp-rs targets macOS, Linux, and Windows. System `diff` is not always present. The UI still needs parsed line tags for colours and sync scroll. This build uses an **in-process LCS** (`crates/app/src/diff.rs`).

## How to choose the two files

Compare uses **two open tabs**. Left pane = active tab. Right pane = resolved partner:

| Priority | Right side |
|----------|------------|
| 1 | Marked partner (⌘/Ctrl-click a tab, or tab context menu) |
| 2 | Other-view tab when dual view already shows a different file |
| 3 | Tab immediately to the **right** of the active tab |
| 4 | If active is last: tab to the **left** |

### Fast path

1. Open two (or more) files.
2. Select the left file.
3. **View → Compare with Other View** — compares against the tab to the right.

### Pick any second tab

1. Select the left file.
2. **⌘-click** (macOS) or **Ctrl-click** the other tab — it shows `⇄` and becomes the partner.
3. **View → Compare with Other View**.

Or right-click the other tab → **Compare with this tab** / **Mark for compare**.

### Dual-view path (still works)

1. Open both files.
2. **View → Move to Other View** on the right-hand file.
3. Activate the left file.
4. **View → Compare with Other View**.

Status line shows: `Compare “dummy.log” | “dummy.log.2” (−N +M)`.

**View → Clear Compare** removes colours.

While compare is on, panes stay pinned to that pair (tab clicks do not swap the left file away).

Closing a tab that is **not** in the pair remaps both sides so Compare stays on. Closing either compared tab clears Compare.

### Jump between differences

- Starting Compare parks **both** panes on the **first** change hunk (same ordinal; status shows hunk count).
- **View → Next Difference** / **Previous Difference** (or **F7** / **Shift+F7**)
- **View → First Difference** / **Last Difference** (or **⌘/Ctrl+F7** / **⌘/Ctrl+Shift+F7**)
- Moves the caret on the focused pane to the start of the next/previous change hunk (wraps), or jumps to the first/last hunk.
- The other pane parks on the **same hunk ordinal** (line numbers may differ when sides disagree).
- Mid-hunk Next skips to the following hunk, not the next red/green line.
- Status shows both sides’ lines + hunk ordinal, e.g. `Compare Next difference → L12 | R15 (2/5)`.
- Click a red/green change line, or move the caret onto one with the keyboard, to park the **other** pane on the same hunk (status: `Compare hunk → L12 | R15 (2/5)`). Equal lines leave the other pane alone.
- When both sides match (including ignore-whitespace / ignore-case), status says `(identical)`.
- Active ignore options appear in the status (`· ignore ws`, `· ignore case`, or `· ignore ws+case`).
- **View → Ignore Whitespace Differences** / **Ignore Case Differences** toggle those Preferences keys (✓ when on) and re-diff immediately while Compare is on.
- **View → Swap Compare Sides** flips left/right files (and scroll), keeps focus on the new left, and re-diffs so delete/insert colours stay correct.

## Limits

- Both panes stay editable. Line tags refresh after edits (~200 ms debounce).
- MVP max: **3000 lines** per side.
- No gap rows for inserts (line numbers stay per-file; sync is by scroll line).
- Start Compare turns on sync H + V scroll.
- Preferences: **Ignore whitespace differences** collapses whitespace runs before LCS.
- Preferences: **Ignore case differences** folds letter case before LCS (combines with ignore-whitespace).

## Not in MVP

- 3-way merge
- Char-level inline diff
- Shelling out to system `diff`

## Code

- `pick_compare_right` / `start_compare` — `crates/app/src/ui.rs`
- `compare_stale` + `refresh_compare_if_stale` — `crates/app/src/editor.rs`, `crates/app/src/ui.rs`
- Line LCS — `crates/app/src/diff.rs`
