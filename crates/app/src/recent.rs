//! Recent files list and small app settings with disk persistence.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const DEFAULT_RECENT_MAX: u8 = 15;
const FILENAME: &str = "recent.txt";
const SETTINGS_FILE: &str = "settings.json";

/// Repo-relative / portable label for status (never a home absolute path).
pub const SETTINGS_REL: &str = "npp-rs/settings.json";
/// Panic log written under the process working directory.
pub const PANIC_LOG_REL: &str = "logs/panic.log";

/// Preference when opening `*.log` files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogTailOnOpen {
    /// Show a small dialog each time.
    #[default]
    Ask,
    /// Enable Monitoring (tail) immediately.
    Always,
    /// Open like a normal file; no prompt.
    Never,
}

/// Default newline for Enter on new / edited text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DefaultEol {
    #[default]
    Lf,
    Crlf,
}

impl DefaultEol {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::Crlf => "\r\n",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Lf => "LF (Unix)",
            Self::Crlf => "CRLF (Windows)",
        }
    }
}

fn default_font_size() -> f32 {
    14.0
}

fn default_show_line_numbers() -> bool {
    true
}

fn default_tab_width() -> u8 {
    4
}

fn default_true() -> bool {
    true
}

fn default_theme_id() -> String {
    "dark".into()
}

fn default_recent_max() -> u8 {
    DEFAULT_RECENT_MAX
}

fn default_find_match_case() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub log_tail_on_open: LogTailOnOpen,
    /// Editor monospace size (also used as zoom restore target).
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    /// Draw line numbers in the editor gutter.
    #[serde(default = "default_show_line_numbers")]
    pub show_line_numbers: bool,
    /// Spaces inserted for Tab / indent (2..=8).
    #[serde(default = "default_tab_width")]
    pub tab_width: u8,
    /// Soft word wrap (session + Preferences).
    #[serde(default)]
    pub word_wrap: bool,
    /// Status bar: show language id.
    #[serde(default = "default_true")]
    pub status_show_lang: bool,
    /// Status bar: show character count.
    #[serde(default = "default_true")]
    pub status_show_chars: bool,
    /// Theme id: `dark`, `light`, or `file:<name>.json`.
    #[serde(default = "default_theme_id")]
    pub theme_id: String,
    /// Extra gutter width in pixels (0..=40).
    #[serde(default)]
    pub gutter_extra: u8,
    /// Blink the text caret.
    #[serde(default = "default_true")]
    pub caret_blink: bool,
    /// Newline inserted by Enter.
    #[serde(default)]
    pub default_eol: DefaultEol,
    /// Max recent-file entries (5..=40).
    #[serde(default = "default_recent_max")]
    pub recent_max: u8,
    /// Reopen last session paths on launch (when argv has no files).
    #[serde(default)]
    pub restore_session: bool,
    /// Find: match case.
    #[serde(default = "default_find_match_case")]
    pub find_match_case: bool,
    /// Find: whole word only.
    #[serde(default)]
    pub find_whole_word: bool,
    /// Last Find query (restored into the find bar).
    #[serde(default)]
    pub find_query: String,
    /// Last Replace string.
    #[serde(default)]
    pub replace_with: String,
    /// Compare: treat runs of whitespace as equal.
    #[serde(default)]
    pub compare_ignore_ws: bool,
    /// Last project panel folder (absolute or relative path string).
    #[serde(default)]
    pub workspace_root: String,
    /// Project panel name filter (substring, case-insensitive).
    #[serde(default)]
    pub project_filter: String,
    /// Find in Files: include globs (comma/semicolon; empty = all).
    #[serde(default)]
    pub find_files_include: String,
    /// Find in Files: exclude names/globs (comma/semicolon).
    #[serde(default = "crate::search_util::default_find_files_exclude")]
    pub find_files_exclude: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            log_tail_on_open: LogTailOnOpen::Ask,
            font_size: default_font_size(),
            show_line_numbers: default_show_line_numbers(),
            tab_width: default_tab_width(),
            word_wrap: false,
            status_show_lang: true,
            status_show_chars: true,
            theme_id: default_theme_id(),
            gutter_extra: 0,
            caret_blink: true,
            default_eol: DefaultEol::Lf,
            recent_max: default_recent_max(),
            restore_session: false,
            find_match_case: true,
            find_whole_word: false,
            find_query: String::new(),
            replace_with: String::new(),
            compare_ignore_ws: false,
            workspace_root: String::new(),
            project_filter: String::new(),
            find_files_include: String::new(),
            find_files_exclude: crate::search_util::default_find_files_exclude(),
        }
    }
}

impl AppSettings {
    pub fn recent_limit(&self) -> usize {
        (self.recent_max.clamp(5, 40)) as usize
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let Ok(path) = settings_store_path() else {
            return Self::default();
        };
        let Ok(text) = fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&text).unwrap_or_default()
    }

    pub fn save(&self) {
        let Ok(path) = settings_store_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let Ok(text) = serde_json::to_string_pretty(self) else {
            return;
        };
        let _ = fs::write(path, text);
    }
}

#[derive(Debug, Clone, Default)]
pub struct RecentFiles {
    paths: Vec<PathBuf>,
}

impl RecentFiles {
    pub fn load() -> Self {
        let mut recent = Self::default();
        let Ok(path) = recent_store_path() else {
            return recent;
        };
        let Ok(file) = fs::File::open(&path) else {
            return recent;
        };
        for line in BufReader::new(file).lines().map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            recent.paths.push(PathBuf::from(line));
            if recent.paths.len() >= DEFAULT_RECENT_MAX as usize {
                break;
            }
        }
        recent
    }

    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }

    /// Push path to the front with a custom cap from Preferences.
    pub fn touch_limited(&mut self, path: &Path, max: usize) {
        let path = canonicalize_best_effort(path);
        let max = max.clamp(5, 40);
        self.paths.retain(|p| p != &path);
        self.paths.insert(0, path);
        self.paths.truncate(max);
        self.save();
    }

    pub fn remove(&mut self, path: &Path) {
        let path = canonicalize_best_effort(path);
        self.paths.retain(|p| p != &path);
        self.save();
    }

    pub fn clear(&mut self) {
        self.paths.clear();
        self.save();
    }

    fn save(&self) {
        let Ok(path) = recent_store_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let Ok(mut file) = fs::File::create(&path) else {
            return;
        };
        for p in &self.paths {
            let _ = writeln!(file, "{}", p.display());
        }
    }
}

fn canonicalize_best_effort(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn recent_store_path() -> Result<PathBuf, ()> {
    let base = config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(FILENAME))
}

fn settings_store_path() -> Result<PathBuf, ()> {
    let base = config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(SETTINGS_FILE))
}

/// True when the path looks like a log file (`*.log`).
pub fn is_log_path(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("log"))
        .unwrap_or(false)
}

/// File name only — safe for status bar (no home absolute path).
pub fn short_path_label(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// Config base dir (Application Support / APPDATA / XDG).
pub(crate) fn config_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")?;
        Some(PathBuf::from(home).join("Library/Application Support"))
    }
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var_os("APPDATA")?;
        Some(PathBuf::from(appdata))
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
            Some(PathBuf::from(xdg))
        } else {
            let home = std::env::var_os("HOME")?;
            Some(PathBuf::from(home).join(".config"))
        }
    }
}

/// Short label for menus: `name — parent` when useful.
pub fn recent_label(path: &Path) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    if let Some(parent) = path.parent() {
        let parent = parent.display().to_string();
        if !parent.is_empty() && parent != "." {
            // Keep menu readable: truncate long parents from the left.
            let parent = if parent.len() > 48 {
                format!("…{}", &parent[parent.len() - 47..])
            } else {
                parent
            };
            return format!("{name}  —  {parent}");
        }
    }
    name
}
