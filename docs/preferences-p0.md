# Preferences and P0 polish (v0.3.1)

Date: 2026-08-29  
Issue: https://github.com/raro42/npp-rust/issues/7

## Settings (`npp-rs/settings.json`)

New keys:

| Key | Meaning |
|-----|---------|
| `gutter_extra` | Extra gutter pixels (0–40) |
| `caret_blink` | Blink caret |
| `default_eol` | `lf` / `crlf` for Enter |
| `recent_max` | Recent file cap (5–40); load and save both use this cap |
| `restore_session` | Reopen session on launch (paths, `@folds`, `@bookmarks`, `@style1`…`@style5`, `@caret`, `@sel`, per-tab `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, `@tail`, `@other`, `@focus`, `@syncv` / `@synch`, `@compare`, `@partner`, `@active`). Zoom Sync itself is a settings key (`zoom_sync`), not session meta. |
| `find_match_case` / `find_whole_word` / `find_in_selection` / `find_wrap` / `find_regex` | Find options |
| `find_query` / `replace_with` | Last find/replace strings |
| `compare_ignore_ws` | Compare ignores whitespace runs (View menu + Preferences; live re-diff + first-hunk park while Compare is on) |
| `compare_ignore_case` | Compare ignores letter case (View menu + Preferences; live re-diff + first-hunk park while Compare is on) |
| `compare_ignore_blank` | Compare skips blank lines in LCS (View menu + Preferences; live re-diff + first-hunk park while Compare is on) |
| `compare_hide_equal` | Compare hides Equal (unchanged) lines in both panes (View menu + Preferences; live apply + first-hunk park while Compare is on) |
| `compare_hide_equal_context` | Equal lines kept visible on each side of a change when hide-equal is on (`0`–`10`, default `3`) |
| `backup_on_save` | Copy on-disk file into `npp-rs/backup/` before overwrite |
| `autosave_interval_secs` | Autosave dirty named tabs (`0` = off; else 15–900) |
| `show_fold_margin` | Gutter fold markers (`−` / `+`); default on |
| `show_whitespace` | Show space/tab glyphs (View + Preferences); default off |
| `show_eol` | Show end-of-line marks; default off |
| `show_npc` | Show non-printing / control characters; default off |
| `show_indent_guide` | Vertical indent guides; default off |
| `text_rtl` | Right-to-left editor line anchors + status cue (Edit → RTL / LTR + Preferences); default off |
| `always_on_top` | Keep the window above others (View → Always on Top + Preferences); default off |
| `post_it` | Post-It / distraction-free: hide menu + status bar (View → Post-It / Distraction Free + Preferences); default off |
| `show_document_map` | Document Map panel open (View → Document Map + Preferences); default off |
| `show_project_panel` | Project panel open (View → Project Panel + Preferences); default off |
| `zoom_sync` | Dual-view Zoom Sync (both panes share font size; View menu + Preferences); default off |
| `shortcut_word_wrap` | Word-wrap toggle chord (`Alt+Z` default; e.g. `Ctrl+W`) |
| `shortcut_find_next` | Find-next chord (`F3` default; Shift flips to find previous) |
| `shortcut_find_next_global` | Global find-next chord (`Cmd+G` default; Shift flips to find previous) |
| `shortcut_set_and_find` | Select-and-find-next chord (`Cmd+F3` default; Shift flips to Select and Find Previous) |
| `shortcut_mark_style` | Mark-all Style 1 chord (`Cmd+1` default; Shift flips to Clear Style 1) |
| `shortcut_mark_style_2` | Mark-all Style 2 chord (`Cmd+2` default; Shift flips to Clear Style 2) |
| `shortcut_mark_style_3` | Mark-all Style 3 chord (`Cmd+3` default; Shift flips to Clear Style 3) |
| `shortcut_mark_style_4` | Mark-all Style 4 chord (`Cmd+4` default; Shift flips to Clear Style 4) |
| `shortcut_mark_style_5` | Mark-all Style 5 chord (`Cmd+5` default; Shift flips to Clear Style 5) |
| `shortcut_clear_all_styles` | Clear all styles chord (`Cmd+Shift+0` default) |
| `shortcut_next_bookmark` | Next-bookmark chord (`F2` default; Shift flips to previous) |
| `shortcut_toggle_bookmark` | Toggle-bookmark chord (`Cmd+F2` default) |
| `shortcut_clear_bookmarks` | Clear-all-bookmarks chord (`Cmd+Shift+F2` default) |
| `shortcut_inverse_bookmarks` | Inverse-bookmarks chord (`Alt+Shift+F2` default) |
| `shortcut_next_diff` | Next-compare-diff chord (`F7` default; Shift flips to previous) |
| `shortcut_first_diff` | First-compare-diff chord (`Cmd+F7` default; Shift flips to last) |
| `shortcut_next_hidden_equal` | Next-hidden-equal chord (`Alt+F7` default; Shift flips to previous) |
| `shortcut_first_hidden_equal` | First-hidden-equal chord (`Cmd+Alt+F7` default; Shift flips to last) |
| `shortcut_apply_compare_hunk` | Apply-compare-hunk-from chord (`Cmd+Alt+Left` default; opposite Left/Right applies to other; Shift applies all) |
| `shortcut_hide_equal_context` | Hide-equal-context increase chord (`Alt+]` default; opposite `[` / `]` decreases) |
| `shortcut_hide_equal` | Hide-unchanged-lines toggle chord (`Alt+H` default) |
| `shortcut_compare_ignore_ws` | Ignore-whitespace chord (`Alt+W` default; Shift flips to Ignore Case) |
| `shortcut_compare_ignore_blank` | Ignore-blank-lines chord (`Alt+B` default) |
| `shortcut_expand_all_unchanged` | Expand-all-unchanged chord (`Alt+E` default; Shift flips to Collapse All Unchanged) |
| `shortcut_expand_unchanged_at_caret` | Expand-unchanged-at-caret chord (`Alt+X` default; Shift flips to Collapse Unchanged at Caret) |
| `shortcut_bookmark_compare_diffs` | Bookmark-compare-differences chord (`Alt+M` default; Shift flips to Clear Compare Difference Bookmarks) |
| `shortcut_compare` | Start-compare chord (`Alt+D` default; Shift flips to Clear Compare) |
| `shortcut_swap_compare` | Swap-compare-sides chord (`Alt+S` default) |
| `shortcut_compare_to_saved` | Compare-to-saved chord (`Alt+Shift+S` default) |
| `shortcut_copy_compare_diff` | Copy-compare-diff chord (`Alt+C` default; Shift flips to Open Compare Diff) |
| `shortcut_copy_compare_summary` | Copy-compare-summary chord (`Alt+Y` default; Shift flips to Open Compare Summary) |
| `shortcut_copy_compare_hunk` | Copy-compare-hunk chord (`Alt+K` default; Shift flips to Open Compare Hunk) |
| `shortcut_word_jump` | Word-jump-back chord (`Alt+Left` default; opposite arrow forward; Shift extends) |
| `shortcut_rect_select` | Rect column-select chord (`Alt+Shift+Left` default; arrow-family = any arrow with same mods) |
| `shortcut_goto_line` | Go-to-line chord (`Cmd+L` default; Cmd/Ctrl+Shift+L delete line stays hard-wired) |
| `shortcut_duplicate_line` | Duplicate-line chord (`Cmd+D` default) |
| `shortcut_delete_line` | Delete-line chord (`Cmd+Shift+L` default) |
| `shortcut_indent` | Indent-lines chord (`Cmd+]` default) |
| `shortcut_outdent` | Outdent-lines chord (`Cmd+[` default) |
| `shortcut_format_document` | Format-document chord (`Cmd+Shift+I` default) |
| `shortcut_close_tab` | Close-tab chord (`Cmd+W` default) |
| `shortcut_close_all` | Close All chord (`Cmd+Alt+W` default; Shift flips to Close All but Current) |
| `shortcut_close_but_pinned` | Close All but Pinned chord (`Cmd+Alt+H` default; Shift flips to Close All Unchanged) |
| `shortcut_close_to_side` | Close All to the Left chord (`Cmd+Alt+I` default; Shift flips to Close All to the Right) |
| `shortcut_trim_trailing` | Trim Trailing chord (`Cmd+Alt+T` default; Shift flips to Trim Leading and Trailing) |
| `shortcut_pin_tab` | Pin / unpin active tab chord (`Alt+Shift+P` default; `IDM_PINTAB`) |
| `shortcut_next_tab` | Next-tab chord (`Cmd+Tab` default; Shift flips to previous) |
| `shortcut_move_tab` | Move-tab-forward chord (`Cmd+Shift+PageDown` default; opposite PageUp moves back; letter remaps use Shift) |
| `shortcut_new` | New-file chord (`Cmd+N` default) |
| `shortcut_open` | Open-file chord (`Cmd+O` default) |
| `shortcut_save` | Save chord (`Cmd+S` default) |
| `shortcut_save_as` | Save-as chord (`Cmd+Shift+S` default) |
| `shortcut_save_copy` | Save a Copy As chord (`Cmd+Alt+Y` default; Shift flips to Rename) |
| `shortcut_save_all` | Save-all chord (`Cmd+Alt+S` default) |
| `shortcut_print` | Print chord (`Cmd+P` default) |
| `shortcut_tab_indent` | Tab indent chord (`Tab` default; inserts spaces at caret / multi-carets) |
| `shortcut_shift_tab_outdent` | Shift+Tab outdent chord (`Shift+Tab` default; alternate to `shortcut_outdent`) |
| `shortcut_fold_all` | Fold-all chord (`Alt+0` default; Shift flips to Unfold All) |
| `shortcut_fold_level` | Fold-level chord (`Alt+1` default; digit-family `1`…`8`; Shift flips to Unfold Level) |
| `shortcut_fold_current` | Fold-current chord (`Alt+F` default; Shift flips to Unfold Current) |
| `shortcut_matching_brace` | Matching-brace chord (`Cmd+B` default; Shift flips to Select matching braces) |
| `shortcut_move_line` | Move-line-up chord (`Cmd+Shift+Up` default; opposite Down moves down; letter remaps use Shift for down) |
| `shortcut_toggle_comment` | Toggle line-comment chord (`Cmd+/` default; Shift flips to Block Comment) |
| `shortcut_lowercase` | Lowercase chord (`Cmd+U` default; Shift flips to UPPERCASE) |
| `shortcut_join_lines` | Join Lines chord (`Cmd+J` default; Shift flips to Split Lines) |
| `shortcut_invert_case` | Invert Case chord (`Cmd+Alt+U` default; Shift flips to Proper Case) |
| `shortcut_blank_line` | Blank line below chord (`Cmd+Enter` default; Shift flips to blank line above) |
| `shortcut_multi_select_all` | Multi-select all chord (`Cmd+Alt+A` default; Shift flips to whole-word) |
| `shortcut_begin_end_select` | Begin/End Select chord (`Cmd+Alt+B` default; Shift flips to column mode rect) |
| `shortcut_toggle_readonly` | Toggle read-only chord (`Cmd+Alt+R` default; `IDM_EDIT_TOGGLEREADONLY`) |
| `shortcut_column_editor` | Column Editor chord (`Cmd+Alt+C` default; `IDM_EDIT_COLUMNMODE`) |
| `shortcut_sort_lines` | Sort Lines Lexicographic Ascending chord (`Cmd+Alt+O` default; Shift flips to Descending) |
| `shortcut_remove_empty_lines` | Remove Empty Lines chord (`Cmd+Alt+E` default; Shift flips to Containing Blank Characters) |
| `shortcut_remove_dup_lines` | Remove Consecutive Duplicate Lines chord (`Cmd+Alt+D` default; Shift flips to any duplicates) |
| `shortcut_insert_datetime` | Date Time (short) chord (`F5` default; Shift flips to long) |
| `shortcut_copy_path` | Copy Full Path chord (`Cmd+Alt+P` default; Shift flips to File Name) |
| `shortcut_copy_dir` | Copy Current Directory Path chord (`Cmd+Alt+K` default) |
| `shortcut_copy_all_paths` | Copy All File Paths chord (`Cmd+Alt+M` default; Shift flips to All Filenames) |
| `shortcut_open_selected` | Open Selected Filename chord (`Cmd+Alt+J` default; Shift flips to Open Containing Folder) |
| `shortcut_open_folder` | Open Containing Folder chord (`Cmd+Alt+L` default; Shift flips to shell here) |
| `shortcut_open_workspace` | Open Folder as Workspace chord (`Cmd+Alt+G` default; Shift flips to Containing Folder as Workspace) |
| `shortcut_multi_select_next` | Multi-select next chord (`Cmd+Alt+N` default; Shift flips to skip current / go to next) |
| `shortcut_find` | Find-bar chord (`Cmd+F` default) |
| `shortcut_find_in_files` | Find in Files chord (`Cmd+Alt+F` default) |
| `shortcut_close_find` | Close Find/Replace chord (`Escape` default) |
| `shortcut_replace` | Replace-bar chord (`Cmd+H` default) |
| `shortcut_replace_alt` | Alternate replace-bar chord (`Cmd+Shift+F` default) |
| `shortcut_select_all` | Select-all chord (`Cmd+A` default) |
| `shortcut_copy` | Copy chord (`Cmd+C` default; when remapped, platform Cmd+C no longer copies) |
| `shortcut_cut` | Cut chord (`Cmd+X` default; when remapped, platform Cmd+X no longer cuts) |
| `shortcut_paste` | Paste chord (`Cmd+V` default; when remapped, platform Cmd+V no longer pastes; remapped chord pastes last copied text) |
| `shortcut_delete_forward` | Forward-delete chord (`Delete` default; when remapped, bare Delete no longer deletes forward) |
| `shortcut_delete_backward` | Backward-delete chord (`Backspace` default; when remapped, bare Backspace no longer deletes backward) |
| `shortcut_enter` | Insert-newline chord (`Enter` default; when remapped, bare Enter no longer inserts a newline; `shortcut_blank_line` / Cmd+Enter stays separate) |
| `shortcut_line_home` | Line-start chord (`Home` default; opposite End = line end; letter remaps use Shift for end; when remapped, bare Home/End no longer move on the line; Cmd+Home/End aliases use `shortcut_document_home`) |
| `shortcut_document_home` | Document-start Home alias (`Cmd+Home` default; opposite End = document end; letter remaps use Shift for end; when remapped, bare Cmd+Home/End no longer jump; `shortcut_goto_document_start` / Cmd+Up stays separate) |
| `shortcut_page_up` | Page-up chord (`PageUp` default; opposite PageDown = page down; letter remaps use Shift for page down; arrow defaults: Shift extends selection; when remapped, bare PageUp/PageDown no longer page) |
| `shortcut_char_left` | Character-left chord (`Left` default; opposite Right = char right; letter remaps use Shift for right; when remapped, bare Left/Right no longer move by character; word jump stays separate) |
| `shortcut_line_up` | Line-up chord (`Up` default; opposite Down = line down; letter remaps use Shift for down; arrow defaults: Shift extends selection; when remapped, bare Up/Down no longer move by line; move-line and document start/end stay separate) |
| `shortcut_undo` | Undo chord (`Cmd+Z` default; Shift flips to redo) |
| `shortcut_redo` | Redo alternate chord (`Cmd+Y` default; Shift+undo also redo) |
| `shortcut_zoom_in` | Zoom-in chord (`Cmd+=` default) |
| `shortcut_zoom_out` | Zoom-out chord (`Cmd+-` default) |
| `shortcut_zoom_restore` | Zoom-restore chord (`Cmd+0` default; mouse wheel stays hard-wired) |
| `shortcut_toggle_log_tail` | Toggle log-tail follow chord (`Cmd+Shift+T` default) |
| `shortcut_document_map` | Document Map toggle chord (`Cmd+Shift+D` default) |
| `shortcut_function_list` | Function List toggle chord (`Cmd+E` default; Shift flips to Character Panel) |
| `shortcut_fullscreen` | Fullscreen toggle chord (`F11` default) |
| `shortcut_postit` | Post-It / distraction-free toggle chord (`F12` default; hides menu + status; same as View → Distraction Free) |
| `shortcut_always_on_top` | Always-on-top toggle chord (`Alt+T` default) |
| `shortcut_show_whitespace` | Show-whitespace toggle chord (`Alt+V` default; Shift flips to Show EOL) |
| `shortcut_show_npc` | Show-NPC toggle chord (`Alt+N` default) |
| `shortcut_show_line_numbers` | Show-line-numbers toggle chord (`Alt+L` default; View → Show Line Numbers) |
| `shortcut_show_fold_margin` | Show-fold-margin toggle chord (`Alt+G` default; View → Show Fold Margin) |
| `shortcut_sync_scroll` | Sync vertical scroll toggle chord (`Alt+Q` default; Shift flips to Sync H) |
| `shortcut_zoom_sync` | Zoom Sync toggle chord (`Alt+Shift+Z` default; dual-view shared font size) |
| `shortcut_focus_other_view` | Focus other dual-view pane chord (`F6` default; Shift swaps tabs) |
| `shortcut_move_other_view` | Move / clone to other view chord (`Alt+O` default; Shift clones) |
| `shortcut_rtl` | RTL text-direction chord (`Alt+R` default; Shift flips to LTR) |
| `shortcut_reload` | Reload-from-disk chord (`Cmd+R` default) |
| `ollama_host` | Local Ollama base URL (loopback only; default `http://127.0.0.1:11434`) |
| `ollama_model` | Model for Plugins → Ask Ollama (default `llama3.2`) |

Unknown extra keys are kept on load/save (not dropped).

See also: `docs/autosave-backup.md`, `docs/folding.md`, `docs/ollama-helper.md`.

## Session

File: `npp-rs/session.txt` (config dir). One path per line; optional `@folds h1,h2,…` after a path restores folded fold-header lines; optional `@bookmarks l1,l2,…` restores bookmarked lines; optional `@style1`…`@style5` line lists restore Search → Style marks; optional `@caret LINE COL` (0-based) restores caret; optional `@sel A_LINE A_COL C_LINE C_COL` restores a non-empty selection (anchor then caret; `@tail` clears it); optional `@scroll LINE` (0-based first visible line per tab) restores viewport scroll (wins over caret follow; tab switches keep each tab’s scroll); optional `@lang ID` restores Language-menu highlight id; optional `@encoding TOKEN` restores save encoding (`utf8`, `utf8-bom`, `utf16-le`, `utf16-be`, `windows-1252`); optional `@readonly` (alias `@ro`) restores Edit → Toggle Read-Only; optional `@pin` restores a pinned tab; optional `@colour N` (alias `@color N`, 1..=5) restores View → Tab colour; optional `@tail` (alias `@logtail`) restores View → Monitoring (log-tail follow; jumps caret to end); optional `@other` (alias `@otherview`) restores the dual-view other pane; optional `@focus` (alias `@otherfocus`) restores keyboard focus on that other pane; optional `@syncv` / `@synch` (aliases `@syncvert` / `@synchoriz`) restore Synchronize Vertical / Horizontal Scrolling with dual view; optional `@compare` (alias `@cmp`) restarts Compare against the other pane; optional `@partner` (alias `@cmppartner`) restores a ⌘/Ctrl-click marked Compare partner tab; optional `@active` marks the focused tab.  
Saved on quit when restore is on. Menu Save/Load Session uses the same file.

## Find

Bar shows Case / Word / Sel / Wrap / Re toggles and live match count. Opening Find with a multi-line selection arms Sel. Next/Prev status is `n/total`. Wrap off reports passed end/beginning of file or selection. Re uses a linear-time regex engine. With Re on, Replace expands `$n` / `${n}` / `$&` / `$$` and `\1` (also `\n` `\t` `\r`).
