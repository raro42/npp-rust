//! Shared helpers for menu command modules.
#![allow(dead_code)]

use super::{ComingSoon, UiFlags};
use crate::editor::EditorState;
use std::path::{Path, PathBuf};

pub(crate) fn open_url(state: &mut EditorState, url: &str) {
    let result = {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(url).status()
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", "", url])
                .status()
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            std::process::Command::new("xdg-open").arg(url).status()
        }
    };
    match result {
        Ok(s) if s.success() => state.status = format!("Opened {url}"),
        Ok(_) => state.status = format!("Could not open {url}"),
        Err(e) => state.status = format!("Open URL failed: {e}"),
    }
}

pub(crate) fn open_path_in_os(state: &mut EditorState, path: &Path) {
    let result = {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(path).status()
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("explorer").arg(path).status()
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            std::process::Command::new("xdg-open").arg(path).status()
        }
    };
    match result {
        Ok(s) if s.success() => state.status = "Opened folder".into(),
        Ok(_) => state.status = "Open folder failed".into(),
        Err(e) => state.status = format!("Open folder failed: {e}"),
    }
}

pub(crate) fn open_active_in_browser(state: &mut EditorState, cmd: &str) {
    let Some(path) = state.tabs.active().path.clone() else {
        state.status = "Save the file first, then open in browser".into();
        return;
    };
    let url = format!("file://{}", path.display());
    let app = match cmd {
        "IDM_VIEW_IN_FIREFOX" => Some("Firefox"),
        "IDM_VIEW_IN_CHROME" => Some("Google Chrome"),
        "IDM_VIEW_IN_EDGE" => Some("Microsoft Edge"),
        _ => None,
    };
    #[cfg(target_os = "macos")]
    {
        let result = if let Some(name) = app {
            std::process::Command::new("open")
                .args(["-a", name, &url])
                .status()
        } else {
            std::process::Command::new("open").arg(&url).status()
        };
        match result {
            Ok(s) if s.success() => state.status = "Opened in browser".into(),
            Ok(_) => open_url(state, &url),
            Err(_) => open_url(state, &url),
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
        open_url(state, &url);
    }
}

pub(crate) fn hash_via_cli(algo: &str, data_file: &Path) -> Result<String, String> {
    let output = match algo {
        "md5" => {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("md5")
                    .args(["-q"])
                    .arg(data_file)
                    .output()
            }
            #[cfg(not(target_os = "macos"))]
            {
                std::process::Command::new("md5sum").arg(data_file).output()
            }
        }
        "sha1" => std::process::Command::new("shasum")
            .args(["-a", "1"])
            .arg(data_file)
            .output(),
        "sha256" => std::process::Command::new("shasum")
            .args(["-a", "256"])
            .arg(data_file)
            .output(),
        "sha512" => std::process::Command::new("shasum")
            .args(["-a", "512"])
            .arg(data_file)
            .output(),
        other => return Err(format!("unknown algo {other}")),
    };
    match output {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout);
            Ok(s.split_whitespace().next().unwrap_or("").to_string())
        }
        Ok(o) => Err(String::from_utf8_lossy(&o.stderr).to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn hash_selection_or_doc(
    state: &mut EditorState,
    ui: &mut UiFlags,
    algo: &str,
    to_clip: bool,
) {
    let text = if let Some((s, e)) = state.tabs.active().buffer.selection() {
        state.tabs.active().buffer.slice(s, e)
    } else {
        state.tabs.active().buffer.to_string()
    };
    let tmp = std::env::temp_dir().join(format!("npp-rs-hash-{algo}.txt"));
    if let Err(e) = std::fs::write(&tmp, text.as_bytes()) {
        state.status = format!("Hash failed: {e}");
        return;
    }
    match hash_via_cli(algo, &tmp) {
        Ok(h) => {
            let _ = std::fs::remove_file(&tmp);
            if to_clip {
                ui.pending_clipboard = Some(h.clone());
                state.status = format!("{algo}: copied to clipboard");
            } else if ensure_editable(state) {
                state.tabs.active_mut().buffer.insert(&format!("\n{h}\n"));
                state.mark_text_changed();
                state.status = format!("{algo}: inserted");
            }
        }
        Err(e) => state.status = format!("Hash failed: {e}"),
    }
}

pub(crate) fn hash_active_file(state: &mut EditorState, ui: &mut UiFlags, algo: &str) {
    let Some(path) = state.tabs.active().path.clone() else {
        state.status = "Hash from file: save first".into();
        return;
    };
    match hash_via_cli(algo, &path) {
        Ok(h) => {
            ui.pending_clipboard = Some(h.clone());
            state.status = format!("{algo} (file): {h}");
        }
        Err(e) => state.status = format!("Hash failed: {e}"),
    }
}

pub(crate) fn cut_selection(state: &mut EditorState, ui: &mut UiFlags) {
    if !ensure_editable(state) {
        return;
    }
    if let Some(text) = state.tabs.active().multi_sels_clipboard_text() {
        ui.pending_clipboard = Some(text.clone());
        ui.last_copied = Some(text.clone());
        let _ = state.tabs.active_mut().delete_backward_multi();
        state.mark_text_changed();
        state.status = format!("Cut {} chars (column)", text.chars().count());
        return;
    }
    if let Some((s, e)) = state.tabs.active().buffer.selection() {
        let text = state.tabs.active().buffer.slice(s, e);
        ui.pending_clipboard = Some(text.clone());
        ui.last_copied = Some(text.clone());
        state.tabs.active_mut().buffer.delete_backward();
        state.mark_text_changed();
        state.status = format!("Cut {} chars", text.chars().count());
    }
}

#[derive(Clone, Copy)]
pub(crate) enum NumSort {
    Integer,
    DecimalComma,
    DecimalDot,
}

pub(crate) fn line_num_key(line: &str, kind: NumSort) -> Option<f64> {
    let t = line.trim();
    match kind {
        NumSort::Integer => {
            let mut end = 0usize;
            let bytes = t.as_bytes();
            if bytes.first().is_some_and(|b| *b == b'+' || *b == b'-') {
                end = 1;
            }
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end == 0 || (end == 1 && !bytes[0].is_ascii_digit()) {
                None
            } else {
                t[..end].parse::<i64>().ok().map(|n| n as f64)
            }
        }
        NumSort::DecimalComma => {
            let norm = t.replace(',', ".");
            parse_leading_float(&norm)
        }
        NumSort::DecimalDot => parse_leading_float(t),
    }
}

pub(crate) fn parse_leading_float(s: &str) -> Option<f64> {
    let bytes = s.as_bytes();
    let mut end = 0usize;
    if bytes.first().is_some_and(|b| *b == b'+' || *b == b'-') {
        end = 1;
    }
    let mut seen_dot = false;
    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_digit() {
            end += 1;
        } else if b == b'.' && !seen_dot {
            seen_dot = true;
            end += 1;
        } else {
            break;
        }
    }
    if end == 0 || (end == 1 && !bytes[0].is_ascii_digit()) {
        None
    } else {
        s[..end].parse().ok()
    }
}

pub(crate) fn cmp_num_key(a: &str, b: &str, kind: NumSort) -> std::cmp::Ordering {
    match (line_num_key(a, kind), line_num_key(b, kind)) {
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.cmp(b),
    }
}

pub(crate) fn map_line_bodies(text: &str, mut f: impl FnMut(&str) -> String) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        let (body, eol) = if let Some(stripped) = line.strip_suffix("\r\n") {
            (stripped, "\r\n")
        } else if let Some(stripped) = line.strip_suffix('\n') {
            (stripped, "\n")
        } else {
            (line, "")
        };
        out.push_str(&f(body));
        out.push_str(eol);
    }
    out
}

pub(crate) fn spaces_to_tabs_leading(body: &str) -> String {
    let spaces = body.chars().take_while(|c| *c == ' ').count();
    let rest: String = body.chars().skip(spaces).collect();
    let tabs = spaces / 4;
    let rem = spaces % 4;
    format!("{}{}{rest}", "\t".repeat(tabs), " ".repeat(rem))
}

pub(crate) fn selected_text(state: &EditorState) -> Option<String> {
    let (s, e) = state.tabs.active().buffer.selection()?;
    let t = state.tabs.active().buffer.slice(s, e);
    let t = t.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

pub(crate) fn resolve_selected_path(state: &EditorState) -> Option<PathBuf> {
    let t = selected_text(state)?;
    let p = PathBuf::from(&t);
    if p.exists() {
        return Some(p);
    }
    if let Some(parent) = state.tabs.active().path.as_ref().and_then(|p| p.parent()) {
        let joined = parent.join(&t);
        if joined.exists() {
            return Some(joined);
        }
    }
    None
}

pub(crate) fn find_matching_brace(text: &str, caret: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return None;
    }
    let try_pos = |pos: usize| -> Option<(usize, char, i32)> {
        let c = *chars.get(pos)?;
        match c {
            '(' | '[' | '{' => Some((pos, c, 1)),
            ')' | ']' | '}' => Some((pos, c, -1)),
            _ => None,
        }
    };
    let last = chars.len() - 1;
    let (start, ch, dir) =
        try_pos(caret.min(last)).or_else(|| caret.checked_sub(1).and_then(try_pos))?;
    let (open, close) = match ch {
        '(' | ')' => ('(', ')'),
        '[' | ']' => ('[', ']'),
        _ => ('{', '}'),
    };
    let mut depth = 0i32;
    if dir > 0 {
        for (i, &c) in chars.iter().enumerate().skip(start) {
            if c == open {
                depth += 1;
            } else if c == close {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }
    } else {
        for i in (0..=start).rev() {
            let c = chars[i];
            if c == close {
                depth += 1;
            } else if c == open {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }
    }
    None
}

pub(crate) fn brace_span(text: &str, caret: usize) -> Option<(usize, usize)> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return None;
    }
    let try_pos = |pos: usize| -> Option<usize> {
        let c = *chars.get(pos)?;
        if matches!(c, '(' | ')' | '[' | ']' | '{' | '}') {
            Some(pos)
        } else {
            None
        }
    };
    let last = chars.len() - 1;
    let start = try_pos(caret.min(last)).or_else(|| caret.checked_sub(1).and_then(try_pos))?;
    let other = find_matching_brace(text, start)?;
    Some((start, other))
}

pub(crate) fn filter_lines_by_bookmarks(state: &mut EditorState, keep_unmarked: bool) {
    if !ensure_editable(state) {
        return;
    }
    let text = state.tabs.active().buffer.to_string();
    let marks = state.tabs.active().bookmarks.clone();
    let mut kept = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let marked = marks.contains(&i);
        if keep_unmarked {
            if !marked {
                kept.push(line);
            }
        } else if marked {
            kept.push(line);
        }
    }
    let mut out = kept.join("\n");
    if text.ends_with('\n') {
        out.push('\n');
    }
    state.tabs.active_mut().buffer.replace_document(&out);
    state.tabs.active_mut().bookmarks.clear();
    state.mark_text_changed();
    state.status = if keep_unmarked {
        "Removed bookmarked lines".into()
    } else {
        "Removed non-bookmarked lines".into()
    };
}

/// Run `f` on tab `tab`'s buffer when editable. Sets status when blocked.
pub(crate) fn with_editable_buffer<R>(
    state: &mut EditorState,
    tab: usize,
    f: impl FnOnce(&mut buffer::TextBuffer) -> R,
) -> Result<R, doc::EditDenied> {
    let denied = state.tabs.get(tab).and_then(|d| d.edit_denied());
    if let Some(denied) = denied {
        state.status = denied.status_message().into();
        return Err(denied);
    }
    let Some(doc) = state.tabs.get_mut(tab) else {
        return Err(doc::EditDenied::Loading);
    };
    Ok(f(&mut doc.buffer))
}

/// Return false and set status when the active document cannot be edited.
pub(crate) fn ensure_editable(state: &mut EditorState) -> bool {
    with_editable_buffer(state, state.tabs.active_index(), |_| ()).is_ok()
}

/// Replace each bookmarked line with `clip` (Notepad++ Paste to Bookmarked Lines).
pub fn paste_over_bookmarked_lines(state: &mut EditorState, clip: &str) {
    if !ensure_editable(state) {
        return;
    }
    let marks = state.tabs.active().bookmarks.clone();
    if marks.is_empty() {
        state.status = "No bookmarks".into();
        return;
    }
    let text = state.tabs.active().buffer.to_string();
    let mut out: Vec<String> = text.lines().map(|l| l.to_string()).collect();
    let had_trailing = text.ends_with('\n');
    for &i in &marks {
        if let Some(slot) = out.get_mut(i) {
            *slot = clip.to_string();
        }
    }
    let mut joined = out.join("\n");
    if had_trailing {
        joined.push('\n');
    }
    state.tabs.active_mut().buffer.replace_document(&joined);
    state.mark_text_changed();
    state.status = format!("Pasted onto {} bookmarked line(s)", marks.len());
}

pub(crate) fn line_comment_prefix(lang: &str) -> Option<&'static str> {
    match lang {
        "html" | "xml" => None,
        "rust" | "c" | "cpp" | "java" | "javascript" | "typescript" | "go" | "json" | "css" => {
            Some("// ")
        }
        "python" | "shell" | "yaml" | "toml" | "ruby" | "perl" | "r" => Some("# "),
        "sql" | "lua" | "haskell" => Some("-- "),
        "matlab" | "octave" => Some("% "),
        "plain" | "text" | "markdown" => Some("# "),
        _ => Some("// "),
    }
}

pub(crate) fn stream_comment_delims(lang: &str) -> Option<(&'static str, &'static str)> {
    match lang {
        "html" | "xml" | "markdown" => Some(("<!--", "-->")),
        "python" => Some(("\"\"\"", "\"\"\"")),
        "plain" | "text" | "shell" | "yaml" | "toml" | "sql" => None,
        _ => Some(("/*", "*/")),
    }
}

pub(crate) fn tab_type_key(doc: &doc::Document) -> String {
    doc.path
        .as_ref()
        .and_then(|p| p.extension())
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_else(|| doc.language.to_lowercase())
}

pub(crate) fn tab_mtime(doc: &doc::Document) -> u64 {
    doc.path
        .as_ref()
        .and_then(|p| std::fs::metadata(p).ok())
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub(crate) fn copy_selection(state: &mut EditorState, ui: &mut UiFlags) {
    if let Some(text) = state.tabs.active().multi_sels_clipboard_text() {
        ui.pending_clipboard = Some(text.clone());
        ui.last_copied = Some(text);
        state.status = "Copied (column)".into();
        return;
    }
    if let Some((s, e)) = state.tabs.active().buffer.selection() {
        let text = state.tabs.active().buffer.slice(s, e);
        ui.pending_clipboard = Some(text.clone());
        ui.last_copied = Some(text);
        state.status = "Copied".into();
    }
}

/// Human-ish feature name from an `IDM_*` id.
pub fn feature_name_from_cmd(cmd: &str) -> String {
    let raw = cmd.strip_prefix("IDM_").unwrap_or(cmd);
    raw.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => {
                    f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn coming_soon_for(cmd: &str) -> ComingSoon {
    ComingSoon {
        cmd: cmd.to_string(),
        feature: feature_name_from_cmd(cmd),
    }
}

/// Short smile lines — pick by hashing the command so the same item feels consistent.
pub fn coming_soon_blurb(cmd: &str) -> &'static str {
    const LINES: &[&str] = &[
        "We’re polishing this one with care. Come back tomorrow — it’ll be friendlier.",
        "Not ready yet, but the elves are typing. See you tomorrow!",
        "Almost there. Sleep well; tomorrow this button gets a real job.",
        "Still in the workshop. Check again tomorrow — we owe you a smile.",
        "Good eye! This feature is on the bench. Tomorrow’s build loves you more.",
        "Patience, explorer. Tomorrow we turn this stub into magic.",
        "Noted in Ralf’s side-project notebook. Tomorrow: progress. Today: coffee.",
        "The menu is honest; the code is catching up. See you tomorrow!",
    ];
    let mut h: u32 = 0;
    for b in cmd.bytes() {
        h = h.wrapping_mul(31).wrapping_add(u32::from(b));
    }
    LINES[(h as usize) % LINES.len()]
}

/// Launch another process of this app; optionally pass the active file path.
pub(crate) fn open_in_new_instance(state: &mut EditorState, move_doc: bool) {
    let path = state.tabs.active().path.clone();
    let Ok(exe) = std::env::current_exe() else {
        state.status = "New instance: cannot find this app".into();
        return;
    };
    let mut cmd = std::process::Command::new(&exe);
    if let Some(ref p) = path {
        cmd.arg(p);
    }
    match cmd.spawn() {
        Ok(_) => {
            if move_doc {
                let idx = state.tabs.active_index();
                state.close_tab(idx);
                state.status = "Moved to new instance".into();
            } else {
                state.status = "Opened in new instance".into();
            }
        }
        Err(e) => state.status = format!("New instance failed: {e}"),
    }
}

/// Indent depth in units of 4 spaces (tabs count as 4).
pub(crate) fn line_indent_level(line: &str) -> usize {
    let mut spaces = 0usize;
    for c in line.chars() {
        match c {
            ' ' => spaces += 1,
            '\t' => spaces += 4,
            _ => break,
        }
    }
    spaces / 4
}

pub(crate) fn hide_selected_or_current_lines(state: &mut EditorState) {
    let (start_line, end_line) = {
        let buf = &state.tabs.active().buffer;
        if let Some((s, e)) = buf.selection() {
            let a = buf.char_to_line(s.min(e));
            let end = s.max(e);
            let line = buf.char_to_line(end);
            let b = if end > 0 && end == buf.line_to_char(line) && line > a {
                line - 1
            } else {
                line
            };
            (a, b)
        } else {
            let line = buf.char_to_line(buf.caret());
            (line, line)
        }
    };
    let hidden = &mut state.tabs.active_mut().hidden_lines;
    for i in start_line..=end_line {
        hidden.insert(i);
    }
    let n = end_line.saturating_sub(start_line) + 1;
    state.status = format!("Hidden {n} line(s)");
}

pub(crate) fn fold_all_by_indent(state: &mut EditorState) {
    let n = state.tabs.active().buffer.line_count();
    let mut to_hide = Vec::new();
    for i in 0..n {
        let raw = state.tabs.active().buffer.line(i);
        if line_indent_level(&raw) > 0 {
            to_hide.push(i);
        }
    }
    let hidden = &mut state.tabs.active_mut().hidden_lines;
    for i in to_hide {
        hidden.insert(i);
    }
    let count = state.tabs.active().hidden_lines.len();
    state.status = format!("Fold all (indent): {count} line(s) hidden");
}

pub(crate) fn unfold_all_hidden(state: &mut EditorState) {
    let n = state.tabs.active().hidden_lines.len();
    state.tabs.active_mut().hidden_lines.clear();
    state.status = format!("Unfold all: showed {n} line(s)");
}

fn current_indent_block(state: &EditorState) -> (usize, usize) {
    let buf = &state.tabs.active().buffer;
    let start = buf.char_to_line(buf.caret());
    let base = line_indent_level(&buf.line(start));
    let n = buf.line_count();
    let mut end = start;
    for i in (start + 1)..n {
        let raw = buf.line(i);
        if raw.trim().is_empty() {
            end = i;
            continue;
        }
        if line_indent_level(&raw) > base {
            end = i;
        } else {
            break;
        }
    }
    (start, end)
}

pub(crate) fn fold_current_block(state: &mut EditorState) {
    let (start, end) = current_indent_block(state);
    if end <= start {
        state.status = "Fold current: nothing to fold".into();
        return;
    }
    let hidden = &mut state.tabs.active_mut().hidden_lines;
    for i in (start + 1)..=end {
        hidden.insert(i);
    }
    state.status = format!("Folded {} line(s)", end - start);
}

pub(crate) fn unfold_current_block(state: &mut EditorState) {
    let (start, end) = current_indent_block(state);
    let hidden = &mut state.tabs.active_mut().hidden_lines;
    let before = hidden.len();
    for i in start..=end {
        hidden.remove(&i);
    }
    let shown = before.saturating_sub(hidden.len());
    state.status = format!("Unfolded {shown} line(s)");
}

pub(crate) fn fold_indent_level(state: &mut EditorState, level: usize) {
    let n = state.tabs.active().buffer.line_count();
    let mut to_hide = Vec::new();
    for i in 0..n {
        let raw = state.tabs.active().buffer.line(i);
        if line_indent_level(&raw) >= level {
            to_hide.push(i);
        }
    }
    let hidden = &mut state.tabs.active_mut().hidden_lines;
    let mut added = 0usize;
    for i in to_hide {
        if hidden.insert(i) {
            added += 1;
        }
    }
    state.status = format!("Fold level {level}: hid {added} line(s)");
}

pub(crate) fn unfold_indent_level(state: &mut EditorState, level: usize) {
    let n = state.tabs.active().buffer.line_count();
    let mut to_show = Vec::new();
    for i in 0..n {
        let raw = state.tabs.active().buffer.line(i);
        if line_indent_level(&raw) == level {
            to_show.push(i);
        }
    }
    let hidden = &mut state.tabs.active_mut().hidden_lines;
    let mut shown = 0usize;
    for i in to_show {
        if hidden.remove(&i) {
            shown += 1;
        }
    }
    state.status = format!("Unfold level {level}: showed {shown} line(s)");
}
