# Next gaps (post v0.3.6)

Date: 2026-08-31  
Status: **P0 done (v0.3.1).** Partial P1: project/ANSI (**v0.3.2**), themes depth (**v0.3.4**), change-history depth (**v0.3.5**), UTF-16 (**v0.3.6**).

## Done recently

- v0.3.1: Preferences / Find / session / compare P0
- v0.3.2: Project panel filter + remember root; ANSI lossy-save confirm
- v0.3.4: Theme JSON tokens + chrome; Notepad++ XML subset
- v0.3.5: Change-history bars, undo remap, CHG status
- v0.3.6: UTF-16 LE/BE BOM open/save

## Priority backlog

### P1 — remaining

1. **Themes** — deeper maps shipped in v0.3.4 (JSON tokens + N++ XML subset); full stylers.xml parity still open
2. **Change history** — bars + undo remap in v0.3.5; Scintilla reverted markers / inline indicators still open
3. **Encoding** — UTF-16 LE/BE with BOM done in v0.3.6; BOM-less UTF-16 optional later

### P2 — larger / later (see also `docs/gap-analysis-vs-npp.md`)

4. Remappable shortcuts + global Find next; file drop; selection drag-move
5. True column / rectangular select + multi-caret typing
6. Char-level / 3-way compare
7. Drop-in plugins
8. LSP beyond in-file call tips
9. UDL / hex / autosave / UI localization
10. Full UI RTL chrome
11. Architecture: `editor-core` rename, sealed Document API, property/fuzz suite

## How to pick work

1. Prefer daily-feel gaps from `docs/gap-analysis-vs-npp.md` (P0 list) or remaining depth on #8.
2. Track on GitHub issue #8 (or a new issue).
3. Work on `main`. Test. Changelog. Version bump. Push.
