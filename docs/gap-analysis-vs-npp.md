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
| Shortcuts | Remappable (`shortcuts.xml`) | Hard-wired set + remaps (`shortcut_word_wrap`, `shortcut_find_next`, `shortcut_find_next_global`, `shortcut_set_and_find`, `shortcut_next_found`, `shortcut_mark_style`, `shortcut_mark_style_2`, `shortcut_mark_style_3`, `shortcut_mark_style_4`, `shortcut_mark_style_5`, `shortcut_clear_all_styles`, `shortcut_inverse_bookmarks`, `shortcut_copy_marked_lines`, `shortcut_next_bookmark`, `shortcut_toggle_bookmark`, `shortcut_compare`, `shortcut_swap_compare`, `shortcut_compare_to_saved`, `shortcut_copy_compare_diff`, `shortcut_copy_compare_summary`, `shortcut_copy_compare_hunk`, `shortcut_word_jump`, `shortcut_rect_select`, `shortcut_next_diff`, `shortcut_first_diff`, `shortcut_next_hidden_equal`, `shortcut_first_hidden_equal`, `shortcut_apply_compare_hunk`, `shortcut_hide_equal`, `shortcut_compare_ignore_ws`, `shortcut_compare_ignore_blank`, `shortcut_expand_all_unchanged`, `shortcut_expand_unchanged_at_caret`, `shortcut_bookmark_compare_diffs`, `shortcut_hide_equal_context`, `shortcut_goto_line`, `shortcut_duplicate_line`, `shortcut_delete_line`, `shortcut_indent`, `shortcut_outdent`, `shortcut_tab_indent`, `shortcut_shift_tab_outdent`, `shortcut_fold_all`, `shortcut_fold_level`, `shortcut_fold_current`, `shortcut_matching_brace`, `shortcut_move_line`, `shortcut_format_document`, `shortcut_close_tab`, `shortcut_close_all`, `shortcut_close_but_pinned`, `shortcut_close_to_side`, `shortcut_save_copy`, `shortcut_trim_trailing`, `shortcut_next_tab`, `shortcut_first_tab`, `shortcut_goto_tab`, `shortcut_move_tab`, `shortcut_new`, `shortcut_open`, `shortcut_reload`, `shortcut_save`, `shortcut_save_as`, `shortcut_save_all`, `shortcut_find`, `shortcut_find_in_files`, `shortcut_close_find`, `shortcut_replace`, `shortcut_replace_alt`, `shortcut_select_all`, `shortcut_copy`, `shortcut_cut`, `shortcut_paste`, `shortcut_delete_forward`, `shortcut_delete_backward`, `shortcut_enter`, `shortcut_undo`, `shortcut_redo`, `shortcut_zoom_in`, `shortcut_zoom_out`, `shortcut_zoom_restore`, `shortcut_toggle_log_tail`, `shortcut_document_map`, `shortcut_function_list`, `shortcut_fullscreen`, `shortcut_postit`, `shortcut_always_on_top`, `shortcut_show_all_characters`, `shortcut_show_whitespace`, `shortcut_show_npc`, `shortcut_sync_scroll`, `shortcut_zoom_sync`, `shortcut_indent_guide`, `shortcut_show_line_numbers`, `shortcut_show_fold_margin`, `shortcut_project_panel`, `shortcut_pin_tab`, `shortcut_lowercase`, `shortcut_autocomplete`, `shortcut_autocomplete_path`, `shortcut_join_lines`, `shortcut_document_home`, `shortcut_multi_select_next`, `shortcut_multi_select_all`, `shortcut_begin_end_select`, `shortcut_toggle_readonly`, `shortcut_column_editor`, `shortcut_sort_lines`, `shortcut_remove_empty_lines`, `shortcut_remove_dup_lines`, `shortcut_insert_datetime`, `shortcut_insert_datetime_custom`, `shortcut_macro_playback`, `shortcut_copy_path`, `shortcut_copy_dir`, `shortcut_copy_all_paths`, `shortcut_open_selected`, `shortcut_open_folder`, `shortcut_open_workspace`, `shortcut_move_other_view`, `shortcut_rtl`, `shortcut_mapper`, `shortcut_tab_colour`, `shortcut_tab_colour_2`, `shortcut_tab_colour_3`) |
| Plugins | DLL ABI + Admin | In-process builtins only |
| Languages | 80+ + UDL | Tree-sitter subset (~7–8) |
| Platforms | Windows-first | macOS / Linux / Windows |

---

## Direct answers

### Are all hotkeys implemented?

**No.**

Hard-wired list lives in `crates/app/src/ui.rs` → `handle_shortcuts` (plus caret keys in `handle_editor_input`). Settings → Shortcut Mapper dumps that list and the effective **word wrap** / **find next** / **find next (global)** / **select and find next** / **next / prev search result** / **mark style 1** / **mark style 2** / **next bookmark** / **toggle bookmark** / **clear all bookmarks** / **inverse bookmarks** / **compare start/clear** / **swap compare sides** / **compare to saved** / **copy / open compare diff** / **copy / open compare summary** / **copy / open compare hunk** / **word jump** / **next difference** / **first difference** / **next hidden equal** / **first hidden equal** / **apply compare hunk** / **hide unchanged lines** / **ignore whitespace / ignore case** / **ignore blank lines** / **expand / collapse all unchanged** / **expand / collapse unchanged at caret** / **bookmark / clear compare difference bookmarks** / **hide-equal context** / **Document Map** / **Fullscreen** / **Post-It** / **Always on Top** / **Show All Characters** / **Show White Space / Show EOL** / **Show NPC** / **Sync Vertical / Horizontal Scroll** / **Zoom Sync** / **Indent Guide** / **Show Line Numbers** / **Show Fold Margin** / **Project Panel** / **Pin tab** / **Focus other view** / **Move / Clone to other view** / **RTL / LTR** / **Find in Files** / **reload** / **go to line** / **duplicate line** / **delete line** / **indent** / **outdent** / **Tab indent** / **Shift+Tab outdent** / **Fold / Unfold all** / **Fold / Unfold level** / **Fold / Unfold current** / **matching brace** / **move line** / **toggle comment** / **lowercase / UPPERCASE** / **Autocomplete / Function Call Tip** / **Path autocomplete** / **Join Lines / Split Lines** / **Invert Case / Proper Case** / **Blank line below / above** / **Multi-select next / skip** / **Multi-select all** / **Multi-select undo** / **Begin/End Select** / **Toggle read-only** / **format document** / **close tab** / **next / prev tab** / **first / last tab** / **new** / **open** / **save** / **save as** / **save all** / **print** / **find** / **Find in Files** / **close find/replace** / **replace** / **replace (alternate)** / **select all** / **copy** / **cut** / **paste** / **delete** / **Backspace** / **Enter** / **undo** / **redo** / **zoom in** / **zoom out** / **zoom restore** / **toggle log tail** chords. Word wrap remaps via Preferences / `shortcut_word_wrap`; Find next via Preferences / `shortcut_find_next` (Shift flips to Find previous); Find next (global) via Preferences / `shortcut_find_next_global` (Shift flips to Find previous); Select and Find next via Preferences / `shortcut_set_and_find` (Shift flips to Select and Find Previous); Next / Previous Search Result via Preferences / `shortcut_next_found` (Shift flips to previous); Next bookmark via Preferences / `shortcut_next_bookmark` (Shift flips to previous); Toggle bookmark via Preferences / `shortcut_toggle_bookmark`; Compare start/clear via Preferences / `shortcut_compare` (Shift flips to Clear); Swap Compare sides via Preferences / `shortcut_swap_compare`; Compare to Saved via Preferences / `shortcut_compare_to_saved`; Copy / Open Compare Diff via Preferences / `shortcut_copy_compare_diff` (Shift flips to open); Copy / Open Compare Summary via Preferences / `shortcut_copy_compare_summary` (Shift flips to open); Copy / Open Compare Hunk via Preferences / `shortcut_copy_compare_hunk` (Shift flips to open); Word jump via Preferences / `shortcut_word_jump` (opposite arrow forward; Shift extends); Rect column select via Preferences / `shortcut_rect_select` (arrow-family = any arrow); Next difference via Preferences / `shortcut_next_diff` (Shift flips to previous); First difference via Preferences / `shortcut_first_diff` (Shift flips to last); Next hidden equal via Preferences / `shortcut_next_hidden_equal` (Shift flips to previous); First hidden equal via Preferences / `shortcut_first_hidden_equal` (Shift flips to last); Apply compare hunk via Preferences / `shortcut_apply_compare_hunk` (opposite Left/Right applies to other; Shift applies all); Hide unchanged lines via Preferences / `shortcut_hide_equal`; Ignore whitespace / Ignore case via Preferences / `shortcut_compare_ignore_ws` (Shift flips to Ignore Case); Ignore blank lines via Preferences / `shortcut_compare_ignore_blank`; Expand / Collapse all unchanged via Preferences / `shortcut_expand_all_unchanged` (Shift flips to collapse); Expand / Collapse unchanged at caret via Preferences / `shortcut_expand_unchanged_at_caret` (Shift flips to collapse); Bookmark / Clear compare difference bookmarks via Preferences / `shortcut_bookmark_compare_diffs` (Shift flips to clear); Hide-equal context via Preferences / `shortcut_hide_equal_context` (opposite bracket decreases); Go to line via Preferences / `shortcut_goto_line`; Duplicate line via Preferences / `shortcut_duplicate_line`; Delete line via Preferences / `shortcut_delete_line`; Indent via Preferences / `shortcut_indent`; Outdent via Preferences / `shortcut_outdent`; Format document via Preferences / `shortcut_format_document`; Close tab via Preferences / `shortcut_close_tab`; Next / prev tab via Preferences / `shortcut_next_tab` (Shift flips to previous); New via Preferences / `shortcut_new`; Open via Preferences / `shortcut_open`; Save via Preferences / `shortcut_save`; Save As via Preferences / `shortcut_save_as`; Save All via Preferences / `shortcut_save_all`; Print via Preferences / `shortcut_print`; Tab indent via Preferences / `shortcut_tab_indent`; Shift+Tab outdent via Preferences / `shortcut_shift_tab_outdent`; Fold / Unfold all via Preferences / `shortcut_fold_all` (Shift flips to unfold); Fold / Unfold level via Preferences / `shortcut_fold_level` (digit-family 1…8; Shift unfolds); Fold / Unfold current via Preferences / `shortcut_fold_current` (Shift flips to unfold); Matching brace via Preferences / `shortcut_matching_brace` (Shift flips to select pair); Move line via Preferences / `shortcut_move_line` (opposite Down moves down); Toggle comment via Preferences / `shortcut_toggle_comment` (Shift flips to Block Comment); lowercase / UPPERCASE via Preferences / `shortcut_lowercase` (Shift flips to UPPERCASE); Join Lines / Split Lines via Preferences / `shortcut_join_lines` (Shift flips to Split Lines); Find via Preferences / `shortcut_find`; Find in Files via Preferences / `shortcut_find_in_files`; Close Find/Replace via Preferences / `shortcut_close_find`; Replace via Preferences / `shortcut_replace`; Replace (alternate) via Preferences / `shortcut_replace_alt`; Select all via Preferences / `shortcut_select_all`; Copy via Preferences / `shortcut_copy`; Cut via Preferences / `shortcut_cut`; Paste via Preferences / `shortcut_paste`; Delete via Preferences / `shortcut_delete_forward`; Backspace via Preferences / `shortcut_delete_backward`; Document Home / End aliases via Preferences / `shortcut_document_home` (opposite End = document end); Undo via Preferences / `shortcut_undo` (Shift flips to Redo); Redo via Preferences / `shortcut_redo`; Zoom in via Preferences / `shortcut_zoom_in`; Zoom out via Preferences / `shortcut_zoom_out`; Zoom restore via Preferences / `shortcut_zoom_restore` (mouse wheel stays hard-wired); Toggle log tail via Preferences / `shortcut_toggle_log_tail`; Show All Characters via Preferences / `shortcut_show_all_characters`; Show White Space / Show EOL via Preferences / `shortcut_show_whitespace` (Shift flips to Show EOL); Show NPC via Preferences / `shortcut_show_npc`; Sync Vertical / Horizontal Scroll via Preferences / `shortcut_sync_scroll` (Shift flips to Sync H); Zoom Sync via Preferences / `shortcut_zoom_sync`; Indent Guide via Preferences / `shortcut_indent_guide`; Show Line Numbers via Preferences / `shortcut_show_line_numbers`; Show Fold Margin via Preferences / `shortcut_show_fold_margin`; Project Panel via Preferences / `shortcut_project_panel`; Pin tab via Preferences / `shortcut_pin_tab`; Focus other view via Preferences / `shortcut_focus_other_view` (Shift swaps tabs). Move / Clone to other view via Preferences / `shortcut_move_other_view` (Shift clones). RTL / LTR via Preferences / `shortcut_rtl` (Shift flips to LTR). Tab colour 1 / clear via Preferences / `shortcut_tab_colour` (Shift clears). Tab colour 2 / clear / Tab colour 3 / clear / Tab colour 4 / clear / Tab colour 5 / clear via Preferences / `shortcut_tab_colour_2`, `shortcut_tab_colour_3`, `shortcut_tab_colour_4`, `shortcut_tab_colour_5` (Shift clears). There is **no** full `shortcuts.xml` remap yet.

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
| Cmd/Ctrl+C (remappable) | Copy (`shortcut_copy`) |
| Cmd/Ctrl+X (remappable) | Cut (`shortcut_cut`) |
| Cmd/Ctrl+V (remappable) | Paste (`shortcut_paste`; default uses system clipboard; remapped uses last copied) |
| Delete (remappable) | Forward delete (`shortcut_delete_forward`) |
| Backspace (remappable) | Backward delete (`shortcut_delete_backward`) |
| Enter (remappable) | Insert newline (`shortcut_enter`; blank line Cmd+Enter separate) |
| Cmd/Ctrl+D | Duplicate line |
| Cmd/Ctrl+Shift+L | Delete line |
| Cmd/Ctrl+] / [ | Indent / outdent |
| Cmd/Ctrl+Shift+I | Format Document |
| Cmd/Ctrl+F | Find |
| Cmd/Ctrl+H / Shift+F | Replace |
| F3 / Shift+F3 | Find next/prev (**global**) |
| Cmd/Ctrl+F3 / Shift+F3 | Select and Find next/prev (remappable `shortcut_set_and_find`) |
| F4 / Shift+F4 | Next / Previous Search Result (remappable `shortcut_next_found`) |
| Cmd/Ctrl+1 / Shift+1 | Mark Style 1 / Clear Style 1 (remappable `shortcut_mark_style`) |
| Cmd/Ctrl+2 / Shift+2 | Mark Style 2 / Clear Style 2 (remappable `shortcut_mark_style_2`) |
| Cmd/Ctrl+3 / Shift+3 | Mark Style 3 / Clear Style 3 (remappable `shortcut_mark_style_3`) |
| Cmd/Ctrl+4 / Shift+4 | Mark Style 4 / Clear Style 4 (remappable `shortcut_mark_style_4`) |
| Cmd/Ctrl+5 / Shift+5 | Mark Style 5 / Clear Style 5 (remappable `shortcut_mark_style_5`) |
| Cmd/Ctrl+Shift+0 | Clear all styles (remappable `shortcut_clear_all_styles`) |
| Cmd/Ctrl+Alt+1…5 / Shift | Next / Previous Style marker 1…5 (remappable `shortcut_next_style`) |
| Cmd/Ctrl+G / Shift+G | Find next/prev (**global**) |
| Cmd/Ctrl+L | Go to line |
| Cmd/Ctrl+B (remappable) | Matching brace (`shortcut_matching_brace`; Shift selects pair) |
| Cmd/Ctrl+Shift+Up / Down (remappable) | Move line up / down (`shortcut_move_line`) |
| F2 / Shift+F2 | Next / previous bookmark |
| Cmd/Ctrl+F2 (remappable) | Toggle bookmark (`shortcut_toggle_bookmark`) |
| Cmd/Ctrl+Shift+F2 (remappable) | Clear all bookmarks (`shortcut_clear_bookmarks`) |
| Alt+Shift+F2 | Inverse bookmarks (remappable `shortcut_inverse_bookmarks`) |
| Cmd/Ctrl+Alt+F2 (remappable) | Copy Bookmarked Lines (`shortcut_copy_marked_lines`; Shift = Cut) |
| Escape (remappable) | Close Find/Replace |
| Cmd/Ctrl+= / − / 0 | Zoom in / out / restore |
| Cmd/Ctrl+mouse wheel | Zoom |
| Alt+Z | Word wrap |
| Cmd/Ctrl+Shift+T | Toggle log tail |
| Alt+← / → (remappable) | Word jump (`shortcut_word_jump`) |
| Cmd/Ctrl+U (remappable) | lowercase (`shortcut_lowercase`; Shift = UPPERCASE) |
| Cmd/Ctrl+Space (remappable) | Autocomplete (`shortcut_autocomplete`; Shift = Function Call Tip) |
| Alt+Down (remappable) | Function Parameters Next (`shortcut_calltip_nav`; opposite Up = previous) |
| Cmd/Ctrl+Alt+Space (remappable) | Path autocomplete (`shortcut_autocomplete_path`) |
| Cmd/Ctrl+J (remappable) | Join Lines (`shortcut_join_lines`; Shift = Split Lines) |
| Cmd/Ctrl+Alt+U (remappable) | Invert Case (`shortcut_invert_case`; Shift = Proper Case) |
| Cmd/Ctrl+Alt+8 (remappable) | Sentence Case Force (`shortcut_sentence_case`; Shift = Blend) |
| Cmd/Ctrl+Alt+9 (remappable) | Proper Case Force (`shortcut_proper_case`; Shift = Blend; Invert+Shift still aliases Force) |
| Cmd/Ctrl+Alt+0 (remappable) | Random Case (`shortcut_random_case`) |
| Alt+J (remappable) | Redact Selection █; Shift = ● (`shortcut_redact`) |
| Alt+U (remappable) | Search on Internet; Shift = Change Search Engine (`shortcut_search_internet`) |
| Alt+Shift+T (remappable) | Trim Leading Space (`shortcut_trim_leading`) |
| Cmd/Ctrl+Enter (remappable) | Blank line below (`shortcut_blank_line`; Shift = blank line above) |
| Cmd/Ctrl+Alt+N (remappable) | Multi-select next (`shortcut_multi_select_next`; Shift = skip current) |
| Cmd/Ctrl+Alt+A (remappable) | Multi-select all (`shortcut_multi_select_all`; Shift = whole-word) |
| Cmd/Ctrl+Alt+Backspace (remappable) | Undo the latest added multi-select (`shortcut_multi_select_undo`) |
| F9 (remappable) | Move to New Instance (`shortcut_new_instance`; Shift = Open in New Instance, keep this tab) |
| Cmd/Ctrl+Shift+1…9 (remappable) | 1st–9th Tab (`shortcut_goto_tab`; letter remap = 1st tab only) |
| Cmd/Ctrl+Alt+B (remappable) | Begin/End Select (`shortcut_begin_end_select`; Shift = column mode rect) |
| Cmd/Ctrl+Alt+R (remappable) | Toggle Read-Only (`shortcut_toggle_readonly`) |
| Cmd/Ctrl+Alt+C (remappable) | Column Editor (`shortcut_column_editor`) |
| Cmd/Ctrl+Alt+O (remappable) | Sort Lines Lexicographic Asc (`shortcut_sort_lines`; Shift = Desc) |
| Cmd/Ctrl+Alt+F5 (remappable) | Sort Lines Integer Asc (`shortcut_sort_lines_integer`; Shift = Desc) |
| Cmd/Ctrl+Alt+F6 (remappable) | Sort Lines Lex Ignoring Case Asc (`shortcut_sort_lines_case_insensitive`; Shift = Desc) |
| Cmd/Ctrl+Alt+F8 (remappable) | Sort Lines Decimal Dot Asc (`shortcut_sort_lines_decimal_dot`; Shift = Desc) |
| Cmd/Ctrl+Alt+F9 (remappable) | Sort Lines Decimal Comma Asc (`shortcut_sort_lines_decimal_comma`; Shift = Desc) |
| Cmd/Ctrl+Alt+F10 (remappable) | Sort Lines by Length Asc (`shortcut_sort_lines_length`; Shift = Desc) |
| Cmd/Ctrl+Alt+6 (remappable) | Sort Lines Reverse Order (`shortcut_sort_lines_reverse`; Shift = Randomly) |
| Cmd/Ctrl+Alt+7 (remappable) | Sort Lines Locale Asc (`shortcut_sort_lines_locale`; Shift = Desc) |
| Cmd/Ctrl+Alt+F11 (remappable) | EOL to Space (`shortcut_eol_to_space`, `shortcut_convert_eol`; Shift = Trim Both and EOL to Space) |
| Cmd/Ctrl+Alt+F12 (remappable) | Convert EOL Unix (`shortcut_convert_eol`; Shift = Windows CRLF) |
| Alt+Shift+F12 (remappable) | Convert EOL Macintosh (`shortcut_convert_eol_mac`) |
| Cmd/Ctrl+F9 (remappable) | UTF-8 (`shortcut_encoding_utf8`; Shift = UTF-8-BOM) |
| Cmd/Ctrl+F10 (remappable) | UTF-16 LE (`shortcut_encoding_utf16`; Shift = UTF-16 BE) |
| Cmd/Ctrl+F11 (remappable) | ANSI (`shortcut_encoding_ansi`) |
| Alt+F10 (remappable) | MD5 generate; Shift = clipboard (`shortcut_md5`) |
| Alt+F11 (remappable) | SHA-256 generate; Shift = clipboard (`shortcut_sha256`) |
| Cmd/Ctrl+F12 (remappable) | SHA-1 generate; Shift = clipboard (`shortcut_sha1`) |
| Cmd/Ctrl+F6 (remappable) | SHA-512 generate; Shift = clipboard (`shortcut_sha512`) |
| Alt+Shift+K (remappable) | Shortcut Mapper (`shortcut_mapper`) |
| Cmd/Ctrl+F5 (remappable) | Tab colour 1; Shift clears (`shortcut_tab_colour`) |
| Cmd/Ctrl+F4 (remappable) | Tab colour 2; Shift clears (`shortcut_tab_colour_2`) |
| Cmd/Ctrl+K (remappable) | Tab colour 3; Shift clears (`shortcut_tab_colour_3`) |
| Cmd/Ctrl+M (remappable) | Tab colour 4; Shift clears (`shortcut_tab_colour_4`) |
| Cmd/Ctrl+; (remappable) | Tab colour 5; Shift clears (`shortcut_tab_colour_5`) |
| Cmd/Ctrl+Shift+C (remappable) | Clipboard History (`shortcut_clipboard_history`) |
| Cmd/Ctrl+Shift+R (remappable) | Run… (`shortcut_run`) |
| Cmd/Ctrl+Shift+O (remappable) | Open in Default Viewer (`shortcut_open_viewer`) |
| F1 (remappable) | About (`shortcut_about`) |
| Alt+Shift+L (remappable) | Hide Lines (`shortcut_hide_lines`; current line or selection) |
| Cmd/Ctrl+Alt+E (remappable) | Remove Empty Lines (`shortcut_remove_empty_lines`; Shift = Containing Blank) |
| Cmd/Ctrl+Alt+D (remappable) | Remove Consecutive Duplicate Lines (`shortcut_remove_dup_lines`; Shift = any) |
| F5 (remappable) | Date Time short (`shortcut_insert_datetime`; Shift = long) |
| Cmd/Ctrl+Alt+= (remappable) | Date Time customized (`shortcut_insert_datetime_custom`) |
| Cmd/Ctrl+Shift+P (remappable) | Macro Playback (`shortcut_macro_playback`) |
| Cmd/Ctrl+Alt+P (remappable) | Copy Full Path (`shortcut_copy_path`; Shift = File Name) |
| Cmd/Ctrl+Alt+K (remappable) | Copy Current Directory Path (`shortcut_copy_dir`) |
| Cmd/Ctrl+Alt+M (remappable) | Copy All Paths (`shortcut_copy_all_paths`; Shift = All Filenames) |
| Cmd/Ctrl+Alt+J (remappable) | Open Selected Filename (`shortcut_open_selected`; Shift = Containing Folder) |
| Cmd/Ctrl+Alt+L (remappable) | Open Containing Folder (`shortcut_open_folder`, `shortcut_open_workspace`; Shift = shell here) |
| Cmd/Ctrl+Alt+G (remappable) | Open Folder as Workspace (`shortcut_open_workspace`; Shift = Containing Folder as Workspace) |
| Cmd/Ctrl+Alt+H (remappable) | Close All but Pinned (`shortcut_close_but_pinned`; Shift = Close All Unchanged) |
| Cmd/Ctrl+Alt+I (remappable) | Close All to the Left (`shortcut_close_to_side`; Shift = Close All to the Right) |
| Cmd/Ctrl+Alt+Y (remappable) | Save a Copy As (`shortcut_save_copy`; Shift = Rename) |
| Cmd/Ctrl+E (remappable) | Function List (`shortcut_function_list`; Shift = Character Panel) |
| Cmd/Ctrl+Shift+PageDown (remappable) | Move tab forward (`shortcut_move_tab`; opposite PageUp = backward) |
| Cmd/Ctrl+Alt+Home (remappable) | First tab (`shortcut_first_tab`; opposite End = last) |
| Cmd/Ctrl+Alt+V (remappable) | TAB to Space (`shortcut_tab_to_space`; Shift = Space to TAB Leading) |
| Cmd/Ctrl+Alt+X (remappable) | Space to TAB (All) (`shortcut_space_to_tab_all`) |
| Alt+Shift+arrows (remappable `shortcut_rect_select`) | Start / extend rectangular column select (virtual cols ok) |

Missing vs typical Notepad++ (examples, not exhaustive):

- Macro record/play keys
- Remappable Scintilla keys (`shortcuts.xml`)
- Fold level keys: remappable via `shortcut_fold_level` (default `Alt+1`…`8`; Shift unfolds). Fold All / Fold Current via `shortcut_fold_all` / `shortcut_fold_current`

Hundreds of menu commands have **no** accelerator.

### Drag and drop of marked code?

**Partial (v0.3.8; cross-pane v0.3.204).** Drag inside a selection moves text; Ctrl/Cmd+drag copies. Dual-view drag into the other pane moves/copies across tabs. Drop files on the window to open. Drop selection onto Find / Replace fills those fields (v0.3.205/206).

| Kind | Status |
|------|--------|
| Drag to select / double-click word / triple-click line | Done |
| Tab drag-reorder | Done |
| Document map click/drag scroll | Done | Compare colours + hunk-start ticks; Find match ticks; fold-header ticks + click toggle; selection wash + caret / multi-caret ticks; click parks hunk/match/caret; drag syncs partner when sync scroll on |
| Drag selection to move or copy text | Done (same buffer + dual-view cross-pane; Ctrl/Cmd = copy) |
| Drop selection onto Find field | Done (fills query; first line, max 200 chars) |
| Drop selection onto Replace field | Done (fills replace text; first line, max 200 chars) |
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
| Change history | Partial | Bars + undo remap (v0.3.5) + next/prev remappable F8 (v0.3.180); not full Scintilla |
| Dual / other view | Partial | Writable panes; no docking layout |
| 2-way compare | Partial | Line LCS + word-aware wash + hide unchanged (···N gaps, click expand, Expand at Caret / Next·Prev·First·Last Hidden Equal / Expand/Collapse All) + bookmark / clear hunk-start bookmarks + apply hunk from/to other / apply all + open unified diff tab; Clear restores pre-start sync H/V; no 3-way |
| Themes / styles | Partial | JSON + N++ XML subset (v0.3.4) |
| Encoding | Partial | UTF-8 / BOM / ANSI / UTF-16 LE·BE (BOM or no-BOM detect, v0.3.17) |
| Session restore | Partial | Path list + folded fold-header lines (`@folds`) + bookmarks (`@bookmarks`) + style marks (`@style1`…`@style5`) + caret (`@caret`) + selection (`@sel`) + per-tab scroll (`@scroll`) + language (`@lang`) + encoding (`@encoding`) + read-only (`@readonly`) + pin (`@pin`) + tab colour (`@colour`) + log-tail (`@tail`) + dual-view other pane (`@other`) + other-pane focus (`@focus`) + sync H/V scroll (`@syncv` / `@synch`) + Compare on (`@compare`) + marked Compare partner (`@partner`) + active tab (`@active`); not full N++ session XML |
| Project panel | Partial | Folder list + refresh/reveal; open state persists (`show_project_panel`); not N++ projects |
| Indent fold / hide lines | Partial | Fold margin + brace/indent heuristics; session restores folded headers; not full Scintilla |
| Column / multi-edit | Partial | Alt+rect + Alt+Shift+arrows + multi-caret + virtual space (`docs/column-mode.md`) |
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
| Run / external tools | Partial | Remappable Run… (`shortcut_run`, Cmd+Shift+R); pick+spawn / shell; no saved Run list |
| Updater | Partial | Opens GitHub Releases |
| Doc map / function list | Partial | Density strip; Compare paints delete/insert + hunk-start ticks + hide-equal ···N gap bands on focused pane; cyan bookmark ticks + slate fold-header ticks + click toggle + amber/green change-history ticks + Find match ticks + click park; selection wash + white caret tick + orange multi-caret ticks + click park; click parks hunk / Equal / hidden-equal gap; drag syncs partner when sync scroll on |
| RTL | Partial | Line anchors + status + `text_rtl` persist + remappable Alt+R / Shift=LTR (`shortcut_rtl`); chrome not mirrored |

---

## What else is missing? (ranked for product sense)

### P0 — daily editor feel

1. **More hotkeys** + broader remap (`shortcuts.xml` or settings) — hard-wired set grew in v0.3.7; word wrap remaps in v0.3.19; find next remaps in v0.3.104; next bookmark remaps in v0.3.105; toggle bookmark remaps in v0.3.112; clear all bookmarks remaps in v0.3.185; inverse bookmarks remaps in v0.3.289; next difference remaps in v0.3.106; go to line remaps in v0.3.107; duplicate line remaps in v0.3.108; indent remaps in v0.3.110; outdent remaps in v0.3.111; find remaps in v0.3.116; replace remaps in v0.3.117; save as remaps in v0.3.118; new remaps in v0.3.119; open remaps in v0.3.120; reload remaps in v0.3.134; select all remaps in v0.3.121; copy remaps in v0.3.186; cut remaps in v0.3.187; paste remaps in v0.3.188; delete remaps in v0.3.189; Backspace remaps in v0.3.190; Enter remaps in v0.3.200; line start/end remaps in v0.3.191; page up/down remaps in v0.3.192; character left/right remaps in v0.3.193; line up/down remaps in v0.3.194; Document Home aliases remaps in v0.3.196; undo remaps in v0.3.122; redo remaps in v0.3.123; zoom in remaps in v0.3.125; zoom out remaps in v0.3.126; zoom restore remaps in v0.3.127; toggle log tail remaps in v0.3.128; find next (global) remaps in v0.3.129; replace (alternate) remaps in v0.3.130; next hidden equal remaps in v0.3.131; apply compare hunk remaps in v0.3.132; hide-equal context remaps in v0.3.133; reload remaps in v0.3.134; first difference remaps in v0.3.135; first hidden equal remaps in v0.3.136; hide equal remaps in v0.3.137; compare start/clear remaps in v0.3.138; close find/replace remaps in v0.3.140; swap compare remaps in v0.3.141; compare to saved remaps in v0.3.143; next/prev tab remaps in v0.3.144; save all remaps in v0.3.145; print remaps in v0.3.146; Shift+Tab outdent remaps in v0.3.147; fold all remaps in v0.3.148; fold current remaps in v0.3.149; matching brace remaps in v0.3.150; move line remaps in v0.3.151; toggle comment remaps in v0.3.152; lowercase / UPPERCASE remaps in v0.3.181; Join Lines / Split Lines remaps in v0.3.182; Invert Case / Proper Case remaps in v0.3.183; blank line below / above remaps in v0.3.184; multi-select next remaps in v0.3.201; multi-select all remaps in v0.3.259; fullscreen remaps in v0.3.202; always-on-top remaps in v0.3.203; pin tab remaps in v0.3.210; show whitespace / Show EOL remaps in v0.3.211; copy/open compare diff remaps in v0.3.153; copy/open compare summary remaps in v0.3.154; copy/open compare hunk remaps in v0.3.155; Tab indent remaps in v0.3.156; ignore whitespace / ignore case remaps in v0.3.157; ignore blank lines remaps in v0.3.158; expand/collapse all unchanged remaps in v0.3.159; expand/collapse unchanged at caret remaps in v0.3.160; bookmark/clear difference bookmarks remaps in v0.3.162; Document Map remaps in v0.3.175; Find in Files remaps in v0.3.176; go to document start/end remaps in v0.3.178; next change history remaps in v0.3.180; autocomplete remaps in v0.3.293; path autocomplete remaps in v0.3.294; sort tabs by name remaps in v0.3.358; full mapper still open
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
| Shortcut Mapper | Static text of hard-wired keys + effective word-wrap / find-next / find-next-global / set-and-find / mark-style / mark-style-2 / mark-style-3 / mark-style-4 / mark-style-5 / clear-all-styles / next-bookmark / toggle-bookmark / clear-bookmarks / next-changed / compare / swap-compare / compare-to-saved / copy-compare-diff / copy-compare-summary / next-diff / first-diff / next-hidden-equal / first-hidden-equal / apply-compare-hunk / hide-equal / compare-ignore-ws / compare-ignore-blank / expand-all-unchanged / expand-unchanged-at-caret / hide-equal-context / reload / go-to-line / duplicate-line / delete-line / indent / outdent / tab-indent / shift-tab-outdent / fold-all / fold-current / matching-brace / move-line / goto-document-start / document-home / toggle-comment / lowercase / join-lines / invert-case / blank-line / multi-select-next / multi-select-all / format / close-tab / pin-tab / next-tab / new / open / save / save-as / save-all / print / find / replace / replace-alt / select-all / copy / cut / paste / delete / delete-backward / line-home / document-home / page-up / char-left / line-up / undo / redo / zoom / document-map / fullscreen / always-on-top / show-all-characters / show-whitespace / show-npc / sync-scroll / indent-guide / show-line-numbers / show-fold-margin / project-panel / pin-tab / rtl / find-in-files remaps |
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
- **Drag marked code:** move/copy in same buffer (v0.3.8) + dual-view cross-pane (v0.3.204) + drop into Find field (v0.3.205) + drop into Replace field (v0.3.206).
- **Largest holes:** remappable keys, plugins ABI, UDL, hex, deeper fold/LSP.
