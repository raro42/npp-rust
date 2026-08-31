# UTF-16 open/save (issue #8 P1)

## Goal
Open and save UTF-16 LE/BE text files with a clear encoding path. Ship on `main`.

## Scope (this batch)
1. Read current encoding: UTF-8 / UTF-8-BOM / ANSI (Windows-1252) in `crates/fs`, `docs/encoding.md`.
2. Detect BOM for UTF-16 LE/BE on open; decode to the rope as Unicode.
3. Save as UTF-16 LE or BE when the tab encoding is set that way (menu + status).
4. Keep ANSI lossy-save confirm behaviour. Do not break UTF-8 paths.
5. Tests for round-trip + BOM. `./scripts/ci-local.sh` before push.
6. Bump version + `docs/changelog.md` when user-visible.

## Out of scope
- Full stylers.xml theme parity
- Further Scintilla change-history markers
- UTF-16 without BOM (optional later if easy)

## References
- Issue: https://github.com/raro42/npp-rust/issues/8
- `docs/next-gaps.md` (P1 Encoding)
- `docs/encoding.md`

## Privacy
No secrets or home paths in commits or issue comments.

## Progress
- Commit `61b8763` — release v0.3.6.
- Added `Utf16Le` / `Utf16Be` to `fs::TextEncoding` and `doc::FileEncoding`.
- Open: detect `FE FF` / `FF FE` BOM before UTF-8; decode via `from_utf16_lossy`.
- Save: Format + Convert-to menu set encoding; write UTF-16 with BOM.
- Tests: load LE/BE, round-trip, odd trailing byte, UTF-8 unchanged.
- Docs updated. Ready for 003 tester.
- Handoff: do not close issue #8 (BOM-less UTF-16 / stylers / Scintilla remain).
