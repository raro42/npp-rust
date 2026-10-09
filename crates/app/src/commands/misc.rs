//! Tools / Window / Settings menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;
use std::path::PathBuf;

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_TOOL_")
        || cmd.starts_with("IDM_WINDOW_")
        || cmd.starts_with("IDM_SETTING_")
        || cmd.starts_with("IDM_EXECUTE")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
        "IDM_SETTING_PLUGINADM" => {
            show_plugin_admin(state);
            CmdResult::Handled
        }
        "IDM_SETTING_SHORTCUT_MAPPER" => {
            show_shortcut_mapper(state);
            CmdResult::Handled
        }
        "IDM_SETTING_PREFERENCE" => {
            ui.show_preferences = true;
            CmdResult::Handled
        }
        "IDM_LANGSTYLE_CONFIG_DLG" => {
            show_style_config(state);
            CmdResult::Handled
        }
        "IDM_SETTING_IMPORTPLUGIN" => {
            import_plugins(state);
            CmdResult::Handled
        }
        "IDM_SETTING_IMPORTSTYLETHEMES" => {
            import_themes(state, ui);
            CmdResult::Handled
        }
        "IDM_WINDOW_WINDOWS" => {
            ui.show_doc_list = true;
            CmdResult::Handled
        }
        "IDM_EXECUTE" => {
            run_execute(state);
            CmdResult::Handled
        }
        "IDM_EXECUTE_VALIDATE_SHORTCUTSXML" => {
            validate_shortcuts_xml(state);
            CmdResult::Handled
        }
        "IDM_TOOL_MD5_GENERATE" | "IDM_TOOL_MD5_GENERATEINTOCLIPBOARD" => {
            hash_selection_or_doc(state, ui, "md5", cmd.ends_with("CLIPBOARD"));
            CmdResult::Handled
        }
        "IDM_TOOL_MD5_GENERATEFROMFILE" => {
            hash_active_file(state, ui, "md5");
            CmdResult::Handled
        }
        "IDM_TOOL_SHA1_GENERATE" | "IDM_TOOL_SHA1_GENERATEINTOCLIPBOARD" => {
            hash_selection_or_doc(state, ui, "sha1", cmd.ends_with("CLIPBOARD"));
            CmdResult::Handled
        }
        "IDM_TOOL_SHA1_GENERATEFROMFILE" => {
            hash_active_file(state, ui, "sha1");
            CmdResult::Handled
        }
        "IDM_TOOL_SHA256_GENERATE" | "IDM_TOOL_SHA256_GENERATEINTOCLIPBOARD" => {
            hash_selection_or_doc(state, ui, "sha256", cmd.ends_with("CLIPBOARD"));
            CmdResult::Handled
        }
        "IDM_TOOL_SHA256_GENERATEFROMFILE" => {
            hash_active_file(state, ui, "sha256");
            CmdResult::Handled
        }
        "IDM_TOOL_SHA512_GENERATE" | "IDM_TOOL_SHA512_GENERATEINTOCLIPBOARD" => {
            hash_selection_or_doc(state, ui, "sha512", cmd.ends_with("CLIPBOARD"));
            CmdResult::Handled
        }
        "IDM_TOOL_SHA512_GENERATEFROMFILE" => {
            hash_active_file(state, ui, "sha512");
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FN_ASC" => {
            state
                .tabs
                .sort_tabs(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            state.status = "Tabs sorted by name ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FN_DSC" => {
            state
                .tabs
                .sort_tabs(|a, b| b.title.to_lowercase().cmp(&a.title.to_lowercase()));
            state.status = "Tabs sorted by name ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FP_ASC" => {
            state.tabs.sort_tabs(|a, b| {
                let ap = a
                    .path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                let bp = b
                    .path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                ap.cmp(&bp)
            });
            state.status = "Tabs sorted by path ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FP_DSC" => {
            state.tabs.sort_tabs(|a, b| {
                let ap = a
                    .path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                let bp = b
                    .path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                bp.cmp(&ap)
            });
            state.status = "Tabs sorted by path ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FT_ASC" => {
            state
                .tabs
                .sort_tabs(|a, b| tab_type_key(a).cmp(&tab_type_key(b)));
            state.status = "Tabs sorted by type ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FT_DSC" => {
            state
                .tabs
                .sort_tabs(|a, b| tab_type_key(b).cmp(&tab_type_key(a)));
            state.status = "Tabs sorted by type ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FS_ASC" => {
            state
                .tabs
                .sort_tabs(|a, b| a.buffer.len_chars().cmp(&b.buffer.len_chars()));
            state.status = "Tabs sorted by size ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FS_DSC" => {
            state
                .tabs
                .sort_tabs(|a, b| b.buffer.len_chars().cmp(&a.buffer.len_chars()));
            state.status = "Tabs sorted by size ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FD_ASC" => {
            state.tabs.sort_tabs(|a, b| tab_mtime(a).cmp(&tab_mtime(b)));
            state.status = "Tabs sorted by modified ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FD_DSC" => {
            state.tabs.sort_tabs(|a, b| tab_mtime(b).cmp(&tab_mtime(a)));
            state.status = "Tabs sorted by modified ↓".into();
            CmdResult::Handled
        }
        "IDM_SETTING_OPENPLUGINSDIR" => {
            let dir = cwd();
            open_path_in_os(state, &dir);
            CmdResult::Handled
        }
        _ => CmdResult::Stub,
    })
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Open an untitled read-only info tab (same pattern as Debug Info).
fn open_info_tab(state: &mut EditorState, title: &str, text: &str) {
    state.tabs.open_untitled();
    {
        let doc = state.tabs.active_mut();
        doc.title = title.into();
        doc.buffer = buffer::TextBuffer::from_str(text);
        doc.dirty = false;
        doc.language = "plain".into();
        doc.read_only = true;
    }
    state.highlight_dirty = true;
    state.reset_view = true;
}

fn show_shortcut_mapper(state: &mut EditorState) {
    use crate::shortcut_chord::{
        resolve_chord, DEFAULT_APPLY_COMPARE_HUNK, DEFAULT_BOOKMARK_COMPARE_DIFFS,
        DEFAULT_CLOSE_FIND, DEFAULT_CLOSE_TAB, DEFAULT_COMPARE, DEFAULT_COMPARE_IGNORE_BLANK,
        DEFAULT_COMPARE_IGNORE_WS, DEFAULT_COMPARE_TO_SAVED, DEFAULT_COPY_COMPARE_DIFF,
        DEFAULT_COPY_COMPARE_HUNK, DEFAULT_COPY_COMPARE_SUMMARY, DEFAULT_DELETE_LINE,
        DEFAULT_DUPLICATE_LINE, DEFAULT_EXPAND_ALL_UNCHANGED, DEFAULT_EXPAND_UNCHANGED_AT_CARET,
        DEFAULT_FIND, DEFAULT_FIND_NEXT, DEFAULT_FIND_NEXT_GLOBAL, DEFAULT_FIRST_DIFF,
        DEFAULT_FIRST_HIDDEN_EQUAL, DEFAULT_FOLD_ALL, DEFAULT_FOLD_CURRENT,
        DEFAULT_FORMAT_DOCUMENT, DEFAULT_GOTO_LINE, DEFAULT_HIDE_EQUAL, DEFAULT_HIDE_EQUAL_CONTEXT,
        DEFAULT_INDENT, DEFAULT_MATCHING_BRACE, DEFAULT_MOVE_LINE, DEFAULT_NEW,
        DEFAULT_NEXT_BOOKMARK, DEFAULT_NEXT_DIFF, DEFAULT_NEXT_HIDDEN_EQUAL, DEFAULT_NEXT_TAB,
        DEFAULT_OPEN, DEFAULT_OUTDENT, DEFAULT_PRINT, DEFAULT_REDO, DEFAULT_RELOAD,
        DEFAULT_REPLACE, DEFAULT_REPLACE_ALT, DEFAULT_SAVE, DEFAULT_SAVE_ALL, DEFAULT_SAVE_AS,
        DEFAULT_SELECT_ALL, DEFAULT_SHIFT_TAB_OUTDENT, DEFAULT_SWAP_COMPARE, DEFAULT_TAB_INDENT,
        DEFAULT_TOGGLE_BOOKMARK, DEFAULT_TOGGLE_COMMENT, DEFAULT_TOGGLE_LOG_TAIL, DEFAULT_UNDO,
        DEFAULT_WORD_JUMP, DEFAULT_WORD_WRAP, DEFAULT_ZOOM_IN, DEFAULT_ZOOM_OUT,
        DEFAULT_ZOOM_RESTORE,
    };
    let wrap = resolve_chord(&state.settings.shortcut_word_wrap, DEFAULT_WORD_WRAP).display();
    let wrap_raw = state.settings.shortcut_word_wrap.trim();
    let wrap_note = if wrap_raw.is_empty() || wrap_raw.eq_ignore_ascii_case(DEFAULT_WORD_WRAP) {
        format!("{wrap} (default; Preferences → Word wrap shortcut)")
    } else {
        format!("{wrap} (from settings.shortcut_word_wrap = {wrap_raw:?})")
    };
    let find_chord = resolve_chord(&state.settings.shortcut_find_next, DEFAULT_FIND_NEXT);
    let find_next = find_chord.display();
    let find_prev = find_chord.flipped_shift().display();
    let find_raw = state.settings.shortcut_find_next.trim();
    let find_note = if find_raw.is_empty() || find_raw.eq_ignore_ascii_case(DEFAULT_FIND_NEXT) {
        format!("{find_next} / {find_prev} (default; Preferences → Find next shortcut)")
    } else {
        format!(
            "{find_next} / {find_prev} (from settings.shortcut_find_next = {find_raw:?}; Shift flips)"
        )
    };
    let find_global_chord = resolve_chord(
        &state.settings.shortcut_find_next_global,
        DEFAULT_FIND_NEXT_GLOBAL,
    );
    let find_global_next = find_global_chord.display();
    let find_global_prev = find_global_chord.flipped_shift().display();
    let find_global_raw = state.settings.shortcut_find_next_global.trim();
    let find_global_note = if find_global_raw.is_empty()
        || find_global_raw.eq_ignore_ascii_case(DEFAULT_FIND_NEXT_GLOBAL)
    {
        format!(
            "{find_global_next} / {find_global_prev} (default; Preferences → Find next (global) shortcut)"
        )
    } else {
        format!(
            "{find_global_next} / {find_global_prev} (from settings.shortcut_find_next_global = {find_global_raw:?}; Shift flips)"
        )
    };
    let bm_chord = resolve_chord(
        &state.settings.shortcut_next_bookmark,
        DEFAULT_NEXT_BOOKMARK,
    );
    let bm_next = bm_chord.display();
    let bm_prev = bm_chord.flipped_shift().display();
    let bm_raw = state.settings.shortcut_next_bookmark.trim();
    let bm_note = if bm_raw.is_empty() || bm_raw.eq_ignore_ascii_case(DEFAULT_NEXT_BOOKMARK) {
        format!("{bm_next} / {bm_prev} (default; Preferences → Next bookmark shortcut)")
    } else {
        format!(
            "{bm_next} / {bm_prev} (from settings.shortcut_next_bookmark = {bm_raw:?}; Shift flips)"
        )
    };
    let toggle_bm = resolve_chord(
        &state.settings.shortcut_toggle_bookmark,
        DEFAULT_TOGGLE_BOOKMARK,
    )
    .display();
    let toggle_bm_raw = state.settings.shortcut_toggle_bookmark.trim();
    let toggle_bm_note = if toggle_bm_raw.is_empty()
        || toggle_bm_raw.eq_ignore_ascii_case(DEFAULT_TOGGLE_BOOKMARK)
    {
        format!("{toggle_bm} (default; Preferences → Toggle bookmark shortcut)")
    } else {
        format!("{toggle_bm} (from settings.shortcut_toggle_bookmark = {toggle_bm_raw:?})")
    };
    let diff_chord = resolve_chord(&state.settings.shortcut_next_diff, DEFAULT_NEXT_DIFF);
    let diff_next = diff_chord.display();
    let diff_prev = diff_chord.flipped_shift().display();
    let diff_raw = state.settings.shortcut_next_diff.trim();
    let diff_note = if diff_raw.is_empty() || diff_raw.eq_ignore_ascii_case(DEFAULT_NEXT_DIFF) {
        format!("{diff_next} / {diff_prev} (default; Preferences → Next difference shortcut)")
    } else {
        format!(
            "{diff_next} / {diff_prev} (from settings.shortcut_next_diff = {diff_raw:?}; Shift flips)"
        )
    };
    let first_diff_chord = resolve_chord(&state.settings.shortcut_first_diff, DEFAULT_FIRST_DIFF);
    let first_diff = first_diff_chord.display();
    let last_diff = first_diff_chord.flipped_shift().display();
    let first_diff_raw = state.settings.shortcut_first_diff.trim();
    let first_diff_note = if first_diff_raw.is_empty()
        || first_diff_raw.eq_ignore_ascii_case(DEFAULT_FIRST_DIFF)
    {
        format!("{first_diff} / {last_diff} (default; Preferences → First difference shortcut)")
    } else {
        format!(
            "{first_diff} / {last_diff} (from settings.shortcut_first_diff = {first_diff_raw:?}; Shift flips)"
        )
    };
    let hidden_chord = resolve_chord(
        &state.settings.shortcut_next_hidden_equal,
        DEFAULT_NEXT_HIDDEN_EQUAL,
    );
    let hidden_next = hidden_chord.display();
    let hidden_prev = hidden_chord.flipped_shift().display();
    let hidden_raw = state.settings.shortcut_next_hidden_equal.trim();
    let hidden_note = if hidden_raw.is_empty()
        || hidden_raw.eq_ignore_ascii_case(DEFAULT_NEXT_HIDDEN_EQUAL)
    {
        format!("{hidden_next} / {hidden_prev} (default; Preferences → Next hidden equal shortcut)")
    } else {
        format!(
            "{hidden_next} / {hidden_prev} (from settings.shortcut_next_hidden_equal = {hidden_raw:?}; Shift flips)"
        )
    };
    let first_hidden_chord = resolve_chord(
        &state.settings.shortcut_first_hidden_equal,
        DEFAULT_FIRST_HIDDEN_EQUAL,
    );
    let first_hidden = first_hidden_chord.display();
    let last_hidden = first_hidden_chord.flipped_shift().display();
    let first_hidden_raw = state.settings.shortcut_first_hidden_equal.trim();
    let first_hidden_note = if first_hidden_raw.is_empty()
        || first_hidden_raw.eq_ignore_ascii_case(DEFAULT_FIRST_HIDDEN_EQUAL)
    {
        format!(
            "{first_hidden} / {last_hidden} (default; Preferences → First hidden equal shortcut)"
        )
    } else {
        format!(
            "{first_hidden} / {last_hidden} (from settings.shortcut_first_hidden_equal = {first_hidden_raw:?}; Shift flips)"
        )
    };
    let apply_chord = resolve_chord(
        &state.settings.shortcut_apply_compare_hunk,
        DEFAULT_APPLY_COMPARE_HUNK,
    );
    let apply_from = apply_chord.display();
    let apply_to = apply_chord.flipped_horizontal().display();
    let apply_all_from = apply_chord.flipped_shift().display();
    let apply_all_to = apply_chord.flipped_horizontal().flipped_shift().display();
    let apply_raw = state.settings.shortcut_apply_compare_hunk.trim();
    let apply_note = if apply_raw.is_empty()
        || apply_raw.eq_ignore_ascii_case(DEFAULT_APPLY_COMPARE_HUNK)
    {
        format!(
            "{apply_from} / {apply_to} · {apply_all_from} / {apply_all_to} (default; Preferences → Apply compare hunk shortcut)"
        )
    } else {
        format!(
            "{apply_from} / {apply_to} · {apply_all_from} / {apply_all_to} (from settings.shortcut_apply_compare_hunk = {apply_raw:?}; opposite arrow = to other; Shift = apply all)"
        )
    };
    let hide_ctx_chord = resolve_chord(
        &state.settings.shortcut_hide_equal_context,
        DEFAULT_HIDE_EQUAL_CONTEXT,
    );
    let hide_ctx_inc = hide_ctx_chord.display();
    let hide_ctx_dec = hide_ctx_chord.flipped_bracket().display();
    let hide_ctx_raw = state.settings.shortcut_hide_equal_context.trim();
    let hide_ctx_note = if hide_ctx_raw.is_empty()
        || hide_ctx_raw.eq_ignore_ascii_case(DEFAULT_HIDE_EQUAL_CONTEXT)
    {
        format!(
            "{hide_ctx_inc} / {hide_ctx_dec} (default; Preferences → Hide-equal context shortcut)"
        )
    } else {
        format!(
            "{hide_ctx_inc} / {hide_ctx_dec} (from settings.shortcut_hide_equal_context = {hide_ctx_raw:?}; opposite bracket = decrease)"
        )
    };
    let hide_equal =
        resolve_chord(&state.settings.shortcut_hide_equal, DEFAULT_HIDE_EQUAL).display();
    let hide_equal_raw = state.settings.shortcut_hide_equal.trim();
    let hide_equal_note =
        if hide_equal_raw.is_empty() || hide_equal_raw.eq_ignore_ascii_case(DEFAULT_HIDE_EQUAL) {
            format!("{hide_equal} (default; Preferences → Hide unchanged lines shortcut)")
        } else {
            format!("{hide_equal} (from settings.shortcut_hide_equal = {hide_equal_raw:?})")
        };
    let ignore_ws_chord = resolve_chord(
        &state.settings.shortcut_compare_ignore_ws,
        DEFAULT_COMPARE_IGNORE_WS,
    );
    let ignore_ws = ignore_ws_chord.display();
    let ignore_case = ignore_ws_chord.flipped_shift().display();
    let ignore_ws_raw = state.settings.shortcut_compare_ignore_ws.trim();
    let ignore_ws_note = if ignore_ws_raw.is_empty()
        || ignore_ws_raw.eq_ignore_ascii_case(DEFAULT_COMPARE_IGNORE_WS)
    {
        format!("{ignore_ws} / {ignore_case} (default; Preferences → Ignore whitespace shortcut)")
    } else {
        format!(
            "{ignore_ws} / {ignore_case} (from settings.shortcut_compare_ignore_ws = {ignore_ws_raw:?}; Shift flips to ignore case)"
        )
    };
    let ignore_blank = resolve_chord(
        &state.settings.shortcut_compare_ignore_blank,
        DEFAULT_COMPARE_IGNORE_BLANK,
    )
    .display();
    let ignore_blank_raw = state.settings.shortcut_compare_ignore_blank.trim();
    let ignore_blank_note = if ignore_blank_raw.is_empty()
        || ignore_blank_raw.eq_ignore_ascii_case(DEFAULT_COMPARE_IGNORE_BLANK)
    {
        format!("{ignore_blank} (default; Preferences → Ignore blank lines shortcut)")
    } else {
        format!(
            "{ignore_blank} (from settings.shortcut_compare_ignore_blank = {ignore_blank_raw:?})"
        )
    };
    let expand_all_chord = resolve_chord(
        &state.settings.shortcut_expand_all_unchanged,
        DEFAULT_EXPAND_ALL_UNCHANGED,
    );
    let expand_all = expand_all_chord.display();
    let collapse_all = expand_all_chord.flipped_shift().display();
    let expand_all_raw = state.settings.shortcut_expand_all_unchanged.trim();
    let expand_all_note = if expand_all_raw.is_empty()
        || expand_all_raw.eq_ignore_ascii_case(DEFAULT_EXPAND_ALL_UNCHANGED)
    {
        format!(
            "{expand_all} / {collapse_all} (default; Preferences → Expand all unchanged shortcut)"
        )
    } else {
        format!(
            "{expand_all} / {collapse_all} (from settings.shortcut_expand_all_unchanged = {expand_all_raw:?}; Shift flips to collapse)"
        )
    };
    let expand_at_caret_chord = resolve_chord(
        &state.settings.shortcut_expand_unchanged_at_caret,
        DEFAULT_EXPAND_UNCHANGED_AT_CARET,
    );
    let expand_at_caret = expand_at_caret_chord.display();
    let collapse_at_caret = expand_at_caret_chord.flipped_shift().display();
    let expand_at_caret_raw = state.settings.shortcut_expand_unchanged_at_caret.trim();
    let expand_at_caret_note = if expand_at_caret_raw.is_empty()
        || expand_at_caret_raw.eq_ignore_ascii_case(DEFAULT_EXPAND_UNCHANGED_AT_CARET)
    {
        format!(
            "{expand_at_caret} / {collapse_at_caret} (default; Preferences → Expand unchanged at caret shortcut)"
        )
    } else {
        format!(
            "{expand_at_caret} / {collapse_at_caret} (from settings.shortcut_expand_unchanged_at_caret = {expand_at_caret_raw:?}; Shift flips to collapse)"
        )
    };
    let bookmark_diffs_chord = resolve_chord(
        &state.settings.shortcut_bookmark_compare_diffs,
        DEFAULT_BOOKMARK_COMPARE_DIFFS,
    );
    let bookmark_diffs = bookmark_diffs_chord.display();
    let clear_diff_bookmarks = bookmark_diffs_chord.flipped_shift().display();
    let bookmark_diffs_raw = state.settings.shortcut_bookmark_compare_diffs.trim();
    let bookmark_diffs_note = if bookmark_diffs_raw.is_empty()
        || bookmark_diffs_raw.eq_ignore_ascii_case(DEFAULT_BOOKMARK_COMPARE_DIFFS)
    {
        format!(
            "{bookmark_diffs} / {clear_diff_bookmarks} (default; Preferences → Bookmark compare differences shortcut)"
        )
    } else {
        format!(
            "{bookmark_diffs} / {clear_diff_bookmarks} (from settings.shortcut_bookmark_compare_diffs = {bookmark_diffs_raw:?}; Shift flips to clear)"
        )
    };
    let compare_chord = resolve_chord(&state.settings.shortcut_compare, DEFAULT_COMPARE);
    let compare_start = compare_chord.display();
    let compare_clear = compare_chord.flipped_shift().display();
    let compare_raw = state.settings.shortcut_compare.trim();
    let compare_note = if compare_raw.is_empty()
        || compare_raw.eq_ignore_ascii_case(DEFAULT_COMPARE)
    {
        format!("{compare_start} / {compare_clear} (default; Preferences → Compare shortcut)")
    } else {
        format!(
                "{compare_start} / {compare_clear} (from settings.shortcut_compare = {compare_raw:?}; Shift flips to clear)"
            )
    };
    let swap_compare =
        resolve_chord(&state.settings.shortcut_swap_compare, DEFAULT_SWAP_COMPARE).display();
    let swap_compare_raw = state.settings.shortcut_swap_compare.trim();
    let swap_compare_note = if swap_compare_raw.is_empty()
        || swap_compare_raw.eq_ignore_ascii_case(DEFAULT_SWAP_COMPARE)
    {
        format!("{swap_compare} (default; Preferences → Swap Compare sides shortcut)")
    } else {
        format!("{swap_compare} (from settings.shortcut_swap_compare = {swap_compare_raw:?})")
    };
    let compare_to_saved = resolve_chord(
        &state.settings.shortcut_compare_to_saved,
        DEFAULT_COMPARE_TO_SAVED,
    )
    .display();
    let compare_to_saved_raw = state.settings.shortcut_compare_to_saved.trim();
    let compare_to_saved_note = if compare_to_saved_raw.is_empty()
        || compare_to_saved_raw.eq_ignore_ascii_case(DEFAULT_COMPARE_TO_SAVED)
    {
        format!("{compare_to_saved} (default; Preferences → Compare to Saved shortcut)")
    } else {
        format!(
            "{compare_to_saved} (from settings.shortcut_compare_to_saved = {compare_to_saved_raw:?})"
        )
    };
    let copy_compare_diff_chord = resolve_chord(
        &state.settings.shortcut_copy_compare_diff,
        DEFAULT_COPY_COMPARE_DIFF,
    );
    let copy_compare_diff = copy_compare_diff_chord.display();
    let open_compare_diff = copy_compare_diff_chord.flipped_shift().display();
    let copy_compare_diff_raw = state.settings.shortcut_copy_compare_diff.trim();
    let copy_compare_diff_note = if copy_compare_diff_raw.is_empty()
        || copy_compare_diff_raw.eq_ignore_ascii_case(DEFAULT_COPY_COMPARE_DIFF)
    {
        format!(
            "{copy_compare_diff} / {open_compare_diff} (default; Preferences → Copy Compare Diff shortcut)"
        )
    } else {
        format!(
            "{copy_compare_diff} / {open_compare_diff} (from settings.shortcut_copy_compare_diff = {copy_compare_diff_raw:?}; Shift flips to open)"
        )
    };
    let copy_compare_summary_chord = resolve_chord(
        &state.settings.shortcut_copy_compare_summary,
        DEFAULT_COPY_COMPARE_SUMMARY,
    );
    let copy_compare_summary = copy_compare_summary_chord.display();
    let open_compare_summary = copy_compare_summary_chord.flipped_shift().display();
    let copy_compare_summary_raw = state.settings.shortcut_copy_compare_summary.trim();
    let copy_compare_summary_note = if copy_compare_summary_raw.is_empty()
        || copy_compare_summary_raw.eq_ignore_ascii_case(DEFAULT_COPY_COMPARE_SUMMARY)
    {
        format!(
            "{copy_compare_summary} / {open_compare_summary} (default; Preferences → Copy Compare Summary shortcut)"
        )
    } else {
        format!(
            "{copy_compare_summary} / {open_compare_summary} (from settings.shortcut_copy_compare_summary = {copy_compare_summary_raw:?}; Shift flips to open)"
        )
    };
    let copy_compare_hunk_chord = resolve_chord(
        &state.settings.shortcut_copy_compare_hunk,
        DEFAULT_COPY_COMPARE_HUNK,
    );
    let copy_compare_hunk = copy_compare_hunk_chord.display();
    let open_compare_hunk = copy_compare_hunk_chord.flipped_shift().display();
    let copy_compare_hunk_raw = state.settings.shortcut_copy_compare_hunk.trim();
    let copy_compare_hunk_note = if copy_compare_hunk_raw.is_empty()
        || copy_compare_hunk_raw.eq_ignore_ascii_case(DEFAULT_COPY_COMPARE_HUNK)
    {
        format!(
            "{copy_compare_hunk} / {open_compare_hunk} (default; Preferences → Copy Compare Hunk shortcut)"
        )
    } else {
        format!(
            "{copy_compare_hunk} / {open_compare_hunk} (from settings.shortcut_copy_compare_hunk = {copy_compare_hunk_raw:?}; Shift flips to open)"
        )
    };
    let word_jump_chord = resolve_chord(&state.settings.shortcut_word_jump, DEFAULT_WORD_JUMP);
    let word_jump_back = word_jump_chord.display();
    let word_jump_fwd = word_jump_chord.flipped_horizontal().display();
    let word_jump_raw = state.settings.shortcut_word_jump.trim();
    let word_jump_note = if word_jump_raw.is_empty()
        || word_jump_raw.eq_ignore_ascii_case(DEFAULT_WORD_JUMP)
    {
        format!(
            "{word_jump_back} / {word_jump_fwd} (default; Preferences → Word jump shortcut; opposite arrow = forward; Shift extends)"
        )
    } else {
        format!(
            "{word_jump_back} / {word_jump_fwd} (from settings.shortcut_word_jump = {word_jump_raw:?}; opposite arrow = forward; Shift extends)"
        )
    };
    let goto = resolve_chord(&state.settings.shortcut_goto_line, DEFAULT_GOTO_LINE).display();
    let goto_raw = state.settings.shortcut_goto_line.trim();
    let goto_note = if goto_raw.is_empty() || goto_raw.eq_ignore_ascii_case(DEFAULT_GOTO_LINE) {
        format!("{goto} (default; Preferences → Go to line shortcut)")
    } else {
        format!("{goto} (from settings.shortcut_goto_line = {goto_raw:?})")
    };
    let dup = resolve_chord(
        &state.settings.shortcut_duplicate_line,
        DEFAULT_DUPLICATE_LINE,
    )
    .display();
    let dup_raw = state.settings.shortcut_duplicate_line.trim();
    let dup_note = if dup_raw.is_empty() || dup_raw.eq_ignore_ascii_case(DEFAULT_DUPLICATE_LINE) {
        format!("{dup} (default; Preferences → Duplicate line shortcut)")
    } else {
        format!("{dup} (from settings.shortcut_duplicate_line = {dup_raw:?})")
    };
    let del = resolve_chord(&state.settings.shortcut_delete_line, DEFAULT_DELETE_LINE).display();
    let del_raw = state.settings.shortcut_delete_line.trim();
    let del_note = if del_raw.is_empty() || del_raw.eq_ignore_ascii_case(DEFAULT_DELETE_LINE) {
        format!("{del} (default; Preferences → Delete line shortcut)")
    } else {
        format!("{del} (from settings.shortcut_delete_line = {del_raw:?})")
    };
    let indent = resolve_chord(&state.settings.shortcut_indent, DEFAULT_INDENT).display();
    let indent_raw = state.settings.shortcut_indent.trim();
    let indent_note = if indent_raw.is_empty() || indent_raw.eq_ignore_ascii_case(DEFAULT_INDENT) {
        format!("{indent} (default; Preferences → Indent shortcut)")
    } else {
        format!("{indent} (from settings.shortcut_indent = {indent_raw:?})")
    };
    let outdent = resolve_chord(&state.settings.shortcut_outdent, DEFAULT_OUTDENT).display();
    let outdent_raw = state.settings.shortcut_outdent.trim();
    let outdent_note =
        if outdent_raw.is_empty() || outdent_raw.eq_ignore_ascii_case(DEFAULT_OUTDENT) {
            format!("{outdent} (default; Preferences → Outdent shortcut)")
        } else {
            format!("{outdent} (from settings.shortcut_outdent = {outdent_raw:?})")
        };
    let tab_indent =
        resolve_chord(&state.settings.shortcut_tab_indent, DEFAULT_TAB_INDENT).display();
    let tab_indent_raw = state.settings.shortcut_tab_indent.trim();
    let tab_indent_note =
        if tab_indent_raw.is_empty() || tab_indent_raw.eq_ignore_ascii_case(DEFAULT_TAB_INDENT) {
            format!("{tab_indent} (default; Preferences → Tab indent shortcut)")
        } else {
            format!("{tab_indent} (from settings.shortcut_tab_indent = {tab_indent_raw:?})")
        };
    let shift_tab_outdent = resolve_chord(
        &state.settings.shortcut_shift_tab_outdent,
        DEFAULT_SHIFT_TAB_OUTDENT,
    )
    .display();
    let shift_tab_raw = state.settings.shortcut_shift_tab_outdent.trim();
    let shift_tab_outdent_note = if shift_tab_raw.is_empty()
        || shift_tab_raw.eq_ignore_ascii_case(DEFAULT_SHIFT_TAB_OUTDENT)
    {
        format!("{shift_tab_outdent} (default; Preferences → Shift+Tab outdent shortcut)")
    } else {
        format!(
            "{shift_tab_outdent} (from settings.shortcut_shift_tab_outdent = {shift_tab_raw:?})"
        )
    };
    let fold_all_chord = resolve_chord(&state.settings.shortcut_fold_all, DEFAULT_FOLD_ALL);
    let fold_all = fold_all_chord.display();
    let unfold_all = fold_all_chord.flipped_shift().display();
    let fold_all_raw = state.settings.shortcut_fold_all.trim();
    let fold_all_note = if fold_all_raw.is_empty()
        || fold_all_raw.eq_ignore_ascii_case(DEFAULT_FOLD_ALL)
    {
        format!("{fold_all} / {unfold_all} (default; Preferences → Fold all shortcut)")
    } else {
        format!(
            "{fold_all} / {unfold_all} (from settings.shortcut_fold_all = {fold_all_raw:?}; Shift flips to unfold)"
        )
    };
    let fold_current_chord =
        resolve_chord(&state.settings.shortcut_fold_current, DEFAULT_FOLD_CURRENT);
    let fold_current = fold_current_chord.display();
    let unfold_current = fold_current_chord.flipped_shift().display();
    let fold_current_raw = state.settings.shortcut_fold_current.trim();
    let fold_current_note = if fold_current_raw.is_empty()
        || fold_current_raw.eq_ignore_ascii_case(DEFAULT_FOLD_CURRENT)
    {
        format!("{fold_current} / {unfold_current} (default; Preferences → Fold current shortcut)")
    } else {
        format!(
            "{fold_current} / {unfold_current} (from settings.shortcut_fold_current = {fold_current_raw:?}; Shift flips to unfold)"
        )
    };
    let matching_brace_chord = resolve_chord(
        &state.settings.shortcut_matching_brace,
        DEFAULT_MATCHING_BRACE,
    );
    let matching_brace = matching_brace_chord.display();
    let select_braces = matching_brace_chord.flipped_shift().display();
    let matching_brace_raw = state.settings.shortcut_matching_brace.trim();
    let matching_brace_note = if matching_brace_raw.is_empty()
        || matching_brace_raw.eq_ignore_ascii_case(DEFAULT_MATCHING_BRACE)
    {
        format!(
            "{matching_brace} / {select_braces} (default; Preferences → Matching brace shortcut)"
        )
    } else {
        format!(
            "{matching_brace} / {select_braces} (from settings.shortcut_matching_brace = {matching_brace_raw:?}; Shift flips to select)"
        )
    };
    let move_line_chord = resolve_chord(&state.settings.shortcut_move_line, DEFAULT_MOVE_LINE);
    let move_line_up = move_line_chord.display();
    let move_line_down = move_line_chord.move_line_down_chord().display();
    let move_line_raw = state.settings.shortcut_move_line.trim();
    let move_line_note = if move_line_raw.is_empty()
        || move_line_raw.eq_ignore_ascii_case(DEFAULT_MOVE_LINE)
    {
        format!("{move_line_up} / {move_line_down} (default; Preferences → Move line shortcut)")
    } else {
        format!(
            "{move_line_up} / {move_line_down} (from settings.shortcut_move_line = {move_line_raw:?}; opposite arrow or Shift moves down)"
        )
    };
    let toggle_comment_chord = resolve_chord(
        &state.settings.shortcut_toggle_comment,
        DEFAULT_TOGGLE_COMMENT,
    );
    let toggle_comment = toggle_comment_chord.display();
    let stream_comment = toggle_comment_chord.flipped_shift().display();
    let toggle_comment_raw = state.settings.shortcut_toggle_comment.trim();
    let toggle_comment_note = if toggle_comment_raw.is_empty()
        || toggle_comment_raw.eq_ignore_ascii_case(DEFAULT_TOGGLE_COMMENT)
    {
        format!(
            "{toggle_comment} / {stream_comment} (default; Preferences → Toggle comment shortcut)"
        )
    } else {
        format!(
            "{toggle_comment} / {stream_comment} (from settings.shortcut_toggle_comment = {toggle_comment_raw:?}; Shift flips to block comment)"
        )
    };
    let format_doc = resolve_chord(
        &state.settings.shortcut_format_document,
        DEFAULT_FORMAT_DOCUMENT,
    )
    .display();
    let format_raw = state.settings.shortcut_format_document.trim();
    let format_note =
        if format_raw.is_empty() || format_raw.eq_ignore_ascii_case(DEFAULT_FORMAT_DOCUMENT) {
            format!("{format_doc} (default; Preferences → Format document shortcut)")
        } else {
            format!("{format_doc} (from settings.shortcut_format_document = {format_raw:?})")
        };
    let close_tab = resolve_chord(&state.settings.shortcut_close_tab, DEFAULT_CLOSE_TAB).display();
    let close_raw = state.settings.shortcut_close_tab.trim();
    let close_note = if close_raw.is_empty() || close_raw.eq_ignore_ascii_case(DEFAULT_CLOSE_TAB) {
        format!("{close_tab} (default; Preferences → Close tab shortcut)")
    } else {
        format!("{close_tab} (from settings.shortcut_close_tab = {close_raw:?})")
    };
    let next_tab_chord = resolve_chord(&state.settings.shortcut_next_tab, DEFAULT_NEXT_TAB);
    let next_tab = next_tab_chord.display();
    let prev_tab = next_tab_chord.flipped_shift().display();
    let next_tab_raw = state.settings.shortcut_next_tab.trim();
    let next_tab_note = if next_tab_raw.is_empty()
        || next_tab_raw.eq_ignore_ascii_case(DEFAULT_NEXT_TAB)
    {
        format!("{next_tab} / {prev_tab} (default; Preferences → Next tab shortcut)")
    } else {
        format!(
            "{next_tab} / {prev_tab} (from settings.shortcut_next_tab = {next_tab_raw:?}; Shift flips)"
        )
    };
    let new_file = resolve_chord(&state.settings.shortcut_new, DEFAULT_NEW).display();
    let new_raw = state.settings.shortcut_new.trim();
    let new_note = if new_raw.is_empty() || new_raw.eq_ignore_ascii_case(DEFAULT_NEW) {
        format!("{new_file} (default; Preferences → New shortcut)")
    } else {
        format!("{new_file} (from settings.shortcut_new = {new_raw:?})")
    };
    let open_file = resolve_chord(&state.settings.shortcut_open, DEFAULT_OPEN).display();
    let open_raw = state.settings.shortcut_open.trim();
    let open_note = if open_raw.is_empty() || open_raw.eq_ignore_ascii_case(DEFAULT_OPEN) {
        format!("{open_file} (default; Preferences → Open shortcut)")
    } else {
        format!("{open_file} (from settings.shortcut_open = {open_raw:?})")
    };
    let reload = resolve_chord(&state.settings.shortcut_reload, DEFAULT_RELOAD).display();
    let reload_raw = state.settings.shortcut_reload.trim();
    let reload_note = if reload_raw.is_empty() || reload_raw.eq_ignore_ascii_case(DEFAULT_RELOAD) {
        format!("{reload} (default; Preferences → Reload shortcut)")
    } else {
        format!("{reload} (from settings.shortcut_reload = {reload_raw:?})")
    };
    let save = resolve_chord(&state.settings.shortcut_save, DEFAULT_SAVE).display();
    let save_raw = state.settings.shortcut_save.trim();
    let save_note = if save_raw.is_empty() || save_raw.eq_ignore_ascii_case(DEFAULT_SAVE) {
        format!("{save} (default; Preferences → Save shortcut)")
    } else {
        format!("{save} (from settings.shortcut_save = {save_raw:?})")
    };
    let save_as = resolve_chord(&state.settings.shortcut_save_as, DEFAULT_SAVE_AS).display();
    let save_as_raw = state.settings.shortcut_save_as.trim();
    let save_as_note =
        if save_as_raw.is_empty() || save_as_raw.eq_ignore_ascii_case(DEFAULT_SAVE_AS) {
            format!("{save_as} (default; Preferences → Save As shortcut)")
        } else {
            format!("{save_as} (from settings.shortcut_save_as = {save_as_raw:?})")
        };
    let save_all = resolve_chord(&state.settings.shortcut_save_all, DEFAULT_SAVE_ALL).display();
    let save_all_raw = state.settings.shortcut_save_all.trim();
    let save_all_note =
        if save_all_raw.is_empty() || save_all_raw.eq_ignore_ascii_case(DEFAULT_SAVE_ALL) {
            format!("{save_all} (default; Preferences → Save All shortcut)")
        } else {
            format!("{save_all} (from settings.shortcut_save_all = {save_all_raw:?})")
        };
    let print = resolve_chord(&state.settings.shortcut_print, DEFAULT_PRINT).display();
    let print_raw = state.settings.shortcut_print.trim();
    let print_note = if print_raw.is_empty() || print_raw.eq_ignore_ascii_case(DEFAULT_PRINT) {
        format!("{print} (default; Preferences → Print shortcut)")
    } else {
        format!("{print} (from settings.shortcut_print = {print_raw:?})")
    };
    let find_open = resolve_chord(&state.settings.shortcut_find, DEFAULT_FIND).display();
    let find_open_raw = state.settings.shortcut_find.trim();
    let find_open_note =
        if find_open_raw.is_empty() || find_open_raw.eq_ignore_ascii_case(DEFAULT_FIND) {
            format!("{find_open} (default; Preferences → Find shortcut)")
        } else {
            format!("{find_open} (from settings.shortcut_find = {find_open_raw:?})")
        };
    let close_find =
        resolve_chord(&state.settings.shortcut_close_find, DEFAULT_CLOSE_FIND).display();
    let close_find_raw = state.settings.shortcut_close_find.trim();
    let close_find_note =
        if close_find_raw.is_empty() || close_find_raw.eq_ignore_ascii_case(DEFAULT_CLOSE_FIND) {
            format!("{close_find} (default; Preferences → Close Find/Replace shortcut)")
        } else {
            format!("{close_find} (from settings.shortcut_close_find = {close_find_raw:?})")
        };
    let replace = resolve_chord(&state.settings.shortcut_replace, DEFAULT_REPLACE).display();
    let replace_raw = state.settings.shortcut_replace.trim();
    let replace_note =
        if replace_raw.is_empty() || replace_raw.eq_ignore_ascii_case(DEFAULT_REPLACE) {
            format!("{replace} (default; Preferences → Replace shortcut)")
        } else {
            format!("{replace} (from settings.shortcut_replace = {replace_raw:?})")
        };
    let replace_alt =
        resolve_chord(&state.settings.shortcut_replace_alt, DEFAULT_REPLACE_ALT).display();
    let replace_alt_raw = state.settings.shortcut_replace_alt.trim();
    let replace_alt_note = if replace_alt_raw.is_empty()
        || replace_alt_raw.eq_ignore_ascii_case(DEFAULT_REPLACE_ALT)
    {
        format!("{replace_alt} (default; Preferences → Replace (alternate) shortcut)")
    } else {
        format!("{replace_alt} (from settings.shortcut_replace_alt = {replace_alt_raw:?})")
    };
    let select_all =
        resolve_chord(&state.settings.shortcut_select_all, DEFAULT_SELECT_ALL).display();
    let select_all_raw = state.settings.shortcut_select_all.trim();
    let select_all_note =
        if select_all_raw.is_empty() || select_all_raw.eq_ignore_ascii_case(DEFAULT_SELECT_ALL) {
            format!("{select_all} (default; Preferences → Select all shortcut)")
        } else {
            format!("{select_all} (from settings.shortcut_select_all = {select_all_raw:?})")
        };
    let undo_chord = resolve_chord(&state.settings.shortcut_undo, DEFAULT_UNDO);
    let undo = undo_chord.display();
    let redo_from_undo = undo_chord.flipped_shift().display();
    let undo_raw = state.settings.shortcut_undo.trim();
    let undo_note = if undo_raw.is_empty() || undo_raw.eq_ignore_ascii_case(DEFAULT_UNDO) {
        format!(
            "{undo} / {redo_from_undo} (default; Preferences → Undo shortcut; Shift flips to Redo)"
        )
    } else {
        format!(
            "{undo} / {redo_from_undo} (from settings.shortcut_undo = {undo_raw:?}; Shift flips)"
        )
    };
    let redo = resolve_chord(&state.settings.shortcut_redo, DEFAULT_REDO).display();
    let redo_raw = state.settings.shortcut_redo.trim();
    let redo_note = if redo_raw.is_empty() || redo_raw.eq_ignore_ascii_case(DEFAULT_REDO) {
        format!("{redo} (default; Preferences → Redo shortcut; alternate)")
    } else {
        format!("{redo} (from settings.shortcut_redo = {redo_raw:?})")
    };
    let zoom_in = resolve_chord(&state.settings.shortcut_zoom_in, DEFAULT_ZOOM_IN).display();
    let zoom_in_raw = state.settings.shortcut_zoom_in.trim();
    let zoom_in_note =
        if zoom_in_raw.is_empty() || zoom_in_raw.eq_ignore_ascii_case(DEFAULT_ZOOM_IN) {
            format!("{zoom_in} (default; Preferences → Zoom in shortcut)")
        } else {
            format!("{zoom_in} (from settings.shortcut_zoom_in = {zoom_in_raw:?})")
        };
    let zoom_out = resolve_chord(&state.settings.shortcut_zoom_out, DEFAULT_ZOOM_OUT).display();
    let zoom_out_raw = state.settings.shortcut_zoom_out.trim();
    let zoom_out_note =
        if zoom_out_raw.is_empty() || zoom_out_raw.eq_ignore_ascii_case(DEFAULT_ZOOM_OUT) {
            format!("{zoom_out} (default; Preferences → Zoom out shortcut)")
        } else {
            format!("{zoom_out} (from settings.shortcut_zoom_out = {zoom_out_raw:?})")
        };
    let zoom_restore =
        resolve_chord(&state.settings.shortcut_zoom_restore, DEFAULT_ZOOM_RESTORE).display();
    let zoom_restore_raw = state.settings.shortcut_zoom_restore.trim();
    let zoom_restore_note = if zoom_restore_raw.is_empty()
        || zoom_restore_raw.eq_ignore_ascii_case(DEFAULT_ZOOM_RESTORE)
    {
        format!("{zoom_restore} (default; Preferences → Zoom restore shortcut)")
    } else {
        format!("{zoom_restore} (from settings.shortcut_zoom_restore = {zoom_restore_raw:?})")
    };
    let log_tail = resolve_chord(
        &state.settings.shortcut_toggle_log_tail,
        DEFAULT_TOGGLE_LOG_TAIL,
    )
    .display();
    let log_tail_raw = state.settings.shortcut_toggle_log_tail.trim();
    let log_tail_note =
        if log_tail_raw.is_empty() || log_tail_raw.eq_ignore_ascii_case(DEFAULT_TOGGLE_LOG_TAIL) {
            format!("{log_tail} (default; Preferences → Toggle log tail shortcut)")
        } else {
            format!("{log_tail} (from settings.shortcut_toggle_log_tail = {log_tail_raw:?})")
        };
    // Mirrors crates/app/src/ui.rs handle_shortcuts (+ remappable bindings).
    let text = format!(
        "\
npp-rs keyboard shortcuts
=========================
Source: ui.rs handle_shortcuts. Most keys are hard-wired.
Word wrap, Find next, Find next (global), Next bookmark, Toggle bookmark, Compare start/clear, Swap Compare sides, Compare to Saved, Copy / Open Compare Diff, Copy / Open Compare Summary, Copy / Open Compare Hunk, Word jump, Next difference, First difference, Next hidden equal, First hidden equal, Apply compare hunk, Hide unchanged lines, Ignore whitespace / Ignore case, Ignore blank lines, Expand / Collapse all unchanged, Expand / Collapse unchanged at caret, Bookmark / Clear compare difference bookmarks, Hide-equal context, Go to line, Duplicate line, Delete line, Move line, Toggle comment, Indent, Outdent, Tab indent, Shift+Tab outdent, Fold / Unfold all, Fold / Unfold current, Matching brace, Format document, Close tab, Next / prev tab, New, Open, Reload, Save, Save As, Save All, Print, Find, Close Find/Replace, Replace, Replace (alternate), Select all, Undo, Redo, Zoom in, Zoom out, Zoom restore, and Toggle log tail are remappable via Preferences or npp-rs/settings.json
(keys shortcut_word_wrap, shortcut_find_next, shortcut_find_next_global, shortcut_next_bookmark, shortcut_toggle_bookmark, shortcut_compare, shortcut_swap_compare, shortcut_compare_to_saved, shortcut_copy_compare_diff, shortcut_copy_compare_summary, shortcut_copy_compare_hunk, shortcut_word_jump, shortcut_next_diff, shortcut_first_diff, shortcut_next_hidden_equal, shortcut_first_hidden_equal, shortcut_apply_compare_hunk, shortcut_hide_equal, shortcut_compare_ignore_ws, shortcut_compare_ignore_blank, shortcut_expand_all_unchanged, shortcut_expand_unchanged_at_caret, shortcut_bookmark_compare_diffs, shortcut_hide_equal_context, shortcut_goto_line, shortcut_duplicate_line, shortcut_delete_line, shortcut_move_line, shortcut_toggle_comment, shortcut_indent, shortcut_outdent, shortcut_tab_indent, shortcut_shift_tab_outdent, shortcut_fold_all, shortcut_fold_current, shortcut_matching_brace, shortcut_format_document, shortcut_close_tab, shortcut_next_tab, shortcut_new, shortcut_open, shortcut_reload, shortcut_save, shortcut_save_as, shortcut_save_all, shortcut_print, shortcut_find, shortcut_close_find, shortcut_replace, shortcut_replace_alt, shortcut_select_all, shortcut_undo, shortcut_redo, shortcut_zoom_in, shortcut_zoom_out, shortcut_zoom_restore, shortcut_toggle_log_tail). Full shortcuts.xml remap is not wired yet.
Settings → Validate shortcuts.xml reports presence only.

modifier notes
--------------
- Cmd on macOS, Ctrl elsewhere (egui command/ctrl).
- Shift means Shift held with the key.

File / edit
-----------
{new_note}
{open_note}
{reload_note}
{save_note}
{save_as_note}
{save_all_note}
{print_note}
{close_note}
{next_tab_note}
{undo_note}
{redo_note}
{select_all_note}
{word_jump_note}
{dup_note}
{del_note}
{move_line_note}
{toggle_comment_note}
{indent_note}
{outdent_note}
{tab_indent_note}
{shift_tab_outdent_note}
{format_note}
{fold_all_note}
{fold_current_note}

Find / navigate
---------------
{find_open_note}
{close_find_note}
{replace_note}
{replace_alt_note}
{find_note}
{find_global_note}
{goto_note}
{matching_brace_note}
{bm_note}
{toggle_bm_note}
{compare_note}
{swap_compare_note}
{compare_to_saved_note}
{copy_compare_diff_note}
{copy_compare_summary_note}
{copy_compare_hunk_note}
{diff_note}
{first_diff_note}
{hidden_note}
{first_hidden_note}
{apply_note}
{hide_equal_note}
{ignore_ws_note}
{ignore_blank_note}
{expand_all_note}
{expand_at_caret_note}
{bookmark_diffs_note}
{hide_ctx_note}

View / zoom
-----------
{zoom_in_note}
{zoom_out_note}
{zoom_restore_note}
Cmd+mouse wheel       Zoom in / out
{wrap_note}
Alt+drag              Rectangular / column select (multi-caret typing)
{log_tail_note}

Language / style
----------------
Use the Language menu (IDM_LANG_*) to set highlight via EditorState::set_language.
Style Configurator lists which langs have tree-sitter grammars.
Preferences (Settings → Preferences) sets log-tail policy and font size.
"
    );
    open_info_tab(state, "Shortcut Mapper", &text);
    state.status = "Shortcut Mapper opened".into();
}

fn show_style_config(state: &mut EditorState) {
    let current = state.tabs.active().language.clone();
    let candidates = [
        "rust",
        "c",
        "cpp",
        "json",
        "python",
        "sql",
        "markdown",
        "plain",
        "toml",
        "yaml",
        "shell",
        "javascript",
        "typescript",
        "html",
        "css",
        "go",
        "java",
    ];
    let mut with_hl = Vec::new();
    let mut without_hl = Vec::new();
    for lang in candidates {
        if lang == "plain" || state.highlighter.supports(lang) {
            with_hl.push(lang);
        } else {
            without_hl.push(lang);
        }
    }
    let mut text = format!(
        "\
npp-rs Style Configurator
=========================
Current document language: {current}

Tree-sitter highlight (highlighter.supports)
--------------------------------------------
"
    );
    for lang in &with_hl {
        let mark = if *lang == current.as_str() {
            "  ← current"
        } else {
            ""
        };
        text.push_str(&format!("  {lang}{mark}\n"));
    }
    text.push_str(
        "\n\
Detected / menu langs without highlight grammar
-----------------------------------------------
",
    );
    for lang in &without_hl {
        let mark = if *lang == current.as_str() {
            "  ← current"
        } else {
            ""
        };
        text.push_str(&format!("  {lang}{mark}\n"));
    }
    text.push_str(
        "\n\
How to set language / look
--------------------------
1. Language menu (IDM_LANG_*) → EditorState::set_language.
2. Preferences → font size (session slider; log-tail policy saves).
3. Settings → Import Style Theme(s) lists themes/ and opens that folder.
   There is no theme apply API yet (egui default look).
There is no full Style Configurator colour UI yet.
",
    );
    open_info_tab(state, "Style Configurator", &text);
    state.status = format!("Style Config: language={current}");
}

fn format_plugin_rows(host: &plugins::PluginHost) -> String {
    let mut text = String::new();
    for row in host.summaries() {
        text.push_str(&format!(
            "  {name}\n    id:   {id}\n    menu: {menu}\n    run:  Plugins menu → {name}\n\n",
            name = row.name,
            id = row.id,
            menu = row.menu_path,
        ));
    }
    text
}

fn list_folder_entries(dir: &PathBuf) -> (usize, String) {
    let mut names = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for ent in rd.flatten() {
            let name = ent.file_name().to_string_lossy().into_owned();
            let kind = if ent.path().is_dir() { "/" } else { "" };
            names.push(format!("{name}{kind}"));
        }
    }
    names.sort();
    let n = names.len();
    let body = if names.is_empty() {
        "  (empty)\n".into()
    } else {
        names
            .into_iter()
            .map(|n| format!("  {n}\n"))
            .collect::<String>()
    };
    (n, body)
}

fn show_plugin_admin(state: &mut EditorState) {
    let host = plugins::PluginHost::new();
    let n = host.summaries().len();
    let plugins_dir = cwd().join("plugins");
    let _ = std::fs::create_dir_all(&plugins_dir);
    let (disk_n, disk_list) = list_folder_entries(&plugins_dir);

    let mut text = format!(
        "\
npp-rs Plugin Admin
===================
In-process plugins only (crates/plugins). DLL / drop-in loaders are not supported.

Installed ({n})
-------------
"
    );
    text.push_str(&format_plugin_rows(&host));
    let format_chord = crate::shortcut_chord::resolve_chord(
        &state.settings.shortcut_format_document,
        crate::shortcut_chord::DEFAULT_FORMAT_DOCUMENT,
    )
    .display();
    text.push_str(&format!(
        "\
plugins/ on disk ({disk_n} entries)
--------------------------------
Path: {path}

{disk}
Action
------
- Run a builtin: Plugins menu (uses PluginHost id).
- Format Document: also {format_chord}.
- Ask Ollama / Ollama Status: local loopback helper (Preferences ollama_host / ollama_model).
- Import Plugin: opens plugins/ and refreshes this listing in a tab.
- Drop-in files in plugins/ are listed only; they do not load.
",
        path = plugins_dir.display(),
        disk = disk_list,
        format_chord = format_chord,
    ));
    open_info_tab(state, "Plugin Admin", &text);
    state.status = format!("Plugin Admin: {n} in-process, {disk_n} on disk in plugins/");
}

fn import_plugins(state: &mut EditorState) {
    let dir = cwd().join("plugins");
    let _ = std::fs::create_dir_all(&dir);
    open_path_in_os(state, &dir);

    let host = plugins::PluginHost::new();
    let n = host.summaries().len();
    let (disk_n, disk_list) = list_folder_entries(&dir);

    let mut text = format!(
        "\
npp-rs Import Plugin
====================
Opened folder: {path}

Limit
-----
Drop-in / external plugins do not load. Only in-process builtins from crates/plugins run.

In-process builtins ({n})
-----------------------
",
        path = dir.display(),
    );
    text.push_str(&format_plugin_rows(&host));
    text.push_str(&format!(
        "\
Current plugins/ listing ({disk_n})
----------------------------------
{disk}
What you can do
---------------
1. Use Plugins menu items for the builtins above.
2. Put files in plugins/ for your own notes; reopen Import Plugin to refresh this list.
3. Plugin Admin shows the same registry + folder snapshot.
",
        disk = disk_list,
    ));
    open_info_tab(state, "Import Plugin", &text);
    state.status = format!(
        "Import Plugin: opened plugins/ — {n} builtin(s), {disk_n} on disk (drop-in not loaded)"
    );
}

fn import_themes(state: &mut EditorState, ui: &mut UiFlags) {
    let dir = crate::theme::ensure_themes_dir();
    open_path_in_os(state, &dir);
    ui.show_theme_picker = true;
    let n = crate::theme::list_theme_choices().len();
    state.status =
        format!("Themes: opened themes/ — {n} choice(s). Use the Theme window to apply (JSON tokens or N++ XML subset).");
}

fn run_execute(state: &mut EditorState) {
    if let Some(path) = rfd::FileDialog::new().set_title("Run…").pick_file() {
        let label = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "program".into());
        match std::process::Command::new(&path).spawn() {
            Ok(_) => state.status = format!("Started {label}"),
            Err(e) => state.status = format!("Run failed: {e}"),
        }
    } else {
        state.open_shell_here();
    }
}

fn validate_shortcuts_xml(state: &mut EditorState) {
    let root = cwd();
    let candidates = [
        "shortcuts.xml",
        "npp-rs/shortcuts.xml",
        "crates/app/data/shortcuts.xml",
    ];
    for rel in candidates {
        let path = root.join(rel);
        if path.is_file() {
            state.status = format!(
                "Found {rel} — XML validator not wired yet (shortcuts still hard-coded in ui.rs)"
            );
            return;
        }
    }
    state.status =
        "Validate shortcuts.xml: file absent (checked shortcuts.xml, npp-rs/, crates/app/data/)"
            .into();
}
