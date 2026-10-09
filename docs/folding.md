# Lexer-aware folding and fold margin

Date: 2026-08-31  
Issue: https://github.com/raro42/npp-rust/issues/14

## Behaviour

- Gutter fold margin shows `−` (open) or `+` (folded) on fold headers.
- Click the fold margin on a header **or** any line inside a foldable region to toggle that region (same pick as View → Fold/Unfold Current).
- **Fold All / Unfold All** hotkey: Preferences → Fold all shortcut, or `shortcut_fold_all` in `npp-rs/settings.json` (default `Alt+0`; Shift flips to Unfold All).
- Preferences → Editor → **Show fold margin** (`show_fold_margin`, default on).

## Language rules

| Languages | Fold rule |
|-----------|-----------|
| rust, c, cpp, json, sql (+ js/ts/java/go ids) | `{` / `}` nesting; skips strings and `//` / `/* */` lightly |
| python, markdown, plain, others | Deeper-indent blocks |

View → Fold / Unfold / Fold level still uses the same regions.

## Limits

- Not full Scintilla fold-level chrome.
- Fold state is saved with the opt-in session file (`@folds` header lines under each path in `npp-rs/session.txt`).
- String / comment skipping is heuristic (not a full lexer).
