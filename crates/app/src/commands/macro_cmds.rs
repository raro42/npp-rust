//! Macro menu commands.
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_MACRO_")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
        "IDM_MACRO_STARTRECORDINGMACRO" => {
            state.macro_recording = true;
            state.macro_cmds.clear();
            state.status = "Macro: recording…".into();
            CmdResult::Handled
        }
        "IDM_MACRO_STOPRECORDINGMACRO" => {
            state.macro_recording = false;
            state.status = format!("Macro: stopped ({} step(s))", state.macro_cmds.len());
            CmdResult::Handled
        }
        "IDM_MACRO_PLAYBACKRECORDEDMACRO" => {
            if state.macro_cmds.is_empty() {
                state.status = "Macro: nothing recorded".into();
            } else {
                let cmds = state.macro_cmds.clone();
                state.macro_recording = false;
                for c in &cmds {
                    let _ = super::dispatch(c, state, ui);
                }
                state.status = format!("Macro: played {} step(s)", cmds.len());
            }
            CmdResult::Handled
        }
        "IDM_MACRO_SAVECURRENTMACRO" => {
            let path = std::path::PathBuf::from("npp-rs-macro.txt");
            let body = state.macro_cmds.join("\n");
            match std::fs::write(&path, body) {
                Ok(()) => state.status = "Macro saved: npp-rs-macro.txt".into(),
                Err(e) => state.status = format!("Macro save failed: {e}"),
            }
            CmdResult::Handled
        }
        "IDM_MACRO_RUNMULTIMACRODLG" => {
            if state.macro_cmds.is_empty() {
                state.status = "Macro: nothing to repeat".into();
            } else {
                let cmds = state.macro_cmds.clone();
                state.macro_recording = false;
                for _ in 0..3 {
                    for c in &cmds {
                        let _ = super::dispatch(c, state, ui);
                    }
                }
                state.status = "Macro: ran 3 times".into();
            }
            CmdResult::Handled
        }
        _ => CmdResult::Stub,
    })
}
