//! Edit menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;
use std::path::Path;

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_EDIT_")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
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
            cut_selection(state);
            CmdResult::Handled
        }
        "IDM_EDIT_COPY" => {
            copy_selection(state, ui);
            CmdResult::Handled
        }
        "IDM_EDIT_PASTE" => {
            state.status = "Paste: use ⌘/Ctrl+V in the editor".into();
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
            state.tabs.active_mut().buffer.indent_lines("    ");
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
            let line = state.tabs.active().buffer.char_to_line(state.tabs.active().buffer.caret());
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
                    lines.sort_by(|a, b| b.chars().count().cmp(&a.chars().count()));
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
                        seed = seed
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1);
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
                    path.parent()
                        .map(Path::to_path_buf)
                        .unwrap_or(path)
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
            state.status = "Text direction RTL noted (layout stays LTR for now)".into();
            CmdResult::Handled
        }
        "IDM_EDIT_LTR" => {
            state.status = "Text direction LTR".into();
            CmdResult::Handled
        }

        _ => CmdResult::Stub,
    })
}
