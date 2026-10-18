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

/// Syntax languages the editor can highlight, in Language-menu order.
const SYNTAX_LANGUAGES: &[&str] = &[
    "plain",
    "c",
    "cpp",
    "html",
    "java",
    "javascript",
    "json",
    "python",
    "rust",
    "sql",
    "xml",
];

/// Next or previous highlight language id. Unknown current ids wrap to an end.
pub(crate) fn next_language_id(current: &str, forward: bool) -> &'static str {
    let len = SYNTAX_LANGUAGES.len();
    let idx = SYNTAX_LANGUAGES.iter().position(|id| *id == current);
    let next = match (idx, forward) {
        (Some(i), true) => (i + 1) % len,
        (Some(i), false) => (i + len - 1) % len,
        (None, true) => 0,
        (None, false) => len - 1,
    };
    SYNTAX_LANGUAGES[next]
}

/// Cycle the active tab's syntax language. `forward` is next; otherwise previous.
pub(crate) fn cycle_language(state: &mut EditorState, forward: bool) {
    let next = next_language_id(&state.tabs.active().language, forward);
    state.set_language(next);
    let dir = if forward { "next" } else { "previous" };
    state.status = format!("Language: {next} ({dir})");
}

#[cfg(test)]
mod tests {
    use super::next_language_id;

    #[test]
    fn language_cycle_wraps_known_ids() {
        assert_eq!(next_language_id("plain", true), "c");
        assert_eq!(next_language_id("xml", true), "plain");
        assert_eq!(next_language_id("plain", false), "xml");
        assert_eq!(next_language_id("rust", false), "python");
        assert_eq!(next_language_id("not-a-lexer", true), "plain");
        assert_eq!(next_language_id("not-a-lexer", false), "xml");
    }
}
