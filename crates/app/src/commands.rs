//! Command dispatch for Notepad++ menu IDs (`IDM_*`).

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
    pub highlight_dirty_scroll_reset: bool,
    /// When set, show the friendly “not ready yet” window.
    pub coming_soon: Option<ComingSoon>,
    /// Zoom delta: -1 / +1 / 0 means restore.
    pub zoom_delta: Option<i8>,
    /// Open Go to Line dialog.
    pub show_goto_line: bool,
    pub always_on_top: Option<bool>,
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
            | "IDM_EDIT_DUP_LINE"
            | "IDM_EDIT_TRIMTRAILING"
            | "IDM_EDIT_JOIN_LINES"
            | "IDM_EDIT_LINE_UP"
            | "IDM_EDIT_LINE_DOWN"
            | "IDM_EDIT_BLANKLINEABOVECURRENT"
            | "IDM_EDIT_REMOVEEMPTYLINES"
            | "IDM_EDIT_REMOVEEMPTYLINESWITHBLANK"
            | "IDM_EDIT_INSERT_DATETIME_SHORT"
            | "IDM_EDIT_INSERT_DATETIME_LONG"
            | "IDM_EDIT_FULLPATHTOCLIP"
            | "IDM_EDIT_FILENAMETOCLIP"
            | "IDM_EDIT_CURRENTDIRTOCLIP"
            | "IDM_EDIT_COPY_ALL_NAMES"
            | "IDM_EDIT_COPY_ALL_PATHS"
            | "IDM_FORMAT_TOUNIX"
            | "IDM_FORMAT_TODOS"
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
            | "IDM_VIEW_GOTO_START"
            | "IDM_VIEW_GOTO_END"
            | "IDM_VIEW_ZOOMIN"
            | "IDM_VIEW_ZOOMOUT"
            | "IDM_VIEW_ZOOMRESTORE"
            | "IDM_VIEW_ALWAYSONTOP"
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
    ) || cmd.starts_with("IDM_LANG_")
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
        "IDM_EDIT_TRIMTRAILING" => {
            state.run_plugin("edit.trim_trailing");
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

        // —— Help (?) ——
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
            CmdResult::Stub
        }
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
