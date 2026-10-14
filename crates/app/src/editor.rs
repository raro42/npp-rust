//! Editor application state and commands.

use crate::recent::{is_log_path, short_path_label, AppSettings, LogTailOnOpen, RecentFiles};
use doc::{Document, DocumentId, FileEncoding, TabSet};
use fs::{
    self, LoadChannel, LoadMsg, OpenResult, TailChannel, TailMsg, TailOutcome, TextEncoding,
    LARGE_FILE_THRESHOLD,
};
use highlight::SyntaxHighlighter;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Pending async load keyed by document id + path (tabs can move/close).
#[derive(Debug, Clone)]
pub struct PendingLoad {
    pub document_id: DocumentId,
    pub path: PathBuf,
}

pub struct EditorState {
    pub tabs: TabSet,
    pub find_query: String,
    pub find_open: bool,
    /// Frozen `[lo, hi)` for Find in selection (not persisted).
    pub find_scope: Option<(usize, usize)>,
    pub status: String,
    pub highlighter: SyntaxHighlighter,
    pub load_channel: LoadChannel,
    pub pending: Vec<PendingLoad>,
    /// Paths whose async load was cancelled (tab closed); late results drop.
    cancelled_loads: HashSet<PathBuf>,
    /// Background Ollama Ask / Status (GUI only applies [`crate::ollama::OllamaMsg`]).
    pub ollama_channel: crate::ollama::OllamaChannel,
    /// True while an Ollama worker is in flight (one at a time).
    pub ollama_busy: bool,
    /// Background tail polls (GUI only applies [`TailMsg`]).
    pub tail_channel: TailChannel,
    /// Paths with an in-flight tail worker (one poll per path).
    tail_inflight: HashSet<PathBuf>,
    /// Cached highlight spans for active doc (absolute char offsets).
    pub highlight_cache: Vec<highlight::Span>,
    pub highlight_lang: String,
    pub highlight_dirty: bool,
    /// Line range covered by `highlight_cache` (start inclusive, end exclusive).
    pub highlight_cover: (usize, usize),
    /// Cached highlight spans for the dual-view other pane tab.
    pub highlight_cache_other: Vec<highlight::Span>,
    pub highlight_lang_other: String,
    pub highlight_dirty_other: bool,
    /// Line range covered by `highlight_cache_other` (start inclusive, end exclusive).
    pub highlight_cover_other: (usize, usize),
    /// Tab index that `highlight_cache_other` was built for (`None` = empty).
    pub highlight_tab_other: Option<usize>,
    pub recent: RecentFiles,
    pub settings: AppSettings,
    /// After opening `*.log`, UI may show the tail prompt.
    pub pending_log_tail_prompt: bool,
    pub pending_encoding_notice: Option<String>,
    /// UI should jump scroll to line 1 (set on open / new).
    pub reset_view: bool,
    /// Last time we scheduled disk polls for tail follow.
    tail_last_poll: Instant,
    /// Begin/End Select: first mark `(char_pos, virtual_col past EOL)`, or `None` when idle.
    pub begin_end_select: Option<(usize, Option<usize>)>,
    /// Search-on-Internet base URL (query is appended).
    pub search_engine: String,
    /// Show space/tab glyphs in the editor.
    pub show_whitespace: bool,
    /// Show end-of-line marks.
    pub show_eol: bool,
    /// Show other non-printing / control characters.
    pub show_npc: bool,
    /// Draw vertical indent guides (every 4 columns).
    pub show_indent_guide: bool,
    /// Soft word wrap (visual; hit-test stays line-based for now).
    pub word_wrap: bool,
    /// Macro recording active.
    pub macro_recording: bool,
    /// Recorded menu command ids.
    pub macro_cmds: Vec<String>,
    /// Tab index waiting for save/discard/cancel before close.
    pub pending_close: Option<usize>,
    /// Active tab waiting for save/discard/cancel before reload-from-disk.
    pub pending_reload: Option<usize>,
    /// After resolving `pending_close`, continue this bulk operation.
    pub bulk_close: BulkClose,
    /// UI should quit when true (no dirty tabs left).
    pub want_quit: bool,
    /// Compare tags need a rebuild after an edit.
    pub compare_stale: bool,
    /// Tab indices closed this frame (at close time), for UI index remap.
    pub closed_tab_indices: Vec<usize>,
    /// Before-edit line snap for change-history remap (tab, snap).
    pending_edit_snap: Option<(usize, doc::LineEditSnap)>,
    /// Folder shown in the Project panel (cwd by default).
    pub workspace_root: PathBuf,
    /// Confirm ANSI save that would map chars to `?` (path to write).
    pub pending_lossy_ansi: Option<PathBuf>,
    /// Last autosave pass (dirty named tabs).
    autosave_last: Instant,
    /// Fold headers to apply after an async session restore open finishes.
    pending_session_folds: HashMap<PathBuf, Vec<usize>>,
    /// Bookmark lines to apply after an async session restore open finishes.
    pending_session_bookmarks: HashMap<PathBuf, Vec<usize>>,
    /// Search → Style 1..=5 mark lines after an async session restore open finishes.
    pending_session_styles: HashMap<PathBuf, [Vec<usize>; 5]>,
    /// Caret (line, col) to apply after an async session restore open finishes.
    pending_session_carets: HashMap<PathBuf, (usize, usize)>,
    /// Selection (anchor line/col, caret line/col) after an async session restore open finishes.
    pending_session_sels: HashMap<PathBuf, crate::session::SessionSelection>,
    /// Language id to apply after an async session restore open finishes.
    pending_session_langs: HashMap<PathBuf, String>,
    /// Save encoding to apply after an async session restore open finishes.
    pending_session_encodings: HashMap<PathBuf, FileEncoding>,
    /// Read-only flag to apply after an async session restore open finishes.
    pending_session_readonly: HashSet<PathBuf>,
    /// Log-tail follow to apply after an async session restore open finishes.
    pending_session_tail: HashSet<PathBuf>,
    /// Path that should become active after its async session open finishes.
    pending_session_active: Option<PathBuf>,
    /// First visible line for the pending active tab (applied with `pending_session_active`).
    pending_session_scroll: Option<usize>,
    /// Per-path first visible line to apply after async session open (inactive tabs too).
    pending_session_scrolls: HashMap<PathBuf, usize>,
    /// UI should scroll to the active caret after session restore (consumed once).
    pub session_follow_caret: bool,
    /// UI should restore first visible line after session restore (consumed once; wins over follow-caret).
    pub session_scroll_line: Option<usize>,
    /// Focused-pane first visible line for the next session write (UI syncs each frame).
    pub session_write_scroll: Option<usize>,
    /// Dual-view other-pane path for the next session write (UI syncs each frame).
    pub session_write_other_view: Option<PathBuf>,
    /// Dual-view other pane had keyboard focus for the next session write (UI syncs each frame).
    pub session_write_focus_other: bool,
    /// Sync vertical scroll for the next session write (UI syncs each frame).
    pub session_write_sync_v: bool,
    /// Sync horizontal scroll for the next session write (UI syncs each frame).
    pub session_write_sync_h: bool,
    /// Compare-on for the next session write (UI syncs each frame).
    pub session_write_compare: bool,
    /// Marked Compare partner path for the next session write (UI syncs each frame).
    pub session_write_compare_partner: Option<PathBuf>,
    /// Dual-view other-pane path to apply after session restore (UI consumes when both tabs exist).
    pub session_other_view: Option<PathBuf>,
    /// Focus the dual-view other pane after session restore (UI consumes with `session_other_view`).
    pub session_focus_other: bool,
    /// Restore Synchronize Vertical Scrolling with dual view (UI consumes with `session_other_view`).
    pub session_sync_scroll_v: bool,
    /// Restore Synchronize Horizontal Scrolling with dual view (UI consumes with `session_other_view`).
    pub session_sync_scroll_h: bool,
    /// Restart Compare after dual-view session restore (UI consumes with `session_other_view`).
    pub session_compare_on: bool,
    /// Re-mark Compare partner tab after session restore (UI consumes when the tab exists).
    pub session_compare_partner: Option<PathBuf>,
}

/// Bulk close mode after a dirty-tab confirm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BulkClose {
    #[default]
    None,
    All,
    AllButCurrent,
    AllButPinned,
    AllToLeft,
    AllToRight,
    AllUnchanged,
    Quit,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorState {
    pub fn new() -> Self {
        let settings = AppSettings::load();
        let word_wrap = settings.word_wrap;
        let show_whitespace = settings.show_whitespace;
        let show_eol = settings.show_eol;
        let show_npc = settings.show_npc;
        let show_indent_guide = settings.show_indent_guide;
        crate::commands::edit::set_text_rtl(settings.text_rtl);
        let find_query = settings.find_query.clone();
        let workspace_root = {
            let from_settings = settings.workspace_root.trim();
            if !from_settings.is_empty() {
                let p = PathBuf::from(from_settings);
                if p.is_dir() {
                    p
                } else {
                    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
                }
            } else {
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            }
        };
        Self {
            tabs: TabSet::new(),
            find_query,
            find_open: false,
            find_scope: None,
            status: "Ready".into(),
            highlighter: SyntaxHighlighter::new(),
            load_channel: LoadChannel::new(),
            pending: Vec::new(),
            cancelled_loads: HashSet::new(),
            ollama_channel: crate::ollama::OllamaChannel::new(),
            ollama_busy: false,
            tail_channel: TailChannel::new(),
            tail_inflight: HashSet::new(),
            highlight_cache: Vec::new(),
            highlight_lang: String::new(),
            highlight_dirty: true,
            highlight_cover: (0, 0),
            highlight_cache_other: Vec::new(),
            highlight_lang_other: String::new(),
            highlight_dirty_other: true,
            highlight_cover_other: (0, 0),
            highlight_tab_other: None,
            recent: RecentFiles::load_limited(settings.recent_limit()),
            settings,
            pending_log_tail_prompt: false,
            pending_encoding_notice: None,
            reset_view: false,
            tail_last_poll: Instant::now() - Duration::from_secs(1),
            begin_end_select: None,
            search_engine: "https://duckduckgo.com/?q=".into(),
            show_whitespace,
            show_eol,
            show_npc,
            show_indent_guide,
            word_wrap,
            macro_recording: false,
            macro_cmds: Vec::new(),
            pending_close: None,
            pending_reload: None,
            bulk_close: BulkClose::None,
            want_quit: false,
            compare_stale: false,
            closed_tab_indices: Vec::new(),
            pending_edit_snap: None,
            workspace_root,
            pending_lossy_ansi: None,
            autosave_last: Instant::now(),
            pending_session_folds: HashMap::new(),
            pending_session_bookmarks: HashMap::new(),
            pending_session_styles: HashMap::new(),
            pending_session_carets: HashMap::new(),
            pending_session_sels: HashMap::new(),
            pending_session_langs: HashMap::new(),
            pending_session_encodings: HashMap::new(),
            pending_session_readonly: HashSet::new(),
            pending_session_tail: HashSet::new(),
            pending_session_active: None,
            pending_session_scroll: None,
            pending_session_scrolls: HashMap::new(),
            session_follow_caret: false,
            session_scroll_line: None,
            session_write_scroll: None,
            session_write_other_view: None,
            session_write_focus_other: false,
            session_write_sync_v: false,
            session_write_sync_h: false,
            session_write_compare: false,
            session_write_compare_partner: None,
            session_other_view: None,
            session_focus_other: false,
            session_sync_scroll_v: false,
            session_sync_scroll_h: false,
            session_compare_on: false,
            session_compare_partner: None,
        }
    }

    /// Drain tab indices closed since the last take (order preserved).
    pub fn take_closed_tabs(&mut self) -> Vec<usize> {
        std::mem::take(&mut self.closed_tab_indices)
    }

    /// Call before mutating a tab buffer so change-history marks can remap.
    pub fn prepare_edit_at(&mut self, tab: usize) {
        if let Some(doc) = self.tabs.get(tab) {
            self.pending_edit_snap = Some((tab, doc.snap_edit()));
        }
    }

    pub fn prepare_edit(&mut self) {
        self.prepare_edit_at(self.tabs.active_index());
    }

    /// True when the active document may be edited. Sets status when blocked.
    pub fn ensure_editable(&mut self) -> bool {
        if let Some(denied) = self.tabs.active().edit_denied() {
            self.status = denied.status_message().into();
            false
        } else {
            true
        }
    }

    /// Run `f` on tab `tab`'s buffer when editable. Sets status when blocked.
    pub fn with_editable_buffer<R>(
        &mut self,
        tab: usize,
        f: impl FnOnce(&mut buffer::TextBuffer) -> R,
    ) -> Result<R, doc::EditDenied> {
        let denied = self.tabs.get(tab).and_then(|d| d.edit_denied());
        if let Some(denied) = denied {
            self.status = denied.status_message().into();
            return Err(denied);
        }
        let Some(doc) = self.tabs.get_mut(tab) else {
            return Err(doc::EditDenied::Loading);
        };
        Ok(f(&mut doc.buffer))
    }

    pub fn mark_text_changed(&mut self) {
        self.mark_text_changed_at(self.tabs.active_index());
    }

    /// Dirty + change-history for a tab (dual view may edit the other pane).
    /// `note_marks`: when true, mark caret/selection lines as unsaved change-history.
    /// Undo/redo pass false so remap runs without re-stamping the caret line.
    pub fn mark_text_changed_at(&mut self, tab: usize) {
        self.mark_text_changed_at_with(tab, true);
    }

    pub fn mark_text_changed_at_with(&mut self, tab: usize, note_marks: bool) {
        let snap = self
            .pending_edit_snap
            .take()
            .filter(|(t, _)| *t == tab)
            .map(|(_, s)| s);
        let Some(doc) = self.tabs.get_mut(tab) else {
            return;
        };
        // Prefer exact buffer line-structure hook over snap heuristic.
        if !doc.consume_line_structure_edit() {
            if let Some(snap) = snap {
                doc.apply_line_snap(snap);
            } else {
                doc.sync_line_marks_after_edit();
            }
        }
        if note_marks {
            Self::note_edit_lines(doc);
        }
        doc.clamp_line_marks_to_buffer();
        doc.sync_dirty_from_revision();
        if doc.tail_follow {
            doc.tail_follow = false;
            self.status = "Tail OFF — editing suspended follow".into();
        }
        if tab == self.tabs.active_index() {
            self.highlight_dirty = true;
        }
        if self.highlight_tab_other == Some(tab) {
            self.highlight_dirty_other = true;
        }
        self.compare_stale = true;
    }

    /// Mark caret / selection lines as change-history (after an edit).
    fn note_edit_lines(doc: &mut doc::Document) {
        let buf = &doc.buffer;
        if let Some((s, e)) = buf.selection() {
            let lo = s.min(e);
            let hi = s.max(e);
            let end = if hi > lo { hi - 1 } else { lo };
            let ls = buf.char_to_line(lo);
            let le = buf.char_to_line(end);
            for line in ls..=le {
                doc.note_line_changed(line);
            }
        } else {
            let line = buf.char_to_line(buf.caret());
            doc.note_line_changed(line);
        }
    }

    /// Recompute syntax spans for a window around `view_first_line` (display line).
    ///
    /// Small files: whole buffer. Large files: capped byte window near the viewport.
    /// Scroll outside the covered inner range triggers a new pass without an edit.
    pub fn refresh_highlight_if_needed(&mut self, view_first_line: usize) {
        let tab = self.tabs.active_index();
        if let Some((spans, lang, cover)) = self.build_highlight_window(
            tab,
            view_first_line,
            self.highlight_dirty,
            self.highlight_cover,
        ) {
            self.highlight_cache = spans;
            self.highlight_lang = lang;
            self.highlight_cover = cover;
            self.highlight_dirty = false;
        }
    }

    /// Recompute syntax spans for the dual-view other pane tab around its scroll line.
    pub fn refresh_highlight_other_if_needed(&mut self, tab: usize, view_first_line: usize) {
        if self.tabs.get(tab).is_none() {
            self.highlight_cache_other.clear();
            self.highlight_lang_other.clear();
            self.highlight_cover_other = (0, 0);
            self.highlight_tab_other = None;
            self.highlight_dirty_other = true;
            return;
        }
        let dirty = self.highlight_dirty_other || self.highlight_tab_other != Some(tab);
        if let Some((spans, lang, cover)) =
            self.build_highlight_window(tab, view_first_line, dirty, self.highlight_cover_other)
        {
            self.highlight_cache_other = spans;
            self.highlight_lang_other = lang;
            self.highlight_cover_other = cover;
            self.highlight_tab_other = Some(tab);
            self.highlight_dirty_other = false;
        }
    }

    /// Remap the other-pane highlight tab after a tab drag-reorder.
    pub fn remap_highlight_tab_other(&mut self, from: usize, to: usize) {
        if let Some(t) = self.highlight_tab_other {
            let new_t = doc::TabSet::remap_index(t, from, to);
            if new_t != t {
                self.highlight_tab_other = Some(new_t);
                self.highlight_dirty_other = true;
            }
        }
    }

    /// Drop other-pane highlight when its tab is closed (`closed` = pre-close index).
    pub fn on_tab_closed_highlight_other(&mut self, closed: usize) {
        match self.highlight_tab_other {
            Some(t) if t == closed => {
                self.highlight_cache_other.clear();
                self.highlight_lang_other.clear();
                self.highlight_cover_other = (0, 0);
                self.highlight_tab_other = None;
                self.highlight_dirty_other = true;
            }
            Some(t) if t > closed => {
                self.highlight_tab_other = Some(t - 1);
                self.highlight_dirty_other = true;
            }
            _ => {}
        }
    }

    /// Build a viewport highlight window for `tab`, or `None` when the existing cover is fine.
    fn build_highlight_window(
        &mut self,
        tab: usize,
        view_first_line: usize,
        dirty: bool,
        cover: (usize, usize),
    ) -> Option<(Vec<highlight::Span>, String, (usize, usize))> {
        const WINDOW_BYTES: usize = 512 * 1024;
        const MARGIN_LINES: usize = 64;
        const AHEAD_LINES: usize = 96;

        let (lang, line_count, start_char, start_line, end_line, slice) = {
            let doc = self.tabs.get(tab)?;
            let line_count = doc.buffer.line_count().max(1);
            let view = view_first_line.min(line_count.saturating_sub(1));

            if !dirty {
                let (lo, hi) = cover;
                if lo == 0 && hi >= line_count {
                    return None;
                }
                let inner_lo = lo.saturating_add(MARGIN_LINES / 2);
                let inner_hi = hi.saturating_sub(MARGIN_LINES / 2).max(inner_lo);
                if view >= inner_lo && view < inner_hi {
                    return None;
                }
            }

            let lang = doc.language.clone();
            let start_line = view.saturating_sub(MARGIN_LINES);
            let min_end = (view + AHEAD_LINES).min(line_count);
            let buf = &doc.buffer;
            let start_char = buf.line_to_char(start_line);
            let mut end_line = start_line;
            let mut nbytes = 0usize;
            while end_line < line_count {
                let add = buf.line(end_line).len();
                if end_line >= min_end && nbytes + add > WINDOW_BYTES && end_line > start_line {
                    break;
                }
                nbytes += add;
                end_line += 1;
                if nbytes >= WINDOW_BYTES && end_line >= min_end {
                    break;
                }
            }
            let end_char = if end_line >= line_count {
                buf.len_chars()
            } else {
                buf.line_to_char(end_line)
            };
            let slice = buf.slice(start_char, end_char);
            (lang, line_count, start_char, start_line, end_line, slice)
        };

        let mut spans = self
            .highlighter
            .highlight(&lang, &slice)
            .unwrap_or_default();
        if start_char > 0 {
            for s in &mut spans {
                s.start += start_char;
                s.end += start_char;
            }
        }
        Some((spans, lang, (start_line, end_line.min(line_count))))
    }

    pub fn new_file(&mut self) {
        self.tabs.open_untitled();
        self.highlight_dirty = true;
        self.reset_view = true;
        self.status = "New file".into();
    }

    /// Open `logs/*.log` relative to the process working directory (e.g. `logs/panic.log`).
    pub fn open_npp_logs(&mut self) {
        let dir = PathBuf::from("logs");
        let mut paths: Vec<PathBuf> = Vec::new();
        let panic = dir.join("panic.log");
        if panic.is_file() {
            paths.push(panic);
        }
        if let Ok(rd) = std::fs::read_dir(&dir) {
            for entry in rd.flatten() {
                let p = entry.path();
                if !p.is_file() {
                    continue;
                }
                let is_log = p
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| e.eq_ignore_ascii_case("log"));
                if is_log && !paths.iter().any(|x| x == &p) {
                    paths.push(p);
                }
            }
        }
        paths.sort();
        if paths.is_empty() {
            self.status =
                "No logs under logs/ (cwd). Panic hook writes logs/panic.log when present.".into();
            return;
        }
        let n = paths.len();
        for p in paths {
            self.open_path(p);
        }
        self.status = format!("Opened {n} log file(s) from logs/");
    }

    /// Open a read-only tab with build / runtime facts (Help → Debug Info).
    pub fn show_debug_info(&mut self) {
        let panic_log = PathBuf::from(crate::recent::PANIC_LOG_REL);
        let panic_state = if panic_log.is_file() {
            "present"
        } else {
            "not found"
        };
        let pref = match self.settings.log_tail_on_open {
            LogTailOnOpen::Ask => "ask",
            LogTailOnOpen::Always => "always",
            LogTailOnOpen::Never => "never",
        };
        let text = format!(
            "npp-rust debug info\n\
             ==================\n\
             \n\
             version: {}\n\
             os: {}\n\
             arch: {}\n\
             \n\
             Working directory: process cwd (paths below are relative to it).\n\
             Panic log: {} ({panic_state})\n\
             Settings: {} (log_tail_on_open = {pref})\n\
             \n\
             What this command does\n\
             ----------------------\n\
             Debug Info opens this tab. It shows build and runtime facts.\n\
             It does not send data anywhere.\n\
             \n\
             Logs\n\
             ----\n\
             Use ? → Open npp-rust Logs to load logs/*.log from the cwd.\n\
             The panic hook appends to logs/panic.log when the process panics.\n\
             Opening any *.log file offers a Monitoring (tail) prompt.\n\
             Remember stores ask/always/never in {}.\n\
             View → Monitoring or Ctrl/Cmd+Shift+T toggles tail follow.\n\
             To ask again: delete npp-rs/settings.json or set log_tail_on_open to ask.\n",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS,
            std::env::consts::ARCH,
            crate::recent::PANIC_LOG_REL,
            crate::recent::SETTINGS_REL,
            crate::recent::SETTINGS_REL,
        );
        self.tabs.open_untitled();
        {
            let doc = self.tabs.active_mut();
            doc.title = "Debug Info".into();
            doc.buffer = buffer::TextBuffer::from_str(&text);
            doc.dirty = false;
            doc.language = "plain".into();
            doc.read_only = true;
        }
        self.highlight_dirty = true;
        self.reset_view = true;
        self.status = "Debug Info opened (? → Debug Info)".into();
    }

    pub fn open_path(&mut self, path: PathBuf) {
        if !path.exists() {
            self.recent.remove(&path);
            self.status = format!(
                "File not found (removed from Recent): {}",
                short_path_label(&path)
            );
            return;
        }
        match fs::file_size(&path) {
            Ok(size) if size >= LARGE_FILE_THRESHOLD => {
                let id = self.tabs.alloc_id();
                let mut doc = Document::from_path(id, path.clone(), String::new());
                doc.loading = true;
                doc.title = format!("{} (loading…)", doc.title);
                self.tabs.open_document(doc);
                self.pending.push(PendingLoad {
                    document_id: id,
                    path: path.clone(),
                });
                self.touch_recent(&path);
                fs::open_async(path.clone(), self.load_channel.tx.clone());
                self.status = format!(
                    "Loading large file in background ({:.1} MiB)…",
                    size as f64 / (1024.0 * 1024.0)
                );
                self.highlight_dirty = true;
            }
            Ok(_) => match fs::read_file(&path) {
                Ok(result) => self.apply_open_result(result),
                Err(e) => {
                    self.recent.remove(&path);
                    self.status = format!("Open failed: {e}");
                }
            },
            Err(e) => {
                self.recent.remove(&path);
                self.status = format!("Open failed: {e}");
            }
        }
    }

    pub fn apply_open_result(&mut self, result: OpenResult) {
        let pending = self.pending.iter().find(|p| p.path == result.path).cloned();
        if let Some(ref p) = pending {
            self.pending.retain(|x| x.document_id != p.document_id);
        } else {
            self.pending.retain(|p| p.path != result.path);
        }

        if let Some(pend) = pending {
            // Apply only if that document id still exists and is still the
            // loading placeholder for this path; else drop the result.
            let ok = self
                .tabs
                .get_by_id(pend.document_id)
                .is_some_and(|d| d.loading && d.path.as_ref() == Some(&result.path));
            if !ok {
                self.status = format!(
                    "Load finished but tab was closed: {}",
                    short_path_label(&result.path)
                );
                return;
            }
            let keep_id = pend.document_id;
            if let Some(doc) = self.tabs.get_mut_by_id(keep_id) {
                *doc = Document::from_path_with_encoding(
                    keep_id,
                    result.path.clone(),
                    result.content,
                    file_encoding_from_fs(result.encoding),
                );
            }
            if let Some(tab_index) = self.tabs.index_of_id(keep_id) {
                self.tabs.set_active(tab_index);
            }
        } else if self.cancelled_loads.remove(&result.path) {
            self.status = format!(
                "Load finished but tab was closed: {}",
                short_path_label(&result.path)
            );
            return;
        } else {
            let id = self.tabs.alloc_id();
            let doc = Document::from_path_with_encoding(
                id,
                result.path.clone(),
                result.content,
                file_encoding_from_fs(result.encoding),
            );
            self.tabs.open_document(doc);
        }
        self.touch_recent(&result.path);
        self.highlight_dirty = true;
        self.reset_view = true;
        // Ensure caret is at the top after open.
        self.tabs.active_mut().buffer.set_caret(0);
        self.tabs.active_mut().tail_bytes = result.bytes;
        self.tabs.active_mut().tail_follow = false;
        let name = short_path_label(&result.path);
        self.status = match &result.encoding_note {
            Some(note) => format!(
                "Opened {name} ({:.1} KiB, {} ms, {}) — {note}",
                result.bytes as f64 / 1024.0,
                result.elapsed_ms,
                result.encoding.label()
            ),
            None => format!(
                "Opened {name} ({:.1} KiB, {} ms, {})",
                result.bytes as f64 / 1024.0,
                result.elapsed_ms,
                result.encoding.label()
            ),
        };
        if let Some(note) = &result.encoding_note {
            self.pending_encoding_notice = Some(format!("{name}: {note}"));
        }
        self.apply_pending_session_folds_for(&result.path);
        self.apply_pending_session_bookmarks_for(&result.path);
        self.apply_pending_session_styles_for(&result.path);
        self.apply_pending_session_caret_for(&result.path);
        self.apply_pending_session_sel_for(&result.path);
        self.apply_pending_session_scroll_for(&result.path);
        self.apply_pending_session_lang_for(&result.path);
        self.apply_pending_session_encoding_for(&result.path);
        self.apply_pending_session_readonly_for(&result.path);
        self.apply_pending_session_tail_for(&result.path);
        self.apply_pending_session_active_for(&result.path);
        self.after_open_log_policy(&result.path);
    }

    /// Apply remembered log-tail preference, or queue the prompt dialog.
    fn after_open_log_policy(&mut self, path: &std::path::Path) {
        self.pending_log_tail_prompt = false;
        if !is_log_path(path) {
            return;
        }
        // Session `@tail` (or an earlier Always policy) already armed follow.
        if self.tabs.active().path.as_deref() == Some(path) && self.tabs.active().tail_follow {
            return;
        }
        match self.settings.log_tail_on_open {
            LogTailOnOpen::Always => {
                let _ = self.enable_tail_follow();
            }
            LogTailOnOpen::Never => {}
            LogTailOnOpen::Ask => {
                self.pending_log_tail_prompt = true;
            }
        }
    }

    /// Persist log-open preference and optionally enable tail now.
    pub fn resolve_log_tail_prompt(&mut self, enable: bool, remember: bool) {
        self.pending_log_tail_prompt = false;
        if remember {
            self.settings.log_tail_on_open = if enable {
                LogTailOnOpen::Always
            } else {
                LogTailOnOpen::Never
            };
            self.settings.save();
            let pref = match self.settings.log_tail_on_open {
                LogTailOnOpen::Always => "always",
                LogTailOnOpen::Never => "never",
                LogTailOnOpen::Ask => "ask",
            };
            self.status = format!(
                "Remembered log tail = {pref} ({})",
                crate::recent::SETTINGS_REL
            );
        }
        if enable {
            let _ = self.enable_tail_follow();
        } else if !remember {
            let name = self
                .tabs
                .active()
                .path
                .as_ref()
                .map(|p| short_path_label(p))
                .unwrap_or_else(|| "*.log".into());
            self.status = format!("Opened {name} without tail");
        }
    }

    /// Clear remembered preference so the next `*.log` open asks again.
    pub fn reset_log_tail_preference(&mut self) {
        self.settings.log_tail_on_open = LogTailOnOpen::Ask;
        self.settings.save();
        self.status = format!(
            "Log tail preference reset to ask ({})",
            crate::recent::SETTINGS_REL
        );
    }

    /// Drop pending load for a closed tab (by document id; path is a backup key).
    pub fn note_tab_closed(&mut self, document_id: DocumentId, path: Option<&std::path::Path>) {
        let mut cancelled = Vec::new();
        self.pending.retain(|p| {
            let drop = p.document_id == document_id || path.is_some_and(|path| p.path == path);
            if drop {
                cancelled.push(p.path.clone());
                false
            } else {
                true
            }
        });
        for p in cancelled {
            self.pending_session_folds.remove(&p);
            self.pending_session_bookmarks.remove(&p);
            self.pending_session_styles.remove(&p);
            self.pending_session_carets.remove(&p);
            self.pending_session_sels.remove(&p);
            self.pending_session_langs.remove(&p);
            self.pending_session_encodings.remove(&p);
            self.pending_session_readonly.remove(&p);
            self.pending_session_tail.remove(&p);
            self.pending_session_scrolls.remove(&p);
            if self.pending_session_active.as_ref() == Some(&p) {
                self.pending_session_active = None;
                self.pending_session_scroll = None;
            }
            self.cancelled_loads.insert(p);
        }
    }

    /// Close a tab and cancel any pending load for its id/path.
    pub fn close_tab(&mut self, index: usize) {
        let (id, path) = self
            .tabs
            .get(index)
            .map(|d| (d.id, d.path.clone()))
            .unwrap_or((0, None));
        self.note_tab_closed(id, path.as_deref());
        if index < self.tabs.len() {
            self.closed_tab_indices.push(index);
        }
        self.tabs.close(index);
        self.highlight_dirty = true;
        if let Some(p) = self.pending_close {
            if p == index {
                self.pending_close = None;
            } else if p > index {
                self.pending_close = Some(p - 1);
            }
        }
        if let Some(p) = self.pending_reload {
            if p == index {
                self.pending_reload = None;
            } else if p > index {
                self.pending_reload = Some(p - 1);
            }
        }
    }

    /// Close tab, or open the unsaved-changes prompt when dirty.
    pub fn request_close_tab(&mut self, index: usize) {
        if index >= self.tabs.len() {
            return;
        }
        if self.tabs.get(index).map(|d| d.dirty).unwrap_or(false) {
            self.tabs.set_active(index);
            self.pending_close = Some(index);
            self.status = "Document has unsaved changes".into();
            return;
        }
        self.close_tab(index);
        self.continue_bulk_close();
    }

    pub fn confirm_close_save(&mut self) -> bool {
        let Some(index) = self.pending_close else {
            return false;
        };
        if index >= self.tabs.len() {
            self.pending_close = None;
            return false;
        }
        self.tabs.set_active(index);
        if !self.save() {
            return false;
        }
        self.pending_close = None;
        self.close_tab(index);
        self.continue_bulk_close();
        true
    }

    pub fn confirm_close_discard(&mut self) {
        let Some(index) = self.pending_close.take() else {
            return;
        };
        if index < self.tabs.len() {
            self.close_tab(index);
        }
        self.continue_bulk_close();
    }

    /// True when quit was requested and all dirty tabs are resolved.
    pub fn take_want_quit(&mut self) -> bool {
        if self.want_quit {
            self.want_quit = false;
            true
        } else {
            false
        }
    }

    pub fn confirm_close_cancel(&mut self) {
        self.pending_close = None;
        self.bulk_close = BulkClose::None;
        self.status = "Close cancelled".into();
    }

    /// File → Reload from Disk. Confirms when dirty (Save / Don't Save / Cancel).
    pub fn request_reload(&mut self) {
        if self.tabs.active().path.is_none() {
            self.status = "Reload: untitled buffer".into();
            return;
        }
        if self.tabs.active().dirty {
            self.pending_reload = Some(self.tabs.active_index());
            self.status = "Reload: document has unsaved changes".into();
            return;
        }
        self.reload_from_disk();
    }

    /// Replace active tab contents from disk. Resets undo and clears dirty.
    pub fn reload_from_disk(&mut self) {
        let Some(path) = self.tabs.active().path.clone() else {
            self.status = "Reload: untitled buffer".into();
            return;
        };
        if !path.exists() {
            self.status = format!(
                "Reload failed: file not found ({})",
                short_path_label(&path)
            );
            return;
        }
        match fs::read_file(&path) {
            Ok(result) => {
                let doc = self.tabs.active_mut();
                doc.buffer = buffer::TextBuffer::from_str(&result.content);
                doc.encoding = file_encoding_from_fs(result.encoding);
                doc.language = doc::detect_language(&path);
                doc.line_mark_basis = doc.buffer.line_count();
                doc.clear_change_history();
                doc.loading = false;
                doc.tail_bytes = result.bytes;
                doc.tail_follow = false;
                doc.buffer.set_caret(0);
                doc.mark_clean();
                self.pending_reload = None;
                self.highlight_dirty = true;
                self.reset_view = true;
                let name = short_path_label(&path);
                self.status = format!("Reloaded {name} ({:.1} KiB)", result.bytes as f64 / 1024.0);
            }
            Err(e) => {
                self.status = format!("Reload failed: {e}");
            }
        }
    }

    pub fn confirm_reload_save(&mut self) -> bool {
        let Some(index) = self.pending_reload else {
            return false;
        };
        if index >= self.tabs.len() {
            self.pending_reload = None;
            return false;
        }
        self.tabs.set_active(index);
        if !self.save() {
            return false;
        }
        self.pending_reload = None;
        self.reload_from_disk();
        true
    }

    pub fn confirm_reload_discard(&mut self) {
        let Some(index) = self.pending_reload.take() else {
            return;
        };
        if index < self.tabs.len() {
            self.tabs.set_active(index);
            self.reload_from_disk();
        }
    }

    pub fn confirm_reload_cancel(&mut self) {
        self.pending_reload = None;
        self.status = "Reload cancelled".into();
    }

    pub fn request_quit(&mut self, ui: &mut crate::commands::UiFlags) {
        self.want_quit = false;
        self.bulk_close = BulkClose::Quit;
        self.continue_bulk_close();
        if self.want_quit {
            ui.request_quit = true;
            self.want_quit = false;
        }
    }

    pub fn start_bulk_close(&mut self, mode: BulkClose) {
        self.bulk_close = mode;
        self.continue_bulk_close();
    }

    fn any_dirty(&self) -> bool {
        self.tabs.iter().any(|d| d.dirty)
    }

    fn continue_bulk_close(&mut self) {
        loop {
            if self.pending_close.is_some() {
                return;
            }
            match self.bulk_close {
                BulkClose::None => return,
                BulkClose::All => {
                    // Closing the last tab recreates untitled — stop when one clean untitled remains.
                    if self.tabs.len() == 1
                        && self.tabs.active().path.is_none()
                        && !self.tabs.active().dirty
                    {
                        self.bulk_close = BulkClose::None;
                        return;
                    }
                    let dirty = self.tabs.get(0).map(|d| d.dirty).unwrap_or(false);
                    if dirty {
                        self.tabs.set_active(0);
                        self.pending_close = Some(0);
                        self.status = "Document has unsaved changes".into();
                        return;
                    }
                    self.close_tab(0);
                }
                BulkClose::AllButCurrent => {
                    if self.tabs.len() <= 1 {
                        self.bulk_close = BulkClose::None;
                        return;
                    }
                    let keep = self.tabs.active_index();
                    let idx = if keep == 0 { 1 } else { 0 };
                    let dirty = self.tabs.get(idx).map(|d| d.dirty).unwrap_or(false);
                    if dirty {
                        self.tabs.set_active(idx);
                        self.pending_close = Some(idx);
                        self.status = "Document has unsaved changes".into();
                        return;
                    }
                    self.close_tab(idx);
                }
                BulkClose::AllButPinned => {
                    let mut found = None;
                    for i in 0..self.tabs.len() {
                        if !self.tabs.get(i).map(|d| d.pinned).unwrap_or(false) {
                            found = Some(i);
                            break;
                        }
                    }
                    match found {
                        Some(idx) => {
                            let dirty = self.tabs.get(idx).map(|d| d.dirty).unwrap_or(false);
                            if dirty {
                                self.tabs.set_active(idx);
                                self.pending_close = Some(idx);
                                self.status = "Document has unsaved changes".into();
                                return;
                            }
                            self.close_tab(idx);
                        }
                        None => {
                            self.bulk_close = BulkClose::None;
                            return;
                        }
                    }
                }
                BulkClose::AllToLeft => {
                    let keep = self.tabs.active_index();
                    if keep == 0 {
                        self.bulk_close = BulkClose::None;
                        return;
                    }
                    let dirty = self.tabs.get(0).map(|d| d.dirty).unwrap_or(false);
                    if dirty {
                        self.tabs.set_active(0);
                        self.pending_close = Some(0);
                        self.status = "Document has unsaved changes".into();
                        return;
                    }
                    self.close_tab(0);
                }
                BulkClose::AllToRight => {
                    let keep = self.tabs.active_index();
                    if keep + 1 >= self.tabs.len() {
                        self.bulk_close = BulkClose::None;
                        return;
                    }
                    let idx = keep + 1;
                    let dirty = self.tabs.get(idx).map(|d| d.dirty).unwrap_or(false);
                    if dirty {
                        self.tabs.set_active(idx);
                        self.pending_close = Some(idx);
                        self.status = "Document has unsaved changes".into();
                        return;
                    }
                    self.close_tab(idx);
                }
                BulkClose::AllUnchanged => {
                    let mut found = None;
                    for i in 0..self.tabs.len() {
                        if !self.tabs.get(i).map(|d| d.dirty).unwrap_or(true) {
                            found = Some(i);
                            break;
                        }
                    }
                    match found {
                        Some(i) => self.close_tab(i),
                        None => {
                            self.bulk_close = BulkClose::None;
                            return;
                        }
                    }
                }
                BulkClose::Quit => {
                    if !self.any_dirty() {
                        self.bulk_close = BulkClose::None;
                        self.want_quit = true;
                        return;
                    }
                    let Some(i) = (0..self.tabs.len())
                        .rev()
                        .find(|&i| self.tabs.get(i).map(|d| d.dirty).unwrap_or(false))
                    else {
                        self.bulk_close = BulkClose::None;
                        self.want_quit = true;
                        return;
                    };
                    self.tabs.set_active(i);
                    self.pending_close = Some(i);
                    self.status = "Document has unsaved changes".into();
                    return;
                }
            }
        }
    }

    pub fn close_tab_title(&self) -> String {
        let Some(i) = self.pending_close else {
            return String::new();
        };
        self.tabs
            .get(i)
            .map(|d| d.title.clone())
            .unwrap_or_default()
    }

    pub fn poll_loads(&mut self) {
        while let Ok(msg) = self.load_channel.rx.try_recv() {
            match msg {
                LoadMsg::Done(result) => self.apply_open_result(result),
                LoadMsg::Failed { path, error } => {
                    let ids: Vec<DocumentId> = self
                        .pending
                        .iter()
                        .filter(|p| p.path == path)
                        .map(|p| p.document_id)
                        .collect();
                    self.pending.retain(|p| p.path != path);
                    self.cancelled_loads.remove(&path);
                    self.recent.remove(&path);
                    let to_close: Vec<usize> = ids
                        .into_iter()
                        .filter_map(|id| {
                            self.tabs.index_of_id(id).filter(|&i| {
                                self.tabs
                                    .get(i)
                                    .is_some_and(|d| d.loading && d.path.as_ref() == Some(&path))
                            })
                        })
                        .collect();
                    for i in to_close.into_iter().rev() {
                        self.tabs.close(i);
                    }
                    self.status = format!("Open failed ({}): {error}", path.display());
                }
            }
        }
    }

    pub fn save(&mut self) -> bool {
        let path = self.tabs.active().path.clone();
        match path {
            Some(path) => self.save_to(&path),
            None => self.save_as_dialog(),
        }
    }

    pub fn save_as_dialog(&mut self) -> bool {
        let path = rfd::FileDialog::new()
            .set_file_name(self.tabs.active().title.as_str())
            .save_file();
        if let Some(path) = path {
            self.save_to(&path)
        } else {
            false
        }
    }

    fn save_to(&mut self, path: &std::path::Path) -> bool {
        self.save_tab_to(self.tabs.active_index(), path, true)
    }

    /// Write tab `tab` to `path`. When `prompt_lossy` is false, skip ANSI lossy confirm
    /// (used by autosave — those tabs stay dirty until the user confirms).
    fn save_tab_to(&mut self, tab: usize, path: &std::path::Path, prompt_lossy: bool) -> bool {
        let Some(doc) = self.tabs.get(tab) else {
            return false;
        };
        let content = doc.buffer.to_string();
        let encoding = doc.encoding;
        if encoding == FileEncoding::Windows1252 {
            let unmapped = fs::count_windows_1252_unmapped(&content);
            if unmapped > 0 {
                if !prompt_lossy {
                    return false;
                }
                if self.pending_lossy_ansi.as_deref() != Some(path) {
                    self.pending_lossy_ansi = Some(path.to_path_buf());
                    self.status =
                        format!("ANSI save: {unmapped} char(s) become '?' — confirm in dialog");
                    return false;
                }
            }
        }
        self.pending_lossy_ansi = None;

        let mut backup_note = String::new();
        if self.settings.backup_on_save {
            match crate::backup::backup_existing_file(path) {
                Ok(true) => {
                    backup_note = format!(" · backup ({})", crate::backup::BACKUP_REL);
                }
                Ok(false) => {}
                Err(e) => {
                    backup_note = format!(" · backup failed ({e})");
                }
            }
        }

        let fs_enc = fs_encoding_from_file(encoding);
        match fs::write_file_with_encoding(path, &content, fs_enc) {
            Ok(()) => {
                let Some(doc) = self.tabs.get_mut(tab) else {
                    return false;
                };
                doc.path = Some(path.to_path_buf());
                doc.title = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                doc.language = doc::detect_language(path);
                doc.mark_clean();
                doc.promote_change_history_on_save();
                self.touch_recent(path);
                if tab == self.tabs.active_index() {
                    self.highlight_dirty = true;
                }
                let unmapped = if encoding == FileEncoding::Windows1252 {
                    fs::count_windows_1252_unmapped(&content)
                } else {
                    0
                };
                let name = short_path_label(path);
                self.status = if unmapped > 0 {
                    format!(
                        "Saved {name} ({}) — {unmapped} char(s) became '?'{backup_note}",
                        encoding.label()
                    )
                } else {
                    format!("Saved {name} ({}){backup_note}", encoding.label())
                };
                true
            }
            Err(e) => {
                self.status = format!("Save failed: {e}");
                false
            }
        }
    }

    /// Autosave dirty tabs that already have a path. Skips untitled and ANSI-lossy prompts.
    /// Returns true when a pass ran (interval elapsed), even if zero files were written.
    pub fn tick_autosave(&mut self) -> bool {
        let secs = self.settings.autosave_secs();
        if secs == 0 {
            return false;
        }
        if self.autosave_last.elapsed() < Duration::from_secs(secs) {
            return false;
        }
        self.autosave_last = Instant::now();
        let active = self.tabs.active_index();
        let mut saved = 0usize;
        let count = self.tabs.len();
        for i in 0..count {
            let Some(path) = self
                .tabs
                .get(i)
                .and_then(|d| if d.dirty { d.path.clone() } else { None })
            else {
                continue;
            };
            if self.save_tab_to(i, &path, false) {
                saved += 1;
            }
        }
        if self.tabs.active_index() != active {
            self.tabs.set_active(active);
        }
        if saved > 0 {
            self.status = format!("Autosave: {saved} file(s)");
        }
        true
    }

    pub fn confirm_lossy_ansi_save(&mut self) {
        if let Some(path) = self.pending_lossy_ansi.clone() {
            let _ = self.save_to(&path);
        }
    }

    pub fn cancel_lossy_ansi_save(&mut self) {
        self.pending_lossy_ansi = None;
        self.status = "ANSI save cancelled".into();
    }

    /// Persist workspace root into settings.json.
    pub fn persist_workspace_root(&mut self) {
        self.settings.workspace_root = self.workspace_root.display().to_string();
        self.settings.save();
    }

    pub fn open_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.open_path(path);
        }
    }

    pub fn clear_recent(&mut self) {
        self.recent.clear();
        self.status = "Recent files cleared".into();
    }

    /// Apply plugin transform to whole document (or selection if present for case plugins).
    pub fn run_plugin(&mut self, plugin_id: &str) {
        if !self.ensure_editable() {
            return;
        }
        let host = plugins::PluginHost::new();
        let Some(plugin) = host.get(plugin_id) else {
            self.status = format!("Unknown plugin: {plugin_id}");
            return;
        };
        let lang = self.tabs.active().language.clone();
        let selection = self.tabs.active().buffer.selection();
        let (src, sel_range) = if matches!(plugin_id, "edit.uppercase" | "edit.lowercase") {
            if let Some((s, e)) = selection {
                (self.tabs.active().buffer.slice(s, e), Some((s, e)))
            } else {
                (self.tabs.active().buffer.to_string(), None)
            }
        } else {
            (self.tabs.active().buffer.to_string(), None)
        };
        let Some(out) = plugin.run(&lang, &src) else {
            self.status = format!("{}: no change", plugin.name());
            return;
        };
        if out == src {
            self.status = format!("{}: already formatted", plugin.name());
            return;
        }
        if let Some((s, e)) = sel_range {
            let buf = self.tabs.active_mut();
            buf.buffer.set_selection(s, e);
            buf.buffer.insert(&out); // replaces selection
        } else {
            let caret = self.tabs.active().buffer.caret();
            self.tabs.active_mut().buffer.replace_document(&out);
            self.tabs
                .active_mut()
                .buffer
                .set_caret(caret.min(out.chars().count()));
            let n = self.tabs.active().buffer.line_count();
            for line in 0..n {
                self.tabs.active_mut().note_line_changed(line);
            }
        }
        self.mark_text_changed();
        self.status = format!("{} applied", plugin.name());
    }

    pub fn format_document(&mut self) {
        self.run_plugin("format.document");
    }

    /// Ask local Ollama about the selection (or whole buffer). Runs in the background;
    /// reply opens in a tab when ready (UI stays responsive).
    pub fn ask_ollama(&mut self) {
        if self.ollama_busy {
            self.status = "Ask Ollama: already running in background…".into();
            return;
        }
        let lang = self.tabs.active().language.clone();
        let (body, used_selection) = if let Some((s, e)) = self.tabs.active().buffer.selection() {
            let slice = self.tabs.active().buffer.slice(s, e);
            if slice.trim().is_empty() {
                (self.tabs.active().buffer.to_string(), false)
            } else {
                (slice, true)
            }
        } else {
            (self.tabs.active().buffer.to_string(), false)
        };
        if body.trim().is_empty() {
            self.status = "Ask Ollama: buffer is empty".into();
            return;
        }
        // Validate host/model on the UI thread so bad prefs fail instantly.
        if let Err(e) = crate::ollama::parse_loopback_host(&self.settings.ollama_host) {
            self.status = format!("Ask Ollama: {e}");
            return;
        }
        if let Err(e) = crate::ollama::effective_model(&self.settings.ollama_model) {
            self.status = format!("Ask Ollama: {e}");
            return;
        }
        let (truncated, cut) =
            crate::ollama::truncate_prompt(&body, crate::ollama::MAX_PROMPT_CHARS);
        let prompt = crate::ollama::build_ask_prompt(&lang, &truncated, cut);
        let scope = match (used_selection, cut) {
            (true, true) => "selection (truncated)",
            (true, false) => "selection",
            (false, true) => "whole buffer (truncated)",
            (false, false) => "whole buffer",
        }
        .to_string();
        self.status = format!(
            "Ask Ollama: calling {} ({}) in background…",
            self.settings.ollama_model, self.settings.ollama_host
        );
        self.ollama_busy = true;
        crate::ollama::generate_async(
            self.settings.ollama_host.clone(),
            self.settings.ollama_model.clone(),
            prompt,
            scope,
            cut,
            self.ollama_channel.tx.clone(),
        );
    }

    /// Ping loopback Ollama and list installed models in a tab (background).
    pub fn ollama_status(&mut self) {
        if self.ollama_busy {
            self.status = "Ollama: already running in background…".into();
            return;
        }
        if let Err(e) = crate::ollama::parse_loopback_host(&self.settings.ollama_host) {
            self.status = format!("Ollama status: {e}");
            return;
        }
        let host = self.settings.ollama_host.clone();
        let preferred = self.settings.ollama_model.trim().to_string();
        self.status = format!("Ollama status: pinging {} in background…", host.trim());
        self.ollama_busy = true;
        crate::ollama::list_models_async(host, preferred, self.ollama_channel.tx.clone());
    }

    /// Apply finished background Ollama results. Returns true if anything arrived.
    pub fn poll_ollama(&mut self) -> bool {
        let mut got = false;
        while let Ok(msg) = self.ollama_channel.rx.try_recv() {
            got = true;
            self.ollama_busy = false;
            match msg {
                crate::ollama::OllamaMsg::AskDone {
                    model,
                    host,
                    scope,
                    cut,
                    reply,
                } => {
                    let mut text = format!(
                        "Ollama reply ({model})\n\
                         =====================\n\
                         Host: {host}\n\
                         Scope: {scope}\n\
                         \n\
                         {reply}\n"
                    );
                    if cut {
                        text.push_str("\n(Note: input was truncated before send.)\n");
                    }
                    self.open_ollama_tab("Ollama reply", &text);
                    self.status = format!("Ask Ollama: reply from {model}");
                }
                crate::ollama::OllamaMsg::AskFailed { error } => {
                    self.status = format!("Ask Ollama: {error}");
                }
                crate::ollama::OllamaMsg::StatusDone {
                    host,
                    preferred_model,
                    models,
                } => {
                    let mut text = format!(
                        "Ollama status\n\
                         =============\n\
                         Host: {host}\n\
                         Preferred model: {preferred_model}\n\
                         \n\
                         Installed models ({n})\n\
                         --------------------\n",
                        n = models.len(),
                    );
                    if models.is_empty() {
                        text.push_str("(none — run `ollama pull llama3.2` in a terminal)\n");
                    } else {
                        for m in &models {
                            text.push_str("- ");
                            text.push_str(m);
                            text.push('\n');
                        }
                    }
                    text.push_str(
                        "\n\
                         Notes\n\
                         -----\n\
                         - npp-rs talks to loopback only (127.0.0.1 / localhost / ::1).\n\
                         - Plugins → Ask Ollama (or editor right-click) sends locally.\n\
                         - Requests run in the background so the window stays responsive.\n\
                         - See docs/ollama-helper.md.\n",
                    );
                    self.open_ollama_tab("Ollama status", &text);
                    self.status = format!("Ollama: {} model(s) on {}", models.len(), host.trim());
                }
                crate::ollama::OllamaMsg::StatusFailed { error } => {
                    self.status = format!("Ollama status: {error}");
                }
            }
        }
        got
    }

    fn open_ollama_tab(&mut self, title: &str, text: &str) {
        self.tabs.open_untitled();
        {
            let doc = self.tabs.active_mut();
            doc.title = title.into();
            doc.buffer = buffer::TextBuffer::from_str(text);
            doc.dirty = false;
            doc.language = "markdown".into();
            doc.read_only = true;
        }
        self.highlight_dirty = true;
        self.reset_view = true;
    }

    fn find_flags(&self) -> crate::search_util::FindFlags {
        crate::search_util::FindFlags {
            match_case: self.settings.find_match_case,
            whole_word: self.settings.find_whole_word,
            use_regex: self.settings.find_regex,
        }
    }

    fn selection_is_query_match(&self) -> bool {
        let q = self.find_query.as_str();
        if q.is_empty() {
            return false;
        }
        let flags = self.find_flags();
        self.tabs
            .active()
            .buffer
            .selection()
            .map(|(s, e)| {
                let slice = self.tabs.active().buffer.slice(s, e);
                let hits = crate::search_util::find_all_matches(&slice, q, flags);
                hits.len() == 1 && hits[0] == (0, slice.chars().count())
            })
            .unwrap_or(false)
    }

    fn find_regex_blocked(&mut self) -> bool {
        if !self.settings.find_regex {
            return false;
        }
        if crate::search_util::find_regex_error(&self.find_query, self.settings.find_match_case)
            .is_some()
        {
            self.status = "Find: invalid regex".into();
            true
        } else {
            false
        }
    }

    /// Capture a search span from the current selection when In selection is on.
    pub fn capture_find_scope(&mut self) {
        if !self.settings.find_in_selection {
            self.find_scope = None;
            return;
        }
        let n = self.tabs.active().buffer.len_chars();
        if let Some((lo, hi)) = self.find_scope {
            if lo < hi && lo < n {
                self.find_scope = Some((lo, hi.min(n)));
                return;
            }
            self.find_scope = None;
        }
        if let Some((s, e)) = self.tabs.active().buffer.selection() {
            if e > s && !self.selection_is_query_match() {
                self.find_scope = Some((s, e));
            }
        }
    }

    fn find_bounds(&self) -> (usize, usize) {
        let n = self.tabs.active().buffer.len_chars();
        if self.settings.find_in_selection {
            if let Some((lo, hi)) = self.find_scope {
                let lo = lo.min(n);
                return (lo, hi.min(n).max(lo));
            }
            // Sel is on but no range yet: do not search the whole file.
            return (0, 0);
        }
        (0, n)
    }

    fn find_bound_label(&self) -> &'static str {
        if self.settings.find_in_selection {
            "selection"
        } else {
            "file"
        }
    }

    /// Open Find: a multi-line selection becomes the Sel range (query unchanged).
    /// A short single-line selection becomes the query.
    pub fn prepare_find_bar_from_selection(&mut self) {
        let Some((s, e)) = self.tabs.active().buffer.selection() else {
            return;
        };
        if e <= s {
            return;
        }
        let sel = self.tabs.active().buffer.slice(s, e);
        if sel.contains('\n') {
            self.settings.find_in_selection = true;
            self.find_scope = Some((s, e));
            return;
        }
        if sel.chars().count() <= 200 {
            self.find_query = sel;
        }
    }

    pub fn replace_next(&mut self, replacement: &str) {
        if !self.ensure_editable() {
            return;
        }
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Replace: empty find".into();
            return;
        }
        if self.find_regex_blocked() {
            return;
        }
        let selection_is_match = self.selection_is_query_match();
        if !selection_is_match {
            self.find_next();
            if !self.selection_is_query_match() {
                self.status = "Replace: no match".into();
                return;
            }
        }
        let (sel_lo, sel_hi) = self.tabs.active().buffer.selection().unwrap_or((0, 0));
        let match_len = sel_hi.saturating_sub(sel_lo);
        let text = self.tabs.active().buffer.to_string();
        let expanded = crate::search_util::expand_replacement(
            &text,
            &q,
            replacement,
            sel_lo,
            sel_hi,
            self.find_flags(),
        );
        self.tabs.active_mut().buffer.insert(&expanded);
        if let Some((lo, hi)) = self.find_scope {
            let delta = expanded.chars().count() as isize - match_len as isize;
            let new_hi = (hi as isize + delta).max(lo as isize) as usize;
            self.find_scope = Some((lo, new_hi));
        }
        self.mark_text_changed();
        self.status = "Replaced once".into();
    }

    pub fn replace_all(&mut self, replacement: &str) {
        if !self.ensure_editable() {
            return;
        }
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Replace All: empty find".into();
            return;
        }
        if self.find_regex_blocked() {
            return;
        }
        self.capture_find_scope();
        let flags = self.find_flags();
        let (lo, hi) = self.find_bounds();
        let text = self.tabs.active().buffer.to_string();
        let (new_text, count, new_hi) =
            crate::search_util::replace_all_in(&text, &q, replacement, lo, hi, flags);
        if count == 0 {
            self.status = "Replace All: no match".into();
            return;
        }
        let mut touched = std::collections::BTreeSet::new();
        for (s, _) in crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags) {
            touched.insert(self.tabs.active().buffer.char_to_line(s));
        }
        self.tabs.active_mut().buffer.replace_document(&new_text);
        if self.settings.find_in_selection {
            self.find_scope = Some((lo, new_hi));
        }
        for line in touched {
            self.tabs.active_mut().note_line_changed(line);
        }
        self.mark_text_changed();
        self.status = format!("Replace All: {count} replacement(s)");
    }

    pub fn set_language(&mut self, lang: &str) {
        self.tabs.active_mut().language = lang.to_string();
        self.highlight_dirty = true;
        if self.highlight_tab_other == Some(self.tabs.active_index()) {
            self.highlight_dirty_other = true;
        }
        self.status = format!("Language: {lang}");
    }

    pub fn find_match_count(&self) -> usize {
        let q = self.find_query.as_str();
        if q.is_empty() {
            return 0;
        }
        let text = self.tabs.active().buffer.to_string();
        let (lo, hi) = self.find_bounds();
        crate::search_util::find_all_matches_in(&text, q, lo, hi, self.find_flags()).len()
    }

    /// Style every line that has a match under `flags` in `[lo, hi)`. Clears that style slot first.
    /// Returns marked line count, or `None` when the style id is bad or regex is invalid.
    pub fn mark_style_matches(
        &mut self,
        style: u8,
        query: &str,
        flags: crate::search_util::FindFlags,
        lo: usize,
        hi: usize,
    ) -> Option<usize> {
        if !(1..=5).contains(&style) {
            self.status = "Bad style id".into();
            return None;
        }
        if query.is_empty() {
            self.status = "Style all: empty token".into();
            return Some(0);
        }
        if flags.use_regex
            && crate::search_util::find_regex_error(query, flags.match_case).is_some()
        {
            self.status = "Find: invalid regex".into();
            return None;
        }
        let text = self.tabs.active().buffer.to_string();
        let matches = crate::search_util::find_all_matches_in(&text, query, lo, hi, flags);
        let mut lines = std::collections::BTreeSet::new();
        {
            let buf = &self.tabs.active().buffer;
            for (start, _) in &matches {
                lines.insert(buf.char_to_line(*start));
            }
        }
        let count = lines.len();
        self.tabs.active_mut().style_marks[(style as usize) - 1] = lines;
        self.status = format!("Styled {count} line(s) with style {style}");
        Some(count)
    }

    /// Mark All Style *N*: selection token (literal + Case/Word, whole file) or Find query
    /// (Case/Word/Re + Sel bounds). `prefer_find_query` skips a non-empty selection (Find bar Mark).
    pub fn mark_all_style(&mut self, style: u8, prefer_find_query: bool) {
        let from_sel = if prefer_find_query {
            None
        } else {
            self.tabs
                .active()
                .buffer
                .selection()
                .filter(|(s, e)| *e > *s)
        };
        if let Some((s, e)) = from_sel {
            let token = self.tabs.active().buffer.slice(s, e);
            let n = self.tabs.active().buffer.len_chars();
            let flags = crate::search_util::FindFlags {
                match_case: self.settings.find_match_case,
                whole_word: self.settings.find_whole_word,
                use_regex: false,
            };
            let _ = self.mark_style_matches(style, &token, flags, 0, n);
            return;
        }
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Style all: select a token or set Find text".into();
            return;
        }
        let (lo, hi) = self.find_bounds();
        let flags = self.find_flags();
        let _ = self.mark_style_matches(style, &q, flags, lo, hi);
    }

    /// Bookmark every line that matches the Find query under Case/Word/Re/Sel.
    /// Adds bookmarks; does not clear existing ones. Returns newly added count, or
    /// `None` when regex is invalid.
    pub fn bookmark_find_matches(&mut self) -> Option<usize> {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Bookmark all: set Find text".into();
            return Some(0);
        }
        if self.find_regex_blocked() {
            return None;
        }
        let (lo, hi) = self.find_bounds();
        let flags = self.find_flags();
        let text = self.tabs.active().buffer.to_string();
        let matches = crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags);
        let mut lines = std::collections::BTreeSet::new();
        {
            let buf = &self.tabs.active().buffer;
            for (start, _) in &matches {
                lines.insert(buf.char_to_line(*start));
            }
        }
        let match_lines = lines.len();
        let doc = self.tabs.active_mut();
        let before = doc.bookmarks.len();
        for line in lines {
            doc.bookmarks.insert(line);
        }
        let added = doc.bookmarks.len().saturating_sub(before);
        self.status = if match_lines == 0 {
            "Bookmark all: no match".into()
        } else if added == match_lines {
            format!("Bookmarked {match_lines} matching line(s)")
        } else {
            format!("Bookmarked {match_lines} matching line(s) ({added} new)")
        };
        Some(added)
    }

    /// Copy every Find match substring under Case/Word/Re/Sel, newline-joined.
    /// Returns the clipboard payload (`Some("")` when empty / no match), or `None`
    /// when regex is invalid.
    pub fn copy_find_matches(&mut self) -> Option<String> {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Copy matches: set Find text".into();
            return Some(String::new());
        }
        if self.find_regex_blocked() {
            return None;
        }
        let (lo, hi) = self.find_bounds();
        let flags = self.find_flags();
        let text = self.tabs.active().buffer.to_string();
        let matches = crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags);
        if matches.is_empty() {
            self.status = "Copy matches: no match".into();
            return Some(String::new());
        }
        let buf = &self.tabs.active().buffer;
        let mut parts = Vec::with_capacity(matches.len());
        for &(s, e) in &matches {
            parts.push(buf.slice(s, e));
        }
        let joined = parts.join("\n");
        let count = matches.len();
        self.status = format!("Copied {count} match(es)");
        Some(joined)
    }

    /// Copy every line that contains a Find match under Case/Word/Re/Sel, newline-joined.
    /// Whole lines (body only, no trailing EOL), distinct from [`Self::copy_find_matches`]
    /// (match substrings) and from Copy Bookmarked Lines. Returns the clipboard payload
    /// (`Some("")` when empty / no match), or `None` when regex is invalid.
    pub fn copy_find_match_lines(&mut self) -> Option<String> {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Copy matching lines: set Find text".into();
            return Some(String::new());
        }
        if self.find_regex_blocked() {
            return None;
        }
        let (lo, hi) = self.find_bounds();
        let flags = self.find_flags();
        let text = self.tabs.active().buffer.to_string();
        let matches = crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags);
        if matches.is_empty() {
            self.status = "Copy matching lines: no match".into();
            return Some(String::new());
        }
        let buf = &self.tabs.active().buffer;
        let mut lines = std::collections::BTreeSet::new();
        for (start, _) in &matches {
            lines.insert(buf.char_to_line(*start));
        }
        let mut parts = Vec::with_capacity(lines.len());
        for &i in &lines {
            parts.push(buf.line(i).trim_end_matches(['\n', '\r']).to_string());
        }
        let joined = parts.join("\n");
        let count = lines.len();
        self.status = format!("Copied {count} matching line(s)");
        Some(joined)
    }

    /// Cut every Find match substring under Case/Word/Re/Sel: clipboard payload plus
    /// delete matches (one document replace). Returns the clipboard text
    /// (`Some("")` when empty / no match / read-only), or `None` when regex is invalid.
    pub fn cut_find_matches(&mut self) -> Option<String> {
        if !self.ensure_editable() {
            return Some(String::new());
        }
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Cut matches: set Find text".into();
            return Some(String::new());
        }
        if self.find_regex_blocked() {
            return None;
        }
        self.capture_find_scope();
        let flags = self.find_flags();
        let (lo, hi) = self.find_bounds();
        let text = self.tabs.active().buffer.to_string();
        let matches = crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags);
        if matches.is_empty() {
            self.status = "Cut matches: no match".into();
            return Some(String::new());
        }
        let buf = &self.tabs.active().buffer;
        let mut parts = Vec::with_capacity(matches.len());
        let mut touched = std::collections::BTreeSet::new();
        for &(s, e) in &matches {
            parts.push(buf.slice(s, e));
            touched.insert(buf.char_to_line(s));
        }
        let joined = parts.join("\n");
        let count = matches.len();
        let (new_text, replaced, new_hi) =
            crate::search_util::replace_all_in(&text, &q, "", lo, hi, flags);
        debug_assert_eq!(replaced, count);
        self.tabs.active_mut().buffer.replace_document(&new_text);
        if self.settings.find_in_selection {
            self.find_scope = Some((lo, new_hi));
        }
        for line in touched {
            self.tabs.active_mut().note_line_changed(line);
        }
        self.mark_text_changed();
        self.status = format!("Cut {count} match(es)");
        Some(joined)
    }

    /// Cut every line that contains a Find match under Case/Word/Re/Sel: clipboard
    /// payload (whole lines, deduped, body only) plus delete those lines (one document
    /// replace). Distinct from [`Self::cut_find_matches`] (match substrings) and from
    /// [`Self::delete_find_match_lines`] (no clipboard). Returns the clipboard text
    /// (`Some("")` when empty / no match / read-only), or `None` when regex is invalid.
    pub fn cut_find_match_lines(&mut self) -> Option<String> {
        if !self.ensure_editable() {
            return Some(String::new());
        }
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Cut matching lines: set Find text".into();
            return Some(String::new());
        }
        if self.find_regex_blocked() {
            return None;
        }
        self.capture_find_scope();
        let flags = self.find_flags();
        let (lo, hi) = self.find_bounds();
        let text = self.tabs.active().buffer.to_string();
        let matches = crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags);
        if matches.is_empty() {
            self.status = "Cut matching lines: no match".into();
            return Some(String::new());
        }
        let (joined, out, drop_lines, removed_before_lo, removed_before_hi) = {
            let buf = &self.tabs.active().buffer;
            let mut drop_lines = std::collections::BTreeSet::new();
            for (start, _) in &matches {
                drop_lines.insert(buf.char_to_line(*start));
            }
            let mut parts = Vec::with_capacity(drop_lines.len());
            for &i in &drop_lines {
                parts.push(buf.line(i).trim_end_matches(['\n', '\r']).to_string());
            }
            let joined = parts.join("\n");
            let line_count = buf.line_count();
            let mut out = String::new();
            let mut removed_before_lo = 0usize;
            let mut removed_before_hi = 0usize;
            for i in 0..line_count {
                let start = buf.line_to_char(i);
                let end = if i + 1 < line_count {
                    buf.line_to_char(i + 1)
                } else {
                    buf.len_chars()
                };
                if !drop_lines.contains(&i) {
                    out.push_str(&buf.slice(start, end));
                    continue;
                }
                if end <= lo {
                    removed_before_lo += end - start;
                    removed_before_hi += end - start;
                } else if start < hi {
                    if start < lo {
                        removed_before_lo += lo - start;
                    }
                    removed_before_hi += hi.min(end).saturating_sub(start);
                }
            }
            (
                joined,
                out,
                drop_lines,
                removed_before_lo,
                removed_before_hi,
            )
        };
        let count = drop_lines.len();
        self.tabs.active_mut().buffer.replace_document(&out);
        if self.settings.find_in_selection {
            let new_lo = lo.saturating_sub(removed_before_lo);
            let new_hi = hi.saturating_sub(removed_before_hi).max(new_lo);
            self.find_scope = Some((new_lo, new_hi));
        }
        for line in &drop_lines {
            self.tabs.active_mut().note_line_changed(*line);
        }
        self.mark_text_changed();
        self.status = format!("Cut {count} matching line(s)");
        Some(joined)
    }

    /// Delete every line that contains a Find match under Case/Word/Re/Sel.
    /// Whole lines are removed (distinct from Cut, which deletes substrings only).
    /// Returns deleted line count, or `None` when regex is invalid.
    pub fn delete_find_match_lines(&mut self) -> Option<usize> {
        if !self.ensure_editable() {
            return Some(0);
        }
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Delete matching lines: set Find text".into();
            return Some(0);
        }
        if self.find_regex_blocked() {
            return None;
        }
        self.capture_find_scope();
        let flags = self.find_flags();
        let (lo, hi) = self.find_bounds();
        let text = self.tabs.active().buffer.to_string();
        let matches = crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags);
        if matches.is_empty() {
            self.status = "Delete matching lines: no match".into();
            return Some(0);
        }
        let (out, drop_lines, removed_before_lo, removed_before_hi) = {
            let buf = &self.tabs.active().buffer;
            let mut drop_lines = std::collections::BTreeSet::new();
            for (start, _) in &matches {
                drop_lines.insert(buf.char_to_line(*start));
            }
            let line_count = buf.line_count();
            let mut out = String::new();
            let mut removed_before_lo = 0usize;
            let mut removed_before_hi = 0usize;
            for i in 0..line_count {
                let start = buf.line_to_char(i);
                let end = if i + 1 < line_count {
                    buf.line_to_char(i + 1)
                } else {
                    buf.len_chars()
                };
                if !drop_lines.contains(&i) {
                    out.push_str(&buf.slice(start, end));
                    continue;
                }
                if end <= lo {
                    removed_before_lo += end - start;
                    removed_before_hi += end - start;
                } else if start < hi {
                    if start < lo {
                        removed_before_lo += lo - start;
                    }
                    removed_before_hi += hi.min(end).saturating_sub(start);
                }
            }
            (out, drop_lines, removed_before_lo, removed_before_hi)
        };
        let count = drop_lines.len();
        self.tabs.active_mut().buffer.replace_document(&out);
        if self.settings.find_in_selection {
            let new_lo = lo.saturating_sub(removed_before_lo);
            let new_hi = hi.saturating_sub(removed_before_hi).max(new_lo);
            self.find_scope = Some((new_lo, new_hi));
        }
        for line in &drop_lines {
            self.tabs.active_mut().note_line_changed(*line);
        }
        self.mark_text_changed();
        self.status = format!("Deleted {count} matching line(s)");
        Some(count)
    }

    /// Find hits for Document Map: `(total_matches, first_span_per_line)`.
    ///
    /// Uses the active tab's Find bounds when `tab` is active; otherwise the whole
    /// buffer. Empty query or Sel-without-scope yields no ticks.
    pub fn find_match_line_hits_for_tab(&self, tab: usize) -> (usize, Vec<(usize, usize, usize)>) {
        let q = self.find_query.as_str();
        if q.is_empty() {
            return (0, Vec::new());
        }
        let Some(doc) = self.tabs.get(tab) else {
            return (0, Vec::new());
        };
        let text = doc.buffer.to_string();
        let n = doc.buffer.len_chars();
        let (lo, hi) = if tab == self.tabs.active_index() {
            self.find_bounds()
        } else {
            (0, n)
        };
        let matches = crate::search_util::find_all_matches_in(&text, q, lo, hi, self.find_flags());
        let total = matches.len();
        let hits =
            crate::search_util::find_match_line_hits(&matches, |c| doc.buffer.char_to_line(c));
        (total, hits)
    }

    pub fn find_next(&mut self) {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Find: empty query".into();
            return;
        }
        if self.find_regex_blocked() {
            return;
        }
        self.capture_find_scope();
        if self.settings.find_in_selection && self.find_scope.is_none() {
            self.status = "Find: select a range first".into();
            return;
        }
        let from = self
            .tabs
            .active()
            .buffer
            .selection()
            .map(|(_, e)| e)
            .unwrap_or_else(|| self.tabs.active().buffer.caret());
        let text = self.tabs.active().buffer.to_string();
        let flags = self.find_flags();
        let wrap = self.settings.find_wrap;
        let (lo, hi) = self.find_bounds();
        let all = crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags);
        let total = all.len();
        if let Some((s, e)) =
            crate::search_util::find_next_in(&text, &q, from, wrap, (lo, hi), flags)
        {
            self.tabs.active_mut().buffer.set_selection(s, e);
            let idx = all
                .iter()
                .position(|(a, _)| *a == s)
                .map(|i| i + 1)
                .unwrap_or(1);
            self.status = format!("Find: {idx}/{total}");
        } else if total == 0 {
            self.status = "Find: no match".into();
        } else {
            self.status = format!("Find: passed end of {}", self.find_bound_label());
        }
    }

    pub fn find_prev(&mut self) {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Find: empty query".into();
            return;
        }
        if self.find_regex_blocked() {
            return;
        }
        self.capture_find_scope();
        if self.settings.find_in_selection && self.find_scope.is_none() {
            self.status = "Find: select a range first".into();
            return;
        }
        let from = self
            .tabs
            .active()
            .buffer
            .selection()
            .map(|(s, _)| s)
            .unwrap_or_else(|| self.tabs.active().buffer.caret());
        let text = self.tabs.active().buffer.to_string();
        let flags = self.find_flags();
        let wrap = self.settings.find_wrap;
        let (lo, hi) = self.find_bounds();
        let all = crate::search_util::find_all_matches_in(&text, &q, lo, hi, flags);
        let total = all.len();
        if let Some((s, e)) =
            crate::search_util::find_prev_in(&text, &q, from, wrap, (lo, hi), flags)
        {
            self.tabs.active_mut().buffer.set_selection(s, e);
            let idx = all
                .iter()
                .position(|(a, _)| *a == s)
                .map(|i| i + 1)
                .unwrap_or(1);
            self.status = format!("Find: {idx}/{total}");
        } else if total == 0 {
            self.status = "Find: no match".into();
        } else {
            self.status = format!("Find: passed beginning of {}", self.find_bound_label());
        }
    }

    /// Push a path onto recent files using Preferences max.
    pub fn touch_recent(&mut self, path: &std::path::Path) {
        let max = self.settings.recent_limit();
        self.recent.touch_limited(path, max);
    }

    /// Write open paths when restore-session is on.
    pub fn persist_session_if_enabled(&mut self) {
        if !self.settings.restore_session {
            return;
        }
        let _ = self.write_session_entries();
    }

    /// Open paths from the config-dir session file (folds, bookmarks, style marks, caret, selection, scroll, language, encoding, read-only, pin, colour, log-tail, other-view, other-pane focus, sync-scroll, compare, marked partner, active tab).
    pub fn restore_session_from_disk(&mut self) {
        let entries = crate::session::load_entries();
        if entries.is_empty() {
            self.status = format!("No session in {}", crate::session::SESSION_REL);
            return;
        }
        self.pending_session_active = None;
        self.pending_session_scroll = None;
        self.pending_session_scrolls.clear();
        self.pending_session_sels.clear();
        self.pending_session_styles.clear();
        self.pending_session_tail.clear();
        self.session_scroll_line = None;
        self.session_other_view = None;
        self.session_focus_other = false;
        self.session_sync_scroll_v = false;
        self.session_sync_scroll_h = false;
        self.session_compare_on = false;
        self.session_compare_partner = None;
        let mut n = 0usize;
        let mut folds = 0usize;
        let mut bookmarks = 0usize;
        let mut styles = 0usize;
        let mut carets = 0usize;
        let mut selections = 0usize;
        let mut scrolls = 0usize;
        let mut langs = 0usize;
        let mut encodings = 0usize;
        let mut read_onlys = 0usize;
        let mut pins = 0usize;
        let mut colours = 0usize;
        let mut tails = 0usize;
        let mut active_path: Option<PathBuf> = None;
        let mut active_scroll: Option<usize> = None;
        let mut other_path: Option<PathBuf> = None;
        let mut partner_path: Option<PathBuf> = None;
        let mut focus_other = false;
        let mut sync_v = false;
        let mut sync_h = false;
        let mut compare_on = false;
        for entry in entries {
            let path = entry.path.clone();
            let headers = entry.fold_headers;
            let marks = entry.bookmarks;
            let style_marks = entry.style_marks;
            let caret = entry.caret;
            let selection = entry.selection;
            let scroll = entry.scroll;
            let language = entry.language;
            let encoding = entry.encoding;
            let want_readonly = entry.read_only;
            let want_pin = entry.pinned;
            let want_colour = entry.tab_colour;
            let want_tail = entry.tail_follow;
            let want_other = entry.other_view;
            let want_focus_other = entry.focus_other;
            let want_sync_v = entry.sync_scroll_v;
            let want_sync_h = entry.sync_scroll_h;
            let want_compare = entry.compare_on;
            let want_partner = entry.compare_partner;
            let want_active = entry.active;
            self.open_path(path.clone());
            n += 1;
            let idx = self
                .tabs
                .iter()
                .position(|d| d.path.as_ref() == Some(&path));
            let Some(idx) = idx else {
                continue;
            };
            let loading = self.tabs.get(idx).is_some_and(|d| d.loading);
            if !headers.is_empty() {
                if loading {
                    self.pending_session_folds.insert(path.clone(), headers);
                } else {
                    folds += self.apply_session_folds_at(idx, &headers);
                }
            }
            if !marks.is_empty() {
                if loading {
                    self.pending_session_bookmarks.insert(path.clone(), marks);
                } else {
                    bookmarks += self.apply_session_bookmarks_at(idx, &marks);
                }
            }
            if style_marks.iter().any(|m| !m.is_empty()) {
                if loading {
                    self.pending_session_styles
                        .insert(path.clone(), style_marks);
                } else {
                    styles += self.apply_session_styles_at(idx, &style_marks);
                }
            }
            if let Some((line, col)) = caret {
                if loading {
                    self.pending_session_carets
                        .insert(path.clone(), (line, col));
                } else if self.apply_session_caret_at(idx, line, col) {
                    carets += 1;
                }
            }
            if let Some(((al, ac), (cl, cc))) = selection {
                if loading {
                    self.pending_session_sels
                        .insert(path.clone(), ((al, ac), (cl, cc)));
                } else if self.apply_session_sel_at(idx, (al, ac), (cl, cc)) {
                    selections += 1;
                }
            }
            if let Some(line) = scroll {
                if loading {
                    self.pending_session_scrolls.insert(path.clone(), line);
                } else if self.apply_session_scroll_at(idx, line) {
                    scrolls += 1;
                }
            }
            if let Some(lang) = language {
                if loading {
                    self.pending_session_langs.insert(path.clone(), lang);
                } else if self.apply_session_lang_at(idx, &lang) {
                    langs += 1;
                }
            }
            if let Some(enc) = encoding {
                if loading {
                    self.pending_session_encodings.insert(path.clone(), enc);
                } else if self.apply_session_encoding_at(idx, enc) {
                    encodings += 1;
                }
            }
            if want_readonly {
                if loading {
                    self.pending_session_readonly.insert(path.clone());
                } else if self.apply_session_readonly_at(idx) {
                    read_onlys += 1;
                }
            }
            if want_pin {
                if let Some(doc) = self.tabs.get_mut(idx) {
                    if !doc.pinned {
                        doc.pinned = true;
                        pins += 1;
                    }
                }
            }
            if let Some(n) = want_colour {
                if let Some(doc) = self.tabs.get_mut(idx) {
                    if doc.tab_colour != Some(n) {
                        doc.tab_colour = Some(n);
                        colours += 1;
                    }
                }
            }
            if want_tail {
                if loading {
                    self.pending_session_tail.insert(path.clone());
                } else if self.apply_session_tail_at(idx) {
                    tails += 1;
                    if self.tabs.active_index() == idx {
                        self.pending_log_tail_prompt = false;
                    }
                }
            }
            if want_other {
                other_path = Some(path.clone());
            }
            if want_focus_other {
                focus_other = true;
            }
            if want_sync_v {
                sync_v = true;
            }
            if want_sync_h {
                sync_h = true;
            }
            if want_compare {
                compare_on = true;
            }
            if want_partner {
                partner_path = Some(path.clone());
            }
            if want_active {
                active_path = Some(path);
                active_scroll = scroll;
            }
        }
        if n > 0 && self.tabs.len() > 1 {
            if let Some(doc) = self.tabs.get(0) {
                if doc.path.is_none() && !doc.dirty && doc.buffer.is_empty() {
                    self.close_tab(0);
                }
            }
        }
        if let Some(ref path) = active_path {
            let idx = self.tabs.iter().position(|d| d.path.as_ref() == Some(path));
            if let Some(idx) = idx {
                if self.tabs.get(idx).is_some_and(|d| d.loading) {
                    self.pending_session_active = Some(path.clone());
                    self.pending_session_scroll = active_scroll;
                } else {
                    self.tabs.set_active(idx);
                    self.reset_view = false;
                    if let Some(scroll) = active_scroll {
                        self.session_scroll_line = Some(scroll);
                        self.session_follow_caret = false;
                    } else {
                        self.session_follow_caret = true;
                    }
                }
            }
        } else if n > 0 {
            self.reset_view = false;
            if let Some(scroll) = active_scroll {
                self.session_scroll_line = Some(scroll);
                self.session_follow_caret = false;
            } else {
                self.session_follow_caret = true;
            }
        }
        let mut bits = format!("Session restored: {n} file(s)");
        if folds > 0 {
            bits.push_str(&format!(", {folds} fold(s)"));
        }
        if bookmarks > 0 {
            bits.push_str(&format!(", {bookmarks} bookmark(s)"));
        }
        if styles > 0 {
            bits.push_str(&format!(", {styles} style mark(s)"));
        }
        if carets > 0 {
            bits.push_str(&format!(", {carets} caret(s)"));
        }
        if selections > 0 {
            bits.push_str(&format!(", {selections} selection(s)"));
        }
        if scrolls > 0 {
            bits.push_str(&format!(", {scrolls} scroll"));
        }
        if langs > 0 {
            bits.push_str(&format!(", {langs} language(s)"));
        }
        if encodings > 0 {
            bits.push_str(&format!(", {encodings} encoding(s)"));
        }
        if read_onlys > 0 {
            bits.push_str(&format!(", {read_onlys} read-only"));
        }
        if pins > 0 {
            bits.push_str(&format!(", {pins} pin(s)"));
        }
        if colours > 0 {
            bits.push_str(&format!(", {colours} colour(s)"));
        }
        if tails > 0 {
            bits.push_str(&format!(", {tails} log-tail"));
        }
        if let Some(path) = other_path {
            if active_path.as_ref() != Some(&path) {
                self.session_other_view = Some(path);
                self.session_focus_other = focus_other;
                self.session_sync_scroll_v = sync_v;
                self.session_sync_scroll_h = sync_h;
                self.session_compare_on = compare_on;
                if focus_other {
                    bits.push_str(", dual view · other focused");
                } else {
                    bits.push_str(", dual view");
                }
                match (sync_v, sync_h) {
                    (true, true) => bits.push_str(" · sync V+H"),
                    (true, false) => bits.push_str(" · sync V"),
                    (false, true) => bits.push_str(" · sync H"),
                    (false, false) => {}
                }
                if compare_on {
                    bits.push_str(" · compare");
                }
            }
        }
        if let Some(path) = partner_path {
            if active_path.as_ref() != Some(&path) {
                self.session_compare_partner = Some(path);
                bits.push_str(", compare partner");
            }
        }
        self.status = bits;
    }

    pub fn save_session_now(&mut self) {
        match self.write_session_entries() {
            Ok(()) => {
                self.status = format!("Session saved ({})", crate::session::SESSION_REL);
            }
            Err(e) => self.status = format!("Save session failed: {e}"),
        }
    }

    fn write_session_entries(&self) -> Result<(), String> {
        let active_path = self.tabs.active().path.clone();
        let active_scroll = self.session_write_scroll;
        let other_path = self.session_write_other_view.clone();
        let partner_path = self.session_write_compare_partner.clone();
        let write_focus_other = self.session_write_focus_other;
        let write_sync_v = self.session_write_sync_v;
        let write_sync_h = self.session_write_sync_h;
        let write_compare = self.session_write_compare;
        let entries: Vec<crate::session::SessionEntry> = self
            .tabs
            .iter()
            .filter_map(|d| {
                let path = d.path.clone()?;
                let regions = crate::fold::compute_fold_regions(d.language.as_str(), &d.buffer);
                let fold_headers = crate::fold::folded_headers(&d.hidden_lines, &regions);
                let bookmarks: Vec<usize> = d.bookmarks.iter().copied().collect();
                let mut style_marks: [Vec<usize>; 5] = Default::default();
                for (i, set) in d.style_marks.iter().enumerate() {
                    style_marks[i] = set.iter().copied().collect();
                }
                let caret_idx = d.buffer.caret();
                let line = d.buffer.char_to_line(caret_idx);
                let col = caret_idx.saturating_sub(d.buffer.line_to_char(line));
                let selection = d.buffer.selection().map(|(start, end)| {
                    let caret_pos = d.buffer.caret();
                    let (anchor_pos, caret_pos) = if caret_pos == end {
                        (start, end)
                    } else {
                        (end, start)
                    };
                    let a_line = d.buffer.char_to_line(anchor_pos);
                    let a_col = anchor_pos.saturating_sub(d.buffer.line_to_char(a_line));
                    let c_line = d.buffer.char_to_line(caret_pos);
                    let c_col = caret_pos.saturating_sub(d.buffer.line_to_char(c_line));
                    ((a_line, a_col), (c_line, c_col))
                });
                let language = {
                    let lang = d.language.trim();
                    if lang.is_empty() {
                        None
                    } else {
                        Some(lang.to_string())
                    }
                };
                let encoding = Some(d.encoding);
                let read_only = d.read_only;
                let pinned = d.pinned;
                let tab_colour = d.tab_colour.filter(|n| (1..=5).contains(n));
                let tail_follow = d.tail_follow;
                let active = active_path.as_ref() == Some(&path);
                let other_view = !active && other_path.as_ref() == Some(&path);
                let compare_partner = !active && partner_path.as_ref() == Some(&path);
                let focus_other = other_view && write_focus_other;
                // Sync / compare flags are UI-level; persist once on the active tab.
                let sync_scroll_v = active && write_sync_v;
                let sync_scroll_h = active && write_sync_h;
                let compare_on = active && write_compare;
                // Prefer live UI scroll for the focused tab; every tab keeps its last viewport.
                let scroll = Some(if active {
                    active_scroll.unwrap_or(d.view_scroll_line)
                } else {
                    d.view_scroll_line
                });
                Some(crate::session::SessionEntry {
                    path,
                    fold_headers,
                    bookmarks,
                    style_marks,
                    caret: Some((line, col)),
                    selection,
                    scroll,
                    language,
                    encoding,
                    read_only,
                    pinned,
                    tab_colour,
                    tail_follow,
                    other_view,
                    focus_other,
                    sync_scroll_v,
                    sync_scroll_h,
                    compare_on,
                    compare_partner,
                    active,
                })
            })
            .collect();
        crate::session::save_entries(&entries)
    }

    fn apply_session_folds_at(&mut self, tab: usize, headers: &[usize]) -> usize {
        let regions = {
            let Some(doc) = self.tabs.get(tab) else {
                return 0;
            };
            crate::fold::compute_fold_regions(doc.language.as_str(), &doc.buffer)
        };
        let Some(doc) = self.tabs.get_mut(tab) else {
            return 0;
        };
        crate::fold::apply_fold_headers(&mut doc.hidden_lines, &regions, headers)
    }

    fn apply_session_bookmarks_at(&mut self, tab: usize, lines: &[usize]) -> usize {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return 0;
        };
        let max = doc.buffer.line_count();
        let before = doc.bookmarks.len();
        for &line in lines {
            if line < max {
                doc.bookmarks.insert(line);
            }
        }
        doc.bookmarks.len().saturating_sub(before)
    }

    fn apply_session_styles_at(&mut self, tab: usize, slots: &[Vec<usize>; 5]) -> usize {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return 0;
        };
        let max = doc.buffer.line_count();
        let mut added = 0usize;
        for (i, lines) in slots.iter().enumerate() {
            let before = doc.style_marks[i].len();
            for &line in lines {
                if line < max {
                    doc.style_marks[i].insert(line);
                }
            }
            added += doc.style_marks[i].len().saturating_sub(before);
        }
        added
    }

    fn apply_session_caret_at(&mut self, tab: usize, line: usize, col: usize) -> bool {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return false;
        };
        let pos = session_line_col_to_pos(&doc.buffer, line, col);
        doc.buffer.set_caret(pos);
        true
    }

    /// Restore a non-empty selection (anchor then caret). `@tail` applied later clears it.
    fn apply_session_sel_at(
        &mut self,
        tab: usize,
        anchor: (usize, usize),
        caret: (usize, usize),
    ) -> bool {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return false;
        };
        let anchor_pos = session_line_col_to_pos(&doc.buffer, anchor.0, anchor.1);
        let caret_pos = session_line_col_to_pos(&doc.buffer, caret.0, caret.1);
        if anchor_pos == caret_pos {
            return false;
        }
        doc.buffer.set_selection(anchor_pos, caret_pos);
        true
    }

    /// Store first visible line on the tab (clamped). Used for session restore and tab switches.
    fn apply_session_scroll_at(&mut self, tab: usize, line: usize) -> bool {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return false;
        };
        let max_line = doc.buffer.line_count().saturating_sub(1);
        doc.view_scroll_line = line.min(max_line);
        true
    }

    fn apply_session_lang_at(&mut self, tab: usize, lang: &str) -> bool {
        let lang = lang.trim();
        if lang.is_empty() {
            return false;
        }
        let Some(doc) = self.tabs.get_mut(tab) else {
            return false;
        };
        if doc.language == lang {
            return false;
        }
        doc.language = lang.to_string();
        self.highlight_dirty = true;
        if self.highlight_tab_other == Some(tab) {
            self.highlight_dirty_other = true;
        }
        true
    }

    fn apply_session_encoding_at(&mut self, tab: usize, encoding: FileEncoding) -> bool {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return false;
        };
        if doc.encoding == encoding {
            return false;
        }
        doc.encoding = encoding;
        true
    }

    fn apply_session_readonly_at(&mut self, tab: usize) -> bool {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return false;
        };
        if doc.read_only {
            return false;
        }
        doc.read_only = true;
        true
    }

    /// Arm log-tail follow on a restored tab (jumps caret to end; wins over `@caret` / `@sel`).
    fn apply_session_tail_at(&mut self, tab: usize) -> bool {
        let Some(doc) = self.tabs.get(tab) else {
            return false;
        };
        if doc.tail_follow {
            return true;
        }
        if doc.dirty {
            return false;
        }
        let Some(path) = doc.path.clone() else {
            return false;
        };
        let Ok(size) = fs::file_size(&path) else {
            return false;
        };
        let Some(doc) = self.tabs.get_mut(tab) else {
            return false;
        };
        doc.tail_bytes = size;
        doc.tail_follow = true;
        let end = doc.buffer.len_chars();
        doc.buffer.set_caret(end);
        true
    }

    fn apply_pending_session_folds_for(&mut self, path: &Path) {
        let Some(headers) = self.pending_session_folds.remove(path) else {
            return;
        };
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            let n = self.apply_session_folds_at(idx, &headers);
            if n > 0 {
                self.status = format!("{} · restored {n} fold(s)", self.status);
            }
        }
    }

    fn apply_pending_session_bookmarks_for(&mut self, path: &Path) {
        let Some(lines) = self.pending_session_bookmarks.remove(path) else {
            return;
        };
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            let n = self.apply_session_bookmarks_at(idx, &lines);
            if n > 0 {
                self.status = format!("{} · restored {n} bookmark(s)", self.status);
            }
        }
    }

    fn apply_pending_session_styles_for(&mut self, path: &Path) {
        let Some(slots) = self.pending_session_styles.remove(path) else {
            return;
        };
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            let n = self.apply_session_styles_at(idx, &slots);
            if n > 0 {
                self.status = format!("{} · restored {n} style mark(s)", self.status);
            }
        }
    }

    fn apply_pending_session_caret_for(&mut self, path: &Path) {
        let Some((line, col)) = self.pending_session_carets.remove(path) else {
            return;
        };
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            if self.apply_session_caret_at(idx, line, col) {
                self.status = format!("{} · restored caret", self.status);
                if self.tabs.active_index() == idx && self.session_scroll_line.is_none() {
                    self.session_follow_caret = true;
                }
            }
        }
    }

    fn apply_pending_session_sel_for(&mut self, path: &Path) {
        let Some((anchor, caret)) = self.pending_session_sels.remove(path) else {
            return;
        };
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            if self.apply_session_sel_at(idx, anchor, caret) {
                self.status = format!("{} · restored selection", self.status);
            }
        }
    }

    fn apply_pending_session_scroll_for(&mut self, path: &Path) {
        let Some(line) = self.pending_session_scrolls.remove(path) else {
            return;
        };
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            if self.apply_session_scroll_at(idx, line) {
                self.status = format!("{} · restored scroll", self.status);
            }
        }
    }

    fn apply_pending_session_lang_for(&mut self, path: &Path) {
        let Some(lang) = self.pending_session_langs.remove(path) else {
            return;
        };
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            if self.apply_session_lang_at(idx, &lang) {
                self.status = format!("{} · restored language {lang}", self.status);
            }
        }
    }

    fn apply_pending_session_encoding_for(&mut self, path: &Path) {
        let Some(encoding) = self.pending_session_encodings.remove(path) else {
            return;
        };
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            if self.apply_session_encoding_at(idx, encoding) {
                self.status = format!("{} · restored encoding {}", self.status, encoding.label());
            }
        }
    }

    fn apply_pending_session_readonly_for(&mut self, path: &Path) {
        if !self.pending_session_readonly.remove(path) {
            return;
        }
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            if self.apply_session_readonly_at(idx) {
                self.status = format!("{} · restored read-only", self.status);
            }
        }
    }

    fn apply_pending_session_tail_for(&mut self, path: &Path) {
        if !self.pending_session_tail.remove(path) {
            return;
        }
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            if self.apply_session_tail_at(idx) {
                self.status = format!("{} · restored log-tail", self.status);
                if self.tabs.active_index() == idx {
                    self.pending_log_tail_prompt = false;
                }
            }
        }
    }

    fn apply_pending_session_active_for(&mut self, path: &Path) {
        if self.pending_session_active.as_deref() != Some(path) {
            return;
        }
        self.pending_session_active = None;
        let scroll = self.pending_session_scroll.take();
        let idx = self
            .tabs
            .iter()
            .position(|d| d.path.as_deref() == Some(path));
        if let Some(idx) = idx {
            self.tabs.set_active(idx);
            self.reset_view = false;
            if let Some(line) = scroll {
                let _ = self.apply_session_scroll_at(idx, line);
                self.session_scroll_line = Some(line);
                self.session_follow_caret = false;
            } else if let Some(doc) = self.tabs.get(idx) {
                if doc.view_scroll_line > 0 {
                    self.session_scroll_line = Some(doc.view_scroll_line);
                    self.session_follow_caret = false;
                } else {
                    self.session_follow_caret = true;
                }
            } else {
                self.session_follow_caret = true;
            }
        }
    }

    pub fn load_session_now(&mut self) {
        self.restore_session_from_disk();
    }

    pub fn undo(&mut self) {
        self.undo_at(self.tabs.active_index());
    }

    pub fn redo(&mut self) {
        self.redo_at(self.tabs.active_index());
    }

    pub fn undo_at(&mut self, tab: usize) {
        if self
            .with_editable_buffer(tab, |buf| buf.undo())
            .unwrap_or(false)
        {
            self.mark_text_changed_at_with(tab, false);
            self.status = "Undo".into();
        }
    }

    pub fn redo_at(&mut self, tab: usize) {
        if self
            .with_editable_buffer(tab, |buf| buf.redo())
            .unwrap_or(false)
        {
            self.mark_text_changed_at_with(tab, false);
            self.status = "Redo".into();
        }
    }

    pub fn save_all(&mut self) {
        let mut ok = 0usize;
        let mut fail = 0usize;
        let count = self.tabs.len();
        for i in 0..count {
            self.tabs.set_active(i);
            if self.tabs.active().path.is_none() {
                if self.save_as_dialog() {
                    ok += 1;
                } else {
                    fail += 1;
                }
            } else if self.save() {
                ok += 1;
            } else {
                fail += 1;
            }
        }
        self.status = format!("Save All: {ok} saved, {fail} skipped/failed");
    }

    pub fn save_copy_as(&mut self) {
        let path = rfd::FileDialog::new()
            .set_file_name(self.tabs.active().title.as_str())
            .save_file();
        if let Some(path) = path {
            let content = self.tabs.active().buffer.to_string();
            let encoding = self.tabs.active().encoding;
            match fs::write_file_with_encoding(&path, &content, fs_encoding_from_file(encoding)) {
                Ok(()) => {
                    self.status =
                        format!("Saved a copy as {} ({})", path.display(), encoding.label());
                }
                Err(e) => self.status = format!("Save copy failed: {e}"),
            }
        }
    }

    pub fn rename_active(&mut self) {
        let Some(old) = self.tabs.active().path.clone() else {
            self.status = "Rename: save the file first".into();
            return;
        };
        let Some(new_path) = rfd::FileDialog::new()
            .set_file_name(self.tabs.active().title.as_str())
            .set_directory(old.parent().unwrap_or_else(|| std::path::Path::new(".")))
            .save_file()
        else {
            return;
        };
        if new_path == old {
            return;
        }
        match std::fs::rename(&old, &new_path) {
            Ok(()) => {
                let doc = self.tabs.active_mut();
                doc.path = Some(new_path.clone());
                doc.title = new_path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| new_path.display().to_string());
                doc.language = doc::detect_language(&new_path);
                self.recent.remove(&old);
                self.touch_recent(&new_path);
                self.highlight_dirty = true;
                self.status = format!("Renamed to {}", new_path.display());
            }
            Err(e) => self.status = format!("Rename failed: {e}"),
        }
    }

    pub fn close_all_but_current(&mut self) {
        self.start_bulk_close(BulkClose::AllButCurrent);
        if self.pending_close.is_none() {
            self.status = "Closed all but active".into();
        }
    }

    /// Close tabs with `pinned == false`. Keeps pinned tabs.
    /// Pin UI may be missing; with no pins, closes nothing.
    pub fn close_all_but_pinned(&mut self) {
        if !self.tabs.iter().any(|d| d.pinned) {
            self.status = "Nothing pinned — closed none".into();
            return;
        }
        self.start_bulk_close(BulkClose::AllButPinned);
        if self.pending_close.is_none() {
            self.status = "Closed all but pinned".into();
        }
    }

    pub fn close_all_to_left(&mut self) {
        self.start_bulk_close(BulkClose::AllToLeft);
        if self.pending_close.is_none() {
            self.status = "Closed tabs to the left".into();
        }
    }

    pub fn close_all_to_right(&mut self) {
        self.start_bulk_close(BulkClose::AllToRight);
        if self.pending_close.is_none() {
            self.status = "Closed tabs to the right".into();
        }
    }

    pub fn close_all_unchanged(&mut self) {
        self.start_bulk_close(BulkClose::AllUnchanged);
        if self.pending_close.is_none() {
            self.status = "Closed unchanged documents".into();
        }
    }

    pub fn pick_workspace_folder(&mut self) {
        if let Some(dir) = rfd::FileDialog::new()
            .set_title("Open Folder as Workspace")
            .pick_folder()
        {
            self.workspace_root = dir;
            self.persist_workspace_root();
            self.status = format!("Workspace: {}", self.workspace_root.display());
        } else {
            self.status = "Workspace folder pick cancelled".into();
        }
    }

    pub fn set_workspace_from_active(&mut self) {
        if let Some(path) = self.tabs.active().path.clone() {
            if let Some(parent) = path.parent() {
                self.workspace_root = parent.to_path_buf();
                self.persist_workspace_root();
                self.status = format!("Workspace: {}", self.workspace_root.display());
                return;
            }
        }
        self.workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.persist_workspace_root();
        self.status = format!("Workspace: {} (cwd)", self.workspace_root.display());
    }

    pub fn open_containing_folder(&mut self) {
        let Some(path) = self.tabs.active().path.clone() else {
            self.status = "No file path — save first".into();
            return;
        };
        let folder = path.parent().unwrap_or(path.as_path());
        let result = {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open").arg(folder).status()
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("explorer").arg(folder).status()
            }
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                std::process::Command::new("xdg-open").arg(folder).status()
            }
        };
        match result {
            Ok(s) if s.success() => self.status = "Opened containing folder".into(),
            Ok(_) => self.status = "Open folder: command failed".into(),
            Err(e) => self.status = format!("Open folder failed: {e}"),
        }
    }

    pub fn open_in_default_viewer(&mut self) {
        let Some(path) = self.tabs.active().path.clone() else {
            self.status = "No file path — save first".into();
            return;
        };
        let result = {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open").arg(&path).status()
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/C", "start", "", &path.to_string_lossy()])
                    .status()
            }
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                std::process::Command::new("xdg-open").arg(&path).status()
            }
        };
        match result {
            Ok(s) if s.success() => self.status = "Opened in default viewer".into(),
            Ok(_) => self.status = "Default viewer: command failed".into(),
            Err(e) => self.status = format!("Default viewer failed: {e}"),
        }
    }

    pub fn reveal_in_os(&mut self) {
        let Some(path) = self.tabs.active().path.clone() else {
            self.status = "No file path — save first".into();
            return;
        };
        self.reveal_path_in_os(&path);
    }

    /// Reveal a file (select when possible) or open a folder in the OS file manager.
    pub fn reveal_path_in_os(&mut self, path: &Path) {
        let result = {
            #[cfg(target_os = "macos")]
            {
                if path.is_dir() {
                    std::process::Command::new("open").arg(path).status()
                } else {
                    std::process::Command::new("open")
                        .args(["-R", &path.to_string_lossy()])
                        .status()
                }
            }
            #[cfg(target_os = "windows")]
            {
                if path.is_dir() {
                    std::process::Command::new("explorer").arg(path).status()
                } else {
                    std::process::Command::new("explorer")
                        .arg(format!("/select,{}", path.display()))
                        .status()
                }
            }
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                let target = if path.is_dir() {
                    path
                } else {
                    path.parent().unwrap_or(path)
                };
                std::process::Command::new("xdg-open").arg(target).status()
            }
        };
        match result {
            Ok(s) if s.success() => self.status = "Revealed in file manager".into(),
            Ok(_) => self.status = "Reveal failed".into(),
            Err(e) => self.status = format!("Reveal failed: {e}"),
        }
    }

    /// Folder listing for the Project panel (skip dotfiles; dirs first; optional name filter).
    pub fn list_project_panel_entries(root: &Path, filter: &str) -> Vec<PathBuf> {
        let filter = filter.to_ascii_lowercase();
        let mut entries = Vec::new();
        if let Ok(rd) = std::fs::read_dir(root) {
            for ent in rd.flatten() {
                let p = ent.path();
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.') {
                    continue;
                }
                if !filter.is_empty() && !name.to_ascii_lowercase().contains(&filter) {
                    continue;
                }
                entries.push(p);
            }
        }
        entries.sort_by(|a, b| {
            b.is_dir()
                .cmp(&a.is_dir())
                .then(a.file_name().cmp(&b.file_name()))
        });
        entries
    }

    pub fn open_shell_here(&mut self) {
        let path = self.tabs.active().path.clone();
        let folder = path
            .as_ref()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        let result = {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .args(["-a", "Terminal"])
                    .arg(&folder)
                    .status()
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args([
                        "/C",
                        "start",
                        "cmd",
                        "/K",
                        "cd",
                        "/D",
                        &folder.to_string_lossy(),
                    ])
                    .status()
            }
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                std::process::Command::new("x-terminal-emulator")
                    .arg(format!("--working-directory={}", folder.display()))
                    .status()
                    .or_else(|_| std::process::Command::new("xdg-open").arg(&folder).status())
            }
        };
        match result {
            Ok(s) if s.success() => self.status = "Opened shell in folder".into(),
            Ok(_) => self.status = "Open shell: command failed".into(),
            Err(e) => self.status = format!("Open shell failed: {e}"),
        }
    }

    pub fn insert_datetime(&mut self, long: bool) {
        if !self.ensure_editable() {
            return;
        }
        let now = chrono_lite_now(long);
        self.tabs.active_mut().buffer.insert(&now);
        self.mark_text_changed();
        self.status = "Inserted date/time".into();
    }

    /// ISO-8601 local style (custom format stand-in until Preferences exist).
    pub fn insert_datetime_custom(&mut self) {
        if !self.ensure_editable() {
            return;
        }
        let now = chrono_lite_custom();
        self.tabs.active_mut().buffer.insert(&now);
        self.mark_text_changed();
        self.status = "Inserted custom date/time".into();
    }

    pub fn switch_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.tabs.set_active(index);
            self.highlight_dirty = true;
            self.status = format!("Tab {}", index + 1);
        }
    }

    pub fn next_tab(&mut self) {
        let n = self.tabs.len();
        if n == 0 {
            return;
        }
        let i = (self.tabs.active_index() + 1) % n;
        self.switch_tab(i);
    }

    pub fn prev_tab(&mut self) {
        let n = self.tabs.len();
        if n == 0 {
            return;
        }
        let i = (self.tabs.active_index() + n - 1) % n;
        self.switch_tab(i);
    }

    /// Enable log-tail follow on the active saved document.
    pub fn enable_tail_follow(&mut self) -> bool {
        let Some(path) = self.tabs.active().path.clone() else {
            self.status = "Tail: save or open a file on disk first".into();
            return false;
        };
        if self.tabs.active().dirty {
            self.status = "Tail: save changes before enabling follow".into();
            return false;
        }
        match fs::file_size(&path) {
            Ok(size) => {
                self.tabs.active_mut().tail_bytes = size;
                self.tabs.active_mut().tail_follow = true;
                // Jump to end so new lines appear in view.
                let end = self.tabs.active().buffer.len_chars();
                self.tabs.active_mut().buffer.set_caret(end);
                self.status = "Tail ON — following log (poll ~250ms)".into();
                true
            }
            Err(e) => {
                self.status = format!("Tail failed: {e}");
                false
            }
        }
    }

    /// Toggle log-tail follow on the active saved document.
    pub fn toggle_tail_follow(&mut self) -> bool {
        if self.tabs.active().path.is_none() {
            self.status = "Tail: save or open a file on disk first".into();
            return false;
        }
        if self.tabs.active().tail_follow {
            self.tabs.active_mut().tail_follow = false;
            self.status = "Tail OFF".into();
            false
        } else {
            self.enable_tail_follow()
        }
    }

    /// Apply queued tail results, then schedule background polls (~250 ms).
    /// Returns true if the active tab grew (for scroll-follow).
    pub fn poll_tail(&mut self) -> bool {
        let mut active_grew = self.drain_tail_msgs();
        if self.tail_last_poll.elapsed() < Duration::from_millis(250) {
            return active_grew;
        }
        self.tail_last_poll = Instant::now();
        self.schedule_tail_polls();
        active_grew |= self.drain_tail_msgs();
        active_grew
    }

    fn schedule_tail_polls(&mut self) {
        let count = self.tabs.len();
        for i in 0..count {
            let Some(doc) = self.tabs.get(i) else {
                continue;
            };
            if !doc.tail_follow {
                continue;
            }
            let Some(path) = doc.path.clone() else {
                continue;
            };
            if self.tail_inflight.contains(&path) {
                continue;
            }
            let offset = doc.tail_bytes;
            self.tail_inflight.insert(path.clone());
            fs::poll_tail_async(path, offset, self.tail_channel.tx.clone());
        }
    }

    fn drain_tail_msgs(&mut self) -> bool {
        let mut active_grew = false;
        while let Ok(msg) = self.tail_channel.rx.try_recv() {
            active_grew |= self.apply_tail_msg(msg);
        }
        active_grew
    }

    /// Apply one worker result. Keeps dirty/suspend policy; drops stale offsets.
    pub fn apply_tail_msg(&mut self, msg: TailMsg) -> bool {
        self.tail_inflight.remove(&msg.path);
        let active = self.tabs.active_index();
        let Some(i) = self
            .tabs
            .iter()
            .enumerate()
            .find(|(_, d)| d.path.as_ref() == Some(&msg.path) && d.tail_follow)
            .map(|(i, _)| i)
        else {
            return false;
        };
        let Some(doc) = self.tabs.get(i) else {
            return false;
        };
        if doc.tail_bytes != msg.offset {
            return false;
        }
        let dirty = doc.dirty;
        match msg.outcome {
            Ok(TailOutcome::Unchanged { .. }) => false,
            Ok(TailOutcome::Appended {
                text,
                size,
                encoding_note,
            }) => {
                if dirty {
                    if let Some(d) = self.tabs.get_mut(i) {
                        d.tail_follow = false;
                    }
                    if i == active {
                        self.status = "Tail OFF — document has unsaved edits".into();
                    }
                    return false;
                }
                let mut grew = false;
                if let Some(d) = self.tabs.get_mut(i) {
                    if !text.is_empty() {
                        let end = d.buffer.len_chars();
                        d.buffer.set_caret(end);
                        d.buffer.insert(&text);
                        d.mark_clean();
                        if i == active {
                            grew = true;
                            let n = text.lines().count().max(1);
                            self.status = match &encoding_note {
                                Some(note) => format!("Tail: +{n} line(s) — {note}"),
                                None => format!("Tail: +{n} line(s)"),
                            };
                        }
                    }
                    d.tail_bytes = size;
                }
                if i == active {
                    self.highlight_dirty = true;
                }
                grew
            }
            Ok(TailOutcome::Rotated {
                content,
                bytes,
                encoding,
                ..
            }) => {
                if dirty {
                    if let Some(d) = self.tabs.get_mut(i) {
                        d.tail_follow = false;
                    }
                    if i == active {
                        self.status =
                            "Tail OFF — file rotated while document has unsaved edits".into();
                    }
                    return false;
                }
                if let Some(d) = self.tabs.get_mut(i) {
                    d.buffer.replace_document(&content);
                    d.encoding = file_encoding_from_fs(encoding);
                    d.tail_bytes = bytes;
                    d.mark_clean();
                    let end = d.buffer.len_chars();
                    d.buffer.set_caret(end);
                }
                if i == active {
                    self.highlight_dirty = true;
                    self.status = "Tail: file rotated — reloaded".into();
                    return true;
                }
                false
            }
            Ok(TailOutcome::RotatedReloadFailed { error, .. }) => {
                if dirty {
                    if let Some(d) = self.tabs.get_mut(i) {
                        d.tail_follow = false;
                    }
                    if i == active {
                        self.status =
                            "Tail OFF — file rotated while document has unsaved edits".into();
                    }
                    return false;
                }
                if i == active {
                    self.status = format!("Tail reload failed: {error}");
                }
                false
            }
            Err(e) => {
                if i == active {
                    self.status = format!("Tail error: {e}");
                }
                false
            }
        }
    }
}

fn file_encoding_from_fs(enc: TextEncoding) -> FileEncoding {
    match enc {
        TextEncoding::Utf8 => FileEncoding::Utf8,
        TextEncoding::Utf8Bom => FileEncoding::Utf8Bom,
        TextEncoding::Utf16Le => FileEncoding::Utf16Le,
        TextEncoding::Utf16Be => FileEncoding::Utf16Be,
        TextEncoding::Windows1252 => FileEncoding::Windows1252,
    }
}

fn fs_encoding_from_file(enc: FileEncoding) -> TextEncoding {
    match enc {
        FileEncoding::Utf8 => TextEncoding::Utf8,
        FileEncoding::Utf8Bom => TextEncoding::Utf8Bom,
        FileEncoding::Utf16Le => TextEncoding::Utf16Le,
        FileEncoding::Utf16Be => TextEncoding::Utf16Be,
        FileEncoding::Windows1252 => TextEncoding::Windows1252,
    }
}

/// Local date/time without extra crates (uses system `date` only as last resort).
fn chrono_lite_now(long: bool) -> String {
    use std::time::SystemTime;
    let _ = SystemTime::now();
    // Prefer OS locale formatting via `date`.
    let args: &[&str] = if long {
        &["+%A, %d %B %Y %H:%M:%S"]
    } else {
        &["+%Y-%m-%d %H:%M"]
    };
    if let Ok(out) = std::process::Command::new("date").args(args).output() {
        if out.status.success() {
            return String::from_utf8_lossy(&out.stdout).trim().to_string();
        }
    }
    if long {
        "DateTime".into()
    } else {
        "YYYY-MM-DD HH:MM".into()
    }
}

fn chrono_lite_custom() -> String {
    if let Ok(out) = std::process::Command::new("date")
        .args(["+%Y-%m-%dT%H:%M:%S"])
        .output()
    {
        if out.status.success() {
            return String::from_utf8_lossy(&out.stdout).trim().to_string();
        }
    }
    "YYYY-MM-DDTHH:MM:SS".into()
}

/// Clamp a 0-based `(line, col)` to a char index in `buf` (session restore).
fn session_line_col_to_pos(buf: &buffer::TextBuffer, line: usize, col: usize) -> usize {
    let line_count = buf.line_count().max(1);
    let line = line.min(line_count.saturating_sub(1));
    let start = buf.line_to_char(line);
    let end = if line + 1 < buf.line_count() {
        buf.line_to_char(line + 1)
    } else {
        buf.len_chars()
    };
    let max_col = end.saturating_sub(start);
    start + col.min(max_col)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tail_doc(state: &mut EditorState, path: PathBuf, content: &str, offset: u64) -> Document {
        let id = state.tabs.alloc_id();
        let mut doc = Document::from_path(id, path, content.to_string());
        doc.tail_follow = true;
        doc.tail_bytes = offset;
        doc.mark_clean();
        doc
    }

    #[test]
    fn apply_tail_appended_updates_buffer() {
        let path = PathBuf::from("tail-apply-append.log");
        let mut state = EditorState::new();
        let doc = tail_doc(&mut state, path.clone(), "line1\n", 6);
        state.tabs.open_document(doc);
        state.tail_inflight.insert(path.clone());
        let grew = state.apply_tail_msg(TailMsg {
            path: path.clone(),
            offset: 6,
            outcome: Ok(TailOutcome::Appended {
                text: "line2\n".into(),
                size: 12,
                encoding_note: None,
            }),
        });
        assert!(grew);
        assert!(!state.tail_inflight.contains(&path));
        assert_eq!(state.tabs.active().buffer.to_string(), "line1\nline2\n");
        assert_eq!(state.tabs.active().tail_bytes, 12);
        assert!(state.tabs.active().tail_follow);
        assert!(!state.tabs.active().dirty);
    }

    #[test]
    fn apply_tail_appended_suspends_when_dirty() {
        let path = PathBuf::from("tail-apply-dirty.log");
        let mut state = EditorState::new();
        let mut doc = tail_doc(&mut state, path.clone(), "line1\n", 6);
        doc.mark_dirty();
        state.tabs.open_document(doc);
        let grew = state.apply_tail_msg(TailMsg {
            path: path.clone(),
            offset: 6,
            outcome: Ok(TailOutcome::Appended {
                text: "line2\n".into(),
                size: 12,
                encoding_note: None,
            }),
        });
        assert!(!grew);
        assert!(!state.tabs.active().tail_follow);
        assert_eq!(state.tabs.active().buffer.to_string(), "line1\n");
        assert!(state.status.contains("unsaved"));
    }

    #[test]
    fn apply_tail_ignores_stale_offset() {
        let path = PathBuf::from("tail-apply-stale.log");
        let mut state = EditorState::new();
        let doc = tail_doc(&mut state, path.clone(), "line1\n", 12);
        state.tabs.open_document(doc);
        let grew = state.apply_tail_msg(TailMsg {
            path,
            offset: 6,
            outcome: Ok(TailOutcome::Appended {
                text: "stale\n".into(),
                size: 12,
                encoding_note: None,
            }),
        });
        assert!(!grew);
        assert_eq!(state.tabs.active().buffer.to_string(), "line1\n");
        assert_eq!(state.tabs.active().tail_bytes, 12);
    }

    #[test]
    fn open_missing_file_is_safe() {
        let mut state = EditorState::new();
        let missing = PathBuf::from("definitely-missing-npp-stability-test-file-xyz.txt");
        let before_tabs = state.tabs.len();
        state.open_path(missing.clone());
        assert!(!missing.exists());
        assert_eq!(state.tabs.len(), before_tabs);
        assert!(state.status.contains("File not found"));
    }

    #[test]
    fn pending_load_matches_by_id_after_reorder() {
        let mut state = EditorState::new();
        let path = PathBuf::from("pending-stability-demo.txt");
        let id = state.tabs.alloc_id();
        let mut placeholder = Document::from_path(id, path.clone(), String::new());
        placeholder.loading = true;
        state.tabs.open_document(placeholder);
        state.pending.push(PendingLoad {
            document_id: id,
            path: path.clone(),
        });
        state.tabs.open_untitled();
        let _ = state
            .tabs
            .move_tab(state.tabs.index_of_id(id).expect("placeholder"), 0);
        let result = OpenResult::new(path.clone(), "hello from async".into(), 16, 1);
        state.apply_open_result(result);
        let doc = state.tabs.get_by_id(id).expect("doc by id");
        assert!(!doc.loading);
        assert_eq!(doc.buffer.to_string(), "hello from async");
        assert_eq!(doc.id, id);
    }

    #[test]
    fn pending_load_does_not_replace_wrong_tab_after_close() {
        let mut state = EditorState::new();
        let path_a = PathBuf::from("pending-a.txt");
        let path_b = PathBuf::from("pending-b.txt");
        let id_a = state.tabs.alloc_id();
        let mut a = Document::from_path(id_a, path_a.clone(), String::new());
        a.loading = true;
        state.tabs.open_document(a);
        state.pending.push(PendingLoad {
            document_id: id_a,
            path: path_a.clone(),
        });
        let id_b = state.tabs.alloc_id();
        let mut b = Document::from_path(id_b, path_b.clone(), "other".into());
        b.loading = false;
        state.tabs.open_document(b);
        let idx_a = state.tabs.index_of_id(id_a).expect("a");
        state.close_tab(idx_a);
        assert!(state.pending.iter().all(|p| p.document_id != id_a));
        state.apply_open_result(OpenResult::new(path_a, "late content".into(), 12, 1));
        assert!(state.tabs.get_by_id(id_a).is_none());
        let doc_b = state.tabs.get_by_id(id_b).expect("b");
        assert_eq!(doc_b.buffer.to_string(), "other");
        assert!(state.status.contains("tab was closed"));
    }

    #[test]
    fn pending_load_drops_when_id_no_longer_loading() {
        let mut state = EditorState::new();
        let path = PathBuf::from("pending-not-loading.txt");
        let id = state.tabs.alloc_id();
        let mut placeholder = Document::from_path(id, path.clone(), "keep".into());
        placeholder.loading = false;
        state.tabs.open_document(placeholder);
        state.pending.push(PendingLoad {
            document_id: id,
            path: path.clone(),
        });
        state.apply_open_result(OpenResult::new(path, "should not apply".into(), 16, 1));
        let doc = state.tabs.get_by_id(id).expect("doc");
        assert_eq!(doc.buffer.to_string(), "keep");
        assert!(state.status.contains("tab was closed"));
    }
    #[test]
    fn opening_log_asks_to_tail_by_default() {
        let dir = std::env::temp_dir().join("npp-rs-log-ask-test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("demo-app.log");
        std::fs::write(&path, b"line1\n").expect("write temp log");

        let mut state = EditorState::new();
        state.settings.log_tail_on_open = LogTailOnOpen::Ask;
        state.apply_open_result(OpenResult::new(path.clone(), "line1\n".into(), 6, 1));
        assert!(state.pending_log_tail_prompt);
        assert!(!state.tabs.active().tail_follow);

        state.resolve_log_tail_prompt(true, false);
        assert!(!state.pending_log_tail_prompt);
        assert!(state.tabs.active().tail_follow);
        assert_eq!(state.settings.log_tail_on_open, LogTailOnOpen::Ask);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn opening_log_always_enables_tail_without_prompt() {
        let mut state = EditorState::new();
        state.settings.log_tail_on_open = LogTailOnOpen::Always;
        state.apply_open_result(OpenResult::new(
            PathBuf::from("always.log"),
            "x\n".into(),
            2,
            1,
        ));
        assert!(!state.pending_log_tail_prompt);
        // enable_tail_follow needs a real file on disk — without it, follow stays off.
        // Policy still must not queue the ask dialog.
    }

    #[test]
    fn opening_log_never_skips_prompt() {
        let mut state = EditorState::new();
        state.settings.log_tail_on_open = LogTailOnOpen::Never;
        state.apply_open_result(OpenResult::new(
            PathBuf::from("skip.log"),
            "x\n".into(),
            2,
            1,
        ));
        assert!(!state.pending_log_tail_prompt);
        assert!(!state.tabs.active().tail_follow);
    }

    #[test]
    fn mark_text_changed_at_dirties_other_tab() {
        let mut state = EditorState::new();
        state.tabs.open_untitled();
        let other = state.tabs.len() - 1;
        state.tabs.set_active(0);
        state.highlight_dirty = false;
        assert!(!state.tabs.get(other).unwrap().dirty);
        if let Some(doc) = state.tabs.get_mut(other) {
            doc.buffer.insert("x");
        }
        state.mark_text_changed_at(other);
        assert!(state.tabs.get(other).unwrap().dirty);
        assert!(!state.tabs.active().dirty);
        assert!(!state.highlight_dirty);
    }

    #[test]
    fn other_pane_highlight_tracks_tab_and_edits() {
        let mut state = EditorState::new();
        state.tabs.open_untitled();
        let other = state.tabs.len() - 1;
        state.tabs.set_active(0);
        {
            let doc = state.tabs.get_mut(other).unwrap();
            doc.buffer = buffer::TextBuffer::from_str("fn hello() {}\n");
            doc.language = "rust".into();
        }
        state.highlight_dirty = false;
        state.refresh_highlight_other_if_needed(other, 0);
        assert_eq!(state.highlight_tab_other, Some(other));
        assert!(!state.highlight_dirty_other);
        assert!(
            !state.highlight_cache_other.is_empty(),
            "rust other-pane window should produce spans"
        );
        if let Some(doc) = state.tabs.get_mut(other) {
            doc.buffer.insert(" ");
        }
        state.mark_text_changed_at(other);
        assert!(state.highlight_dirty_other);
        assert!(!state.highlight_dirty);
        state.refresh_highlight_other_if_needed(other, 0);
        assert!(!state.highlight_dirty_other);
    }

    #[test]
    fn highlight_window_follows_view_line() {
        let mut state = EditorState::new();
        let mut body = String::new();
        for i in 0..800 {
            body.push_str(&format!("fn f{i}() {{ let x = {i}; }}\n"));
        }
        {
            let doc = state.tabs.active_mut();
            doc.buffer = buffer::TextBuffer::from_str(&body);
            doc.language = "rust".into();
        }
        state.highlight_dirty = true;
        state.refresh_highlight_if_needed(600);
        let (lo, hi) = state.highlight_cover;
        assert!(lo > 0, "cover should not stay at file start for deep view");
        assert!(lo <= 600);
        assert!(hi > 600);
        assert!(!state.highlight_cache.is_empty());
        let start_char = state.tabs.active().buffer.line_to_char(lo);
        assert!(state.highlight_cache.iter().any(|s| s.start >= start_char));
    }

    #[test]
    fn highlight_skips_when_view_inside_cover() {
        let mut state = EditorState::new();
        {
            let doc = state.tabs.active_mut();
            doc.buffer = buffer::TextBuffer::from_str("fn a() {}\nfn b() {}\n");
            doc.language = "rust".into();
        }
        state.highlight_dirty = true;
        state.refresh_highlight_if_needed(0);
        assert!(!state.highlight_dirty);
        let first_len = state.highlight_cache.len();
        state.refresh_highlight_if_needed(0);
        assert_eq!(state.highlight_cache.len(), first_len);
    }

    #[test]
    fn read_only_blocks_edit_command_and_toggle_still_works() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer.insert("hello");
        state.mark_text_changed();
        let before = state.tabs.active().buffer.to_string();
        state.tabs.active_mut().read_only = true;
        let mut ui = crate::commands::UiFlags::default();
        assert_eq!(
            crate::commands::dispatch("IDM_EDIT_DELETE", &mut state, &mut ui),
            crate::commands::CmdResult::Handled
        );
        assert_eq!(state.tabs.active().buffer.to_string(), before);
        assert_eq!(state.status, "Document is read-only");
        assert_eq!(
            crate::commands::dispatch("IDM_EDIT_TOGGLEREADONLY", &mut state, &mut ui),
            crate::commands::CmdResult::Handled
        );
        assert!(!state.tabs.active().read_only);
        state.tabs.active_mut().buffer.select_all();
        assert_eq!(
            crate::commands::dispatch("IDM_EDIT_DELETE", &mut state, &mut ui),
            crate::commands::CmdResult::Handled
        );
        assert_ne!(state.tabs.active().buffer.to_string(), before);
    }

    #[test]
    fn undo_at_targets_named_tab() {
        let mut state = EditorState::new();
        state.tabs.open_untitled();
        let other = state.tabs.len() - 1;
        state.tabs.set_active(0);
        if let Some(doc) = state.tabs.get_mut(other) {
            doc.buffer.insert("hello");
        }
        state.mark_text_changed_at(other);
        assert_eq!(state.tabs.get(other).unwrap().buffer.to_string(), "hello");
        state.undo_at(other);
        assert_eq!(state.tabs.get(other).unwrap().buffer.to_string(), "");
        assert_eq!(state.tabs.active().buffer.to_string(), "");
    }

    #[test]
    fn mark_text_changed_sets_compare_stale() {
        let mut state = EditorState::new();
        assert!(!state.compare_stale);
        state.tabs.active_mut().buffer.insert("a");
        state.mark_text_changed();
        assert!(state.compare_stale);
    }

    #[test]
    fn reload_replaces_document_text() {
        let dir = std::env::temp_dir().join("npp-rs-reload-test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("reload-demo.txt");
        std::fs::write(&path, b"on disk v1\n").expect("write");
        let mut state = EditorState::new();
        state.apply_open_result(OpenResult::new(path.clone(), "on disk v1\n".into(), 11, 1));
        assert_eq!(state.tabs.active().buffer.to_string(), "on disk v1\n");
        state.tabs.active_mut().buffer.insert("edited ");
        state.mark_text_changed();
        assert!(state.tabs.active().dirty);
        state.request_reload();
        assert!(state.pending_reload.is_some());
        assert!(state.tabs.active().buffer.to_string().contains("edited"));
        state.confirm_reload_discard();
        assert_eq!(state.tabs.active().buffer.to_string(), "on disk v1\n");
        assert!(!state.tabs.active().dirty);
        std::fs::write(&path, b"on disk v2\n").expect("rewrite");
        state.request_reload();
        assert_eq!(state.tabs.active().buffer.to_string(), "on disk v2\n");
        assert!(!state.tabs.active().dirty);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn undo_to_saved_clears_dirty_flag() {
        let mut state = EditorState::new();
        {
            let doc = state.tabs.active_mut();
            doc.path = Some(PathBuf::from("mem-save-rev.txt"));
            doc.buffer.insert("base");
        }
        state.mark_text_changed();
        assert!(state.tabs.active().dirty);
        state.tabs.active_mut().mark_clean();
        assert!(!state.tabs.active().dirty);
        state.tabs.active_mut().buffer.insert("!");
        state.mark_text_changed();
        assert!(state.tabs.active().dirty);
        state.undo();
        assert!(!state.tabs.active().dirty);
        assert_eq!(state.tabs.active().buffer.to_string(), "base");
        state.redo();
        assert!(state.tabs.active().dirty);
    }

    #[test]
    fn undo_remaps_change_history_without_restamp() {
        let mut state = EditorState::new();
        {
            let doc = state.tabs.active_mut();
            let text: String = (0..6).map(|i| format!("L{i}\n")).collect();
            doc.buffer = buffer::TextBuffer::from_str(&text);
            doc.line_mark_basis = doc.buffer.line_count();
            doc.note_line_changed(4);
        }
        state.tabs.active_mut().buffer.set_caret(0);
        state.prepare_edit();
        state.tabs.active_mut().buffer.insert("\n\n");
        state.mark_text_changed();
        assert!(state.tabs.active().changed_unsaved.contains(&6));
        let before_undo: Vec<usize> = state
            .tabs
            .active()
            .changed_unsaved
            .iter()
            .copied()
            .collect();
        state.undo();
        // Remap brings mark 6 → 4; undo must not drop the remapped mark.
        assert!(state.tabs.active().changed_unsaved.contains(&4));
        assert!(!state.tabs.active().changed_unsaved.contains(&6));
        // Undo path does not add a brand-new stamp beyond remapped set size growth.
        assert!(state.tabs.active().changed_unsaved.len() <= before_undo.len());
    }

    #[test]
    fn find_replace_respects_selection_scope() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("foo bar foo");
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = false;
        state.settings.find_in_selection = true;
        state.tabs.active_mut().buffer.set_selection(0, 7);
        state.capture_find_scope();
        assert_eq!(state.find_match_count(), 1);
        state.find_next();
        assert_eq!(state.tabs.active().buffer.selection(), Some((0, 3)));
        state.replace_all("X");
        assert_eq!(state.tabs.active().buffer.to_string(), "X bar foo");
        assert!(state.status.contains("1 replacement"));
    }

    #[test]
    fn regex_replace_expands_capture_groups() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("ab cd");
        state.find_query = r"(\w)(\w)".into();
        state.settings.find_regex = true;
        state.settings.find_wrap = false;
        state.replace_all("$2$1");
        assert_eq!(state.tabs.active().buffer.to_string(), "ba dc");
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("xy");
        state.find_next();
        state.replace_next(r"\1-\2");
        assert_eq!(state.tabs.active().buffer.to_string(), "x-y");
    }

    #[test]
    fn find_wrap_off_stops_at_end() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("foo bar foo");
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_wrap = false;
        state.tabs.active_mut().buffer.set_caret(0);
        state.find_next();
        assert_eq!(state.tabs.active().buffer.selection(), Some((0, 3)));
        assert_eq!(state.status, "Find: 1/2");
        state.find_next();
        assert_eq!(state.tabs.active().buffer.selection(), Some((8, 11)));
        assert_eq!(state.status, "Find: 2/2");
        state.find_next();
        assert_eq!(state.tabs.active().buffer.selection(), Some((8, 11)));
        assert_eq!(state.status, "Find: passed end of file");
        state.find_prev();
        assert_eq!(state.tabs.active().buffer.selection(), Some((0, 3)));
        assert_eq!(state.status, "Find: 1/2");
        state.find_prev();
        assert_eq!(state.tabs.active().buffer.selection(), Some((0, 3)));
        assert_eq!(state.status, "Find: passed beginning of file");
    }

    #[test]
    fn find_bar_multiline_selection_arms_sel_scope() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("foo\nbar\nfoo\nzzz");
        state.find_query = "needle".into();
        state.tabs.active_mut().buffer.set_selection(0, 11); // foo\nbar\nfoo
        state.prepare_find_bar_from_selection();
        assert!(state.settings.find_in_selection);
        assert_eq!(state.find_scope, Some((0, 11)));
        assert_eq!(state.find_query, "needle");
        state.find_query = "foo".into();
        assert_eq!(state.find_match_count(), 2);
        state.settings.find_wrap = false;
        state.tabs.active_mut().buffer.set_caret(0);
        state.find_next();
        assert_eq!(state.tabs.active().buffer.selection(), Some((0, 3)));
        state.find_next();
        assert_eq!(state.tabs.active().buffer.selection(), Some((8, 11)));
        state.find_next();
        assert_eq!(state.status, "Find: passed end of selection");
    }

    #[test]
    fn find_bar_single_line_selection_seeds_query() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("alpha beta");
        state.tabs.active_mut().buffer.set_selection(0, 5);
        state.prepare_find_bar_from_selection();
        assert!(!state.settings.find_in_selection);
        assert_eq!(state.find_query, "alpha");
        assert!(state.find_scope.is_none());
    }

    #[test]
    fn find_sel_without_scope_counts_zero() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("foo foo");
        state.find_query = "foo".into();
        state.settings.find_in_selection = true;
        assert_eq!(state.find_match_count(), 0);
    }

    #[test]
    fn mark_all_style_respects_case_and_word() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar");
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        state.mark_all_style(1, true);
        let marks = &state.tabs.active().style_marks[0];
        assert_eq!(marks.iter().copied().collect::<Vec<_>>(), vec![1]);
        assert!(state.status.contains("Styled 1 line"));
    }

    #[test]
    fn mark_all_style_regex_and_sel_bounds() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("a12\nb3\nc99\nd");
        state.find_query = r"\d+".into();
        state.settings.find_regex = true;
        state.settings.find_in_selection = true;
        // Select first two lines only (a12\nb3\n).
        state.find_scope = Some((0, 7));
        state.mark_all_style(2, true);
        let marks = &state.tabs.active().style_marks[1];
        assert_eq!(marks.iter().copied().collect::<Vec<_>>(), vec![0, 1]);
    }

    #[test]
    fn mark_all_style_selection_token_skips_find_query() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("alpha\nbeta alpha\ngamma");
        state.find_query = "gamma".into();
        state.tabs.active_mut().buffer.set_selection(0, 5); // alpha
        state.mark_all_style(1, false);
        let marks = &state.tabs.active().style_marks[0];
        assert_eq!(marks.iter().copied().collect::<Vec<_>>(), vec![0, 1]);
    }

    #[test]
    fn bookmark_find_matches_respects_case_word_and_keeps_existing() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer =
            buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar\nfoo");
        // Line 1 already bookmarked and is a match; line 0 is not a whole-word match.
        state.tabs.active_mut().bookmarks.insert(1);
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let added = state.bookmark_find_matches().unwrap();
        assert_eq!(added, 1);
        let bms: Vec<_> = state.tabs.active().bookmarks.iter().copied().collect();
        assert_eq!(bms, vec![1, 3]);
        assert!(state.status.contains("Bookmarked 2 matching"));
        assert!(state.status.contains("1 new"));
    }

    #[test]
    fn bookmark_find_matches_regex_and_sel_bounds() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("a12\nb3\nc99\nd");
        state.find_query = r"\d+".into();
        state.settings.find_regex = true;
        state.settings.find_in_selection = true;
        state.find_scope = Some((0, 7)); // a12\nb3\n
        let added = state.bookmark_find_matches().unwrap();
        assert_eq!(added, 2);
        let bms: Vec<_> = state.tabs.active().bookmarks.iter().copied().collect();
        assert_eq!(bms, vec![0, 1]);
    }

    #[test]
    fn copy_find_matches_respects_case_word() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer =
            buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar\nfoo");
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let text = state.copy_find_matches().unwrap();
        assert_eq!(text, "foo\nfoo");
        assert!(state.status.contains("Copied 2"));
    }

    #[test]
    fn copy_find_matches_regex_and_sel_bounds() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("a12\nb3\nc99\nd");
        state.find_query = r"\d+".into();
        state.settings.find_regex = true;
        state.settings.find_in_selection = true;
        state.find_scope = Some((0, 7)); // a12\nb3\n
        let text = state.copy_find_matches().unwrap();
        assert_eq!(text, "12\n3");
        assert!(state.status.contains("Copied 2"));
    }

    #[test]
    fn copy_find_match_lines_respects_case_word() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer =
            buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar\nfoo");
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let text = state.copy_find_match_lines().unwrap();
        assert_eq!(text, "foo\nfoo");
        assert!(state.status.contains("Copied 2 matching line"));
    }

    #[test]
    fn copy_find_match_lines_dedupes_and_sel_bounds() {
        let mut state = EditorState::new();
        // Two matches on line 0; Sel excludes line 2's "99".
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("a12 x12\nb3\nc99\nd");
        state.find_query = r"\d+".into();
        state.settings.find_regex = true;
        state.settings.find_in_selection = true;
        state.find_scope = Some((0, 11)); // a12 x12\nb3\n
        let text = state.copy_find_match_lines().unwrap();
        assert_eq!(text, "a12 x12\nb3");
        assert!(state.status.contains("Copied 2 matching line"));
    }

    #[test]
    fn cut_find_matches_respects_case_word() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer =
            buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar\nfoo");
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let text = state.cut_find_matches().unwrap();
        assert_eq!(text, "foo\nfoo");
        assert_eq!(
            state.tabs.active().buffer.to_string(),
            "Foo food\n\nFOO bar\n"
        );
        assert!(state.status.contains("Cut 2"));
    }

    #[test]
    fn cut_find_matches_regex_and_sel_bounds() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("a12\nb3\nc99\nd");
        state.find_query = r"\d+".into();
        state.settings.find_regex = true;
        state.settings.find_in_selection = true;
        state.find_scope = Some((0, 7)); // a12\nb3\n
        let text = state.cut_find_matches().unwrap();
        assert_eq!(text, "12\n3");
        assert_eq!(state.tabs.active().buffer.to_string(), "a\nb\nc99\nd");
        assert!(state.status.contains("Cut 2"));
        assert_eq!(state.find_scope, Some((0, 4))); // a\nb\n
    }

    #[test]
    fn cut_find_match_lines_respects_case_word() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer =
            buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar\nfoo");
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let text = state.cut_find_match_lines().unwrap();
        assert_eq!(text, "foo\nfoo");
        assert_eq!(
            state.tabs.active().buffer.to_string(),
            "Foo food\nFOO bar\n"
        );
        assert!(state.status.contains("Cut 2 matching line"));
    }

    #[test]
    fn cut_find_match_lines_dedupes_and_sel_bounds() {
        let mut state = EditorState::new();
        // Two matches on line 0; Sel excludes line 2's "99".
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("a12 x12\nb3\nc99\nd");
        state.find_query = r"\d+".into();
        state.settings.find_regex = true;
        state.settings.find_in_selection = true;
        state.find_scope = Some((0, 11)); // a12 x12\nb3\n
        let text = state.cut_find_match_lines().unwrap();
        assert_eq!(text, "a12 x12\nb3");
        assert_eq!(state.tabs.active().buffer.to_string(), "c99\nd");
        assert!(state.status.contains("Cut 2 matching line"));
        assert_eq!(state.find_scope, Some((0, 0)));
    }

    #[test]
    fn delete_find_match_lines_respects_case_word() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer =
            buffer::TextBuffer::from_str("Foo food\nfoo\nFOO bar\nfoo");
        state.find_query = "foo".into();
        state.settings.find_match_case = true;
        state.settings.find_whole_word = true;
        let n = state.delete_find_match_lines().unwrap();
        assert_eq!(n, 2);
        assert_eq!(
            state.tabs.active().buffer.to_string(),
            "Foo food\nFOO bar\n"
        );
        assert!(state.status.contains("Deleted 2 matching"));
    }

    #[test]
    fn delete_find_match_lines_regex_and_sel_bounds() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("a12\nb3\nc99\nd");
        state.find_query = r"\d+".into();
        state.settings.find_regex = true;
        state.settings.find_in_selection = true;
        state.find_scope = Some((0, 7)); // a12\nb3\n
        let n = state.delete_find_match_lines().unwrap();
        assert_eq!(n, 2);
        assert_eq!(state.tabs.active().buffer.to_string(), "c99\nd");
        assert!(state.status.contains("Deleted 2 matching"));
        assert_eq!(state.find_scope, Some((0, 0)));
    }

    #[test]
    fn find_regex_next_and_invalid() {
        let mut state = EditorState::new();
        state.tabs.active_mut().buffer = buffer::TextBuffer::from_str("a12 b3");
        state.find_query = r"\d+".into();
        state.settings.find_regex = true;
        state.settings.find_wrap = false;
        assert_eq!(state.find_match_count(), 2);
        state.find_next();
        assert_eq!(state.tabs.active().buffer.selection(), Some((1, 3)));
        state.find_next();
        assert_eq!(state.tabs.active().buffer.selection(), Some((5, 6)));
        state.find_query = "[".into();
        state.find_next();
        assert_eq!(state.status, "Find: invalid regex");
    }

    #[test]
    fn project_panel_entries_filter_and_order() {
        let dir = std::env::temp_dir().join("npp-rs-project-panel-list");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mkdir");
        std::fs::create_dir_all(dir.join("src")).expect("mkdir src");
        std::fs::write(dir.join("readme.md"), b"x").expect("write");
        std::fs::write(dir.join("notes.txt"), b"y").expect("write");
        std::fs::write(dir.join(".hidden"), b"z").expect("write hidden");

        let all = EditorState::list_project_panel_entries(&dir, "");
        let names: Vec<String> = all
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec!["src", "notes.txt", "readme.md"]);
        assert!(!names.iter().any(|n| n.starts_with('.')));

        let filtered = EditorState::list_project_panel_entries(&dir, "READ");
        assert_eq!(filtered.len(), 1);
        assert_eq!(
            filtered[0].file_name().and_then(|n| n.to_str()),
            Some("readme.md")
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ask_ollama_empty_buffer_does_not_start_worker() {
        let mut state = EditorState::new();
        state.ask_ollama();
        assert!(!state.ollama_busy);
        assert!(state.status.contains("empty"));
    }

    #[test]
    fn poll_ollama_applies_ask_done() {
        let mut state = EditorState::new();
        state.ollama_busy = true;
        state
            .ollama_channel
            .tx
            .send(crate::ollama::OllamaMsg::AskDone {
                model: "test-model".into(),
                host: "http://127.0.0.1:11434".into(),
                scope: "selection".into(),
                cut: false,
                reply: "hello from worker".into(),
            })
            .expect("send");
        assert!(state.poll_ollama());
        assert!(!state.ollama_busy);
        assert!(state.status.contains("test-model"));
        let active = state.tabs.active();
        assert_eq!(active.title, "Ollama reply");
        assert!(active.buffer.to_string().contains("hello from worker"));
        assert!(active.read_only);
    }
}
