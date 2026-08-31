# Next gaps (post v0.3.5)

Date: 2026-08-31  
Status: **P0 done (v0.3.1).** Partial P1: project/ANSI (**v0.3.2**), themes depth (**v0.3.4**), change-history depth (**v0.3.5**).

## Done recently

- v0.3.1: Preferences / Find / session / compare P0
- v0.3.2: Project panel filter + remember root; ANSI lossy-save confirm
- v0.3.4: Theme JSON tokens + chrome; Notepad++ XML subset
- v0.3.5: Change-history bars, undo remap, CHG status

## Priority backlog

### P1 — remaining

1. **Themes** — deeper maps shipped in v0.3.4 (JSON tokens + N++ XML subset); full stylers.xml parity still open
2. **Change history** — bars + undo remap in v0.3.5; Scintilla reverted markers / inline indicators still open
3. **Encoding** — UTF-16 LE/BE open/save (warn on lossy ANSI is done)

### P2 — larger / later

4. Char-level / 3-way compare
5. Drop-in plugins
6. LSP beyond in-file call tips
7. Full UI RTL chrome
8. Architecture: `editor-core` rename, sealed Document API, property/fuzz suite

## How to pick work

1. Take the lowest open P1 item.
2. Track on GitHub issue #8 (or a new issue).
3. Work on `main`. Test. Changelog. Version bump. Push.
