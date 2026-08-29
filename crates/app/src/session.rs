//! Persist open-file session paths under the app config dir.

use std::fs;
use std::io::Write;
use std::path::PathBuf;

const SESSION_FILE: &str = "session.txt";

/// Portable label for status (never a home absolute path).
pub const SESSION_REL: &str = "npp-rs/session.txt";

fn session_store_path() -> Result<PathBuf, ()> {
    let base = crate::recent::config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(SESSION_FILE))
}

/// Write one path per line.
pub fn save_paths(paths: &[PathBuf]) -> Result<(), String> {
    let path = session_store_path().map_err(|_| "no config dir".to_string())?;
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let mut file = fs::File::create(&path).map_err(|e| e.to_string())?;
    for p in paths {
        writeln!(file, "{}", p.display()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Load existing paths from the session file (skips missing files).
pub fn load_existing_paths() -> Vec<PathBuf> {
    let Ok(path) = session_store_path() else {
        return Vec::new();
    };
    let Ok(text) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in text.lines() {
        let p = PathBuf::from(line.trim());
        if p.as_os_str().is_empty() {
            continue;
        }
        if p.exists() {
            out.push(p);
        }
    }
    out
}
