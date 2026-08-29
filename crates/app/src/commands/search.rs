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
            find_in_files(state, ui);
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
        "IDM_SEARCH_CHANGED_NEXT" => {
            goto_dirty_tab(state, ui, true);
            CmdResult::Handled
        }
        "IDM_SEARCH_CHANGED_PREV" => {
            goto_dirty_tab(state, ui, false);
            CmdResult::Handled
        }
        "IDM_SEARCH_CLEAR_CHANGE_HISTORY" => {
            // Stand-in: clear selection. Real edit marks need editor.rs.
            let buf = &mut state.tabs.active_mut().buffer;
            if buf.selection().is_some() {
                buf.clear_selection();
                state.status = "Cleared selection (change-history stand-in)".into();
            } else {
                state.status =
                    "No selection to clear (change-history stand-in)".into();
            }
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
    for i in from..n {
        let cp = chars[i] as u32;
        if cp >= lo && cp <= hi {
            found = Some(i);
            break;
        }
    }
    if found.is_none() {
        for i in 0..from.min(n) {
            let cp = chars[i] as u32;
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

/// Scan cwd (shallow) for `find_query`; write hits into a new untitled results tab.
fn find_in_files(state: &mut EditorState, ui: &mut UiFlags) {
    let q = state.find_query.clone();
    if q.is_empty() {
        ui.find_open = true;
        ui.show_replace = false;
        ui.find_focus_once = true;
        state.status = "Find in Files: set Find text, then run again".into();
        return;
    }
    let cwd = match std::env::current_dir() {
        Ok(p) => p,
        Err(_) => {
            state.status = "Find in Files: cannot read working directory".into();
            return;
        }
    };
    const MAX_FILE_BYTES: u64 = 512 * 1024;
    const MAX_MATCHES: usize = 500;
    const MAX_FILES: usize = 200;

    let mut lines_out: Vec<String> = Vec::new();
    lines_out.push(format!("Find in Files: {q:?}"));
    lines_out.push("Directory: . (process cwd)".into());
    lines_out.push(String::new());

    let mut files_ok = 0usize;
    let mut match_count = 0usize;
    let Ok(rd) = std::fs::read_dir(&cwd) else {
        state.status = "Find in Files: cannot list working directory".into();
        return;
    };
    let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        if files_ok >= MAX_FILES || match_count >= MAX_MATCHES {
            break;
        }
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') {
            continue;
        }
        if !path.is_file() {
            continue;
        }
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.len() > MAX_FILE_BYTES {
            continue;
        }
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        if bytes.contains(&0) {
            continue;
        }
        let Ok(text) = String::from_utf8(bytes) else {
            continue;
        };
        files_ok += 1;
        let rel = name.to_string();
        for (li, line) in text.lines().enumerate() {
            if match_count >= MAX_MATCHES {
                break;
            }
            if line.contains(&q) {
                lines_out.push(format!("{rel}:{}:{line}", li + 1));
                match_count += 1;
            }
        }
    }

    lines_out.push(String::new());
    lines_out.push(format!(
        "— {match_count} match(es) in {files_ok} file(s) (cwd only, max {MAX_MATCHES})"
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
    state.status = format!("Find in Files: {match_count} match(es) in {files_ok} file(s)");
}

/// Stand-in for change history: jump among dirty tabs (real marks need editor.rs).
fn goto_dirty_tab(state: &mut EditorState, ui: &mut UiFlags, forward: bool) {
    let dirty: Vec<usize> = (0..state.tabs.len())
        .filter(|&i| state.tabs.get(i).map(|d| d.dirty).unwrap_or(false))
        .collect();
    if dirty.is_empty() {
        state.status = "No dirty documents".into();
        return;
    }
    let cur = state.tabs.active_index();
    let next = if forward {
        dirty
            .iter()
            .copied()
            .find(|&i| i > cur)
            .or_else(|| dirty.first().copied())
    } else {
        dirty
            .iter()
            .rev()
            .copied()
            .find(|&i| i < cur)
            .or_else(|| dirty.last().copied())
    };
    let Some(i) = next else {
        state.status = "No dirty documents".into();
        return;
    };
    state.tabs.set_active(i);
    state.highlight_dirty = true;
    ui.follow_caret = true;
    let title = state.tabs.active().title.clone();
    state.status = format!("Dirty document: {title}");
}
