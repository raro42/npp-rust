# Changelog

## [Unreleased]

## [0.3.116] — 2026-10-08

Settings / shortcuts:

- **Find** is remappable: Preferences → Find shortcut, or `shortcut_find` in `npp-rs/settings.json` (default `Cmd+F`). Replace stays hard-wired `Cmd+H` / `Cmd+Shift+F`. Shortcut Mapper and About show the effective binding.

## [0.3.115] — 2026-10-08

Settings / shortcuts:

- **Save** is remappable: Preferences → Save shortcut, or `shortcut_save` in `npp-rs/settings.json` (default `Cmd+S`). Save As stays hard-wired `Cmd+Shift+S`. Shortcut Mapper and About show the effective binding.

## [0.3.114] — 2026-10-07

Settings / shortcuts:

- **Close tab** is remappable: Preferences → Close tab shortcut, or `shortcut_close_tab` in `npp-rs/settings.json` (default `Cmd+W`). Shortcut Mapper and About show the effective binding.

## [0.3.113] — 2026-10-07

Settings / shortcuts:

- **Format document** is remappable: Preferences → Format document shortcut, or `shortcut_format_document` in `npp-rs/settings.json` (default `Cmd+Shift+I`). Shortcut Mapper and About show the effective binding.

## [0.3.112] — 2026-10-07

Settings / shortcuts:

- **Toggle bookmark** is remappable: Preferences → Toggle bookmark shortcut, or `shortcut_toggle_bookmark` in `npp-rs/settings.json` (default `Cmd+F2`). Shortcut Mapper and About show the effective binding.

## [0.3.111] — 2026-10-07

Settings / shortcuts:

- **Outdent** is remappable: Preferences → Outdent shortcut, or `shortcut_outdent` in `npp-rs/settings.json` (default `Cmd+[`). Shortcut Mapper and About show the effective binding.

## [0.3.110] — 2026-10-07

Settings / shortcuts:

- **Indent** is remappable: Preferences → Indent shortcut, or `shortcut_indent` in `npp-rs/settings.json` (default `Cmd+]`). Outdent stays hard-wired `Cmd+[`. Shortcut Mapper and About show the effective binding.

## [0.3.109] — 2026-10-07

Settings / shortcuts:

- **Delete line** is remappable: Preferences → Delete line shortcut, or `shortcut_delete_line` in `npp-rs/settings.json` (default `Cmd+Shift+L`). Shortcut Mapper and About show the effective binding.

## [0.3.108] — 2026-10-07

Settings / shortcuts:

- **Duplicate line** is remappable: Preferences → Duplicate line shortcut, or `shortcut_duplicate_line` in `npp-rs/settings.json` (default `Cmd+D`). Shortcut Mapper and About show the effective binding.

## [0.3.107] — 2026-10-07

Settings / shortcuts:

- **Go to line** is remappable: Preferences → Go to line shortcut, or `shortcut_goto_line` in `npp-rs/settings.json` (default `Cmd+L`). Cmd/Ctrl+Shift+L delete line stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.106] — 2026-10-07

Settings / shortcuts:

- **Next difference** is remappable: Preferences → Next difference shortcut, or `shortcut_next_diff` in `npp-rs/settings.json` (default `F7`). Shift + that chord is Previous difference. Cmd/Ctrl+F7 first/last and Alt+F7 hidden-equal stay hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.105] — 2026-10-07

Settings / shortcuts:

- **Next bookmark** is remappable: Preferences → Next bookmark shortcut, or `shortcut_next_bookmark` in `npp-rs/settings.json` (default `F2`). Shift + that chord is Previous bookmark. Cmd/Ctrl+F2 toggle stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.104] — 2026-10-07

Settings / shortcuts:

- **Find next** is remappable: Preferences → Find next shortcut, or `shortcut_find_next` in `npp-rs/settings.json` (default `F3`). Shift + that chord is Find previous. Cmd/Ctrl+G stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.103] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status appends the same ignore / hide-equal bits as the live pair line, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal · ignore ws+case · hide equal ±3 · 5 hidden` (identical pairs keep `(identical)` plus those bits when active).

## [0.3.102] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status appends Equal-line match percent, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal` (identical pairs stay `(identical)`).

## [0.3.101] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status includes pair −/+/kind tallies, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert)` (or `(identical)`).

## [0.3.100] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard / edit re-diff) auto-reveals a Hide-Unchanged collapsed Equal line on either pane so the parked caret stays visible. Edit re-diff on Equal appends `· equal L|R` to pair status (hunk landings still use `· at L|R`).

## [0.3.99] — 2026-10-07

Compare:

- After the ~200 ms edit re-diff, the other pane parks on the focused caret’s change hunk (or Equal partner); pair status appends `· at L|R (i/n kind −/+)`. Focused caret and selection stay put so typing is not yanked.

## [0.3.98] — 2026-10-07

Compare:

- Changing **Hide-equal context** (Alt+]/ / Alt+[ or Preferences slider) while Compare + Hide Unchanged Lines is on parks **and selects** both panes on the first change hunk; status is `Hide-equal context ±N — Compare … · at L|R (1/n kind −/+)`.

## [0.3.97] — 2026-10-07

Compare:

- **Hide Unchanged Lines** while Compare is on parks **and selects** both panes on the first change hunk; status appends `· at L|R (1/n kind −/+)` like ignore re-diff / Compare start / swap sides.

## [0.3.96] — 2026-10-07

Compare:

- **Next / Previous / First / Last Hidden Equal** hard-wired shortcuts: Alt+F7 / Alt+Shift+F7 walk ···N gaps; ⌘/Ctrl+Alt+F7 / ⌘/Ctrl+Alt+Shift+F7 jump to first/last. Plain F7 family still walks change hunks. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.95] — 2026-10-07

Compare:

- **Apply All Hunks From / To Other View** hard-wired shortcuts: ⌘/Ctrl+Alt+Shift+← pulls every remaining hunk from the other pane; ⌘/Ctrl+Alt+Shift+→ pushes every remaining hunk to the other pane. Single-hunk ⌘/Ctrl+Alt+←/→ unchanged. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.94] — 2026-10-07

Compare:

- **Apply Hunk From / To Other View** hard-wired shortcuts: ⌘/Ctrl+Alt+← pulls the other pane into the focused hunk; ⌘/Ctrl+Alt+→ pushes the focused hunk to the other pane. Alt alone still word-jumps. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.93] — 2026-10-06

Compare:

- **Next / Previous / First / Last Hidden Equal** status includes −/+ and hunk-kind tallies (`Compare Next hidden equal → ···5 (1/3) (−1 +2, 2 hunks: 1 delete, 1 insert) · wrapped`).

## [0.3.92] — 2026-10-06

Compare:

- **Expand Unchanged at Caret** / click ···N / **Collapse Unchanged at Caret** status includes −/+ and hunk-kind tallies (`Compare expanded ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 still hidden)` / `Compare collapsed ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 hidden)`).

## [0.3.91] — 2026-10-06

Compare:

- **Expand All Unchanged Lines** / **Collapse All Unchanged Lines** status includes −/+ and hunk-kind tallies (`Compare expanded all unchanged lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 shown)` / `Compare collapsed expanded equal lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 re-hidden, 8 hidden)`).

## [0.3.90] — 2026-10-06

Compare:

- **Bookmark Compare Differences** / **Clear Compare Difference Bookmarks** status includes −/+ and hunk-kind tallies (`Compare bookmarked differences (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, +3 new)` / `Compare cleared difference bookmarks (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, −3 removed)`).

## [0.3.89] — 2026-10-06

Compare:

- **Clear Compare** status keeps the last pair overview with −/+ and hunk-kind tallies (`Compare cleared (−1 +2, 2 hunks: 1 delete, 1 insert) “a” | “b”`, or `(identical)`; saved-snapshot clear keeps the same counts after the closed-snapshot prefix).

## [0.3.88] — 2026-10-06

Compare:

- **Swap Compare Sides** re-diffs and parks **and selects** both panes on the first change hunk; status appends `· at L|R (1/n kind −/+)` like Compare start / ignore re-diff.

## [0.3.87] — 2026-10-06

Compare:

- While Compare is on, toggling Ignore Whitespace / Case / Blank Lines re-diffs and parks **and selects** both panes on the first remaining change hunk; status appends `· at L|R (1/n kind −/+)` like Compare start.

## [0.3.86] — 2026-10-06

Compare:

- Starting Compare (and Compare to Saved) status appends the parked first hunk (`· at L12 | R15 (1/5 replace −1 +1)`), matching nav/click ordinal bits.

## [0.3.85] — 2026-10-06

Compare:

- **Copy Compare Diff** / **Open Compare Diff** status includes hunk-kind tallies (`Copied unified diff (−1 +2, 2 hunks: 1 delete, 1 insert) …`), matching Copy/Open Compare Summary.

## [0.3.84] — 2026-10-06

Compare:

- **Copy Compare Summary** / **Open Compare Summary** status includes hunk-kind tallies (`Copied compare summary (−1 +2, 2 hunks: 1 delete, 1 insert) …`), matching the live Compare status / summary header.

## [0.3.83] — 2026-10-06

Compare:

- **Copy Compare Hunk** / **Open Compare Hunk** status includes the hunk kind and −/+ counts (`Copied hunk (2/5 replace −1 +1) unified diff …`), matching nav/apply ordinal bits.

## [0.3.82] — 2026-10-06

Compare:

- **Apply All Hunks From/To Other View** status includes kind tallies and total −/+ for the hunks actually applied (`Applied 3 hunks (1 delete, 1 insert, 1 replace −2 +2) from other view · identical`).

## [0.3.81] — 2026-10-06

Compare:

- **Apply Hunk From/To Other View** status includes the applied hunk kind and −/+ counts, and the next remaining hunk uses the same ordinal bit (`Applied hunk (1/5 replace −2 +1) from other view → L12 | R15 (2/4 delete −1 +0)`).

## [0.3.80] — 2026-10-06

Compare:

- Next/Previous/First/Last Difference and click/keyboard hunk sync status include the current hunk −/+ line counts (`(2/5 replace −1 +1)`), so hunk size is visible without opening Compare Summary.

## [0.3.79] — 2026-10-06

Compare:

- Next/Previous/First/Last Difference and click/keyboard hunk sync status include the current hunk kind (`(2/5 replace)`), so delete/insert/replace is visible without opening Compare Summary.

## [0.3.78] — 2026-10-06

Compare:

- Live Compare status line tallies hunk kinds after the hunk count (`· K hunks: N delete, M insert, P replace`) so delete/insert/replace mix is visible without opening Compare Summary.

## [0.3.77] — 2026-10-06

Compare:

- Live Compare status line appends Equal-line match percent (`· N% equal`) for non-identical pairs so similarity is visible without opening Compare Summary.

## [0.3.76] — 2026-10-06

Compare:

- **Open / Copy Compare Summary** header includes an Equal-line match percent (`· N% equal`) so similarity is visible without scanning every hunk.

## [0.3.75] — 2026-10-06

Compare:

- **Open / Copy Compare Summary** shows up to 3 preview lines per side under each hunk (`-` / `+`, max 72 chars), then `… (+N more)` when the side is longer, so multi-line hunks stay scannable.

## [0.3.74] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** header appends active ignore flags (`· ignore ws+case+blank`) so a copied or opened summary records which Ignore toggles shaped the hunk list.

## [0.3.73] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** header tallies hunk kinds (`N delete, M insert, K replace`) next to the hunk count.

## [0.3.72] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** includes a truncated first-line preview under each hunk (`-` / `+`, max 72 chars) so the index is scannable without opening every hunk.

## [0.3.71] — 2026-10-05

Compare:

- **View → Copy Compare Summary** copies the hunk index text to the clipboard (same body as Open Compare Summary; Compare stays on).

## [0.3.70] — 2026-10-05

Compare:

- **View → Open Compare Summary** opens a `compare-summary.txt` tab listing every change hunk with L|R line ranges and −/+ counts (clears Compare so the tab stays visible).

## [0.3.69] — 2026-10-05

Compare / Preferences:

- **Hide-equal context** (0–10, default 3) sets how many Equal lines stay visible on each side of a change when Hide Unchanged Lines is on. Status shows `· hide equal ±N`. Key: `compare_hide_equal_context`.

## [0.3.68] — 2026-10-05

Compare:

- **View → First Hidden Equal** / **Last Hidden Equal** jump to the first/last ···N collapsed Equal gap while Hide Unchanged Lines is on (parks both panes; status shows ordinal).

## [0.3.67] — 2026-10-05

Compare:

- **View → Next Hidden Equal** / **Previous Hidden Equal** jump between ···N collapsed Equal gaps while Hide Unchanged Lines is on (wraps with `· wrapped` status; parks both panes on the aligned Equal partner).

## [0.3.66] — 2026-10-05

Folding / session:

- Opt-in session restore now keeps **folded regions** (writes `@folds` header lines under each path in `npp-rs/session.txt`; applies on restore when headers still match).

## [0.3.65] — 2026-10-05

Compare:

- **View → Collapse Unchanged at Caret** re-collapses the expanded Equal run nearest the caret on both panes (···N cue returns for that run; hide-equal stays on).

## [0.3.64] — 2026-10-05

Compare:

- **View → Expand Unchanged at Caret** expands the collapsed Equal run nearest the caret on both panes (same as clicking that `···N` cue; hide-equal stays on).

## [0.3.63] — 2026-10-05

Compare:

- Click or move the caret onto an Equal (unchanged) line parks the other pane on the LCS-aligned partner line (status: `Compare equal → Lx | Ry`). Change-line hunk select is unchanged.

## [0.3.62] — 2026-10-05

Compare:

- **View → Clear Compare Difference Bookmarks** removes bookmarks at each change-hunk start on both panes (other bookmarks stay).

## [0.3.61] — 2026-10-05

Compare:

- **View → Bookmark Compare Differences** bookmarks the start of every change hunk on both panes so F2 / Shift+F2 can walk differences after you leave a hunk.

## [0.3.60] — 2026-10-05

Compare:

- **View → Collapse All Unchanged Lines** re-collapses every click/menu-expanded Equal run on both panes while Hide Unchanged Lines stays on (`···N` cues return without flipping the preference).

## [0.3.59] — 2026-10-05

Compare:

- **View → Expand All Unchanged Lines** reveals every collapsed Equal run on both panes while Hide Unchanged Lines stays on (clears remaining `···N` cues without flipping the preference).

## [0.3.58] — 2026-10-05

Compare:

- **Hide Unchanged Lines**: clicking a `···N` cue expands the collapsed Equal run on **both panes** (LCS-aligned partner lines). Status notes `both panes` when the other side opened too.

## [0.3.57] — 2026-10-05

Compare:

- **Hide Unchanged Lines**: click a gutter `···N` cue to expand that collapsed Equal run on the clicked pane. Status reports how many lines opened and how many remain hidden. Clear Compare / toggle hide-equal off / swap sides resets expansions.

## [0.3.56] — 2026-10-05

Compare:

- **Hide Unchanged Lines** paints gutter `···N` gap cues (and a hairline) between collapsed Equal runs, and status shows how many lines are hidden (`· N hidden` or `· Lx|Ry hidden`).

## [0.3.55] — 2026-10-05

Compare:

- **Hide Unchanged Lines** keeps **±3 Equal lines of context** around each change hunk so collapsed Compare panes stay readable. Status shows `· hide equal ±3`.

## [0.3.54] — 2026-10-05

Compare:

- **View → Hide Unchanged Lines** (Preferences persist) collapses Equal-tagged lines in both Compare panes so only change lines stay visible. Identical pairs stay fully shown. Status appends `· hide equal`.

## [0.3.53] — 2026-10-05

Compare:

- Replace-hunk intra-line wash is **word-aware**: LCS on word/separator tokens, with char refine on 1:1 token replaces so typos still highlight inside a word. Whole-word edits (e.g. `cat` → `dog`) wash the token, not scattered characters.

## [0.3.52] — 2026-10-05

Compare:

- **View → Clear Compare** (and other clear paths) closes the read-only `name (saved)` snapshot tab when it was part of the compare pair, so Compare to Saved does not leave an orphan tab.

## [0.3.51] — 2026-10-05

Compare:

- **View → Compare to Saved** diffs the active named tab against a read-only snapshot of its on-disk contents (reuses the `name (saved)` tab when present). Status uses the usual Compare −/+ / identical wording.

## [0.3.50] — 2026-10-05

Compare:

- **View → Apply All Hunks To Other View** pushes every remaining change hunk onto the other pane in one undo (last hunk first). Status shows how many hunks were applied, plus `· identical` when the pair matches.

## [0.3.49] — 2026-10-05

Compare:

- **View → Apply Hunk To Other View** pushes the change hunk at the focused caret onto the other pane (one undo). Parks and selects the next remaining hunk on both panes, same as Apply From Other. Status shows the next `L|R` ordinal, or `· identical` when the pair matches.

## [0.3.48] — 2026-10-05

Compare:

- **View → Open Compare Hunk** opens a `compare-hunk.diff` tab with the unified diff for the change hunk at the caret (same payload as Copy Compare Hunk). Compare mode turns off so the tab stays visible. Status shows hunk ordinal and −/+ counts.

## [0.3.47] — 2026-10-04

Compare:

- **View → Ignore Blank Lines** (Preferences `compare_ignore_blank`) skips blank / whitespace-only lines in the LCS so extra empty lines do not create hunks. Combines with ignore-whitespace / ignore-case; status shows `· ignore blank` (or `ws+case+blank`).

## [0.3.46] — 2026-10-04

Compare:

- **Apply Hunk From Other View** parks and selects the next remaining hunk on both panes after a successful apply. Status shows the next `L|R` ordinal, or `· identical` when the pair matches. **Apply All** also appends `· identical` when nothing is left.

## [0.3.45] — 2026-10-04

Search:

- Regex **Replace** / **Replace All** expand `$n`, `${n}`, `$0` / `$&` (whole match), `$$`, and `\1` (plus `\n` `\t` `\r`). Literal replace is unchanged when **Re** is off.

## [0.3.44] — 2026-10-04

Compare:

- **View → Open Compare Diff** opens a `compare.diff` tab with the same unified diff as Copy Compare Diff. Compare mode turns off so the tab stays visible. Status shows change counts or `(identical)`.

## [0.3.43] — 2026-10-04

Search:

- **Find in Files** honors the Find bar **Re** toggle (same linear-time regex as in-file Find). Invalid patterns report `Find in Files: invalid regex` and do not open a results tab. Literal search is unchanged when **Re** is off.

## [0.3.42] — 2026-10-04

Compare:

- **View → Apply All Hunks From Other View** copies every remaining change hunk onto the focused pane in one undo. Insert-only and delete-only hunks work. Status shows how many hunks were applied.

## [0.3.41] — 2026-10-04

Compare:

- Replace hunks highlight the **changed characters** on both panes (stronger wash on the intra-line LCS). Pure insert/delete lines stay whole-line colour. Lines over 256 characters keep the line wash only.

## [0.3.40] — 2026-10-04

Compare:

- **View → Apply Hunk From Other View** replaces the focused pane's change hunk with the other pane (one undo). Insert-only and delete-only hunks work. Status shows the hunk ordinal.

## [0.3.39] — 2026-10-04

Compare:

- **View → Copy Compare Hunk** copies the change hunk at the focused caret (or the next hunk) as a unified diff, with 3 equal context lines. Status shows hunk ordinal and −/+ counts.

## [0.3.38] — 2026-10-04

Compare:

- **View → Copy Compare Diff** copies a unified diff of the compared pair to the clipboard (3 lines of context). Status shows change counts or `(identical)`.

## [0.3.37] — 2026-10-04

Preferences:

- Unknown keys in `npp-rs/settings.json` survive the next save (hand-edited or future fields are not dropped).

## [0.3.36] — 2026-10-04

Compare:

- Next/Prev/First/Last Difference, click/keyboard hunk sync, and Start Compare select the change hunk on **both** panes (Copy/Delete still use the focused pane).

## [0.3.35] — 2026-10-04

Preferences / View:

- Show white space and TAB, Show EOL, Show NPC, and Indent guide persist in `npp-rs/settings.json` (View menu + Preferences) and restore on launch.

## [0.3.34] — 2026-10-04

Preferences:

- Recent file list load now honors Preferences **Recent file count** (`recent_max`, 5–40) instead of hard-capping at 15 on restart.

## [0.3.33] — 2026-10-04

Compare:

- Next/Prev/First/Last Difference selects the whole change hunk on the focused pane (Copy/Delete apply to the change).

## [0.3.32] — 2026-10-04

Compare:

- Next/Prev Difference status appends `· wrapped` when the jump circles past the end or beginning of the file.

## [0.3.31] — 2026-10-04

Compare:

- Keyboard caret motion onto a red/green change line parks the other pane on the same hunk (same as click).

## [0.3.30] — 2026-10-04

Compare:

- Click a red/green change line to park the other pane on the same hunk ordinal.
- Next/Prev/First/Last Difference status shows both sides (`L12 | R15 (2/5)`).

## [0.3.29] — 2026-10-04

Compare:

- **View → Ignore Whitespace Differences** / **Ignore Case Differences** toggle (✓ in menu), persist to settings, and re-diff immediately while Compare is on.
- Starting Compare parks **both** panes on the first hunk ordinal (not only the left caret).
- Status line shows active ignore flags (`· ignore ws`, `· ignore case`, or `· ignore ws+case`).

## [0.3.28] — 2026-10-04

Compare:

- **View → Swap Compare Sides** flips the left/right pair (scroll included), keeps focus on the new left, and re-diffs so delete/insert colours stay oriented.

## [0.3.27] — 2026-10-04

Compare:

- Closing a tab that is not part of the compare pair remaps both sides so Compare stays on; closing either compared tab still clears Compare.

## [0.3.26] — 2026-10-04

Compare:

- **View → First Difference** / **Last Difference** (⌘/Ctrl+F7 / ⌘/Ctrl+Shift+F7) jump to the first or last change hunk; other pane stays on the same ordinal.

## [0.3.25] — 2026-10-04

Compare:

- Next/Previous Difference also parks the other pane on the same hunk ordinal.
- Status says `(identical)` when both sides match (including ignore options); live re-diff keeps hunk count.

## [0.3.24] — 2026-10-04

Compare:

- Preferences: **Ignore case differences** (`compare_ignore_case` in settings.json). Combines with ignore-whitespace; live re-diff while Compare is on.

## [0.3.23] — 2026-10-04

Compare:

- Starting Compare jumps the left caret to the first change hunk and shows hunk count in the status line.
- Next/Previous Difference status shows hunk ordinal, e.g. `(2/5)`.

## [0.3.22] — 2026-10-04

Project panel:

- **Reveal** opens the workspace folder (or a right-clicked entry) in the OS file manager.
- **Refresh** reloads a cached folder listing instead of re-reading every frame.
- Context menu: Open / Enter folder / Reveal in file manager.

## [0.3.21] — 2026-10-04

Folding:

- Fold margin click uses the same region pick as View → Fold/Unfold Current: toggle works on header markers and on any line inside a foldable block (primary and dual view).

## [0.3.20] — 2026-10-03

Compare:

- **Next / Previous Difference** (View menu, **F7** / **Shift+F7**) jumps the focused pane to the start of the next/previous change hunk while Compare is on (wraps). Mid-hunk Next skips to the following hunk.

## [0.3.19] — 2026-10-03

Settings / shortcuts:

- **Word wrap** is remappable: Preferences → Word wrap shortcut, or `shortcut_word_wrap` in `npp-rs/settings.json` (default `Alt+Z`). Shortcut Mapper and About show the effective binding. Other keys stay hard-wired; full `shortcuts.xml` remap is still later.

## [0.3.18] — 2026-10-03

Undo:

- Multi-caret / column typing coalesce: successive inserts at the same carets merge into one undo unit (same 1s window as plain typing). Replace-selection and deletes still start a new unit.

## [0.3.17] — 2026-10-03

Encoding:

- Open detects **UTF-16 LE/BE without a BOM** when at least two-thirds of 16-bit units have a zero high byte (typical ASCII / Latin). Status notes the missing BOM; the next save writes one.
- Valid UTF-8 without that NUL pattern is unchanged. Invalid UTF-8 still falls back to Windows-1252.

## [0.3.16] — 2026-10-03

Find / replace:

- Opening Find or Replace with a **multi-line selection** turns **Sel** on and searches that range (query stays as-is). A short single-line selection still fills the query.
- Match count is 0 while Sel is on but no range is captured (no silent whole-file search).
- Wrap-off Next/Prev says `passed end of selection` when Sel is on.

## [0.3.15] — 2026-10-03

Find / replace:

- **Re** on the Find bar (and Preferences **Regular expression**) treats the query as a regex. Engine is linear-time, so nested quantifiers do not hang the UI. Invalid patterns report `Find: invalid regex`. Replace uses a literal replacement string.
- Setting `find_regex` in `npp-rs/settings.json`

## [0.3.14] — 2026-10-03

Find / replace:

- **Wrap** on the Find bar (and Preferences) turns wrap-around on or off. Off: Next/Prev stop at the bound (`Find: passed end of file`). Default on, same as before.
- Setting `find_wrap` in `npp-rs/settings.json`

## [0.3.13] — 2026-10-03

Find / replace:

- **In selection** (`Sel` on the Find bar, Preferences Find) limits Next / Prev / Replace All to a captured range
- Setting `find_in_selection` in `npp-rs/settings.json`

## [0.3.12] — 2026-08-31

Issue #14 (P1 lexer-aware folding + fold margin):

- Gutter fold markers (`−` / `+`); click toggles a region
- Brace folds for Rust/C-like languages; indent folds for Python and others
- View → Fold / Unfold / levels use the same regions
- Preferences: **Show fold margin** (`show_fold_margin` in `npp-rs/settings.json`)
- Docs: `docs/folding.md`

## [0.3.11] — 2026-08-31

Issue #13 (P1 autosave / backup-on-save):

- Preferences → Files: **Backup on save** copies the on-disk file into `npp-rs/backup/` (path layout) before overwrite
- Preferences → Files: **Autosave interval** (0 = off, else 15–900s) saves dirty tabs that already have a path
- Settings keys: `backup_on_save`, `autosave_interval_secs` in `npp-rs/settings.json`
- Docs: `docs/autosave-backup.md`

## [0.3.10] — 2026-08-31

Issue #12 (P1 deeper Find in Files):

- Search uses the Project / workspace root and walks folders recursively
- Skips hidden dirs, symlinks, binary / huge files; caps matches and depth
- Include / exclude globs on the Find bar (persisted); default exclude skips `target`, `node_modules`, and similar
- Menu **Search → Find in Files** and the Find bar **Find in Files** button

## [0.3.9] — 2026-08-31

Issue #11 (P0 column / rect select + multi-caret typing):

- Alt+drag (Option+drag on macOS) builds a rectangular / column selection
- Typing, Backspace, Delete, Paste, Enter, and Tab apply to all multi-carets (one undo)
- Copy/Cut of multi ranges joins line slices with newlines
- Docs: `docs/column-mode.md` (no virtual space; arrows clear multi-carets)

## [0.3.8] — 2026-08-31

Issue #10 (P0 file drop + selection drag):

- Drop files onto the window to open them (folders skipped)
- Drag selected text to move; Ctrl/Cmd+drag to copy (one undo)
- Orange drop caret while dragging; dual-view panes supported

## [0.3.7] — 2026-08-31

Issue #9 (P0 hotkeys / Find next):

- F3 / Shift+F3 find next/prev without the Find bar open
- Cmd/Ctrl+G / Shift+G find next/prev are global
- Cmd/Ctrl+L go to line; F2 / Shift+F2 / Cmd+F2 bookmarks
- Cmd/Ctrl+= − 0 and Cmd/Ctrl+mouse wheel zoom; Alt+Z word wrap
- Cmd/Ctrl+H replace; Shortcut Mapper and About lists updated

## [0.3.6] — 2026-08-31

Issue #8 (P1 encoding):

- Open UTF-16 LE/BE with BOM; decode to Unicode in the buffer
- Save as UTF-16 LE/BE when Format (or Convert to) sets that encoding
- Status / docs: `docs/encoding.md`

## [0.3.5] — 2026-08-31

Issue #8 (P1 change-history depth):

- Gutter: full-height SC_MARK_BAR-style blocks (amber unsaved / green saved) with joined runs
- Soft line wash + dual-view pane marks
- Undo/redo remaps line marks via buffer line-structure hooks (no caret re-stamp)
- Status `CHG u/s`, Clear/jump messages name unsaved vs saved

## [0.3.4] — 2026-08-30

Issue #8 (P1 themes depth):

- Theme JSON: selection, caret, whitespace, indent guide, and syntax `tokens` map
- Notepad++ XML subset: GlobalStyles chrome + preferred lexer WordsStyle → highlight tokens
- Samples: deeper `themes/slate.json`, `themes/mini-dark.xml`
- Primary and secondary panes use theme chrome colours


## [0.3.3] — 2026-08-30

- Compare: default partner is the tab to the right (or left if last); ⌘/Ctrl-click or tab menu to pick any second tab
- Help / About / cmdline links use `main` (not `dev`)
- README feature-tour GIF (`docs/screens/`) + rebuild scripts

## [0.3.2] — 2026-08-29

Issue #8 (partial P1):

- Project panel: name filter, Refresh, persist workspace root in settings.json
- ANSI (Windows-1252) save: confirm dialog when characters would become `?`

## [0.3.1] — 2026-08-29

Issue #7 P0 gap batch:

- Preferences: gutter extra, caret blink, default EOL, recent count, restore session, find options, compare ignore-whitespace
- Find/Replace: Match case / Whole word, live match count, persisted query; Replace All is one undo + marks compare stale
- Session: config-dir `npp-rs/session.txt`; opt-in restore on launch; save on quit
- Compare: sync H+V scroll on start; optional ignore whitespace

## [0.3.0] — 2026-08-29

Stability and architecture batch for issues #3 / #4 / #6:

- Atomic saves; DocumentId async loads; tail worker; bookmark remap
- Read-only edit gate; reload + saved revision; transactional undo
- Honest UTF-8/Windows-1252 open; CI fmt/clippy; highlight viewport
- Theme apply MVP; project file panel; RTL layout cue


## [0.2.14] — 2026-08-29

### Stability (issue #4)

- File → Reload from Disk replaces buffer contents (dirty confirm)
- Dirty follows `saved_generation` vs `edit_generation` (undo-to-saved clears dirty)


## [0.2.13] — 2026-08-29

### Stability (issue #4)

- File → Reload from Disk replaces buffer contents (dirty confirm)
- Dirty follows saved vs edit generation (undo-to-saved clears dirty)

### Themes / project / RTL

- Theme MVP: built-in Dark/Light + `themes/*.json`; Preferences and Import Style Theme(s) apply egui visuals and editor bg/fg/gutter
- Project panel lists the workspace folder and opens files on click; Open Folder as Workspace sets the root
- RTL/LTR mirrors editor line anchors and shows an RTL status cue

## [0.2.12] — 2026-08-29

### Read-only

- Menu mutate paths refuse edits when the document is read-only or still loading (`Document::try_buffer_mut`, command `ensure_editable`)
- Toggle / clear read-only commands still work


## [0.2.11] — 2026-08-29

### Build / CI

- Fix TailMsg test initializers, `EditorState::workspace_root`, and edit `ensure_editable` gate
- Restore clippy allows on buffer `from_str` / `to_string`
- Mark issue #4 Agent E checklist done

## [0.2.10] — 2026-08-29

### Encoding honesty

- Open/tail never insert U+FFFD via `from_utf8_lossy`; invalid UTF-8 uses Windows-1252 with clear status
- Tests cover invalid byte sequences (`docs/encoding.md`)

## [0.2.9] — 2026-08-29

### Undo

- One user command is one undo unit (`with_transaction` for multi-edit helpers)
- Typing coalesce: same kind (insert), adjacent caret, within 1s
- Notes: `docs/undo-transactions.md`

## [0.2.8] — 2026-08-29

### Highlight

- Byte→char conversion is one forward pass (no per-span full rescans)
- Refresh uses a viewport-oriented window (still capped at 512 KiB)
- Notes: `docs/highlight-viewport.md`

## [0.2.7] — 2026-08-29

### Bookmarks

- Bookmarks (and other line marks) shift when inserts or deletes change line structure
- Buffer records `LineStructureEdit`; the editor prefers that over the snap heuristic

## [0.2.6] — 2026-08-29

### Stability (issue #4)

- Log tail: disk reads and rotate reload run on a background worker; the UI only applies `TailMsg` (dirty/suspend policy unchanged)

### Tabs / open

- Async large-file load binds to a stable `DocumentId` (not tab index)
- Pending load apply drops when the id is gone or no longer the loading placeholder

## [0.2.5] — 2026-08-29

### Save

- Atomic save: write a sibling temp file, `sync_all`, then rename over the target (std rename replaces on Windows)
- Save no longer creates missing parent directories

## [0.2.4] — 2026-08-29

### CLI

- `-V` / `--version`, `-n` / `--line`, `-ro` / `--read-only`, `--` end of options
- Help → Command Line Arguments documents the same flags

### Preferences follow-up

- View → Word wrap writes `settings.json`
- Edit → Indent Tab uses Preferences tab width

## [0.2.3] — 2026-08-29

### Dual view

- Menu Edit (and Format) commands use the focused pane tab, not only the tab-bar active document

### Encoding

- Format → ANSI: save writes Windows-1252 (lossy); UTF-8 / UTF-8-BOM set per-tab save encoding
- Open files keep the detected encoding for later save (`docs/encoding.md`)

### Compare

- Re-diff line tags after edits (~200 ms debounce) while Compare is on

### Change history

- Amber gutter ticks for unsaved edits; green after save (promote, not clear)
- Line-index remap on insert/delete (`LineEditSnap` / `prepare_edit`)

## [0.2.2] — 2026-08-29

### Release checkpoint

- About tagline: “a Notepad++ inspired editor, rebuilt for fun”
- Overnight gap loop started on issue #6 (`docs/overnight-gaps.md`)

## [0.2.1] — 2026-08-29

### Compare fix

- Move to Other View no longer undoes itself (removed extra Switch)
- Compare pins left/right panes to the compared pair so colours stay visible
- Switch in compare mode swaps both sides and their tags
- Clearer compare how-to in `docs/compare.md`

## [0.2.0] — 2026-08-29

Large feature batch after 0.1.2 (menu parity, dual view, compare, UX).

### Dual view edit

- Other view pane is writable (type, delete, clipboard, caret, selection)
- Click a pane to focus keyboard input; undo shortcuts follow the focused pane
- Sync scroll / zoom sync / compare washes unchanged

### Built-in compare

- View → Compare with Other View: 2-way side-by-side colours (red delete / green insert) + sync scroll
- In-process line LCS (`diff.rs`); Clear Compare to exit; max 3000 lines/side

### Menu parity (issue #1)

- File: `IDM_PINTAB` toggles active-tab pin; Close All but Pinned reports keep/closed counts
- Fold / hide lines: hidden lines leave the viewport
- Style-mark washes and bookmark ticks in the gutter
- Search: style mark, jump, clear, and copy-styled helpers
- Edit Cut / Copy / Paste use the session clipboard
- Placeholder / status-only menus cleared; Coming Soon stubs gone (`docs/menu-todo.md`)

### Editor UX

- Drag tabs on the tab bar to reorder (live slide; dual/compare indices remap)
- Prompt before closing unsaved tabs (Save / Don't Save / Cancel), including Close All variants, Exit, and window close
- Keep the last editor line clear of the status bar (status panel before editor + bottom padding)
- Help (?) → Changelog opens `docs/changelog.md` on GitHub
- Status bar shows `v{version}` and git short hash
- Preferences: log-tail, font size, line numbers
- CLI: open path args + `-h` / `--help`

### Earlier on `dev` (after 0.1.2)

- Command split by domain for parallel agents (`docs/agent-parallel.md`)
- Log open dialog, Help → Debug Info / Open Logs, dirty-safe tail stability

## [0.1.2] — 2026-08-28

### Menu parity (issue #1)

- Encoding menu: all `IDM_FORMAT_*` acknowledged (UTF-8 in memory)
- Edit: sort lines, remove dups, blank below, sentence case, split, Mac EOL

Ready ~337 / stub ~237 — see `docs/menu-todo.md`.

## [0.1.1] — 2026-08-28

### Progress on menu parity ([#1](https://github.com/raro42/npp-rust/issues/1))

- Inventory: `docs/menu-todo.md`
- File: Save All / Copy As / Rename / close variants / open folder & viewer / shell
- Edit: case, join/move lines, datetime, path copy helpers
- Search: set-and-find, Go to Line
- View: zoom, always-on-top, tab switch

## [0.1.0] — 2026-08-28

First public release of **npp-rs**.

### Features

- Multi-tab editor (UTF-8), open / save / recent files
- Find and replace
- Undo / redo, indent, line ops, word select
- Tree-sitter highlight (Rust, C/C++, Python, SQL, Markdown, JSON, …)
- Full Notepad++-style menu tree; ready items tinted teal; stubs show Coming Soon
- In-process plugins and format helpers
- Agent loop + public-repo privacy gates (`agents/`, `scripts/gh-safe.sh`)

### Builds

GitHub Actions release binaries for Linux, Windows, and macOS.
