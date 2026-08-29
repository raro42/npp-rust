//! Document tabs and metadata.

use buffer::TextBuffer;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// One open document (one tab).
#[derive(Debug, Clone)]
pub struct Document {
    pub title: String,
    pub path: Option<PathBuf>,
    pub buffer: TextBuffer,
    pub dirty: bool,
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
    /// Hidden line indices (0-based); View → Hide Lines / fold light path.
    pub hidden_lines: BTreeSet<usize>,
    /// Optional tab color id 1..=5; `None` = default.
    pub tab_colour: Option<u8>,
    /// Style marks 1..=5: line indices (Search → Style).
    pub style_marks: [BTreeSet<usize>; 5],
    /// Extra multi-select ranges (char indices), newest last. Primary selection is separate.
    pub multi_sels: Vec<(usize, usize)>,
}

impl Document {
    pub fn untitled(id: usize) -> Self {
        Self {
            title: format!("Untitled-{id}"),
            path: None,
            buffer: TextBuffer::new(),
            dirty: false,
            language: "plain".into(),
            loading: false,
            tail_follow: false,
            tail_bytes: 0,
            read_only: false,
            bookmarks: BTreeSet::new(),
            hidden_lines: BTreeSet::new(),
            tab_colour: None,
            style_marks: Default::default(),
            multi_sels: Vec::new(),
        }
    }

    pub fn from_path(path: PathBuf, content: String) -> Self {
        let title = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        let language = detect_language(&path);
        let tail_bytes = std::fs::metadata(&path)
            .map(|m| m.len())
            .unwrap_or(content.len() as u64);
        Self {
            title,
            path: Some(path),
            buffer: TextBuffer::from_str(&content),
            dirty: false,
            language,
            loading: false,
            tail_follow: false,
            tail_bytes,
            read_only: false,
            bookmarks: BTreeSet::new(),
            hidden_lines: BTreeSet::new(),
            tab_colour: None,
            style_marks: Default::default(),
            multi_sels: Vec::new(),
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

/// Tab strip and active document index.
#[derive(Debug)]
pub struct TabSet {
    docs: Vec<Document>,
    active: usize,
    next_untitled: usize,
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
        };
        tabs.open_untitled();
        tabs
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
        &self.docs[self.active]
    }

    pub fn active_mut(&mut self) -> &mut Document {
        &mut self.docs[self.active]
    }

    pub fn get(&self, index: usize) -> Option<&Document> {
        self.docs.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Document> {
        self.docs.get_mut(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.docs.iter()
    }

    pub fn open_untitled(&mut self) -> usize {
        let doc = Document::untitled(self.next_untitled);
        self.next_untitled += 1;
        self.docs.push(doc);
        self.active = self.docs.len() - 1;
        self.active
    }

    pub fn open_document(&mut self, doc: Document) -> usize {
        // Reuse existing tab for same path.
        if let Some(path) = doc.path.as_ref() {
            if let Some(i) = self
                .docs
                .iter()
                .position(|d| d.path.as_ref() == Some(path))
            {
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

    /// Sort open tabs. Keeps the same document active when possible.
    pub fn sort_tabs<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&Document, &Document) -> std::cmp::Ordering,
    {
        if self.docs.len() < 2 {
            return;
        }
        let key_path = self.docs[self.active].path.clone();
        let key_title = self.docs[self.active].title.clone();
        self.docs.sort_by(|a, b| cmp(a, b));
        self.active = self
            .docs
            .iter()
            .position(|d| {
                if let (Some(ap), Some(bp)) = (key_path.as_ref(), d.path.as_ref()) {
                    ap == bp
                } else {
                    d.title == key_title && d.path == key_path
                }
            })
            .unwrap_or(0);
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
}
