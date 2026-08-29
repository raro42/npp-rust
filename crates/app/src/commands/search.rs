//! Search menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_SEARCH_") || cmd.starts_with("IDM_FOCUS_ON_")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
        "IDM_SEARCH_FIND" => {
            ui.find_open = true;
            ui.show_replace = false;
            ui.find_focus_once = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_REPLACE" => {
            ui.find_open = true;
            ui.show_replace = true;
            ui.find_focus_once = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDNEXT" => {
            state.find_next();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDPREV" => {
            state.find_prev();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_SETANDFINDNEXT" => {
            if let Some((s, e)) = state.tabs.active().buffer.selection() {
                state.find_query = state.tabs.active().buffer.slice(s, e);
            }
            state.find_next();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_SETANDFINDPREV" => {
            if let Some((s, e)) = state.tabs.active().buffer.selection() {
                state.find_query = state.tabs.active().buffer.slice(s, e);
            }
            state.find_prev();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_GOTOLINE" => {
            ui.show_goto_line = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_VOLATILE_FINDNEXT" => {
            state.find_next();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_VOLATILE_FINDPREV" => {
            state.find_prev();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDINCREMENT" | "IDM_FOCUS_ON_FOUND_RESULTS" => {
            ui.find_open = true;
            ui.show_replace = false;
            ui.find_focus_once = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_GOTOMATCHINGBRACE" => {
            let text = state.tabs.active().buffer.to_string();
            let caret = state.tabs.active().buffer.caret();
            if let Some(at) = find_matching_brace(&text, caret) {
                state.tabs.active_mut().buffer.set_caret(at);
                ui.follow_caret = true;
                state.status = "Matching brace".into();
            } else {
                state.status = "No matching brace".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_SELECTMATCHINGBRACES" => {
            let text = state.tabs.active().buffer.to_string();
            let caret = state.tabs.active().buffer.caret();
            if let Some((a, b)) = brace_span(&text, caret) {
                let (s, e) = if a < b { (a, b + 1) } else { (b, a + 1) };
                state.tabs.active_mut().buffer.set_selection(s, e);
                ui.follow_caret = true;
                state.status = "Selected brace pair".into();
            } else {
                state.status = "No matching brace".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_TOGGLE_BOOKMARK" => {
            let line = state
                .tabs
                .active()
                .buffer
                .char_to_line(state.tabs.active().buffer.caret());
            let marks = &mut state.tabs.active_mut().bookmarks;
            if !marks.remove(&line) {
                marks.insert(line);
                state.status = format!("Bookmark on line {}", line + 1);
            } else {
                state.status = format!("Bookmark cleared on line {}", line + 1);
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_NEXT_BOOKMARK" => {
            let line = state
                .tabs
                .active()
                .buffer
                .char_to_line(state.tabs.active().buffer.caret());
            let next = state
                .tabs
                .active()
                .bookmarks
                .iter()
                .copied()
                .find(|&l| l > line)
                .or_else(|| state.tabs.active().bookmarks.iter().copied().next());
            if let Some(l) = next {
                let at = state.tabs.active().buffer.line_to_char(l);
                state.tabs.active_mut().buffer.set_caret(at);
                ui.follow_caret = true;
                state.status = format!("Bookmark line {}", l + 1);
            } else {
                state.status = "No bookmarks".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_PREV_BOOKMARK" => {
            let line = state
                .tabs
                .active()
                .buffer
                .char_to_line(state.tabs.active().buffer.caret());
            let prev = state
                .tabs
                .active()
                .bookmarks
                .iter()
                .rev()
                .copied()
                .find(|&l| l < line)
                .or_else(|| state.tabs.active().bookmarks.iter().next_back().copied());
            if let Some(l) = prev {
                let at = state.tabs.active().buffer.line_to_char(l);
                state.tabs.active_mut().buffer.set_caret(at);
                ui.follow_caret = true;
                state.status = format!("Bookmark line {}", l + 1);
            } else {
                state.status = "No bookmarks".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_CLEAR_BOOKMARKS" => {
            state.tabs.active_mut().bookmarks.clear();
            state.status = "Bookmarks cleared".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_INVERSEMARKS" => {
            let n = state.tabs.active().buffer.line_count();
            let old = state.tabs.active().bookmarks.clone();
            let marks = &mut state.tabs.active_mut().bookmarks;
            marks.clear();
            for l in 0..n {
                if !old.contains(&l) {
                    marks.insert(l);
                }
            }
            state.status = "Bookmarks inverted".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_COPYMARKEDLINES" => {
            let text = state.tabs.active().buffer.to_string();
            let lines: Vec<&str> = text.lines().collect();
            let marked: Vec<&str> = state
                .tabs
                .active()
                .bookmarks
                .iter()
                .filter_map(|&i| lines.get(i).copied())
                .collect();
            let joined = marked.join("\n");
            ui.last_copied = Some(joined.clone());
            ui.pending_clipboard = Some(joined);
            state.status = format!("Copied {} bookmarked line(s)", marked.len());
            CmdResult::Handled
        }
        "IDM_SEARCH_CUTMARKEDLINES" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            let text = state.tabs.active().buffer.to_string();
            let lines: Vec<&str> = text.lines().collect();
            let marked: Vec<&str> = state
                .tabs
                .active()
                .bookmarks
                .iter()
                .filter_map(|&i| lines.get(i).copied())
                .collect();
            let n = marked.len();
            let joined = marked.join("\n");
            ui.last_copied = Some(joined.clone());
            ui.pending_clipboard = Some(joined);
            filter_lines_by_bookmarks(state, true);
            state.status = format!("Cut {n} bookmarked line(s)");
            CmdResult::Handled
        }
        "IDM_SEARCH_PASTEMARKEDLINES" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            if let Some(clip) = ui.last_copied.clone() {
                paste_over_bookmarked_lines(state, &clip);
            } else {
                ui.await_paste_bookmarks = true;
                state.status =
                    "Press ⌘V / Ctrl+V to paste onto bookmarked lines".into();
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_DELETEMARKEDLINES" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            filter_lines_by_bookmarks(state, true);
            CmdResult::Handled
        }
        "IDM_SEARCH_DELETEUNMARKEDLINES" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            filter_lines_by_bookmarks(state, false);
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDINFILES" => {
            ui.find_open = true;
            ui.show_replace = false;
            ui.find_focus_once = true;
            state.status = "Find in Files: use Find for now (folder search next)".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_GOTONEXTFOUND" => {
            state.find_next();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_GOTOPREVFOUND" => {
            state.find_prev();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_SEARCH_MARK" => {
            ui.find_open = true;
            ui.find_focus_once = true;
            state.status = "Mark: find then use Style All Occurrences".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_CHANGED_NEXT" | "IDM_SEARCH_CHANGED_PREV" | "IDM_SEARCH_CLEAR_CHANGE_HISTORY" => {
            state.status = "Change history not tracked yet".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDCHARINRANGE" => {
            ui.find_open = true;
            ui.find_focus_once = true;
            state.status = "Find characters in range: use Find for now".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_MARKALLEXT1"
        | "IDM_SEARCH_MARKALLEXT2"
        | "IDM_SEARCH_MARKALLEXT3"
        | "IDM_SEARCH_MARKALLEXT4"
        | "IDM_SEARCH_MARKALLEXT5" => {
            let style = style_from_cmd(cmd);
            mark_all_token(state, style, false);
            CmdResult::Handled
        }
        "IDM_SEARCH_MARKONEEXT1"
        | "IDM_SEARCH_MARKONEEXT2"
        | "IDM_SEARCH_MARKONEEXT3"
        | "IDM_SEARCH_MARKONEEXT4"
        | "IDM_SEARCH_MARKONEEXT5" => {
            let style = style_from_cmd(cmd);
            mark_current_line(state, style);
            CmdResult::Handled
        }
        "IDM_SEARCH_UNMARKALLEXT1"
        | "IDM_SEARCH_UNMARKALLEXT2"
        | "IDM_SEARCH_UNMARKALLEXT3"
        | "IDM_SEARCH_UNMARKALLEXT4"
        | "IDM_SEARCH_UNMARKALLEXT5" => {
            if let Some(style) = style_from_cmd(cmd) {
                if let Some(s) = style_slot(state, Some(style)) {
                    s.clear();
                    state.status = format!("Cleared style {style}");
                }
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_CLEARALLMARKS" => {
            for s in &mut state.tabs.active_mut().style_marks {
                s.clear();
            }
            state.status = "Cleared all styles".into();
            CmdResult::Handled
        }
        "IDM_SEARCH_GOPREVMARKER_DEF" => {
            jump_bookmarks(state, ui, false);
            CmdResult::Handled
        }
        "IDM_SEARCH_GONEXTMARKER_DEF" => {
            jump_bookmarks(state, ui, true);
            CmdResult::Handled
        }
        "IDM_SEARCH_GOPREVMARKER1"
        | "IDM_SEARCH_GOPREVMARKER2"
        | "IDM_SEARCH_GOPREVMARKER3"
        | "IDM_SEARCH_GOPREVMARKER4"
        | "IDM_SEARCH_GOPREVMARKER5" => {
            let style = style_from_cmd(cmd).unwrap_or(1);
            jump_style(state, ui, style, false);
            CmdResult::Handled
        }
        "IDM_SEARCH_GONEXTMARKER1"
        | "IDM_SEARCH_GONEXTMARKER2"
        | "IDM_SEARCH_GONEXTMARKER3"
        | "IDM_SEARCH_GONEXTMARKER4"
        | "IDM_SEARCH_GONEXTMARKER5" => {
            let style = style_from_cmd(cmd).unwrap_or(1);
            jump_style(state, ui, style, true);
            CmdResult::Handled
        }
        "IDM_SEARCH_STYLE1TOCLIP"
        | "IDM_SEARCH_STYLE2TOCLIP"
        | "IDM_SEARCH_STYLE3TOCLIP"
        | "IDM_SEARCH_STYLE4TOCLIP"
        | "IDM_SEARCH_STYLE5TOCLIP" => {
            let style = style_from_cmd(cmd).unwrap_or(1);
            copy_style_lines(state, ui, Some(style));
            CmdResult::Handled
        }
        "IDM_SEARCH_ALLSTYLESTOCLIP" => {
            copy_style_lines(state, ui, None);
            CmdResult::Handled
        }
        "IDM_SEARCH_MARKEDTOCLIP" => {
            // Find-mark style → bookmarked lines for now.
            let text = state.tabs.active().buffer.to_string();
            let lines: Vec<&str> = text.lines().collect();
            let marked: Vec<&str> = state
                .tabs
                .active()
                .bookmarks
                .iter()
                .filter_map(|&i| lines.get(i).copied())
                .collect();
            ui.pending_clipboard = Some(marked.join("\n"));
            state.status = format!("Copied {} find-mark line(s)", marked.len());
            CmdResult::Handled
        }

        _ => CmdResult::Stub,
    })
}

fn style_from_cmd(cmd: &str) -> Option<u8> {
    let digits: String = cmd.chars().rev().take_while(|c| c.is_ascii_digit()).collect();
    let digits: String = digits.chars().rev().collect();
    digits.parse().ok().filter(|&n| (1..=5).contains(&n))
}

fn style_slot(state: &mut EditorState, style: Option<u8>) -> Option<&mut std::collections::BTreeSet<usize>> {
    let n = style? as usize;
    if (1..=5).contains(&n) {
        Some(&mut state.tabs.active_mut().style_marks[n - 1])
    } else {
        None
    }
}

fn mark_current_line(state: &mut EditorState, style: Option<u8>) {
    let Some(style) = style else {
        state.status = "Bad style id".into();
        return;
    };
    let line = state
        .tabs
        .active()
        .buffer
        .char_to_line(state.tabs.active().buffer.caret());
    if let Some(slot) = style_slot(state, Some(style)) {
        slot.insert(line);
        state.status = format!("Marked line {} with style {style}", line + 1);
    }
}

fn mark_all_token(state: &mut EditorState, style: Option<u8>, _one: bool) {
    let Some(style) = style else {
        state.status = "Bad style id".into();
        return;
    };
    let token = if let Some((s, e)) = state.tabs.active().buffer.selection() {
        state.tabs.active().buffer.slice(s, e)
    } else if !state.find_query.is_empty() {
        state.find_query.clone()
    } else {
        state.status = "Style all: select a token or set Find text".into();
        return;
    };
    if token.is_empty() {
        state.status = "Style all: empty token".into();
        return;
    }
    let text = state.tabs.active().buffer.to_string();
    let mut count = 0usize;
    {
        let slot = match style_slot(state, Some(style)) {
            Some(s) => s,
            None => return,
        };
        slot.clear();
        for (i, line) in text.lines().enumerate() {
            if line.contains(&token) {
                slot.insert(i);
                count += 1;
            }
        }
    }
    state.status = format!("Styled {count} line(s) with style {style}");
}

fn jump_marks(
    state: &mut EditorState,
    ui: &mut UiFlags,
    marks: &std::collections::BTreeSet<usize>,
    forward: bool,
) -> Option<usize> {
    if marks.is_empty() {
        return None;
    }
    let line = state
        .tabs
        .active()
        .buffer
        .char_to_line(state.tabs.active().buffer.caret());
    let next = if forward {
        marks
            .iter()
            .copied()
            .find(|&l| l > line)
            .or_else(|| marks.iter().copied().next())
    } else {
        marks
            .iter()
            .rev()
            .copied()
            .find(|&l| l < line)
            .or_else(|| marks.iter().next_back().copied())
    };
    if let Some(l) = next {
        let at = state.tabs.active().buffer.line_to_char(l);
        state.tabs.active_mut().buffer.set_caret(at);
        ui.follow_caret = true;
    }
    next
}

fn jump_style(state: &mut EditorState, ui: &mut UiFlags, style: u8, forward: bool) {
    let marks = state.tabs.active().style_marks[(style as usize) - 1].clone();
    if let Some(l) = jump_marks(state, ui, &marks, forward) {
        state.status = format!("Style {style} → line {}", l + 1);
    } else {
        state.status = format!("No marks for style {style}");
    }
}

fn jump_bookmarks(state: &mut EditorState, ui: &mut UiFlags, forward: bool) {
    let marks = state.tabs.active().bookmarks.clone();
    if let Some(l) = jump_marks(state, ui, &marks, forward) {
        state.status = format!("Find mark → line {}", l + 1);
    } else {
        state.status = "No find marks (bookmarks)".into();
    }
}

fn copy_style_lines(state: &mut EditorState, ui: &mut UiFlags, style: Option<u8>) {
    let text = state.tabs.active().buffer.to_string();
    let lines: Vec<&str> = text.lines().collect();
    let mut idxs: Vec<usize> = Vec::new();
    if let Some(s) = style {
        idxs.extend(state.tabs.active().style_marks[(s as usize) - 1].iter().copied());
    } else {
        for set in &state.tabs.active().style_marks {
            idxs.extend(set.iter().copied());
        }
        idxs.sort_unstable();
        idxs.dedup();
    }
    let marked: Vec<&str> = idxs.iter().filter_map(|&i| lines.get(i).copied()).collect();
    let n = marked.len();
    ui.pending_clipboard = Some(marked.join("\n"));
    state.status = format!("Copied {n} styled line(s)");
}
