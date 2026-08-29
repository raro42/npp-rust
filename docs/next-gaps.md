# Next gaps (post v0.3.1)

Date: 2026-08-29  
Status: **P0 from issue #7 shipped in v0.3.1.** Remaining items below.

## Done in v0.3.1 (was P0)

- Preferences depth (gutter, caret blink, EOL, recent count, session, find, compare WS)
- Find / Replace polish (case/word, match count, persist, Replace All undo/stale)
- Opt-in session restore (`npp-rs/session.txt`)
- Compare sync scroll on start + ignore whitespace

## Priority backlog

### P1 — depth on existing MVPs

1. **Themes** — load simple colour maps (JSON or subset of N++ XML) for editor + egui chrome
2. **Project panel** — remember folder roots; filter; open selected; refresh
3. **Change history** — gutter ticks closer to Scintilla (block marks on edit spans)
4. **Encoding** — optional UTF-16 LE/BE; warn before lossy ANSI save

### P2 — larger / later

5. Char-level / 3-way compare
6. Drop-in plugins
7. LSP beyond in-file call tips
8. Full UI RTL chrome
9. Architecture: `editor-core` rename, sealed Document API, property/fuzz suite

## How to pick work

1. Take the lowest open P1 item.
2. Open or update a GitHub issue. Point here or to `whats-missing.md`.
3. Work on `dev`. Test. Changelog. Version bump. Push. Merge `main` when ready to release.

## Sources

- `docs/whats-missing.md`
- `docs/menu-todo.md`
- `docs/compare.md`, `docs/themes.md`, `docs/encoding.md`
