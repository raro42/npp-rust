# Next gaps (post v0.3.286)

Date: 2026-10-14  
Status: Issues **#1**–**#20** closed or handed off (this refresh). Overnight idle work: `docs/autoresearch/standing_backlog.md`.

Teal / `is_implemented` still means “no Coming Soon dialog”, not full Notepad++ depth. See `docs/whats-missing.md` and `docs/gap-analysis-vs-npp.md`.

## Done since next-gaps last claimed “open” (#8–#20 + keeps)

Wave that closed the old table (#8–#14) and later product depth through **v0.3.248**:

| Wave | What shipped (short) |
|------|----------------------|
| v0.3.1–v0.3.6 | Preferences, project/ANSI, themes, change-history, UTF-16 |
| v0.3.7–v0.3.12 | Global F3, file drop + selection drag, Alt+rect / multi-caret, Find in Files, autosave/backup, fold margin (#9–#14) |
| v0.3.13–v0.3.18 | About, scrollbar, submenu size, Ollama helper (#15–#18); Find wrap / regex / Sel auto-arm; BOM-less UTF-16; multi-caret coalesce |
| v0.3.19–v0.3.163 | Large remappable shortcut set + deep **2-way Compare** (ignore flags, hide-equal ···N, apply hunk From/To, summary/diff/hunk tabs, Compare to Saved, soft-truncate, …) |
| v0.3.164–v0.3.214 | Scrollable menus; Document Map ticks (compare / find / fold / bookmarks / CH / selection); more remaps (map, FIF, focus other view, case, join, …) |
| v0.3.215–v0.3.241 | Dual-view other-pane syntax; rich **session restore** (`@folds`…`@partner`); persist Always on Top / Document Map / Project panel / Zoom Sync / Post-It; Post-It hides menu+status |
| v0.3.242 | Ask Ollama / Status on background thread so the window does not stall (#19) |
| v0.3.243 | Refresh this file: ranked remaining gaps; drop stale open-#14 table (#20) |
| v0.3.244–v0.3.247 | Column virtual space: Alt+drag pad/copy + past EOL; plain free caret; stream Shift-select |
| v0.3.248 | Plain arrows move all multi-carets / rect carets together |
| v0.3.249 | Ctrl/Cmd+click add / toggle multi-carets |
| v0.3.250 | Shift+extend stream multi-carets (Left/Right/Up/Down) |
| v0.3.251 | Rect Shift+extend (Left/Right/Up/Down) |
| v0.3.252 | Multi-caret word-jump move + Shift-extend |
| v0.3.253 | Multi-caret line Home/End move + Shift-extend |
| v0.3.254 | Multi-caret Page Up/Down move + Shift-extend |
| v0.3.255 | Escape clears multi-carets / rect when Find closed |
| v0.3.256–v0.3.258 | Remappable Close All / Trim Trailing; multi-caret paste line-split |
| v0.3.259 | Remappable Multi-select All (`shortcut_multi_select_all`) |
| v0.3.260 | Begin/End Select column mode = real rect; remappable (`shortcut_begin_end_select`) |
| v0.3.261 | Begin/End Select column mode virtual cols past EOL (+ stream end in virtual space) |
| v0.3.262 | Rect Shift+Home/End extend (anchor corner + col 0 / line body end) |
| v0.3.263 | Alt+Shift+arrows start / extend rectangular column select |
| v0.3.264 | Document Home/End move / Shift-extend multi-carets / rect |
| v0.3.265 | Remappable Toggle Read-Only (`shortcut_toggle_readonly`) |
| v0.3.266 | Remappable Column Editor (`shortcut_column_editor`) |
| v0.3.267 | Remappable Sort Lines Lexicographic Asc/Desc (`shortcut_sort_lines`) |
| v0.3.268 | Remappable Remove Empty Lines / Containing Blank (`shortcut_remove_empty_lines`) |
| v0.3.269 | Remappable Remove Consecutive / Any Duplicate Lines (`shortcut_remove_dup_lines`) |
| v0.3.270 | Remappable Date Time short/long (`shortcut_insert_datetime`) |
| v0.3.271 | Remappable Copy Full Path / File Name (`shortcut_copy_path`) |
| v0.3.272 | Remappable Copy Current Directory Path (`shortcut_copy_dir`) |
| v0.3.273 | Remappable Copy All Paths / All Filenames (`shortcut_copy_all_paths`) |
| v0.3.274 | Remappable Open Selected Filename / Containing Folder (`shortcut_open_selected`) |
| v0.3.275 | Remappable Open Containing Folder / Shell (`shortcut_open_folder`) |
| v0.3.276 | Remappable Open Folder as Workspace / Containing Folder (`shortcut_open_workspace`) |
| v0.3.277 | Remappable Close All but Pinned / Close All Unchanged (`shortcut_close_but_pinned`) |
| v0.3.278 | Remappable Close All to the Left / Right (`shortcut_close_to_side`) |
| v0.3.279 | Remappable Save a Copy As / Rename (`shortcut_save_copy`) |
| v0.3.280 | Remappable Function List / Character Panel (`shortcut_function_list`) |
| v0.3.281 | Remappable Move Tab Forward / Backward (`shortcut_move_tab`) |
| v0.3.282 | Remappable Select and Find Next / Previous (`shortcut_set_and_find`) |
| v0.3.283 | Remappable Mark Style 1 / Clear Style 1 (`shortcut_mark_style`) |
| v0.3.284 | Remappable Mark Style 2 / Clear Style 2 (`shortcut_mark_style_2`) |
| v0.3.285 | Remappable Mark Style 3 / Clear Style 3 (`shortcut_mark_style_3`) |
| v0.3.286 | Remappable Mark Style 4 / Clear Style 4 (`shortcut_mark_style_4`) |

Honest partials (still shallower than N++) live in `docs/whats-missing.md`. Compare detail: `docs/compare.md`.

## Open GitHub issues (loop)

None. When the issue queue is empty, 009 picks the first non-struck item in `docs/autoresearch/standing_backlog.md`.

## Still missing (ranked — real gaps)

### P0 — daily editor feel

1. **Full shortcut mapper / `shortcuts.xml`** — most common chords remappable via `shortcut_*` settings keys; dump in Shortcut Mapper / About. Still no N++-style full remap file, and many menu commands still have no accelerator.
2. **Column / multi-edit depth** — Alt+rect + multi-caret typing + **pad-on-type / pad-on-copy** + **Alt+drag click/drag past EOL** + **plain free caret past EOL** (click/Right + pad-on-type) + **stream select into virtual space** (Shift+click/drag + Shift+Right/Left at EOL) + **plain arrows move all multi-carets** + **Ctrl/Cmd+click add carets** + **Shift+extend stream multi** + **Rect Shift+extend** + **word-jump move / Shift-extend multi-carets** + **line Home/End move / Shift-extend multi-carets** + **Rect Shift+Home/End extend** + **Page Up/Down move / Shift-extend multi-carets** + **Escape clears multi-carets / rect when Find closed** + **paste line-split when clipboard lines match caret count** + **Begin/End Select column mode virtual cols** + **Alt+Shift+arrows start/extend rect** + **Document Home/End move / Shift-extend multi-carets / rect**. Further column polish (`docs/column-mode.md`).
3. **Find / Replace UI depth** — case/word/count/wrap/sel + linear-time Re + `$n` replace work; not the full N++ Find/Replace dialog (filters, mark-all panel, etc.).
4. **Find in Files UI** — recursive workspace + globs + Re; not full N++ FIF dialog.

### P1 — power users expect from N++

5. **Popup autocomplete / call tips → LSP** — in-file words/paths only today.
6. **Macros** — record menu IDs / “multi” replay only; no typing record or named saved macros.
7. **Preferences / Style Config depth** — solid `settings.json` surface (`docs/preferences-p0.md`); not multi-language UI, full stylers.xml parity, or N++ Preferences tabs.
8. **Change history** — amber/green bars + undo remap + F8 next/prev; not full Scintilla reverted / indicator parity.
9. **Folding** — margin + brace/indent + session `@folds`; not full Scintilla nested fold chrome.
10. **Project panel** — folder list + refresh/reveal + persist open; not N++ project files / workspace trees.
11. **Session** — rich opt-in `session.txt` meta; not full N++ session XML.
12. **Dual view layout** — writable panes + sync + compare; no docking / multi-panel shell.
13. **Document Map / Function list** — density strip + compare/find/fold/bookmark/CH/selection ticks; no Function List panel.
14. **RTL** — line anchors + status + persist + remap; full bidi / UI chrome mirror still open.
15. **Print** — `lp` on saved path; no preview.
16. **Clipboard history** — one entry, not a panel.
17. **Run / external tools** — pick+spawn / shell; no saved Run list.
18. **Themes** — JSON + N++ XML subset; full stylers / multi-lexer import parity open.
19. **Tree-sitter languages** — small subset vs N++ 80+ lexers.
20. **Ollama helper** — Ask/Status (background, loopback); deeper agent / apply-edit later (`docs/ollama-helper.md`).

### P2 — large / ecosystem

21. Drop-in plugins (or an explicit “no DLL ABI” product rule) — builtins + Plugin Admin list only.
22. User Defined Language (UDL).
23. Hex editor.
24. **3-way** compare / richer syntax-aware inline diff (2-way is deep; see `docs/compare.md` “Not in MVP”).
25. UI localization (Language menu = syntax, not UI lang).
26. FTP / remote / cloud (usually plugins in N++).
27. Updater beyond opening GitHub Releases.

## How to pick work

1. Lowest open GitHub issue (P0/P1) still wins.
2. Else first non-struck item in `docs/autoresearch/standing_backlog.md`.
3. Work on `main`. Test. Changelog. Version bump when shipping user-visible change. Push after `./scripts/ci-local.sh`.
4. Deeper tables: `docs/gap-analysis-vs-npp.md`. Honest partials: `docs/whats-missing.md`. Inventory: `docs/menu-todo.md`.
