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
            | "IDM_CHANGELOG"
            | "IDM_DEBUGINFO"
            | "IDM_OPEN_NPP_LOGS"
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
        "IDM_CHANGELOG" => {
            open_url(
                state,
                "https://github.com/raro42/npp-rust/blob/dev/docs/changelog.md",
            );
            CmdResult::Handled
        }
        "IDM_OPEN_NPP_LOGS" => {
            state.open_npp_logs();
            CmdResult::Handled
        }
        "IDM_DEBUGINFO" => {
            state.show_debug_info();
            CmdResult::Handled
        }
        "IDM_CMDLINEARGUMENTS" => {
            show_cmdline_arguments(state);
            CmdResult::Handled
        }
        "IDM_ABOUT" => {
            ui.show_about = true;
            CmdResult::Handled
        }

        _ => CmdResult::Stub,
    })
}

/// Read-only tab: honest argv docs (no CLI flags yet). Same pattern as Debug Info.
fn show_cmdline_arguments(state: &mut EditorState) {
    let text = format!(
        "npp-rs command line arguments\n         ============================\n         \n         Current support\n         ---------------\n         npp-rs does not parse CLI flags yet.\n         The process ignores argv after the program name.\n         \n         Open paths as args\n         ------------------\n         Passing file paths on the command line is not supported yet.\n         Planned form (not active):\n           npp-rs path/to/file.txt [more paths…]\n         \n         How to open files today\n         -----------------------\n         - File → Open\n         - File → Open Recent\n         \n         How to run\n         ----------\n           cargo run -p app --release\n           ./scripts/run-npp-rust.command\n         \n         Binary name: npp-rs\n         \n         More\n         ----\n         See README for build and run notes:\n         https://github.com/raro42/npp-rust/blob/dev/README.md\n         \n         version: {}\n         os: {}\n         arch: {}\n",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
    );
    state.tabs.open_untitled();
    {
        let doc = state.tabs.active_mut();
        doc.title = "Command Line Arguments".into();
        doc.buffer = buffer::TextBuffer::from_str(&text);
        doc.dirty = false;
        doc.language = "plain".into();
        doc.read_only = true;
    }
    state.highlight_dirty = true;
    state.reset_view = true;
    state.status = "Command Line Arguments opened (? → Command Line Arguments...)".into();
}
