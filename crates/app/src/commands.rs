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
            | "IDM_FILE_CLOSE"
            | "IDM_FILE_CLOSEALL"
            | "IDM_FILE_EXIT"
            | "IDM_FILE_RELOAD"
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
            | "IDM_EDIT_DUP_LINE"
            | "IDM_EDIT_TRIMTRAILING"
            | "IDM_FORMAT_TOUNIX"
            | "IDM_FORMAT_TODOS"
            | "IDM_SEARCH_FIND"
            | "IDM_SEARCH_REPLACE"
            | "IDM_SEARCH_FINDNEXT"
            | "IDM_SEARCH_FINDPREV"
            | "IDM_VIEW_GOTO_START"
            | "IDM_VIEW_GOTO_END"
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
        "IDM_EDIT_DUP_LINE" => {
            state.tabs.active_mut().buffer.duplicate_line();
            state.mark_text_changed();
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
