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

/// Read-only tab: argv usage (paths + -h/--help). Same pattern as Debug Info.
fn show_cmdline_arguments(state: &mut EditorState) {
    let text = format!(
        "npp-rs command line arguments\n\
         ============================\n\
         \n\
         Usage\n\
         -----\n\
           npp-rs [OPTIONS] [FILE]...\n\
         \n\
         Open paths\n\
         ----------\n\
         Each FILE after the program name opens in a tab when the path exists.\n\
         Missing paths are skipped. The status line lists them.\n\
         The app does not create missing files from argv.\n\
         \n\
           npp-rs path/to/file.txt [more paths…]\n\
           cargo run -p app --release -- path/to/file.txt\n\
         \n\
         Options\n\
         -------\n\
           -h, --help    Print usage to stderr and exit (no GUI)\n\
         \n\
         Other ways to open files\n\
         ------------------------\n\
         - File → Open\n\
         - File → Open Recent\n\
         \n\
         How to run\n\
         ----------\n\
           cargo run -p app --release\n\
           ./scripts/run-npp-rust.command\n\
         \n\
         Binary name: npp-rs\n\
         \n\
         More\n\
         ----\n\
         See README for build and run notes:\n\
         https://github.com/raro42/npp-rust/blob/dev/README.md\n\
         \n\
         version: {}\n\
         os: {}\n\
         arch: {}\n",
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
