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
| Shortcuts | Remappable (`shortcuts.xml`) | Hard-wired set + remaps (`shortcut_word_wrap`, `shortcut_find_next`, `shortcut_find_next_global`, `shortcut_next_bookmark`, `shortcut_toggle_bookmark`, `shortcut_compare`, `shortcut_swap_compare`, `shortcut_compare_to_saved`, `shortcut_copy_compare_diff`, `shortcut_copy_compare_summary`, `shortcut_copy_compare_hunk`, `shortcut_word_jump`, `shortcut_next_diff`, `shortcut_first_diff`, `shortcut_next_hidden_equal`, `shortcut_first_hidden_equal`, `shortcut_apply_compare_hunk`, `shortcut_hide_equal`, `shortcut_compare_ignore_ws`, `shortcut_compare_ignore_blank`, `shortcut_expand_all_unchanged`, `shortcut_expand_unchanged_at_caret`, `shortcut_bookmark_compare_diffs`, `shortcut_hide_equal_context`, `shortcut_goto_line`, `shortcut_duplicate_line`, `shortcut_delete_line`, `shortcut_indent`, `shortcut_outdent`, `shortcut_tab_indent`, `shortcut_shift_tab_outdent`, `shortcut_fold_all`, `shortcut_fold_current`, `shortcut_matching_brace`, `shortcut_move_line`, `shortcut_format_document`, `shortcut_close_tab`, `shortcut_next_tab`, `shortcut_new`, `shortcut_open`, `shortcut_reload`, `shortcut_save`, `shortcut_save_as`, `shortcut_save_all`, `shortcut_find`, `shortcut_close_find`, `shortcut_replace`, `shortcut_replace_alt`, `shortcut_select_all`, `shortcut_undo`, `shortcut_redo`, `shortcut_zoom_in`, `shortcut_zoom_out`, `shortcut_zoom_restore`, `shortcut_toggle_log_tail`) |
| Plugins | DLL ABI + Admin | In-process builtins only |
| Languages | 80+ + UDL | Tree-sitter subset (~7–8) |
| Platforms | Windows-first | macOS / Linux / Windows |

---

## Direct answers

### Are all hotkeys implemented?

**No.**

Hard-wired list lives in `crates/app/src/ui.rs` → `handle_shortcuts` (plus caret keys in `handle_editor_input`). Settings → Shortcut Mapper dumps that list and the effective **word wrap** / **find next** / **find next (global)** / **next bookmark** / **toggle bookmark** / **compare start/clear** / **swap compare sides** / **compare to saved** / **copy / open compare diff** / **copy / open compare summary** / **copy / open compare hunk** / **word jump** / **next difference** / **first difference** / **next hidden equal** / **first hidden equal** / **apply compare hunk** / **hide unchanged lines** / **ignore whitespace / ignore case** / **ignore blank lines** / **expand / collapse all unchanged** / **expand / collapse unchanged at caret** / **bookmark / clear compare difference bookmarks** / **hide-equal context** / **reload** / **go to line** / **duplicate line** / **delete line** / **indent** / **outdent** / **Tab indent** / **Shift+Tab outdent** / **Fold / Unfold all** / **Fold / Unfold current** / **matching brace** / **move line** / **toggle comment** / **format document** / **close tab** / **next / prev tab** / **new** / **open** / **save** / **save as** / **save all** / **print** / **find** / **close find/replace** / **replace** / **replace (alternate)** / **select all** / **undo** / **redo** / **zoom in** / **zoom out** / **zoom restore** / **toggle log tail** chords. Word wrap remaps via Preferences / `shortcut_word_wrap`; Find next via Preferences / `shortcut_find_next` (Shift flips to Find previous); Find next (global) via Preferences / `shortcut_find_next_global` (Shift flips to Find previous); Next bookmark via Preferences / `shortcut_next_bookmark` (Shift flips to previous); Toggle bookmark via Preferences / `shortcut_toggle_bookmark`; Compare start/clear via Preferences / `shortcut_compare` (Shift flips to Clear); Swap Compare sides via Preferences / `shortcut_swap_compare`; Compare to Saved via Preferences / `shortcut_compare_to_saved`; Copy / Open Compare Diff via Preferences / `shortcut_copy_compare_diff` (Shift flips to open); Copy / Open Compare Summary via Preferences / `shortcut_copy_compare_summary` (Shift flips to open); Copy / Open Compare Hunk via Preferences / `shortcut_copy_compare_hunk` (Shift flips to open); Word jump via Preferences / `shortcut_word_jump` (opposite arrow forward; Shift extends); Next difference via Preferences / `shortcut_next_diff` (Shift flips to previous); First difference via Preferences / `shortcut_first_diff` (Shift flips to last); Next hidden equal via Preferences / `shortcut_next_hidden_equal` (Shift flips to previous); First hidden equal via Preferences / `shortcut_first_hidden_equal` (Shift flips to last); Apply compare hunk via Preferences / `shortcut_apply_compare_hunk` (opposite Left/Right applies to other; Shift applies all); Hide unchanged lines via Preferences / `shortcut_hide_equal`; Ignore whitespace / Ignore case via Preferences / `shortcut_compare_ignore_ws` (Shift flips to Ignore Case); Ignore blank lines via Preferences / `shortcut_compare_ignore_blank`; Expand / Collapse all unchanged via Preferences / `shortcut_expand_all_unchanged` (Shift flips to collapse); Expand / Collapse unchanged at caret via Preferences / `shortcut_expand_unchanged_at_caret` (Shift flips to collapse); Bookmark / Clear compare difference bookmarks via Preferences / `shortcut_bookmark_compare_diffs` (Shift flips to clear); Hide-equal context via Preferences / `shortcut_hide_equal_context` (opposite bracket decreases); Go to line via Preferences / `shortcut_goto_line`; Duplicate line via Preferences / `shortcut_duplicate_line`; Delete line via Preferences / `shortcut_delete_line`; Indent via Preferences / `shortcut_indent`; Outdent via Preferences / `shortcut_outdent`; Format document via Preferences / `shortcut_format_document`; Close tab via Preferences / `shortcut_close_tab`; Next / prev tab via Preferences / `shortcut_next_tab` (Shift flips to previous); New via Preferences / `shortcut_new`; Open via Preferences / `shortcut_open`; Save via Preferences / `shortcut_save`; Save As via Preferences / `shortcut_save_as`; Save All via Preferences / `shortcut_save_all`; Print via Preferences / `shortcut_print`; Tab indent via Preferences / `shortcut_tab_indent`; Shift+Tab outdent via Preferences / `shortcut_shift_tab_outdent`; Fold / Unfold all via Preferences / `shortcut_fold_all` (Shift flips to unfold); Fold / Unfold current via Preferences / `shortcut_fold_current` (Shift flips to unfold); Matching brace via Preferences / `shortcut_matching_brace` (Shift flips to select pair); Move line via Preferences / `shortcut_move_line` (opposite Down moves down); Toggle comment via Preferences / `shortcut_toggle_comment` (Shift flips to Block Comment); Find via Preferences / `shortcut_find`; Close Find/Replace via Preferences / `shortcut_close_find`; Replace via Preferences / `shortcut_replace`; Replace (alternate) via Preferences / `shortcut_replace_alt`; Select all via Preferences / `shortcut_select_all`; Undo via Preferences / `shortcut_undo` (Shift flips to Redo); Redo via Preferences / `shortcut_redo`; Zoom in via Preferences / `shortcut_zoom_in`; Zoom out via Preferences / `shortcut_zoom_out`; Zoom restore via Preferences / `shortcut_zoom_restore` (mouse wheel stays hard-wired); Toggle log tail via Preferences / `shortcut_toggle_log_tail`. There is **no** full `shortcuts.xml` remap yet.

| Shortcut | Action |
|----------|--------|
| Cmd/Ctrl+N | New |
| Cmd/Ctrl+O | Open |
| Cmd/Ctrl+R | Reload from disk |
| Cmd/Ctrl+S | Save |
| Cmd/Ctrl+Shift+S | Save As |
| Cmd/Ctrl+Alt+S | Save All |
| Cmd/Ctrl+P (remappable) | Print |
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
| Cmd/Ctrl+B (remappable) | Matching brace (`shortcut_matching_brace`; Shift selects pair) |
| Cmd/Ctrl+Shift+Up / Down (remappable) | Move line up / down (`shortcut_move_line`) |
| F2 / Shift+F2 | Next / previous bookmark |
| Cmd/Ctrl+F2 | Toggle bookmark |
| Escape (remappable) | Close Find/Replace |
| Cmd/Ctrl+= / − / 0 | Zoom in / out / restore |
| Cmd/Ctrl+mouse wheel | Zoom |
| Alt+Z | Word wrap |
| Cmd/Ctrl+Shift+T | Toggle log tail |
| Alt+← / → (remappable) | Word jump (`shortcut_word_jump`) |

Missing vs typical Notepad++ (examples, not exhaustive):

- Macro record/play keys
- Multi-select / column mode keys
- Remappable Scintilla keys (`shortcuts.xml`)
- Fold level keys (Fold All / Fold Current remappable via `shortcut_fold_all` / `shortcut_fold_current`)

Hundreds of menu commands have **no** accelerator.

### Drag and drop of marked code?

**Partial (v0.3.8).** Drag inside a selection moves text; Ctrl/Cmd+drag copies. Drop files on the window to open. Cross-document drag and drop-into-find are still missing.

| Kind | Status |
|------|--------|
| Drag to select / double-click word / triple-click line | Done |
| Tab drag-reorder | Done |
| Document map click/drag scroll | Done | Compare colours + hunk-start ticks; Find match ticks; click parks hunk/match; drag syncs partner when sync scroll on |
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
| Doc map / function list | Partial | Density strip; Compare paints delete/insert + hunk-start ticks + hide-equal ···N gap bands on focused pane; cyan bookmark ticks + amber/green change-history ticks + click park; click parks hunk / Equal / hidden-equal gap; drag syncs partner when sync scroll on |
| RTL | Partial | Line anchors + status; chrome not mirrored |

---

## What else is missing? (ranked for product sense)

### P0 — daily editor feel

1. **More hotkeys** + broader remap (`shortcuts.xml` or settings) — hard-wired set grew in v0.3.7; word wrap remaps in v0.3.19; find next remaps in v0.3.104; next bookmark remaps in v0.3.105; toggle bookmark remaps in v0.3.112; next difference remaps in v0.3.106; go to line remaps in v0.3.107; duplicate line remaps in v0.3.108; indent remaps in v0.3.110; outdent remaps in v0.3.111; find remaps in v0.3.116; replace remaps in v0.3.117; save as remaps in v0.3.118; new remaps in v0.3.119; open remaps in v0.3.120; reload remaps in v0.3.134; select all remaps in v0.3.121; undo remaps in v0.3.122; redo remaps in v0.3.123; zoom in remaps in v0.3.125; zoom out remaps in v0.3.126; zoom restore remaps in v0.3.127; toggle log tail remaps in v0.3.128; find next (global) remaps in v0.3.129; replace (alternate) remaps in v0.3.130; next hidden equal remaps in v0.3.131; apply compare hunk remaps in v0.3.132; hide-equal context remaps in v0.3.133; reload remaps in v0.3.134; first difference remaps in v0.3.135; first hidden equal remaps in v0.3.136; hide equal remaps in v0.3.137; compare start/clear remaps in v0.3.138; close find/replace remaps in v0.3.140; swap compare remaps in v0.3.141; compare to saved remaps in v0.3.143; next/prev tab remaps in v0.3.144; save all remaps in v0.3.145; print remaps in v0.3.146; Shift+Tab outdent remaps in v0.3.147; fold all remaps in v0.3.148; fold current remaps in v0.3.149; matching brace remaps in v0.3.150; move line remaps in v0.3.151; toggle comment remaps in v0.3.152; copy/open compare diff remaps in v0.3.153; copy/open compare summary remaps in v0.3.154; copy/open compare hunk remaps in v0.3.155; Tab indent remaps in v0.3.156; ignore whitespace / ignore case remaps in v0.3.157; ignore blank lines remaps in v0.3.158; expand/collapse all unchanged remaps in v0.3.159; expand/collapse unchanged at caret remaps in v0.3.160; bookmark/clear difference bookmarks remaps in v0.3.162; full mapper still open
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
| Shortcut Mapper | Static text of hard-wired keys + effective word-wrap / find-next / find-next-global / next-bookmark / toggle-bookmark / compare / swap-compare / compare-to-saved / copy-compare-diff / copy-compare-summary / next-diff / first-diff / next-hidden-equal / first-hidden-equal / apply-compare-hunk / hide-equal / compare-ignore-ws / compare-ignore-blank / expand-all-unchanged / expand-unchanged-at-caret / hide-equal-context / reload / go-to-line / duplicate-line / delete-line / indent / outdent / tab-indent / shift-tab-outdent / fold-all / fold-current / matching-brace / move-line / toggle-comment / format / close-tab / next-tab / new / open / save / save-as / save-all / print / find / replace / replace-alt / select-all / undo / redo / zoom remaps |
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
