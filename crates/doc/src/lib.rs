//! Document tabs and metadata.

use buffer::TextBuffer;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Stable id for one open document. Survives tab move, sort, and reorder.
pub type DocumentId = u64;

/// Why [`Document::try_buffer_mut`] refused a mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditDenied {
    /// App-level read-only flag.
    ReadOnly,
    /// Async open still in progress.
    Loading,
}

impl EditDenied {
    pub fn status_message(self) -> &'static str {
        match self {
            Self::ReadOnly => "Document is read-only",
            Self::Loading => "Document is still loading",
        }
    }
}

/// How save writes bytes for this tab (memory stays UTF-8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileEncoding {
    /// UTF-8 without BOM.
    #[default]
    Utf8,
    /// UTF-8 with leading BOM on disk.
    Utf8Bom,
    /// UTF-16 little-endian with BOM on disk.
    Utf16Le,
    /// UTF-16 big-endian with BOM on disk.
    Utf16Be,
    /// Windows-1252 (ANSI stand-in); lossy on save.
    Windows1252,
}

impl FileEncoding {
    pub fn label(self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Utf8Bom => "UTF-8-BOM",
            Self::Utf16Le => "UTF-16 LE",
            Self::Utf16Be => "UTF-16 BE",
            Self::Windows1252 => "Windows-1252",
        }
    }
}

/// Remap 0-based line indices after inserting `n` lines at `at`.
/// Marks on lines `>= at` move down by `n`.
pub fn remap_lines_insert(set: &mut BTreeSet<usize>, at: usize, n: usize) {
    if n == 0 {
        return;
    }
    *set = set
        .iter()
        .map(|&line| if line >= at { line + n } else { line })
        .collect();
}

/// Remap after deleting `n` lines starting at `first_removed` (inclusive).
/// Marks in the removed range drop. Marks at/after `first_removed + n` move up.
pub fn remap_lines_delete(set: &mut BTreeSet<usize>, first_removed: usize, n: usize) {
    if n == 0 {
        return;
    }
    let end = first_removed + n;
    *set = set
        .iter()
        .filter_map(|&line| {
            if line >= first_removed && line < end {
                None
            } else if line >= end {
                Some(line - n)
            } else {
                Some(line)
            }
        })
        .collect();
}

/// One open document (one tab).
#[derive(Debug, Clone)]
pub struct Document {
    /// Stable id assigned when the tab is created.
    pub id: DocumentId,
    pub title: String,
    pub path: Option<PathBuf>,
    pub buffer: TextBuffer,
    pub dirty: bool,
    /// `buffer.edit_generation()` at last save / clean load.
    pub saved_generation: u64,
    /// Encoding used when saving this tab.
    pub encoding: FileEncoding,
    /// Language id for highlight / format (e.g. `rust`, `python`, `plain`).
    pub language: String,
    /// True while a background load is still running.
    pub loading: bool,
    /// Follow file growth (log tail).
    pub tail_follow: bool,
    /// Last synced on-disk byte length while tailing.
    pub tail_bytes: u64,
    /// App-level read-only (blocks edits in the UI).
    pub read_only: bool,
    /// Bookmarked line indices (0-based).
    pub bookmarks: BTreeSet<usize>,
    /// Unsaved edit marks (amber gutter). Edited since last save.
    pub changed_unsaved: BTreeSet<usize>,
    /// Saved edit marks (green gutter). Edited earlier, then saved.
    pub changed_saved: BTreeSet<usize>,
    /// Line count when line-index sets last matched the buffer (for remap).
    pub line_mark_basis: usize,
    /// Hidden line indices (0-based); View → Hide Lines / fold light path.
    pub hidden_lines: BTreeSet<usize>,
    /// Optional tab color id 1..=5; `None` = default.
    pub tab_colour: Option<u8>,
    /// Pinned tab: kept by Close All but Pinned. Default false. Pin UI may be missing.
    pub pinned: bool,
    /// Style marks 1..=5: line indices (Search → Style).
    pub style_marks: [BTreeSet<usize>; 5],
    /// Extra multi-select ranges (char indices), newest last. Primary selection is separate.
    pub multi_sels: Vec<(usize, usize)>,
}

impl Document {
    pub fn untitled(id: DocumentId, number: usize) -> Self {
        let buffer = TextBuffer::new();
        let line_mark_basis = buffer.line_count();
        Self {
            id,
            title: format!("Untitled-{number}"),
            path: None,
            buffer,
            dirty: false,
            saved_generation: 0,
            encoding: FileEncoding::Utf8,
            language: "plain".into(),
            loading: false,
            tail_follow: false,
            tail_bytes: 0,
            read_only: false,
            bookmarks: BTreeSet::new(),
            changed_unsaved: BTreeSet::new(),
            changed_saved: BTreeSet::new(),
            line_mark_basis,
            hidden_lines: BTreeSet::new(),
            tab_colour: None,
            pinned: false,
            style_marks: Default::default(),
            multi_sels: Vec::new(),
        }
    }

    pub fn from_path(id: DocumentId, path: PathBuf, content: String) -> Self {
        Self::from_path_with_encoding(id, path, content, FileEncoding::Utf8)
    }

    pub fn from_path_with_encoding(
        id: DocumentId,
        path: PathBuf,
        content: String,
        encoding: FileEncoding,
    ) -> Self {
        let title = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        let language = detect_language(&path);
        let tail_bytes = std::fs::metadata(&path)
            .map(|m| m.len())
            .unwrap_or(content.len() as u64);
        let buffer = TextBuffer::from_str(&content);
        let line_mark_basis = buffer.line_count();
        Self {
            id,
            title,
            path: Some(path),
            buffer,
            dirty: false,
            saved_generation: 0,
            encoding,
            language,
            loading: false,
            tail_follow: false,
            tail_bytes,
            read_only: false,
            bookmarks: BTreeSet::new(),
            changed_unsaved: BTreeSet::new(),
            changed_saved: BTreeSet::new(),
            line_mark_basis,
            hidden_lines: BTreeSet::new(),
            tab_colour: None,
            pinned: false,
            style_marks: Default::default(),
            multi_sels: Vec::new(),
        }
    }
}

/// Snapshot of caret/selection line geometry before a buffer edit.
#[derive(Debug, Clone, Copy)]
pub struct LineEditSnap {
    pub start_line: usize,
    /// True when the edit starts at column 0 of `start_line`.
    pub at_line_start: bool,
    pub line_count: usize,
}

impl Document {
    /// Capture line geometry before mutating the buffer.
    pub fn snap_edit(&self) -> LineEditSnap {
        let buf = &self.buffer;
        let pos = buf
            .selection()
            .map(|(s, e)| s.min(e))
            .unwrap_or_else(|| buf.caret());
        let start_line = buf.char_to_line(pos);
        let at_line_start = pos == buf.line_to_char(start_line);
        LineEditSnap {
            start_line,
            at_line_start,
            line_count: buf.line_count(),
        }
    }

    /// Apply remap from a before-edit snapshot, then refresh `line_mark_basis`.
    pub fn apply_line_snap(&mut self, snap: LineEditSnap) {
        let new_count = self.buffer.line_count();
        let delta = new_count as isize - snap.line_count as isize;
        if delta > 0 {
            let at = if snap.at_line_start {
                snap.start_line
            } else {
                snap.start_line + 1
            };
            self.remap_all_line_sets_insert(at, delta as usize);
        } else if delta < 0 {
            let first = if snap.at_line_start {
                snap.start_line
            } else {
                snap.start_line + 1
            };
            self.remap_all_line_sets_delete(first, (-delta) as usize);
        }
        self.line_mark_basis = new_count;
        self.clamp_line_marks_to_buffer();
    }

    /// Apply a buffer-recorded line-structure edit, then refresh `line_mark_basis`.
    pub fn apply_line_structure_edit(&mut self, edit: buffer::LineStructureEdit) {
        match edit {
            buffer::LineStructureEdit::Insert { at, n } => {
                self.remap_all_line_sets_insert(at, n);
            }
            buffer::LineStructureEdit::Delete { first, n } => {
                self.remap_all_line_sets_delete(first, n);
            }
        }
        self.line_mark_basis = self.buffer.line_count();
        self.clamp_line_marks_to_buffer();
    }

    /// Prefer buffer hook when present. Returns true when a hook was applied.
    pub fn consume_line_structure_edit(&mut self) -> bool {
        match self.buffer.take_line_structure_edit() {
            Some(edit) => {
                self.apply_line_structure_edit(edit);
                true
            }
            None => false,
        }
    }

    /// Drop line marks that fell past the end of the buffer.
    pub fn clamp_line_marks_to_buffer(&mut self) {
        let max = self.buffer.line_count();
        let clamp = |set: &mut BTreeSet<usize>| {
            set.retain(|&line| line < max);
        };
        clamp(&mut self.bookmarks);
        clamp(&mut self.changed_unsaved);
        clamp(&mut self.changed_saved);
        clamp(&mut self.hidden_lines);
        for slot in &mut self.style_marks {
            clamp(slot);
        }
        self.line_mark_basis = max;
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Mark clean and record the current buffer revision as saved.
    pub fn mark_clean(&mut self) {
        self.buffer.break_typing_coalesce();
        self.saved_generation = self.buffer.edit_generation();
        self.dirty = false;
    }

    /// Set dirty from whether the buffer revision matches the last save.
    pub fn sync_dirty_from_revision(&mut self) {
        self.dirty = self.buffer.edit_generation() != self.saved_generation;
    }

    /// Reason edits are blocked, if any.
    pub fn edit_denied(&self) -> Option<EditDenied> {
        if self.loading {
            Some(EditDenied::Loading)
        } else if self.read_only {
            Some(EditDenied::ReadOnly)
        } else {
            None
        }
    }

    /// Mutable buffer access when the document allows edits.
    pub fn try_buffer_mut(&mut self) -> Result<&mut TextBuffer, EditDenied> {
        if let Some(denied) = self.edit_denied() {
            Err(denied)
        } else {
            Ok(&mut self.buffer)
        }
    }

    /// All change-history line indices (unsaved ∪ saved).
    pub fn change_history_lines(&self) -> BTreeSet<usize> {
        self.changed_unsaved
            .union(&self.changed_saved)
            .copied()
            .collect()
    }

    /// Counts: (unsaved amber, saved green).
    pub fn change_history_counts(&self) -> (usize, usize) {
        (self.changed_unsaved.len(), self.changed_saved.len())
    }

    /// Unsaved takes priority when both somehow set.
    pub fn change_history_is_saved(&self, line: usize) -> Option<bool> {
        if self.changed_unsaved.contains(&line) {
            Some(false)
        } else if self.changed_saved.contains(&line) {
            Some(true)
        } else {
            None
        }
    }

    /// Record one line as edited (unsaved amber mark).
    pub fn note_line_changed(&mut self, line: usize) {
        self.changed_saved.remove(&line);
        self.changed_unsaved.insert(line);
    }

    /// After save: unsaved marks become saved (green).
    pub fn promote_change_history_on_save(&mut self) {
        for line in std::mem::take(&mut self.changed_unsaved) {
            self.changed_saved.insert(line);
        }
    }

    /// Clear Scintilla-style change-history marks.
    pub fn clear_change_history(&mut self) {
        self.changed_unsaved.clear();
        self.changed_saved.clear();
    }

    /// Shift line-index sets after the buffer line count changes.
    /// Uses caret position after the edit (MVP heuristic).
    pub fn sync_line_marks_after_edit(&mut self) {
        let new_count = self.buffer.line_count();
        let old = self.line_mark_basis;
        if new_count == old {
            return;
        }
        let delta = new_count as isize - old as isize;
        let caret_line = self.buffer.char_to_line(self.buffer.caret());
        if delta > 0 {
            let n = delta as usize;
            let at = caret_line.saturating_sub(n);
            self.remap_all_line_sets_insert(at, n);
        } else {
            let n = (-delta) as usize;
            let first_removed = caret_line + 1;
            self.remap_all_line_sets_delete(first_removed, n);
        }
        self.line_mark_basis = new_count;
        self.clamp_line_marks_to_buffer();
    }

    fn remap_all_line_sets_insert(&mut self, at: usize, n: usize) {
        remap_lines_insert(&mut self.bookmarks, at, n);
        remap_lines_insert(&mut self.changed_unsaved, at, n);
        remap_lines_insert(&mut self.changed_saved, at, n);
        remap_lines_insert(&mut self.hidden_lines, at, n);
        for slot in &mut self.style_marks {
            remap_lines_insert(slot, at, n);
        }
    }

    fn remap_all_line_sets_delete(&mut self, first_removed: usize, n: usize) {
        remap_lines_delete(&mut self.bookmarks, first_removed, n);
        remap_lines_delete(&mut self.changed_unsaved, first_removed, n);
        remap_lines_delete(&mut self.changed_saved, first_removed, n);
        remap_lines_delete(&mut self.hidden_lines, first_removed, n);
        for slot in &mut self.style_marks {
            remap_lines_delete(slot, first_removed, n);
        }
    }
}

/// Tab strip and active document index.
#[derive(Debug)]
pub struct TabSet {
    docs: Vec<Document>,
    active: usize,
    next_untitled: usize,
    next_id: DocumentId,
}

impl Default for TabSet {
    fn default() -> Self {
        Self::new()
    }
}

impl TabSet {
    pub fn new() -> Self {
        let mut tabs = Self {
            docs: Vec::new(),
            active: 0,
            next_untitled: 1,
            next_id: 1,
        };
        tabs.open_untitled();
        tabs
    }

    /// Allocate the next stable document id.
    pub fn alloc_id(&mut self) -> DocumentId {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        id
    }

    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.docs.len() {
            self.active = index;
        }
    }

    pub fn active(&self) -> &Document {
        let idx = self.active.min(self.docs.len().saturating_sub(1));
        &self.docs[idx]
    }

    pub fn active_mut(&mut self) -> &mut Document {
        let idx = self.active.min(self.docs.len().saturating_sub(1));
        self.active = idx;
        &mut self.docs[idx]
    }

    pub fn get(&self, index: usize) -> Option<&Document> {
        self.docs.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Document> {
        self.docs.get_mut(index)
    }

    /// Tab index for a document id, if that tab is still open.
    pub fn index_of_id(&self, id: DocumentId) -> Option<usize> {
        self.docs.iter().position(|d| d.id == id)
    }

    pub fn get_by_id(&self, id: DocumentId) -> Option<&Document> {
        self.index_of_id(id).and_then(|i| self.docs.get(i))
    }

    pub fn get_mut_by_id(&mut self, id: DocumentId) -> Option<&mut Document> {
        let i = self.index_of_id(id)?;
        self.docs.get_mut(i)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.docs.iter()
    }

    pub fn open_untitled(&mut self) -> usize {
        let id = self.alloc_id();
        let doc = Document::untitled(id, self.next_untitled);
        self.next_untitled += 1;
        self.docs.push(doc);
        self.active = self.docs.len() - 1;
        self.active
    }

    pub fn open_document(&mut self, doc: Document) -> usize {
        // Reuse existing tab for same path.
        if let Some(path) = doc.path.as_ref() {
            if let Some(i) = self.docs.iter().position(|d| d.path.as_ref() == Some(path)) {
                self.active = i;
                return i;
            }
        }
        self.docs.push(doc);
        self.active = self.docs.len() - 1;
        self.active
    }

    /// Close tab at `index`. Returns false if it was the last tab (replaced with untitled).
    pub fn close(&mut self, index: usize) -> bool {
        if index >= self.docs.len() {
            return false;
        }
        self.docs.remove(index);
        if self.docs.is_empty() {
            self.open_untitled();
            return false;
        }
        if self.active >= self.docs.len() {
            self.active = self.docs.len() - 1;
        } else if index < self.active {
            self.active -= 1;
        }
        true
    }

    /// Move the active tab by `delta` (-1 or +1). Returns true if it moved.
    pub fn move_active_tab(&mut self, delta: isize) -> bool {
        if self.docs.len() < 2 || delta == 0 {
            return false;
        }
        let from = self.active;
        let to = from as isize + delta;
        if to < 0 || to as usize >= self.docs.len() {
            return false;
        }
        let to = to as usize;
        self.docs.swap(from, to);
        self.active = to;
        true
    }

    /// Remap a tab index after `move_tab(from, to)`.
    pub fn remap_index(idx: usize, from: usize, to: usize) -> usize {
        if idx == from {
            return to;
        }
        if from < to {
            if idx > from && idx <= to {
                idx - 1
            } else {
                idx
            }
        } else if idx >= to && idx < from {
            idx + 1
        } else {
            idx
        }
    }

    /// Move tab at `from` to index `to` (same length). Updates the active index.
    pub fn move_tab(&mut self, from: usize, to: usize) -> bool {
        if from == to || from >= self.docs.len() || to >= self.docs.len() {
            return false;
        }
        let doc = self.docs.remove(from);
        self.docs.insert(to, doc);
        self.active = Self::remap_index(self.active, from, to);
        true
    }

    /// Sort open tabs. Keeps the same document active when possible.
    pub fn sort_tabs<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&Document, &Document) -> std::cmp::Ordering,
    {
        if self.docs.len() < 2 {
            return;
        }
        let key_id = self.docs[self.active].id;
        self.docs.sort_by(|a, b| cmp(a, b));
        self.active = self.index_of_id(key_id).unwrap_or(0);
    }
}

pub fn detect_language(path: &Path) -> String {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "rs" => "rust".into(),
        "c" | "h" => "c".into(),
        "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" => "cpp".into(),
        "json" => "json".into(),
        "py" | "pyw" => "python".into(),
        "sql" => "sql".into(),
        "md" | "markdown" => "markdown".into(),
        "toml" => "toml".into(),
        "yaml" | "yml" => "yaml".into(),
        "sh" | "bash" => "shell".into(),
        "js" => "javascript".into(),
        "ts" => "typescript".into(),
        "html" | "htm" => "html".into(),
        "css" => "css".into(),
        "go" => "go".into(),
        "java" => "java".into(),
        "txt" => "plain".into(),
        _ => "plain".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn untitled_and_close() {
        let mut tabs = TabSet::new();
        assert_eq!(tabs.len(), 1);
        tabs.open_untitled();
        assert_eq!(tabs.len(), 2);
        tabs.close(0);
        assert_eq!(tabs.len(), 1);
    }

    #[test]
    fn move_tab_keeps_active_and_order() {
        let mut tabs = TabSet::new();
        tabs.open_untitled();
        tabs.open_untitled();
        assert_eq!(tabs.len(), 3);
        tabs.set_active(0);
        assert!(tabs.move_tab(0, 2));
        assert_eq!(tabs.active_index(), 2);
        assert_eq!(TabSet::remap_index(1, 0, 2), 0);
        assert_eq!(TabSet::remap_index(2, 0, 2), 1);
        assert!(tabs.move_tab(2, 0));
        assert_eq!(tabs.active_index(), 0);
    }

    #[test]
    fn index_of_id_survives_move() {
        let mut tabs = TabSet::new();
        let id0 = tabs.get(0).unwrap().id;
        tabs.open_untitled();
        let id1 = tabs.get(1).unwrap().id;
        assert_ne!(id0, id1);
        tabs.set_active(0);
        assert!(tabs.move_tab(0, 1));
        assert_eq!(tabs.index_of_id(id0), Some(1));
        assert_eq!(tabs.index_of_id(id1), Some(0));
        assert_eq!(tabs.get_by_id(id0).map(|d| d.id), Some(id0));
    }

    #[test]
    fn change_history_note_promote_and_clear() {
        let mut doc = Document::untitled(1, 1);
        doc.note_line_changed(2);
        doc.note_line_changed(5);
        assert_eq!(doc.changed_unsaved.len(), 2);
        assert!(doc.changed_saved.is_empty());
        assert_eq!(doc.change_history_counts(), (2, 0));
        doc.promote_change_history_on_save();
        assert!(doc.changed_unsaved.is_empty());
        assert_eq!(doc.changed_saved.len(), 2);
        assert_eq!(doc.change_history_counts(), (0, 2));
        assert_eq!(doc.change_history_is_saved(2), Some(true));
        doc.note_line_changed(2);
        assert!(doc.changed_unsaved.contains(&2));
        assert!(!doc.changed_saved.contains(&2));
        assert_eq!(doc.change_history_is_saved(2), Some(false));
        doc.clear_change_history();
        assert!(doc.change_history_lines().is_empty());
    }

    #[test]
    fn change_history_remap_survives_undo_newline() {
        let mut doc = Document::untitled(1, 1);
        let text: String = (0..8).map(|i| format!("L{i}\n")).collect();
        doc.buffer = TextBuffer::from_str(&text);
        doc.line_mark_basis = doc.buffer.line_count();
        doc.note_line_changed(5);
        doc.buffer.set_caret(0);
        doc.buffer.insert("\n\n");
        assert!(doc.consume_line_structure_edit());
        assert!(doc.changed_unsaved.contains(&7));
        assert!(doc.buffer.undo());
        assert!(doc.consume_line_structure_edit());
        assert!(doc.changed_unsaved.contains(&5));
        assert!(!doc.changed_unsaved.contains(&7));
    }

    #[test]
    fn remap_insert_and_delete() {
        let mut set: BTreeSet<usize> = [1usize, 3, 5].into_iter().collect();
        remap_lines_insert(&mut set, 2, 2);
        // Lines >= 2 shift: 3→5, 5→7; line 1 stays.
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![1, 5, 7]);
        remap_lines_delete(&mut set, 5, 1);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![1, 6]);
    }

    #[test]
    fn sync_line_marks_shifts_on_newline() {
        let mut doc = Document::untitled(1, 1);
        doc.buffer = TextBuffer::from_str("a\nb\nc\n");
        doc.line_mark_basis = doc.buffer.line_count();
        doc.note_line_changed(1);
        doc.bookmarks.insert(2);
        // Insert at start of line 1 (column 0).
        let at = doc.buffer.line_to_char(1);
        doc.buffer.set_caret(at);
        let snap = doc.snap_edit();
        doc.buffer.insert("\n");
        doc.apply_line_snap(snap);
        assert!(doc.changed_unsaved.contains(&2));
        assert!(doc.bookmarks.contains(&3));
    }

    #[test]
    fn bookmark_insert_three_lines_at_start_shifts_mark() {
        let mut doc = Document::untitled(1, 1);
        let text: String = (0..15).map(|i| format!("L{i}\n")).collect();
        doc.buffer = TextBuffer::from_str(&text);
        doc.line_mark_basis = doc.buffer.line_count();
        doc.bookmarks.insert(10);
        doc.buffer.set_caret(0);
        doc.buffer.insert("\n\n\n");
        assert!(doc.consume_line_structure_edit());
        assert!(doc.bookmarks.contains(&13));
        assert!(!doc.bookmarks.contains(&10));
    }

    #[test]
    fn bookmark_delete_lines_above_shifts_down() {
        let mut doc = Document::untitled(1, 1);
        let text: String = (0..15).map(|i| format!("L{i}\n")).collect();
        doc.buffer = TextBuffer::from_str(&text);
        doc.line_mark_basis = doc.buffer.line_count();
        doc.bookmarks.insert(10);
        // Delete lines 0..2 (three lines).
        let end = doc.buffer.line_to_char(3);
        doc.buffer.set_selection(0, end);
        doc.buffer.delete_backward();
        assert!(doc.consume_line_structure_edit());
        assert!(doc.bookmarks.contains(&7));
        assert!(!doc.bookmarks.contains(&10));
    }

    #[test]
    fn bookmark_removed_when_its_line_deleted() {
        let mut doc = Document::untitled(1, 1);
        let text: String = (0..15).map(|i| format!("L{i}\n")).collect();
        doc.buffer = TextBuffer::from_str(&text);
        doc.line_mark_basis = doc.buffer.line_count();
        doc.bookmarks.insert(10);
        doc.buffer.set_caret(doc.buffer.line_to_char(10));
        doc.buffer.delete_line();
        assert!(doc.consume_line_structure_edit());
        assert!(doc.bookmarks.is_empty());
    }

    #[test]
    fn bookmark_remap_via_snap_when_delete_line() {
        let mut doc = Document::untitled(1, 1);
        let text: String = (0..8).map(|i| format!("L{i}\n")).collect();
        doc.buffer = TextBuffer::from_str(&text);
        doc.line_mark_basis = doc.buffer.line_count();
        doc.bookmarks.insert(3);
        doc.buffer.set_caret(doc.buffer.line_to_char(3));
        let snap = doc.snap_edit();
        doc.buffer.delete_line();
        // Drop buffer hook so snap path is exercised.
        let _ = doc.buffer.take_line_structure_edit();
        doc.apply_line_snap(snap);
        assert!(doc.bookmarks.is_empty());
    }

    #[test]
    fn bookmark_remap_document_buffer_edit_and_hook() {
        let mut doc = Document::untitled(1, 1);
        let text: String = (0..12).map(|i| format!("L{i}\n")).collect();
        doc.buffer = TextBuffer::from_str(&text);
        doc.line_mark_basis = doc.buffer.line_count();
        doc.bookmarks.insert(5);
        doc.bookmarks.insert(9);
        doc.buffer.set_caret(0);
        let snap = doc.snap_edit();
        doc.buffer.insert("x\ny\n");
        // Hook wins over snap when both are available.
        assert!(doc.consume_line_structure_edit());
        let _ = snap;
        assert!(doc.bookmarks.contains(&7));
        assert!(doc.bookmarks.contains(&11));
    }

    #[test]
    fn detect_common_extensions() {
        assert_eq!(detect_language(Path::new("a.py")), "python");
        assert_eq!(detect_language(Path::new("a.pyw")), "python");
        assert_eq!(detect_language(Path::new("q.sql")), "sql");
        assert_eq!(detect_language(Path::new("r.md")), "markdown");
        assert_eq!(detect_language(Path::new("r.markdown")), "markdown");
        assert_eq!(detect_language(Path::new("c.toml")), "toml");
        assert_eq!(detect_language(Path::new("d.yaml")), "yaml");
        assert_eq!(detect_language(Path::new("d.yml")), "yaml");
        assert_eq!(detect_language(Path::new("e.sh")), "shell");
        assert_eq!(detect_language(Path::new("e.bash")), "shell");
        assert_eq!(detect_language(Path::new("f.js")), "javascript");
        assert_eq!(detect_language(Path::new("f.ts")), "typescript");
        assert_eq!(detect_language(Path::new("g.html")), "html");
        assert_eq!(detect_language(Path::new("g.css")), "css");
        assert_eq!(detect_language(Path::new("h.go")), "go");
        assert_eq!(detect_language(Path::new("i.java")), "java");
        assert_eq!(detect_language(Path::new("j.txt")), "plain");
    }

    #[test]
    fn try_buffer_mut_blocks_read_only_and_loading() {
        let mut doc = Document::untitled(1, 1);
        assert!(doc.try_buffer_mut().is_ok());
        doc.read_only = true;
        assert_eq!(doc.try_buffer_mut().err(), Some(EditDenied::ReadOnly));
        doc.read_only = false;
        doc.loading = true;
        assert_eq!(doc.try_buffer_mut().err(), Some(EditDenied::Loading));
        doc.loading = false;
        doc.try_buffer_mut().unwrap().insert("ok");
        assert_eq!(doc.buffer.to_string(), "ok");
    }

    #[test]
    fn undo_to_saved_generation_clears_dirty() {
        let mut doc = Document::untitled(1, 1);
        doc.buffer.insert("hello");
        doc.sync_dirty_from_revision();
        assert!(doc.dirty);
        doc.mark_clean();
        assert!(!doc.dirty);
        assert_eq!(doc.saved_generation, 1);
        doc.buffer.insert("!");
        doc.sync_dirty_from_revision();
        assert!(doc.dirty);
        assert!(doc.buffer.undo());
        doc.sync_dirty_from_revision();
        assert!(!doc.dirty);
        assert!(doc.buffer.redo());
        doc.sync_dirty_from_revision();
        assert!(doc.dirty);
    }
}
