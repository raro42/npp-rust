# Next gaps (post v0.3.0)

Date: 2026-08-29  
Status: **Batch #1–#6 closed.** This file is the improvement backlog.

## Done checkpoint

| Track | Status |
|-------|--------|
| Coming Soon stubs | None (478 menu IDs teal) |
| Issues #1–#6 | Closed |
| Release | [v0.3.0](https://github.com/raro42/npp-rust/releases/tag/v0.3.0) |

Honest partials and product depth stay in `docs/whats-missing.md`.

## Priority backlog

Ship small, real UX. Prefer one theme per release. Bump semver when you ship.

### P0 — high user value, bounded scope

1. **Preferences depth** — more tabs that write `npp-rs/settings.json` (margins, caret blink, default EOL, recent-file count). Keep UI small.
2. **Find / Replace polish** — remember last options; clearer match count; replace-all undo as one transaction if not already.
3. **Session restore** — reopen last session paths on launch (opt-in in Preferences).
4. **Compare** — side-by-side sync scroll; optional ignore-whitespace.

### P1 — depth on existing MVPs

5. **Themes** — load simple colour maps (JSON or subset of N++ XML) for editor + egui chrome.
6. **Project panel** — remember folder roots; filter; open selected; refresh.
7. **Change history** — gutter ticks closer to Scintilla (block marks on edit spans, not only lines).
8. **Encoding** — optional UTF-16 LE/BE open/save; warn before lossy ANSI save when chars map to `?`.

### P2 — larger / later

9. **Char-level / 3-way compare**
10. **Drop-in plugins** (load external libs — hard; keep listing builtins until then)
11. **LSP** beyond in-file call tips
12. **Full UI RTL chrome** (not only editor line anchors)
13. **Architecture follow-ups** from issue #3 out-of-scope: `editor-core` rename, sealed Document API, property/fuzz suite

## How to pick work

1. Take the lowest P0 that is not started.
2. Open or update a GitHub issue. Keep the body short; point here or to `whats-missing.md`.
3. Work on `dev`. Test. Changelog. Version bump. Push. Merge `main` when ready to release.

## Sources

- `docs/whats-missing.md`
- `docs/menu-todo.md`
- `docs/issue-3-architecture.md` (out-of-scope list)
- `docs/compare.md`, `docs/themes.md`, `docs/encoding.md`
