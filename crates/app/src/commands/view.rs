//! View menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

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
            let n = cmd
                .chars()
                .last()
                .and_then(|c| c.to_digit(10))
                .unwrap_or(1) as u8;
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
        "IDM_VIEW_ZOOM_SYNC" | "IDM_VIEW_SYNSCROLLV" | "IDM_VIEW_SYNSCROLLH" => {
            state.status = "Single view — sync has no second pane yet".into();
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

        _ => CmdResult::Stub,
    })
}
