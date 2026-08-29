//! Rope-backed text buffer with caret and undo/redo.
//!
//! App-level read-only / loading gates live on `doc::Document::try_buffer_mut`
//! (and command helpers). Prefer those over calling mutate methods directly from menus.

use ropey::Rope;
use std::collections::VecDeque;

const MAX_UNDO: usize = 256;

#[derive(Debug, Clone)]
enum Edit {
    Insert { index: usize, text: String },
    Delete { index: usize, text: String },
    Replace { old: String, new: String },
}

/// Line-count change from the last mutating edit (for bookmark / mark remap).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineStructureEdit {
    /// `n` lines inserted; marks at or after `at` shift down by `n`.
    Insert { at: usize, n: usize },
    /// `n` lines removed starting at `first`; marks in range drop; later marks shift up.
    Delete { first: usize, n: usize },
}

/// Text buffer with caret, selection, and undo history.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
    caret: usize,
    /// Selection anchor; `None` means no selection (caret only).
    sel_anchor: Option<usize>,
    undo: VecDeque<Edit>,
    redo: VecDeque<Edit>,
    /// Coalesce consecutive typing into one undo step.
    last_insert_end: Option<usize>,
    /// Net line-structure change from the latest mutation (taken by Document remap).
    last_line_edit: Option<LineStructureEdit>,
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            caret: 0,
            sel_anchor: None,
            undo: VecDeque::new(),
            redo: VecDeque::new(),
            last_insert_end: None,
            last_line_edit: None,
        }
    }

    /// Take the line-structure edit from the last mutation, if any.
    pub fn take_line_structure_edit(&mut self) -> Option<LineStructureEdit> {
        self.last_line_edit.take()
    }

    fn begin_line_edit(&mut self) {
        self.last_line_edit = None;
    }

    fn merge_line_edit(&mut self, edit: LineStructureEdit) {
        self.last_line_edit = match (self.last_line_edit, edit) {
            (
                Some(LineStructureEdit::Delete { first, n: dn }),
                LineStructureEdit::Insert { at, n: i },
            ) => {
                // Replace-selection: delete then insert in one public op.
                let net = i as isize - dn as isize;
                if net > 0 {
                    Some(LineStructureEdit::Insert {
                        at: first.min(at),
                        n: net as usize,
                    })
                } else if net < 0 {
                    Some(LineStructureEdit::Delete {
                        first,
                        n: (-net) as usize,
                    })
                } else {
                    None
                }
            }
            (_, e) => Some(e),
        };
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let mut buf = Self::new();
        buf.rope = Rope::from_str(s);
        buf.caret = 0;
        buf
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    pub fn caret(&self) -> usize {
        self.caret
    }

    pub fn set_caret(&mut self, index: usize) {
        self.caret = index.min(self.len_chars());
        self.sel_anchor = None;
        self.last_insert_end = None;
    }

    pub fn selection(&self) -> Option<(usize, usize)> {
        let a = self.sel_anchor?;
        let (start, end) = if a <= self.caret {
            (a, self.caret)
        } else {
            (self.caret, a)
        };
        if start == end {
            None
        } else {
            Some((start, end))
        }
    }

    pub fn set_selection(&mut self, anchor: usize, caret: usize) {
        let len = self.len_chars();
        self.sel_anchor = Some(anchor.min(len));
        self.caret = caret.min(len);
        self.last_insert_end = None;
    }

    pub fn clear_selection(&mut self) {
        self.sel_anchor = None;
    }

    pub fn select_all(&mut self) {
        let len = self.len_chars();
        if len == 0 {
            self.sel_anchor = None;
            self.caret = 0;
            return;
        }
        self.set_selection(0, len);
    }

    /// Word under caret (or at index): [start, end) char indices.
    pub fn word_bounds_at(&self, index: usize) -> (usize, usize) {
        let len = self.len_chars();
        if len == 0 {
            return (0, 0);
        }
        let index = index.min(len);
        let text = self.to_string();
        let chars: Vec<char> = text.chars().collect();
        let i = if index >= chars.len() {
            chars.len().saturating_sub(1)
        } else {
            index
        };
        if !is_word_char(chars[i]) {
            // Click on punctuation/space: select single char if not space, else empty.
            if chars[i].is_whitespace() {
                return (index, index);
            }
            return (i, i + 1);
        }
        let mut start = i;
        while start > 0 && is_word_char(chars[start - 1]) {
            start -= 1;
        }
        let mut end = i + 1;
        while end < chars.len() && is_word_char(chars[end]) {
            end += 1;
        }
        (start, end)
    }

    pub fn select_word_at(&mut self, index: usize) {
        let (s, e) = self.word_bounds_at(index);
        if s < e {
            self.set_selection(s, e);
        } else {
            self.set_caret(index);
        }
    }

    pub fn select_line_at(&mut self, index: usize) {
        let line = self.char_to_line(index);
        let start = self.line_to_char(line);
        let end = if line + 1 < self.line_count() {
            self.line_to_char(line + 1)
        } else {
            self.len_chars()
        };
        self.set_selection(start, end);
    }

    pub fn move_word(&mut self, forward: bool, select: bool) {
        let len = self.len_chars();
        let text = self.to_string();
        let chars: Vec<char> = text.chars().collect();
        let mut i = self.caret;
        if forward {
            if i >= len {
                return;
            }
            // Skip current word chars, then whitespace.
            if i < chars.len() && is_word_char(chars[i]) {
                while i < chars.len() && is_word_char(chars[i]) {
                    i += 1;
                }
            }
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
        } else {
            if i == 0 {
                return;
            }
            i = i.saturating_sub(1);
            while i > 0 && chars[i].is_whitespace() {
                i -= 1;
            }
            if is_word_char(chars[i]) {
                while i > 0 && is_word_char(chars[i - 1]) {
                    i -= 1;
                }
            }
        }
        if select {
            let anchor = self.sel_anchor.unwrap_or(self.caret);
            self.set_selection(anchor, i);
        } else {
            self.set_caret(i);
        }
    }

    /// Indent selected lines (or current line) with `prefix` (e.g. `"    "` or `"\t"`).
    pub fn indent_lines(&mut self, prefix: &str) {
        let (start_line, end_line) = self.selected_line_range();
        let mut offset_add = 0usize;
        let caret_line = self.char_to_line(self.caret);
        for line in (start_line..=end_line).rev() {
            let at = self.line_to_char(line);
            self.rope.insert(at, prefix);
            self.push_undo(Edit::Insert {
                index: at,
                text: prefix.to_string(),
            });
            offset_add += prefix.chars().count();
            let _ = caret_line;
        }
        self.redo.clear();
        self.last_insert_end = None;
        // Expand selection to cover indented block.
        let new_start = self.line_to_char(start_line);
        let new_end = if end_line + 1 < self.line_count() {
            self.line_to_char(end_line + 1)
        } else {
            self.len_chars()
        };
        self.set_selection(new_start, new_end);
        let _ = offset_add;
    }

    pub fn outdent_lines(&mut self, width: usize) {
        let (start_line, end_line) = self.selected_line_range();
        for line in start_line..=end_line {
            let raw = self.line(line);
            let trim = if raw.starts_with('\t') {
                1
            } else {
                raw.chars().take(width).take_while(|c| *c == ' ').count()
            };
            if trim == 0 {
                continue;
            }
            let at = self.line_to_char(line);
            let removed: String = raw.chars().take(trim).collect();
            self.rope.remove(at..at + trim);
            self.push_undo(Edit::Delete {
                index: at,
                text: removed,
            });
        }
        self.redo.clear();
        self.last_insert_end = None;
        let new_start = self.line_to_char(start_line);
        let new_end = if end_line + 1 < self.line_count() {
            self.line_to_char(end_line + 1)
        } else {
            self.len_chars()
        };
        if start_line != end_line || self.selection().is_some() {
            self.set_selection(new_start, new_end);
        }
    }

    pub fn selected_line_range(&self) -> (usize, usize) {
        if let Some((s, e)) = self.selection() {
            let start_line = self.char_to_line(s);
            let end_idx = e.saturating_sub(1).max(s);
            let end_line = self.char_to_line(end_idx);
            (start_line, end_line)
        } else {
            let line = self.char_to_line(self.caret);
            (line, line)
        }
    }

    /// Prefix selected lines with a line-comment marker (e.g. `"// "`).
    pub fn comment_lines(&mut self, prefix: &str) {
        if prefix.is_empty() {
            return;
        }
        let (start_line, end_line) = self.selected_line_range();
        for line in (start_line..=end_line).rev() {
            let raw = self.line(line);
            let body = raw.trim_end_matches(['\n', '\r']);
            if body.trim().is_empty() {
                continue;
            }
            let trimmed = body.trim_start();
            let lead = body.chars().count() - trimmed.chars().count();
            if trimmed.starts_with(prefix.trim_end()) {
                continue;
            }
            let at = self.line_to_char(line) + lead;
            self.rope.insert(at, prefix);
            self.push_undo(Edit::Insert {
                index: at,
                text: prefix.to_string(),
            });
        }
        self.redo.clear();
        self.last_insert_end = None;
        self.reselect_lines(start_line, end_line);
    }

    /// Strip a line-comment marker from selected lines when present.
    pub fn uncomment_lines(&mut self, prefix: &str) {
        if prefix.is_empty() {
            return;
        }
        let bare = prefix.trim_end();
        let (start_line, end_line) = self.selected_line_range();
        for line in start_line..=end_line {
            let raw = self.line(line);
            let body = raw.trim_end_matches(['\n', '\r']);
            let trimmed = body.trim_start();
            let lead = body.chars().count() - trimmed.chars().count();
            let remove = if trimmed.starts_with(prefix) {
                prefix.chars().count()
            } else if let Some(after) = trimmed.strip_prefix(bare) {
                if after.starts_with(' ') {
                    bare.chars().count() + 1
                } else {
                    bare.chars().count()
                }
            } else {
                0
            };
            if remove == 0 {
                continue;
            }
            let at = self.line_to_char(line) + lead;
            let removed: String = trimmed.chars().take(remove).collect();
            self.rope.remove(at..at + remove);
            self.push_undo(Edit::Delete {
                index: at,
                text: removed,
            });
        }
        self.redo.clear();
        self.last_insert_end = None;
        self.reselect_lines(start_line, end_line);
    }

    /// Toggle line comments: uncomment if every non-empty line is commented.
    pub fn toggle_line_comments(&mut self, prefix: &str) {
        if prefix.is_empty() {
            return;
        }
        let bare = prefix.trim_end();
        let (start_line, end_line) = self.selected_line_range();
        let mut any = false;
        let mut all_commented = true;
        for line in start_line..=end_line {
            let body = self.line(line);
            let trimmed = body.trim_end_matches(['\n', '\r']).trim_start();
            if trimmed.is_empty() {
                continue;
            }
            any = true;
            if !(trimmed.starts_with(prefix) || trimmed.starts_with(bare)) {
                all_commented = false;
                break;
            }
        }
        if any && all_commented {
            self.uncomment_lines(prefix);
        } else {
            self.comment_lines(prefix);
        }
    }

    /// Wrap the selection (or insert at caret) with block comment markers.
    pub fn stream_comment(&mut self, open: &str, close: &str) {
        if open.is_empty() && close.is_empty() {
            return;
        }
        if let Some((s, e)) = self.selection() {
            let mid = self.slice(s, e);
            let wrapped = format!("{open}{mid}{close}");
            self.set_selection(s, e);
            self.insert(&wrapped);
            self.set_selection(s, s + wrapped.chars().count());
        } else {
            let at = self.caret;
            let wrapped = format!("{open}{close}");
            self.insert(&wrapped);
            self.set_caret(at + open.chars().count());
        }
    }

    /// Unwrap block comment markers around the selection when present.
    pub fn stream_uncomment(&mut self, open: &str, close: &str) {
        let Some((s, e)) = self.selection() else {
            return;
        };
        let mid = self.slice(s, e);
        let Some(stripped) = mid
            .strip_prefix(open)
            .and_then(|r| r.strip_suffix(close))
            .map(|s| s.to_string())
        else {
            return;
        };
        self.set_selection(s, e);
        if stripped.is_empty() {
            self.delete_backward();
        } else {
            self.insert(&stripped);
            self.set_selection(s, s + stripped.chars().count());
        }
    }

    fn reselect_lines(&mut self, start_line: usize, end_line: usize) {
        let new_start = self.line_to_char(start_line);
        let new_end = if end_line + 1 < self.line_count() {
            self.line_to_char(end_line + 1)
        } else {
            self.len_chars()
        };
        if start_line != end_line || self.selection().is_some() {
            self.set_selection(new_start, new_end);
        }
    }

    pub fn duplicate_line(&mut self) {
        let line = self.char_to_line(self.caret);
        let start = self.line_to_char(line);
        let end = if line + 1 < self.line_count() {
            self.line_to_char(line + 1)
        } else {
            self.len_chars()
        };
        let mut chunk = self.slice(start, end);
        if !chunk.ends_with('\n') {
            chunk.push('\n');
        }
        self.rope.insert(end, &chunk);
        self.push_undo(Edit::Insert {
            index: end,
            text: chunk.clone(),
        });
        self.redo.clear();
        self.last_insert_end = None;
        self.set_caret(end + chunk.chars().count().saturating_sub(1));
    }

    pub fn delete_line(&mut self) {
        self.begin_line_edit();
        let line = self.char_to_line(self.caret);
        let start = self.line_to_char(line);
        let end = if line + 1 < self.line_count() {
            self.line_to_char(line + 1)
        } else {
            self.len_chars()
        };
        if start < end {
            self.delete_range(start, end, true);
            self.caret = start.min(self.len_chars());
            self.sel_anchor = None;
            self.last_insert_end = None;
        }
    }

    /// Insert an empty line above the caret line.
    pub fn blank_line_above(&mut self) {
        self.begin_line_edit();
        let line = self.char_to_line(self.caret);
        let at = self.line_to_char(line);
        self.apply_insert(at, "\n", true);
        self.push_undo(Edit::Insert {
            index: at,
            text: "\n".into(),
        });
        self.redo.clear();
        self.last_insert_end = None;
        self.set_caret(at);
    }

    /// Join selected lines (or current + next) with a space.
    pub fn join_lines(&mut self) {
        let (start_line, mut end_line) = self.selected_line_range();
        if start_line == end_line {
            end_line = (start_line + 1).min(self.line_count().saturating_sub(1));
        }
        if start_line >= end_line {
            return;
        }
        let start = self.line_to_char(start_line);
        let end = if end_line + 1 < self.line_count() {
            self.line_to_char(end_line + 1)
        } else {
            self.len_chars()
        };
        let chunk = self.slice(start, end);
        let joined = chunk
            .lines()
            .map(|l| l.trim_end_matches('\r'))
            .collect::<Vec<_>>()
            .join(" ");
        let mut out = joined;
        if chunk.ends_with('\n') {
            out.push('\n');
        }
        self.begin_line_edit();
        self.delete_range(start, end, true);
        self.caret = start;
        self.sel_anchor = None;
        self.insert_without_begin(&out);
        self.set_caret(
            start
                + out
                    .chars()
                    .count()
                    .saturating_sub(if out.ends_with('\n') { 1 } else { 0 }),
        );
    }

    pub fn move_line_up(&mut self) {
        let line = self.char_to_line(self.caret);
        if line == 0 {
            return;
        }
        self.swap_lines(line - 1, line);
        let new_start = self.line_to_char(line - 1);
        self.set_caret(new_start);
    }

    pub fn move_line_down(&mut self) {
        let line = self.char_to_line(self.caret);
        if line + 1 >= self.line_count() {
            return;
        }
        self.swap_lines(line, line + 1);
        let new_start = self.line_to_char(line + 1);
        self.set_caret(new_start);
    }

    fn swap_lines(&mut self, a: usize, b: usize) {
        if a + 1 != b || b >= self.line_count() {
            return;
        }
        let a_start = self.line_to_char(a);
        let b_start = self.line_to_char(b);
        let b_end = if b + 1 < self.line_count() {
            self.line_to_char(b + 1)
        } else {
            self.len_chars()
        };
        let line_a = self.slice(a_start, b_start);
        let line_b = self.slice(b_start, b_end);
        // Normalize: first line always ends with \n when not last pair at EOF oddly.
        let mut first = line_b.trim_end_matches(['\r', '\n']).to_string();
        first.push('\n');
        let second = if line_a.ends_with('\n') || b + 1 >= self.line_count() {
            // Keep second as original first line content.
            if b + 1 >= self.line_count() && !line_b.ends_with('\n') {
                // Moving last line up: second should not force trailing nl beyond doc.
                line_a.trim_end_matches('\n').to_string()
            } else {
                line_a
            }
        } else {
            line_a
        };
        let combined = format!("{first}{second}");
        self.delete_range(a_start, b_end, true);
        self.caret = a_start;
        self.sel_anchor = None;
        self.insert(&combined);
    }

    /// Remove empty lines in the whole document (or selection).
    pub fn remove_empty_lines(&mut self, blank_only: bool) {
        let text = if let Some((s, e)) = self.selection() {
            self.slice(s, e)
        } else {
            self.to_string()
        };
        let filtered: String = text
            .lines()
            .filter(|l| {
                if blank_only {
                    !l.chars().all(|c| c.is_whitespace())
                } else {
                    !l.is_empty()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let mut out = filtered;
        if text.ends_with('\n') && !out.ends_with('\n') {
            out.push('\n');
        }
        if let Some((s, e)) = self.selection() {
            self.set_selection(s, e);
            self.insert(&out);
        } else {
            self.replace_document(&out);
        }
    }

    /// Map selection (or whole doc) through `f`.
    pub fn map_text<F: FnOnce(&str) -> String>(&mut self, f: F) {
        if let Some((s, e)) = self.selection() {
            let src = self.slice(s, e);
            let out = f(&src);
            self.set_selection(s, e);
            self.insert(&out);
        } else {
            let src = self.to_string();
            self.replace_document(&f(&src));
        }
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn char_to_line(&self, char_idx: usize) -> usize {
        if self.len_chars() == 0 {
            return 0;
        }
        self.rope.char_to_line(char_idx.min(self.len_chars()))
    }

    pub fn line_to_char(&self, line_idx: usize) -> usize {
        let line = line_idx.min(self.line_count().saturating_sub(1));
        self.rope.line_to_char(line)
    }

    pub fn line(&self, line_idx: usize) -> String {
        if line_idx >= self.line_count() {
            return String::new();
        }
        self.rope.line(line_idx).to_string()
    }

    /// Full document as owned String (avoid for huge files in hot paths).
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Slice of characters as String.
    pub fn slice(&self, start: usize, end: usize) -> String {
        let start = start.min(self.len_chars());
        let end = end.min(self.len_chars()).max(start);
        self.rope.slice(start..end).to_string()
    }

    pub fn insert(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        self.begin_line_edit();
        self.insert_without_begin(text);
    }

    /// Insert at caret (replace selection). Does not clear `last_line_edit` (for compound edits).
    fn insert_without_begin(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        if let Some((start, end)) = self.selection() {
            self.delete_range(start, end, true);
            self.caret = start;
            self.sel_anchor = None;
        }
        let index = self.caret;
        self.apply_insert(index, text, true);
        self.caret = index + text.chars().count();
        self.sel_anchor = None;

        // Coalesce adjacent inserts for undo.
        if self.last_insert_end == Some(index) {
            if let Some(Edit::Insert { text: prev, .. }) = self.undo.back_mut() {
                prev.push_str(text);
            }
        } else {
            self.push_undo(Edit::Insert {
                index,
                text: text.to_string(),
            });
        }
        self.last_insert_end = Some(self.caret);
        self.redo.clear();
    }

    pub fn delete_backward(&mut self) {
        self.begin_line_edit();
        if let Some((start, end)) = self.selection() {
            self.delete_range(start, end, true);
            self.caret = start;
            self.sel_anchor = None;
            self.last_insert_end = None;
            return;
        }
        if self.caret == 0 {
            return;
        }
        let start = self.caret - 1;
        let end = self.caret;
        self.delete_range(start, end, true);
        self.caret = start;
        self.last_insert_end = None;
    }

    pub fn delete_forward(&mut self) {
        self.begin_line_edit();
        if let Some((start, end)) = self.selection() {
            self.delete_range(start, end, true);
            self.caret = start;
            self.sel_anchor = None;
            self.last_insert_end = None;
            return;
        }
        if self.caret >= self.len_chars() {
            return;
        }
        let start = self.caret;
        let end = self.caret + 1;
        self.delete_range(start, end, true);
        self.last_insert_end = None;
    }

    fn delete_range(&mut self, start: usize, end: usize, record_undo: bool) {
        if start >= end {
            return;
        }
        let lines_before = self.line_count();
        let start_line = self.char_to_line(start);
        let at_line_start = start == self.line_to_char(start_line);
        let text = self.slice(start, end);
        self.rope.remove(start..end);
        let n = lines_before.saturating_sub(self.line_count());
        if n > 0 {
            let first = if at_line_start {
                start_line
            } else {
                start_line + 1
            };
            self.merge_line_edit(LineStructureEdit::Delete { first, n });
        }
        if record_undo {
            self.push_undo(Edit::Delete { index: start, text });
            self.redo.clear();
        }
    }

    fn apply_insert(&mut self, index: usize, text: &str, _record: bool) {
        let lines_before = self.line_count();
        let at_line = if self.len_chars() == 0 {
            0
        } else {
            self.char_to_line(index.min(self.len_chars()))
        };
        let at_line_start = self.len_chars() == 0 || index == self.line_to_char(at_line);
        self.rope.insert(index, text);
        let n = self.line_count().saturating_sub(lines_before);
        if n > 0 {
            let at = if at_line_start { at_line } else { at_line + 1 };
            self.merge_line_edit(LineStructureEdit::Insert { at, n });
        }
    }

    fn push_undo(&mut self, edit: Edit) {
        if self.undo.len() >= MAX_UNDO {
            self.undo.pop_front();
        }
        self.undo.push_back(edit);
    }

    /// Replace full document text and record one undo step (for UI sync).
    /// Consecutive replaces coalesce into one undo entry.
    pub fn replace_document(&mut self, new_text: &str) {
        let old = self.to_string();
        if old == new_text {
            return;
        }
        self.rope = Rope::from_str(new_text);
        self.caret = self.caret.min(self.len_chars());
        self.sel_anchor = None;
        // Coalesce: keep original `old` from the first replace in a streak.
        if let Some(Edit::Replace { new, .. }) = self.undo.back_mut() {
            *new = new_text.to_string();
        } else {
            self.push_undo(Edit::Replace {
                old,
                new: new_text.to_string(),
            });
        }
        self.last_insert_end = None;
        self.last_line_edit = None;
        self.redo.clear();
    }

    pub fn undo(&mut self) -> bool {
        let Some(edit) = self.undo.pop_back() else {
            return false;
        };
        match &edit {
            Edit::Insert { index, text } => {
                let end = index + text.chars().count();
                self.rope.remove(*index..end);
                self.caret = *index;
            }
            Edit::Delete { index, text } => {
                self.rope.insert(*index, text);
                self.caret = index + text.chars().count();
            }
            Edit::Replace { old, .. } => {
                self.rope = Rope::from_str(old);
                self.caret = self.caret.min(self.len_chars());
            }
        }
        self.redo.push_back(edit);
        self.sel_anchor = None;
        self.last_insert_end = None;
        self.last_line_edit = None;
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(edit) = self.redo.pop_back() else {
            return false;
        };
        match &edit {
            Edit::Insert { index, text } => {
                self.rope.insert(*index, text);
                self.caret = index + text.chars().count();
            }
            Edit::Delete { index, text } => {
                let end = index + text.chars().count();
                self.rope.remove(*index..end);
                self.caret = *index;
            }
            Edit::Replace { new, .. } => {
                self.rope = Rope::from_str(new);
                self.caret = self.caret.min(self.len_chars());
            }
        }
        self.undo.push_back(edit);
        self.sel_anchor = None;
        self.last_insert_end = None;
        self.last_line_edit = None;
        true
    }

    /// Find next plain-text match starting after `from` (char index). Wraps around.
    pub fn find_next(&self, query: &str, from: usize, wrap: bool) -> Option<(usize, usize)> {
        if query.is_empty() {
            return None;
        }
        let text = self.to_string();
        let q_len = query.chars().count();
        if let Some(byte_pos) =
            text[char_to_byte(&text, from.min(text.chars().count()))..].find(query)
        {
            let abs_byte = char_to_byte(&text, from.min(text.chars().count())) + byte_pos;
            let start = byte_to_char(&text, abs_byte);
            return Some((start, start + q_len));
        }
        if wrap && from > 0 {
            if let Some(byte_pos) = text.find(query) {
                let start = byte_to_char(&text, byte_pos);
                if start < from {
                    return Some((start, start + q_len));
                }
            }
        }
        None
    }

    pub fn find_prev(&self, query: &str, from: usize, wrap: bool) -> Option<(usize, usize)> {
        if query.is_empty() {
            return None;
        }
        let text = self.to_string();
        let q_len = query.chars().count();
        let search_end = char_to_byte(&text, from.min(text.chars().count()));
        if let Some(byte_pos) = text[..search_end].rfind(query) {
            let start = byte_to_char(&text, byte_pos);
            return Some((start, start + q_len));
        }
        if wrap {
            if let Some(byte_pos) = text.rfind(query) {
                let start = byte_to_char(&text, byte_pos);
                if start >= from {
                    return Some((start, start + q_len));
                }
            }
        }
        None
    }
}

fn char_to_byte(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

fn byte_to_char(s: &str, byte_idx: usize) -> usize {
    s[..byte_idx].chars().count()
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_undo() {
        let mut b = TextBuffer::new();
        b.insert("hello");
        assert_eq!(b.to_string(), "hello");
        assert!(b.undo());
        assert_eq!(b.to_string(), "");
        assert!(b.redo());
        assert_eq!(b.to_string(), "hello");
    }

    #[test]
    fn find_next_wraps() {
        let b = TextBuffer::from_str("abc def abc");
        let (s, e) = b.find_next("abc", 4, true).unwrap();
        assert_eq!((s, e), (8, 11));
        let (s, e) = b.find_next("abc", 11, true).unwrap();
        assert_eq!((s, e), (0, 3));
    }

    #[test]
    fn select_word_at_middle() {
        let mut b = TextBuffer::from_str("foo bar_baz qux");
        b.select_word_at(6); // inside bar_baz
        assert_eq!(b.selection(), Some((4, 11)));
    }

    #[test]
    fn toggle_line_comments() {
        let mut b = TextBuffer::from_str("one\ntwo\n");
        b.set_selection(0, b.len_chars());
        b.toggle_line_comments("// ");
        assert_eq!(b.to_string(), "// one\n// two\n");
        b.set_selection(0, b.len_chars());
        b.toggle_line_comments("// ");
        assert_eq!(b.to_string(), "one\ntwo\n");
    }

    #[test]
    fn stream_comment_wraps_selection() {
        let mut b = TextBuffer::from_str("hello");
        b.set_selection(0, 5);
        b.stream_comment("/*", "*/");
        assert_eq!(b.to_string(), "/*hello*/");
        b.stream_uncomment("/*", "*/");
        assert_eq!(b.to_string(), "hello");
    }

    #[test]
    fn comment_lines_with_leading_unicode_whitespace() {
        // Non-breaking space before text: byte offset != char offset.
        let mut b = TextBuffer::from_str("\u{00A0}café\n");
        b.set_selection(0, b.len_chars());
        b.comment_lines("// ");
        assert_eq!(b.to_string(), "\u{00A0}// café\n");
        b.set_selection(0, b.len_chars());
        b.uncomment_lines("// ");
        assert_eq!(b.to_string(), "\u{00A0}café\n");
    }

    #[test]
    fn line_structure_insert_at_start() {
        let mut b = TextBuffer::from_str("a\nb\nc\n");
        b.set_caret(0);
        b.insert("\n\n\n");
        assert_eq!(
            b.take_line_structure_edit(),
            Some(LineStructureEdit::Insert { at: 0, n: 3 })
        );
    }

    #[test]
    fn line_structure_delete_line() {
        let mut b = TextBuffer::from_str("a\nb\nc\n");
        b.set_caret(b.line_to_char(1));
        b.delete_line();
        assert_eq!(
            b.take_line_structure_edit(),
            Some(LineStructureEdit::Delete { first: 1, n: 1 })
        );
    }
}
