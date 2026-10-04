# Changelog

## [Unreleased]

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
