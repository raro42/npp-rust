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
        resolve_chord, DEFAULT_ALWAYS_ON_TOP, DEFAULT_APPLY_COMPARE_HUNK, DEFAULT_AUTOCOMPLETE,
        DEFAULT_AUTOCOMPLETE_PATH, DEFAULT_BEGIN_END_SELECT, DEFAULT_BLANK_LINE,
        DEFAULT_BOOKMARK_COMPARE_DIFFS, DEFAULT_CHAR_LEFT, DEFAULT_CLEAR_ALL_STYLES,
        DEFAULT_CLEAR_BOOKMARKS, DEFAULT_CLEAR_CHANGE_HISTORY, DEFAULT_CLOSE_ALL,
        DEFAULT_CLOSE_BUT_PINNED, DEFAULT_CLOSE_FIND, DEFAULT_CLOSE_TAB, DEFAULT_CLOSE_TO_SIDE,
        DEFAULT_COLUMN_EDITOR, DEFAULT_COMPARE, DEFAULT_COMPARE_IGNORE_BLANK,
        DEFAULT_COMPARE_IGNORE_WS, DEFAULT_COMPARE_TO_SAVED, DEFAULT_COPY, DEFAULT_COPY_ALL_PATHS,
        DEFAULT_COPY_COMPARE_DIFF, DEFAULT_COPY_COMPARE_HUNK, DEFAULT_COPY_COMPARE_SUMMARY,
        DEFAULT_COPY_DIR, DEFAULT_COPY_MARKED_LINES, DEFAULT_COPY_PATH, DEFAULT_CUT,
        DEFAULT_DELETE_BACKWARD, DEFAULT_DELETE_FORWARD, DEFAULT_DELETE_LINE,
        DEFAULT_DOCUMENT_HOME, DEFAULT_DOCUMENT_MAP, DEFAULT_DUPLICATE_LINE, DEFAULT_ENTER,
        DEFAULT_EXPAND_ALL_UNCHANGED, DEFAULT_EXPAND_UNCHANGED_AT_CARET, DEFAULT_FIND,
        DEFAULT_FIND_ALL, DEFAULT_FIND_BOOKMARK, DEFAULT_FIND_CHAR_RANGE, DEFAULT_FIND_COPY,
        DEFAULT_FIND_CUT, DEFAULT_FIND_INCREMENTAL, DEFAULT_FIND_IN_FILES, DEFAULT_FIND_MARK,
        DEFAULT_FIND_NEXT, DEFAULT_FIND_NEXT_GLOBAL, DEFAULT_FIND_PURGE, DEFAULT_FIND_VOLATILE,
        DEFAULT_FIRST_DIFF, DEFAULT_FIRST_HIDDEN_EQUAL, DEFAULT_FIRST_TAB,
        DEFAULT_FOCUS_OTHER_VIEW, DEFAULT_FOLD_ALL, DEFAULT_FOLD_CURRENT, DEFAULT_FOLD_LEVEL,
        DEFAULT_FORMAT_DOCUMENT, DEFAULT_FULLSCREEN, DEFAULT_FUNCTION_LIST,
        DEFAULT_GOTO_DOCUMENT_START, DEFAULT_GOTO_LINE, DEFAULT_HIDE_EQUAL,
        DEFAULT_HIDE_EQUAL_CONTEXT, DEFAULT_INDENT, DEFAULT_INDENT_GUIDE, DEFAULT_INSERT_DATETIME,
        DEFAULT_INVERSE_BOOKMARKS, DEFAULT_INVERT_CASE, DEFAULT_JOIN_LINES, DEFAULT_LINE_HOME,
        DEFAULT_LINE_UP, DEFAULT_LOWERCASE, DEFAULT_MARK_ONE_STYLE, DEFAULT_MARK_STYLE,
        DEFAULT_MARK_STYLE_2, DEFAULT_MARK_STYLE_3, DEFAULT_MARK_STYLE_4, DEFAULT_MARK_STYLE_5,
        DEFAULT_MATCHING_BRACE, DEFAULT_MOVE_LINE, DEFAULT_MOVE_OTHER_VIEW, DEFAULT_MOVE_TAB,
        DEFAULT_MULTI_SELECT_ALL, DEFAULT_MULTI_SELECT_NEXT, DEFAULT_NEW, DEFAULT_NEXT_BOOKMARK,
        DEFAULT_NEXT_CHANGED, DEFAULT_NEXT_DIFF, DEFAULT_NEXT_FOUND, DEFAULT_NEXT_HIDDEN_EQUAL,
        DEFAULT_NEXT_STYLE, DEFAULT_NEXT_TAB, DEFAULT_OPEN, DEFAULT_OPEN_FOLDER,
        DEFAULT_OPEN_SELECTED, DEFAULT_OPEN_WORKSPACE, DEFAULT_OUTDENT, DEFAULT_PAGE_UP,
        DEFAULT_PASTE, DEFAULT_PASTE_MARKED_LINES, DEFAULT_PIN_TAB, DEFAULT_POSTIT, DEFAULT_PRINT,
        DEFAULT_PROJECT_PANEL, DEFAULT_RECT_SELECT, DEFAULT_REDO, DEFAULT_RELOAD,
        DEFAULT_REMOVE_DUP_LINES, DEFAULT_REMOVE_EMPTY_LINES, DEFAULT_REMOVE_UNMARKED,
        DEFAULT_REPLACE, DEFAULT_REPLACE_ALT, DEFAULT_RTL, DEFAULT_SAVE, DEFAULT_SAVE_ALL,
        DEFAULT_SAVE_AS, DEFAULT_SAVE_COPY, DEFAULT_SELECT_ALL, DEFAULT_SET_AND_FIND,
        DEFAULT_SHIFT_TAB_OUTDENT, DEFAULT_SHOW_ALL_CHARACTERS, DEFAULT_SHOW_FOLD_MARGIN,
        DEFAULT_SHOW_LINE_NUMBERS, DEFAULT_SHOW_NPC, DEFAULT_SHOW_WHITESPACE, DEFAULT_SORT_LINES,
        DEFAULT_SPACE_TO_TAB_ALL, DEFAULT_STYLE_TO_CLIP, DEFAULT_SWAP_COMPARE, DEFAULT_SYNC_SCROLL,
        DEFAULT_TAB_INDENT, DEFAULT_TAB_TO_SPACE, DEFAULT_TOGGLE_BOOKMARK, DEFAULT_TOGGLE_COMMENT,
        DEFAULT_TOGGLE_LOG_TAIL, DEFAULT_TOGGLE_READONLY, DEFAULT_TRIM_TRAILING, DEFAULT_UNDO,
        DEFAULT_WORD_JUMP, DEFAULT_WORD_WRAP, DEFAULT_ZOOM_IN, DEFAULT_ZOOM_OUT,
        DEFAULT_ZOOM_RESTORE, DEFAULT_ZOOM_SYNC,
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
    let set_and_find_chord =
        resolve_chord(&state.settings.shortcut_set_and_find, DEFAULT_SET_AND_FIND);
    let set_and_find_next = set_and_find_chord.display();
    let set_and_find_prev = set_and_find_chord.flipped_shift().display();
    let set_and_find_raw = state.settings.shortcut_set_and_find.trim();
    let set_and_find_note = if set_and_find_raw.is_empty()
        || set_and_find_raw.eq_ignore_ascii_case(DEFAULT_SET_AND_FIND)
    {
        format!(
            "{set_and_find_next} / {set_and_find_prev} (default; Preferences → Select and Find shortcut)"
        )
    } else {
        format!(
            "{set_and_find_next} / {set_and_find_prev} (from settings.shortcut_set_and_find = {set_and_find_raw:?}; Shift flips)"
        )
    };
    let next_found_chord = resolve_chord(&state.settings.shortcut_next_found, DEFAULT_NEXT_FOUND);
    let next_found_next = next_found_chord.display();
    let next_found_prev = next_found_chord.flipped_shift().display();
    let next_found_raw = state.settings.shortcut_next_found.trim();
    let next_found_note = if next_found_raw.is_empty()
        || next_found_raw.eq_ignore_ascii_case(DEFAULT_NEXT_FOUND)
    {
        format!(
            "{next_found_next} / {next_found_prev} (default; Preferences → Next search result shortcut)"
        )
    } else {
        format!(
            "{next_found_next} / {next_found_prev} (from settings.shortcut_next_found = {next_found_raw:?}; Shift flips)"
        )
    };
    let find_all_chord = resolve_chord(&state.settings.shortcut_find_all, DEFAULT_FIND_ALL);
    let find_all_current = find_all_chord.display();
    let find_all_opened = find_all_chord.flipped_shift().display();
    let find_all_raw = state.settings.shortcut_find_all.trim();
    let find_all_note = if find_all_raw.is_empty()
        || find_all_raw.eq_ignore_ascii_case(DEFAULT_FIND_ALL)
    {
        format!("{find_all_current} / {find_all_opened} (default; Preferences → Find All shortcut)")
    } else {
        format!(
            "{find_all_current} / {find_all_opened} (from settings.shortcut_find_all = {find_all_raw:?}; Shift flips)"
        )
    };
    let find_incremental = resolve_chord(
        &state.settings.shortcut_find_incremental,
        DEFAULT_FIND_INCREMENTAL,
    )
    .display();
    let find_incremental_raw = state.settings.shortcut_find_incremental.trim();
    let find_incremental_note = if find_incremental_raw.is_empty()
        || find_incremental_raw.eq_ignore_ascii_case(DEFAULT_FIND_INCREMENTAL)
    {
        format!("{find_incremental} (default; Preferences → Incremental Search shortcut)")
    } else {
        format!(
            "{find_incremental} (from settings.shortcut_find_incremental = {find_incremental_raw:?})"
        )
    };
    let find_mark = resolve_chord(&state.settings.shortcut_find_mark, DEFAULT_FIND_MARK).display();
    let find_mark_raw = state.settings.shortcut_find_mark.trim();
    let find_mark_note =
        if find_mark_raw.is_empty() || find_mark_raw.eq_ignore_ascii_case(DEFAULT_FIND_MARK) {
            format!("{find_mark} (default; Preferences → Find Mark shortcut)")
        } else {
            format!("{find_mark} (from settings.shortcut_find_mark = {find_mark_raw:?})")
        };
    let find_bookmark = resolve_chord(
        &state.settings.shortcut_find_bookmark,
        DEFAULT_FIND_BOOKMARK,
    )
    .display();
    let find_bookmark_raw = state.settings.shortcut_find_bookmark.trim();
    let find_bookmark_note = if find_bookmark_raw.is_empty()
        || find_bookmark_raw.eq_ignore_ascii_case(DEFAULT_FIND_BOOKMARK)
    {
        format!("{find_bookmark} (default; Preferences → Find Bookmark shortcut)")
    } else {
        format!("{find_bookmark} (from settings.shortcut_find_bookmark = {find_bookmark_raw:?})")
    };
    let find_copy_chord = resolve_chord(&state.settings.shortcut_find_copy, DEFAULT_FIND_COPY);
    let find_copy_matches = find_copy_chord.display();
    let find_copy_lines = find_copy_chord.flipped_shift().display();
    let find_copy_raw = state.settings.shortcut_find_copy.trim();
    let find_copy_note = if find_copy_raw.is_empty()
        || find_copy_raw.eq_ignore_ascii_case(DEFAULT_FIND_COPY)
    {
        format!(
            "{find_copy_matches} / {find_copy_lines} (default; Preferences → Find Copy shortcut)"
        )
    } else {
        format!(
            "{find_copy_matches} / {find_copy_lines} (from settings.shortcut_find_copy = {find_copy_raw:?}; Shift flips to matching lines)"
        )
    };
    let find_cut_chord = resolve_chord(&state.settings.shortcut_find_cut, DEFAULT_FIND_CUT);
    let find_cut_matches = find_cut_chord.display();
    let find_cut_lines = find_cut_chord.flipped_shift().display();
    let find_cut_raw = state.settings.shortcut_find_cut.trim();
    let find_cut_note = if find_cut_raw.is_empty()
        || find_cut_raw.eq_ignore_ascii_case(DEFAULT_FIND_CUT)
    {
        format!("{find_cut_matches} / {find_cut_lines} (default; Preferences → Find Cut shortcut)")
    } else {
        format!(
            "{find_cut_matches} / {find_cut_lines} (from settings.shortcut_find_cut = {find_cut_raw:?}; Shift flips to matching lines)"
        )
    };
    let find_purge_chord = resolve_chord(&state.settings.shortcut_find_purge, DEFAULT_FIND_PURGE);
    let find_purge_lines = find_purge_chord.display();
    let find_purge_matches = find_purge_chord.flipped_shift().display();
    let find_purge_raw = state.settings.shortcut_find_purge.trim();
    let find_purge_note = if find_purge_raw.is_empty()
        || find_purge_raw.eq_ignore_ascii_case(DEFAULT_FIND_PURGE)
    {
        format!(
            "{find_purge_lines} / {find_purge_matches} (default; Preferences → Find Purge shortcut)"
        )
    } else {
        format!(
            "{find_purge_lines} / {find_purge_matches} (from settings.shortcut_find_purge = {find_purge_raw:?}; Shift flips to matching substrings)"
        )
    };
    let find_char_range_chord = resolve_chord(
        &state.settings.shortcut_find_char_range,
        DEFAULT_FIND_CHAR_RANGE,
    );
    let find_char_range_next = find_char_range_chord.display();
    let find_char_range_prev = find_char_range_chord.flipped_shift().display();
    let find_char_range_raw = state.settings.shortcut_find_char_range.trim();
    let find_char_range_note = if find_char_range_raw.is_empty()
        || find_char_range_raw.eq_ignore_ascii_case(DEFAULT_FIND_CHAR_RANGE)
    {
        format!(
            "{find_char_range_next} / {find_char_range_prev} (default; Preferences → Find characters in range shortcut)"
        )
    } else {
        format!(
            "{find_char_range_next} / {find_char_range_prev} (from settings.shortcut_find_char_range = {find_char_range_raw:?}; Shift flips to previous)"
        )
    };
    let find_volatile_chord = resolve_chord(
        &state.settings.shortcut_find_volatile,
        DEFAULT_FIND_VOLATILE,
    );
    let find_volatile_next = find_volatile_chord.display();
    let find_volatile_prev = find_volatile_chord.flipped_shift().display();
    let find_volatile_raw = state.settings.shortcut_find_volatile.trim();
    let find_volatile_note = if find_volatile_raw.is_empty()
        || find_volatile_raw.eq_ignore_ascii_case(DEFAULT_FIND_VOLATILE)
    {
        format!(
            "{find_volatile_next} / {find_volatile_prev} (default; Preferences → Find Next (Volatile) shortcut)"
        )
    } else {
        format!(
            "{find_volatile_next} / {find_volatile_prev} (from settings.shortcut_find_volatile = {find_volatile_raw:?}; Shift flips to previous)"
        )
    };
    let mark_style_chord = resolve_chord(&state.settings.shortcut_mark_style, DEFAULT_MARK_STYLE);
    let mark_style_mark = mark_style_chord.display();
    let mark_style_clear = mark_style_chord.flipped_shift().display();
    let mark_style_raw = state.settings.shortcut_mark_style.trim();
    let mark_style_note = if mark_style_raw.is_empty()
        || mark_style_raw.eq_ignore_ascii_case(DEFAULT_MARK_STYLE)
    {
        format!(
            "{mark_style_mark} / {mark_style_clear} (default; Preferences → Mark Style 1 shortcut)"
        )
    } else {
        format!(
            "{mark_style_mark} / {mark_style_clear} (from settings.shortcut_mark_style = {mark_style_raw:?}; Shift clears Style 1)"
        )
    };
    let mark_style_2_chord =
        resolve_chord(&state.settings.shortcut_mark_style_2, DEFAULT_MARK_STYLE_2);
    let mark_style_2_mark = mark_style_2_chord.display();
    let mark_style_2_clear = mark_style_2_chord.flipped_shift().display();
    let mark_style_2_raw = state.settings.shortcut_mark_style_2.trim();
    let mark_style_2_note = if mark_style_2_raw.is_empty()
        || mark_style_2_raw.eq_ignore_ascii_case(DEFAULT_MARK_STYLE_2)
    {
        format!(
            "{mark_style_2_mark} / {mark_style_2_clear} (default; Preferences → Mark Style 2 shortcut)"
        )
    } else {
        format!(
            "{mark_style_2_mark} / {mark_style_2_clear} (from settings.shortcut_mark_style_2 = {mark_style_2_raw:?}; Shift clears Style 2)"
        )
    };
    let mark_style_3_chord =
        resolve_chord(&state.settings.shortcut_mark_style_3, DEFAULT_MARK_STYLE_3);
    let mark_style_3_mark = mark_style_3_chord.display();
    let mark_style_3_clear = mark_style_3_chord.flipped_shift().display();
    let mark_style_3_raw = state.settings.shortcut_mark_style_3.trim();
    let mark_style_3_note = if mark_style_3_raw.is_empty()
        || mark_style_3_raw.eq_ignore_ascii_case(DEFAULT_MARK_STYLE_3)
    {
        format!(
            "{mark_style_3_mark} / {mark_style_3_clear} (default; Preferences → Mark Style 3 shortcut)"
        )
    } else {
        format!(
            "{mark_style_3_mark} / {mark_style_3_clear} (from settings.shortcut_mark_style_3 = {mark_style_3_raw:?}; Shift clears Style 3)"
        )
    };
    let mark_style_4_chord =
        resolve_chord(&state.settings.shortcut_mark_style_4, DEFAULT_MARK_STYLE_4);
    let mark_style_4_mark = mark_style_4_chord.display();
    let mark_style_4_clear = mark_style_4_chord.flipped_shift().display();
    let mark_style_4_raw = state.settings.shortcut_mark_style_4.trim();
    let mark_style_4_note = if mark_style_4_raw.is_empty()
        || mark_style_4_raw.eq_ignore_ascii_case(DEFAULT_MARK_STYLE_4)
    {
        format!(
            "{mark_style_4_mark} / {mark_style_4_clear} (default; Preferences → Mark Style 4 shortcut)"
        )
    } else {
        format!(
            "{mark_style_4_mark} / {mark_style_4_clear} (from settings.shortcut_mark_style_4 = {mark_style_4_raw:?}; Shift clears Style 4)"
        )
    };
    let mark_style_5_chord =
        resolve_chord(&state.settings.shortcut_mark_style_5, DEFAULT_MARK_STYLE_5);
    let mark_style_5_mark = mark_style_5_chord.display();
    let mark_style_5_clear = mark_style_5_chord.flipped_shift().display();
    let mark_style_5_raw = state.settings.shortcut_mark_style_5.trim();
    let mark_style_5_note = if mark_style_5_raw.is_empty()
        || mark_style_5_raw.eq_ignore_ascii_case(DEFAULT_MARK_STYLE_5)
    {
        format!(
            "{mark_style_5_mark} / {mark_style_5_clear} (default; Preferences → Mark Style 5 shortcut)"
        )
    } else {
        format!(
            "{mark_style_5_mark} / {mark_style_5_clear} (from settings.shortcut_mark_style_5 = {mark_style_5_raw:?}; Shift clears Style 5)"
        )
    };
    let clear_all_styles = resolve_chord(
        &state.settings.shortcut_clear_all_styles,
        DEFAULT_CLEAR_ALL_STYLES,
    )
    .display();
    let clear_all_styles_raw = state.settings.shortcut_clear_all_styles.trim();
    let clear_all_styles_note = if clear_all_styles_raw.is_empty()
        || clear_all_styles_raw.eq_ignore_ascii_case(DEFAULT_CLEAR_ALL_STYLES)
    {
        format!("{clear_all_styles} (default; Preferences → Clear all styles shortcut)")
    } else {
        format!(
            "{clear_all_styles} (from settings.shortcut_clear_all_styles = {clear_all_styles_raw:?})"
        )
    };
    let next_style = resolve_chord(&state.settings.shortcut_next_style, DEFAULT_NEXT_STYLE)
        .style_marker_display();
    let next_style_raw = state.settings.shortcut_next_style.trim();
    let next_style_note = if next_style_raw.is_empty()
        || next_style_raw.eq_ignore_ascii_case(DEFAULT_NEXT_STYLE)
    {
        format!(
            "{next_style} (default; Preferences → Next style marker shortcut; digit-family = 1…5; Shift = prev)"
        )
    } else {
        format!(
            "{next_style} (from settings.shortcut_next_style = {next_style_raw:?}; digit-family = 1…5; Shift = prev)"
        )
    };
    let style_to_clip = resolve_chord(
        &state.settings.shortcut_style_to_clip,
        DEFAULT_STYLE_TO_CLIP,
    )
    .style_to_clip_display();
    let style_to_clip_raw = state.settings.shortcut_style_to_clip.trim();
    let style_to_clip_note = if style_to_clip_raw.is_empty()
        || style_to_clip_raw.eq_ignore_ascii_case(DEFAULT_STYLE_TO_CLIP)
    {
        format!(
            "{style_to_clip} (default; Preferences → Copy styled text shortcut; digit-family = 1…5; 0 = all)"
        )
    } else {
        format!(
            "{style_to_clip} (from settings.shortcut_style_to_clip = {style_to_clip_raw:?}; digit-family = 1…5; 0 = all / Shift = all on letter remaps)"
        )
    };
    let mark_one_style = resolve_chord(
        &state.settings.shortcut_mark_one_style,
        DEFAULT_MARK_ONE_STYLE,
    )
    .mark_one_style_display();
    let mark_one_style_raw = state.settings.shortcut_mark_one_style.trim();
    let mark_one_style_note = if mark_one_style_raw.is_empty()
        || mark_one_style_raw.eq_ignore_ascii_case(DEFAULT_MARK_ONE_STYLE)
    {
        format!(
            "{mark_one_style} (default; Preferences → Mark current line style shortcut; digit-family = 1…5)"
        )
    } else {
        format!(
            "{mark_one_style} (from settings.shortcut_mark_one_style = {mark_one_style_raw:?}; digit-family = 1…5)"
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
    let clear_bm = resolve_chord(
        &state.settings.shortcut_clear_bookmarks,
        DEFAULT_CLEAR_BOOKMARKS,
    )
    .display();
    let clear_bm_raw = state.settings.shortcut_clear_bookmarks.trim();
    let clear_bm_note =
        if clear_bm_raw.is_empty() || clear_bm_raw.eq_ignore_ascii_case(DEFAULT_CLEAR_BOOKMARKS) {
            format!("{clear_bm} (default; Preferences → Clear all bookmarks shortcut)")
        } else {
            format!("{clear_bm} (from settings.shortcut_clear_bookmarks = {clear_bm_raw:?})")
        };
    let inverse_bm = resolve_chord(
        &state.settings.shortcut_inverse_bookmarks,
        DEFAULT_INVERSE_BOOKMARKS,
    )
    .display();
    let inverse_bm_raw = state.settings.shortcut_inverse_bookmarks.trim();
    let inverse_bm_note = if inverse_bm_raw.is_empty()
        || inverse_bm_raw.eq_ignore_ascii_case(DEFAULT_INVERSE_BOOKMARKS)
    {
        format!("{inverse_bm} (default; Preferences → Inverse bookmarks shortcut)")
    } else {
        format!("{inverse_bm} (from settings.shortcut_inverse_bookmarks = {inverse_bm_raw:?})")
    };
    let copy_marked_chord = resolve_chord(
        &state.settings.shortcut_copy_marked_lines,
        DEFAULT_COPY_MARKED_LINES,
    );
    let copy_marked = copy_marked_chord.display();
    let cut_marked = copy_marked_chord.flipped_shift().display();
    let copy_marked_raw = state.settings.shortcut_copy_marked_lines.trim();
    let copy_marked_note = if copy_marked_raw.is_empty()
        || copy_marked_raw.eq_ignore_ascii_case(DEFAULT_COPY_MARKED_LINES)
    {
        format!(
            "{copy_marked} / {cut_marked} (default; Preferences → Copy bookmarked lines shortcut; Shift flips to Cut)"
        )
    } else {
        format!(
            "{copy_marked} / {cut_marked} (from settings.shortcut_copy_marked_lines = {copy_marked_raw:?}; Shift flips to Cut)"
        )
    };
    let paste_marked_chord = resolve_chord(
        &state.settings.shortcut_paste_marked_lines,
        DEFAULT_PASTE_MARKED_LINES,
    );
    let paste_marked = paste_marked_chord.display();
    let remove_marked = paste_marked_chord.flipped_shift().display();
    let paste_marked_raw = state.settings.shortcut_paste_marked_lines.trim();
    let paste_marked_note = if paste_marked_raw.is_empty()
        || paste_marked_raw.eq_ignore_ascii_case(DEFAULT_PASTE_MARKED_LINES)
    {
        format!(
            "{paste_marked} / {remove_marked} (default; Preferences → Paste bookmarked lines shortcut; Shift flips to Remove)"
        )
    } else {
        format!(
            "{paste_marked} / {remove_marked} (from settings.shortcut_paste_marked_lines = {paste_marked_raw:?}; Shift flips to Remove)"
        )
    };
    let remove_unmarked = resolve_chord(
        &state.settings.shortcut_remove_unmarked,
        DEFAULT_REMOVE_UNMARKED,
    )
    .display();
    let remove_unmarked_raw = state.settings.shortcut_remove_unmarked.trim();
    let remove_unmarked_note = if remove_unmarked_raw.is_empty()
        || remove_unmarked_raw.eq_ignore_ascii_case(DEFAULT_REMOVE_UNMARKED)
    {
        format!("{remove_unmarked} (default; Preferences → Remove unmarked lines shortcut)")
    } else {
        format!(
            "{remove_unmarked} (from settings.shortcut_remove_unmarked = {remove_unmarked_raw:?})"
        )
    };
    let changed_chord = resolve_chord(&state.settings.shortcut_next_changed, DEFAULT_NEXT_CHANGED);
    let changed_next = changed_chord.display();
    let changed_prev = changed_chord.flipped_shift().display();
    let changed_raw = state.settings.shortcut_next_changed.trim();
    let changed_note = if changed_raw.is_empty()
        || changed_raw.eq_ignore_ascii_case(DEFAULT_NEXT_CHANGED)
    {
        format!(
            "{changed_next} / {changed_prev} (default; Preferences → Next change history shortcut)"
        )
    } else {
        format!(
            "{changed_next} / {changed_prev} (from settings.shortcut_next_changed = {changed_raw:?}; Shift flips)"
        )
    };
    let clear_change_history = resolve_chord(
        &state.settings.shortcut_clear_change_history,
        DEFAULT_CLEAR_CHANGE_HISTORY,
    )
    .display();
    let clear_change_history_raw = state.settings.shortcut_clear_change_history.trim();
    let clear_change_history_note = if clear_change_history_raw.is_empty()
        || clear_change_history_raw.eq_ignore_ascii_case(DEFAULT_CLEAR_CHANGE_HISTORY)
    {
        format!("{clear_change_history} (default; Preferences → Clear change history shortcut)")
    } else {
        format!(
            "{clear_change_history} (from settings.shortcut_clear_change_history = {clear_change_history_raw:?})"
        )
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
    let rect_select = resolve_chord(&state.settings.shortcut_rect_select, DEFAULT_RECT_SELECT)
        .rect_select_display();
    let rect_select_raw = state.settings.shortcut_rect_select.trim();
    let rect_select_note = if rect_select_raw.is_empty()
        || rect_select_raw.eq_ignore_ascii_case(DEFAULT_RECT_SELECT)
    {
        format!(
            "{rect_select} (default; Preferences → Rect column select shortcut; arrow-family = any arrow)"
        )
    } else {
        format!(
            "{rect_select} (from settings.shortcut_rect_select = {rect_select_raw:?}; arrow-family = any arrow)"
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
    let fold_level =
        resolve_chord(&state.settings.shortcut_fold_level, DEFAULT_FOLD_LEVEL).fold_level_display();
    let fold_level_raw = state.settings.shortcut_fold_level.trim();
    let fold_level_note = if fold_level_raw.is_empty()
        || fold_level_raw.eq_ignore_ascii_case(DEFAULT_FOLD_LEVEL)
    {
        format!(
            "{fold_level} (default; Preferences → Fold level shortcut; digit-family = 1…8; Shift = unfold)"
        )
    } else {
        format!(
            "{fold_level} (from settings.shortcut_fold_level = {fold_level_raw:?}; digit-family = 1…8; Shift = unfold)"
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
    let goto_doc_chord = resolve_chord(
        &state.settings.shortcut_goto_document_start,
        DEFAULT_GOTO_DOCUMENT_START,
    );
    let goto_doc_start = goto_doc_chord.display();
    let goto_doc_end = goto_doc_chord.goto_document_end_chord().display();
    let goto_doc_raw = state.settings.shortcut_goto_document_start.trim();
    let goto_doc_note = if goto_doc_raw.is_empty()
        || goto_doc_raw.eq_ignore_ascii_case(DEFAULT_GOTO_DOCUMENT_START)
    {
        format!(
            "{goto_doc_start} / {goto_doc_end} (default; Preferences → Document start shortcut; Cmd+Home / Cmd+End aliases use Document Home shortcut)"
        )
    } else {
        format!(
            "{goto_doc_start} / {goto_doc_end} (from settings.shortcut_goto_document_start = {goto_doc_raw:?}; opposite arrow or Shift jumps to end; Cmd+Home / Cmd+End aliases use Document Home shortcut)"
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
    let case_chord = resolve_chord(&state.settings.shortcut_lowercase, DEFAULT_LOWERCASE);
    let case_lower = case_chord.display();
    let case_upper = case_chord.flipped_shift().display();
    let case_raw = state.settings.shortcut_lowercase.trim();
    let case_note = if case_raw.is_empty() || case_raw.eq_ignore_ascii_case(DEFAULT_LOWERCASE) {
        format!("{case_lower} / {case_upper} (default; Preferences → Lowercase shortcut)")
    } else {
        format!(
            "{case_lower} / {case_upper} (from settings.shortcut_lowercase = {case_raw:?}; Shift flips to UPPERCASE)"
        )
    };
    let autocomplete_chord =
        resolve_chord(&state.settings.shortcut_autocomplete, DEFAULT_AUTOCOMPLETE);
    let autocomplete = autocomplete_chord.display();
    let call_tip = autocomplete_chord.flipped_shift().display();
    let autocomplete_raw = state.settings.shortcut_autocomplete.trim();
    let autocomplete_note = if autocomplete_raw.is_empty()
        || autocomplete_raw.eq_ignore_ascii_case(DEFAULT_AUTOCOMPLETE)
    {
        format!(
            "{autocomplete} / {call_tip} (default; Preferences → Autocomplete shortcut; Shift flips to Function Call Tip)"
        )
    } else {
        format!(
            "{autocomplete} / {call_tip} (from settings.shortcut_autocomplete = {autocomplete_raw:?}; Shift flips to Function Call Tip)"
        )
    };
    let path_complete = resolve_chord(
        &state.settings.shortcut_autocomplete_path,
        DEFAULT_AUTOCOMPLETE_PATH,
    )
    .display();
    let path_raw = state.settings.shortcut_autocomplete_path.trim();
    let path_note =
        if path_raw.is_empty() || path_raw.eq_ignore_ascii_case(DEFAULT_AUTOCOMPLETE_PATH) {
            format!("{path_complete} (default; Preferences → Path autocomplete shortcut)")
        } else {
            format!("{path_complete} (from settings.shortcut_autocomplete_path = {path_raw:?})")
        };
    let join_chord = resolve_chord(&state.settings.shortcut_join_lines, DEFAULT_JOIN_LINES);
    let join_lines = join_chord.display();
    let split_lines = join_chord.flipped_shift().display();
    let join_raw = state.settings.shortcut_join_lines.trim();
    let join_note = if join_raw.is_empty() || join_raw.eq_ignore_ascii_case(DEFAULT_JOIN_LINES) {
        format!("{join_lines} / {split_lines} (default; Preferences → Join Lines shortcut)")
    } else {
        format!(
            "{join_lines} / {split_lines} (from settings.shortcut_join_lines = {join_raw:?}; Shift flips to Split Lines)"
        )
    };
    let invert_chord = resolve_chord(&state.settings.shortcut_invert_case, DEFAULT_INVERT_CASE);
    let invert_case = invert_chord.display();
    let proper_case = invert_chord.flipped_shift().display();
    let invert_raw = state.settings.shortcut_invert_case.trim();
    let invert_note = if invert_raw.is_empty()
        || invert_raw.eq_ignore_ascii_case(DEFAULT_INVERT_CASE)
    {
        format!("{invert_case} / {proper_case} (default; Preferences → Invert Case shortcut)")
    } else {
        format!(
            "{invert_case} / {proper_case} (from settings.shortcut_invert_case = {invert_raw:?}; Shift flips to Proper Case)"
        )
    };
    let blank_chord = resolve_chord(&state.settings.shortcut_blank_line, DEFAULT_BLANK_LINE);
    let blank_below = blank_chord.display();
    let blank_above = blank_chord.flipped_shift().display();
    let blank_raw = state.settings.shortcut_blank_line.trim();
    let blank_note = if blank_raw.is_empty() || blank_raw.eq_ignore_ascii_case(DEFAULT_BLANK_LINE) {
        format!("{blank_below} / {blank_above} (default; Preferences → Blank line shortcut)")
    } else {
        format!(
            "{blank_below} / {blank_above} (from settings.shortcut_blank_line = {blank_raw:?}; Shift flips to blank line above)"
        )
    };
    let multi_select_chord = resolve_chord(
        &state.settings.shortcut_multi_select_next,
        DEFAULT_MULTI_SELECT_NEXT,
    );
    let multi_select_next = multi_select_chord.display();
    let multi_select_skip = multi_select_chord.flipped_shift().display();
    let multi_select_raw = state.settings.shortcut_multi_select_next.trim();
    let multi_select_note = if multi_select_raw.is_empty()
        || multi_select_raw.eq_ignore_ascii_case(DEFAULT_MULTI_SELECT_NEXT)
    {
        format!(
            "{multi_select_next} / {multi_select_skip} (default; Preferences → Multi-select next shortcut)"
        )
    } else {
        format!(
            "{multi_select_next} / {multi_select_skip} (from settings.shortcut_multi_select_next = {multi_select_raw:?}; Shift flips to skip current)"
        )
    };
    let multi_select_all_chord = resolve_chord(
        &state.settings.shortcut_multi_select_all,
        DEFAULT_MULTI_SELECT_ALL,
    );
    let multi_select_all = multi_select_all_chord.display();
    let multi_select_all_word = multi_select_all_chord.flipped_shift().display();
    let multi_select_all_raw = state.settings.shortcut_multi_select_all.trim();
    let multi_select_all_note = if multi_select_all_raw.is_empty()
        || multi_select_all_raw.eq_ignore_ascii_case(DEFAULT_MULTI_SELECT_ALL)
    {
        format!(
            "{multi_select_all} / {multi_select_all_word} (default; Preferences → Multi-select all shortcut)"
        )
    } else {
        format!(
            "{multi_select_all} / {multi_select_all_word} (from settings.shortcut_multi_select_all = {multi_select_all_raw:?}; Shift flips to whole-word)"
        )
    };
    let begin_end_select_chord = resolve_chord(
        &state.settings.shortcut_begin_end_select,
        DEFAULT_BEGIN_END_SELECT,
    );
    let begin_end_select = begin_end_select_chord.display();
    let begin_end_select_column = begin_end_select_chord.flipped_shift().display();
    let begin_end_select_raw = state.settings.shortcut_begin_end_select.trim();
    let begin_end_select_note = if begin_end_select_raw.is_empty()
        || begin_end_select_raw.eq_ignore_ascii_case(DEFAULT_BEGIN_END_SELECT)
    {
        format!(
            "{begin_end_select} / {begin_end_select_column} (default; Preferences → Begin/End Select shortcut)"
        )
    } else {
        format!(
            "{begin_end_select} / {begin_end_select_column} (from settings.shortcut_begin_end_select = {begin_end_select_raw:?}; Shift flips to column mode)"
        )
    };
    let toggle_readonly = resolve_chord(
        &state.settings.shortcut_toggle_readonly,
        DEFAULT_TOGGLE_READONLY,
    )
    .display();
    let toggle_readonly_raw = state.settings.shortcut_toggle_readonly.trim();
    let toggle_readonly_note = if toggle_readonly_raw.is_empty()
        || toggle_readonly_raw.eq_ignore_ascii_case(DEFAULT_TOGGLE_READONLY)
    {
        format!("{toggle_readonly} (default; Preferences → Toggle read-only shortcut)")
    } else {
        format!(
            "{toggle_readonly} (from settings.shortcut_toggle_readonly = {toggle_readonly_raw:?})"
        )
    };
    let column_editor = resolve_chord(
        &state.settings.shortcut_column_editor,
        DEFAULT_COLUMN_EDITOR,
    )
    .display();
    let column_editor_raw = state.settings.shortcut_column_editor.trim();
    let column_editor_note = if column_editor_raw.is_empty()
        || column_editor_raw.eq_ignore_ascii_case(DEFAULT_COLUMN_EDITOR)
    {
        format!("{column_editor} (default; Preferences → Column Editor shortcut)")
    } else {
        format!("{column_editor} (from settings.shortcut_column_editor = {column_editor_raw:?})")
    };
    let sort_lines_chord = resolve_chord(&state.settings.shortcut_sort_lines, DEFAULT_SORT_LINES);
    let sort_lines = sort_lines_chord.display();
    let sort_lines_desc = sort_lines_chord.flipped_shift().display();
    let sort_lines_raw = state.settings.shortcut_sort_lines.trim();
    let sort_lines_note = if sort_lines_raw.is_empty()
        || sort_lines_raw.eq_ignore_ascii_case(DEFAULT_SORT_LINES)
    {
        format!(
            "{sort_lines} / {sort_lines_desc} (default; Preferences → Sort Lines shortcut; Shift flips to descending)"
        )
    } else {
        format!(
            "{sort_lines} / {sort_lines_desc} (from settings.shortcut_sort_lines = {sort_lines_raw:?}; Shift flips to descending)"
        )
    };
    let remove_empty_chord = resolve_chord(
        &state.settings.shortcut_remove_empty_lines,
        DEFAULT_REMOVE_EMPTY_LINES,
    );
    let remove_empty = remove_empty_chord.display();
    let remove_empty_blank = remove_empty_chord.flipped_shift().display();
    let remove_empty_raw = state.settings.shortcut_remove_empty_lines.trim();
    let remove_empty_note = if remove_empty_raw.is_empty()
        || remove_empty_raw.eq_ignore_ascii_case(DEFAULT_REMOVE_EMPTY_LINES)
    {
        format!(
            "{remove_empty} / {remove_empty_blank} (default; Preferences → Remove Empty Lines shortcut; Shift flips to containing blank)"
        )
    } else {
        format!(
            "{remove_empty} / {remove_empty_blank} (from settings.shortcut_remove_empty_lines = {remove_empty_raw:?}; Shift flips to containing blank)"
        )
    };
    let remove_dup_chord = resolve_chord(
        &state.settings.shortcut_remove_dup_lines,
        DEFAULT_REMOVE_DUP_LINES,
    );
    let remove_dup = remove_dup_chord.display();
    let remove_dup_any = remove_dup_chord.flipped_shift().display();
    let remove_dup_raw = state.settings.shortcut_remove_dup_lines.trim();
    let remove_dup_note = if remove_dup_raw.is_empty()
        || remove_dup_raw.eq_ignore_ascii_case(DEFAULT_REMOVE_DUP_LINES)
    {
        format!(
            "{remove_dup} / {remove_dup_any} (default; Preferences → Remove Duplicate Lines shortcut; Shift flips to any duplicates)"
        )
    } else {
        format!(
            "{remove_dup} / {remove_dup_any} (from settings.shortcut_remove_dup_lines = {remove_dup_raw:?}; Shift flips to any duplicates)"
        )
    };
    let insert_datetime_chord = resolve_chord(
        &state.settings.shortcut_insert_datetime,
        DEFAULT_INSERT_DATETIME,
    );
    let insert_datetime = insert_datetime_chord.display();
    let insert_datetime_long = insert_datetime_chord.flipped_shift().display();
    let insert_datetime_raw = state.settings.shortcut_insert_datetime.trim();
    let insert_datetime_note = if insert_datetime_raw.is_empty()
        || insert_datetime_raw.eq_ignore_ascii_case(DEFAULT_INSERT_DATETIME)
    {
        format!(
            "{insert_datetime} / {insert_datetime_long} (default; Preferences → Date Time shortcut; Shift flips to long)"
        )
    } else {
        format!(
            "{insert_datetime} / {insert_datetime_long} (from settings.shortcut_insert_datetime = {insert_datetime_raw:?}; Shift flips to long)"
        )
    };
    let copy_path_chord = resolve_chord(&state.settings.shortcut_copy_path, DEFAULT_COPY_PATH);
    let copy_path = copy_path_chord.display();
    let copy_path_name = copy_path_chord.flipped_shift().display();
    let copy_path_raw = state.settings.shortcut_copy_path.trim();
    let copy_path_note = if copy_path_raw.is_empty()
        || copy_path_raw.eq_ignore_ascii_case(DEFAULT_COPY_PATH)
    {
        format!(
            "{copy_path} / {copy_path_name} (default; Preferences → Copy path shortcut; Shift flips to file name)"
        )
    } else {
        format!(
            "{copy_path} / {copy_path_name} (from settings.shortcut_copy_path = {copy_path_raw:?}; Shift flips to file name)"
        )
    };
    let copy_dir = resolve_chord(&state.settings.shortcut_copy_dir, DEFAULT_COPY_DIR).display();
    let copy_dir_raw = state.settings.shortcut_copy_dir.trim();
    let copy_dir_note =
        if copy_dir_raw.is_empty() || copy_dir_raw.eq_ignore_ascii_case(DEFAULT_COPY_DIR) {
            format!("{copy_dir} (default; Preferences → Copy directory shortcut)")
        } else {
            format!("{copy_dir} (from settings.shortcut_copy_dir = {copy_dir_raw:?})")
        };
    let copy_all_paths_chord = resolve_chord(
        &state.settings.shortcut_copy_all_paths,
        DEFAULT_COPY_ALL_PATHS,
    );
    let copy_all_paths = copy_all_paths_chord.display();
    let copy_all_names = copy_all_paths_chord.flipped_shift().display();
    let copy_all_paths_raw = state.settings.shortcut_copy_all_paths.trim();
    let copy_all_paths_note = if copy_all_paths_raw.is_empty()
        || copy_all_paths_raw.eq_ignore_ascii_case(DEFAULT_COPY_ALL_PATHS)
    {
        format!(
            "{copy_all_paths} / {copy_all_names} (default; Preferences → Copy all paths shortcut; Shift flips to all filenames)"
        )
    } else {
        format!(
            "{copy_all_paths} / {copy_all_names} (from settings.shortcut_copy_all_paths = {copy_all_paths_raw:?}; Shift flips to all filenames)"
        )
    };
    let open_selected_chord = resolve_chord(
        &state.settings.shortcut_open_selected,
        DEFAULT_OPEN_SELECTED,
    );
    let open_selected = open_selected_chord.display();
    let open_selected_folder = open_selected_chord.flipped_shift().display();
    let open_selected_raw = state.settings.shortcut_open_selected.trim();
    let open_selected_note = if open_selected_raw.is_empty()
        || open_selected_raw.eq_ignore_ascii_case(DEFAULT_OPEN_SELECTED)
    {
        format!(
            "{open_selected} / {open_selected_folder} (default; Preferences → Open selected shortcut; Shift flips to containing folder)"
        )
    } else {
        format!(
            "{open_selected} / {open_selected_folder} (from settings.shortcut_open_selected = {open_selected_raw:?}; Shift flips to containing folder)"
        )
    };
    let open_folder_chord =
        resolve_chord(&state.settings.shortcut_open_folder, DEFAULT_OPEN_FOLDER);
    let open_folder = open_folder_chord.display();
    let open_folder_shell = open_folder_chord.flipped_shift().display();
    let open_folder_raw = state.settings.shortcut_open_folder.trim();
    let open_folder_note = if open_folder_raw.is_empty()
        || open_folder_raw.eq_ignore_ascii_case(DEFAULT_OPEN_FOLDER)
    {
        format!(
            "{open_folder} / {open_folder_shell} (default; Preferences → Open folder shortcut; Shift flips to shell here)"
        )
    } else {
        format!(
            "{open_folder} / {open_folder_shell} (from settings.shortcut_open_folder = {open_folder_raw:?}; Shift flips to shell here)"
        )
    };
    let open_workspace_chord = resolve_chord(
        &state.settings.shortcut_open_workspace,
        DEFAULT_OPEN_WORKSPACE,
    );
    let open_workspace = open_workspace_chord.display();
    let open_workspace_containing = open_workspace_chord.flipped_shift().display();
    let open_workspace_raw = state.settings.shortcut_open_workspace.trim();
    let open_workspace_note = if open_workspace_raw.is_empty()
        || open_workspace_raw.eq_ignore_ascii_case(DEFAULT_OPEN_WORKSPACE)
    {
        format!(
            "{open_workspace} / {open_workspace_containing} (default; Preferences → Open workspace shortcut; Shift flips to containing folder as workspace)"
        )
    } else {
        format!(
            "{open_workspace} / {open_workspace_containing} (from settings.shortcut_open_workspace = {open_workspace_raw:?}; Shift flips to containing folder as workspace)"
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
    let close_all_chord = resolve_chord(&state.settings.shortcut_close_all, DEFAULT_CLOSE_ALL);
    let close_all = close_all_chord.display();
    let close_all_but = close_all_chord.flipped_shift().display();
    let close_all_raw = state.settings.shortcut_close_all.trim();
    let close_all_note = if close_all_raw.is_empty()
        || close_all_raw.eq_ignore_ascii_case(DEFAULT_CLOSE_ALL)
    {
        format!(
            "{close_all} / {close_all_but} (default; Preferences → Close All shortcut; Shift flips to Close All but Current)"
        )
    } else {
        format!(
            "{close_all} / {close_all_but} (from settings.shortcut_close_all = {close_all_raw:?}; Shift flips)"
        )
    };
    let close_but_pinned_chord = resolve_chord(
        &state.settings.shortcut_close_but_pinned,
        DEFAULT_CLOSE_BUT_PINNED,
    );
    let close_but_pinned = close_but_pinned_chord.display();
    let close_unchanged = close_but_pinned_chord.flipped_shift().display();
    let close_but_pinned_raw = state.settings.shortcut_close_but_pinned.trim();
    let close_but_pinned_note = if close_but_pinned_raw.is_empty()
        || close_but_pinned_raw.eq_ignore_ascii_case(DEFAULT_CLOSE_BUT_PINNED)
    {
        format!(
            "{close_but_pinned} / {close_unchanged} (default; Preferences → Close All but Pinned shortcut; Shift flips to Close All Unchanged)"
        )
    } else {
        format!(
            "{close_but_pinned} / {close_unchanged} (from settings.shortcut_close_but_pinned = {close_but_pinned_raw:?}; Shift flips to Close All Unchanged)"
        )
    };
    let close_to_side_chord = resolve_chord(
        &state.settings.shortcut_close_to_side,
        DEFAULT_CLOSE_TO_SIDE,
    );
    let close_to_left = close_to_side_chord.display();
    let close_to_right = close_to_side_chord.flipped_shift().display();
    let close_to_side_raw = state.settings.shortcut_close_to_side.trim();
    let close_to_side_note = if close_to_side_raw.is_empty()
        || close_to_side_raw.eq_ignore_ascii_case(DEFAULT_CLOSE_TO_SIDE)
    {
        format!(
            "{close_to_left} / {close_to_right} (default; Preferences → Close All to Left shortcut; Shift flips to Close All to the Right)"
        )
    } else {
        format!(
            "{close_to_left} / {close_to_right} (from settings.shortcut_close_to_side = {close_to_side_raw:?}; Shift flips to Close All to the Right)"
        )
    };
    let trim_trailing_chord = resolve_chord(
        &state.settings.shortcut_trim_trailing,
        DEFAULT_TRIM_TRAILING,
    );
    let trim_trailing = trim_trailing_chord.display();
    let trim_both = trim_trailing_chord.flipped_shift().display();
    let trim_trailing_raw = state.settings.shortcut_trim_trailing.trim();
    let trim_trailing_note = if trim_trailing_raw.is_empty()
        || trim_trailing_raw.eq_ignore_ascii_case(DEFAULT_TRIM_TRAILING)
    {
        format!(
            "{trim_trailing} / {trim_both} (default; Preferences → Trim Trailing shortcut; Shift flips to Trim Leading and Trailing)"
        )
    } else {
        format!(
            "{trim_trailing} / {trim_both} (from settings.shortcut_trim_trailing = {trim_trailing_raw:?}; Shift flips)"
        )
    };
    let tab_to_space_chord =
        resolve_chord(&state.settings.shortcut_tab_to_space, DEFAULT_TAB_TO_SPACE);
    let tab_to_space = tab_to_space_chord.display();
    let space_to_tab = tab_to_space_chord.flipped_shift().display();
    let tab_to_space_raw = state.settings.shortcut_tab_to_space.trim();
    let tab_to_space_note = if tab_to_space_raw.is_empty()
        || tab_to_space_raw.eq_ignore_ascii_case(DEFAULT_TAB_TO_SPACE)
    {
        format!(
            "{tab_to_space} / {space_to_tab} (default; Preferences → TAB to Space shortcut; Shift flips to Space to TAB Leading)"
        )
    } else {
        format!(
            "{tab_to_space} / {space_to_tab} (from settings.shortcut_tab_to_space = {tab_to_space_raw:?}; Shift flips to Space to TAB Leading)"
        )
    };
    let space_to_tab_all = resolve_chord(
        &state.settings.shortcut_space_to_tab_all,
        DEFAULT_SPACE_TO_TAB_ALL,
    )
    .display();
    let space_to_tab_all_raw = state.settings.shortcut_space_to_tab_all.trim();
    let space_to_tab_all_note = if space_to_tab_all_raw.is_empty()
        || space_to_tab_all_raw.eq_ignore_ascii_case(DEFAULT_SPACE_TO_TAB_ALL)
    {
        format!("{space_to_tab_all} (default; Preferences → Space to TAB (All) shortcut)")
    } else {
        format!(
            "{space_to_tab_all} (from settings.shortcut_space_to_tab_all = {space_to_tab_all_raw:?})"
        )
    };
    let pin_tab = resolve_chord(&state.settings.shortcut_pin_tab, DEFAULT_PIN_TAB).display();
    let pin_tab_raw = state.settings.shortcut_pin_tab.trim();
    let pin_tab_note =
        if pin_tab_raw.is_empty() || pin_tab_raw.eq_ignore_ascii_case(DEFAULT_PIN_TAB) {
            format!("{pin_tab} (default; Preferences → Pin tab shortcut)")
        } else {
            format!("{pin_tab} (from settings.shortcut_pin_tab = {pin_tab_raw:?})")
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
    let first_tab_chord = resolve_chord(&state.settings.shortcut_first_tab, DEFAULT_FIRST_TAB);
    let first_tab = first_tab_chord.display();
    let last_tab = first_tab_chord.line_end_chord().display();
    let first_tab_raw = state.settings.shortcut_first_tab.trim();
    let first_tab_note = if first_tab_raw.is_empty()
        || first_tab_raw.eq_ignore_ascii_case(DEFAULT_FIRST_TAB)
    {
        format!(
            "{first_tab} / {last_tab} (default; Preferences → First tab shortcut; opposite End = last)"
        )
    } else {
        format!(
            "{first_tab} / {last_tab} (from settings.shortcut_first_tab = {first_tab_raw:?}; opposite End or Shift = last)"
        )
    };
    let move_tab_chord = resolve_chord(&state.settings.shortcut_move_tab, DEFAULT_MOVE_TAB);
    let move_tab_fwd = move_tab_chord.display();
    let move_tab_back = move_tab_chord.page_down_chord().display();
    let move_tab_raw = state.settings.shortcut_move_tab.trim();
    let move_tab_note = if move_tab_raw.is_empty()
        || move_tab_raw.eq_ignore_ascii_case(DEFAULT_MOVE_TAB)
    {
        format!(
            "{move_tab_fwd} / {move_tab_back} (default; Preferences → Move tab shortcut; opposite PageUp moves back)"
        )
    } else {
        format!(
            "{move_tab_fwd} / {move_tab_back} (from settings.shortcut_move_tab = {move_tab_raw:?}; opposite PageUp or Shift moves back)"
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
    let save_copy_chord = resolve_chord(&state.settings.shortcut_save_copy, DEFAULT_SAVE_COPY);
    let save_copy = save_copy_chord.display();
    let rename = save_copy_chord.flipped_shift().display();
    let save_copy_raw = state.settings.shortcut_save_copy.trim();
    let save_copy_note = if save_copy_raw.is_empty()
        || save_copy_raw.eq_ignore_ascii_case(DEFAULT_SAVE_COPY)
    {
        format!(
            "{save_copy} / {rename} (default; Preferences → Save a Copy As shortcut; Shift flips to Rename)"
        )
    } else {
        format!(
            "{save_copy} / {rename} (from settings.shortcut_save_copy = {save_copy_raw:?}; Shift flips to Rename)"
        )
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
    let find_in_files = resolve_chord(
        &state.settings.shortcut_find_in_files,
        DEFAULT_FIND_IN_FILES,
    )
    .display();
    let find_in_files_raw = state.settings.shortcut_find_in_files.trim();
    let find_in_files_note = if find_in_files_raw.is_empty()
        || find_in_files_raw.eq_ignore_ascii_case(DEFAULT_FIND_IN_FILES)
    {
        format!("{find_in_files} (default; Preferences → Find in Files shortcut)")
    } else {
        format!("{find_in_files} (from settings.shortcut_find_in_files = {find_in_files_raw:?})")
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
    let copy = resolve_chord(&state.settings.shortcut_copy, DEFAULT_COPY).display();
    let copy_raw = state.settings.shortcut_copy.trim();
    let copy_note = if copy_raw.is_empty() || copy_raw.eq_ignore_ascii_case(DEFAULT_COPY) {
        format!("{copy} (default; Preferences → Copy shortcut)")
    } else {
        format!("{copy} (from settings.shortcut_copy = {copy_raw:?})")
    };
    let cut = resolve_chord(&state.settings.shortcut_cut, DEFAULT_CUT).display();
    let cut_raw = state.settings.shortcut_cut.trim();
    let cut_note = if cut_raw.is_empty() || cut_raw.eq_ignore_ascii_case(DEFAULT_CUT) {
        format!("{cut} (default; Preferences → Cut shortcut)")
    } else {
        format!("{cut} (from settings.shortcut_cut = {cut_raw:?})")
    };
    let paste = resolve_chord(&state.settings.shortcut_paste, DEFAULT_PASTE).display();
    let paste_raw = state.settings.shortcut_paste.trim();
    let paste_note = if paste_raw.is_empty() || paste_raw.eq_ignore_ascii_case(DEFAULT_PASTE) {
        format!("{paste} (default; Preferences → Paste shortcut)")
    } else {
        format!("{paste} (from settings.shortcut_paste = {paste_raw:?})")
    };
    let delete_forward = resolve_chord(
        &state.settings.shortcut_delete_forward,
        DEFAULT_DELETE_FORWARD,
    )
    .display();
    let delete_forward_raw = state.settings.shortcut_delete_forward.trim();
    let delete_forward_note = if delete_forward_raw.is_empty()
        || delete_forward_raw.eq_ignore_ascii_case(DEFAULT_DELETE_FORWARD)
    {
        format!("{delete_forward} (default; Preferences → Delete shortcut)")
    } else {
        format!("{delete_forward} (from settings.shortcut_delete_forward = {delete_forward_raw:?})")
    };
    let delete_backward = resolve_chord(
        &state.settings.shortcut_delete_backward,
        DEFAULT_DELETE_BACKWARD,
    )
    .display();
    let delete_backward_raw = state.settings.shortcut_delete_backward.trim();
    let delete_backward_note = if delete_backward_raw.is_empty()
        || delete_backward_raw.eq_ignore_ascii_case(DEFAULT_DELETE_BACKWARD)
    {
        format!("{delete_backward} (default; Preferences → Backspace shortcut)")
    } else {
        format!(
            "{delete_backward} (from settings.shortcut_delete_backward = {delete_backward_raw:?})"
        )
    };
    let enter = resolve_chord(&state.settings.shortcut_enter, DEFAULT_ENTER).display();
    let enter_raw = state.settings.shortcut_enter.trim();
    let enter_note = if enter_raw.is_empty() || enter_raw.eq_ignore_ascii_case(DEFAULT_ENTER) {
        format!(
            "{enter} (default; Preferences → Enter shortcut; blank line Cmd+Enter stays separate)"
        )
    } else {
        format!("{enter} (from settings.shortcut_enter = {enter_raw:?}; blank line stays separate)")
    };
    let line_home_chord = resolve_chord(&state.settings.shortcut_line_home, DEFAULT_LINE_HOME);
    let line_home = line_home_chord.display();
    let line_end = line_home_chord.line_end_chord().display();
    let line_home_raw = state.settings.shortcut_line_home.trim();
    let line_home_note = if line_home_raw.is_empty()
        || line_home_raw.eq_ignore_ascii_case(DEFAULT_LINE_HOME)
    {
        format!(
            "{line_home} / {line_end} (default; Preferences → Line start shortcut; opposite End = line end; Cmd+Home / Cmd+End aliases use Document Home shortcut)"
        )
    } else {
        format!(
            "{line_home} / {line_end} (from settings.shortcut_line_home = {line_home_raw:?}; opposite End or Shift = line end; Cmd+Home / Cmd+End aliases use Document Home shortcut)"
        )
    };
    let document_home_chord = resolve_chord(
        &state.settings.shortcut_document_home,
        DEFAULT_DOCUMENT_HOME,
    );
    let document_home = document_home_chord.display();
    let document_end = document_home_chord.document_end_chord().display();
    let document_home_raw = state.settings.shortcut_document_home.trim();
    let document_home_note = if document_home_raw.is_empty()
        || document_home_raw.eq_ignore_ascii_case(DEFAULT_DOCUMENT_HOME)
    {
        format!(
            "{document_home} / {document_end} (default; Preferences → Document Home shortcut; opposite End = document end)"
        )
    } else {
        format!(
            "{document_home} / {document_end} (from settings.shortcut_document_home = {document_home_raw:?}; opposite End or Shift = document end)"
        )
    };
    let page_up_chord = resolve_chord(&state.settings.shortcut_page_up, DEFAULT_PAGE_UP);
    let page_up = page_up_chord.display();
    let page_down = page_up_chord.page_down_chord().display();
    let page_up_raw = state.settings.shortcut_page_up.trim();
    let page_up_note = if page_up_raw.is_empty()
        || page_up_raw.eq_ignore_ascii_case(DEFAULT_PAGE_UP)
    {
        format!(
            "{page_up} / {page_down} (default; Preferences → Page up shortcut; opposite PageDown = page down)"
        )
    } else {
        format!(
            "{page_up} / {page_down} (from settings.shortcut_page_up = {page_up_raw:?}; opposite PageDown or Shift = page down)"
        )
    };
    let char_left_chord = resolve_chord(&state.settings.shortcut_char_left, DEFAULT_CHAR_LEFT);
    let char_left = char_left_chord.display();
    let char_right = char_left_chord.char_right_chord().display();
    let char_left_raw = state.settings.shortcut_char_left.trim();
    let char_left_note = if char_left_raw.is_empty()
        || char_left_raw.eq_ignore_ascii_case(DEFAULT_CHAR_LEFT)
    {
        format!(
            "{char_left} / {char_right} (default; Preferences → Character left shortcut; opposite Right = char right)"
        )
    } else {
        format!(
            "{char_left} / {char_right} (from settings.shortcut_char_left = {char_left_raw:?}; opposite Right or Shift = char right)"
        )
    };
    let line_up_chord = resolve_chord(&state.settings.shortcut_line_up, DEFAULT_LINE_UP);
    let line_up = line_up_chord.display();
    let line_down = line_up_chord.line_down_chord().display();
    let line_up_raw = state.settings.shortcut_line_up.trim();
    let line_up_note = if line_up_raw.is_empty()
        || line_up_raw.eq_ignore_ascii_case(DEFAULT_LINE_UP)
    {
        format!(
            "{line_up} / {line_down} (default; Preferences → Line up shortcut; opposite Down = line down)"
        )
    } else {
        format!(
            "{line_up} / {line_down} (from settings.shortcut_line_up = {line_up_raw:?}; opposite Down or Shift = line down)"
        )
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
    let document_map =
        resolve_chord(&state.settings.shortcut_document_map, DEFAULT_DOCUMENT_MAP).display();
    let document_map_raw = state.settings.shortcut_document_map.trim();
    let document_map_note = if document_map_raw.is_empty()
        || document_map_raw.eq_ignore_ascii_case(DEFAULT_DOCUMENT_MAP)
    {
        format!("{document_map} (default; Preferences → Document Map shortcut)")
    } else {
        format!("{document_map} (from settings.shortcut_document_map = {document_map_raw:?})")
    };
    let function_list_chord = resolve_chord(
        &state.settings.shortcut_function_list,
        DEFAULT_FUNCTION_LIST,
    );
    let function_list = function_list_chord.display();
    let char_panel = function_list_chord.flipped_shift().display();
    let function_list_raw = state.settings.shortcut_function_list.trim();
    let function_list_note = if function_list_raw.is_empty()
        || function_list_raw.eq_ignore_ascii_case(DEFAULT_FUNCTION_LIST)
    {
        format!(
            "{function_list} / {char_panel} (default; Preferences → Function List shortcut; Shift flips to Character Panel)"
        )
    } else {
        format!(
            "{function_list} / {char_panel} (from settings.shortcut_function_list = {function_list_raw:?}; Shift flips to Character Panel)"
        )
    };
    let focus_other_chord = resolve_chord(
        &state.settings.shortcut_focus_other_view,
        DEFAULT_FOCUS_OTHER_VIEW,
    );
    let focus_other = focus_other_chord.display();
    let focus_other_swap = focus_other_chord.flipped_shift().display();
    let focus_other_raw = state.settings.shortcut_focus_other_view.trim();
    let focus_other_note = if focus_other_raw.is_empty()
        || focus_other_raw.eq_ignore_ascii_case(DEFAULT_FOCUS_OTHER_VIEW)
    {
        format!(
            "{focus_other} / {focus_other_swap} (default; Preferences → Focus other view shortcut; Shift swaps tabs)"
        )
    } else {
        format!(
            "{focus_other} / {focus_other_swap} (from settings.shortcut_focus_other_view = {focus_other_raw:?}; Shift swaps tabs)"
        )
    };
    let move_other_chord = resolve_chord(
        &state.settings.shortcut_move_other_view,
        DEFAULT_MOVE_OTHER_VIEW,
    );
    let move_other = move_other_chord.display();
    let move_other_clone = move_other_chord.flipped_shift().display();
    let move_other_raw = state.settings.shortcut_move_other_view.trim();
    let move_other_note = if move_other_raw.is_empty()
        || move_other_raw.eq_ignore_ascii_case(DEFAULT_MOVE_OTHER_VIEW)
    {
        format!(
            "{move_other} / {move_other_clone} (default; Preferences → Move / clone other view shortcut; Shift clones)"
        )
    } else {
        format!(
            "{move_other} / {move_other_clone} (from settings.shortcut_move_other_view = {move_other_raw:?}; Shift clones)"
        )
    };
    let rtl_chord = resolve_chord(&state.settings.shortcut_rtl, DEFAULT_RTL);
    let rtl = rtl_chord.display();
    let rtl_ltr = rtl_chord.flipped_shift().display();
    let rtl_raw = state.settings.shortcut_rtl.trim();
    let rtl_note = if rtl_raw.is_empty() || rtl_raw.eq_ignore_ascii_case(DEFAULT_RTL) {
        format!("{rtl} / {rtl_ltr} (default; Preferences → RTL / LTR shortcut; Shift = LTR)")
    } else {
        format!("{rtl} / {rtl_ltr} (from settings.shortcut_rtl = {rtl_raw:?}; Shift = LTR)")
    };
    let fullscreen =
        resolve_chord(&state.settings.shortcut_fullscreen, DEFAULT_FULLSCREEN).display();
    let fullscreen_raw = state.settings.shortcut_fullscreen.trim();
    let fullscreen_note =
        if fullscreen_raw.is_empty() || fullscreen_raw.eq_ignore_ascii_case(DEFAULT_FULLSCREEN) {
            format!("{fullscreen} (default; Preferences → Fullscreen shortcut)")
        } else {
            format!("{fullscreen} (from settings.shortcut_fullscreen = {fullscreen_raw:?})")
        };
    let postit = resolve_chord(&state.settings.shortcut_postit, DEFAULT_POSTIT).display();
    let postit_raw = state.settings.shortcut_postit.trim();
    let postit_note = if postit_raw.is_empty() || postit_raw.eq_ignore_ascii_case(DEFAULT_POSTIT) {
        format!("{postit} (default; Preferences → Post-It shortcut)")
    } else {
        format!("{postit} (from settings.shortcut_postit = {postit_raw:?})")
    };
    let always_on_top = resolve_chord(
        &state.settings.shortcut_always_on_top,
        DEFAULT_ALWAYS_ON_TOP,
    )
    .display();
    let always_on_top_raw = state.settings.shortcut_always_on_top.trim();
    let always_on_top_note = if always_on_top_raw.is_empty()
        || always_on_top_raw.eq_ignore_ascii_case(DEFAULT_ALWAYS_ON_TOP)
    {
        format!("{always_on_top} (default; Preferences → Always on top shortcut)")
    } else {
        format!("{always_on_top} (from settings.shortcut_always_on_top = {always_on_top_raw:?})")
    };
    let show_all_characters = resolve_chord(
        &state.settings.shortcut_show_all_characters,
        DEFAULT_SHOW_ALL_CHARACTERS,
    )
    .display();
    let show_all_characters_raw = state.settings.shortcut_show_all_characters.trim();
    let show_all_characters_note = if show_all_characters_raw.is_empty()
        || show_all_characters_raw.eq_ignore_ascii_case(DEFAULT_SHOW_ALL_CHARACTERS)
    {
        format!("{show_all_characters} (default; Preferences → Show all characters shortcut)")
    } else {
        format!(
            "{show_all_characters} (from settings.shortcut_show_all_characters = {show_all_characters_raw:?})"
        )
    };
    let show_ws_chord = resolve_chord(
        &state.settings.shortcut_show_whitespace,
        DEFAULT_SHOW_WHITESPACE,
    );
    let show_whitespace = show_ws_chord.display();
    let show_eol = show_ws_chord.flipped_shift().display();
    let show_whitespace_raw = state.settings.shortcut_show_whitespace.trim();
    let show_whitespace_note = if show_whitespace_raw.is_empty()
        || show_whitespace_raw.eq_ignore_ascii_case(DEFAULT_SHOW_WHITESPACE)
    {
        format!(
            "{show_whitespace} / {show_eol} (default; Preferences → Show whitespace shortcut; Shift flips to Show EOL)"
        )
    } else {
        format!(
            "{show_whitespace} / {show_eol} (from settings.shortcut_show_whitespace = {show_whitespace_raw:?}; Shift flips to Show EOL)"
        )
    };
    let show_npc = resolve_chord(&state.settings.shortcut_show_npc, DEFAULT_SHOW_NPC).display();
    let show_npc_raw = state.settings.shortcut_show_npc.trim();
    let show_npc_note =
        if show_npc_raw.is_empty() || show_npc_raw.eq_ignore_ascii_case(DEFAULT_SHOW_NPC) {
            format!("{show_npc} (default; Preferences → Show NPC shortcut)")
        } else {
            format!("{show_npc} (from settings.shortcut_show_npc = {show_npc_raw:?})")
        };
    let sync_scroll_chord =
        resolve_chord(&state.settings.shortcut_sync_scroll, DEFAULT_SYNC_SCROLL);
    let sync_v = sync_scroll_chord.display();
    let sync_h = sync_scroll_chord.flipped_shift().display();
    let sync_scroll_raw = state.settings.shortcut_sync_scroll.trim();
    let sync_scroll_note = if sync_scroll_raw.is_empty()
        || sync_scroll_raw.eq_ignore_ascii_case(DEFAULT_SYNC_SCROLL)
    {
        format!(
            "{sync_v} / {sync_h} (default; Preferences → Sync scroll shortcut; Shift flips to Sync H)"
        )
    } else {
        format!(
            "{sync_v} / {sync_h} (from settings.shortcut_sync_scroll = {sync_scroll_raw:?}; Shift flips to Sync H)"
        )
    };
    let zoom_sync = resolve_chord(&state.settings.shortcut_zoom_sync, DEFAULT_ZOOM_SYNC).display();
    let zoom_sync_raw = state.settings.shortcut_zoom_sync.trim();
    let zoom_sync_note =
        if zoom_sync_raw.is_empty() || zoom_sync_raw.eq_ignore_ascii_case(DEFAULT_ZOOM_SYNC) {
            format!("{zoom_sync} (default; Preferences → Zoom Sync shortcut)")
        } else {
            format!("{zoom_sync} (from settings.shortcut_zoom_sync = {zoom_sync_raw:?})")
        };
    let indent_guide =
        resolve_chord(&state.settings.shortcut_indent_guide, DEFAULT_INDENT_GUIDE).display();
    let indent_guide_raw = state.settings.shortcut_indent_guide.trim();
    let indent_guide_note = if indent_guide_raw.is_empty()
        || indent_guide_raw.eq_ignore_ascii_case(DEFAULT_INDENT_GUIDE)
    {
        format!("{indent_guide} (default; Preferences → Indent guide shortcut)")
    } else {
        format!("{indent_guide} (from settings.shortcut_indent_guide = {indent_guide_raw:?})")
    };
    let show_line_numbers = resolve_chord(
        &state.settings.shortcut_show_line_numbers,
        DEFAULT_SHOW_LINE_NUMBERS,
    )
    .display();
    let show_line_numbers_raw = state.settings.shortcut_show_line_numbers.trim();
    let show_line_numbers_note = if show_line_numbers_raw.is_empty()
        || show_line_numbers_raw.eq_ignore_ascii_case(DEFAULT_SHOW_LINE_NUMBERS)
    {
        format!("{show_line_numbers} (default; Preferences → Show line numbers shortcut)")
    } else {
        format!(
            "{show_line_numbers} (from settings.shortcut_show_line_numbers = {show_line_numbers_raw:?})"
        )
    };
    let show_fold_margin = resolve_chord(
        &state.settings.shortcut_show_fold_margin,
        DEFAULT_SHOW_FOLD_MARGIN,
    )
    .display();
    let show_fold_margin_raw = state.settings.shortcut_show_fold_margin.trim();
    let show_fold_margin_note = if show_fold_margin_raw.is_empty()
        || show_fold_margin_raw.eq_ignore_ascii_case(DEFAULT_SHOW_FOLD_MARGIN)
    {
        format!("{show_fold_margin} (default; Preferences → Show fold margin shortcut)")
    } else {
        format!(
            "{show_fold_margin} (from settings.shortcut_show_fold_margin = {show_fold_margin_raw:?})"
        )
    };
    let project_panel = resolve_chord(
        &state.settings.shortcut_project_panel,
        DEFAULT_PROJECT_PANEL,
    )
    .display();
    let project_panel_raw = state.settings.shortcut_project_panel.trim();
    let project_panel_note = if project_panel_raw.is_empty()
        || project_panel_raw.eq_ignore_ascii_case(DEFAULT_PROJECT_PANEL)
    {
        format!("{project_panel} (default; Preferences → Project panel shortcut)")
    } else {
        format!("{project_panel} (from settings.shortcut_project_panel = {project_panel_raw:?})")
    };
    // Mirrors crates/app/src/ui.rs handle_shortcuts (+ remappable bindings).
    let text = format!(
        "\
npp-rs keyboard shortcuts
=========================
Source: ui.rs handle_shortcuts. Most keys are hard-wired.
Word wrap, Find next, Find next (global), Select and Find next / prev, Next / prev search result, Find All / Find All in opened, Incremental Search, Find Mark, Find Bookmark, Find Copy matches / matching lines, Find Cut matches / matching lines, Find Purge matching lines / matches, Find characters in range next / prev, Find Next / Previous (Volatile), Mark Style 1 / Clear Style 1, Mark Style 2 / Clear Style 2, Mark Style 3 / Clear Style 3, Mark Style 4 / Clear Style 4, Mark Style 5 / Clear Style 5, Clear all styles, Next / prev style marker, Copy Styled Text 1…5 / All Styles, Mark current line Style 1…5, Next bookmark, Toggle bookmark, Clear all bookmarks, Inverse bookmarks, Copy / Cut Bookmarked Lines, Paste / Remove Bookmarked Lines, Remove Unmarked Lines, Next change history, Clear change history, Compare start/clear, Swap Compare sides, Compare to Saved, Copy / Open Compare Diff, Copy / Open Compare Summary, Copy / Open Compare Hunk, Word jump, Rect column select, Next difference, First difference, Next hidden equal, First hidden equal, Apply compare hunk, Hide unchanged lines, Ignore whitespace / Ignore case, Ignore blank lines, Expand / Collapse all unchanged, Expand / Collapse unchanged at caret, Bookmark / Clear compare difference bookmarks, Hide-equal context, Go to line, Duplicate line, Delete line, Move line, Go to document start/end, Document Home / End aliases, Toggle comment, lowercase / UPPERCASE, Autocomplete / Function Call Tip, Path autocomplete, Join Lines / Split Lines, Invert Case / Proper Case, Blank line below / above, Multi-select next / skip current, Multi-select all / all whole word, Begin/End Select / column mode, Toggle read-only, Column Editor, Sort Lines Lexicographic Asc / Desc, Remove Empty Lines / Containing Blank, Remove Consecutive / Any Duplicate Lines, Date Time short / long, Copy Full Path / File Name, Copy Current Directory Path, Copy All Paths / All Filenames, Open Selected Filename / Containing Folder, Open Containing Folder / Shell, Open Folder as Workspace / Containing Folder, Indent, Outdent, Tab indent, Shift+Tab outdent, Fold / Unfold all, Fold / Unfold level 1…8, Fold / Unfold current, Matching brace, Format document, Close tab, Close All / Close All but Current, Close All but Pinned / Close All Unchanged, Close All to the Left / Close All to the Right, Trim Trailing / Trim Both, TAB to Space / Space to TAB (Leading), Space to TAB (All), Pin tab, Next / prev tab, Move tab forward / backward, New, Open, Reload, Save, Save As, Save a Copy As / Rename, Save All, Print, Find, Find in Files, Close Find/Replace, Replace, Replace (alternate), Select all, Copy, Cut, Paste, Delete, Backspace, Enter / newline, Line start / end, Page up / down, Character left / right, Line up / down, Undo, Redo, Zoom in, Zoom out, Zoom restore, Toggle log tail, Document Map, Function List / Character Panel, Fullscreen, Always on Top, Show All Characters, Show White Space / Show EOL, Show NPC, Sync Vertical / Horizontal Scroll, Zoom Sync, Indent Guide, Show Line Numbers, Show Fold Margin, Project Panel, Post-It, Focus other view, Move / clone to other view, and RTL / LTR are remappable via Preferences or npp-rs/settings.json
(keys shortcut_word_wrap, shortcut_find_next, shortcut_find_next_global, shortcut_set_and_find, shortcut_next_found, shortcut_find_all, shortcut_find_incremental, shortcut_find_mark, shortcut_find_bookmark, shortcut_find_copy, shortcut_find_cut, shortcut_find_purge, shortcut_find_char_range, shortcut_find_volatile, shortcut_mark_style, shortcut_mark_style_2, shortcut_mark_style_3, shortcut_mark_style_4, shortcut_mark_style_5, shortcut_clear_all_styles, shortcut_next_style, shortcut_style_to_clip, shortcut_mark_one_style, shortcut_next_bookmark, shortcut_toggle_bookmark, shortcut_clear_bookmarks, shortcut_inverse_bookmarks, shortcut_copy_marked_lines, shortcut_paste_marked_lines, shortcut_remove_unmarked, shortcut_next_changed, shortcut_clear_change_history, shortcut_compare, shortcut_swap_compare, shortcut_compare_to_saved, shortcut_copy_compare_diff, shortcut_copy_compare_summary, shortcut_copy_compare_hunk, shortcut_word_jump, shortcut_rect_select, shortcut_next_diff, shortcut_first_diff, shortcut_next_hidden_equal, shortcut_first_hidden_equal, shortcut_apply_compare_hunk, shortcut_hide_equal, shortcut_compare_ignore_ws, shortcut_compare_ignore_blank, shortcut_expand_all_unchanged, shortcut_expand_unchanged_at_caret, shortcut_bookmark_compare_diffs, shortcut_hide_equal_context, shortcut_goto_line, shortcut_duplicate_line, shortcut_delete_line, shortcut_move_line, shortcut_goto_document_start, shortcut_document_home, shortcut_toggle_comment, shortcut_lowercase, shortcut_autocomplete, shortcut_autocomplete_path, shortcut_join_lines, shortcut_invert_case, shortcut_blank_line, shortcut_multi_select_next, shortcut_multi_select_all, shortcut_begin_end_select, shortcut_toggle_readonly, shortcut_column_editor, shortcut_sort_lines, shortcut_remove_empty_lines, shortcut_remove_dup_lines, shortcut_insert_datetime, shortcut_copy_path, shortcut_copy_dir, shortcut_copy_all_paths, shortcut_open_selected, shortcut_open_folder, shortcut_open_workspace, shortcut_indent, shortcut_outdent, shortcut_tab_indent, shortcut_shift_tab_outdent, shortcut_fold_all, shortcut_fold_level, shortcut_fold_current, shortcut_matching_brace, shortcut_format_document, shortcut_close_tab, shortcut_close_all, shortcut_close_but_pinned, shortcut_close_to_side, shortcut_trim_trailing, shortcut_tab_to_space, shortcut_space_to_tab_all, shortcut_pin_tab, shortcut_next_tab, shortcut_first_tab, shortcut_move_tab, shortcut_new, shortcut_open, shortcut_reload, shortcut_save, shortcut_save_as, shortcut_save_copy, shortcut_save_all, shortcut_print, shortcut_find, shortcut_find_in_files, shortcut_close_find, shortcut_replace, shortcut_replace_alt, shortcut_select_all, shortcut_copy, shortcut_cut, shortcut_paste, shortcut_delete_forward, shortcut_delete_backward, shortcut_enter, shortcut_line_home, shortcut_page_up, shortcut_char_left, shortcut_line_up, shortcut_undo, shortcut_redo, shortcut_zoom_in, shortcut_zoom_out, shortcut_zoom_restore, shortcut_toggle_log_tail, shortcut_document_map, shortcut_function_list, shortcut_fullscreen, shortcut_postit, shortcut_always_on_top, shortcut_show_all_characters, shortcut_show_whitespace, shortcut_show_npc, shortcut_sync_scroll, shortcut_zoom_sync, shortcut_indent_guide, shortcut_show_line_numbers, shortcut_show_fold_margin, shortcut_project_panel, shortcut_focus_other_view, shortcut_move_other_view, shortcut_rtl). Full shortcuts.xml remap is not wired yet.
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
{save_copy_note}
{save_all_note}
{print_note}
{close_note}
{close_all_note}
{close_but_pinned_note}
{close_to_side_note}
{trim_trailing_note}
{tab_to_space_note}
{space_to_tab_all_note}
{pin_tab_note}
{next_tab_note}
{first_tab_note}
{move_tab_note}
{undo_note}
{redo_note}
{select_all_note}
{copy_note}
{cut_note}
{paste_note}
{delete_forward_note}
{delete_backward_note}
{enter_note}
{line_home_note}
{document_home_note}
{page_up_note}
{char_left_note}
{line_up_note}
{word_jump_note}
{rect_select_note}
{dup_note}
{del_note}
{move_line_note}
{goto_doc_note}
{toggle_comment_note}
{case_note}
{autocomplete_note}
{path_note}
{join_note}
{invert_note}
{blank_note}
{multi_select_note}
{multi_select_all_note}
{begin_end_select_note}
{toggle_readonly_note}
{column_editor_note}
{sort_lines_note}
{remove_empty_note}
{remove_dup_note}
{insert_datetime_note}
{copy_path_note}
{copy_dir_note}
{copy_all_paths_note}
{open_selected_note}
{open_folder_note}
{open_workspace_note}
{indent_note}
{outdent_note}
{tab_indent_note}
{shift_tab_outdent_note}
{format_note}
{fold_all_note}
{fold_level_note}
{fold_current_note}

Find / navigate
---------------
{find_open_note}
{find_in_files_note}
{close_find_note}
{replace_note}
{replace_alt_note}
{find_note}
{find_global_note}
{set_and_find_note}
{next_found_note}
{find_all_note}
{find_incremental_note}
{find_mark_note}
{find_bookmark_note}
{find_copy_note}
{find_cut_note}
{find_purge_note}
{find_char_range_note}
{find_volatile_note}
{mark_style_note}
{mark_style_2_note}
{mark_style_3_note}
{mark_style_4_note}
{mark_style_5_note}
{clear_all_styles_note}
{next_style_note}
{style_to_clip_note}
{mark_one_style_note}
{goto_note}
{matching_brace_note}
{bm_note}
{toggle_bm_note}
{clear_bm_note}
{inverse_bm_note}
{copy_marked_note}
{paste_marked_note}
{remove_unmarked_note}
{changed_note}
{clear_change_history_note}
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
{document_map_note}
{function_list_note}
{fullscreen_note}
{postit_note}
{always_on_top_note}
{show_all_characters_note}
{show_whitespace_note}
{show_npc_note}
{sync_scroll_note}
{zoom_sync_note}
{indent_guide_note}
{show_line_numbers_note}
{show_fold_margin_note}
{project_panel_note}
{focus_other_note}
{move_other_note}
{rtl_note}

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
