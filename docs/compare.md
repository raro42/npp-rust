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

### Against last save

1. Open a file from disk (and edit it if you want).
2. **View → Compare to Saved** — opens (or refreshes) a read-only `name (saved)` snapshot of the on-disk bytes and starts Compare against it.
3. Untitled tabs need a path first (**File → Save**).

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

**View → Clear Compare** removes colours. If a Compare-to-Saved `name (saved)` snapshot was in the pair, that tab closes too (status: `Compare cleared (closed saved snapshot)`).

While compare is on, panes stay pinned to that pair (tab clicks do not swap the left file away).

Closing a tab that is **not** in the pair remaps both sides so Compare stays on. Closing either compared tab clears Compare.

### Jump between differences

- Starting Compare parks **and selects** both panes on the **first** change hunk (same ordinal; status shows hunk count).
- **View → Next Difference** / **Previous Difference** (or **F7** / **Shift+F7**)
- **View → First Difference** / **Last Difference** (or **⌘/Ctrl+F7** / **⌘/Ctrl+Shift+F7**)
- Moves the caret on the focused pane to the start of the next/previous change hunk (wraps), or jumps to the first/last hunk, and **selects that hunk’s change lines on both panes** (Copy/Delete still apply to the focused pane).
- Next/Prev that wrap past the end/beginning append `· wrapped` to the status (e.g. `Compare Next difference → L12 | R15 (1/5) · wrapped`).
- The other pane parks on the **same hunk ordinal** (line numbers may differ when sides disagree) and selects that side’s hunk too.
- Mid-hunk Next skips to the following hunk, not the next red/green line.
- Status shows both sides’ lines + hunk ordinal, e.g. `Compare Next difference → L12 | R15 (2/5)`.
- Click a red/green change line, or move the caret onto one with the keyboard, to park **and select** the same hunk on both panes (status: `Compare hunk → L12 | R15 (2/5)`). Equal lines leave the other pane alone.
- On replace hunks (a changed line on both sides), differing **words** (and refined characters inside a 1:1 token replace) get a stronger wash so you can see the intra-line edit. Insert-only / delete-only lines stay the usual full-line colour. Very long lines (over 256 characters) skip intra-line LCS.
- When both sides match (including ignore-whitespace / ignore-case), status says `(identical)`.
- Active ignore options appear in the status (`· ignore ws`, `· ignore case`, `· ignore blank`, or combined `ws+case+blank`).
- **View → Ignore Whitespace Differences** / **Ignore Case Differences** / **Ignore Blank Lines** toggle those Preferences keys (✓ when on) and re-diff immediately while Compare is on. Blank-line ignore skips empty / whitespace-only lines in the LCS so padding blank lines do not create hunks.
- **View → Hide Unchanged Lines** toggles a Preferences key (✓ when on). While Compare is on, Equal-tagged lines are hidden in both panes except **±3 lines of context** around each change (folds still apply). Identical pairs stay fully visible so the panes do not go blank. Status appends `· hide equal ±3`.
- **View → Swap Compare Sides** flips left/right files (and scroll), keeps focus on the new left, and re-diffs so delete/insert colours stay correct.
- **View → Copy Compare Diff** copies a unified diff of the pair to the clipboard (3 lines of context). Status shows `Copied unified diff (−N +M)` or `(identical)`.
- **View → Open Compare Diff** opens that unified diff in a `compare.diff` tab (Compare turns off so the tab is not pinned away). Status shows `Opened unified diff (−N +M)` or `(identical)`.
- **View → Copy Compare Hunk** copies only the change hunk at the focused caret (or the next hunk if the caret is on an equal line), with 3 equal context lines. Status shows `Copied hunk (i/n) unified diff (−N +M)`.
- **View → Open Compare Hunk** opens that same caret hunk as a `compare-hunk.diff` tab (Compare turns off so the tab is not pinned away). Status shows `Opened hunk (i/n) unified diff (−N +M)`.
- **View → Apply Hunk From Other View** replaces the focused pane's change hunk with the other pane (one undo). Insert-only / delete-only hunks insert or delete lines. After apply, both panes park on the **next** remaining hunk (status: `Applied hunk (i/n) from other view → Lx | Ry (j/m)`), or `· identical` when none remain.
- **View → Apply Hunk To Other View** pushes the focused pane's change hunk onto the other pane (one undo). Same next-hunk park / `· identical` status as Apply From Other.
- **View → Apply All Hunks From Other View** applies every remaining change hunk to the focused pane (one undo), last hunk first. Status shows `Applied N hunks from other view` (plus `· identical` when the pair matches).
- **View → Apply All Hunks To Other View** pushes every remaining change hunk onto the other pane (one undo), last hunk first. Status shows `Applied N hunks to other view` (plus `· identical` when the pair matches).

## Limits

- Both panes stay editable. Line tags refresh after edits (~200 ms debounce).
- MVP max: **3000 lines** per side.
- No gap rows for inserts (line numbers stay per-file; sync is by scroll line).
- Intra-line wash is word-aware LCS on paired replace lines (char refine on 1:1 tokens). Not 3-way.
- Start Compare turns on sync H + V scroll.
- Preferences: **Ignore whitespace differences** collapses whitespace runs before LCS.
- Preferences: **Ignore case differences** folds letter case before LCS (combines with ignore-whitespace).
- Preferences: **Ignore blank lines** skips blank / whitespace-only lines in the LCS (combines with the other ignore toggles).
- Preferences: **Hide unchanged lines** collapses Equal lines in the Compare panes, keeping ±3 Equal lines of context around each change (display only; does not change the LCS).

## Not in MVP

- 3-way merge
- Richer inline diff (syntax-aware tokens)
- Shelling out to system `diff`

## Code

- `pick_compare_right` / `start_compare` / `compare_to_saved` / `open_compare_diff_tab` / `open_compare_hunk_tab` — `crates/app/src/ui.rs`
- `compare_stale` + `refresh_compare_if_stale` — `crates/app/src/editor.rs`, `crates/app/src/ui.rs`
- Line LCS — `crates/app/src/diff.rs`
