//! Command dispatch for Notepad++ menu IDs (`IDM_*`).

use crate::editor::EditorState;
use std::path::{Path, PathBuf};

/// UI-side flags/commands the menu dispatcher may set.
#[derive(Debug, Default)]
pub struct UiFlags {
    pub show_about: bool,
    pub find_open: bool,
    pub show_replace: bool,
    pub find_focus_once: bool,
    pub follow_caret: bool,
    pub request_quit: bool,
    pub pending_clipboard: Option<String>,
    pub highlight_dirty_scroll_reset: bool,
    /// When set, show the friendly “not ready yet” window.
    pub coming_soon: Option<ComingSoon>,
    /// Zoom delta: -1 / +1 / 0 means restore.
    pub zoom_delta: Option<i8>,
    /// Open Go to Line dialog.
    pub show_goto_line: bool,
    pub always_on_top: Option<bool>,
    /// Toggle OS fullscreen.
    pub fullscreen_toggle: bool,
}

/// Content for the “working on it — come back tomorrow” dialog.
#[derive(Debug, Clone)]
pub struct ComingSoon {
    pub cmd: String,
    pub feature: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmdResult {
    Handled,
    Stub,
}

/// True when `dispatch` would return [`CmdResult::Handled`] (no side effects).
pub fn is_implemented(cmd: &str) -> bool {
    matches!(
        cmd,
        "IDM_FILE_NEW"
            | "IDM_FILE_OPEN"
            | "IDM_FILE_SAVE"
            | "IDM_FILE_SAVEAS"
            | "IDM_FILE_SAVECOPYAS"
            | "IDM_FILE_SAVEALL"
            | "IDM_FILE_RENAME"
            | "IDM_FILE_CLOSE"
            | "IDM_FILE_CLOSEALL"
            | "IDM_FILE_CLOSEALL_BUT_CURRENT"
            | "IDM_FILE_CLOSEALL_TOLEFT"
            | "IDM_FILE_CLOSEALL_TORIGHT"
            | "IDM_FILE_CLOSEALL_UNCHANGED"
            | "IDM_FILE_EXIT"
            | "IDM_FILE_RELOAD"
            | "IDM_FILE_OPEN_FOLDER"
            | "IDM_FILE_OPENFOLDERASWORKSPACE"
            | "IDM_FILE_CONTAININGFOLDERASWORKSPACE"
            | "IDM_FILE_OPEN_CMD"
            | "IDM_FILE_OPEN_POWERSHELL"
            | "IDM_FILE_OPEN_DEFAULT_VIEWER"
            | "IDM_EDIT_UNDO"
            | "IDM_EDIT_REDO"
            | "IDM_EDIT_CUT"
            | "IDM_EDIT_COPY"
            | "IDM_EDIT_PASTE"
            | "IDM_EDIT_DELETE"
            | "IDM_EDIT_SELECTALL"
            | "IDM_EDIT_INS_TAB"
            | "IDM_EDIT_RMV_TAB"
            | "IDM_EDIT_UPPERCASE"
            | "IDM_EDIT_LOWERCASE"
            | "IDM_EDIT_INVERTCASE"
            | "IDM_EDIT_PROPERCASE_FORCE"
            | "IDM_EDIT_PROPERCASE_BLEND"
            | "IDM_EDIT_DUP_LINE"
            | "IDM_EDIT_TRIMTRAILING"
            | "IDM_EDIT_TRIMLINEHEAD"
            | "IDM_EDIT_TRIM_BOTH"
            | "IDM_EDIT_EOL2WS"
            | "IDM_EDIT_TRIMALL"
            | "IDM_EDIT_TAB2SW"
            | "IDM_EDIT_SW2TAB_ALL"
            | "IDM_EDIT_SW2TAB_LEADING"
            | "IDM_EDIT_BEGINENDSELECT"
            | "IDM_EDIT_BEGINENDSELECT_COLUMNMODE"
            | "IDM_EDIT_OPENSELECTEDFILETOEDIT"
            | "IDM_EDIT_OPENSELECTEDFILEFOLDERINEXPLORER"
            | "IDM_EDIT_SEARCHONINTERNET"
            | "IDM_EDIT_CHANGESEARCHENGINE"
            | "IDM_EDIT_JOIN_LINES"
            | "IDM_EDIT_LINE_UP"
            | "IDM_EDIT_LINE_DOWN"
            | "IDM_EDIT_BLANKLINEABOVECURRENT"
            | "IDM_EDIT_BLANKLINEBELOWCURRENT"
            | "IDM_EDIT_REMOVEEMPTYLINES"
            | "IDM_EDIT_REMOVEEMPTYLINESWITHBLANK"
            | "IDM_EDIT_REMOVE_ANY_DUP_LINES"
            | "IDM_EDIT_REMOVE_CONSECUTIVE_DUP_LINES"
            | "IDM_EDIT_SPLIT_LINES"
            | "IDM_EDIT_SORTLINES_LEXICOGRAPHIC_ASCENDING"
            | "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_ASCENDING"
            | "IDM_EDIT_SORTLINES_LOCALE_ASCENDING"
            | "IDM_EDIT_SORTLINES_REVERSE_ORDER"
            | "IDM_EDIT_SORTLINES_LENGTH_ASCENDING"
            | "IDM_EDIT_SORTLINES_LENGTH_DESCENDING"
            | "IDM_EDIT_SORTLINES_INTEGER_ASCENDING"
            | "IDM_EDIT_SORTLINES_INTEGER_DESCENDING"
            | "IDM_EDIT_SORTLINES_DECIMALCOMMA_ASCENDING"
            | "IDM_EDIT_SORTLINES_DECIMALCOMMA_DESCENDING"
            | "IDM_EDIT_SORTLINES_DECIMALDOT_ASCENDING"
            | "IDM_EDIT_SORTLINES_DECIMALDOT_DESCENDING"
            | "IDM_EDIT_SORTLINES_LOCALE_DESCENDING"
            | "IDM_EDIT_SENTENCECASE_FORCE"
            | "IDM_EDIT_SENTENCECASE_BLEND"
            | "IDM_EDIT_INSERT_DATETIME_SHORT"
            | "IDM_EDIT_INSERT_DATETIME_LONG"
            | "IDM_EDIT_FULLPATHTOCLIP"
            | "IDM_EDIT_FILENAMETOCLIP"
            | "IDM_EDIT_CURRENTDIRTOCLIP"
            | "IDM_EDIT_COPY_ALL_NAMES"
            | "IDM_EDIT_COPY_ALL_PATHS"
            | "IDM_FORMAT_TOUNIX"
            | "IDM_FORMAT_TODOS"
            | "IDM_FORMAT_TOMAC"
            | "IDM_FORMAT_UTF_8"
            | "IDM_FORMAT_AS_UTF_8"
            | "IDM_FORMAT_ANSI"
            | "IDM_SEARCH_FIND"
            | "IDM_SEARCH_REPLACE"
            | "IDM_SEARCH_FINDNEXT"
            | "IDM_SEARCH_FINDPREV"
            | "IDM_SEARCH_SETANDFINDNEXT"
            | "IDM_SEARCH_SETANDFINDPREV"
            | "IDM_SEARCH_GOTOLINE"
            | "IDM_SEARCH_VOLATILE_FINDNEXT"
            | "IDM_SEARCH_VOLATILE_FINDPREV"
            | "IDM_SEARCH_FINDINCREMENT"
            | "IDM_FOCUS_ON_FOUND_RESULTS"
            | "IDM_SEARCH_GOTOMATCHINGBRACE"
            | "IDM_SEARCH_SELECTMATCHINGBRACES"
            | "IDM_VIEW_GOTO_START"
            | "IDM_VIEW_GOTO_END"
            | "IDM_VIEW_ZOOMIN"
            | "IDM_VIEW_ZOOMOUT"
            | "IDM_VIEW_ZOOMRESTORE"
            | "IDM_VIEW_ALWAYSONTOP"
            | "IDM_VIEW_MONITORING"
            | "IDM_VIEW_FULLSCREENTOGGLE"
            | "IDM_VIEW_DISTRACTIONFREE"
            | "IDM_VIEW_IN_FIREFOX"
            | "IDM_VIEW_IN_CHROME"
            | "IDM_VIEW_IN_EDGE"
            | "IDM_VIEW_IN_IE"
            | "IDM_TOOL_MD5_GENERATE"
            | "IDM_TOOL_MD5_GENERATEFROMFILE"
            | "IDM_TOOL_MD5_GENERATEINTOCLIPBOARD"
            | "IDM_TOOL_SHA1_GENERATE"
            | "IDM_TOOL_SHA1_GENERATEFROMFILE"
            | "IDM_TOOL_SHA1_GENERATEINTOCLIPBOARD"
            | "IDM_TOOL_SHA256_GENERATE"
            | "IDM_TOOL_SHA256_GENERATEFROMFILE"
            | "IDM_TOOL_SHA256_GENERATEINTOCLIPBOARD"
            | "IDM_TOOL_SHA512_GENERATE"
            | "IDM_TOOL_SHA512_GENERATEFROMFILE"
            | "IDM_TOOL_SHA512_GENERATEINTOCLIPBOARD"
            | "IDM_WINDOW_SORT_FN_ASC"
            | "IDM_WINDOW_SORT_FN_DSC"
            | "IDM_WINDOW_SORT_FP_ASC"
            | "IDM_WINDOW_SORT_FP_DSC"
            | "IDM_WINDOW_SORT_FT_ASC"
            | "IDM_WINDOW_SORT_FT_DSC"
            | "IDM_WINDOW_SORT_FS_ASC"
            | "IDM_WINDOW_SORT_FS_DSC"
            | "IDM_WINDOW_SORT_FD_ASC"
            | "IDM_WINDOW_SORT_FD_DSC"
            | "IDM_EDIT_REDACT_SELECTION"
            | "IDM_EDIT_TOGGLEREADONLY"
            | "IDM_EDIT_SETREADONLYFORALLDOCS"
            | "IDM_EDIT_CLEARREADONLYFORALLDOCS"
            | "IDM_SEARCH_TOGGLE_BOOKMARK"
            | "IDM_SEARCH_NEXT_BOOKMARK"
            | "IDM_SEARCH_PREV_BOOKMARK"
            | "IDM_SEARCH_CLEAR_BOOKMARKS"
            | "IDM_SEARCH_COPYMARKEDLINES"
            | "IDM_SEARCH_DELETEMARKEDLINES"
            | "IDM_SEARCH_DELETEUNMARKEDLINES"
            | "IDM_SEARCH_INVERSEMARKS"
            | "IDM_SETTING_OPENPLUGINSDIR"
            | "IDM_CMDLINEARGUMENTS"
            | "IDM_EDIT_SORTLINES_LEXICOGRAPHIC_DESCENDING"
            | "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_DESCENDING"
            | "IDM_EDIT_SORTLINES_RANDOMLY"
            | "IDM_EDIT_RANDOMCASE"
            | "IDM_VIEW_TAB1"
            | "IDM_VIEW_TAB2"
            | "IDM_VIEW_TAB3"
            | "IDM_VIEW_TAB4"
            | "IDM_VIEW_TAB5"
            | "IDM_VIEW_TAB6"
            | "IDM_VIEW_TAB7"
            | "IDM_VIEW_TAB8"
            | "IDM_VIEW_TAB9"
            | "IDM_VIEW_TAB_NEXT"
            | "IDM_VIEW_TAB_PREV"
            | "IDM_VIEW_TAB_START"
            | "IDM_VIEW_TAB_END"
            | "IDM_LANG_C"
            | "IDM_LANG_CPP"
            | "IDM_LANG_JAVA"
            | "IDM_LANG_JS"
            | "IDM_LANG_JAVASCRIPT"
            | "IDM_LANG_JSON"
            | "IDM_LANG_HTML"
            | "IDM_LANG_XML"
            | "IDM_LANG_PYTHON"
            | "IDM_LANG_SQL"
            | "IDM_LANG_MD"
            | "IDM_LANG_MARKDOWN"
            | "IDM_LANG_RUST"
            | "IDM_LANG_TEXT"
            | "IDM_ABOUT"
            | "IDM_HOMESWEETHOME"
            | "IDM_PROJECTPAGE"
            | "IDM_ONLINEDOCUMENT"
            | "IDM_FORUM"
            | "IDM_UPDATE_NPP"
            | "IDM_DEBUGINFO"
    ) || cmd.starts_with("IDM_LANG_")
        || cmd.starts_with("IDM_FORMAT_")
}

/// Run a menu command. Returns whether it was implemented or only stubbed.
pub fn dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> CmdResult {
    match cmd {
        // —— File ——
        "IDM_FILE_NEW" => {
            state.new_file();
            CmdResult::Handled
        }
        "IDM_FILE_OPEN" => {
            state.open_dialog();
            CmdResult::Handled
        }
        "IDM_FILE_SAVE" => {
            state.save();
            CmdResult::Handled
        }
        "IDM_FILE_SAVEAS" => {
            state.save_as_dialog();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSE" => {
            let idx = state.tabs.active_index();
            state.tabs.close(idx);
            state.highlight_dirty = true;
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL" => {
            while state.tabs.len() > 0 {
                state.tabs.close(0);
            }
            state.highlight_dirty = true;
            CmdResult::Handled
        }
        "IDM_FILE_EXIT" => {
            ui.request_quit = true;
            CmdResult::Handled
        }
        "IDM_FILE_RELOAD" => {
            if let Some(path) = state.tabs.active().path.clone() {
                state.open_path(path);
            } else {
                state.status = "Reload: untitled buffer".into();
            }
            CmdResult::Handled
        }
        "IDM_FILE_SAVEALL" => {
            state.save_all();
            CmdResult::Handled
        }
        "IDM_FILE_SAVECOPYAS" => {
            state.save_copy_as();
            CmdResult::Handled
        }
        "IDM_FILE_RENAME" => {
            state.rename_active();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_BUT_CURRENT" => {
            state.close_all_but_current();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_TOLEFT" => {
            state.close_all_to_left();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_TORIGHT" => {
            state.close_all_to_right();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_UNCHANGED" => {
            state.close_all_unchanged();
            CmdResult::Handled
        }
        "IDM_FILE_OPEN_FOLDER" => {
            state.reveal_in_os();
            CmdResult::Handled
        }
        "IDM_FILE_OPENFOLDERASWORKSPACE" | "IDM_FILE_CONTAININGFOLDERASWORKSPACE" => {
            state.open_containing_folder();
            CmdResult::Handled
        }
        "IDM_FILE_OPEN_CMD" | "IDM_FILE_OPEN_POWERSHELL" => {
            state.open_shell_here();
            CmdResult::Handled
        }
        "IDM_FILE_OPEN_DEFAULT_VIEWER" => {
            state.open_in_default_viewer();
            CmdResult::Handled
        }

        // —— Edit ——
        "IDM_EDIT_UNDO" => {
            state.undo();
            CmdResult::Handled
        }
        "IDM_EDIT_REDO" => {
            state.redo();
            CmdResult::Handled
        }
        "IDM_EDIT_CUT" => {
            cut_selection(state);
            CmdResult::Handled
        }
        "IDM_EDIT_COPY" => {
            copy_selection(state, ui);
            CmdResult::Handled
        }
        "IDM_EDIT_PASTE" => {
            state.status = "Paste: use ⌘/Ctrl+V in the editor".into();
            CmdResult::Handled
        }
        "IDM_EDIT_DELETE" => {
            state.tabs.active_mut().buffer.delete_forward();
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_SELECTALL" => {
            state.tabs.active_mut().buffer.select_all();
            CmdResult::Handled
        }
        "IDM_EDIT_INS_TAB" => {
            state.tabs.active_mut().buffer.indent_lines("    ");
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_RMV_TAB" => {
            state.tabs.active_mut().buffer.outdent_lines(4);
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_UPPERCASE" => {
            state.run_plugin("edit.uppercase");
            CmdResult::Handled
        }
        "IDM_EDIT_LOWERCASE" => {
            state.run_plugin("edit.lowercase");
            CmdResult::Handled
        }
        "IDM_EDIT_INVERTCASE" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                s.chars()
                    .map(|c| {
                        if c.is_uppercase() {
                            c.to_lowercase().collect::<String>()
                        } else if c.is_lowercase() {
                            c.to_uppercase().collect::<String>()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect()
            });
            state.mark_text_changed();
            state.status = "Invert case".into();
            CmdResult::Handled
        }
        "IDM_EDIT_PROPERCASE_FORCE" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut out = String::new();
                let mut new_word = true;
                for c in s.chars() {
                    if c.is_alphabetic() {
                        if new_word {
                            out.extend(c.to_uppercase());
                            new_word = false;
                        } else {
                            out.extend(c.to_lowercase());
                        }
                    } else {
                        out.push(c);
                        new_word = !c.is_alphanumeric();
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Proper Case".into();
            CmdResult::Handled
        }
        "IDM_EDIT_PROPERCASE_BLEND" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut out = String::new();
                let mut new_word = true;
                for c in s.chars() {
                    if c.is_alphabetic() {
                        if new_word {
                            out.extend(c.to_uppercase());
                            new_word = false;
                        } else {
                            out.push(c);
                        }
                    } else {
                        out.push(c);
                        new_word = !c.is_alphanumeric();
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Proper Case (blend)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_DUP_LINE" => {
            state.tabs.active_mut().buffer.duplicate_line();
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_JOIN_LINES" => {
            state.tabs.active_mut().buffer.join_lines();
            state.mark_text_changed();
            state.status = "Join lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_LINE_UP" => {
            state.tabs.active_mut().buffer.move_line_up();
            state.mark_text_changed();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_EDIT_LINE_DOWN" => {
            state.tabs.active_mut().buffer.move_line_down();
            state.mark_text_changed();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_EDIT_BLANKLINEABOVECURRENT" => {
            state.tabs.active_mut().buffer.blank_line_above();
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_BLANKLINEBELOWCURRENT" => {
            let line = state.tabs.active().buffer.char_to_line(state.tabs.active().buffer.caret());
            let at = if line + 1 < state.tabs.active().buffer.line_count() {
                state.tabs.active().buffer.line_to_char(line + 1)
            } else {
                state.tabs.active().buffer.len_chars()
            };
            state.tabs.active_mut().buffer.set_caret(at);
            state.tabs.active_mut().buffer.insert("\n");
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_SPLIT_LINES" => {
            // Split at caret: insert newline (Notepad++ wraps selection; we insert NL).
            state.tabs.active_mut().buffer.insert("\n");
            state.mark_text_changed();
            state.status = "Split line".into();
            CmdResult::Handled
        }
        "IDM_EDIT_REMOVE_CONSECUTIVE_DUP_LINES" | "IDM_EDIT_REMOVE_ANY_DUP_LINES" => {
            let any = cmd == "IDM_EDIT_REMOVE_ANY_DUP_LINES";
            let text = state.tabs.active().buffer.to_string();
            let mut out_lines = Vec::new();
            let mut seen = std::collections::HashSet::new();
            let mut prev: Option<String> = None;
            for line in text.lines() {
                let key = line.to_string();
                if any {
                    if seen.insert(key.clone()) {
                        out_lines.push(key);
                    }
                } else if prev.as_ref() != Some(&key) {
                    out_lines.push(key.clone());
                    prev = Some(key);
                }
            }
            let mut out = out_lines.join("\n");
            if text.ends_with('\n') {
                out.push('\n');
            }
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Removed duplicate lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SORTLINES_LEXICOGRAPHIC_ASCENDING"
        | "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_ASCENDING"
        | "IDM_EDIT_SORTLINES_LOCALE_ASCENDING"
        | "IDM_EDIT_SORTLINES_LOCALE_DESCENDING"
        | "IDM_EDIT_SORTLINES_REVERSE_ORDER"
        | "IDM_EDIT_SORTLINES_LENGTH_ASCENDING"
        | "IDM_EDIT_SORTLINES_LENGTH_DESCENDING"
        | "IDM_EDIT_SORTLINES_LEXICOGRAPHIC_DESCENDING"
        | "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_DESCENDING"
        | "IDM_EDIT_SORTLINES_INTEGER_ASCENDING"
        | "IDM_EDIT_SORTLINES_INTEGER_DESCENDING"
        | "IDM_EDIT_SORTLINES_DECIMALCOMMA_ASCENDING"
        | "IDM_EDIT_SORTLINES_DECIMALCOMMA_DESCENDING"
        | "IDM_EDIT_SORTLINES_DECIMALDOT_ASCENDING"
        | "IDM_EDIT_SORTLINES_DECIMALDOT_DESCENDING"
        | "IDM_EDIT_SORTLINES_RANDOMLY" => {
            let text = state.tabs.active().buffer.to_string();
            let mut lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
            match cmd {
                "IDM_EDIT_SORTLINES_REVERSE_ORDER" => lines.reverse(),
                "IDM_EDIT_SORTLINES_LENGTH_ASCENDING" => {
                    lines.sort_by_key(|l| l.chars().count());
                }
                "IDM_EDIT_SORTLINES_LENGTH_DESCENDING" => {
                    lines.sort_by(|a, b| b.chars().count().cmp(&a.chars().count()));
                }
                "IDM_EDIT_SORTLINES_LEXICOGRAPHIC_DESCENDING"
                | "IDM_EDIT_SORTLINES_LOCALE_DESCENDING" => {
                    lines.sort();
                    lines.reverse();
                }
                "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_DESCENDING" => {
                    lines.sort_by_key(|l| l.to_ascii_lowercase());
                    lines.reverse();
                }
                "IDM_EDIT_SORTLINES_INTEGER_ASCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(a, b, NumSort::Integer));
                }
                "IDM_EDIT_SORTLINES_INTEGER_DESCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(b, a, NumSort::Integer));
                }
                "IDM_EDIT_SORTLINES_DECIMALCOMMA_ASCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(a, b, NumSort::DecimalComma));
                }
                "IDM_EDIT_SORTLINES_DECIMALCOMMA_DESCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(b, a, NumSort::DecimalComma));
                }
                "IDM_EDIT_SORTLINES_DECIMALDOT_ASCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(a, b, NumSort::DecimalDot));
                }
                "IDM_EDIT_SORTLINES_DECIMALDOT_DESCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(b, a, NumSort::DecimalDot));
                }
                "IDM_EDIT_SORTLINES_RANDOMLY" => {
                    // Simple deterministic shuffle from content hash (no extra crate).
                    let mut seed: u64 = lines.len() as u64;
                    for l in &lines {
                        for b in l.bytes() {
                            seed = seed.wrapping_mul(31).wrapping_add(u64::from(b));
                        }
                    }
                    for i in (1..lines.len()).rev() {
                        seed = seed
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1);
                        let j = (seed as usize) % (i + 1);
                        lines.swap(i, j);
                    }
                }
                "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_ASCENDING" => {
                    lines.sort_by_key(|l| l.to_ascii_lowercase());
                }
                _ => lines.sort(),
            }
            let mut out = lines.join("\n");
            if text.ends_with('\n') {
                out.push('\n');
            }
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Sorted lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SENTENCECASE_FORCE" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut out = String::new();
                let mut cap = true;
                for c in s.chars() {
                    if cap && c.is_alphabetic() {
                        out.extend(c.to_uppercase());
                        cap = false;
                    } else {
                        out.extend(c.to_lowercase());
                        if matches!(c, '.' | '!' | '?') {
                            cap = true;
                        }
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Sentence case".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SENTENCECASE_BLEND" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut out = String::new();
                let mut cap = true;
                for c in s.chars() {
                    if cap && c.is_alphabetic() {
                        out.extend(c.to_uppercase());
                        cap = false;
                    } else {
                        out.push(c);
                        if matches!(c, '.' | '!' | '?') {
                            cap = true;
                        }
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Sentence case (blend)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_RANDOMCASE" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut seed: u64 = s.len() as u64;
                let mut out = String::new();
                for c in s.chars() {
                    seed = seed.wrapping_mul(31).wrapping_add(c as u64);
                    if c.is_alphabetic() {
                        if seed & 1 == 0 {
                            out.extend(c.to_uppercase());
                        } else {
                            out.extend(c.to_lowercase());
                        }
                    } else {
                        out.push(c);
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Random case".into();
            CmdResult::Handled
        }
        "IDM_EDIT_REMOVEEMPTYLINES" => {
            state.tabs.active_mut().buffer.remove_empty_lines(false);
            state.mark_text_changed();
            state.status = "Removed empty lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_REMOVEEMPTYLINESWITHBLANK" => {
            state.tabs.active_mut().buffer.remove_empty_lines(true);
            state.mark_text_changed();
            state.status = "Removed blank lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_INSERT_DATETIME_SHORT" => {
            state.insert_datetime(false);
            CmdResult::Handled
        }
        "IDM_EDIT_INSERT_DATETIME_LONG" => {
            state.insert_datetime(true);
            CmdResult::Handled
        }
        "IDM_EDIT_FULLPATHTOCLIP" => {
            if let Some(p) = state.tabs.active().path.clone() {
                ui.pending_clipboard = Some(p.display().to_string());
                state.status = "Full path copied".into();
            } else {
                state.status = "No path to copy".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_FILENAMETOCLIP" => {
            ui.pending_clipboard = Some(state.tabs.active().title.clone());
            state.status = "File name copied".into();
            CmdResult::Handled
        }
        "IDM_EDIT_CURRENTDIRTOCLIP" => {
            if let Some(p) = state.tabs.active().path.as_ref().and_then(|p| p.parent()) {
                ui.pending_clipboard = Some(p.display().to_string());
                state.status = "Directory copied".into();
            } else {
                state.status = "No directory to copy".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_COPY_ALL_NAMES" => {
            let names: Vec<_> = state.tabs.iter().map(|d| d.title.clone()).collect();
            ui.pending_clipboard = Some(names.join("\n"));
            state.status = "All tab names copied".into();
            CmdResult::Handled
        }
        "IDM_EDIT_COPY_ALL_PATHS" => {
            let paths: Vec<_> = state
                .tabs
                .iter()
                .map(|d| {
                    d.path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| d.title.clone())
                })
                .collect();
            ui.pending_clipboard = Some(paths.join("\n"));
            state.status = "All paths copied".into();
            CmdResult::Handled
        }
        "IDM_EDIT_BEGINENDSELECT" | "IDM_EDIT_BEGINENDSELECT_COLUMNMODE" => {
            let caret = state.tabs.active().buffer.caret();
            match state.begin_end_select {
                None => {
                    state.begin_end_select = Some(caret);
                    state.status = "Begin/End Select: start set".into();
                }
                Some(anchor) => {
                    state.tabs.active_mut().buffer.set_selection(anchor, caret);
                    state.begin_end_select = None;
                    ui.follow_caret = true;
                    state.status = "Begin/End Select: selection set".into();
                }
            }
            CmdResult::Handled
        }
        "IDM_EDIT_OPENSELECTEDFILETOEDIT" => {
            if let Some(path) = resolve_selected_path(state) {
                state.open_path(path);
            } else {
                state.status = "Open selection: no existing path in selection".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_OPENSELECTEDFILEFOLDERINEXPLORER" => {
            if let Some(path) = resolve_selected_path(state) {
                let folder = if path.is_dir() {
                    path
                } else {
                    path.parent()
                        .map(Path::to_path_buf)
                        .unwrap_or(path)
                };
                open_path_in_os(state, &folder);
            } else {
                state.status = "Open folder: no existing path in selection".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_SEARCHONINTERNET" => {
            let q = if let Some((s, e)) = state.tabs.active().buffer.selection() {
                state.tabs.active().buffer.slice(s, e)
            } else {
                String::new()
            };
            let q = q.trim();
            if q.is_empty() {
                state.status = "Search on Internet: select text first".into();
            } else {
                let enc: String = {
                    let mut out = String::new();
                    for b in q.as_bytes() {
                        match *b {
                            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                                out.push(*b as char);
                            }
                            b' ' => out.push('+'),
                            _ => out.push_str(&format!("%{b:02X}")),
                        }
                    }
                    out
                };
                let url = format!("{}{enc}", state.search_engine);
                open_url(state, &url);
            }
            CmdResult::Handled
        }
        "IDM_EDIT_CHANGESEARCHENGINE" => {
            state.search_engine = if state.search_engine.contains("duckduckgo") {
                "https://www.google.com/search?q=".into()
            } else if state.search_engine.contains("google") {
                "https://www.bing.com/search?q=".into()
            } else {
                "https://duckduckgo.com/?q=".into()
            };
            state.status = format!("Search engine: {}", state.search_engine);
            CmdResult::Handled
        }
        "IDM_EDIT_REDACT_SELECTION" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return CmdResult::Handled;
            }
            if let Some((s, e)) = state.tabs.active().buffer.selection() {
                let n = e.saturating_sub(s);
                let block = "█".repeat(n.max(1));
                state.tabs.active_mut().buffer.insert(&block);
                state.mark_text_changed();
                state.status = "Redacted selection".into();
            } else {
                state.status = "Redact: select text first".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_TOGGLEREADONLY" => {
            let d = state.tabs.active_mut();
            d.read_only = !d.read_only;
            state.status = if d.read_only {
                "Read-only: on".into()
            } else {
                "Read-only: off".into()
            };
            CmdResult::Handled
        }
        "IDM_EDIT_SETREADONLYFORALLDOCS" => {
            for i in 0..state.tabs.len() {
                if let Some(d) = state.tabs.get_mut(i) {
                    d.read_only = true;
                }
            }
            state.status = "Read-only: all documents".into();
            CmdResult::Handled
        }
        "IDM_EDIT_CLEARREADONLYFORALLDOCS" => {
            for i in 0..state.tabs.len() {
                if let Some(d) = state.tabs.get_mut(i) {
                    d.read_only = false;
                }
            }
            state.status = "Read-only: cleared for all".into();
            CmdResult::Handled
        }
        "IDM_EDIT_TRIMTRAILING" => {
            state.run_plugin("edit.trim_trailing");
            CmdResult::Handled
        }
        "IDM_EDIT_TRIMLINEHEAD" => {
            let text = state.tabs.active().buffer.to_string();
            let out = map_line_bodies(&text, |body| body.trim_start().to_string());
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Trim leading space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_TRIM_BOTH" => {
            let text = state.tabs.active().buffer.to_string();
            let out = map_line_bodies(&text, |body| body.trim().to_string());
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Trim leading and trailing space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_EOL2WS" => {
            let text = state.tabs.active().buffer.to_string();
            let out = text.replace("\r\n", " ").replace(['\n', '\r'], " ");
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "EOL to space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_TRIMALL" => {
            let text = state.tabs.active().buffer.to_string();
            let trimmed = map_line_bodies(&text, |body| body.trim().to_string());
            let out = trimmed.replace("\r\n", " ").replace(['\n', '\r'], " ");
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Trim both and EOL to space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_TAB2SW" => {
            let text = state.tabs.active().buffer.to_string().replace('\t', "    ");
            state.tabs.active_mut().buffer.replace_document(&text);
            state.mark_text_changed();
            state.status = "TAB to space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SW2TAB_ALL" => {
            let text = state.tabs.active().buffer.to_string().replace("    ", "\t");
            state.tabs.active_mut().buffer.replace_document(&text);
            state.mark_text_changed();
            state.status = "Space to TAB (all)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SW2TAB_LEADING" => {
            let text = state.tabs.active().buffer.to_string();
            let out = map_line_bodies(&text, spaces_to_tabs_leading);
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Space to TAB (leading)".into();
            CmdResult::Handled
        }
        "IDM_FORMAT_TOUNIX" => {
            state.run_plugin("edit.to_unix_eol");
            CmdResult::Handled
        }
        "IDM_FORMAT_TODOS" => {
            state.run_plugin("edit.to_windows_eol");
            CmdResult::Handled
        }
        "IDM_FORMAT_UTF_8" | "IDM_FORMAT_AS_UTF_8" => {
            state.status = "Encoding: UTF-8 (native)".into();
            CmdResult::Handled
        }
        "IDM_FORMAT_ANSI" => {
            state.status = "Encoding: ANSI requested — npp-rs keeps UTF-8 in memory".into();
            CmdResult::Handled
        }
        "IDM_FORMAT_TOMAC" => {
            // Classic Mac CR line endings.
            let text = state.tabs.active().buffer.to_string().replace("\r\n", "\n").replace('\n', "\r");
            state.tabs.active_mut().buffer.replace_document(&text);
            state.mark_text_changed();
            state.status = "EOL: Macintosh (CR)".into();
            CmdResult::Handled
        }

        // —— Search ——
        "IDM_SEARCH_FIND" => {
            ui.find_open = true;
            ui.show_replace = false;
            ui.find_focus_once = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_REPLACE" => {
            ui.find_open = true;
            ui.show_replace = true;
            ui.find_focus_once = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDNEXT" => {
            state.find_next();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDPREV" => {
            state.find_prev();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_SETANDFINDNEXT" => {
            if let Some((s, e)) = state.tabs.active().buffer.selection() {
                state.find_query = state.tabs.active().buffer.slice(s, e);
            }
            state.find_next();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_SETANDFINDPREV" => {
            if let Some((s, e)) = state.tabs.active().buffer.selection() {
                state.find_query = state.tabs.active().buffer.slice(s, e);
            }
            state.find_prev();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_GOTOLINE" => {
            ui.show_goto_line = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_VOLATILE_FINDNEXT" => {
            state.find_next();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_VOLATILE_FINDPREV" => {
            state.find_prev();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDINCREMENT" | "IDM_FOCUS_ON_FOUND_RESULTS" => {
            ui.find_open = true;
            ui.show_replace = false;
            ui.find_focus_once = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_GOTOMATCHINGBRACE" => {
            let text = state.tabs.active().buffer.to_string();
            let caret = state.tabs.active().buffer.caret();
            if let Some(at) = find_matching_brace(&text, caret) {
                state.tabs.active_mut().buffer.set_caret(at);
                ui.follow_caret = true;
                state.status = "Matching brace".into();
            } else {
                state.status = "No matching brace".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_SELECTMATCHINGBRACES" => {
            let text = state.tabs.active().buffer.to_string();
            let caret = state.tabs.active().buffer.caret();
            if let Some((a, b)) = brace_span(&text, caret) {
                let (s, e) = if a < b { (a, b + 1) } else { (b, a + 1) };
                state.tabs.active_mut().buffer.set_selection(s, e);
                ui.follow_caret = true;
                state.status = "Selected brace pair".into();
            } else {
                state.status = "No matching brace".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_TOGGLE_BOOKMARK" => {
            let line = state
                .tabs
                .active()
                .buffer
                .char_to_line(state.tabs.active().buffer.caret());
            let marks = &mut state.tabs.active_mut().bookmarks;
            if !marks.remove(&line) {
                marks.insert(line);
                state.status = format!("Bookmark on line {}", line + 1);
            } else {
                state.status = format!("Bookmark cleared on line {}", line + 1);
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_NEXT_BOOKMARK" => {
            let line = state
                .tabs
                .active()
                .buffer
                .char_to_line(state.tabs.active().buffer.caret());
            let next = state
                .tabs
                .active()
                .bookmarks
                .iter()
                .copied()
                .find(|&l| l > line)
                .or_else(|| state.tabs.active().bookmarks.iter().copied().next());
            if let Some(l) = next {
                let at = state.tabs.active().buffer.line_to_char(l);
                state.tabs.active_mut().buffer.set_caret(at);
                ui.follow_caret = true;
                state.status = format!("Bookmark line {}", l + 1);
            } else {
                state.status = "No bookmarks".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_PREV_BOOKMARK" => {
            let line = state
                .tabs
                .active()
                .buffer
                .char_to_line(state.tabs.active().buffer.caret());
            let prev = state
                .tabs
                .active()
                .bookmarks
                .iter()
                .rev()
                .copied()
                .find(|&l| l < line)
                .or_else(|| state.tabs.active().bookmarks.iter().next_back().copied());
            if let Some(l) = prev {
                let at = state.tabs.active().buffer.line_to_char(l);
                state.tabs.active_mut().buffer.set_caret(at);
                ui.follow_caret = true;
                state.status = format!("Bookmark line {}", l + 1);
            } else {
                state.status = "No bookmarks".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_CLEAR_BOOKMARKS" => {
            state.tabs.active_mut().bookmarks.clear();
            state.status = "Bookmarks cleared".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_INVERSEMARKS" => {
            let n = state.tabs.active().buffer.line_count();
            let old = state.tabs.active().bookmarks.clone();
            let marks = &mut state.tabs.active_mut().bookmarks;
            marks.clear();
            for l in 0..n {
                if !old.contains(&l) {
                    marks.insert(l);
                }
            }
            state.status = "Bookmarks inverted".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_COPYMARKEDLINES" => {
            let text = state.tabs.active().buffer.to_string();
            let lines: Vec<&str> = text.lines().collect();
            let marked: Vec<&str> = state
                .tabs
                .active()
                .bookmarks
                .iter()
                .filter_map(|&i| lines.get(i).copied())
                .collect();
            ui.pending_clipboard = Some(marked.join("\n"));
            state.status = format!("Copied {} bookmarked line(s)", marked.len());
            CmdResult::Handled
        }
        "IDM_SEARCH_DELETEMARKEDLINES" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return CmdResult::Handled;
            }
            filter_lines_by_bookmarks(state, true);
            CmdResult::Handled
        }
        "IDM_SEARCH_DELETEUNMARKEDLINES" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return CmdResult::Handled;
            }
            filter_lines_by_bookmarks(state, false);
            CmdResult::Handled
        }

        "IDM_VIEW_GOTO_START" => {
            let b = state.tabs.active_mut();
            b.buffer.set_caret(0);
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_VIEW_GOTO_END" => {
            let end = state.tabs.active().buffer.len_chars();
            state.tabs.active_mut().buffer.set_caret(end);
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_VIEW_ZOOMIN" => {
            ui.zoom_delta = Some(1);
            CmdResult::Handled
        }
        "IDM_VIEW_ZOOMOUT" => {
            ui.zoom_delta = Some(-1);
            CmdResult::Handled
        }
        "IDM_VIEW_ZOOMRESTORE" => {
            ui.zoom_delta = Some(0);
            CmdResult::Handled
        }
        "IDM_VIEW_ALWAYSONTOP" => {
            ui.always_on_top = Some(true);
            state.status = "Always on top requested".into();
            CmdResult::Handled
        }
        "IDM_VIEW_MONITORING" => {
            if state.toggle_tail_follow() {
                ui.follow_caret = true;
            }
            CmdResult::Handled
        }
        "IDM_VIEW_FULLSCREENTOGGLE" | "IDM_VIEW_DISTRACTIONFREE" => {
            ui.fullscreen_toggle = true;
            state.status = "Fullscreen toggled".into();
            CmdResult::Handled
        }
        "IDM_VIEW_IN_FIREFOX" | "IDM_VIEW_IN_CHROME" | "IDM_VIEW_IN_EDGE" | "IDM_VIEW_IN_IE" => {
            open_active_in_browser(state, cmd);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB1" => {
            state.switch_tab(0);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB2" => {
            state.switch_tab(1);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB3" => {
            state.switch_tab(2);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB4" => {
            state.switch_tab(3);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB5" => {
            state.switch_tab(4);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB6" => {
            state.switch_tab(5);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB7" => {
            state.switch_tab(6);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB8" => {
            state.switch_tab(7);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB9" => {
            state.switch_tab(8);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_NEXT" => {
            state.next_tab();
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_PREV" => {
            state.prev_tab();
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_START" => {
            state.switch_tab(0);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_END" => {
            let last = state.tabs.len().saturating_sub(1);
            state.switch_tab(last);
            CmdResult::Handled
        }

        // —— Language (subset we highlight) ——
        "IDM_LANG_C" => {
            state.set_language("c");
            CmdResult::Handled
        }
        "IDM_LANG_CPP" => {
            state.set_language("cpp");
            CmdResult::Handled
        }
        "IDM_LANG_JAVA" => {
            state.set_language("java");
            CmdResult::Handled
        }
        "IDM_LANG_JS" | "IDM_LANG_JAVASCRIPT" => {
            state.set_language("javascript");
            CmdResult::Handled
        }
        "IDM_LANG_JSON" => {
            state.set_language("json");
            CmdResult::Handled
        }
        "IDM_LANG_HTML" => {
            state.set_language("html");
            CmdResult::Handled
        }
        "IDM_LANG_XML" => {
            state.set_language("xml");
            CmdResult::Handled
        }
        "IDM_LANG_PYTHON" => {
            state.set_language("python");
            CmdResult::Handled
        }
        "IDM_LANG_SQL" => {
            state.set_language("sql");
            CmdResult::Handled
        }
        "IDM_LANG_MD" | "IDM_LANG_MARKDOWN" => {
            state.set_language("markdown");
            CmdResult::Handled
        }
        "IDM_LANG_RUST" => {
            state.set_language("rust");
            CmdResult::Handled
        }
        "IDM_LANG_TEXT" => {
            state.set_language("plain");
            CmdResult::Handled
        }

        // —— Tools / plugins ——
        "IDM_SETTING_PLUGINADM" | "IDM_SETTING_SHORTCUT_MAPPER" => CmdResult::Stub,
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
            state.tabs.sort_tabs(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            state.status = "Tabs sorted by name ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FN_DSC" => {
            state.tabs.sort_tabs(|a, b| b.title.to_lowercase().cmp(&a.title.to_lowercase()));
            state.status = "Tabs sorted by name ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FP_ASC" => {
            state.tabs.sort_tabs(|a, b| {
                let ap = a.path.as_ref().map(|p| p.to_string_lossy().to_lowercase()).unwrap_or_default();
                let bp = b.path.as_ref().map(|p| p.to_string_lossy().to_lowercase()).unwrap_or_default();
                ap.cmp(&bp)
            });
            state.status = "Tabs sorted by path ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FP_DSC" => {
            state.tabs.sort_tabs(|a, b| {
                let ap = a.path.as_ref().map(|p| p.to_string_lossy().to_lowercase()).unwrap_or_default();
                let bp = b.path.as_ref().map(|p| p.to_string_lossy().to_lowercase()).unwrap_or_default();
                bp.cmp(&ap)
            });
            state.status = "Tabs sorted by path ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FT_ASC" => {
            state.tabs.sort_tabs(|a, b| tab_type_key(a).cmp(&tab_type_key(b)));
            state.status = "Tabs sorted by type ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FT_DSC" => {
            state.tabs.sort_tabs(|a, b| tab_type_key(b).cmp(&tab_type_key(a)));
            state.status = "Tabs sorted by type ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FS_ASC" => {
            state.tabs.sort_tabs(|a, b| a.buffer.len_chars().cmp(&b.buffer.len_chars()));
            state.status = "Tabs sorted by size ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FS_DSC" => {
            state.tabs.sort_tabs(|a, b| b.buffer.len_chars().cmp(&a.buffer.len_chars()));
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
            let dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            open_path_in_os(state, &dir);
            CmdResult::Handled
        }
        "IDM_CMDLINEARGUMENTS" => {
            state.status =
                "npp-rs: open files via OS / drag-drop; no CLI flags yet (see README)".into();
            CmdResult::Handled
        }

        // —— Help (?) ——
        "IDM_HOMESWEETHOME" => {
            open_url(state, "https://github.com/raro42/npp-rust");
            CmdResult::Handled
        }
        "IDM_PROJECTPAGE" => {
            open_url(state, "https://github.com/raro42/npp-rust/issues");
            CmdResult::Handled
        }
        "IDM_FORUM" => {
            open_url(state, "https://github.com/raro42/npp-rust/discussions");
            CmdResult::Handled
        }
        "IDM_ONLINEDOCUMENT" => {
            open_url(state, "https://github.com/raro42/npp-rust/wiki");
            CmdResult::Handled
        }
        "IDM_UPDATE_NPP" => {
            open_url(state, "https://github.com/raro42/npp-rust/releases");
            CmdResult::Handled
        }
        "IDM_DEBUGINFO" => {
            state.status = format!(
                "npp-rust {} · Rust · {}",
                env!("CARGO_PKG_VERSION"),
                std::env::consts::OS
            );
            CmdResult::Handled
        }
        "IDM_ABOUT" => {
            ui.show_about = true;
            CmdResult::Handled
        }

        _ => {
            // Language items often named IDM_LANG_*
            if let Some(lang) = cmd.strip_prefix("IDM_LANG_") {
                let mapped = match lang {
                    "C" => "c",
                    "CPP" => "cpp",
                    "PYTHON" => "python",
                    "SQL" => "sql",
                    "RUST" => "rust",
                    "JSON" => "json",
                    "USER" => "plain",
                    other => {
                        state.status = format!(
                            "Language '{other}' selected (highlight may be limited)"
                        );
                        state.tabs.active_mut().language = other.to_ascii_lowercase();
                        state.highlight_dirty = true;
                        return CmdResult::Handled;
                    }
                };
                state.set_language(mapped);
                return CmdResult::Handled;
            }
            // Encoding menu: npp-rs stores UTF-8; acknowledge other picks honestly.
            if let Some(enc) = cmd.strip_prefix("IDM_FORMAT_") {
                if matches!(enc, "TOUNIX" | "TODOS" | "TOMAC" | "UTF_8" | "AS_UTF_8" | "ANSI") {
                    // handled above
                } else {
                    state.status = format!(
                        "Encoding '{enc}' noted — document stays UTF-8 in npp-rs"
                    );
                    return CmdResult::Handled;
                }
            }
            CmdResult::Stub
        }
    }
}

fn open_url(state: &mut EditorState, url: &str) {
    let result = {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(url).status()
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", "", url])
                .status()
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            std::process::Command::new("xdg-open").arg(url).status()
        }
    };
    match result {
        Ok(s) if s.success() => state.status = format!("Opened {url}"),
        Ok(_) => state.status = format!("Could not open {url}"),
        Err(e) => state.status = format!("Open URL failed: {e}"),
    }
}

fn open_path_in_os(state: &mut EditorState, path: &Path) {
    let result = {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(path).status()
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("explorer").arg(path).status()
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            std::process::Command::new("xdg-open").arg(path).status()
        }
    };
    match result {
        Ok(s) if s.success() => state.status = "Opened folder".into(),
        Ok(_) => state.status = "Open folder failed".into(),
        Err(e) => state.status = format!("Open folder failed: {e}"),
    }
}

fn open_active_in_browser(state: &mut EditorState, cmd: &str) {
    let Some(path) = state.tabs.active().path.clone() else {
        state.status = "Save the file first, then open in browser".into();
        return;
    };
    let url = format!("file://{}", path.display());
    let app = match cmd {
        "IDM_VIEW_IN_FIREFOX" => Some("Firefox"),
        "IDM_VIEW_IN_CHROME" => Some("Google Chrome"),
        "IDM_VIEW_IN_EDGE" => Some("Microsoft Edge"),
        _ => None,
    };
    #[cfg(target_os = "macos")]
    {
        let result = if let Some(name) = app {
            std::process::Command::new("open")
                .args(["-a", name, &url])
                .status()
        } else {
            std::process::Command::new("open").arg(&url).status()
        };
        match result {
            Ok(s) if s.success() => state.status = "Opened in browser".into(),
            Ok(_) => open_url(state, &url),
            Err(_) => open_url(state, &url),
        }
        return;
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
        open_url(state, &url);
    }
}

fn hash_via_cli(algo: &str, data_file: &Path) -> Result<String, String> {
    let output = match algo {
        "md5" => {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("md5").args(["-q"]).arg(data_file).output()
            }
            #[cfg(not(target_os = "macos"))]
            {
                std::process::Command::new("md5sum").arg(data_file).output()
            }
        }
        "sha1" => std::process::Command::new("shasum")
            .args(["-a", "1"])
            .arg(data_file)
            .output(),
        "sha256" => std::process::Command::new("shasum")
            .args(["-a", "256"])
            .arg(data_file)
            .output(),
        "sha512" => std::process::Command::new("shasum")
            .args(["-a", "512"])
            .arg(data_file)
            .output(),
        other => return Err(format!("unknown algo {other}")),
    };
    match output {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout);
            Ok(s.split_whitespace().next().unwrap_or("").to_string())
        }
        Ok(o) => Err(String::from_utf8_lossy(&o.stderr).to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn hash_selection_or_doc(state: &mut EditorState, ui: &mut UiFlags, algo: &str, to_clip: bool) {
    let text = if let Some((s, e)) = state.tabs.active().buffer.selection() {
        state.tabs.active().buffer.slice(s, e)
    } else {
        state.tabs.active().buffer.to_string()
    };
    let tmp = std::env::temp_dir().join(format!("npp-rs-hash-{algo}.txt"));
    if let Err(e) = std::fs::write(&tmp, text.as_bytes()) {
        state.status = format!("Hash failed: {e}");
        return;
    }
    match hash_via_cli(algo, &tmp) {
        Ok(h) => {
            let _ = std::fs::remove_file(&tmp);
            if to_clip {
                ui.pending_clipboard = Some(h.clone());
                state.status = format!("{algo}: copied to clipboard");
            } else {
                state.tabs.active_mut().buffer.insert(&format!("\n{h}\n"));
                state.mark_text_changed();
                state.status = format!("{algo}: inserted");
            }
        }
        Err(e) => state.status = format!("Hash failed: {e}"),
    }
}

fn hash_active_file(state: &mut EditorState, ui: &mut UiFlags, algo: &str) {
    let Some(path) = state.tabs.active().path.clone() else {
        state.status = "Hash from file: save first".into();
        return;
    };
    match hash_via_cli(algo, &path) {
        Ok(h) => {
            ui.pending_clipboard = Some(h.clone());
            state.status = format!("{algo} (file): {h}");
        }
        Err(e) => state.status = format!("Hash failed: {e}"),
    }
}

fn cut_selection(state: &mut EditorState) {
    if let Some((s, e)) = state.tabs.active().buffer.selection() {
        let text = state.tabs.active().buffer.slice(s, e);
        state.tabs.active_mut().buffer.delete_backward();
        state.mark_text_changed();
        // Clipboard set by UI layer via pending_clipboard
        state.status = format!("Cut {} chars (clipboard via UI)", text.chars().count());
    }
}

#[derive(Clone, Copy)]
enum NumSort {
    Integer,
    DecimalComma,
    DecimalDot,
}

fn line_num_key(line: &str, kind: NumSort) -> Option<f64> {
    let t = line.trim();
    match kind {
        NumSort::Integer => {
            let mut end = 0usize;
            let bytes = t.as_bytes();
            if bytes.first().is_some_and(|b| *b == b'+' || *b == b'-') {
                end = 1;
            }
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end == 0 || (end == 1 && !bytes[0].is_ascii_digit()) {
                None
            } else {
                t[..end].parse::<i64>().ok().map(|n| n as f64)
            }
        }
        NumSort::DecimalComma => {
            let norm = t.replace(',', ".");
            parse_leading_float(&norm)
        }
        NumSort::DecimalDot => parse_leading_float(t),
    }
}

fn parse_leading_float(s: &str) -> Option<f64> {
    let bytes = s.as_bytes();
    let mut end = 0usize;
    if bytes.first().is_some_and(|b| *b == b'+' || *b == b'-') {
        end = 1;
    }
    let mut seen_dot = false;
    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_digit() {
            end += 1;
        } else if b == b'.' && !seen_dot {
            seen_dot = true;
            end += 1;
        } else {
            break;
        }
    }
    if end == 0 || (end == 1 && !bytes[0].is_ascii_digit()) {
        None
    } else {
        s[..end].parse().ok()
    }
}

fn cmp_num_key(a: &str, b: &str, kind: NumSort) -> std::cmp::Ordering {
    match (line_num_key(a, kind), line_num_key(b, kind)) {
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.cmp(b),
    }
}

fn map_line_bodies(text: &str, mut f: impl FnMut(&str) -> String) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        let (body, eol) = if let Some(stripped) = line.strip_suffix("\r\n") {
            (stripped, "\r\n")
        } else if let Some(stripped) = line.strip_suffix('\n') {
            (stripped, "\n")
        } else {
            (line, "")
        };
        out.push_str(&f(body));
        out.push_str(eol);
    }
    out
}

fn spaces_to_tabs_leading(body: &str) -> String {
    let spaces = body.chars().take_while(|c| *c == ' ').count();
    let rest: String = body.chars().skip(spaces).collect();
    let tabs = spaces / 4;
    let rem = spaces % 4;
    format!("{}{}{rest}", "\t".repeat(tabs), " ".repeat(rem))
}

fn selected_text(state: &EditorState) -> Option<String> {
    let (s, e) = state.tabs.active().buffer.selection()?;
    let t = state.tabs.active().buffer.slice(s, e);
    let t = t.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn resolve_selected_path(state: &EditorState) -> Option<PathBuf> {
    let t = selected_text(state)?;
    let p = PathBuf::from(&t);
    if p.exists() {
        return Some(p);
    }
    if let Some(parent) = state.tabs.active().path.as_ref().and_then(|p| p.parent()) {
        let joined = parent.join(&t);
        if joined.exists() {
            return Some(joined);
        }
    }
    None
}

fn find_matching_brace(text: &str, caret: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return None;
    }
    let try_pos = |pos: usize| -> Option<(usize, char, i32)> {
        let c = *chars.get(pos)?;
        match c {
            '(' | '[' | '{' => Some((pos, c, 1)),
            ')' | ']' | '}' => Some((pos, c, -1)),
            _ => None,
        }
    };
    let last = chars.len() - 1;
    let (start, ch, dir) = try_pos(caret.min(last)).or_else(|| caret.checked_sub(1).and_then(try_pos))?;
    let (open, close) = match ch {
        '(' | ')' => ('(', ')'),
        '[' | ']' => ('[', ']'),
        _ => ('{', '}'),
    };
    let mut depth = 0i32;
    if dir > 0 {
        for (i, &c) in chars.iter().enumerate().skip(start) {
            if c == open {
                depth += 1;
            } else if c == close {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }
    } else {
        for i in (0..=start).rev() {
            let c = chars[i];
            if c == close {
                depth += 1;
            } else if c == open {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }
    }
    None
}

fn brace_span(text: &str, caret: usize) -> Option<(usize, usize)> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return None;
    }
    let try_pos = |pos: usize| -> Option<usize> {
        let c = *chars.get(pos)?;
        if matches!(c, '(' | ')' | '[' | ']' | '{' | '}') {
            Some(pos)
        } else {
            None
        }
    };
    let last = chars.len() - 1;
    let start = try_pos(caret.min(last)).or_else(|| caret.checked_sub(1).and_then(try_pos))?;
    let other = find_matching_brace(text, start)?;
    Some((start, other))
}

fn filter_lines_by_bookmarks(state: &mut EditorState, keep_unmarked: bool) {
    let text = state.tabs.active().buffer.to_string();
    let marks = state.tabs.active().bookmarks.clone();
    let mut kept = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let marked = marks.contains(&i);
        if keep_unmarked {
            if !marked {
                kept.push(line);
            }
        } else if marked {
            kept.push(line);
        }
    }
    let mut out = kept.join("\n");
    if text.ends_with('\n') {
        out.push('\n');
    }
    state.tabs.active_mut().buffer.replace_document(&out);
    state.tabs.active_mut().bookmarks.clear();
    state.mark_text_changed();
    state.status = if keep_unmarked {
        "Removed bookmarked lines".into()
    } else {
        "Removed non-bookmarked lines".into()
    };
}

fn tab_type_key(doc: &doc::Document) -> String {
    doc.path
        .as_ref()
        .and_then(|p| p.extension())
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_else(|| doc.language.to_lowercase())
}

fn tab_mtime(doc: &doc::Document) -> u64 {
    doc.path
        .as_ref()
        .and_then(|p| std::fs::metadata(p).ok())
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn copy_selection(state: &mut EditorState, ui: &mut UiFlags) {
    if let Some((s, e)) = state.tabs.active().buffer.selection() {
        ui.pending_clipboard = Some(state.tabs.active().buffer.slice(s, e));
        state.status = "Copied".into();
    }
}

/// Human-ish feature name from an `IDM_*` id.
pub fn feature_name_from_cmd(cmd: &str) -> String {
    let raw = cmd.strip_prefix("IDM_").unwrap_or(cmd);
    raw.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn coming_soon_for(cmd: &str) -> ComingSoon {
    ComingSoon {
        cmd: cmd.to_string(),
        feature: feature_name_from_cmd(cmd),
    }
}

/// Short smile lines — pick by hashing the command so the same item feels consistent.
pub fn coming_soon_blurb(cmd: &str) -> &'static str {
    const LINES: &[&str] = &[
        "We’re polishing this one with care. Come back tomorrow — it’ll be friendlier.",
        "Not ready yet, but the elves are typing. See you tomorrow!",
        "Almost there. Sleep well; tomorrow this button gets a real job.",
        "Still in the workshop. Check again tomorrow — we owe you a smile.",
        "Good eye! This feature is on the bench. Tomorrow’s build loves you more.",
        "Patience, explorer. Tomorrow we turn this stub into magic.",
        "Noted in Ralf’s side-project notebook. Tomorrow: progress. Today: coffee.",
        "The menu is honest; the code is catching up. See you tomorrow!",
    ];
    let mut h: u32 = 0;
    for b in cmd.bytes() {
        h = h.wrapping_mul(31).wrapping_add(u32::from(b));
    }
    LINES[(h as usize) % LINES.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_commands_are_marked_implemented() {
        assert!(is_implemented("IDM_FILE_NEW"));
        assert!(is_implemented("IDM_ABOUT"));
        assert!(is_implemented("IDM_LANG_RUST"));
        assert!(is_implemented("IDM_LANG_FOO")); // language catch-all
        assert!(!is_implemented("IDM_EDIT_CLIPBOARDHISTORY"));
        assert!(!is_implemented("IDM_SETTING_PLUGINADM"));
    }
}
