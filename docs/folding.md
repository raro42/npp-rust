# Lexer-aware folding and fold margin

Date: 2026-08-31  
Issue: https://github.com/raro42/npp-rust/issues/14

## Behaviour

- Gutter fold margin shows `−` (open) or `+` (folded) on fold headers.
- Click a marker to fold or unfold that region.
- Preferences → Editor → **Show fold margin** (`show_fold_margin`, default on).

## Language rules

| Languages | Fold rule |
|-----------|-----------|
| rust, c, cpp, json, sql (+ js/ts/java/go ids) | `{` / `}` nesting; skips strings and `//` / `/* */` lightly |
| python, markdown, plain, others | Deeper-indent blocks |

View → Fold / Unfold / Fold level still uses the same regions.

## Limits

- Not full Scintilla fold-level chrome.
- Fold state is not saved across sessions.
- String / comment skipping is heuristic (not a full lexer).
