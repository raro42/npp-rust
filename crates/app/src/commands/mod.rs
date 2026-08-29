//! Command dispatch for Notepad++ menu IDs (`IDM_*`).
//!
//! Split by menu area so agents can edit in parallel:
//! `file`, `edit`, `search`, `view`, `format`, `lang`, `misc`, `help`.

mod common;
mod file;
mod edit;
mod format;
mod search;
mod view;
mod lang;
mod misc;
mod help;

use crate::editor::EditorState;

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
    /// Last text this session copied via menu (for Paste Bookmarked Lines).
    pub last_copied: Option<String>,
    /// Next editor Paste replaces bookmarked lines instead of inserting.
    pub await_paste_bookmarks: bool,
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
    /// Show document summary dialog.
    pub show_summary: bool,
    /// Show document list dialog.
    pub show_doc_list: bool,
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
            | "IDM_FILE_CLOSEALL_BUT_PINNED"
            | "IDM_FILE_DELETE"
            | "IDM_FILE_SAVESESSION"
            | "IDM_FILE_LOADSESSION"
            | "IDM_FILE_PRINT"
            | "IDM_FILE_PRINTNOW"
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
            | "IDM_EDIT_INSERT_DATETIME_CUSTOMIZED"
            | "IDM_EDIT_BLOCK_COMMENT"
            | "IDM_EDIT_BLOCK_COMMENT_SET"
            | "IDM_EDIT_BLOCK_UNCOMMENT"
            | "IDM_EDIT_STREAM_COMMENT"
            | "IDM_EDIT_STREAM_UNCOMMENT"
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
            | "IDM_VIEW_TAB_SPACE"
            | "IDM_VIEW_EOL"
            | "IDM_VIEW_NPC"
            | "IDM_VIEW_NPC_CCUNIEOL"
            | "IDM_VIEW_ALL_CHARACTERS"
            | "IDM_VIEW_INDENT_GUIDE"
            | "IDM_VIEW_WRAP_SYMBOL"
            | "IDM_VIEW_WRAP"
            | "IDM_VIEW_TAB_MOVEFORWARD"
            | "IDM_VIEW_TAB_MOVEBACKWARD"
            | "IDM_VIEW_TAB_COLOUR_1"
            | "IDM_VIEW_TAB_COLOUR_2"
            | "IDM_VIEW_TAB_COLOUR_3"
            | "IDM_VIEW_TAB_COLOUR_4"
            | "IDM_VIEW_TAB_COLOUR_5"
            | "IDM_VIEW_TAB_COLOUR_NONE"
            | "IDM_VIEW_SUMMARY"
            | "IDM_VIEW_FILEBROWSER"
            | "IDM_VIEW_DOCLIST"
            | "IDM_VIEW_ZOOM_SYNC"
            | "IDM_VIEW_SYNSCROLLV"
            | "IDM_VIEW_SYNSCROLLH"
            | "IDM_EDIT_RTL"
            | "IDM_EDIT_LTR"
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
            | "IDM_SEARCH_CUTMARKEDLINES"
            | "IDM_SEARCH_PASTEMARKEDLINES"
            | "IDM_SEARCH_DELETEMARKEDLINES"
            | "IDM_SEARCH_DELETEUNMARKEDLINES"
            | "IDM_SEARCH_INVERSEMARKS"
            | "IDM_SEARCH_FINDINFILES"
            | "IDM_SEARCH_GOTONEXTFOUND"
            | "IDM_SEARCH_GOTOPREVFOUND"
            | "IDM_SEARCH_MARK"
            | "IDM_SEARCH_CHANGED_NEXT"
            | "IDM_SEARCH_CHANGED_PREV"
            | "IDM_SEARCH_CLEAR_CHANGE_HISTORY"
            | "IDM_SEARCH_FINDCHARINRANGE"
            | "IDM_SEARCH_MARKALLEXT1"
            | "IDM_SEARCH_MARKALLEXT2"
            | "IDM_SEARCH_MARKALLEXT3"
            | "IDM_SEARCH_MARKALLEXT4"
            | "IDM_SEARCH_MARKALLEXT5"
            | "IDM_SEARCH_MARKONEEXT1"
            | "IDM_SEARCH_MARKONEEXT2"
            | "IDM_SEARCH_MARKONEEXT3"
            | "IDM_SEARCH_MARKONEEXT4"
            | "IDM_SEARCH_MARKONEEXT5"
            | "IDM_SEARCH_UNMARKALLEXT1"
            | "IDM_SEARCH_UNMARKALLEXT2"
            | "IDM_SEARCH_UNMARKALLEXT3"
            | "IDM_SEARCH_UNMARKALLEXT4"
            | "IDM_SEARCH_UNMARKALLEXT5"
            | "IDM_SEARCH_CLEARALLMARKS"
            | "IDM_SEARCH_GOPREVMARKER1"
            | "IDM_SEARCH_GOPREVMARKER2"
            | "IDM_SEARCH_GOPREVMARKER3"
            | "IDM_SEARCH_GOPREVMARKER4"
            | "IDM_SEARCH_GOPREVMARKER5"
            | "IDM_SEARCH_GOPREVMARKER_DEF"
            | "IDM_SEARCH_GONEXTMARKER1"
            | "IDM_SEARCH_GONEXTMARKER2"
            | "IDM_SEARCH_GONEXTMARKER3"
            | "IDM_SEARCH_GONEXTMARKER4"
            | "IDM_SEARCH_GONEXTMARKER5"
            | "IDM_SEARCH_GONEXTMARKER_DEF"
            | "IDM_SEARCH_STYLE1TOCLIP"
            | "IDM_SEARCH_STYLE2TOCLIP"
            | "IDM_SEARCH_STYLE3TOCLIP"
            | "IDM_SEARCH_STYLE4TOCLIP"
            | "IDM_SEARCH_STYLE5TOCLIP"
            | "IDM_SEARCH_ALLSTYLESTOCLIP"
            | "IDM_SEARCH_MARKEDTOCLIP"
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
    if let Some(r) = file::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = edit::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = format::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = search::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = view::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = lang::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = misc::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = help::try_dispatch(cmd, state, ui) {
        return r;
    }
    // Fallback: language / encoding catch-alls (same as before).
    match cmd {
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

pub use common::{
    coming_soon_blurb, coming_soon_for, feature_name_from_cmd, paste_over_bookmarked_lines,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_ready_commands() {
        assert!(is_implemented("IDM_FILE_NEW"));
        assert!(is_implemented("IDM_ABOUT"));
        assert!(is_implemented("IDM_LANG_RUST"));
        assert!(is_implemented("IDM_LANG_FOO"));
        assert!(!is_implemented("IDM_SETTING_PLUGINADM"));
    }
}
