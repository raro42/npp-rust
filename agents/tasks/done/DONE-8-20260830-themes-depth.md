# Themes depth (issue #8 P1)

## Goal
Deepen theme colour maps beyond MVP apply. Prefer JSON theme files and/or a useful subset of Notepad++ XML themes. Ship on `main`.

## Scope (this batch)
1. Read current theme apply path (`docs/themes.md`, theme picker, settings).
2. Extend colour coverage for editor chrome and/or highlight tokens in a clear, documented way.
3. Keep Preferences / existing themes working. No Coming Soon stubs.
4. Tests where practical. `./scripts/ci-local.sh` before push.
5. Bump version + `docs/changelog.md` when user-visible.

## Out of scope for this batch
- Change history depth
- UTF-16
- Full N++ XML parity

## References
- Issue: https://github.com/raro42/npp-rust/issues/8
- `docs/next-gaps.md` (P1 #1 Themes)
- `docs/themes.md`

## Privacy
No secrets or home paths in commits or issue comments.

## Progress
- Extended JSON themes: selection, caret, whitespace, indent guide, `tokens` map.
- Paint path uses theme chrome + token colours (primary and secondary panes).
- Notepad++ XML subset: GlobalStyles + preferred lexer WordsStyle → highlight tokens.
- Samples: `themes/slate.json`, `themes/mini-dark.xml`.
- Docs/changelog; version **0.3.4**.
- Unit tests for JSON overrides and XML subset parse.
- Hand off to tester as `TEST-` (issue left open).

## Tester (003)
- Verified claims: `crates/app/src/theme.rs` (JSON tokens + XML subset), samples, docs, version 0.3.4.
- `./scripts/ci-local.sh`: **pass** (fmt, clippy, workspace tests incl. `theme::tests::*`, release build).
- Result: DONE. Issue left open for handoff.
