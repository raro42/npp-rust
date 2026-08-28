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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmdResult {
    Handled,
    Stub,
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

pub fn stub_message(cmd: &str) -> String {
    format!("Not implemented yet (Notepad++ parity stub): {cmd}")
}
