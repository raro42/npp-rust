# Next gaps (post v0.3.12)

Date: 2026-08-31  
Status: Issues **#8**–**#13** closed or in handoff. **#14** coder → TEST.

## Done recently

- v0.3.1–v0.3.6: Preferences, project/ANSI, themes, change-history, UTF-16
- v0.3.7: Global F3 find next/prev + more hard-wired hotkeys (#9)
- v0.3.8: File drop open + selection drag move/copy (#10)
- v0.3.9: Alt+drag rect select + multi-caret typing (#11)
- v0.3.10: Recursive Find in Files on workspace root + filters (#12)
- v0.3.11: Autosave interval + backup-on-save (#13)
- v0.3.12: Lexer-aware folding + fold margin (#14 coder)
- Gap analysis: `docs/gap-analysis-vs-npp.md`

## Open GitHub issues (loop)

| # | Priority | Title |
|---|----------|--------|
| [14](https://github.com/raro42/npp-rust/issues/14) | P1 | Lexer-aware folding + fold margin — TEST |

## Later (no issue yet)

- Popup autocomplete / call tips (then LSP)
- Macro: record typing + save named macros
- Char-level / 3-way compare; drop-in plugins; UDL; hex; UI i18n; full RTL chrome

## How to pick work

1. Lowest open issue number among open P0/P1 issues.
2. Work on `main`. Test. Changelog. Version bump. Push.
3. See `docs/gap-analysis-vs-npp.md`.
