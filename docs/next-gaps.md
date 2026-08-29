# Next gaps (post v0.3.2)

Date: 2026-08-29  
Status: **P0 done (v0.3.1).** Partial P1 in **v0.3.2** (project filter + ANSI warn).

## Done recently

- v0.3.1: Preferences / Find / session / compare P0
- v0.3.2: Project panel filter + remember root; ANSI lossy-save confirm

## Priority backlog

### P1 — remaining

1. **Themes** — deeper colour maps (JSON / subset of N++ XML)
2. **Change history** — closer to Scintilla block marks
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
3. Work on `dev`. Test. Changelog. Version bump. Push. Merge `main`.
