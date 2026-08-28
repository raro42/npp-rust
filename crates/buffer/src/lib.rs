//! Rope-backed text buffer with caret and undo/redo.

use ropey::Rope;
use std::collections::VecDeque;

const MAX_UNDO: usize = 256;

#[derive(Debug, Clone)]
enum Edit {
    Insert { index: usize, text: String },
    Delete { index: usize, text: String },
    Replace { old: String, new: String },
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
        }
    }

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

    fn selected_line_range(&self) -> (usize, usize) {
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
        let text = self.slice(start, end);
        self.rope.remove(start..end);
        if record_undo {
            self.push_undo(Edit::Delete { index: start, text });
            self.redo.clear();
        }
    }

    fn apply_insert(&mut self, index: usize, text: &str, _record: bool) {
        self.rope.insert(index, text);
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
        true
    }

    /// Find next plain-text match starting after `from` (char index). Wraps around.
    pub fn find_next(&self, query: &str, from: usize, wrap: bool) -> Option<(usize, usize)> {
        if query.is_empty() {
            return None;
        }
        let text = self.to_string();
        let q_len = query.chars().count();
        if let Some(byte_pos) = text[char_to_byte(&text, from.min(text.chars().count()))..]
            .find(query)
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
}
