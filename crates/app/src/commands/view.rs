//! View menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;
use std::sync::atomic::{AtomicBool, Ordering};

/// Session preference for dual-view sync.
static SYNC_SCROLL_H: AtomicBool = AtomicBool::new(false);
static SYNC_SCROLL_V: AtomicBool = AtomicBool::new(false);
static ZOOM_SYNC: AtomicBool = AtomicBool::new(false);

fn toggle_flag(flag: &AtomicBool) -> bool {
    let next = !flag.load(Ordering::Relaxed);
    flag.store(next, Ordering::Relaxed);
    next
}

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_VIEW_")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
        "IDM_VIEW_GOTO_START" => {
            let b = state.tabs.active_mut();
            b.buffer.set_caret(0);
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_VIEW_GOTO_END" => {
            let end = state.tabs.active().buffer.len_chars();
            state.tabs.active_mut().buffer.set_caret(end);
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_VIEW_ZOOMIN" => {
            ui.zoom_delta = Some(1);
            CmdResult::Handled
        }
        "IDM_VIEW_ZOOMOUT" => {
            ui.zoom_delta = Some(-1);
            CmdResult::Handled
        }
        "IDM_VIEW_ZOOMRESTORE" => {
            ui.zoom_delta = Some(0);
            CmdResult::Handled
        }
        "IDM_VIEW_ALWAYSONTOP" => {
            ui.always_on_top = Some(true);
            state.status = "Always on top requested".into();
            CmdResult::Handled
        }
        "IDM_VIEW_MONITORING" => {
            if state.toggle_tail_follow() {
                ui.follow_caret = true;
            }
            CmdResult::Handled
        }
        "IDM_VIEW_FULLSCREENTOGGLE" | "IDM_VIEW_DISTRACTIONFREE" => {
            ui.fullscreen_toggle = true;
            state.status = "Fullscreen toggled".into();
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_SPACE" => {
            state.show_whitespace = !state.show_whitespace;
            state.status = format!(
                "Show space/tab: {}",
                if state.show_whitespace { "on" } else { "off" }
            );
            CmdResult::Handled
        }
        "IDM_VIEW_EOL" => {
            state.show_eol = !state.show_eol;
            state.status = format!("Show EOL: {}", if state.show_eol { "on" } else { "off" });
            CmdResult::Handled
        }
        "IDM_VIEW_NPC" | "IDM_VIEW_NPC_CCUNIEOL" => {
            state.show_npc = !state.show_npc;
            state.status = format!("Show NPC: {}", if state.show_npc { "on" } else { "off" });
            CmdResult::Handled
        }
        "IDM_VIEW_ALL_CHARACTERS" => {
            let on = !(state.show_whitespace && state.show_eol && state.show_npc);
            state.show_whitespace = on;
            state.show_eol = on;
            state.show_npc = on;
            state.status = format!("Show all characters: {}", if on { "on" } else { "off" });
            CmdResult::Handled
        }
        "IDM_VIEW_INDENT_GUIDE" => {
            state.show_indent_guide = !state.show_indent_guide;
            state.status = format!(
                "Indent guide: {}",
                if state.show_indent_guide { "on" } else { "off" }
            );
            CmdResult::Handled
        }
        "IDM_VIEW_WRAP_SYMBOL" => {
            // Used together with word wrap; toggle is recorded for UI.
            state.status = if state.word_wrap {
                "Wrap symbol follows word wrap".into()
            } else {
                "Wrap symbol: enable Word wrap to see soft-wrap marks".into()
            };
            CmdResult::Handled
        }
        "IDM_VIEW_WRAP" => {
            state.word_wrap = !state.word_wrap;
            state.settings.word_wrap = state.word_wrap;
            state.settings.save();
            state.status = format!(
                "Word wrap: {} (visual wrap is approximate)",
                if state.word_wrap { "on" } else { "off" }
            );
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_MOVEFORWARD" => {
            if state.tabs.move_active_tab(1) {
                state.status = "Tab moved forward".into();
            } else {
                state.status = "Tab already at end".into();
            }
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_MOVEBACKWARD" => {
            if state.tabs.move_active_tab(-1) {
                state.status = "Tab moved backward".into();
            } else {
                state.status = "Tab already at start".into();
            }
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_COLOUR_1"
        | "IDM_VIEW_TAB_COLOUR_2"
        | "IDM_VIEW_TAB_COLOUR_3"
        | "IDM_VIEW_TAB_COLOUR_4"
        | "IDM_VIEW_TAB_COLOUR_5" => {
            let n = cmd.chars().last().and_then(|c| c.to_digit(10)).unwrap_or(1) as u8;
            state.tabs.active_mut().tab_colour = Some(n);
            state.status = format!("Tab colour {n}");
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_COLOUR_NONE" => {
            state.tabs.active_mut().tab_colour = None;
            state.status = "Tab colour removed".into();
            CmdResult::Handled
        }
        "IDM_VIEW_SUMMARY" => {
            ui.show_summary = true;
            CmdResult::Handled
        }
        "IDM_VIEW_FILEBROWSER" => {
            state.open_containing_folder();
            CmdResult::Handled
        }
        "IDM_VIEW_DOCLIST" => {
            ui.show_doc_list = true;
            CmdResult::Handled
        }
        "IDM_VIEW_SYNSCROLLH" => {
            let on = toggle_flag(&SYNC_SCROLL_H);
            ui.sync_scroll_h = Some(on);
            ui.dual_view = Some(true);
            state.status = format!(
                "Sync H scroll: {} (dual view — shared line scroll)",
                if on { "on" } else { "off" }
            );
            CmdResult::Handled
        }
        "IDM_VIEW_SYNSCROLLV" => {
            let on = toggle_flag(&SYNC_SCROLL_V);
            ui.sync_scroll_v = Some(on);
            ui.dual_view = Some(true);
            state.status = format!(
                "Sync V scroll: {} (dual view)",
                if on { "on" } else { "off" }
            );
            CmdResult::Handled
        }
        "IDM_VIEW_COMPARE" => {
            ui.start_compare = true;
            CmdResult::Handled
        }
        "IDM_VIEW_CLEARCOMPARE" => {
            ui.clear_compare = true;
            CmdResult::Handled
        }
        "IDM_VIEW_ZOOM_SYNC" => {
            let on = toggle_flag(&ZOOM_SYNC);
            ui.zoom_sync = Some(on);
            ui.dual_view = Some(true);
            state.status = format!(
                "Zoom sync: {} (both panes share font size)",
                if on { "on" } else { "off" }
            );
            CmdResult::Handled
        }
        "IDM_VIEW_IN_FIREFOX" | "IDM_VIEW_IN_CHROME" | "IDM_VIEW_IN_EDGE" | "IDM_VIEW_IN_IE" => {
            open_active_in_browser(state, cmd);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB1" => {
            state.switch_tab(0);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB2" => {
            state.switch_tab(1);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB3" => {
            state.switch_tab(2);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB4" => {
            state.switch_tab(3);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB5" => {
            state.switch_tab(4);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB6" => {
            state.switch_tab(5);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB7" => {
            state.switch_tab(6);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB8" => {
            state.switch_tab(7);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB9" => {
            state.switch_tab(8);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_NEXT" => {
            state.next_tab();
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_PREV" => {
            state.prev_tab();
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_START" => {
            state.switch_tab(0);
            CmdResult::Handled
        }
        "IDM_VIEW_TAB_END" => {
            let last = state.tabs.len().saturating_sub(1);
            state.switch_tab(last);
            CmdResult::Handled
        }
        "IDM_VIEW_POSTIT" => {
            ui.fullscreen_toggle = true;
            state.status = "Post-It (distraction-free) toggled".into();
            CmdResult::Handled
        }
        "IDM_VIEW_GOTO_ANOTHER_VIEW" => {
            // Put the active tab in the other pane; keep focus on a remaining tab.
            // Do not also Switch — that undoes the move.
            ui.dual_view = Some(true);
            ui.assign_other_view = true;
            state.status = "Moved tab to other view".into();
            CmdResult::Handled
        }
        "IDM_VIEW_SWITCHTO_OTHER_VIEW" => {
            ui.dual_view = Some(true);
            ui.switch_other_view = true;
            state.status = "Switched to other view".into();
            CmdResult::Handled
        }
        "IDM_VIEW_CLONE_TO_ANOTHER_VIEW" => {
            let src = state.tabs.active_index();
            let mut clone = state.tabs.active().clone();
            clone.path = None;
            clone.title = format!("{} (clone)", clone.title);
            clone.dirty = true;
            clone.loading = false;
            clone.tail_follow = false;
            state.tabs.open_document(clone);
            let clone_idx = state.tabs.active_index();
            state.tabs.set_active(src);
            state.highlight_dirty = true;
            ui.dual_view = Some(true);
            ui.other_view_tab = Some(clone_idx);
            state.status = "Cloned document to other view".into();
            CmdResult::Handled
        }
        "IDM_VIEW_GOTO_NEW_INSTANCE" => {
            open_in_new_instance(state, true);
            CmdResult::Handled
        }
        "IDM_VIEW_LOAD_IN_NEW_INSTANCE" => {
            open_in_new_instance(state, false);
            CmdResult::Handled
        }
        "IDM_VIEW_HIDELINES" => {
            hide_selected_or_current_lines(state);
            CmdResult::Handled
        }
        "IDM_VIEW_FOLDALL" => {
            fold_all_by_indent(state);
            CmdResult::Handled
        }
        "IDM_VIEW_UNFOLDALL" => {
            unfold_all_hidden(state);
            CmdResult::Handled
        }
        "IDM_VIEW_FOLD_CURRENT" => {
            fold_current_block(state);
            CmdResult::Handled
        }
        "IDM_VIEW_UNFOLD_CURRENT" => {
            unfold_current_block(state);
            CmdResult::Handled
        }
        "IDM_VIEW_FOLD_1" | "IDM_VIEW_FOLD_2" | "IDM_VIEW_FOLD_3" | "IDM_VIEW_FOLD_4"
        | "IDM_VIEW_FOLD_5" | "IDM_VIEW_FOLD_6" | "IDM_VIEW_FOLD_7" | "IDM_VIEW_FOLD_8" => {
            let level = cmd.chars().last().and_then(|c| c.to_digit(10)).unwrap_or(1) as usize;
            fold_indent_level(state, level);
            CmdResult::Handled
        }
        "IDM_VIEW_UNFOLD_1" | "IDM_VIEW_UNFOLD_2" | "IDM_VIEW_UNFOLD_3" | "IDM_VIEW_UNFOLD_4"
        | "IDM_VIEW_UNFOLD_5" | "IDM_VIEW_UNFOLD_6" | "IDM_VIEW_UNFOLD_7" | "IDM_VIEW_UNFOLD_8" => {
            let level = cmd.chars().last().and_then(|c| c.to_digit(10)).unwrap_or(1) as usize;
            unfold_indent_level(state, level);
            CmdResult::Handled
        }
        "IDM_VIEW_PROJECT_PANEL_1" | "IDM_VIEW_PROJECT_PANEL_2" | "IDM_VIEW_PROJECT_PANEL_3" => {
            // Project panels are not modelled — open the document list instead.
            ui.show_doc_list = true;
            state.status = "Opened document list (no project panel yet)".into();
            CmdResult::Handled
        }
        "IDM_VIEW_DOC_MAP" => {
            ui.show_doc_map = true;
            state.status = "Document Map".into();
            CmdResult::Handled
        }
        "IDM_VIEW_FUNC_LIST" => {
            ui.show_func_list = true;
            state.status = "Function List".into();
            CmdResult::Handled
        }

        _ => CmdResult::Stub,
    })
}
