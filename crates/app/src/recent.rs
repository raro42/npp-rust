//! Recent files list and small app settings with disk persistence.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const DEFAULT_RECENT_MAX: u8 = 15;
const FILENAME: &str = "recent.txt";
const SETTINGS_FILE: &str = "settings.json";

/// Repo-relative / portable label for status (never a home absolute path).
pub const SETTINGS_REL: &str = "npp-rs/settings.json";
/// Panic log written under the process working directory.
pub const PANIC_LOG_REL: &str = "logs/panic.log";

/// Preference when opening `*.log` files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogTailOnOpen {
    /// Show a small dialog each time.
    #[default]
    Ask,
    /// Enable Monitoring (tail) immediately.
    Always,
    /// Open like a normal file; no prompt.
    Never,
}

/// Default newline for Enter on new / edited text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DefaultEol {
    #[default]
    Lf,
    Crlf,
}

impl DefaultEol {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::Crlf => "\r\n",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Lf => "LF (Unix)",
            Self::Crlf => "CRLF (Windows)",
        }
    }
}

fn default_font_size() -> f32 {
    14.0
}

fn default_show_line_numbers() -> bool {
    true
}

fn default_show_fold_margin() -> bool {
    true
}

fn default_tab_width() -> u8 {
    4
}

fn default_true() -> bool {
    true
}

fn default_theme_id() -> String {
    "dark".into()
}

fn default_recent_max() -> u8 {
    DEFAULT_RECENT_MAX
}

fn default_find_match_case() -> bool {
    true
}

fn default_shortcut_word_wrap() -> String {
    crate::shortcut_chord::DEFAULT_WORD_WRAP.into()
}

fn default_shortcut_find_next() -> String {
    crate::shortcut_chord::DEFAULT_FIND_NEXT.into()
}

fn default_shortcut_find_next_global() -> String {
    crate::shortcut_chord::DEFAULT_FIND_NEXT_GLOBAL.into()
}

fn default_shortcut_next_bookmark() -> String {
    crate::shortcut_chord::DEFAULT_NEXT_BOOKMARK.into()
}

fn default_shortcut_toggle_bookmark() -> String {
    crate::shortcut_chord::DEFAULT_TOGGLE_BOOKMARK.into()
}

fn default_shortcut_next_diff() -> String {
    crate::shortcut_chord::DEFAULT_NEXT_DIFF.into()
}

fn default_shortcut_first_diff() -> String {
    crate::shortcut_chord::DEFAULT_FIRST_DIFF.into()
}

fn default_shortcut_next_hidden_equal() -> String {
    crate::shortcut_chord::DEFAULT_NEXT_HIDDEN_EQUAL.into()
}

fn default_shortcut_first_hidden_equal() -> String {
    crate::shortcut_chord::DEFAULT_FIRST_HIDDEN_EQUAL.into()
}

fn default_shortcut_apply_compare_hunk() -> String {
    crate::shortcut_chord::DEFAULT_APPLY_COMPARE_HUNK.into()
}

fn default_shortcut_hide_equal_context() -> String {
    crate::shortcut_chord::DEFAULT_HIDE_EQUAL_CONTEXT.into()
}

fn default_shortcut_hide_equal() -> String {
    crate::shortcut_chord::DEFAULT_HIDE_EQUAL.into()
}

fn default_shortcut_compare_ignore_ws() -> String {
    crate::shortcut_chord::DEFAULT_COMPARE_IGNORE_WS.into()
}

fn default_shortcut_compare() -> String {
    crate::shortcut_chord::DEFAULT_COMPARE.into()
}

fn default_shortcut_swap_compare() -> String {
    crate::shortcut_chord::DEFAULT_SWAP_COMPARE.into()
}

fn default_shortcut_compare_to_saved() -> String {
    crate::shortcut_chord::DEFAULT_COMPARE_TO_SAVED.into()
}

fn default_shortcut_copy_compare_diff() -> String {
    crate::shortcut_chord::DEFAULT_COPY_COMPARE_DIFF.into()
}

fn default_shortcut_copy_compare_summary() -> String {
    crate::shortcut_chord::DEFAULT_COPY_COMPARE_SUMMARY.into()
}

fn default_shortcut_copy_compare_hunk() -> String {
    crate::shortcut_chord::DEFAULT_COPY_COMPARE_HUNK.into()
}

fn default_shortcut_word_jump() -> String {
    crate::shortcut_chord::DEFAULT_WORD_JUMP.into()
}

fn default_shortcut_goto_line() -> String {
    crate::shortcut_chord::DEFAULT_GOTO_LINE.into()
}

fn default_shortcut_duplicate_line() -> String {
    crate::shortcut_chord::DEFAULT_DUPLICATE_LINE.into()
}

fn default_shortcut_delete_line() -> String {
    crate::shortcut_chord::DEFAULT_DELETE_LINE.into()
}

fn default_shortcut_indent() -> String {
    crate::shortcut_chord::DEFAULT_INDENT.into()
}

fn default_shortcut_outdent() -> String {
    crate::shortcut_chord::DEFAULT_OUTDENT.into()
}

fn default_shortcut_format_document() -> String {
    crate::shortcut_chord::DEFAULT_FORMAT_DOCUMENT.into()
}

fn default_shortcut_close_tab() -> String {
    crate::shortcut_chord::DEFAULT_CLOSE_TAB.into()
}

fn default_shortcut_next_tab() -> String {
    crate::shortcut_chord::DEFAULT_NEXT_TAB.into()
}

fn default_shortcut_new() -> String {
    crate::shortcut_chord::DEFAULT_NEW.into()
}

fn default_shortcut_open() -> String {
    crate::shortcut_chord::DEFAULT_OPEN.into()
}

fn default_shortcut_save() -> String {
    crate::shortcut_chord::DEFAULT_SAVE.into()
}

fn default_shortcut_save_as() -> String {
    crate::shortcut_chord::DEFAULT_SAVE_AS.into()
}

fn default_shortcut_save_all() -> String {
    crate::shortcut_chord::DEFAULT_SAVE_ALL.into()
}

fn default_shortcut_print() -> String {
    crate::shortcut_chord::DEFAULT_PRINT.into()
}

fn default_shortcut_tab_indent() -> String {
    crate::shortcut_chord::DEFAULT_TAB_INDENT.into()
}

fn default_shortcut_shift_tab_outdent() -> String {
    crate::shortcut_chord::DEFAULT_SHIFT_TAB_OUTDENT.into()
}

fn default_shortcut_fold_all() -> String {
    crate::shortcut_chord::DEFAULT_FOLD_ALL.into()
}

fn default_shortcut_fold_current() -> String {
    crate::shortcut_chord::DEFAULT_FOLD_CURRENT.into()
}

fn default_shortcut_matching_brace() -> String {
    crate::shortcut_chord::DEFAULT_MATCHING_BRACE.into()
}

fn default_shortcut_move_line() -> String {
    crate::shortcut_chord::DEFAULT_MOVE_LINE.into()
}

fn default_shortcut_toggle_comment() -> String {
    crate::shortcut_chord::DEFAULT_TOGGLE_COMMENT.into()
}

fn default_shortcut_find() -> String {
    crate::shortcut_chord::DEFAULT_FIND.into()
}

fn default_shortcut_close_find() -> String {
    crate::shortcut_chord::DEFAULT_CLOSE_FIND.into()
}

fn default_shortcut_replace() -> String {
    crate::shortcut_chord::DEFAULT_REPLACE.into()
}

fn default_shortcut_replace_alt() -> String {
    crate::shortcut_chord::DEFAULT_REPLACE_ALT.into()
}

fn default_shortcut_select_all() -> String {
    crate::shortcut_chord::DEFAULT_SELECT_ALL.into()
}

fn default_shortcut_undo() -> String {
    crate::shortcut_chord::DEFAULT_UNDO.into()
}

fn default_shortcut_redo() -> String {
    crate::shortcut_chord::DEFAULT_REDO.into()
}

fn default_shortcut_zoom_in() -> String {
    crate::shortcut_chord::DEFAULT_ZOOM_IN.into()
}

fn default_shortcut_zoom_out() -> String {
    crate::shortcut_chord::DEFAULT_ZOOM_OUT.into()
}

fn default_shortcut_zoom_restore() -> String {
    crate::shortcut_chord::DEFAULT_ZOOM_RESTORE.into()
}

fn default_shortcut_toggle_log_tail() -> String {
    crate::shortcut_chord::DEFAULT_TOGGLE_LOG_TAIL.into()
}

fn default_shortcut_reload() -> String {
    crate::shortcut_chord::DEFAULT_RELOAD.into()
}

fn default_compare_hide_equal_context() -> u8 {
    u8::try_from(crate::diff::COMPARE_HIDE_EQUAL_CONTEXT).unwrap_or(3)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub log_tail_on_open: LogTailOnOpen,
    /// Editor monospace size (also used as zoom restore target).
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    /// Draw line numbers in the editor gutter.
    #[serde(default = "default_show_line_numbers")]
    pub show_line_numbers: bool,
    /// Draw fold markers (−/+) in the gutter; click toggles a fold.
    #[serde(default = "default_show_fold_margin")]
    pub show_fold_margin: bool,
    /// Spaces inserted for Tab / indent (2..=8).
    #[serde(default = "default_tab_width")]
    pub tab_width: u8,
    /// Soft word wrap (session + Preferences).
    #[serde(default)]
    pub word_wrap: bool,
    /// Show space/tab glyphs (View → Show Symbol → Show White Space and TAB).
    #[serde(default)]
    pub show_whitespace: bool,
    /// Show end-of-line marks.
    #[serde(default)]
    pub show_eol: bool,
    /// Show non-printing / control characters.
    #[serde(default)]
    pub show_npc: bool,
    /// Draw vertical indent guides.
    #[serde(default)]
    pub show_indent_guide: bool,
    /// Status bar: show language id.
    #[serde(default = "default_true")]
    pub status_show_lang: bool,
    /// Status bar: show character count.
    #[serde(default = "default_true")]
    pub status_show_chars: bool,
    /// Theme id: `dark`, `light`, or `file:<name>.json`.
    #[serde(default = "default_theme_id")]
    pub theme_id: String,
    /// Extra gutter width in pixels (0..=40).
    #[serde(default)]
    pub gutter_extra: u8,
    /// Blink the text caret.
    #[serde(default = "default_true")]
    pub caret_blink: bool,
    /// Newline inserted by Enter.
    #[serde(default)]
    pub default_eol: DefaultEol,
    /// Max recent-file entries (5..=40).
    #[serde(default = "default_recent_max")]
    pub recent_max: u8,
    /// Reopen last session paths on launch (when argv has no files).
    #[serde(default)]
    pub restore_session: bool,
    /// Find: match case.
    #[serde(default = "default_find_match_case")]
    pub find_match_case: bool,
    /// Find: whole word only.
    #[serde(default)]
    pub find_whole_word: bool,
    /// Find / replace only inside the captured selection.
    #[serde(default)]
    pub find_in_selection: bool,
    /// Find: wrap from the other end when Next/Prev hits the bound.
    #[serde(default = "default_true")]
    pub find_wrap: bool,
    /// Find: treat the query as a linear-time regular expression.
    #[serde(default)]
    pub find_regex: bool,
    /// Last Find query (restored into the find bar).
    #[serde(default)]
    pub find_query: String,
    /// Last Replace string.
    #[serde(default)]
    pub replace_with: String,
    /// Compare: treat runs of whitespace as equal.
    #[serde(default)]
    pub compare_ignore_ws: bool,
    /// Compare: treat letter case as equal.
    #[serde(default)]
    pub compare_ignore_case: bool,
    /// Compare: skip blank / whitespace-only lines in the LCS.
    #[serde(default)]
    pub compare_ignore_blank: bool,
    /// Compare: hide Equal-tagged lines in both panes (change lines stay visible).
    #[serde(default)]
    pub compare_hide_equal: bool,
    /// Equal lines kept visible on each side of a change when hide-equal is on (0..=10).
    #[serde(default = "default_compare_hide_equal_context")]
    pub compare_hide_equal_context: u8,
    /// Last project panel folder (absolute or relative path string).
    #[serde(default)]
    pub workspace_root: String,
    /// Project panel name filter (substring, case-insensitive).
    #[serde(default)]
    pub project_filter: String,
    /// Find in Files: include globs (comma/semicolon; empty = all).
    #[serde(default)]
    pub find_files_include: String,
    /// Find in Files: exclude names/globs (comma/semicolon).
    #[serde(default = "crate::search_util::default_find_files_exclude")]
    pub find_files_exclude: String,
    /// Before overwrite save, copy the on-disk file into `npp-rs/backup/`.
    #[serde(default)]
    pub backup_on_save: bool,
    /// Autosave dirty tabs that have a path every N seconds (`0` = off).
    #[serde(default)]
    pub autosave_interval_secs: u32,
    /// Word-wrap toggle chord (`Alt+Z` default). See Preferences / Shortcut Mapper.
    #[serde(default = "default_shortcut_word_wrap")]
    pub shortcut_word_wrap: String,
    /// Find-next chord (`F3` default). Shift + same key is find previous.
    #[serde(default = "default_shortcut_find_next")]
    pub shortcut_find_next: String,
    /// Global find-next chord (`Cmd+G` default). Shift + same key is find previous.
    #[serde(default = "default_shortcut_find_next_global")]
    pub shortcut_find_next_global: String,
    /// Next-bookmark chord (`F2` default). Shift + same key is previous bookmark.
    #[serde(default = "default_shortcut_next_bookmark")]
    pub shortcut_next_bookmark: String,
    /// Toggle-bookmark chord (`Cmd+F2` default).
    #[serde(default = "default_shortcut_toggle_bookmark")]
    pub shortcut_toggle_bookmark: String,
    /// Next-compare-diff chord (`F7` default). Shift + same key is previous difference.
    #[serde(default = "default_shortcut_next_diff")]
    pub shortcut_next_diff: String,
    /// First-compare-diff chord (`Cmd+F7` default). Shift + same key is last difference.
    #[serde(default = "default_shortcut_first_diff")]
    pub shortcut_first_diff: String,
    /// Next-hidden-equal chord (`Alt+F7` default). Shift + same key is previous hidden equal.
    #[serde(default = "default_shortcut_next_hidden_equal")]
    pub shortcut_next_hidden_equal: String,
    /// First-hidden-equal chord (`Cmd+Alt+F7` default). Shift + same key is last hidden equal.
    #[serde(default = "default_shortcut_first_hidden_equal")]
    pub shortcut_first_hidden_equal: String,
    /// Apply-compare-hunk-from chord (`Cmd+Alt+Left` default). Opposite arrow applies to other; Shift applies all.
    #[serde(default = "default_shortcut_apply_compare_hunk")]
    pub shortcut_apply_compare_hunk: String,
    /// Hide-equal-context increase chord (`Alt+]` default). Opposite bracket decreases.
    #[serde(default = "default_shortcut_hide_equal_context")]
    pub shortcut_hide_equal_context: String,
    /// Hide-unchanged-lines toggle chord (`Alt+H` default).
    #[serde(default = "default_shortcut_hide_equal")]
    pub shortcut_hide_equal: String,
    /// Ignore-whitespace-differences toggle chord (`Alt+W` default). Shift flips to Ignore Case.
    #[serde(default = "default_shortcut_compare_ignore_ws")]
    pub shortcut_compare_ignore_ws: String,
    /// Start-compare chord (`Alt+D` default). Shift + same key is Clear Compare.
    #[serde(default = "default_shortcut_compare")]
    pub shortcut_compare: String,
    /// Swap-compare-sides chord (`Alt+S` default).
    #[serde(default = "default_shortcut_swap_compare")]
    pub shortcut_swap_compare: String,
    /// Compare-to-saved chord (`Alt+Shift+S` default).
    #[serde(default = "default_shortcut_compare_to_saved")]
    pub shortcut_compare_to_saved: String,
    /// Copy-compare-diff chord (`Alt+C` default; Shift flips to Open Compare Diff).
    #[serde(default = "default_shortcut_copy_compare_diff")]
    pub shortcut_copy_compare_diff: String,
    /// Copy-compare-summary chord (`Alt+Y` default; Shift flips to Open Compare Summary).
    #[serde(default = "default_shortcut_copy_compare_summary")]
    pub shortcut_copy_compare_summary: String,
    /// Copy-compare-hunk chord (`Alt+K` default; Shift flips to Open Compare Hunk).
    #[serde(default = "default_shortcut_copy_compare_hunk")]
    pub shortcut_copy_compare_hunk: String,
    /// Word-jump-back chord (`Alt+Left` default). Opposite arrow jumps forward; Shift extends.
    #[serde(default = "default_shortcut_word_jump")]
    pub shortcut_word_jump: String,
    /// Go-to-line chord (`Cmd+L` default).
    #[serde(default = "default_shortcut_goto_line")]
    pub shortcut_goto_line: String,
    /// Duplicate-line chord (`Cmd+D` default).
    #[serde(default = "default_shortcut_duplicate_line")]
    pub shortcut_duplicate_line: String,
    /// Delete-line chord (`Cmd+Shift+L` default).
    #[serde(default = "default_shortcut_delete_line")]
    pub shortcut_delete_line: String,
    /// Indent-lines chord (`Cmd+]` default).
    #[serde(default = "default_shortcut_indent")]
    pub shortcut_indent: String,
    /// Outdent-lines chord (`Cmd+[` default).
    #[serde(default = "default_shortcut_outdent")]
    pub shortcut_outdent: String,
    /// Format-document chord (`Cmd+Shift+I` default).
    #[serde(default = "default_shortcut_format_document")]
    pub shortcut_format_document: String,
    /// Close-tab chord (`Cmd+W` default).
    #[serde(default = "default_shortcut_close_tab")]
    pub shortcut_close_tab: String,
    /// Next-tab chord (`Cmd+Tab` default; Shift flips to previous).
    #[serde(default = "default_shortcut_next_tab")]
    pub shortcut_next_tab: String,
    /// New-file chord (`Cmd+N` default).
    #[serde(default = "default_shortcut_new")]
    pub shortcut_new: String,
    /// Open-file chord (`Cmd+O` default).
    #[serde(default = "default_shortcut_open")]
    pub shortcut_open: String,
    /// Save chord (`Cmd+S` default).
    #[serde(default = "default_shortcut_save")]
    pub shortcut_save: String,
    /// Save-as chord (`Cmd+Shift+S` default).
    #[serde(default = "default_shortcut_save_as")]
    pub shortcut_save_as: String,
    /// Save-all chord (`Cmd+Alt+S` default).
    #[serde(default = "default_shortcut_save_all")]
    pub shortcut_save_all: String,
    /// Print chord (`Cmd+P` default).
    #[serde(default = "default_shortcut_print")]
    pub shortcut_print: String,
    /// Tab indent chord (`Tab` default; inserts spaces at caret / multi-carets).
    #[serde(default = "default_shortcut_tab_indent")]
    pub shortcut_tab_indent: String,
    /// Shift+Tab outdent chord (`Shift+Tab` default; alternate to `shortcut_outdent`).
    #[serde(default = "default_shortcut_shift_tab_outdent")]
    pub shortcut_shift_tab_outdent: String,
    /// Fold-all chord (`Alt+0` default; Shift flips to Unfold All).
    #[serde(default = "default_shortcut_fold_all")]
    pub shortcut_fold_all: String,
    /// Fold-current chord (`Alt+F` default; Shift flips to Unfold Current).
    #[serde(default = "default_shortcut_fold_current")]
    pub shortcut_fold_current: String,
    /// Matching-brace chord (`Cmd+B` default; Shift flips to Select matching braces).
    #[serde(default = "default_shortcut_matching_brace")]
    pub shortcut_matching_brace: String,
    /// Move-line-up chord (`Cmd+Shift+Up` default; opposite arrow / Shift moves down).
    #[serde(default = "default_shortcut_move_line")]
    pub shortcut_move_line: String,
    /// Toggle line-comment chord (`Cmd+/` default; Shift flips to Block Comment).
    #[serde(default = "default_shortcut_toggle_comment")]
    pub shortcut_toggle_comment: String,
    /// Find-bar chord (`Cmd+F` default).
    #[serde(default = "default_shortcut_find")]
    pub shortcut_find: String,
    /// Close Find/Replace chord (`Escape` default).
    #[serde(default = "default_shortcut_close_find")]
    pub shortcut_close_find: String,
    /// Replace-bar chord (`Cmd+H` default).
    #[serde(default = "default_shortcut_replace")]
    pub shortcut_replace: String,
    /// Alternate replace-bar chord (`Cmd+Shift+F` default).
    #[serde(default = "default_shortcut_replace_alt")]
    pub shortcut_replace_alt: String,
    /// Select-all chord (`Cmd+A` default).
    #[serde(default = "default_shortcut_select_all")]
    pub shortcut_select_all: String,
    /// Undo chord (`Cmd+Z` default; Shift flips to redo).
    #[serde(default = "default_shortcut_undo")]
    pub shortcut_undo: String,
    /// Redo alternate chord (`Cmd+Y` default; Shift+undo also redo).
    #[serde(default = "default_shortcut_redo")]
    pub shortcut_redo: String,
    /// Zoom-in chord (`Cmd+=` default).
    #[serde(default = "default_shortcut_zoom_in")]
    pub shortcut_zoom_in: String,
    /// Zoom-out chord (`Cmd+-` default).
    #[serde(default = "default_shortcut_zoom_out")]
    pub shortcut_zoom_out: String,
    /// Zoom-restore chord (`Cmd+0` default). Mouse wheel stays hard-wired.
    #[serde(default = "default_shortcut_zoom_restore")]
    pub shortcut_zoom_restore: String,
    /// Toggle log-tail follow chord (`Cmd+Shift+T` default).
    #[serde(default = "default_shortcut_toggle_log_tail")]
    pub shortcut_toggle_log_tail: String,
    /// Reload-from-disk chord (`Cmd+R` default).
    #[serde(default = "default_shortcut_reload")]
    pub shortcut_reload: String,
    /// Unknown keys from disk. Kept so a save does not drop hand-edited or future fields.
    #[serde(flatten, default, skip_serializing_if = "serde_json::Map::is_empty")]
    extra: serde_json::Map<String, serde_json::Value>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            log_tail_on_open: LogTailOnOpen::Ask,
            font_size: default_font_size(),
            show_line_numbers: default_show_line_numbers(),
            show_fold_margin: default_show_fold_margin(),
            tab_width: default_tab_width(),
            word_wrap: false,
            show_whitespace: false,
            show_eol: false,
            show_npc: false,
            show_indent_guide: false,
            status_show_lang: true,
            status_show_chars: true,
            theme_id: default_theme_id(),
            gutter_extra: 0,
            caret_blink: true,
            default_eol: DefaultEol::Lf,
            recent_max: default_recent_max(),
            restore_session: false,
            find_match_case: true,
            find_whole_word: false,
            find_in_selection: false,
            find_wrap: true,
            find_regex: false,
            find_query: String::new(),
            replace_with: String::new(),
            compare_ignore_ws: false,
            compare_ignore_case: false,
            compare_ignore_blank: false,
            compare_hide_equal: false,
            compare_hide_equal_context: default_compare_hide_equal_context(),
            workspace_root: String::new(),
            project_filter: String::new(),
            find_files_include: String::new(),
            find_files_exclude: crate::search_util::default_find_files_exclude(),
            backup_on_save: false,
            autosave_interval_secs: 0,
            shortcut_word_wrap: default_shortcut_word_wrap(),
            shortcut_find_next: default_shortcut_find_next(),
            shortcut_find_next_global: default_shortcut_find_next_global(),
            shortcut_next_bookmark: default_shortcut_next_bookmark(),
            shortcut_toggle_bookmark: default_shortcut_toggle_bookmark(),
            shortcut_next_diff: default_shortcut_next_diff(),
            shortcut_first_diff: default_shortcut_first_diff(),
            shortcut_next_hidden_equal: default_shortcut_next_hidden_equal(),
            shortcut_first_hidden_equal: default_shortcut_first_hidden_equal(),
            shortcut_apply_compare_hunk: default_shortcut_apply_compare_hunk(),
            shortcut_hide_equal_context: default_shortcut_hide_equal_context(),
            shortcut_hide_equal: default_shortcut_hide_equal(),
            shortcut_compare_ignore_ws: default_shortcut_compare_ignore_ws(),
            shortcut_compare: default_shortcut_compare(),
            shortcut_swap_compare: default_shortcut_swap_compare(),
            shortcut_compare_to_saved: default_shortcut_compare_to_saved(),
            shortcut_copy_compare_diff: default_shortcut_copy_compare_diff(),
            shortcut_copy_compare_summary: default_shortcut_copy_compare_summary(),
            shortcut_copy_compare_hunk: default_shortcut_copy_compare_hunk(),
            shortcut_word_jump: default_shortcut_word_jump(),
            shortcut_goto_line: default_shortcut_goto_line(),
            shortcut_duplicate_line: default_shortcut_duplicate_line(),
            shortcut_delete_line: default_shortcut_delete_line(),
            shortcut_indent: default_shortcut_indent(),
            shortcut_outdent: default_shortcut_outdent(),
            shortcut_format_document: default_shortcut_format_document(),
            shortcut_close_tab: default_shortcut_close_tab(),
            shortcut_next_tab: default_shortcut_next_tab(),
            shortcut_new: default_shortcut_new(),
            shortcut_open: default_shortcut_open(),
            shortcut_save: default_shortcut_save(),
            shortcut_save_as: default_shortcut_save_as(),
            shortcut_save_all: default_shortcut_save_all(),
            shortcut_print: default_shortcut_print(),
            shortcut_tab_indent: default_shortcut_tab_indent(),
            shortcut_shift_tab_outdent: default_shortcut_shift_tab_outdent(),
            shortcut_fold_all: default_shortcut_fold_all(),
            shortcut_fold_current: default_shortcut_fold_current(),
            shortcut_matching_brace: default_shortcut_matching_brace(),
            shortcut_move_line: default_shortcut_move_line(),
            shortcut_toggle_comment: default_shortcut_toggle_comment(),
            shortcut_find: default_shortcut_find(),
            shortcut_close_find: default_shortcut_close_find(),
            shortcut_replace: default_shortcut_replace(),
            shortcut_replace_alt: default_shortcut_replace_alt(),
            shortcut_select_all: default_shortcut_select_all(),
            shortcut_undo: default_shortcut_undo(),
            shortcut_redo: default_shortcut_redo(),
            shortcut_zoom_in: default_shortcut_zoom_in(),
            shortcut_zoom_out: default_shortcut_zoom_out(),
            shortcut_zoom_restore: default_shortcut_zoom_restore(),
            shortcut_toggle_log_tail: default_shortcut_toggle_log_tail(),
            shortcut_reload: default_shortcut_reload(),
            extra: serde_json::Map::new(),
        }
    }
}

impl AppSettings {
    pub fn recent_limit(&self) -> usize {
        (self.recent_max.clamp(5, 40)) as usize
    }

    /// Equal context lines kept when Hide Unchanged Lines is on (clamped 0..=10).
    pub fn compare_hide_equal_context_lines(&self) -> usize {
        usize::from(self.compare_hide_equal_context.clamp(0, 10))
    }

    /// Effective autosave period in seconds (`0` = disabled). Non-zero values clamp to 15..=900.
    pub fn autosave_secs(&self) -> u64 {
        match self.autosave_interval_secs {
            0 => 0,
            n => u64::from(n.clamp(15, 900)),
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let Ok(path) = settings_store_path() else {
            return Self::default();
        };
        let Ok(text) = fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&text).unwrap_or_default()
    }

    pub fn save(&self) {
        let Ok(path) = settings_store_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let Ok(text) = serde_json::to_string_pretty(self) else {
            return;
        };
        let _ = fs::write(path, text);
    }
}

#[derive(Debug, Clone, Default)]
pub struct RecentFiles {
    paths: Vec<PathBuf>,
}

impl RecentFiles {
    /// Load recent paths capped by Preferences `recent_max` (clamped 5..=40).
    pub fn load_limited(max: usize) -> Self {
        let mut recent = Self::default();
        let Ok(path) = recent_store_path() else {
            return recent;
        };
        let Ok(file) = fs::File::open(&path) else {
            return recent;
        };
        recent.extend_from_lines(BufReader::new(file).lines().map_while(Result::ok), max);
        recent
    }

    fn extend_from_lines<I>(&mut self, lines: I, max: usize)
    where
        I: IntoIterator<Item = String>,
    {
        let max = max.clamp(5, 40);
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            self.paths.push(PathBuf::from(line));
            if self.paths.len() >= max {
                break;
            }
        }
    }

    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }

    /// Push path to the front with a custom cap from Preferences.
    pub fn touch_limited(&mut self, path: &Path, max: usize) {
        let path = canonicalize_best_effort(path);
        let max = max.clamp(5, 40);
        self.paths.retain(|p| p != &path);
        self.paths.insert(0, path);
        self.paths.truncate(max);
        self.save();
    }

    pub fn remove(&mut self, path: &Path) {
        let path = canonicalize_best_effort(path);
        self.paths.retain(|p| p != &path);
        self.save();
    }

    pub fn clear(&mut self) {
        self.paths.clear();
        self.save();
    }

    fn save(&self) {
        let Ok(path) = recent_store_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let Ok(mut file) = fs::File::create(&path) else {
            return;
        };
        for p in &self.paths {
            let _ = writeln!(file, "{}", p.display());
        }
    }
}

fn canonicalize_best_effort(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn recent_store_path() -> Result<PathBuf, ()> {
    let base = config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(FILENAME))
}

fn settings_store_path() -> Result<PathBuf, ()> {
    let base = config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(SETTINGS_FILE))
}

/// True when the path looks like a log file (`*.log`).
pub fn is_log_path(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("log"))
        .unwrap_or(false)
}

/// File name only — safe for status bar (no home absolute path).
pub fn short_path_label(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// Config base dir (Application Support / APPDATA / XDG).
pub fn config_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")?;
        Some(PathBuf::from(home).join("Library/Application Support"))
    }
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var_os("APPDATA")?;
        Some(PathBuf::from(appdata))
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
            Some(PathBuf::from(xdg))
        } else {
            let home = std::env::var_os("HOME")?;
            Some(PathBuf::from(home).join(".config"))
        }
    }
}

/// Short label for menus: `name — parent` when useful.
pub fn recent_label(path: &Path) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    if let Some(parent) = path.parent() {
        let parent = parent.display().to_string();
        if !parent.is_empty() && parent != "." {
            // Keep menu readable: truncate long parents from the left.
            let parent = if parent.len() > 48 {
                format!("…{}", &parent[parent.len() - 47..])
            } else {
                parent
            };
            return format!("{name}  —  {parent}");
        }
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_lines<I, S>(lines: I, max: usize) -> RecentFiles
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut recent = RecentFiles::default();
        recent.extend_from_lines(lines.into_iter().map(|s| s.as_ref().to_string()), max);
        recent
    }

    #[test]
    fn recent_from_lines_honors_preference_cap() {
        let lines: Vec<String> = (1..=20).map(|i| format!("f{i}.txt")).collect();
        let loaded = from_lines(lines.clone(), 25);
        assert_eq!(loaded.paths().len(), 20);
        assert_eq!(loaded.paths()[0], PathBuf::from("f1.txt"));
        assert_eq!(loaded.paths()[19], PathBuf::from("f20.txt"));

        let capped = from_lines(lines, 10);
        assert_eq!(capped.paths().len(), 10);
        assert_eq!(capped.paths()[9], PathBuf::from("f10.txt"));
    }

    #[test]
    fn recent_from_lines_clamps_max_floor() {
        // Preferences clamp is 5..=40; asking for 1 still keeps five.
        let lines: Vec<String> = (1..=8).map(|i| format!("file{i}")).collect();
        let loaded = from_lines(lines, 1);
        assert_eq!(loaded.paths().len(), 5);
    }

    #[test]
    fn recent_from_lines_skips_blank() {
        let loaded = from_lines(["a", "", "  ", "b"], 10);
        assert_eq!(loaded.paths(), &[PathBuf::from("a"), PathBuf::from("b")]);
    }

    #[test]
    fn recent_limit_matches_settings_clamp() {
        let s25 = AppSettings {
            recent_max: 25,
            ..Default::default()
        };
        assert_eq!(s25.recent_limit(), 25);
        let s3 = AppSettings {
            recent_max: 3,
            ..Default::default()
        };
        assert_eq!(s3.recent_limit(), 5);
        let s99 = AppSettings {
            recent_max: 99,
            ..Default::default()
        };
        assert_eq!(s99.recent_limit(), 40);
    }

    #[test]
    fn view_symbol_flags_round_trip_json() {
        let s = AppSettings {
            show_whitespace: true,
            show_eol: true,
            show_npc: true,
            show_indent_guide: true,
            ..Default::default()
        };
        let text = serde_json::to_string(&s).expect("serialize");
        let back: AppSettings = serde_json::from_str(&text).expect("deserialize");
        assert!(back.show_whitespace);
        assert!(back.show_eol);
        assert!(back.show_npc);
        assert!(back.show_indent_guide);

        let defaults: AppSettings = serde_json::from_str("{}").expect("empty object");
        assert!(!defaults.show_whitespace);
        assert!(!defaults.show_eol);
        assert!(!defaults.show_npc);
        assert!(!defaults.show_indent_guide);
    }

    #[test]
    fn unknown_settings_keys_survive_serialize() {
        let loaded: AppSettings =
            serde_json::from_str(r#"{"word_wrap":true,"future_flag":true,"custom_note":"keep"}"#)
                .expect("deserialize with extras");
        assert!(loaded.word_wrap);
        assert_eq!(loaded.extra.len(), 2);
        let text = serde_json::to_string(&loaded).expect("serialize");
        let value: serde_json::Value = serde_json::from_str(&text).expect("json");
        assert_eq!(value["word_wrap"], serde_json::Value::Bool(true));
        assert_eq!(value["future_flag"], serde_json::Value::Bool(true));
        assert_eq!(
            value["custom_note"],
            serde_json::Value::String("keep".into())
        );
        assert!(value.get("extra").is_none());
    }

    #[test]
    fn known_settings_keys_round_trip_through_json() {
        let original = AppSettings {
            font_size: 18.0,
            show_line_numbers: false,
            tab_width: 2,
            restore_session: true,
            find_query: "needle".into(),
            replace_with: "hay".into(),
            compare_ignore_ws: true,
            compare_ignore_case: true,
            compare_ignore_blank: true,
            compare_hide_equal: true,
            compare_hide_equal_context: 5,
            backup_on_save: true,
            autosave_interval_secs: 60,
            shortcut_find_next: "Ctrl+F3".into(),
            shortcut_find_next_global: "Ctrl+Alt+G".into(),
            shortcut_next_bookmark: "Ctrl+F2".into(),
            shortcut_toggle_bookmark: "Ctrl+Shift+F2".into(),
            shortcut_next_diff: "Ctrl+F7".into(),
            shortcut_first_diff: "Ctrl+Alt+F7".into(),
            shortcut_next_hidden_equal: "Ctrl+Alt+H".into(),
            shortcut_first_hidden_equal: "Ctrl+Alt+E".into(),
            shortcut_apply_compare_hunk: "Ctrl+Alt+Left".into(),
            shortcut_hide_equal_context: "Ctrl+Alt+]".into(),
            shortcut_hide_equal: "Ctrl+Alt+U".into(),
            shortcut_compare_ignore_ws: "Ctrl+Alt+W".into(),
            shortcut_compare: "Ctrl+Alt+D".into(),
            shortcut_swap_compare: "Ctrl+Alt+Shift+S".into(),
            shortcut_compare_to_saved: "Ctrl+Alt+Shift+V".into(),
            shortcut_copy_compare_diff: "Ctrl+Alt+C".into(),
            shortcut_copy_compare_summary: "Ctrl+Alt+Y".into(),
            shortcut_copy_compare_hunk: "Ctrl+Alt+K".into(),
            shortcut_word_jump: "Ctrl+Alt+Left".into(),
            shortcut_goto_line: "Ctrl+Shift+G".into(),
            shortcut_duplicate_line: "Ctrl+Shift+D".into(),
            shortcut_delete_line: "Ctrl+Shift+K".into(),
            shortcut_indent: "Ctrl+Shift+]".into(),
            shortcut_outdent: "Ctrl+Shift+[".into(),
            shortcut_format_document: "Ctrl+Alt+I".into(),
            shortcut_close_tab: "Ctrl+Shift+W".into(),
            shortcut_next_tab: "Ctrl+Alt+Tab".into(),
            shortcut_new: "Ctrl+Alt+N".into(),
            shortcut_open: "Ctrl+Alt+O".into(),
            shortcut_save: "Ctrl+Alt+S".into(),
            shortcut_save_as: "Ctrl+Alt+Shift+S".into(),
            shortcut_save_all: "Ctrl+Shift+A".into(),
            shortcut_print: "Ctrl+Alt+P".into(),
            shortcut_tab_indent: "Ctrl+I".into(),
            shortcut_shift_tab_outdent: "Ctrl+Shift+Tab".into(),
            shortcut_fold_all: "Ctrl+Alt+0".into(),
            shortcut_fold_current: "Ctrl+Alt+F".into(),
            shortcut_matching_brace: "Ctrl+Alt+B".into(),
            shortcut_move_line: "Ctrl+Alt+Up".into(),
            shortcut_toggle_comment: "Ctrl+Alt+/".into(),
            shortcut_find: "Ctrl+Alt+G".into(),
            shortcut_close_find: "Ctrl+Alt+Escape".into(),
            shortcut_replace: "Ctrl+Alt+H".into(),
            shortcut_replace_alt: "Ctrl+Alt+Shift+F".into(),
            shortcut_select_all: "Ctrl+Alt+A".into(),
            shortcut_undo: "Ctrl+Alt+Z".into(),
            shortcut_redo: "Ctrl+Alt+Y".into(),
            shortcut_zoom_in: "Ctrl+Alt+=".into(),
            shortcut_zoom_out: "Ctrl+Alt+-".into(),
            shortcut_zoom_restore: "Ctrl+Alt+0".into(),
            shortcut_toggle_log_tail: "Ctrl+Alt+T".into(),
            ..Default::default()
        };
        let text = serde_json::to_string_pretty(&original).expect("serialize");
        let back: AppSettings = serde_json::from_str(&text).expect("deserialize");
        let again = serde_json::to_string_pretty(&back).expect("serialize again");
        assert_eq!(text, again);
        assert_eq!(back.font_size, 18.0);
        assert!(!back.show_line_numbers);
        assert_eq!(back.tab_width, 2);
        assert!(back.restore_session);
        assert_eq!(back.find_query, "needle");
        assert_eq!(back.replace_with, "hay");
        assert!(back.compare_ignore_ws);
        assert!(back.compare_ignore_case);
        assert!(back.compare_ignore_blank);
        assert!(back.compare_hide_equal);
        assert_eq!(back.compare_hide_equal_context, 5);
        assert_eq!(back.compare_hide_equal_context_lines(), 5);
        assert!(back.backup_on_save);
        assert_eq!(back.autosave_interval_secs, 60);
        assert_eq!(back.shortcut_find_next, "Ctrl+F3");
        assert_eq!(back.shortcut_find_next_global, "Ctrl+Alt+G");
        assert_eq!(back.shortcut_next_bookmark, "Ctrl+F2");
        assert_eq!(back.shortcut_toggle_bookmark, "Ctrl+Shift+F2");
        assert_eq!(back.shortcut_next_diff, "Ctrl+F7");
        assert_eq!(back.shortcut_first_diff, "Ctrl+Alt+F7");
        assert_eq!(back.shortcut_next_hidden_equal, "Ctrl+Alt+H");
        assert_eq!(back.shortcut_first_hidden_equal, "Ctrl+Alt+E");
        assert_eq!(back.shortcut_apply_compare_hunk, "Ctrl+Alt+Left");
        assert_eq!(back.shortcut_hide_equal_context, "Ctrl+Alt+]");
        assert_eq!(back.shortcut_hide_equal, "Ctrl+Alt+U");
        assert_eq!(back.shortcut_compare_ignore_ws, "Ctrl+Alt+W");
        assert_eq!(back.shortcut_compare, "Ctrl+Alt+D");
        assert_eq!(back.shortcut_swap_compare, "Ctrl+Alt+Shift+S");
        assert_eq!(back.shortcut_compare_to_saved, "Ctrl+Alt+Shift+V");
        assert_eq!(back.shortcut_copy_compare_diff, "Ctrl+Alt+C");
        assert_eq!(back.shortcut_copy_compare_summary, "Ctrl+Alt+Y");
        assert_eq!(back.shortcut_copy_compare_hunk, "Ctrl+Alt+K");
        assert_eq!(back.shortcut_word_jump, "Ctrl+Alt+Left");
        assert_eq!(back.shortcut_goto_line, "Ctrl+Shift+G");
        assert_eq!(back.shortcut_duplicate_line, "Ctrl+Shift+D");
        assert_eq!(back.shortcut_delete_line, "Ctrl+Shift+K");
        assert_eq!(back.shortcut_indent, "Ctrl+Shift+]");
        assert_eq!(back.shortcut_outdent, "Ctrl+Shift+[");
        assert_eq!(back.shortcut_format_document, "Ctrl+Alt+I");
        assert_eq!(back.shortcut_close_tab, "Ctrl+Shift+W");
        assert_eq!(back.shortcut_next_tab, "Ctrl+Alt+Tab");
        assert_eq!(back.shortcut_new, "Ctrl+Alt+N");
        assert_eq!(back.shortcut_open, "Ctrl+Alt+O");
        assert_eq!(back.shortcut_save, "Ctrl+Alt+S");
        assert_eq!(back.shortcut_save_as, "Ctrl+Alt+Shift+S");
        assert_eq!(back.shortcut_save_all, "Ctrl+Shift+A");
        assert_eq!(back.shortcut_print, "Ctrl+Alt+P");
        assert_eq!(back.shortcut_tab_indent, "Ctrl+I");
        assert_eq!(back.shortcut_shift_tab_outdent, "Ctrl+Shift+Tab");
        assert_eq!(back.shortcut_fold_all, "Ctrl+Alt+0");
        assert_eq!(back.shortcut_fold_current, "Ctrl+Alt+F");
        assert_eq!(back.shortcut_matching_brace, "Ctrl+Alt+B");
        assert_eq!(back.shortcut_move_line, "Ctrl+Alt+Up");
        assert_eq!(back.shortcut_toggle_comment, "Ctrl+Alt+/");
        assert_eq!(back.shortcut_find, "Ctrl+Alt+G");
        assert_eq!(back.shortcut_close_find, "Ctrl+Alt+Escape");
        assert_eq!(back.shortcut_replace, "Ctrl+Alt+H");
        assert_eq!(back.shortcut_replace_alt, "Ctrl+Alt+Shift+F");
        assert_eq!(back.shortcut_select_all, "Ctrl+Alt+A");
        assert_eq!(back.shortcut_undo, "Ctrl+Alt+Z");
        assert_eq!(back.shortcut_redo, "Ctrl+Alt+Y");
        assert_eq!(back.shortcut_zoom_in, "Ctrl+Alt+=");
        assert_eq!(back.shortcut_zoom_out, "Ctrl+Alt+-");
        assert_eq!(back.shortcut_zoom_restore, "Ctrl+Alt+0");
        assert_eq!(back.shortcut_toggle_log_tail, "Ctrl+Alt+T");
    }

    #[test]
    fn compare_hide_equal_context_clamps() {
        let low = AppSettings {
            compare_hide_equal_context: 0,
            ..Default::default()
        };
        assert_eq!(low.compare_hide_equal_context_lines(), 0);
        let high = AppSettings {
            compare_hide_equal_context: 99,
            ..Default::default()
        };
        assert_eq!(high.compare_hide_equal_context_lines(), 10);
        assert_eq!(AppSettings::default().compare_hide_equal_context_lines(), 3);
    }
}
