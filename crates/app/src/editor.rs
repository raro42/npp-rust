//! Editor application state and commands.

use crate::recent::RecentFiles;
use doc::{Document, TabSet};
use fs::{self, LoadChannel, LoadMsg, OpenResult, LARGE_FILE_THRESHOLD};
use highlight::SyntaxHighlighter;
use std::path::PathBuf;

/// Pending placeholder tab index waiting for async load.
#[derive(Debug, Clone)]
pub struct PendingLoad {
    pub tab_index: usize,
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
    /// UI should jump scroll to line 1 (set on open / new).
    pub reset_view: bool,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorState {
    pub fn new() -> Self {
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
            reset_view: false,
        }
    }

    pub fn mark_text_changed(&mut self) {
        self.tabs.active_mut().mark_dirty();
        self.highlight_dirty = true;
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

    pub fn open_path(&mut self, path: PathBuf) {
        if !path.exists() {
            self.recent.remove(&path);
            self.status = format!("File not found (removed from Recent): {}", path.display());
            return;
        }
        match fs::file_size(&path) {
            Ok(size) if size >= LARGE_FILE_THRESHOLD => {
                let mut doc = Document::from_path(path.clone(), String::new());
                doc.loading = true;
                doc.title = format!("{} (loading…)", doc.title);
                let idx = self.tabs.open_document(doc);
                self.pending.push(PendingLoad {
                    tab_index: idx,
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
        if let Some(pos) = self.pending.iter().position(|p| p.path == result.path) {
            let PendingLoad { tab_index, .. } = self.pending.remove(pos);
            if let Some(doc) = self.tabs.get_mut(tab_index) {
                *doc = Document::from_path(result.path.clone(), result.content);
                self.tabs.set_active(tab_index);
            } else {
                let doc = Document::from_path(result.path.clone(), result.content);
                self.tabs.open_document(doc);
            }
        } else {
            let doc = Document::from_path(result.path.clone(), result.content);
            self.tabs.open_document(doc);
        }
        self.recent.touch(&result.path);
        self.highlight_dirty = true;
        self.reset_view = true;
        // Ensure caret is at the top after open.
        self.tabs.active_mut().buffer.set_caret(0);
        self.status = format!(
            "Opened {} ({:.1} KiB, {} ms)",
            result.path.display(),
            result.bytes as f64 / 1024.0,
            result.elapsed_ms
        );
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
        match fs::write_file(path, &content) {
            Ok(()) => {
                let doc = self.tabs.active_mut();
                doc.path = Some(path.to_path_buf());
                doc.title = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                doc.language = doc::detect_language(path);
                doc.mark_clean();
                self.recent.touch(path);
                self.highlight_dirty = true;
                self.status = format!("Saved {}", path.display());
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
            buf.mark_dirty();
        } else {
            let caret = self.tabs.active().buffer.caret();
            self.tabs.active_mut().buffer.replace_document(&out);
            self.tabs.active_mut().buffer.set_caret(caret.min(out.chars().count()));
            self.tabs.active_mut().mark_dirty();
        }
        self.highlight_dirty = true;
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
        self.tabs.active_mut().mark_dirty();
        self.highlight_dirty = true;
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
        let count = text.matches(&q).count();
        let new_text = text.replace(&q, replacement);
        self.tabs.active_mut().buffer.replace_document(&new_text);
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
        if self.tabs.active_mut().buffer.undo() {
            self.mark_text_changed();
            self.status = "Undo".into();
        }
    }

    pub fn redo(&mut self) {
        if self.tabs.active_mut().buffer.redo() {
            self.mark_text_changed();
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
            match fs::write_file(&path, &content) {
                Ok(()) => {
                    self.status = format!("Saved a copy as {}", path.display());
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
        let keep = self.tabs.active_index();
        let mut i = self.tabs.len();
        while i > 0 {
            i -= 1;
            if i != keep {
                self.tabs.close(i);
            }
        }
        self.highlight_dirty = true;
        self.status = "Closed all but active".into();
    }

    pub fn close_all_to_left(&mut self) {
        let keep = self.tabs.active_index();
        for i in (0..keep).rev() {
            self.tabs.close(i);
        }
        self.highlight_dirty = true;
        self.status = "Closed tabs to the left".into();
    }

    pub fn close_all_to_right(&mut self) {
        let keep = self.tabs.active_index();
        while self.tabs.len() > keep + 1 {
            self.tabs.close(keep + 1);
        }
        self.highlight_dirty = true;
        self.status = "Closed tabs to the right".into();
    }

    pub fn close_all_unchanged(&mut self) {
        let mut i = self.tabs.len();
        while i > 0 {
            i -= 1;
            if !self.tabs.get(i).map(|d| d.dirty).unwrap_or(true) {
                self.tabs.close(i);
            }
        }
        self.highlight_dirty = true;
        self.status = "Closed unchanged documents".into();
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
