//! Language menu commands.
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_LANG_")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, _ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
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

        _ => CmdResult::Stub,
    })
}
