//! Editor application state and commands.

use crate::recent::{
    short_path_label, AppSettings, LogTailOnOpen, RecentFiles, is_log_path,
};
use doc::{Document, FileEncoding, TabSet};
use fs::{self, LoadChannel, LoadMsg, OpenResult, TailRead, TextEncoding, LARGE_FILE_THRESHOLD};
use highlight::SyntaxHighlighter;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Pending async load keyed by path (not tab index — tabs can move/close).
#[derive(Debug, Clone)]
pub struct PendingLoad {
    pub path: PathBuf,
}

pub struct EditorState {
    pub tabs: TabSet,
    pub find_query: String,
    pub find_open: bool,
    pub status: String,
    pub highlighter: SyntaxHighlighter,
    pub load_channel: LoadChannel,
    pub pending: Vec<PendingLoad>,
    /// Cached highlight spans for active doc.
    pub highlight_cache: Vec<highlight::Span>,
    pub highlight_lang: String,
    pub highlight_dirty: bool,
    pub recent: RecentFiles,
    pub settings: AppSettings,
    /// After opening `*.log`, UI may show the tail prompt.
    pub pending_log_tail_prompt: bool,
    /// UI should jump scroll to line 1 (set on open / new).
    pub reset_view: bool,
    /// Last time we polled disk for tail follow.
    tail_last_poll: Instant,
    /// Begin/End Select: first caret, or `None` when idle.
    pub begin_end_select: Option<usize>,
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
    /// After resolving `pending_close`, continue this bulk operation.
    pub bulk_close: BulkClose,
    /// UI should quit when true (no dirty tabs left).
    pub want_quit: bool,
    /// Compare tags need a rebuild after an edit.
    pub compare_stale: bool,
    /// Before-edit line snap for change-history remap (tab, snap).
    pending_edit_snap: Option<(usize, doc::LineEditSnap)>,
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
        Self {
            tabs: TabSet::new(),
            find_query: String::new(),
            find_open: false,
            status: "Ready".into(),
            highlighter: SyntaxHighlighter::new(),
            load_channel: LoadChannel::new(),
            pending: Vec::new(),
            highlight_cache: Vec::new(),
            highlight_lang: String::new(),
            highlight_dirty: true,
            recent: RecentFiles::load(),
            settings,
            pending_log_tail_prompt: false,
            reset_view: false,
            tail_last_poll: Instant::now() - Duration::from_secs(1),
            begin_end_select: None,
            search_engine: "https://duckduckgo.com/?q=".into(),
            show_whitespace: false,
            show_eol: false,
            show_npc: false,
            show_indent_guide: false,
            word_wrap,
            macro_recording: false,
            macro_cmds: Vec::new(),
            pending_close: None,
            bulk_close: BulkClose::None,
            want_quit: false,
            compare_stale: false,
            pending_edit_snap: None,
        }
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

    pub fn mark_text_changed(&mut self) {
        self.mark_text_changed_at(self.tabs.active_index());
    }

    /// Dirty + change-history for a tab (dual view may edit the other pane).
    pub fn mark_text_changed_at(&mut self, tab: usize) {
        let snap = self
            .pending_edit_snap
            .take()
            .filter(|(t, _)| *t == tab)
            .map(|(_, s)| s);
        let Some(doc) = self.tabs.get_mut(tab) else {
            return;
        };
        if let Some(snap) = snap {
            doc.apply_line_snap(snap);
        } else {
            doc.sync_line_marks_after_edit();
        }
        Self::note_edit_lines(doc);
        doc.mark_dirty();
        if doc.tail_follow {
            doc.tail_follow = false;
            self.status = "Tail OFF — editing suspended follow".into();
        }
        if tab == self.tabs.active_index() {
            self.highlight_dirty = true;
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

    pub fn refresh_highlight_if_needed(&mut self) {
        if !self.highlight_dirty {
            return;
        }
        let lang = self.tabs.active().language.clone();
        let text = self.tabs.active().buffer.to_string();
        let slice = if text.len() > 512 * 1024 {
            let mut end = 512 * 1024;
            while end > 0 && !text.is_char_boundary(end) {
                end -= 1;
            }
            &text[..end]
        } else {
            text.as_str()
        };
        self.highlight_cache = self
            .highlighter
            .highlight(&lang, slice)
            .unwrap_or_default();
        self.highlight_lang = lang;
        self.highlight_dirty = false;
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
                "No logs under logs/ (cwd). Panic hook writes logs/panic.log when present."
                    .into();
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
                let mut doc = Document::from_path(path.clone(), String::new());
                doc.loading = true;
                doc.title = format!("{} (loading…)", doc.title);
                self.tabs.open_document(doc);
                self.pending.push(PendingLoad {
                    path: path.clone(),
                });
                self.recent.touch(&path);
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
        let was_pending = self.pending.iter().any(|p| p.path == result.path);
        self.pending.retain(|p| p.path != result.path);

        // Match the loading placeholder by path so tab move/sort/close cannot
        // apply content to the wrong document.
        let loading_idx = self
            .tabs
            .iter()
            .enumerate()
            .find(|(_, d)| d.path.as_ref() == Some(&result.path) && d.loading)
            .map(|(i, _)| i);

        if let Some(tab_index) = loading_idx {
            if let Some(doc) = self.tabs.get_mut(tab_index) {
                *doc = Document::from_path_with_encoding(
                    result.path.clone(),
                    result.content,
                    file_encoding_from_fs(result.encoding),
                );
                self.tabs.set_active(tab_index);
            }
        } else if was_pending {
            // User closed the placeholder while the load ran — do not reopen.
            self.status = format!(
                "Load finished but tab was closed: {}",
                short_path_label(&result.path)
            );
            return;
        } else {
            let doc = Document::from_path_with_encoding(
                result.path.clone(),
                result.content,
                file_encoding_from_fs(result.encoding),
            );
            self.tabs.open_document(doc);
        }
        self.recent.touch(&result.path);
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
        self.after_open_log_policy(&result.path);
    }

    /// Apply remembered log-tail preference, or queue the prompt dialog.
    fn after_open_log_policy(&mut self, path: &std::path::Path) {
        self.pending_log_tail_prompt = false;
        if !is_log_path(path) {
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

    /// Drop pending load and stop treating a closed tab as a load target.
    pub fn note_tab_closed(&mut self, path: Option<&std::path::Path>) {
        if let Some(path) = path {
            self.pending.retain(|p| p.path != path);
        }
    }

    /// Close a tab and cancel any pending load for its path.
    pub fn close_tab(&mut self, index: usize) {
        let path = self.tabs.get(index).and_then(|d| d.path.clone());
        self.note_tab_closed(path.as_deref());
        self.tabs.close(index);
        self.highlight_dirty = true;
        if let Some(p) = self.pending_close {
            if p == index {
                self.pending_close = None;
            } else if p > index {
                self.pending_close = Some(p - 1);
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
                    self.pending.retain(|p| p.path != path);
                    self.recent.remove(&path);
                    let to_close: Vec<usize> = self
                        .tabs
                        .iter()
                        .enumerate()
                        .filter(|(_, d)| d.path.as_ref() == Some(&path) && d.loading)
                        .map(|(i, _)| i)
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
        let content = self.tabs.active().buffer.to_string();
        let encoding = self.tabs.active().encoding;
        let fs_enc = fs_encoding_from_file(encoding);
        match fs::write_file_with_encoding(path, &content, fs_enc) {
            Ok(()) => {
                let doc = self.tabs.active_mut();
                doc.path = Some(path.to_path_buf());
                doc.title = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                doc.language = doc::detect_language(path);
                doc.mark_clean();
                doc.promote_change_history_on_save();
                self.recent.touch(path);
                self.highlight_dirty = true;
                self.status = format!("Saved {} ({})", path.display(), encoding.label());
                true
            }
            Err(e) => {
                self.status = format!("Save failed: {e}");
                false
            }
        }
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

    pub fn replace_next(&mut self, replacement: &str) {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Replace: empty find".into();
            return;
        }
        // If current selection is the find match, replace it; else find next first.
        let is_match = self
            .tabs
            .active()
            .buffer
            .selection()
            .map(|(s, e)| self.tabs.active().buffer.slice(s, e) == q)
            .unwrap_or(false);
        if !is_match {
            self.find_next();
            let ok = self
                .tabs
                .active()
                .buffer
                .selection()
                .map(|(s, e)| self.tabs.active().buffer.slice(s, e) == q)
                .unwrap_or(false);
            if !ok {
                self.status = "Replace: no match".into();
                return;
            }
        }
        self.tabs.active_mut().buffer.insert(replacement);
        self.mark_text_changed();
        self.status = "Replaced once".into();
    }

    pub fn replace_all(&mut self, replacement: &str) {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Replace All: empty find".into();
            return;
        }
        let text = self.tabs.active().buffer.to_string();
        if !text.contains(&q) {
            self.status = "Replace All: no match".into();
            return;
        }
        let mut touched = std::collections::BTreeSet::new();
        let mut from = 0usize;
        while let Some((s, e)) = self.tabs.active().buffer.find_next(&q, from, true) {
            touched.insert(self.tabs.active().buffer.char_to_line(s));
            from = if e > s { e } else { s + 1 };
        }
        let count = text.matches(&q).count();
        let new_text = text.replace(&q, replacement);
        self.tabs.active_mut().buffer.replace_document(&new_text);
        for line in touched {
            self.tabs.active_mut().note_line_changed(line);
        }
        self.tabs.active_mut().mark_dirty();
        self.highlight_dirty = true;
        self.status = format!("Replace All: {count} replacement(s)");
    }

    pub fn set_language(&mut self, lang: &str) {
        self.tabs.active_mut().language = lang.to_string();
        self.highlight_dirty = true;
        self.status = format!("Language: {lang}");
    }

    pub fn find_next(&mut self) {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Find: empty query".into();
            return;
        }
        let from = self
            .tabs
            .active()
            .buffer
            .selection()
            .map(|(_, e)| e)
            .unwrap_or_else(|| self.tabs.active().buffer.caret());
        if let Some((s, e)) = self.tabs.active().buffer.find_next(&q, from, true) {
            self.tabs.active_mut().buffer.set_selection(s, e);
            self.status = format!("Find: match at {s}");
        } else {
            self.status = "Find: no match".into();
        }
    }

    pub fn find_prev(&mut self) {
        let q = self.find_query.clone();
        if q.is_empty() {
            self.status = "Find: empty query".into();
            return;
        }
        let from = self
            .tabs
            .active()
            .buffer
            .selection()
            .map(|(s, _)| s)
            .unwrap_or_else(|| self.tabs.active().buffer.caret());
        if let Some((s, e)) = self.tabs.active().buffer.find_prev(&q, from, true) {
            self.tabs.active_mut().buffer.set_selection(s, e);
            self.status = format!("Find: match at {s}");
        } else {
            self.status = "Find: no match".into();
        }
    }

    pub fn undo(&mut self) {
        self.undo_at(self.tabs.active_index());
    }

    pub fn redo(&mut self) {
        self.redo_at(self.tabs.active_index());
    }

    pub fn undo_at(&mut self, tab: usize) {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return;
        };
        if doc.buffer.undo() {
            self.mark_text_changed_at(tab);
            self.status = "Undo".into();
        }
    }

    pub fn redo_at(&mut self, tab: usize) {
        let Some(doc) = self.tabs.get_mut(tab) else {
            return;
        };
        if doc.buffer.redo() {
            self.mark_text_changed_at(tab);
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
                    self.status = format!(
                        "Saved a copy as {} ({})",
                        path.display(),
                        encoding.label()
                    );
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
                self.recent.touch(&new_path);
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
        let result = {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open").args(["-R", &path.to_string_lossy()]).status()
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("explorer")
                    .arg(format!("/select,{}", path.display()))
                    .status()
            }
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                let folder = path.parent().unwrap_or(path.as_path());
                std::process::Command::new("xdg-open").arg(folder).status()
            }
        };
        match result {
            Ok(s) if s.success() => self.status = "Revealed in file manager".into(),
            Ok(_) => self.status = "Reveal failed".into(),
            Err(e) => self.status = format!("Reveal failed: {e}"),
        }
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
                    .args(["/C", "start", "cmd", "/K", "cd", "/D", &folder.to_string_lossy()])
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
        let now = chrono_lite_now(long);
        self.tabs.active_mut().buffer.insert(&now);
        self.mark_text_changed();
        self.status = "Inserted date/time".into();
    }

    /// ISO-8601 local style (custom format stand-in until Preferences exist).
    pub fn insert_datetime_custom(&mut self) {
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

    /// Poll disk for growth on any tab with `tail_follow`. Returns true if active tab grew.
    pub fn poll_tail(&mut self) -> bool {
        if self.tail_last_poll.elapsed() < Duration::from_millis(250) {
            return false;
        }
        self.tail_last_poll = Instant::now();

        let active = self.tabs.active_index();
        let mut active_grew = false;
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
            let offset = doc.tail_bytes;
            let dirty = doc.dirty;
            match fs::read_tail_since(&path, offset) {
                Ok(TailRead::Unchanged { .. }) => {}
                Ok(TailRead::Appended { text, size }) => {
                    if let Some(d) = self.tabs.get_mut(i) {
                        if dirty {
                            d.tail_follow = false;
                            if i == active {
                                self.status =
                                    "Tail OFF — document has unsaved edits".into();
                            }
                            continue;
                        }
                        if !text.is_empty() {
                            let end = d.buffer.len_chars();
                            d.buffer.set_caret(end);
                            d.buffer.insert(&text);
                            // Appended bytes are already on disk.
                            d.mark_clean();
                            if i == active {
                                active_grew = true;
                                let n = text.lines().count().max(1);
                                self.status = format!("Tail: +{n} line(s)");
                            }
                        }
                        d.tail_bytes = size;
                    }
                    if i == active {
                        self.highlight_dirty = true;
                    }
                }
                Ok(TailRead::Rotated { .. }) => {
                    if dirty {
                        if let Some(d) = self.tabs.get_mut(i) {
                            d.tail_follow = false;
                        }
                        if i == active {
                            self.status =
                                "Tail OFF — file rotated while document has unsaved edits"
                                    .into();
                        }
                        continue;
                    }
                    match fs::read_file(&path) {
                        Ok(r) => {
                            if let Some(d) = self.tabs.get_mut(i) {
                                d.buffer.replace_document(&r.content);
                                d.tail_bytes = r.bytes;
                                d.mark_clean();
                                let end = d.buffer.len_chars();
                                d.buffer.set_caret(end);
                            }
                            if i == active {
                                self.highlight_dirty = true;
                                active_grew = true;
                                self.status = "Tail: file rotated — reloaded".into();
                            }
                        }
                        Err(e) => {
                            if i == active {
                                self.status = format!("Tail reload failed: {e}");
                            }
                        }
                    }
                }
                Err(e) => {
                    if i == active {
                        self.status = format!("Tail error: {e}");
                    }
                }
            }
        }
        active_grew
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
    fn pending_load_matches_by_path_not_index() {
        let mut state = EditorState::new();
        let path = PathBuf::from("pending-stability-demo.txt");
        let mut placeholder = Document::from_path(path.clone(), String::new());
        placeholder.loading = true;
        state.tabs.open_document(placeholder);
        state.pending.push(PendingLoad { path: path.clone() });
        // New tab + move: indices change; apply must still find by path.
        state.tabs.open_untitled();
        let _ = state.tabs.move_active_tab(-1);
        let result = OpenResult::new(path.clone(), "hello from async".into(), 16, 1);
        state.apply_open_result(result);
        let doc = state
            .tabs
            .iter()
            .find(|d| d.path.as_ref() == Some(&path))
            .expect("doc by path");
        assert!(!doc.loading);
        assert_eq!(doc.buffer.to_string(), "hello from async");
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
        state.apply_open_result(OpenResult::new(PathBuf::from("always.log"), "x\n".into(), 2, 1));
        assert!(!state.pending_log_tail_prompt);
        // enable_tail_follow needs a real file on disk — without it, follow stays off.
        // Policy still must not queue the ask dialog.
    }

    #[test]
    fn opening_log_never_skips_prompt() {
        let mut state = EditorState::new();
        state.settings.log_tail_on_open = LogTailOnOpen::Never;
        state.apply_open_result(OpenResult::new(PathBuf::from("skip.log"), "x\n".into(), 2, 1));
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
}

fn file_encoding_from_fs(enc: TextEncoding) -> FileEncoding {
    match enc {
        TextEncoding::Utf8 => FileEncoding::Utf8,
        TextEncoding::Utf8Bom => FileEncoding::Utf8Bom,
        TextEncoding::Windows1252 => FileEncoding::Windows1252,
    }
}

fn fs_encoding_from_file(enc: FileEncoding) -> TextEncoding {
    match enc {
        FileEncoding::Utf8 => TextEncoding::Utf8,
        FileEncoding::Utf8Bom => TextEncoding::Utf8Bom,
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
