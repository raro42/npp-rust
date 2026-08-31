# Next gaps (post v0.3.9)

Date: 2026-08-31  
Status: Issue **#8**–**#10** closed. **#11** coder batch in v0.3.9 (await TEST). Open: **#12–#14**.

## Done recently

- v0.3.1–v0.3.6: Preferences, project/ANSI, themes, change-history, UTF-16
- v0.3.7: Global F3 find next/prev + more hard-wired hotkeys (#9)
- v0.3.8: File drop open + selection drag move/copy (#10)
- v0.3.9: Alt+drag rect select + multi-caret typing (#11 coder)
- Gap analysis: `docs/gap-analysis-vs-npp.md`

## Open GitHub issues (loop)

| # | Priority | Title |
|---|----------|--------|
| [11](https://github.com/raro42/npp-rust/issues/11) | P0 | Column/rect select + multi-caret typing — coding done, test pending |
| [12](https://github.com/raro42/npp-rust/issues/12) | P1 | Deeper Find in Files |
| [13](https://github.com/raro42/npp-rust/issues/13) | P1 | Autosave / backup-on-save |
| [14](https://github.com/raro42/npp-rust/issues/14) | P1 | Lexer-aware folding + fold margin |

## Later (no issue yet)

- Char-level / 3-way compare; drop-in plugins; LSP; UDL; hex; UI i18n; full RTL chrome

## How to pick work

1. Lowest open issue number among #11–#14 (P0 before P1).
2. Work on `main`. Test. Changelog. Version bump. Push.
3. See `docs/gap-analysis-vs-npp.md`.
