//! Format menu commands.
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

/// UTF-8 BOM as a Unicode character. Save writes it as `EF BB BF`.
const UTF8_BOM: char = '\u{FEFF}';

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_FORMAT_")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, _ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
        "IDM_FORMAT_TOUNIX" => {
            state.run_plugin("edit.to_unix_eol");
            CmdResult::Handled
        }
        "IDM_FORMAT_TODOS" => {
            state.run_plugin("edit.to_windows_eol");
            CmdResult::Handled
        }
        "IDM_FORMAT_AS_UTF_8" => {
            // Encode in UTF-8 without BOM: strip leading BOM; leave EOL alone.
            let stripped = strip_leading_bom(state);
            state.status = if stripped {
                "Encoding: UTF-8 (no BOM) — memory is UTF-8; BOM removed from start".into()
            } else {
                "Encoding: UTF-8 (no BOM) — memory is UTF-8".into()
            };
            CmdResult::Handled
        }
        "IDM_FORMAT_UTF_8" => {
            // Encode in UTF-8-BOM: keep a leading U+FEFF so save writes a BOM.
            let added = ensure_leading_bom(state);
            state.status = if added {
                "Encoding: UTF-8-BOM — memory is UTF-8; BOM at start for save".into()
            } else {
                "Encoding: UTF-8-BOM — memory is UTF-8; BOM already at start".into()
            };
            CmdResult::Handled
        }
        "IDM_FORMAT_ANSI" => {
            // npp-rs has no ANSI code-page convert. Strip BOM; stay UTF-8 in memory.
            let stripped = strip_leading_bom(state);
            state.status = if stripped {
                "Encoding: ANSI chosen — stays UTF-8 in memory; BOM removed (no ANSI convert)"
                    .into()
            } else {
                "Encoding: ANSI chosen — stays UTF-8 in memory (no ANSI convert)".into()
            };
            CmdResult::Handled
        }
        "IDM_FORMAT_TOMAC" => {
            // Classic Mac CR line endings.
            let text = state
                .tabs
                .active()
                .buffer
                .to_string()
                .replace("\r\n", "\n")
                .replace('\n', "\r");
            state.tabs.active_mut().buffer.replace_document(&text);
            state.mark_text_changed();
            state.status = "EOL: Macintosh (CR)".into();
            CmdResult::Handled
        }

        _ => CmdResult::Stub,
    })
}

/// Remove a leading UTF-8 BOM character. Does not change EOL. Returns true if removed.
fn strip_leading_bom(state: &mut EditorState) -> bool {
    let text = state.tabs.active().buffer.to_string();
    let Some(rest) = text.strip_prefix(UTF8_BOM) else {
        return false;
    };
    state.tabs.active_mut().buffer.replace_document(rest);
    state.mark_text_changed();
    true
}

/// Ensure a leading UTF-8 BOM character. Does not change EOL. Returns true if added.
fn ensure_leading_bom(state: &mut EditorState) -> bool {
    let text = state.tabs.active().buffer.to_string();
    if text.starts_with(UTF8_BOM) {
        return false;
    }
    let mut with_bom = String::with_capacity(text.len() + UTF8_BOM.len_utf8());
    with_bom.push(UTF8_BOM);
    with_bom.push_str(&text);
    state.tabs.active_mut().buffer.replace_document(&with_bom);
    state.mark_text_changed();
    true
}
