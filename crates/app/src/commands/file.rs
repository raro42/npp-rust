//! File menu commands.
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_FILE_") || is_pin_toggle_idm(cmd)
}

/// Upstream tab-context pin (`IDM_PINTAB`). Any `*PIN*` that is not Close All but Pinned.
fn is_pin_toggle_idm(cmd: &str) -> bool {
    if cmd.contains("CLOSEALL_BUT_PINNED") {
        return false;
    }
    cmd.contains("PIN")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
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
            state.request_close_tab(idx);
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL" => {
            state.start_bulk_close(crate::editor::BulkClose::All);
            CmdResult::Handled
        }
        "IDM_FILE_EXIT" => {
            state.request_quit(ui);
            CmdResult::Handled
        }
        "IDM_FILE_RELOAD" => {
            state.request_reload();
            CmdResult::Handled
        }
        "IDM_FILE_SAVEALL" => {
            state.save_all();
            CmdResult::Handled
        }
        "IDM_FILE_SAVECOPYAS" => {
            state.save_copy_as();
            CmdResult::Handled
        }
        "IDM_FILE_RENAME" => {
            state.rename_active();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_BUT_CURRENT" => {
            state.close_all_but_current();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_TOLEFT" => {
            state.close_all_to_left();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_TORIGHT" => {
            state.close_all_to_right();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_UNCHANGED" => {
            state.close_all_unchanged();
            CmdResult::Handled
        }
        "IDM_FILE_OPEN_FOLDER" => {
            state.reveal_in_os();
            CmdResult::Handled
        }
        "IDM_FILE_OPENFOLDERASWORKSPACE" => {
            state.pick_workspace_folder();
            ui.show_project_panel = true;
            CmdResult::Handled
        }
        "IDM_FILE_CONTAININGFOLDERASWORKSPACE" => {
            state.set_workspace_from_active();
            ui.show_project_panel = true;
            CmdResult::Handled
        }
        "IDM_FILE_OPEN_CMD" | "IDM_FILE_OPEN_POWERSHELL" => {
            state.open_shell_here();
            CmdResult::Handled
        }
        "IDM_FILE_OPEN_DEFAULT_VIEWER" => {
            state.open_in_default_viewer();
            CmdResult::Handled
        }
        "IDM_FILE_CLOSEALL_BUT_PINNED" => {
            close_all_but_pinned_with_status(state);
            CmdResult::Handled
        }
        "IDM_FILE_DELETE" => {
            move_active_to_trash(state);
            CmdResult::Handled
        }
        "IDM_FILE_SAVESESSION" => {
            save_session(state);
            CmdResult::Handled
        }
        "IDM_FILE_LOADSESSION" => {
            load_session(state);
            CmdResult::Handled
        }
        "IDM_FILE_PRINT" | "IDM_FILE_PRINTNOW" => {
            print_active(state);
            CmdResult::Handled
        }

        other if is_pin_toggle_idm(other) => {
            toggle_active_pin(state);
            CmdResult::Handled
        }
        _ => CmdResult::Stub,
    })
}

/// Toggle `Document.pinned` on the active tab. Used by `IDM_PINTAB` and any `*PIN*` IDM.
fn toggle_active_pin(state: &mut EditorState) {
    let doc = state.tabs.active_mut();
    let title = doc.title.clone();
    doc.pinned = !doc.pinned;
    if doc.pinned {
        state.status = format!("Pinned tab: {title}");
    } else {
        state.status = format!("Unpinned tab: {title}");
    }
}

/// Close unpinned tabs. Clear status when none are pinned (pin via `IDM_PINTAB`).
fn close_all_but_pinned_with_status(state: &mut EditorState) {
    let pinned = state.tabs.iter().filter(|d| d.pinned).count();
    if pinned == 0 {
        state.status = "Close all but pinned: no tabs are pinned — pin first (IDM_PINTAB)".into();
        return;
    }
    let before = state.tabs.len();
    state.close_all_but_pinned();
    if state.pending_close.is_none() {
        let after = state.tabs.len();
        let closed = before.saturating_sub(after);
        state.status = format!("Close all but pinned: kept {pinned} pinned, closed {closed}");
    }
}

fn move_active_to_trash(state: &mut EditorState) {
    let Some(path) = state.tabs.active().path.clone() else {
        state.status = "Move to trash: save the file first".into();
        return;
    };
    let home = std::env::var_os("HOME").map(std::path::PathBuf::from);
    let Some(home) = home else {
        state.status = "Move to trash: no HOME".into();
        return;
    };
    let trash = home.join(".Trash");
    if let Err(e) = std::fs::create_dir_all(&trash) {
        state.status = format!("Trash folder failed: {e}");
        return;
    }
    let name = path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_else(|| "untitled".into());
    let mut dest = trash.join(&name);
    if dest.exists() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        dest = trash.join(format!("{}_{stamp}", name.to_string_lossy()));
    }
    match std::fs::rename(&path, &dest) {
        Ok(()) => {
            let idx = state.tabs.active_index();
            state.close_tab(idx);
            state.status = format!("Moved to Trash: {}", dest.display());
        }
        Err(e) => state.status = format!("Move to trash failed: {e}"),
    }
}

fn session_path() -> std::path::PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("npp-rs-session.txt")
}

fn save_session(state: &mut EditorState) {
    let mut lines = Vec::new();
    for d in state.tabs.iter() {
        if let Some(p) = &d.path {
            lines.push(p.display().to_string());
        }
    }
    let path = session_path();
    match std::fs::write(&path, lines.join("\n")) {
        Ok(()) => state.status = format!("Session saved: {}", path.display()),
        Err(e) => state.status = format!("Save session failed: {e}"),
    }
}

fn load_session(state: &mut EditorState) {
    let path = session_path();
    let Ok(text) = std::fs::read_to_string(&path) else {
        state.status = format!("Load session: missing {}", path.display());
        return;
    };
    let mut n = 0usize;
    for line in text.lines() {
        let p = std::path::PathBuf::from(line.trim());
        if p.exists() {
            state.open_path(p);
            n += 1;
        }
    }
    state.status = format!("Session loaded: {n} file(s)");
}

fn print_active(state: &mut EditorState) {
    let Some(path) = state.tabs.active().path.clone() else {
        state.status = "Print: save the file first".into();
        return;
    };
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("lp").arg(&path).status();
    #[cfg(not(target_os = "macos"))]
    let result = std::process::Command::new("lp").arg(&path).status();
    match result {
        Ok(s) if s.success() => state.status = "Sent to printer (lp)".into(),
        Ok(_) => state.status = "Print: lp failed".into(),
        Err(e) => state.status = format!("Print failed: {e}"),
    }
}
