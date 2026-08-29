//! Format menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_FORMAT_")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
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

        _ => CmdResult::Stub,
    })
}
