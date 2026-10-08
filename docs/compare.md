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

Status line shows: `Compare “dummy.log” | “dummy.log.2” (−N +M) · K hunks: D delete, I insert, R replace · P% equal` (identical pairs say `(identical)` instead of counts; kind tallies omit zero kinds).

**View → Clear Compare** removes colours. Status keeps the last pair overview with −/+ and hunk-kind tallies (e.g. `Compare cleared (−1 +2, 2 hunks: 1 delete, 1 insert) “a” | “b”`, or `(identical)`). If a Compare-to-Saved `name (saved)` snapshot was in the pair, that tab closes too (status prefix: `Compare cleared (closed saved snapshot) …`).

While compare is on, panes stay pinned to that pair (tab clicks do not swap the left file away).

Closing a tab that is **not** in the pair remaps both sides so Compare stays on. Closing either compared tab clears Compare.

### Jump between differences

- Starting Compare parks **and selects** both panes on the **first** change hunk (same ordinal). Status keeps the pair overview and appends the landing, e.g. `· at L12 | R15 (1/5 replace −1 +1)`.
- **View → Next Difference** / **Previous Difference** (or **F7** / **Shift+F7**)
- **View → First Difference** / **Last Difference** (or **⌘/Ctrl+F7** / **⌘/Ctrl+Shift+F7**)
- Moves the caret on the focused pane to the start of the next/previous change hunk (wraps), or jumps to the first/last hunk, and **selects that hunk’s change lines on both panes** (Copy/Delete still apply to the focused pane).
- Next/Prev that wrap past the end/beginning append `· wrapped` to the status (e.g. `Compare Next difference → L12 | R15 (1/5 delete) · wrapped`).
- The other pane parks on the **same hunk ordinal** (line numbers may differ when sides disagree) and selects that side’s hunk too.
- Mid-hunk Next skips to the following hunk, not the next red/green line.
- Status shows both sides’ lines + hunk ordinal + kind + −/+ counts, e.g. `Compare Next difference → L12 | R15 (2/5 replace −1 +1)`.
- Click a red/green change line, or move the caret onto one with the keyboard, to park **and select** the same hunk on both panes (status: `Compare hunk → L12 | R15 (2/5 replace −1 +1)`). Click or move the caret onto an **Equal** line to park the other pane on the LCS-aligned partner (status: `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal · ignore ws · hide equal ±3 · 5 hidden` with pair −/+/kind tallies, Equal-line match percent, and the same ignore / hide-equal bits as the live pair status, or `(identical)`; no hunk selection). If Hide Unchanged Lines had collapsed that Equal partner (or the focused Equal), the park **reveals** those lines so the caret stays on-screen.
- On replace hunks (a changed line on both sides), differing **words** (and refined characters inside a 1:1 token replace) get a stronger wash so you can see the intra-line edit. Insert-only / delete-only lines stay the usual full-line colour. Very long lines (over 256 characters) skip intra-line LCS.
- When both sides match (including ignore-whitespace / ignore-case), status says `(identical)`.
- Non-identical pairs append hunk-kind tallies after the hunk count (`· K hunks: N delete, M insert, P replace`) and an Equal-line match percent (`· N% equal`) on the live status line (same metrics as Compare Summary).
- Active ignore options appear in the status (`· ignore ws`, `· ignore case`, `· ignore blank`, or combined `ws+case+blank`).
- **View → Ignore Whitespace Differences** / **Ignore Case Differences** / **Ignore Blank Lines** toggle those Preferences keys (✓ when on) and re-diff immediately while Compare is on. Blank-line ignore skips empty / whitespace-only lines in the LCS so padding blank lines do not create hunks. After a live re-diff, both panes park **and select** the first remaining change hunk; status appends the same `· at L|R (1/n kind −/+)` landing bit as Compare start.
- **View → Hide Unchanged Lines** toggles a Preferences key (✓ when on). While Compare is on, Equal-tagged lines are hidden in both panes except **±N lines of context** around each change (Preferences **Hide-equal context**, default 3; Alt+]/ / Alt+[; folds still apply). Identical pairs stay fully visible so the panes do not go blank. Toggling while Compare is on parks **and selects** both panes on the first change hunk; status appends `· at L|R (1/n kind −/+)` like ignore re-diff / Compare start. Changing hide-equal context (hotkeys or Preferences slider) while Hide Unchanged is on likewise parks **and selects** the first change hunk; status is `Hide-equal context ±N — Compare … · at L|R (1/n kind −/+)`. Status also appends `· hide equal ±N` plus a hidden-line count (`· N hidden` or `· Lx|Ry hidden`). Collapsed runs show a gutter `···N` cue and a hairline between the visible rows. **Click a `···N` cue** to expand that collapsed Equal run on **both panes** (aligned partner lines; status: `Compare expanded ···N both panes (−N +M, K hunks: D delete, I insert, R replace, … still hidden)` or `… all equal lines shown`; toggle hide-equal off or Clear Compare resets). **View → Expand Unchanged at Caret** expands the collapsed Equal run nearest the caret on both panes (same status as that click). **View → Collapse Unchanged at Caret** re-collapses the expanded Equal run nearest the caret on both panes (status: `Compare collapsed ···N both panes (−N +M, K hunks: …, M hidden)`). **View → Next Hidden Equal** / **Previous Hidden Equal** (Alt+F7 / Alt+Shift+F7) jump to the next/previous ···N gap (park on the visible row above the cue; status: `Compare Next hidden equal → ···N (i/n) (−N +M, K hunks: D delete, I insert, R replace)` plus `· wrapped` when wrapping; other pane parks on the LCS-aligned Equal partner). **View → First Hidden Equal** / **Last Hidden Equal** (⌘/Ctrl+Alt+F7 / ⌘/Ctrl+Alt+Shift+F7) jump to the first/last ···N gap (same −/+/kind tallies; no wrap bit). **View → Expand All Unchanged Lines** reveals every remaining collapsed Equal run on both panes without turning hide-equal off (status: `Compare expanded all unchanged lines (−N +M, K hunks: D delete, I insert, R replace, N shown)`). **View → Collapse All Unchanged Lines** clears those expansions on both panes so `···N` cues return (status: `Compare collapsed expanded equal lines (−N +M, K hunks: D delete, I insert, R replace, N re-hidden, M hidden)`).
- **View → Bookmark Compare Differences** bookmarks the start of every change hunk on both panes (status: `Compare bookmarked differences (−N +M, Lx|Ry, n hunks: D delete, I insert, R replace, +m new)`). Then **F2** / **Shift+F2** walk those marks. Identical pairs report no differences.
- **View → Clear Compare Difference Bookmarks** removes bookmarks at those hunk starts on both panes only (status: `Compare cleared difference bookmarks (−N +M, Lx|Ry, n hunks: D delete, I insert, R replace, −m removed)`). Other bookmarks stay.
- **View → Swap Compare Sides** flips left/right files (and scroll), keeps focus on the new left, and re-diffs so delete/insert colours stay correct. Both panes park **and select** the first change hunk; status is `Swapped sides — Compare … · at L|R (1/n kind −/+)`.
- **View → Copy Compare Diff** copies a unified diff of the pair to the clipboard (3 lines of context). Status shows `Copied unified diff (−N +M, K hunks: D delete, I insert, R replace)` (zero kinds omitted) or `(identical)`.
- **View → Open Compare Diff** opens that unified diff in a `compare.diff` tab (Compare turns off so the tab is not pinned away). Status shows `Opened unified diff (−N +M, K hunks: …)` or `(identical)`.
- **View → Copy Compare Summary** copies the hunk index text (same body as Open Compare Summary) to the clipboard without clearing Compare. Status shows `Copied compare summary (−N +M, K hunks: D delete, I insert, R replace)` (zero kinds omitted; singular `hunk` when K is 1).
- **View → Open Compare Summary** opens a `compare-summary.txt` tab listing every change hunk with L|R line ranges, −/+ counts, kind (delete/insert/replace), and up to **3** truncated preview lines per side (`-` / `+`, max 72 chars), then `… (+N more)` when the hunk side is longer. The header also tallies hunk kinds (`N delete, M insert, K replace`), an Equal-line match percent (`· N% equal`), and appends active ignore flags (`· ignore ws+case+blank`) when any Ignore toggle is on. Compare turns off so the tab stays visible. Status shows `Opened compare summary (−N +M, K hunks: D delete, I insert, R replace)` (same kind tallies as Copy).
- **View → Copy Compare Hunk** copies only the change hunk at the focused caret (or the next hunk if the caret is on an equal line), with 3 equal context lines. Status shows `Copied hunk (i/n replace −A +B) unified diff “…” | “…”`.
- **View → Open Compare Hunk** opens that same caret hunk as a `compare-hunk.diff` tab (Compare turns off so the tab is not pinned away). Status shows `Opened hunk (i/n replace −A +B) unified diff “…” | “…”`.
- **View → Apply Hunk From Other View** (⌘/Ctrl+Alt+←) replaces the focused pane's change hunk with the other pane (one undo). Insert-only / delete-only hunks insert or delete lines. After apply, both panes park on the **next** remaining hunk (status: `Applied hunk (i/n replace −A +B) from other view → Lx | Ry (j/m delete −C +D)`), or `· identical` when none remain.
- **View → Apply Hunk To Other View** (⌘/Ctrl+Alt+→) pushes the focused pane's change hunk onto the other pane (one undo). Same next-hunk park / kind/−/+ status / `· identical` as Apply From Other.
- **View → Apply All Hunks From Other View** (⌘/Ctrl+Alt+Shift+←) applies every remaining change hunk to the focused pane (one undo), last hunk first. Status shows `Applied N hunks (K delete, I insert, R replace −A +B) from other view` (plus `· identical` when the pair matches; kind/−/+ cover only hunks that changed).
- **View → Apply All Hunks To Other View** (⌘/Ctrl+Alt+Shift+→) pushes every remaining change hunk onto the other pane (one undo), last hunk first. Status shows the same kind/−/+ bit as Apply All From (`Applied N hunks (…) to other view`).

## Limits

- Both panes stay editable. Line tags refresh after edits (~200 ms debounce). After that re-diff, the other pane parks on the focused caret’s change hunk (or Equal partner) and the pair status appends `· at L|R (i/n kind −/+)` on a change or `· equal L|R` on Equal; hide-equal collapsed partners are revealed so the park is visible. The focused caret and selection are left alone so typing is not yanked.
- MVP max: **3000 lines** per side. Larger files still Compare: LCS uses the first 3000 lines on each side; the rest stays uncoloured and status appends `· first 3000 lines`.
- No gap rows for inserts (line numbers stay per-file; sync is by scroll line).
- Intra-line wash is word-aware LCS on paired replace lines (char refine on 1:1 tokens). Not 3-way.
- Start Compare turns on sync H + V scroll.
- Preferences: **Ignore whitespace differences** collapses whitespace runs before LCS.
- Preferences: **Ignore case differences** folds letter case before LCS (combines with ignore-whitespace).
- Preferences: **Ignore blank lines** skips blank / whitespace-only lines in the LCS (combines with the other ignore toggles).
- Preferences: **Hide unchanged lines** collapses Equal lines in the Compare panes. **Hide-equal context** (`compare_hide_equal_context`, 0–10, default 3) sets how many Equal lines stay visible before/after each change (display only; does not change the LCS). While Compare + Hide Unchanged is on, changing that value (slider or Alt+]/ / Alt+[) parks both panes on the first change hunk. Gap markers (`···N`) and status hidden counts show how much was collapsed.

## Not in MVP

- 3-way merge
- Richer inline diff (syntax-aware tokens)
- Shelling out to system `diff`

## Code

- `pick_compare_right` / `start_compare` / `compare_to_saved` / `open_compare_diff_tab` / `open_compare_summary_tab` / `copy_compare_summary` / `open_compare_hunk_tab` — `crates/app/src/ui.rs`
- `compare_stale` + `refresh_compare_if_stale` — `crates/app/src/editor.rs`, `crates/app/src/ui.rs`
- Line LCS — `crates/app/src/diff.rs`
