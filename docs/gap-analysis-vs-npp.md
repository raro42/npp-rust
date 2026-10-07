# Gap analysis: npp-rs vs Notepad++

Date: 2026-08-31  
Basis: code under `crates/` (prefer over marketing docs).  
Principle: teal / `is_implemented` = “no Coming Soon dialog”, **not** full Notepad++ parity (`docs/menu-todo.md`).

## Big picture

Notepad++ has ~20 years of Scintilla + Win32 + plugins + community. npp-rs is a **cross-platform Rust / egui** editor with an N++-shaped menu. It is early (beta).

Expect large gaps. The menu tree looks complete. Behaviour depth does not.

| Layer | Notepad++ | npp-rs today |
|-------|-----------|--------------|
| Menu IDs | Full Win32 tree | ~478 IDs, almost all teal |
| Edit engine | Scintilla | Custom rope + egui paint |
| Shortcuts | Remappable (`shortcuts.xml`) | Hard-wired set + remaps (`shortcut_word_wrap`, `shortcut_find_next`, `shortcut_next_bookmark`, `shortcut_toggle_bookmark`, `shortcut_next_diff`, `shortcut_goto_line`, `shortcut_duplicate_line`, `shortcut_delete_line`, `shortcut_indent`, `shortcut_outdent`, `shortcut_format_document`) |
| Plugins | DLL ABI + Admin | In-process builtins only |
| Languages | 80+ + UDL | Tree-sitter subset (~7–8) |
| Platforms | Windows-first | macOS / Linux / Windows |

---

## Direct answers

### Are all hotkeys implemented?

**No.**

Hard-wired list lives in `crates/app/src/ui.rs` → `handle_shortcuts` (plus caret keys in `handle_editor_input`). Settings → Shortcut Mapper dumps that list and the effective **word wrap** / **find next** / **next bookmark** / **toggle bookmark** / **next difference** / **go to line** / **duplicate line** / **delete line** / **indent** / **outdent** / **format document** chords. Word wrap remaps via Preferences / `shortcut_word_wrap`; Find next via Preferences / `shortcut_find_next` (Shift flips to Find previous); Next bookmark via Preferences / `shortcut_next_bookmark` (Shift flips to previous); Toggle bookmark via Preferences / `shortcut_toggle_bookmark`; Next difference via Preferences / `shortcut_next_diff` (Shift flips to previous; Cmd/Ctrl+F7 first/last and Alt+F7 hidden-equal stay hard-wired); Go to line via Preferences / `shortcut_goto_line`; Duplicate line via Preferences / `shortcut_duplicate_line`; Delete line via Preferences / `shortcut_delete_line`; Indent via Preferences / `shortcut_indent`; Outdent via Preferences / `shortcut_outdent`; Format document via Preferences / `shortcut_format_document`. There is **no** full `shortcuts.xml` remap yet.

| Shortcut | Action |
|----------|--------|
| Cmd/Ctrl+N | New |
| Cmd/Ctrl+O | Open |
| Cmd/Ctrl+S | Save |
| Cmd/Ctrl+Shift+S | Save As |
| Cmd/Ctrl+W | Close tab |
| Cmd/Ctrl+Z | Undo |
| Cmd/Ctrl+Shift+Z / Y | Redo |
| Cmd/Ctrl+A | Select all |
| Cmd/Ctrl+D | Duplicate line |
| Cmd/Ctrl+Shift+L | Delete line |
| Cmd/Ctrl+] / [ | Indent / outdent |
| Cmd/Ctrl+Shift+I | Format Document |
| Cmd/Ctrl+F | Find |
| Cmd/Ctrl+H / Shift+F | Replace |
| F3 / Shift+F3 | Find next/prev (**global**) |
| Cmd/Ctrl+G / Shift+G | Find next/prev (**global**) |
| Cmd/Ctrl+L | Go to line |
| F2 / Shift+F2 | Next / previous bookmark |
| Cmd/Ctrl+F2 | Toggle bookmark |
| Escape | Close Find/Replace |
| Cmd/Ctrl+= / − / 0 | Zoom in / out / restore |
| Cmd/Ctrl+mouse wheel | Zoom |
| Alt+Z | Word wrap |
| Cmd/Ctrl+Shift+T | Toggle log tail |
| Alt+← / → | Word jump |

Missing vs typical Notepad++ (examples, not exhaustive):

- Macro record/play keys
- Multi-select / column mode keys
- Remappable Scintilla keys (`shortcuts.xml`)
- Fold margin keys / print accelerator

Hundreds of menu commands have **no** accelerator.

### Drag and drop of marked code?

**Partial (v0.3.8).** Drag inside a selection moves text; Ctrl/Cmd+drag copies. Drop files on the window to open. Cross-document drag and drop-into-find are still missing.

| Kind | Status |
|------|--------|
| Drag to select / double-click word / triple-click line | Done |
| Tab drag-reorder | Done |
| Document map click/drag scroll | Done |
| Drag selection to move or copy text | Done (same buffer; Ctrl/Cmd = copy) |
| Drop files onto the window to open | Done |

---

## Major feature areas

Legend: **Done** usable core · **Partial** real code, shallower than N++ · **Missing** no product feature

| Area | Verdict | Notes |
|------|---------|--------|
| Multi-tab / open / save / recent | Done | Solid MVP |
| Undo / redo / rope edits | Done | Coalesce + generations |
| Find / Replace (in file) | Partial | Case/word/count/wrap/sel + linear-time regex + `$n` replace; not full N++ replace UI |
| Find in Files | Partial | Recursive workspace scan + include/exclude globs + Find **Re** (v0.3.43); not full N++ UI |
| Bookmarks | Partial | Strong MVP; not full N++ mark set |
| Change history | Partial | Bars + undo remap (v0.3.5); not full Scintilla |
| Dual / other view | Partial | Writable panes; no docking layout |
| 2-way compare | Partial | Line LCS + word-aware wash + hide unchanged (···N gaps, click expand, Expand at Caret / Next·Prev·First·Last Hidden Equal / Expand/Collapse All) + bookmark / clear hunk-start bookmarks + apply hunk from/to other / apply all + open unified diff tab; no 3-way |
| Themes / styles | Partial | JSON + N++ XML subset (v0.3.4) |
| Encoding | Partial | UTF-8 / BOM / ANSI / UTF-16 LE·BE (BOM or no-BOM detect, v0.3.17) |
| Session restore | Partial | Path list + folded fold-header lines (`@folds`); not full N++ session XML |
| Project panel | Partial | Folder list + refresh/reveal; not N++ projects |
| Indent fold / hide lines | Partial | Fold margin + brace/indent heuristics; session restores folded headers; not full Scintilla |
| Column / multi-edit | Partial | Alt+rect + multi-caret typing (v0.3.9); no virtual space |
| Autocomplete / call tips | Partial | In-file words/paths; no LSP popup |
| Macros | Partial | Records menu IDs only; “multi” = 3 plays |
| Plugins | Partial / Missing ABI | Builtins only |
| UDL | Missing | Menu IDs → plain |
| Hex editor | Missing | |
| FTP / cloud | Missing | |
| UI localization | Missing | Language menu = syntax, not UI lang |
| Autosave / backup | MVP: prefs + `npp-rs/backup/` + interval autosave (`docs/autosave-backup.md`) | |
| Print | Partial | `lp` on saved path; no preview |
| Clipboard history | Partial | One entry, not a panel |
| Run / external tools | Partial | Pick+spawn / shell; no saved Run list |
| Updater | Partial | Opens GitHub Releases |
| Doc map / function list | Partial | Heuristic / density strip |
| RTL | Partial | Line anchors + status; chrome not mirrored |

---

## What else is missing? (ranked for product sense)

### P0 — daily editor feel

1. **More hotkeys** + broader remap (`shortcuts.xml` or settings) — hard-wired set grew in v0.3.7; word wrap remaps in v0.3.19; find next remaps in v0.3.104; next bookmark remaps in v0.3.105; toggle bookmark remaps in v0.3.112; next difference remaps in v0.3.106; go to line remaps in v0.3.107; duplicate line remaps in v0.3.108; indent remaps in v0.3.110; outdent remaps in v0.3.111; full mapper still open
2. **Global Find next** (F3) — **done** in v0.3.7
3. **File drop** onto the window — **done** in v0.3.8
4. **Selection drag** move/copy — **done** in v0.3.8 (same-buffer; Ctrl/Cmd = copy)
5. **True column / rectangular select** (Alt+drag) + multi-caret typing — **done** in v0.3.9 (no virtual space)

### P1 — power users expect from N++

6. Deeper Find in Files (recursive, filters, workspace root) — **done** in v0.3.10 (MVP; not full N++ dialog)
7. Lexer-aware folding + fold margin — **done MVP** in v0.3.12
8. Popup autocomplete / call tips (then LSP)
9. Macro: record typing + save named macros
10. Fuller Preferences (margins, default encoding, backup)
11. Autosave / backup-on-save — **done MVP** (v0.3.11)
12. Deeper Style Config / stylers.xml

### P2 — large / ecosystem

13. Drop-in plugins (or a clear “no ABI” product rule)
14. User Defined Language (UDL)
15. Hex view
16. 3-way / char-level compare
17. UI translations
18. Print preview / better print
19. Docking / multi-panel layout
20. FTP / remote (usually plugins in N++)

---

## Menu coverage trap

| Claim | Reality |
|-------|---------|
| “478 IDs implemented / 0 stubs” | Handlers exist; many are MVP or status-only |
| Shortcut Mapper | Static text of hard-wired keys + effective word-wrap / find-next / next-bookmark / toggle-bookmark / next-diff / go-to-line / duplicate-line / delete-line / indent / outdent remaps |
| Plugin Admin | Lists builtins; does not load plugins |
| Column mode tip | Tip string; no Scintilla rect mode |

See `docs/menu-todo.md`, `docs/whats-missing.md`. Refresh `docs/scope.md` when you change the product story (it can lag).

---

## Code index

| Area | Paths |
|------|--------|
| Shortcuts / select drag / dual view | `crates/app/src/ui.rs`, `ui_paint.rs` |
| Shortcut Mapper dump | `crates/app/src/commands/misc.rs` → `show_shortcut_mapper` |
| Commands | `crates/app/src/commands/*.rs` |
| Menu data | `crates/app/data/npp_menu.json` |
| Multi-sels / bookmarks / folds | `crates/doc`, `crates/buffer` |
| Encoding | `crates/fs` |
| Highlight | `crates/highlight` |
| Plugins | `crates/plugins` |
| Themes | `crates/app/src/theme.rs`, `themes/` |
| Ranked short backlog | `docs/next-gaps.md` |

---

## Bottom line

npp-rs is a serious **early** editor with an N++ menu shell. It is **not** a 20-year feature clone.

- **Hotkeys:** small hard-wired set — not all N++ keys.
- **Drag marked code:** move/copy in same buffer (v0.3.8); no cross-doc drag yet.
- **Largest holes:** remappable keys, plugins ABI, UDL, hex, deeper fold/LSP.
