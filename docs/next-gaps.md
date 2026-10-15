# Next gaps (post v0.3.294)

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
| v0.3.287 | Remappable Mark Style 5 / Clear Style 5 (`shortcut_mark_style_5`) |
| v0.3.288 | Remappable Clear all styles (`shortcut_clear_all_styles`) |
| v0.3.289 | Remappable Inverse Bookmarks (`shortcut_inverse_bookmarks`) |
| v0.3.290 | Remappable Zoom Sync (`shortcut_zoom_sync`) |
| v0.3.291 | Remappable rect column select (`shortcut_rect_select`; Alt+Shift+arrows) |
| v0.3.292 | Remappable Fold Level 1…8 (`shortcut_fold_level`; Alt+1…8; Shift unfolds) |
| v0.3.293 | Remappable Autocomplete / Function Call Tip (`shortcut_autocomplete`; Cmd+Space; Shift = call tip) |
| v0.3.294 | Remappable Path autocomplete (`shortcut_autocomplete_path`; Cmd+Alt+Space) |
| v0.3.295 | Remappable Next / Previous Search Result (`shortcut_next_found`; F4; Shift=prev) |
| v0.3.296 | Remappable First / Last Tab (`shortcut_first_tab`; Cmd+Alt+Home; opposite End = last) |
| v0.3.297 | Remappable TAB to Space / Space to TAB Leading (`shortcut_tab_to_space`; Cmd+Alt+V; Shift=leading) |
| v0.3.298 | Remappable Copy / Cut Bookmarked Lines (`shortcut_copy_marked_lines`; Cmd+Alt+F2; Shift=cut) |
| v0.3.299 | Remappable Paste / Remove Bookmarked Lines (`shortcut_paste_marked_lines`; Cmd+Alt+Q; Shift=remove) |
| v0.3.300 | Remappable Space to TAB (All) (`shortcut_space_to_tab_all`; Cmd+Alt+X) |
| v0.3.301 | Find-bar **Mark** + Style All Occurrences honor Case/Word/Re/Sel |
| v0.3.302 | Find-bar **Bookmark** all matching lines (Case/Word/Re/Sel) |
| v0.3.303 | Find-bar **Copy** matching substrings (Case/Word/Re/Sel) |
| v0.3.304 | Find-bar **Cut** matching substrings (Case/Word/Re/Sel; deletes matches) |
| v0.3.305 | Find-bar **Purge** matching lines (Case/Word/Re/Sel; whole lines) |
| v0.3.306 | Find-bar **Lines** copy matching lines (Case/Word/Re/Sel; whole lines, deduped) |
| v0.3.307 | Find-bar **Shift+Cut** cut matching lines (Case/Word/Re/Sel; whole lines, clipboard + delete) |
| v0.3.308 | Find-bar **Shift+Purge** delete matching substrings (Case/Word/Re/Sel; no clipboard) |
| v0.3.309 | Find-bar **All** / Search Results Window → current-doc Find Results tab (Case/Word/Re/Sel) |
| v0.3.310 | Find-bar **Shift+All** → Find All in opened editor tabs (Case/Word/Re; Sel ignored) |
| v0.3.311 | Next / Previous Search Result walks Find Results hits (F4; multi-doc; wrap status) |
| v0.3.312 | Next / Previous Search Result walks Find in Files hits (open from workspace; wrap status) |
| v0.3.313 | Find Results / Find in Files double-click hit → open + park match |
| v0.3.314 | Remappable Find All / Find All in opened (`shortcut_find_all`; Cmd+Alt+F4; Shift=opened) |
| v0.3.315 | Find Results / Find in Files Enter on hit row → open + park match |
| v0.3.316 | Find Results / Find in Files dual-view secondary double-click hit jump |
| v0.3.317 | Find-bar Shift+Enter = Find Previous |
| v0.3.318 | Replace-bar Enter = Replace / Shift+Enter = Replace All |
| v0.3.319 | Remappable Next / Previous Style marker (`shortcut_next_style`; Cmd+Alt+1…5; Shift=prev) |
| v0.3.320 | Find Results / Find in Files Shift+Enter = Previous Search Result |
| v0.3.321 | Find-bar Incremental Search (Inc; as-you-type jump) |
| v0.3.322 | Remappable Remove Unmarked Lines (`shortcut_remove_unmarked`; Cmd+Alt+Z default) |
| v0.3.323 | Remappable Incremental Search (`shortcut_find_incremental`; Alt+Shift+I default) |
| v0.3.324 | Remappable Clear Change History (`shortcut_clear_change_history`; Cmd+Shift+F8 default) |
| v0.3.325 | Remappable Copy Styled Text (`shortcut_style_to_clip`; Cmd+Alt+Shift+1…5; 0 = all) |
| v0.3.326 | Remappable Mark current line Style (`shortcut_mark_one_style`; Alt+Shift+1…5) |
| v0.3.327 | Remappable Find Mark (`shortcut_find_mark`; Alt+Shift+F3 default) |
| v0.3.328 | Remappable Find Bookmark (`shortcut_find_bookmark`; Alt+Shift+F4 default) |
| v0.3.329 | Remappable Find Copy / matching lines (`shortcut_find_copy`; Alt+Shift+F5 default; Shift=lines) |

Honest partials (still shallower than N++) live in `docs/whats-missing.md`. Compare detail: `docs/compare.md`.

## Open GitHub issues (loop)

None. When the issue queue is empty, 009 picks the first non-struck item in `docs/autoresearch/standing_backlog.md`.

## Still missing (ranked — real gaps)

### P0 — daily editor feel

1. **Full shortcut mapper / `shortcuts.xml`** — most common chords remappable via `shortcut_*` settings keys (including Fold Level 1…8, Autocomplete / Call Tip, Path autocomplete, Next / Previous Search Result, Find All / Find All in opened, Incremental Search, Find Mark, Find Bookmark, Find Copy / matching lines, First / Last Tab, TAB to Space / Space to TAB Leading, Space to TAB (All), Copy / Cut Bookmarked Lines, Paste / Remove Bookmarked Lines, Remove Unmarked Lines, Clear Change History, Next / Previous Style marker 1…5, Copy Styled Text 1…5 / All Styles, and Mark current line Style 1…5); dump in Shortcut Mapper / About. Still no N++-style full remap file, and many menu commands still have no accelerator.
2. **Column / multi-edit depth** — Alt+rect + multi-caret typing + **pad-on-type / pad-on-copy** + **Alt+drag click/drag past EOL** + **plain free caret past EOL** (click/Right + pad-on-type) + **stream select into virtual space** (Shift+click/drag + Shift+Right/Left at EOL) + **plain arrows move all multi-carets** + **Ctrl/Cmd+click add carets** + **Shift+extend stream multi** + **Rect Shift+extend** + **word-jump move / Shift-extend multi-carets** + **line Home/End move / Shift-extend multi-carets** + **Rect Shift+Home/End extend** + **Page Up/Down move / Shift-extend multi-carets** + **Escape clears multi-carets / rect when Find closed** + **paste line-split when clipboard lines match caret count** + **Begin/End Select column mode virtual cols** + **Alt+Shift+arrows start/extend rect** (remappable `shortcut_rect_select`) + **Document Home/End move / Shift-extend multi-carets / rect**. Further column polish (`docs/column-mode.md`).
3. **Find / Replace UI depth** — case/word/count/wrap/sel + linear-time Re + `$n` replace + Find-bar **Mark** (Style 1; Case/Word/Re/Sel) + Find-bar **Bookmark** all matching lines + Find-bar **Copy** matching substrings + Find-bar **Lines** matching lines + Find-bar **Cut** matching substrings + Find-bar **Shift+Cut** matching lines + Find-bar **Purge** matching lines + Find-bar **Shift+Purge** matching substrings + Find-bar **All** / Search Results Window (current-doc Find Results) + Find-bar **Shift+All** (all opened editor tabs; Case/Word/Re; Sel ignored) + **Next / Previous Search Result** walks Find Results hits (F4 remappable; multi-doc; wrap status; else Find Next/Prev) + **double-click / Enter hit jump** on Find Results (primary + dual-view secondary) + Find-bar **Shift+Enter** = Find Previous + Replace-bar **Enter** = Replace / **Shift+Enter** = Replace All + Find Results / Find in Files **Shift+Enter** = Previous Search Result + Find-bar **Inc** / Search → Incremental Search (as-you-type jump) work; not the full N++ Find/Replace dialog (filters, dedicated mark-all panel, etc.).
4. **Find in Files UI** — recursive workspace + globs + Re; Next/Prev Search Result walks FIF hits (open from workspace + select match); **double-click / Enter hit jump** (primary + dual-view secondary) opens + parks; not full N++ FIF dialog.

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
