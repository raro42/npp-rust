# What’s missing (after stub sweep)

Date: 2026-08-29  
**Checkpoint:** Issues #1–#6 closed. **v0.3.1** shipped issue #7 P0. Next: `docs/next-gaps.md` (P1).

## Stubs (Coming Soon dialog)

**None.** All export menu IDs are marked implemented (teal). No grey “Coming Soon” stubs remain.

## Honest partials (work, but not full Notepad++)

Handlers that work but stay shallower than upstream N++:

| Area | Gap |
|------|-----|
| View | Dual view: both panes writable; focused-pane Edit. Project panel = folder list + refresh/reveal (not N++ projects) |
| Edit | RTL/LTR: editor line anchors + status cue; full bidi / UI chrome mirror still open |
| Encoding | ANSI / UTF-8 / UTF-8-BOM / UTF-16 LE-BE per-tab save. Open detects UTF-16 without a BOM when most units have a zero high byte. Unmapped chars → `?` on ANSI save |
| Search | Change History: amber/green gutter bars + wash; undo remap; CHG status. Find has case/word + match count + in-selection (multi-line auto-arm) + wrap + linear-time regex + `$n` / `\n` replace. Find in Files uses the same **Re** toggle (linear-time); not full N++ FIF dialog |
| Settings | Themes: JSON tokens + chrome; N++ XML subset (GlobalStyles + one lexer); full stylers parity open; plugins: builtins listed, no drop-in load. Preferences deeper in v0.3.1. Word wrap + Find next + Next bookmark + Toggle bookmark + Next difference + Go to line + Duplicate line + Delete line + Indent + Outdent + Format document + Close tab + New + Open + Save + Save As + Find + Replace + Select all + Undo + Redo + Zoom in + Zoom out + Zoom restore shortcuts remappable; View symbol toggles (whitespace/EOL/NPC/indent guide) persist. Unknown `settings.json` keys survive save. Full shortcut mapper / `shortcuts.xml` open |
| File | Pin from tab chrome + `IDM_PINTAB`; Close All but Pinned keeps pinned tabs; opt-in session restore (paths + folded headers) |
| View | Tab drag-reorder; file drop open; selection drag move/copy (Ctrl/Cmd = copy) |

## Larger product gaps (not menu stubs)

- Full Preferences (multi-language UI, …). More keys in `npp-rs/settings.json` (see `docs/preferences-p0.md`). Autosave/backup MVP: `docs/autosave-backup.md`
- Project panels: folder list + refresh/reveal; not N++ projects
- File compare: **2-way** + re-diff + ignore-whitespace + ignore-case + ignore-blank (View toggles; live ignore re-diff parks first hunk · at L|R) + **hide unchanged lines (toggle parks first hunk · at L|R; Preferences hide-equal context 0–10 (change parks first hunk · at L|R), ···N gap cues, click-to-expand both panes + Expand/Collapse Unchanged at Caret (status −/+/kind tallies), Next/Previous/First/Last Hidden Equal (Alt+F7 family hotkeys), Expand/Collapse All Unchanged Lines (status −/+/kind tallies), hidden counts)** + next/prev/first/last hunk + ordinal status **with current hunk kind and −/+ counts** + dual-pane hunk align + identical status + swap sides (re-diff parks first hunk · at L|R) + click/keyboard hunk sync (L|R status) + **Equal-line partner park (status −/+/kind tallies · N% equal · ignore + hide-equal bits)** + both-pane hunk select + **bookmark / clear compare difference bookmarks (status −/+/kind tallies)** + **copy unified diff** + **open unified diff tab** + **copy compare summary** + **open compare summary (multi-line previews + kind tallies + ignore flags + equal-line match percent)** + **live status · K hunks: kind tallies · N% equal** + **copy current hunk** + **open current hunk tab** + **copy/open hunk status kind/−/+** + **copy/open summary status kind tallies** + **copy/open unified diff status kind tallies** + **apply hunk from other view** (⌘/Ctrl+Alt+←) + **apply hunk to other view** (⌘/Ctrl+Alt+→) + **apply all hunks from/to other view** (⌘/Ctrl+Alt+Shift+←/→) + **apply hunk status kind/−/+** + **apply all hunks status kind/−/+** + **word-aware wash on replace hunks** + **compare to saved (disk snapshot)** + **clear closes saved snapshot** + **clear status −/+/kind tallies** + **edit re-diff parks partner + · at L|R / · equal L|R** + **Equal partner park reveals hide-equal collapsed lines**. soft-truncates oversize pairs to first 3000 lines (`· first 3000 lines`). 3-way still open
- Change history: bar marks + undo remap; full Scintilla reverted/indicator parity open
- CLI MVP: `-h`, `-V`, `-n`/`--line`, `-ro`, path args
- LSP (call tips: in-file snippets only)

**Ranked backlog:** `docs/next-gaps.md`.  
**Deep vs Notepad++:** `docs/gap-analysis-vs-npp.md` (hotkeys, DnD, feature table).  
**Inventory:** `docs/menu-todo.md`.
