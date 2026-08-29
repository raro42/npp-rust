//! Edit menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// Session text-direction preference. Layout paint stays LTR until ui_paint reads this.
static TEXT_RTL: AtomicBool = AtomicBool::new(false);

/// True when the session asks for right-to-left layout.
pub fn text_is_rtl() -> bool {
    TEXT_RTL.load(Ordering::Relaxed)
}

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_EDIT_")
}

/// True when this edit command may change buffer text.
fn mutates_buffer(cmd: &str) -> bool {
    !matches!(
        cmd,
        "IDM_EDIT_COPY"
            | "IDM_EDIT_SELECTALL"
            | "IDM_EDIT_FULLPATHTOCLIP"
            | "IDM_EDIT_FILENAMETOCLIP"
            | "IDM_EDIT_CURRENTDIRTOCLIP"
            | "IDM_EDIT_COPY_ALL_NAMES"
            | "IDM_EDIT_COPY_ALL_PATHS"
            | "IDM_EDIT_BEGINENDSELECT"
            | "IDM_EDIT_BEGINENDSELECT_COLUMNMODE"
            | "IDM_EDIT_OPENSELECTEDFILETOEDIT"
            | "IDM_EDIT_OPENSELECTEDFILEFOLDERINEXPLORER"
            | "IDM_EDIT_SEARCHONINTERNET"
            | "IDM_EDIT_CHANGESEARCHENGINE"
            | "IDM_EDIT_TOGGLEREADONLY"
            | "IDM_EDIT_SETREADONLYFORALLDOCS"
            | "IDM_EDIT_CLEARREADONLYFORALLDOCS"
            | "IDM_EDIT_RTL"
            | "IDM_EDIT_LTR"
            | "IDM_EDIT_COPY_BINARY"
            | "IDM_EDIT_MULTISELECTALL"
            | "IDM_EDIT_MULTISELECTALLMATCHCASE"
            | "IDM_EDIT_MULTISELECTALLWHOLEWORD"
            | "IDM_EDIT_MULTISELECTALLMATCHCASEWHOLEWORD"
            | "IDM_EDIT_MULTISELECTNEXT"
            | "IDM_EDIT_MULTISELECTNEXTMATCHCASE"
            | "IDM_EDIT_MULTISELECTNEXTWHOLEWORD"
            | "IDM_EDIT_MULTISELECTNEXTMATCHCASEWHOLEWORD"
            | "IDM_EDIT_MULTISELECTUNDO"
            | "IDM_EDIT_MULTISELECTSSKIP"
            | "IDM_EDIT_COLUMNMODETIP"
            | "IDM_EDIT_CHAR_PANEL"
            | "IDM_EDIT_CLIPBOARDHISTORY_PANEL"
            | "IDM_EDIT_TOGGLESYSTEMREADONLY"
            | "IDM_EDIT_FUNCCALLTIP"
            | "IDM_EDIT_FUNCCALLTIP_PREVIOUS"
            | "IDM_EDIT_FUNCCALLTIP_NEXT"
    )
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    if mutates_buffer(cmd) && !ensure_editable(state) {
        return Some(CmdResult::Handled);
    }
    Some(match cmd {
        "IDM_EDIT_UNDO" => {
            state.undo();
            CmdResult::Handled
        }
        "IDM_EDIT_REDO" => {
            state.redo();
            CmdResult::Handled
        }
        "IDM_EDIT_CUT" => {
            cut_selection(state, ui);
            CmdResult::Handled
        }
        "IDM_EDIT_COPY" => {
            copy_selection(state, ui);
            CmdResult::Handled
        }
        "IDM_EDIT_PASTE" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
            } else if let Some(t) = ui.last_copied.clone() {
                if t.is_empty() {
                    state.status = "Paste: clipboard empty".into();
                } else {
                    state.tabs.active_mut().buffer.insert(&t);
                    state.mark_text_changed();
                    ui.follow_caret = true;
                    state.status = "Pasted".into();
                }
            } else {
                state.status = "Paste: copy text first, or use ⌘/Ctrl+V".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_DELETE" => {
            state.tabs.active_mut().buffer.delete_forward();
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_SELECTALL" => {
            state.tabs.active_mut().buffer.select_all();
            CmdResult::Handled
        }
        "IDM_EDIT_INS_TAB" => {
            {
                let n = state.settings.tab_width.max(1) as usize;
                let pad = " ".repeat(n);
                state.tabs.active_mut().buffer.indent_lines(&pad);
            }
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_RMV_TAB" => {
            state.tabs.active_mut().buffer.outdent_lines(4);
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_UPPERCASE" => {
            state.run_plugin("edit.uppercase");
            CmdResult::Handled
        }
        "IDM_EDIT_LOWERCASE" => {
            state.run_plugin("edit.lowercase");
            CmdResult::Handled
        }
        "IDM_EDIT_INVERTCASE" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                s.chars()
                    .map(|c| {
                        if c.is_uppercase() {
                            c.to_lowercase().collect::<String>()
                        } else if c.is_lowercase() {
                            c.to_uppercase().collect::<String>()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect()
            });
            state.mark_text_changed();
            state.status = "Invert case".into();
            CmdResult::Handled
        }
        "IDM_EDIT_PROPERCASE_FORCE" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut out = String::new();
                let mut new_word = true;
                for c in s.chars() {
                    if c.is_alphabetic() {
                        if new_word {
                            out.extend(c.to_uppercase());
                            new_word = false;
                        } else {
                            out.extend(c.to_lowercase());
                        }
                    } else {
                        out.push(c);
                        new_word = !c.is_alphanumeric();
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Proper Case".into();
            CmdResult::Handled
        }
        "IDM_EDIT_PROPERCASE_BLEND" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut out = String::new();
                let mut new_word = true;
                for c in s.chars() {
                    if c.is_alphabetic() {
                        if new_word {
                            out.extend(c.to_uppercase());
                            new_word = false;
                        } else {
                            out.push(c);
                        }
                    } else {
                        out.push(c);
                        new_word = !c.is_alphanumeric();
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Proper Case (blend)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_DUP_LINE" => {
            state.tabs.active_mut().buffer.duplicate_line();
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_JOIN_LINES" => {
            state.tabs.active_mut().buffer.join_lines();
            state.mark_text_changed();
            state.status = "Join lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_LINE_UP" => {
            state.tabs.active_mut().buffer.move_line_up();
            state.mark_text_changed();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_EDIT_LINE_DOWN" => {
            state.tabs.active_mut().buffer.move_line_down();
            state.mark_text_changed();
            ui.follow_caret = true;
            CmdResult::Handled
        }
        "IDM_EDIT_BLANKLINEABOVECURRENT" => {
            state.tabs.active_mut().buffer.blank_line_above();
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_BLANKLINEBELOWCURRENT" => {
            let line = state
                .tabs
                .active()
                .buffer
                .char_to_line(state.tabs.active().buffer.caret());
            let at = if line + 1 < state.tabs.active().buffer.line_count() {
                state.tabs.active().buffer.line_to_char(line + 1)
            } else {
                state.tabs.active().buffer.len_chars()
            };
            state.tabs.active_mut().buffer.set_caret(at);
            state.tabs.active_mut().buffer.insert("\n");
            state.mark_text_changed();
            CmdResult::Handled
        }
        "IDM_EDIT_SPLIT_LINES" => {
            // Split at caret: insert newline (Notepad++ wraps selection; we insert NL).
            state.tabs.active_mut().buffer.insert("\n");
            state.mark_text_changed();
            state.status = "Split line".into();
            CmdResult::Handled
        }
        "IDM_EDIT_REMOVE_CONSECUTIVE_DUP_LINES" | "IDM_EDIT_REMOVE_ANY_DUP_LINES" => {
            let any = cmd == "IDM_EDIT_REMOVE_ANY_DUP_LINES";
            let text = state.tabs.active().buffer.to_string();
            let mut out_lines = Vec::new();
            let mut seen = std::collections::HashSet::new();
            let mut prev: Option<String> = None;
            for line in text.lines() {
                let key = line.to_string();
                if any {
                    if seen.insert(key.clone()) {
                        out_lines.push(key);
                    }
                } else if prev.as_ref() != Some(&key) {
                    out_lines.push(key.clone());
                    prev = Some(key);
                }
            }
            let mut out = out_lines.join("\n");
            if text.ends_with('\n') {
                out.push('\n');
            }
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Removed duplicate lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SORTLINES_LEXICOGRAPHIC_ASCENDING"
        | "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_ASCENDING"
        | "IDM_EDIT_SORTLINES_LOCALE_ASCENDING"
        | "IDM_EDIT_SORTLINES_LOCALE_DESCENDING"
        | "IDM_EDIT_SORTLINES_REVERSE_ORDER"
        | "IDM_EDIT_SORTLINES_LENGTH_ASCENDING"
        | "IDM_EDIT_SORTLINES_LENGTH_DESCENDING"
        | "IDM_EDIT_SORTLINES_LEXICOGRAPHIC_DESCENDING"
        | "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_DESCENDING"
        | "IDM_EDIT_SORTLINES_INTEGER_ASCENDING"
        | "IDM_EDIT_SORTLINES_INTEGER_DESCENDING"
        | "IDM_EDIT_SORTLINES_DECIMALCOMMA_ASCENDING"
        | "IDM_EDIT_SORTLINES_DECIMALCOMMA_DESCENDING"
        | "IDM_EDIT_SORTLINES_DECIMALDOT_ASCENDING"
        | "IDM_EDIT_SORTLINES_DECIMALDOT_DESCENDING"
        | "IDM_EDIT_SORTLINES_RANDOMLY" => {
            let text = state.tabs.active().buffer.to_string();
            let mut lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
            match cmd {
                "IDM_EDIT_SORTLINES_REVERSE_ORDER" => lines.reverse(),
                "IDM_EDIT_SORTLINES_LENGTH_ASCENDING" => {
                    lines.sort_by_key(|l| l.chars().count());
                }
                "IDM_EDIT_SORTLINES_LENGTH_DESCENDING" => {
                    lines.sort_by_key(|b| std::cmp::Reverse(b.chars().count()));
                }
                "IDM_EDIT_SORTLINES_LEXICOGRAPHIC_DESCENDING"
                | "IDM_EDIT_SORTLINES_LOCALE_DESCENDING" => {
                    lines.sort();
                    lines.reverse();
                }
                "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_DESCENDING" => {
                    lines.sort_by_key(|l| l.to_ascii_lowercase());
                    lines.reverse();
                }
                "IDM_EDIT_SORTLINES_INTEGER_ASCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(a, b, NumSort::Integer));
                }
                "IDM_EDIT_SORTLINES_INTEGER_DESCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(b, a, NumSort::Integer));
                }
                "IDM_EDIT_SORTLINES_DECIMALCOMMA_ASCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(a, b, NumSort::DecimalComma));
                }
                "IDM_EDIT_SORTLINES_DECIMALCOMMA_DESCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(b, a, NumSort::DecimalComma));
                }
                "IDM_EDIT_SORTLINES_DECIMALDOT_ASCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(a, b, NumSort::DecimalDot));
                }
                "IDM_EDIT_SORTLINES_DECIMALDOT_DESCENDING" => {
                    lines.sort_by(|a, b| cmp_num_key(b, a, NumSort::DecimalDot));
                }
                "IDM_EDIT_SORTLINES_RANDOMLY" => {
                    // Simple deterministic shuffle from content hash (no extra crate).
                    let mut seed: u64 = lines.len() as u64;
                    for l in &lines {
                        for b in l.bytes() {
                            seed = seed.wrapping_mul(31).wrapping_add(u64::from(b));
                        }
                    }
                    for i in (1..lines.len()).rev() {
                        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                        let j = (seed as usize) % (i + 1);
                        lines.swap(i, j);
                    }
                }
                "IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_ASCENDING" => {
                    lines.sort_by_key(|l| l.to_ascii_lowercase());
                }
                _ => lines.sort(),
            }
            let mut out = lines.join("\n");
            if text.ends_with('\n') {
                out.push('\n');
            }
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Sorted lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SENTENCECASE_FORCE" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut out = String::new();
                let mut cap = true;
                for c in s.chars() {
                    if cap && c.is_alphabetic() {
                        out.extend(c.to_uppercase());
                        cap = false;
                    } else {
                        out.extend(c.to_lowercase());
                        if matches!(c, '.' | '!' | '?') {
                            cap = true;
                        }
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Sentence case".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SENTENCECASE_BLEND" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut out = String::new();
                let mut cap = true;
                for c in s.chars() {
                    if cap && c.is_alphabetic() {
                        out.extend(c.to_uppercase());
                        cap = false;
                    } else {
                        out.push(c);
                        if matches!(c, '.' | '!' | '?') {
                            cap = true;
                        }
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Sentence case (blend)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_RANDOMCASE" => {
            state.tabs.active_mut().buffer.map_text(|s| {
                let mut seed: u64 = s.len() as u64;
                let mut out = String::new();
                for c in s.chars() {
                    seed = seed.wrapping_mul(31).wrapping_add(c as u64);
                    if c.is_alphabetic() {
                        if seed & 1 == 0 {
                            out.extend(c.to_uppercase());
                        } else {
                            out.extend(c.to_lowercase());
                        }
                    } else {
                        out.push(c);
                    }
                }
                out
            });
            state.mark_text_changed();
            state.status = "Random case".into();
            CmdResult::Handled
        }
        "IDM_EDIT_REMOVEEMPTYLINES" => {
            state.tabs.active_mut().buffer.remove_empty_lines(false);
            state.mark_text_changed();
            state.status = "Removed empty lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_REMOVEEMPTYLINESWITHBLANK" => {
            state.tabs.active_mut().buffer.remove_empty_lines(true);
            state.mark_text_changed();
            state.status = "Removed blank lines".into();
            CmdResult::Handled
        }
        "IDM_EDIT_INSERT_DATETIME_SHORT" => {
            state.insert_datetime(false);
            CmdResult::Handled
        }
        "IDM_EDIT_INSERT_DATETIME_LONG" => {
            state.insert_datetime(true);
            CmdResult::Handled
        }
        "IDM_EDIT_INSERT_DATETIME_CUSTOMIZED" => {
            state.insert_datetime_custom();
            CmdResult::Handled
        }
        "IDM_EDIT_BLOCK_COMMENT" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            let lang = state.tabs.active().language.clone();
            if let Some(prefix) = line_comment_prefix(&lang) {
                state.tabs.active_mut().buffer.toggle_line_comments(prefix);
                state.mark_text_changed();
                state.status = "Toggled line comment".into();
            } else {
                state.status = "No line comment for this language".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_BLOCK_COMMENT_SET" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            let lang = state.tabs.active().language.clone();
            if let Some(prefix) = line_comment_prefix(&lang) {
                state.tabs.active_mut().buffer.comment_lines(prefix);
                state.mark_text_changed();
                state.status = "Commented lines".into();
            } else {
                state.status = "No line comment for this language".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_BLOCK_UNCOMMENT" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            let lang = state.tabs.active().language.clone();
            if let Some(prefix) = line_comment_prefix(&lang) {
                state.tabs.active_mut().buffer.uncomment_lines(prefix);
                state.mark_text_changed();
                state.status = "Uncommented lines".into();
            } else {
                state.status = "No line comment for this language".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_STREAM_COMMENT" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            let lang = state.tabs.active().language.clone();
            if let Some((open, close)) = stream_comment_delims(&lang) {
                state.tabs.active_mut().buffer.stream_comment(open, close);
                state.mark_text_changed();
                state.status = "Block comment applied".into();
            } else {
                state.status = "No block comment for this language".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_STREAM_UNCOMMENT" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            let lang = state.tabs.active().language.clone();
            if let Some((open, close)) = stream_comment_delims(&lang) {
                state.tabs.active_mut().buffer.stream_uncomment(open, close);
                state.mark_text_changed();
                state.status = "Block comment removed".into();
            } else {
                state.status = "No block comment for this language".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_FULLPATHTOCLIP" => {
            if let Some(p) = state.tabs.active().path.clone() {
                ui.pending_clipboard = Some(p.display().to_string());
                state.status = "Full path copied".into();
            } else {
                state.status = "No path to copy".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_FILENAMETOCLIP" => {
            ui.pending_clipboard = Some(state.tabs.active().title.clone());
            state.status = "File name copied".into();
            CmdResult::Handled
        }
        "IDM_EDIT_CURRENTDIRTOCLIP" => {
            if let Some(p) = state.tabs.active().path.as_ref().and_then(|p| p.parent()) {
                ui.pending_clipboard = Some(p.display().to_string());
                state.status = "Directory copied".into();
            } else {
                state.status = "No directory to copy".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_COPY_ALL_NAMES" => {
            let names: Vec<_> = state.tabs.iter().map(|d| d.title.clone()).collect();
            ui.pending_clipboard = Some(names.join("\n"));
            state.status = "All tab names copied".into();
            CmdResult::Handled
        }
        "IDM_EDIT_COPY_ALL_PATHS" => {
            let paths: Vec<_> = state
                .tabs
                .iter()
                .map(|d| {
                    d.path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| d.title.clone())
                })
                .collect();
            ui.pending_clipboard = Some(paths.join("\n"));
            state.status = "All paths copied".into();
            CmdResult::Handled
        }
        "IDM_EDIT_BEGINENDSELECT" | "IDM_EDIT_BEGINENDSELECT_COLUMNMODE" => {
            let caret = state.tabs.active().buffer.caret();
            match state.begin_end_select {
                None => {
                    state.begin_end_select = Some(caret);
                    state.status = "Begin/End Select: start set".into();
                }
                Some(anchor) => {
                    state.tabs.active_mut().buffer.set_selection(anchor, caret);
                    state.begin_end_select = None;
                    ui.follow_caret = true;
                    state.status = "Begin/End Select: selection set".into();
                }
            }
            CmdResult::Handled
        }
        "IDM_EDIT_OPENSELECTEDFILETOEDIT" => {
            if let Some(path) = resolve_selected_path(state) {
                state.open_path(path);
            } else {
                state.status = "Open selection: no existing path in selection".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_OPENSELECTEDFILEFOLDERINEXPLORER" => {
            if let Some(path) = resolve_selected_path(state) {
                let folder = if path.is_dir() {
                    path
                } else {
                    path.parent().map(Path::to_path_buf).unwrap_or(path)
                };
                open_path_in_os(state, &folder);
            } else {
                state.status = "Open folder: no existing path in selection".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_SEARCHONINTERNET" => {
            let q = if let Some((s, e)) = state.tabs.active().buffer.selection() {
                state.tabs.active().buffer.slice(s, e)
            } else {
                String::new()
            };
            let q = q.trim();
            if q.is_empty() {
                state.status = "Search on Internet: select text first".into();
            } else {
                let enc: String = {
                    let mut out = String::new();
                    for b in q.as_bytes() {
                        match *b {
                            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                                out.push(*b as char);
                            }
                            b' ' => out.push('+'),
                            _ => out.push_str(&format!("%{b:02X}")),
                        }
                    }
                    out
                };
                let url = format!("{}{enc}", state.search_engine);
                open_url(state, &url);
            }
            CmdResult::Handled
        }
        "IDM_EDIT_CHANGESEARCHENGINE" => {
            state.search_engine = if state.search_engine.contains("duckduckgo") {
                "https://www.google.com/search?q=".into()
            } else if state.search_engine.contains("google") {
                "https://www.bing.com/search?q=".into()
            } else {
                "https://duckduckgo.com/?q=".into()
            };
            state.status = format!("Search engine: {}", state.search_engine);
            CmdResult::Handled
        }
        "IDM_EDIT_REDACT_SELECTION" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            if let Some((s, e)) = state.tabs.active().buffer.selection() {
                let n = e.saturating_sub(s);
                let block = "█".repeat(n.max(1));
                state.tabs.active_mut().buffer.insert(&block);
                state.mark_text_changed();
                state.status = "Redacted selection".into();
            } else {
                state.status = "Redact: select text first".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_TOGGLEREADONLY" => {
            let d = state.tabs.active_mut();
            d.read_only = !d.read_only;
            state.status = if d.read_only {
                "Read-only: on".into()
            } else {
                "Read-only: off".into()
            };
            CmdResult::Handled
        }
        "IDM_EDIT_SETREADONLYFORALLDOCS" => {
            for i in 0..state.tabs.len() {
                if let Some(d) = state.tabs.get_mut(i) {
                    d.read_only = true;
                }
            }
            state.status = "Read-only: all documents".into();
            CmdResult::Handled
        }
        "IDM_EDIT_CLEARREADONLYFORALLDOCS" => {
            for i in 0..state.tabs.len() {
                if let Some(d) = state.tabs.get_mut(i) {
                    d.read_only = false;
                }
            }
            state.status = "Read-only: cleared for all".into();
            CmdResult::Handled
        }
        "IDM_EDIT_TRIMTRAILING" => {
            state.run_plugin("edit.trim_trailing");
            CmdResult::Handled
        }
        "IDM_EDIT_TRIMLINEHEAD" => {
            let text = state.tabs.active().buffer.to_string();
            let out = map_line_bodies(&text, |body| body.trim_start().to_string());
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Trim leading space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_TRIM_BOTH" => {
            let text = state.tabs.active().buffer.to_string();
            let out = map_line_bodies(&text, |body| body.trim().to_string());
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Trim leading and trailing space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_EOL2WS" => {
            let text = state.tabs.active().buffer.to_string();
            let out = text.replace("\r\n", " ").replace(['\n', '\r'], " ");
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "EOL to space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_TRIMALL" => {
            let text = state.tabs.active().buffer.to_string();
            let trimmed = map_line_bodies(&text, |body| body.trim().to_string());
            let out = trimmed.replace("\r\n", " ").replace(['\n', '\r'], " ");
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Trim both and EOL to space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_TAB2SW" => {
            let text = state.tabs.active().buffer.to_string().replace('\t', "    ");
            state.tabs.active_mut().buffer.replace_document(&text);
            state.mark_text_changed();
            state.status = "TAB to space".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SW2TAB_ALL" => {
            let text = state.tabs.active().buffer.to_string().replace("    ", "\t");
            state.tabs.active_mut().buffer.replace_document(&text);
            state.mark_text_changed();
            state.status = "Space to TAB (all)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_SW2TAB_LEADING" => {
            let text = state.tabs.active().buffer.to_string();
            let out = map_line_bodies(&text, spaces_to_tabs_leading);
            state.tabs.active_mut().buffer.replace_document(&out);
            state.mark_text_changed();
            state.status = "Space to TAB (leading)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_RTL" => {
            TEXT_RTL.store(true, Ordering::Relaxed);
            state.status = "Text direction: RTL (layout + status cue)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_LTR" => {
            TEXT_RTL.store(false, Ordering::Relaxed);
            state.status = "Text direction: LTR".into();
            CmdResult::Handled
        }
        "IDM_EDIT_PASTE_AS_HTML" | "IDM_EDIT_PASTE_AS_RTF" | "IDM_EDIT_PASTE_BINARY" => {
            paste_plain_fallback(state, ui, cmd);
            CmdResult::Handled
        }
        "IDM_EDIT_COPY_BINARY" => {
            copy_selection(state, ui);
            if ui.pending_clipboard.is_some() {
                state.status = "Binary copy (plain text to clipboard)".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_CUT_BINARY" => {
            if state.tabs.active().read_only {
                state.status = "Document is read-only".into();
                return Some(CmdResult::Handled);
            }
            if let Some((s, e)) = state.tabs.active().buffer.selection() {
                let text = state.tabs.active().buffer.slice(s, e);
                ui.pending_clipboard = Some(text);
                state.tabs.active_mut().buffer.delete_backward();
                state.mark_text_changed();
                state.status = "Binary cut (plain text to clipboard)".into();
            } else {
                state.status = "Binary cut: select text first".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTALL" => {
            multi_select_all(state, ui, false, false);
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTALLMATCHCASE" => {
            multi_select_all(state, ui, true, false);
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTALLWHOLEWORD" => {
            multi_select_all(state, ui, false, true);
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTALLMATCHCASEWHOLEWORD" => {
            multi_select_all(state, ui, true, true);
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTNEXT" => {
            multi_select_next(state, ui, false, false, false);
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTNEXTMATCHCASE" => {
            multi_select_next(state, ui, true, false, false);
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTNEXTWHOLEWORD" => {
            multi_select_next(state, ui, false, true, false);
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTNEXTMATCHCASEWHOLEWORD" => {
            multi_select_next(state, ui, true, true, false);
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTUNDO" => {
            let doc = state.tabs.active_mut();
            if let Some((s, e)) = doc.multi_sels.pop() {
                if let Some(&(ps, pe)) = doc.multi_sels.last() {
                    doc.buffer.set_selection(ps, pe);
                } else {
                    doc.buffer.set_selection(s, e);
                }
                let n = doc.multi_sels.len();
                state.status = format!("Multi-select undo: {n} left");
                ui.follow_caret = true;
            } else {
                state.status = "Multi-select undo: nothing to undo".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_MULTISELECTSSKIP" => {
            multi_select_next(state, ui, false, false, true);
            CmdResult::Handled
        }
        "IDM_EDIT_COLUMNMODETIP" => {
            state.status = "Column tip: select lines (or multi-carets), copy one-line text, then                 Column Editor inserts it at the caret column. Empty clipboard inserts 0,1,2…"
                .into();
            CmdResult::Handled
        }
        "IDM_EDIT_COLUMNMODE" => {
            column_editor_insert(state, ui);
            CmdResult::Handled
        }
        "IDM_EDIT_CHAR_PANEL" => {
            ui.show_char_panel = true;
            state.status = "Character Panel".into();
            CmdResult::Handled
        }
        "IDM_EDIT_CLIPBOARDHISTORY_PANEL" => {
            if let Some(t) = ui.last_copied.as_ref() {
                let preview: String = t.chars().take(40).collect();
                let more = if t.chars().count() > 40 { "…" } else { "" };
                state.status = format!(
                    "Clipboard history (1 entry): \"{preview}{more}\" ({} chars)",
                    t.chars().count()
                );
            } else {
                state.status = "Clipboard history: empty (copy text first)".into();
            }
            CmdResult::Handled
        }
        "IDM_EDIT_TOGGLESYSTEMREADONLY" => {
            toggle_system_readonly(state);
            CmdResult::Handled
        }
        "IDM_EDIT_AUTOCOMPLETE" | "IDM_EDIT_AUTOCOMPLETE_CURRENTFILE" => {
            word_complete(state);
            CmdResult::Handled
        }
        "IDM_EDIT_AUTOCOMPLETE_PATH" => {
            path_complete(state);
            CmdResult::Handled
        }
        "IDM_EDIT_FUNCCALLTIP" => {
            func_call_tip(state, 0);
            CmdResult::Handled
        }
        "IDM_EDIT_FUNCCALLTIP_PREVIOUS" => {
            func_call_tip(state, -1);
            CmdResult::Handled
        }
        "IDM_EDIT_FUNCCALLTIP_NEXT" => {
            func_call_tip(state, 1);
            CmdResult::Handled
        }

        _ => CmdResult::Stub,
    })
}

fn paste_plain_fallback(state: &mut EditorState, ui: &mut UiFlags, cmd: &str) {
    if state.tabs.active().read_only {
        state.status = "Document is read-only".into();
        return;
    }
    let Some(text) = ui.last_copied.clone() else {
        state.status = "Paste special: copy text first (plain-text fallback)".into();
        return;
    };
    state.tabs.active_mut().buffer.insert(&text);
    state.mark_text_changed();
    let kind = if cmd.contains("HTML") {
        "HTML"
    } else if cmd.contains("RTF") {
        "RTF"
    } else {
        "binary"
    };
    state.status = format!("Pasted as plain text ({kind} fallback)");
}

fn multi_query(state: &EditorState) -> Option<String> {
    if let Some((s, e)) = state.tabs.active().buffer.selection() {
        let t = state.tabs.active().buffer.slice(s, e);
        if !t.is_empty() && !t.contains('\n') {
            return Some(t);
        }
    }
    if let Some(&(s, e)) = state.tabs.active().multi_sels.first() {
        let t = state.tabs.active().buffer.slice(s, e);
        if !t.is_empty() {
            return Some(t);
        }
    }
    let caret = state.tabs.active().buffer.caret();
    let (ws, we) = state.tabs.active().buffer.word_bounds_at(caret);
    if ws < we {
        Some(state.tabs.active().buffer.slice(ws, we))
    } else {
        None
    }
}

fn find_all_matches(
    text: &str,
    query: &str,
    match_case: bool,
    whole_word: bool,
) -> Vec<(usize, usize)> {
    crate::search_util::find_all_matches(text, query, match_case, whole_word)
}

fn is_word_char(c: char) -> bool {
    crate::search_util::is_word_char(c)
}

fn multi_select_all(state: &mut EditorState, ui: &mut UiFlags, match_case: bool, whole_word: bool) {
    let Some(query) = multi_query(state) else {
        state.status = "Multi-select all: select a word first".into();
        return;
    };
    let text = state.tabs.active().buffer.to_string();
    let matches = find_all_matches(&text, &query, match_case, whole_word);
    if matches.is_empty() {
        state.status = "Multi-select all: no matches".into();
        return;
    }
    let n = matches.len();
    let first = matches[0];
    {
        let doc = state.tabs.active_mut();
        doc.multi_sels = matches;
        doc.buffer.set_selection(first.0, first.1);
    }
    ui.follow_caret = true;
    state.status = format!("Multi-select all: {n} matches");
}

fn multi_select_next(
    state: &mut EditorState,
    ui: &mut UiFlags,
    match_case: bool,
    whole_word: bool,
    skip: bool,
) {
    let Some(query) = multi_query(state) else {
        state.status = "Multi-select next: select a word first".into();
        return;
    };
    if skip {
        let _ = state.tabs.active_mut().multi_sels.pop();
    }
    let text = state.tabs.active().buffer.to_string();
    let all = find_all_matches(&text, &query, match_case, whole_word);
    if all.is_empty() {
        state.status = "Multi-select next: no matches".into();
        return;
    }
    let from = state
        .tabs
        .active()
        .buffer
        .selection()
        .map(|(_, e)| e)
        .unwrap_or_else(|| state.tabs.active().buffer.caret());
    let next = all
        .iter()
        .copied()
        .find(|(s, _)| *s >= from)
        .or_else(|| all.first().copied());
    let Some((s, e)) = next else {
        state.status = "Multi-select next: no further match".into();
        return;
    };
    {
        let doc = state.tabs.active_mut();
        if !doc.multi_sels.contains(&(s, e)) {
            doc.multi_sels.push((s, e));
        }
        doc.buffer.set_selection(s, e);
    }
    ui.follow_caret = true;
    let n = state.tabs.active().multi_sels.len();
    let verb = if skip { "skip" } else { "next" };
    state.status = format!("Multi-select {verb}: {n} ranges");
}

/// Insert clipboard text (or 0,1,2…) at the caret column on each selected line,
/// or at each multi-select start.
fn column_editor_insert(state: &mut EditorState, ui: &mut UiFlags) {
    if state.tabs.active().read_only {
        state.status = "Document is read-only".into();
        return;
    }
    let clip = ui
        .last_copied
        .as_ref()
        .filter(|s| !s.is_empty() && !s.contains('\n'))
        .cloned();
    let multi = state.tabs.active().multi_sels.clone();

    if multi.len() >= 2 {
        let mut starts: Vec<usize> = multi.iter().map(|&(s, _)| s).collect();
        starts.sort_unstable();
        starts.dedup();
        let n = starts.len();
        let pieces: Vec<(usize, String)> = starts
            .iter()
            .enumerate()
            .rev()
            .map(|(ord, &pos)| {
                let piece = match &clip {
                    Some(t) => t.clone(),
                    None => format!("{ord}"),
                };
                (pos, piece)
            })
            .collect();
        state.tabs.active_mut().buffer.with_transaction(|buf| {
            for (pos, piece) in pieces {
                buf.set_caret(pos);
                buf.insert(&piece);
            }
        });
        state.mark_text_changed();
        ui.follow_caret = true;
        state.status = if clip.is_some() {
            format!("Column editor: inserted clipboard text at {n} carets")
        } else {
            format!("Column editor: inserted numbers at {n} carets")
        };
        return;
    }

    let (start_line, end_line, col) = {
        let buf = &state.tabs.active().buffer;
        let (start_line, end_line) = buf.selected_line_range();
        let col = if let Some((s, _)) = buf.selection() {
            s.saturating_sub(buf.line_to_char(buf.char_to_line(s)))
        } else {
            let c = buf.caret();
            c.saturating_sub(buf.line_to_char(buf.char_to_line(c)))
        };
        (start_line, end_line, col)
    };

    let line_ops: Vec<(usize, String)> = (start_line..=end_line)
        .rev()
        .map(|line| {
            let piece = match &clip {
                Some(t) => t.clone(),
                None => format!("{}", line - start_line),
            };
            (line, piece)
        })
        .collect();
    state.tabs.active_mut().buffer.with_transaction(|buf| {
        for (line, piece) in line_ops {
            let line_start = buf.line_to_char(line);
            let raw = buf.line(line);
            let body_len = raw.trim_end_matches(['\n', '\r']).chars().count();
            if col > body_len {
                let pad = col - body_len;
                buf.set_caret(line_start + body_len);
                let mut s = " ".repeat(pad);
                s.push_str(&piece);
                buf.insert(&s);
            } else {
                buf.set_caret(line_start + col);
                buf.insert(&piece);
            }
        }
    });
    state.mark_text_changed();
    ui.follow_caret = true;
    let n_lines = end_line - start_line + 1;
    state.status = if clip.is_some() {
        format!("Column editor: inserted clipboard text on {n_lines} lines (col {col})")
    } else {
        format!("Column editor: inserted numbers on {n_lines} lines (col {col})")
    };
}

thread_local! {
    static CALL_TIP_WORD: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
    static CALL_TIP_IDX: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Function call tip from word under caret + nearby `fn`/`def` lines (no LSP).
fn func_call_tip(state: &mut EditorState, delta: i8) {
    let word = call_tip_word(state);
    if word.is_empty() {
        state.status = "Call tip: place caret on a function name".into();
        return;
    }
    let hints = call_tip_hints(state, &word);
    if hints.is_empty() {
        state.status = format!("Call tip: {word}(…) — no def or call in this file");
        return;
    }
    let mut idx = CALL_TIP_WORD.with(|w| {
        let mut stored = w.borrow_mut();
        if *stored != word {
            *stored = word.clone();
            CALL_TIP_IDX.set(0);
        }
        CALL_TIP_IDX.get()
    });
    let n = hints.len();
    if delta > 0 {
        idx = (idx + 1) % n;
    } else if delta < 0 {
        idx = if idx == 0 { n - 1 } else { idx - 1 };
    }
    CALL_TIP_IDX.set(idx);
    state.status = format!("Call tip [{}/{n}]: {}", idx + 1, hints[idx]);
}

fn call_tip_word(state: &EditorState) -> String {
    let buf = &state.tabs.active().buffer;
    let caret = buf.caret();
    let text = buf.to_string();
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    let len = chars.len();
    let caret = caret.min(len);

    // Prefer the callee name before '(' when the caret is inside call arguments.
    let mut depth = 0i32;
    let mut i = caret;
    while i > 0 {
        let c = chars[i - 1];
        if c == ')' {
            depth += 1;
        } else if c == '(' {
            if depth == 0 {
                let before = i.saturating_sub(1);
                if let Some(name) = ident_ending_at(&chars, before) {
                    return name;
                }
                break;
            }
            depth -= 1;
        } else if depth == 0 && matches!(c, ';' | '{' | '}' | '\n' | '\r') {
            break;
        }
        i -= 1;
    }

    // Word under caret, or the identifier just left of the caret / of `.` / `::`.
    let mut probe = if caret < len && is_ident_char(chars[caret]) {
        caret
    } else {
        caret.saturating_sub(1)
    };
    while probe > 0 && matches!(chars[probe], '.' | ':') {
        probe = probe.saturating_sub(1);
    }
    if probe < len && is_ident_char(chars[probe]) {
        let mut end = probe + 1;
        while end < len && is_ident_char(chars[end]) {
            end += 1;
        }
        if let Some(name) = ident_ending_at(&chars, end) {
            return name;
        }
    }
    String::new()
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Identifier that ends at `end` (exclusive char index into `chars`).
fn ident_ending_at(chars: &[char], end: usize) -> Option<String> {
    if end == 0 || end > chars.len() {
        return None;
    }
    let mut i = end;
    while i > 0 && is_ident_char(chars[i - 1]) {
        i -= 1;
    }
    if i >= end {
        return None;
    }
    // Skip leading digits-only tokens.
    let name: String = chars[i..end].iter().collect();
    if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(name)
}

fn trim_tip(s: &str, max: usize) -> String {
    let t = s.trim();
    if t.chars().count() > max {
        format!(
            "{}…",
            t.chars().take(max.saturating_sub(1)).collect::<String>()
        )
    } else {
        t.to_string()
    }
}

fn call_tip_hints(state: &EditorState, word: &str) -> Vec<String> {
    let text = state.tabs.active().buffer.to_string();
    let caret_line = {
        let buf = &state.tabs.active().buffer;
        buf.char_to_line(buf.caret())
    };
    let mut defs = Vec::new();
    let mut calls = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (line_no, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if let Some(snip) = def_line_snippet(trimmed, word) {
            let tip = format!("L{} def {}", line_no + 1, trim_tip(&snip, 64));
            if seen.insert(tip.clone()) {
                let dist = line_no.abs_diff(caret_line);
                defs.push((dist, tip));
            }
            continue;
        }
        if let Some(at) = find_call_site(line, word) {
            let after = &line[at + word.len()..];
            let close = after
                .find(')')
                .map(|i| i + 1)
                .unwrap_or(after.chars().count().min(48));
            let snippet: String = after.chars().take(close.max(1)).collect();
            let tip = format!("L{} {}{}", line_no + 1, word, trim_tip(&snippet, 56));
            if seen.insert(tip.clone()) {
                let dist = line_no.abs_diff(caret_line);
                calls.push((dist, tip));
            }
        }
    }

    // Prefer defs near the caret, then nearby call sites.
    defs.sort_by_key(|(d, _)| *d);
    calls.sort_by_key(|(d, _)| *d);
    let mut out: Vec<String> = defs.into_iter().map(|(_, t)| t).collect();
    for (_, t) in calls {
        if out.len() >= 24 {
            break;
        }
        out.push(t);
    }
    if out.is_empty() {
        out.push(format!("{word}(…)"));
    }
    out
}

/// Match `fn name`, `def name`, `function name`, `func name` (optional `(` / `<`).
fn def_line_snippet(trimmed: &str, word: &str) -> Option<String> {
    const KEYWORDS: &[&str] = &["fn", "def", "function", "func", "sub", "proc"];
    for kw in KEYWORDS {
        let rest = if let Some(r) = trimmed.strip_prefix(kw) {
            r
        } else {
            continue;
        };
        if rest.is_empty() || is_ident_char(rest.chars().next()?) {
            continue;
        }
        let after_kw = rest.trim_start();
        // Rust: `fn name<…>` / `pub async fn name(`
        let name_start = after_kw;
        // Allow `pub ` / `async ` / `static ` already stripped if keyword is mid-line.
        if !name_start.starts_with(word) {
            continue;
        }
        let after_name = &name_start[word.len()..];
        let ok_boundary = after_name
            .chars()
            .next()
            .map(|c| matches!(c, '(' | '<' | ' ' | '\t' | '{' | ':') || !is_ident_char(c))
            .unwrap_or(true);
        if !ok_boundary {
            continue;
        }
        return Some(trimmed.chars().take(72).collect());
    }
    // Also: `pub fn word` / `async fn word` / `pub async fn word`
    for prefix in ["pub ", "async ", "static ", "export "] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            if let Some(s) = def_line_snippet(rest.trim_start(), word) {
                return Some(s);
            }
        }
    }
    // Mid-line: look for " fn word" / " def word"
    for kw in KEYWORDS {
        let pat = format!(" {kw} ");
        if let Some(at) = trimmed.find(&pat) {
            let after = trimmed[at + pat.len()..].trim_start();
            if let Some(after_name) = after.strip_prefix(word) {
                let ok = after_name
                    .chars()
                    .next()
                    .map(|c| {
                        matches!(c, '(' | '<' | '{' | ':') || c.is_whitespace() || !is_ident_char(c)
                    })
                    .unwrap_or(true);
                if ok {
                    return Some(trimmed.chars().take(72).collect());
                }
            }
        }
    }
    None
}

fn find_call_site(line: &str, word: &str) -> Option<usize> {
    let needle = format!("{word}(");
    let mut from = 0;
    while let Some(rel) = line[from..].find(&needle) {
        let at = from + rel;
        let ok_before = at == 0 || {
            let c = line[..at].chars().next_back().unwrap_or('\0');
            !is_ident_char(c)
        };
        if ok_before {
            // Skip obvious defs already handled.
            let before = line[..at].trim_end();
            let is_def = ["fn", "def", "function", "func", "sub", "proc"]
                .iter()
                .any(|kw| before.ends_with(kw));
            if !is_def {
                return Some(at);
            }
        }
        from = at + word.len();
    }
    None
}

fn toggle_system_readonly(state: &mut EditorState) {
    let Some(path) = state.tabs.active().path.clone() else {
        state.status = "System read-only: save the file first".into();
        return;
    };
    match std::fs::metadata(&path) {
        Ok(meta) => {
            let mut perms = meta.permissions();
            let next = !perms.readonly();
            perms.set_readonly(next);
            match std::fs::set_permissions(&path, perms) {
                Ok(()) => {
                    state.tabs.active_mut().read_only = next;
                    state.status = if next {
                        "System read-only: on (file + app)".into()
                    } else {
                        "System read-only: off".into()
                    };
                }
                Err(_) => state.status = "System read-only: permission change failed".into(),
            }
        }
        Err(_) => state.status = "System read-only: cannot read file metadata".into(),
    }
}

fn word_complete(state: &mut EditorState) {
    if state.tabs.active().read_only {
        state.status = "Document is read-only".into();
        return;
    }
    let buf = &state.tabs.active().buffer;
    let caret = buf.caret();
    let (ws, we) = buf.word_bounds_at(caret.saturating_sub(1.min(caret)));
    if ws >= we || caret < ws || caret > we {
        state.status = "Word completion: type a prefix first".into();
        return;
    }
    let prefix = buf.slice(ws, caret);
    if prefix.is_empty() {
        state.status = "Word completion: type a prefix first".into();
        return;
    }
    let text = buf.to_string();
    let mut cands = std::collections::BTreeSet::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        if is_word_char(chars[i]) {
            let start = i;
            while i < chars.len() && is_word_char(chars[i]) {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let ok = if prefix.is_ascii() {
                word.len() > prefix.len()
                    && word
                        .to_ascii_lowercase()
                        .starts_with(&prefix.to_ascii_lowercase())
            } else {
                word.len() > prefix.len() && word.starts_with(&prefix)
            };
            if ok {
                cands.insert(word);
            }
        } else {
            i += 1;
        }
    }
    if cands.is_empty() {
        state.status = format!("Word completion: no match for '{prefix}'");
        return;
    }
    if cands.len() == 1 {
        let word = cands.iter().next().unwrap().clone();
        let rest: String = word.chars().skip(prefix.chars().count()).collect();
        state.tabs.active_mut().buffer.insert(&rest);
        state.mark_text_changed();
        state.status = format!("Word completion: {word}");
    } else {
        let preview: Vec<_> = cands.iter().take(5).cloned().collect();
        state.status = format!(
            "Word completion: {} matches — {}",
            cands.len(),
            preview.join(", ")
        );
    }
}

fn path_complete(state: &mut EditorState) {
    if state.tabs.active().read_only {
        state.status = "Document is read-only".into();
        return;
    }
    let buf = &state.tabs.active().buffer;
    let caret = buf.caret();
    let text = buf.to_string();
    let chars: Vec<char> = text.chars().collect();
    if caret == 0 || caret > chars.len() {
        state.status = "Path completion: type a path prefix first".into();
        return;
    }
    let mut start = caret;
    while start > 0 {
        let c = chars[start - 1];
        if c.is_whitespace() || matches!(c, '"' | '\'' | '<' | '>' | '|' | '\n' | '\r') {
            break;
        }
        start -= 1;
    }
    let partial: String = chars[start..caret].iter().collect();
    if partial.is_empty() {
        state.status = "Path completion: type a path prefix first".into();
        return;
    }
    let path = std::path::Path::new(&partial);
    let (dir, file_prefix) = if partial.ends_with('/') || partial.ends_with('\\') {
        (path.to_path_buf(), String::new())
    } else {
        (
            path.parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::path::PathBuf::from(".")),
            path.file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default(),
        )
    };
    let base = if dir.as_os_str().is_empty() {
        std::path::PathBuf::from(".")
    } else {
        dir
    };
    let Ok(rd) = std::fs::read_dir(&base) else {
        state.status = "Path completion: directory not found".into();
        return;
    };
    let mut names: Vec<String> = rd
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if file_prefix.is_empty() || name.starts_with(&file_prefix) {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    names.sort();
    names.dedup();
    if names.is_empty() {
        state.status = "Path completion: no match".into();
        return;
    }
    if names.len() == 1 {
        let name = &names[0];
        let rest: String = name.chars().skip(file_prefix.chars().count()).collect();
        state.tabs.active_mut().buffer.insert(&rest);
        state.mark_text_changed();
        state.status = format!("Path completion: {name}");
    } else {
        let preview: Vec<_> = names.iter().take(5).cloned().collect();
        state.status = format!(
            "Path completion: {} matches — {}",
            names.len(),
            preview.join(", ")
        );
    }
}
