# Themes

Date: 2026-08-30

Apply: egui visuals, editor chrome (bg/fg/gutter/selection/caret/whitespace/indent), and syntax token colours.

## Sources

- Built-in: `dark`, `light`
- Files in `themes/`: `*.json` (native) and `*.xml` (Notepad++ styler subset)

## JSON

See `themes/README.md`. Token overrides use tree-sitter highlight names (`keyword`, `string`, …). Nested names fall back to the base key (`string.escape` → `string`).

## XML subset

Parses GlobalStyles for chrome and one preferred LexerType WordsStyle list for tokens. Sample: `themes/mini-dark.xml`.

Limits: not full stylers.xml; unused WidgetStyle / lexer styles are ignored.
