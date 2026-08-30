# Themes

Apply from Preferences or Import Style Theme(s).

## Formats

| Kind | Extension | What applies |
|------|-----------|----------------|
| Native JSON | `.json` | egui dark/light, editor chrome, selection/caret, whitespace, indent guides, syntax `tokens` |
| N++ subset | `.xml` | GlobalStyles chrome + one lexer WordsStyle map (prefers cpp / rust / python / c) |

## JSON keys

- Chrome: `egui`, `bg`, `fg`, `gutter`, `gutter_line`, `line_number`, `selection`, `caret`, `whitespace`, `indent_guide`
- Tokens: `tokens` map of tree-sitter highlight names (`keyword`, `string`, `comment`, …) to `[r,g,b]`

Unset keys keep the built-in dark or light base.

## XML subset

Optional hint: `<?npp-rs name="My Theme"?>`.

Reads `WidgetStyle` names such as Default Style, Line number margin, Selected text colour, Caret colour, White space symbol, Indent guideline style.

Maps common WordsStyle names (COMMENT, STRING, INSTRUCTION WORD, …) onto highlight token keys.

Full Notepad++ stylers.xml parity is out of scope.
