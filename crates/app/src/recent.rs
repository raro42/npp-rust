//! Recent files list with simple disk persistence.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

const MAX_RECENT: usize = 15;
const FILENAME: &str = "recent.txt";

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
