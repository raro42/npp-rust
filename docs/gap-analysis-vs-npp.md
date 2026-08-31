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
| Shortcuts | Remappable (`shortcuts.xml`) | ~20 hard-wired keys |
| Plugins | DLL ABI + Admin | In-process builtins only |
| Languages | 80+ + UDL | Tree-sitter subset (~7–8) |
| Platforms | Windows-first | macOS / Linux / Windows |

---

## Direct answers

### Are all hotkeys implemented?

**No.**

Hard-wired list lives in `crates/app/src/ui.rs` → `handle_shortcuts` (plus caret keys in `handle_editor_input`). Settings → Shortcut Mapper is a **read-only dump** of that list. There is **no** `shortcuts.xml` remap.

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
| Cmd/Ctrl+Shift+F | Replace |
| Cmd/Ctrl+G / Shift+G | Find next/prev **only while Find/Replace is open** |
| Escape | Close Find/Replace |
| Cmd/Ctrl+Shift+T | Toggle log tail |
| Alt+← / → | Word jump |

Missing vs typical Notepad++ (examples, not exhaustive):

- Global F3 / Shift+F3 (Find next without Find bar)
- Ctrl+H as Replace (we use Shift+F)
- Goto line, bookmarks, fold, zoom, wrap, print
- Macro record/play keys
- Multi-select / column mode keys
- Remappable Scintilla keys (`shortcuts.xml`)
- Ctrl+MouseWheel zoom (menu text mentions it; code does not)

Hundreds of menu commands have **no** accelerator.

### Drag and drop of marked code?

**No.** Selection drag only **extends** the selection (`drag_anchor`). There is no drag-to-move or drag-to-copy of selected text.

| Kind | Status |
|------|--------|
| Drag to select / double-click word / triple-click line | Done |
| Tab drag-reorder | Done |
| Document map click/drag scroll | Done |
| Drag selection to move or copy text | **Missing** |
| Drop files onto the window to open | **Missing** |

---

## Major feature areas

Legend: **Done** usable core · **Partial** real code, shallower than N++ · **Missing** no product feature

| Area | Verdict | Notes |
|------|---------|--------|
| Multi-tab / open / save / recent | Done | Solid MVP |
| Undo / redo / rope edits | Done | Coalesce + generations |
| Find / Replace (in file) | Partial | Case/word/count; no full N++ regex UI depth |
| Find in Files | Partial | Shallow cwd scan only |
| Bookmarks | Partial | Strong MVP; not full N++ mark set |
| Change history | Partial | Bars + undo remap (v0.3.5); not full Scintilla |
| Dual / other view | Partial | Writable panes; no docking layout |
| 2-way compare | Partial | Line LCS; no 3-way / char-level |
| Themes / styles | Partial | JSON + N++ XML subset (v0.3.4) |
| Encoding | Partial | UTF-8 / BOM / ANSI / UTF-16 LE·BE BOM (v0.3.6) |
| Session restore | Partial | Path list; not full N++ session XML |
| Project panel | Partial | Folder list, not N++ projects |
| Indent fold / hide lines | Partial | No lexer fold margin |
| Column / multi-edit | Partial | Insert at columns; **no** Alt-rect; typing ignores multi-carets |
| Autocomplete / call tips | Partial | In-file words/paths; no LSP popup |
| Macros | Partial | Records menu IDs only; “multi” = 3 plays |
| Plugins | Partial / Missing ABI | Builtins only |
| UDL | Missing | Menu IDs → plain |
| Hex editor | Missing | |
| FTP / cloud | Missing | |
| UI localization | Missing | Language menu = syntax, not UI lang |
| Autosave / backup | Missing | |
| Print | Partial | `lp` on saved path; no preview |
| Clipboard history | Partial | One entry, not a panel |
| Run / external tools | Partial | Pick+spawn / shell; no saved Run list |
| Updater | Partial | Opens GitHub Releases |
| Doc map / function list | Partial | Heuristic / density strip |
| RTL | Partial | Line anchors + status; chrome not mirrored |

---

## What else is missing? (ranked for product sense)

### P0 — daily editor feel

1. **More hotkeys** + optional remap (`shortcuts.xml` or settings)
2. **Global Find next** (F3) without Find bar open
3. **File drop** onto the window
4. **Selection drag** move/copy (optional with modifier)
5. **True column / rectangular select** (Alt+drag) + multi-caret typing

### P1 — power users expect from N++

6. Deeper Find in Files (recursive, filters, workspace root)
7. Lexer-aware folding + fold margin
8. Popup autocomplete / call tips (then LSP)
9. Macro: record typing + save named macros
10. Fuller Preferences (margins, default encoding, backup)
11. Autosave / backup-on-save
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
| Shortcut Mapper | Static text of hard-wired keys |
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
- **Drag marked code:** not implemented (select drag only).
- **Largest holes:** remappable keys, file/selection DnD, true column/multi-caret, plugins ABI, UDL, hex, autosave, deeper search/fold/LSP.
