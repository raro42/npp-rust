//! Recent files list and small app settings with disk persistence.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const MAX_RECENT: usize = 15;
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

fn default_font_size() -> f32 {
    14.0
}

fn default_show_line_numbers() -> bool {
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
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            log_tail_on_open: LogTailOnOpen::Ask,
            font_size: default_font_size(),
            show_line_numbers: default_show_line_numbers(),
        }
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
        for line in BufReader::new(file).lines().flatten() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            recent.paths.push(PathBuf::from(line));
            if recent.paths.len() >= MAX_RECENT {
                break;
            }
        }
        recent
    }

    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }

    /// Push path to the front. Dedupes. Persists.
    pub fn touch(&mut self, path: &Path) {
        let path = canonicalize_best_effort(path);
        self.paths.retain(|p| p != &path);
        self.paths.insert(0, path);
        self.paths.truncate(MAX_RECENT);
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

fn config_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")?;
        return Some(PathBuf::from(home).join("Library/Application Support"));
    }
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var_os("APPDATA")?;
        return Some(PathBuf::from(appdata));
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
            return Some(PathBuf::from(xdg));
        }
        let home = std::env::var_os("HOME")?;
        Some(PathBuf::from(home).join(".config"))
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
