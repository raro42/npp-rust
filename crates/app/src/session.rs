//! Persist open-file session paths under the app config dir.
//!
//! Format (backward compatible):
//! - one absolute path per line
//! - optional `@folds h1,h2,…` line after a path lists folded **header** line indices

use std::fs;
use std::io::Write;
use std::path::PathBuf;

const SESSION_FILE: &str = "session.txt";

/// Portable label for status (never a home absolute path).
pub const SESSION_REL: &str = "npp-rs/session.txt";

/// One session tab: path plus optional folded fold-header lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEntry {
    pub path: PathBuf,
    pub fold_headers: Vec<usize>,
}

fn session_store_path() -> Result<PathBuf, ()> {
    let base = crate::recent::config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(SESSION_FILE))
}

/// Write paths (and optional `@folds` lines) for restore.
pub fn save_entries(entries: &[SessionEntry]) -> Result<(), String> {
    let path = session_store_path().map_err(|_| "no config dir".to_string())?;
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let mut file = fs::File::create(&path).map_err(|e| e.to_string())?;
    for entry in entries {
        writeln!(file, "{}", entry.path.display()).map_err(|e| e.to_string())?;
        if !entry.fold_headers.is_empty() {
            let joined = entry
                .fold_headers
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<_>>()
                .join(",");
            writeln!(file, "@folds {joined}").map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Load session entries (skips missing files). Old path-only files still work.
pub fn load_entries() -> Vec<SessionEntry> {
    let Ok(path) = session_store_path() else {
        return Vec::new();
    };
    let Ok(text) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    parse_session_text(&text)
}

fn parse_session_text(text: &str) -> Vec<SessionEntry> {
    let mut out: Vec<SessionEntry> = Vec::new();
    let mut folds_for_last = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("@folds") {
            if folds_for_last {
                if let Some(last) = out.last_mut() {
                    last.fold_headers = parse_fold_headers(rest.trim());
                }
            }
            folds_for_last = false;
            continue;
        }
        let p = PathBuf::from(trimmed);
        if p.as_os_str().is_empty() {
            folds_for_last = false;
            continue;
        }
        if p.exists() {
            out.push(SessionEntry {
                path: p,
                fold_headers: Vec::new(),
            });
            folds_for_last = true;
        } else {
            // Missing path: skip it and any following @folds line.
            folds_for_last = false;
        }
    }
    out
}

fn parse_fold_headers(s: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for part in s.split(',') {
        let t = part.trim();
        if t.is_empty() {
            continue;
        }
        if let Ok(n) = t.parse::<usize>() {
            out.push(n);
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parse_path_only_and_folds() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("a.rs");
        let b = dir.join("b.rs");
        fs::write(&a, "fn main() {}\n").unwrap();
        fs::write(&b, "x\n").unwrap();

        let text = format!(
            "{}\n@folds 0,4\n{}\n@folds 2\nmissing/file.rs\n@folds 9\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].path, a);
        assert_eq!(entries[0].fold_headers, vec![0, 4]);
        assert_eq!(entries[1].path, b);
        assert_eq!(entries[1].fold_headers, vec![2]);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_ignores_orphan_folds() {
        let text = "@folds 1,2\n";
        assert!(parse_session_text(text).is_empty());
    }
}
