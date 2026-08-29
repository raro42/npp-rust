//! Help (?) menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

pub fn covers(cmd: &str) -> bool {
    matches!(
        cmd,
        "IDM_ABOUT"
            | "IDM_HOMESWEETHOME"
            | "IDM_PROJECTPAGE"
            | "IDM_FORUM"
            | "IDM_ONLINEDOCUMENT"
            | "IDM_UPDATE_NPP"
            | "IDM_DEBUGINFO"
            | "IDM_CMDLINEARGUMENTS"
    )
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
        "IDM_HOMESWEETHOME" => {
            open_url(state, "https://github.com/raro42/npp-rust");
            CmdResult::Handled
        }
        "IDM_PROJECTPAGE" => {
            open_url(state, "https://github.com/raro42/npp-rust/issues");
            CmdResult::Handled
        }
        "IDM_FORUM" => {
            open_url(state, "https://github.com/raro42/npp-rust/discussions");
            CmdResult::Handled
        }
        "IDM_ONLINEDOCUMENT" => {
            open_url(state, "https://github.com/raro42/npp-rust/wiki");
            CmdResult::Handled
        }
        "IDM_UPDATE_NPP" => {
            open_url(state, "https://github.com/raro42/npp-rust/releases");
            CmdResult::Handled
        }
        "IDM_DEBUGINFO" => {
            state.status = format!(
                "npp-rust {} · Rust · {}",
                env!("CARGO_PKG_VERSION"),
                std::env::consts::OS
            );
            CmdResult::Handled
        }
        "IDM_ABOUT" => {
            ui.show_about = true;
            CmdResult::Handled
        }

        _ => CmdResult::Stub,
    })
}
