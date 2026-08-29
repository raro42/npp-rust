//! Format menu commands.
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;
use doc::FileEncoding;
use fs::UTF8_BOM_CHAR;

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
            // UTF-8 without BOM: strip leading BOM; save writes plain UTF-8.
            let stripped = strip_leading_bom(state);
            state.tabs.active_mut().encoding = FileEncoding::Utf8;
            state.status = if stripped {
                "Encoding: UTF-8 — save writes UTF-8 without BOM (BOM removed)".into()
            } else {
                "Encoding: UTF-8 — save writes UTF-8 without BOM".into()
            };
            CmdResult::Handled
        }
        "IDM_FORMAT_UTF_8" => {
            // UTF-8-BOM: keep leading U+FEFF so save writes EF BB BF.
            let added = ensure_leading_bom(state);
            state.tabs.active_mut().encoding = FileEncoding::Utf8Bom;
            state.status = if added {
                "Encoding: UTF-8-BOM — save writes UTF-8 with BOM".into()
            } else {
                "Encoding: UTF-8-BOM — save writes UTF-8 with BOM (already set)".into()
            };
            CmdResult::Handled
        }
        "IDM_FORMAT_ANSI" => {
            // ANSI: strip BOM; save re-encodes as Windows-1252 (lossy).
            let stripped = strip_leading_bom(state);
            state.tabs.active_mut().encoding = FileEncoding::Windows1252;
            state.status = if stripped {
                "Encoding: ANSI — save writes Windows-1252 (BOM removed; unmapped chars → ?)".into()
            } else {
                "Encoding: ANSI — save writes Windows-1252 (unmapped chars → ?)".into()
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
    let Some(rest) = text.strip_prefix(UTF8_BOM_CHAR) else {
        return false;
    };
    state.tabs.active_mut().buffer.replace_document(rest);
    state.mark_text_changed();
    true
}

/// Ensure a leading UTF-8 BOM character. Does not change EOL. Returns true if added.
fn ensure_leading_bom(state: &mut EditorState) -> bool {
    let text = state.tabs.active().buffer.to_string();
    if text.starts_with(UTF8_BOM_CHAR) {
        return false;
    }
    let mut with_bom = String::with_capacity(text.len() + UTF8_BOM_CHAR.len_utf8());
    with_bom.push(UTF8_BOM_CHAR);
    with_bom.push_str(&text);
    state.tabs.active_mut().buffer.replace_document(&with_bom);
    state.mark_text_changed();
    true
}
