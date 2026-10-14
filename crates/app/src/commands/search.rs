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
            state.prepare_find_bar_from_selection();
            CmdResult::Handled
        }
        "IDM_SEARCH_REPLACE" => {
            ui.find_open = true;
            ui.show_replace = true;
            ui.find_focus_once = true;
            state.prepare_find_bar_from_selection();
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
        "IDM_SEARCH_FINDINCREMENT" => {
            ui.find_open = true;
            ui.show_replace = false;
            ui.find_focus_once = true;
            CmdResult::Handled
        }
        "IDM_FOCUS_ON_FOUND_RESULTS" => {
            find_all_current(state, ui);
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDALLINOPENEDDOCS" => {
            find_all_opened(state, ui);
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
                state.status = "Press ⌘V / Ctrl+V to paste onto bookmarked lines".into();
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
            find_in_files(state, ui);
            CmdResult::Handled
        }
        "IDM_SEARCH_GOTONEXTFOUND" => {
            goto_search_result(state, ui, true);
            CmdResult::Handled
        }
        "IDM_SEARCH_GOTOPREVFOUND" => {
            goto_search_result(state, ui, false);
            CmdResult::Handled
        }
        "IDM_SEARCH_MARK" => {
            ui.find_open = true;
            ui.show_replace = false;
            ui.find_focus_once = true;
            state.prepare_find_bar_from_selection();
            if state.find_query.is_empty() {
                state.status =
                    "Mark: set Find text, then Mark (Style 1) or Style All Occurrences".into();
            } else {
                state.mark_all_style(1, true);
            }
            CmdResult::Handled
        }
        "IDM_SEARCH_CHANGED_NEXT" => {
            goto_changed_line(state, ui, true);
            CmdResult::Handled
        }
        "IDM_SEARCH_CHANGED_PREV" => {
            goto_changed_line(state, ui, false);
            CmdResult::Handled
        }
        "IDM_SEARCH_CLEAR_CHANGE_HISTORY" => {
            let (u, s) = state.tabs.active().change_history_counts();
            state.tabs.active_mut().clear_change_history();
            state.status = if u + s == 0 {
                "No change-history marks".into()
            } else {
                format!("Cleared change history ({u} unsaved, {s} saved)")
            };
            CmdResult::Handled
        }
        "IDM_SEARCH_FINDCHARINRANGE" => {
            find_char_in_range(state, ui);
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
    let digits: String = cmd
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let digits: String = digits.chars().rev().collect();
    digits.parse().ok().filter(|&n| (1..=5).contains(&n))
}

fn style_slot(
    state: &mut EditorState,
    style: Option<u8>,
) -> Option<&mut std::collections::BTreeSet<usize>> {
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
    // Selection token (literal + Case/Word) or Find query (Case/Word/Re + Sel).
    state.mark_all_style(style, false);
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
        idxs.extend(
            state.tabs.active().style_marks[(s as usize) - 1]
                .iter()
                .copied(),
        );
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

/// Parse Find text as a code-point range: `ascii`, `non-ascii`, or `start-end` (decimal).
fn parse_char_range(query: &str) -> Option<(u32, u32)> {
    let q = query.trim();
    if q.is_empty() {
        return None;
    }
    let lower = q.to_ascii_lowercase();
    if lower == "ascii" {
        return Some((0, 127));
    }
    if lower == "non-ascii" || lower == "nonascii" {
        return Some((128, 255));
    }
    let (a, b) = q.split_once('-')?;
    let start: u32 = a.trim().parse().ok()?;
    let end: u32 = b.trim().parse().ok()?;
    if start > end {
        return None;
    }
    Some((start, end))
}

fn find_char_in_range(state: &mut EditorState, ui: &mut UiFlags) {
    let Some((lo, hi)) = parse_char_range(&state.find_query) else {
        ui.find_open = true;
        ui.show_replace = false;
        ui.find_focus_once = true;
        if state.find_query.trim().is_empty() {
            state.find_query = "0-127".into();
        }
        state.status =
            "Find char range: set Find to ascii, non-ascii, or start-end (e.g. 65-90)".into();
        return;
    };
    let text = state.tabs.active().buffer.to_string();
    let from = state
        .tabs
        .active()
        .buffer
        .selection()
        .map(|(_, e)| e)
        .unwrap_or_else(|| state.tabs.active().buffer.caret());
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut found = None;
    for (i, ch) in chars.iter().enumerate().skip(from) {
        let cp = *ch as u32;
        if cp >= lo && cp <= hi {
            found = Some(i);
            break;
        }
    }
    if found.is_none() {
        for (i, ch) in chars.iter().enumerate().take(from.min(n)) {
            let cp = *ch as u32;
            if cp >= lo && cp <= hi {
                found = Some(i);
                break;
            }
        }
    }
    if let Some(i) = found {
        state.tabs.active_mut().buffer.set_selection(i, i + 1);
        ui.follow_caret = true;
        state.status = format!("Char range {lo}-{hi}: match at {i}");
    } else {
        state.status = format!("Char range {lo}-{hi}: no match");
    }
}

const FIND_RESULTS_TITLE: &str = "Find Results";

fn is_find_results_tab(title: &str) -> bool {
    title == FIND_RESULTS_TITLE || title == "Find in Files"
}

/// One hit line inside Find Results (`line:col:match`) or Find in Files (`path:line:text`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct FindResultsHit {
    /// 0-based line index inside the results buffer.
    results_line: usize,
    doc_label: String,
    /// 1-based line in the source document.
    line: usize,
    /// 1-based column in the source document; `0` means resolve via Find query (FIF).
    col: usize,
    matched: String,
}

/// Parse `LINE:COL:text` (1-based). Returns `None` when the line is not a hit.
fn parse_find_results_hit_line(line: &str) -> Option<(usize, usize, String)> {
    let mut parts = line.splitn(3, ':');
    let a = parts.next()?;
    let b = parts.next()?;
    let rest = parts.next()?;
    let a_ok = !a.is_empty() && a.bytes().all(|c| c.is_ascii_digit());
    let b_ok = !b.is_empty() && b.bytes().all(|c| c.is_ascii_digit());
    if !a_ok || !b_ok {
        return None;
    }
    let line_no: usize = a.parse().ok()?;
    let col: usize = b.parse().ok()?;
    if line_no == 0 || col == 0 {
        return None;
    }
    let matched = rest.replace("\\n", "\n").replace("\\r", "\r");
    Some((line_no, col, matched))
}

/// Parse Find in Files `path:LINE:text` (1-based line; path is not all-digits).
/// `col` is 0: park resolves the match via the current Find query.
fn parse_find_in_files_hit_line(line: &str) -> Option<(String, usize, String)> {
    let mut parts = line.splitn(3, ':');
    let path = parts.next()?;
    let b = parts.next()?;
    let rest = parts.next()?;
    if path.is_empty() || path.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if b.is_empty() || !b.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let line_no: usize = b.parse().ok()?;
    if line_no == 0 {
        return None;
    }
    Some((path.to_string(), line_no, rest.to_string()))
}

/// Collect hit rows from Find All / Find in Files results bodies.
fn parse_find_results_hits(text: &str) -> Vec<FindResultsHit> {
    let mut default_doc: Option<String> = None;
    let mut section_doc: Option<String> = None;
    let mut hits = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if let Some(rest) = line.strip_prefix("Document: ") {
            let label = rest.trim();
            if !label.is_empty() {
                default_doc = Some(label.to_string());
            }
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with("===") && trimmed.ends_with("===") {
            let inner = trimmed.trim_start_matches('=').trim_end_matches('=').trim();
            if !inner.is_empty() {
                section_doc = Some(inner.to_string());
            }
            continue;
        }
        if let Some((line_no, col, matched)) = parse_find_results_hit_line(line) {
            let Some(doc_label) = section_doc
                .clone()
                .or_else(|| default_doc.clone())
                .filter(|s| !s.is_empty())
            else {
                continue;
            };
            hits.push(FindResultsHit {
                results_line: i,
                doc_label,
                line: line_no,
                col,
                matched,
            });
            continue;
        }
        if let Some((path, line_no, text_line)) = parse_find_in_files_hit_line(line) {
            hits.push(FindResultsHit {
                results_line: i,
                doc_label: path,
                line: line_no,
                col: 0,
                matched: text_line,
            });
        }
    }
    hits
}

fn find_results_tab_index(state: &EditorState) -> Option<usize> {
    let active = state.tabs.active_index();
    if is_find_results_tab(&state.tabs.active().title) {
        return Some(active);
    }
    state
        .tabs
        .iter()
        .enumerate()
        .find(|(_, d)| d.title == FIND_RESULTS_TITLE)
        .map(|(i, _)| i)
        .or_else(|| {
            state
                .tabs
                .iter()
                .enumerate()
                .find(|(_, d)| d.title == "Find in Files")
                .map(|(i, _)| i)
        })
}

fn tab_index_for_find_label(state: &EditorState, label: &str) -> Option<usize> {
    state
        .tabs
        .iter()
        .enumerate()
        .find(|(_, d)| {
            if is_find_results_tab(&d.title) {
                return false;
            }
            let doc_label = d
                .path
                .as_ref()
                .map(|p| crate::recent::short_path_label(p.as_path()))
                .unwrap_or_else(|| d.title.clone());
            if doc_label == label || d.title == label {
                return true;
            }
            if let Some(path) = d.path.as_ref() {
                if path.ends_with(label) {
                    return true;
                }
                if let Ok(rel) = path.strip_prefix(&state.workspace_root) {
                    let rel = rel.to_string_lossy().replace('\\', "/");
                    if rel == label {
                        return true;
                    }
                }
            }
            false
        })
        .map(|(i, _)| i)
}

/// Resolve a Find Results / Find in Files document label; open from workspace when needed.
fn resolve_or_open_find_target(state: &mut EditorState, label: &str) -> Option<usize> {
    if let Some(i) = tab_index_for_find_label(state, label) {
        return Some(i);
    }
    let path = state.workspace_root.join(label);
    if !path.is_file() {
        return None;
    }
    state.open_path(path.clone());
    state
        .tabs
        .iter()
        .enumerate()
        .find(|(_, d)| d.path.as_ref() == Some(&path))
        .map(|(i, _)| i)
        .or_else(|| tab_index_for_find_label(state, label))
}

/// Park the source document on one Find Results / Find in Files hit.
fn park_find_results_hit(
    state: &mut EditorState,
    ui: &mut UiFlags,
    results_idx: usize,
    hit: &FindResultsHit,
    ordinal: usize,
    total: usize,
    wrapped: bool,
) {
    if let Some(doc) = state.tabs.get_mut(results_idx) {
        let at = doc.buffer.line_to_char(hit.results_line);
        doc.buffer.set_caret(at);
    }

    let Some(src_idx) = resolve_or_open_find_target(state, &hit.doc_label) else {
        state.status = format!(
            "Search result {ordinal}/{total}: document {:?} not open",
            hit.doc_label
        );
        ui.follow_caret = true;
        return;
    };

    state.tabs.set_active(src_idx);
    let line_idx = hit.line.saturating_sub(1);
    let line_count = state.tabs.active().buffer.line_count();
    if line_idx >= line_count {
        state.status = format!(
            "Search result {ordinal}/{total}: line {} missing in {:?}",
            hit.line, hit.doc_label
        );
        ui.follow_caret = true;
        return;
    }

    let (col_1based, sel_start, sel_end) = {
        let buf = &state.tabs.active().buffer;
        let line_start = buf.line_to_char(line_idx);
        let body_len = buf.line_body_len(line_idx);
        let line_body = buf.line(line_idx);
        let line_body = line_body.trim_end_matches(['\n', '\r']);

        if hit.col == 0 {
            // Find in Files: locate the Find query on the line (FIF scan has no column).
            let flags = crate::search_util::FindFlags {
                match_case: state.settings.find_match_case,
                whole_word: false,
                use_regex: state.settings.find_regex,
            };
            if let Some((a, b)) =
                crate::search_util::find_all_matches(line_body, &state.find_query, flags)
                    .into_iter()
                    .next()
            {
                let start = line_start + a;
                let end = line_start + b.min(body_len);
                (a + 1, start, end)
            } else {
                (1, line_start, line_start)
            }
        } else {
            let col_idx = hit.col.saturating_sub(1);
            let start = line_start + col_idx.min(body_len);
            let match_len = hit.matched.chars().count();
            let end = (start + match_len).min(line_start + body_len);
            (hit.col, start, end)
        }
    };

    {
        let buf = &mut state.tabs.active_mut().buffer;
        if sel_end > sel_start {
            buf.set_selection(sel_start, sel_end);
        } else {
            buf.set_caret(sel_start);
        }
    }
    ui.follow_caret = true;
    let wrap_bit = if wrapped { " · wrapped" } else { "" };
    state.status = format!(
        "Search result → {line}:{col} in {:?} ({ordinal}/{total}){wrap_bit}",
        hit.doc_label,
        line = hit.line,
        col = col_1based,
    );
}

/// Double-click jump: park the hit on the given 0-based line of the active results tab.
/// Returns `false` when the active tab is not results, or that line is not a hit row.
pub(crate) fn goto_search_result_at_line(
    state: &mut EditorState,
    ui: &mut UiFlags,
    results_line: usize,
) -> bool {
    if !is_find_results_tab(&state.tabs.active().title) {
        return false;
    }
    let results_idx = state.tabs.active_index();
    let body = state
        .tabs
        .get(results_idx)
        .map(|d| d.buffer.to_string())
        .unwrap_or_default();
    let hits = parse_find_results_hits(&body);
    let Some((pick, hit)) = hits
        .iter()
        .enumerate()
        .find(|(_, h)| h.results_line == results_line)
    else {
        return false;
    };
    let hit = hit.clone();
    park_find_results_hit(state, ui, results_idx, &hit, pick + 1, hits.len(), false);
    true
}

/// Next / Previous Search Result: walk Find Results / Find in Files hit lines and park the match.
/// Falls back to Find Next / Prev when no results tab (or no parseable hits).
fn goto_search_result(state: &mut EditorState, ui: &mut UiFlags, forward: bool) {
    let Some(results_idx) = find_results_tab_index(state) else {
        if forward {
            state.find_next();
        } else {
            state.find_prev();
        }
        ui.follow_caret = true;
        return;
    };

    let body = state
        .tabs
        .get(results_idx)
        .map(|d| d.buffer.to_string())
        .unwrap_or_default();
    let hits = parse_find_results_hits(&body);
    if hits.is_empty() {
        if forward {
            state.find_next();
        } else {
            state.find_prev();
        }
        ui.follow_caret = true;
        return;
    }

    let cur_line = state
        .tabs
        .get(results_idx)
        .map(|d| d.buffer.char_to_line(d.buffer.caret()))
        .unwrap_or(0);

    let (pick, wrapped) = if forward {
        if let Some((i, _)) = hits
            .iter()
            .enumerate()
            .find(|(_, h)| h.results_line > cur_line)
        {
            (i, false)
        } else {
            (0, hits[0].results_line <= cur_line)
        }
    } else if let Some((i, _)) = hits
        .iter()
        .enumerate()
        .rev()
        .find(|(_, h)| h.results_line < cur_line)
    {
        (i, false)
    } else {
        let last = hits.len() - 1;
        (last, hits[last].results_line >= cur_line)
    };

    let hit = hits[pick].clone();
    park_find_results_hit(state, ui, results_idx, &hit, pick + 1, hits.len(), wrapped);
}

/// Prefer the active editor tab; if already on a results tab, use the previous non-results tab.
fn find_all_source_index(state: &EditorState) -> usize {
    let active = state.tabs.active_index();
    if !is_find_results_tab(&state.tabs.active().title) {
        return active;
    }
    if active > 0 {
        let prev = active - 1;
        if let Some(doc) = state.tabs.get(prev) {
            if !is_find_results_tab(&doc.title) {
                return prev;
            }
        }
    }
    state
        .tabs
        .iter()
        .enumerate()
        .find(|(_, d)| !is_find_results_tab(&d.title))
        .map(|(i, _)| i)
        .unwrap_or(active)
}

/// Open or refresh the Find Results tab with `body`.
fn present_find_results(state: &mut EditorState, body: &str) {
    let existing = state
        .tabs
        .iter()
        .enumerate()
        .find(|(_, d)| d.title == FIND_RESULTS_TITLE && d.read_only && d.path.is_none())
        .map(|(i, _)| i);
    if let Some(i) = existing {
        state.tabs.set_active(i);
        {
            let doc = state.tabs.active_mut();
            doc.buffer = buffer::TextBuffer::from_str(body);
            doc.read_only = true;
            doc.dirty = false;
            doc.language = "plain".into();
        }
    } else {
        state.tabs.open_untitled();
        {
            let doc = state.tabs.active_mut();
            doc.title = FIND_RESULTS_TITLE.into();
            doc.buffer = buffer::TextBuffer::from_str(body);
            doc.dirty = false;
            doc.language = "plain".into();
            doc.read_only = true;
        }
    }
    state.highlight_dirty = true;
    state.reset_view = true;
}

/// Find All in the active document (Case/Word/Re/Sel); open or refresh Find Results tab.
fn find_all_current(state: &mut EditorState, ui: &mut UiFlags) {
    let source = find_all_source_index(state);
    let saved = state.tabs.active_index();
    if source != saved {
        state.tabs.set_active(source);
    }
    let Some((body, _count)) = state.find_all_current_report() else {
        if source != saved && saved < state.tabs.len() {
            state.tabs.set_active(saved);
        }
        return;
    };
    if body.is_empty() {
        if source != saved && saved < state.tabs.len() {
            state.tabs.set_active(saved);
        }
        ui.find_open = true;
        ui.show_replace = false;
        ui.find_focus_once = true;
        return;
    }
    present_find_results(state, &body);
}

/// Find All across open editor tabs (Case/Word/Re; Sel ignored); open or refresh Find Results.
fn find_all_opened(state: &mut EditorState, ui: &mut UiFlags) {
    let Some((body, _count)) = state.find_all_opened_report() else {
        return;
    };
    if body.is_empty() {
        ui.find_open = true;
        ui.show_replace = false;
        ui.find_focus_once = true;
        return;
    }
    present_find_results(state, &body);
}

/// Scan workspace root recursively for `find_query`; write hits into a results tab.
fn find_in_files(state: &mut EditorState, ui: &mut UiFlags) {
    let q = state.find_query.clone();
    if q.is_empty() {
        ui.find_open = true;
        ui.show_replace = false;
        ui.find_focus_once = true;
        state.status = "Find in Files: set Find text, then run again".into();
        return;
    }

    let root = if state.workspace_root.is_dir() {
        state.workspace_root.clone()
    } else {
        match std::env::current_dir() {
            Ok(p) => p,
            Err(_) => {
                state.status = "Find in Files: cannot read workspace root".into();
                return;
            }
        }
    };

    let include = crate::search_util::split_filters(&state.settings.find_files_include);
    let exclude = {
        let raw = state.settings.find_files_exclude.trim();
        if raw.is_empty() {
            crate::search_util::split_filters(&crate::search_util::default_find_files_exclude())
        } else {
            crate::search_util::split_filters(raw)
        }
    };
    let caps = crate::search_util::FindInFilesCaps::default();
    let use_regex = state.settings.find_regex;
    let report = crate::search_util::find_in_files_scan(
        &root,
        &q,
        state.settings.find_match_case,
        use_regex,
        &include,
        &exclude,
        caps,
    );
    if report.invalid_regex {
        state.status = "Find in Files: invalid regex".into();
        return;
    }

    let mut lines_out: Vec<String> = Vec::with_capacity(report.hits.len() + 6);
    let mode = if use_regex { " (regex)" } else { "" };
    lines_out.push(format!("Find in Files{mode}: {q:?}"));
    lines_out.push("Directory: workspace root (recursive)".into());
    let inc_label = if include.is_empty() {
        "*".into()
    } else {
        include.join(",")
    };
    lines_out.push(format!(
        "Filters: include={inc_label} exclude={}",
        exclude.join(",")
    ));
    lines_out.push(String::new());

    for hit in &report.hits {
        lines_out.push(format!("{}:{}:{}", hit.rel_path, hit.line_no, hit.line));
    }

    let match_count = report.hits.len();
    let files_ok = report.files_scanned;
    lines_out.push(String::new());
    let trunc = if report.truncated { ", truncated" } else { "" };
    lines_out.push(format!(
        "— {match_count} match(es) in {files_ok} file(s) (recursive, max {}{trunc})",
        caps.max_matches
    ));

    let body = lines_out.join("\n");
    state.tabs.open_untitled();
    {
        let doc = state.tabs.active_mut();
        doc.title = "Find in Files".into();
        doc.buffer = buffer::TextBuffer::from_str(&body);
        doc.dirty = false;
        doc.language = "plain".into();
        doc.read_only = true;
    }
    state.highlight_dirty = true;
    state.reset_view = true;
    state.status = format!("Find in Files{mode}: {match_count} match(es) in {files_ok} file(s)");
}

/// Jump to the next/previous change-history line mark in the active document.
fn goto_changed_line(state: &mut EditorState, ui: &mut UiFlags, forward: bool) {
    let marks = state.tabs.active().change_history_lines();
    if let Some(l) = jump_marks(state, ui, &marks, forward) {
        let kind = match state.tabs.active().change_history_is_saved(l) {
            Some(true) => "saved",
            Some(false) => "unsaved",
            None => "mark",
        };
        state.status = format!("Change history → line {} ({kind})", l + 1);
    } else {
        state.status = "No change-history marks".into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::UiFlags;

    #[test]
    fn parse_hit_line_basic_and_escapes() {
        assert_eq!(
            parse_find_results_hit_line("12:3:foo"),
            Some((12, 3, "foo".into()))
        );
        assert_eq!(
            parse_find_results_hit_line("1:1:a\\nb"),
            Some((1, 1, "a\nb".into()))
        );
        assert_eq!(parse_find_results_hit_line("Document: x"), None);
        assert_eq!(parse_find_results_hit_line("=== a.txt ==="), None);
        assert_eq!(parse_find_results_hit_line("src/a.rs:3:hit"), None);
        assert_eq!(
            parse_find_in_files_hit_line("src/a.rs:3:hit"),
            Some(("src/a.rs".into(), 3, "hit".into()))
        );
        assert_eq!(
            parse_find_in_files_hit_line("src/a.rs:3:foo:bar"),
            Some(("src/a.rs".into(), 3, "foo:bar".into()))
        );
        assert_eq!(parse_find_in_files_hit_line("12:3:foo"), None);
    }

    #[test]
    fn parse_hits_find_in_files_paths() {
        let body = "\
Find in Files: \"foo\"
Directory: workspace root (recursive)
Filters: include=* exclude=target

src/a.rs:1:foo one
sub/b.txt:2:x foo y

— 2 match(es) in 2 file(s)";
        let hits = parse_find_results_hits(body);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].doc_label, "src/a.rs");
        assert_eq!(hits[0].line, 1);
        assert_eq!(hits[0].col, 0);
        assert_eq!(hits[1].doc_label, "sub/b.txt");
        assert_eq!(hits[1].line, 2);
    }

    #[test]
    fn parse_hits_current_and_opened_sections() {
        let current = "\
Find All: \"foo\" · case
Document: sample.txt

2:1:foo
4:1:foo

— 2 match(es)";
        let hits = parse_find_results_hits(current);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].doc_label, "sample.txt");
        assert_eq!(hits[0].line, 2);
        assert_eq!(hits[0].col, 1);
        assert_eq!(hits[1].line, 4);

        let opened = "\
Find All (opened): \"foo\"
Documents: 2

=== a.txt ===
1:1:foo

=== b.txt ===
1:5:foo

— 2 match(es) in 2 document(s)";
        let hits = parse_find_results_hits(opened);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].doc_label, "a.txt");
        assert_eq!(hits[1].doc_label, "b.txt");
        assert_eq!(hits[1].col, 5);
    }

    #[test]
    fn goto_search_result_at_line_jumps_hit() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer =
            buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar\nfoo");
        state.tabs.active_mut().title = "sample.txt".into();
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let (body, n) = state.find_all_current_report().unwrap();
        assert_eq!(n, 2);
        present_find_results(&mut state, &body);

        let hits = parse_find_results_hits(&body);
        assert_eq!(hits.len(), 2);
        let second_line = hits[1].results_line;

        let mut ui = UiFlags::default();
        assert!(!goto_search_result_at_line(&mut state, &mut ui, 0));
        assert!(goto_search_result_at_line(&mut state, &mut ui, second_line));
        assert_eq!(state.tabs.active().title, "sample.txt");
        assert_eq!(state.tabs.active().buffer.selection(), Some((21, 24)));
        assert!(state.status.contains("4:1"));
        assert!(state.status.contains("2/2"));
        assert!(ui.follow_caret);
    }

    #[test]
    fn goto_search_result_walks_find_results_and_wraps() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer =
            buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar\nfoo");
        state.tabs.active_mut().title = "sample.txt".into();
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let (body, n) = state.find_all_current_report().unwrap();
        assert_eq!(n, 2);
        present_find_results(&mut state, &body);

        let mut ui = UiFlags::default();
        goto_search_result(&mut state, &mut ui, true);
        assert_eq!(state.tabs.active().title, "sample.txt");
        assert_eq!(state.tabs.active().buffer.selection(), Some((9, 12)));
        assert!(state.status.contains("2:1"));
        assert!(state.status.contains("1/2"));
        assert!(ui.follow_caret);

        goto_search_result(&mut state, &mut ui, true);
        assert_eq!(state.tabs.active().buffer.selection(), Some((21, 24)));
        assert!(state.status.contains("4:1"));
        assert!(state.status.contains("2/2"));
        assert!(!state.status.contains("wrapped"));

        goto_search_result(&mut state, &mut ui, true);
        assert_eq!(state.tabs.active().buffer.selection(), Some((9, 12)));
        assert!(state.status.contains("· wrapped"));

        goto_search_result(&mut state, &mut ui, false);
        assert_eq!(state.tabs.active().buffer.selection(), Some((21, 24)));
        assert!(state.status.contains("2/2"));
    }

    #[test]
    fn goto_search_result_opened_switches_document() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("foo one\n");
        state.tabs.active_mut().title = "a.txt".into();
        state.tabs.open_untitled();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("bar foo\n");
        state.tabs.active_mut().title = "b.txt".into();
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let (body, n) = state.find_all_opened_report().unwrap();
        assert_eq!(n, 2);
        present_find_results(&mut state, &body);

        let mut ui = UiFlags::default();
        goto_search_result(&mut state, &mut ui, true);
        assert_eq!(state.tabs.active().title, "a.txt");
        assert_eq!(state.tabs.active().buffer.selection(), Some((0, 3)));

        goto_search_result(&mut state, &mut ui, true);
        assert_eq!(state.tabs.active().title, "b.txt");
        assert_eq!(state.tabs.active().buffer.selection(), Some((4, 7)));
    }

    #[test]
    fn goto_search_result_walks_find_in_files_and_opens() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("npp-fif-goto-{stamp}"));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("src/a.rs"), "foo one\n").unwrap();
        std::fs::write(root.join("sub/b.txt"), "x\nbar foo\n").unwrap();

        let mut state = EditorState::new();
        state.workspace_root = root.clone();
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_regex = false;
        state.settings.find_files_include = String::new();
        state.settings.find_files_exclude = "target".into();

        let mut ui = UiFlags::default();
        find_in_files(&mut state, &mut ui);
        assert_eq!(state.tabs.active().title, "Find in Files");
        assert!(state
            .tabs
            .active()
            .buffer
            .to_string()
            .contains("src/a.rs:1:"));

        goto_search_result(&mut state, &mut ui, true);
        let first = state
            .tabs
            .active()
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
        assert!(
            first.ends_with("src/a.rs") || first.ends_with("sub/b.txt"),
            "status={}",
            state.status
        );
        assert!(state.tabs.active().buffer.selection().is_some());
        assert!(state.status.contains("1/"));
        assert!(!state.status.contains("wrapped"));

        goto_search_result(&mut state, &mut ui, true);
        let second = state
            .tabs
            .active()
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
        assert!(
            second.ends_with("src/a.rs") || second.ends_with("sub/b.txt"),
            "status={}",
            state.status
        );
        assert_ne!(first, second);
        assert!(state.status.contains("2/"));

        goto_search_result(&mut state, &mut ui, true);
        let wrapped = state
            .tabs
            .active()
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
        assert_eq!(wrapped, first);
        assert!(state.status.contains("· wrapped"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn goto_search_result_at_line_opens_find_in_files_hit() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("npp-fif-click-{stamp}"));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join("src/a.rs"), "foo one\n").unwrap();

        let mut state = EditorState::new();
        state.workspace_root = root.clone();
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_regex = false;
        state.settings.find_files_include = String::new();
        state.settings.find_files_exclude = "target".into();

        let mut ui = UiFlags::default();
        find_in_files(&mut state, &mut ui);
        assert_eq!(state.tabs.active().title, "Find in Files");
        let body = state.tabs.active().buffer.to_string();
        let hits = parse_find_results_hits(&body);
        assert!(!hits.is_empty());
        let line = hits[0].results_line;

        assert!(goto_search_result_at_line(&mut state, &mut ui, line));
        let path = state
            .tabs
            .active()
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
        assert!(path.ends_with("src/a.rs"), "status={}", state.status);
        assert_eq!(state.tabs.active().buffer.selection(), Some((0, 3)));
        assert!(ui.follow_caret);

        let _ = std::fs::remove_dir_all(&root);
    }
}
