//! Persist open-file session paths under the app config dir.
//!
//! Format (backward compatible):
//! - one absolute path per line
//! - optional `@folds h1,h2,…` line after a path lists folded **header** line indices
//! - optional `@caret LINE COL` line after a path (0-based line and column)
//! - optional `@active` marker on the entry that should be focused on restore

use std::fs;
use std::io::Write;
use std::path::PathBuf;

const SESSION_FILE: &str = "session.txt";

/// Portable label for status (never a home absolute path).
pub const SESSION_REL: &str = "npp-rs/session.txt";

/// One session tab: path plus optional fold headers, caret, and active flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEntry {
    pub path: PathBuf,
    pub fold_headers: Vec<usize>,
    /// 0-based line and column when present.
    pub caret: Option<(usize, usize)>,
    pub active: bool,
}

fn session_store_path() -> Result<PathBuf, ()> {
    let base = crate::recent::config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(SESSION_FILE))
}

/// Write paths (and optional `@folds` / `@caret` / `@active` lines) for restore.
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
        if let Some((line, col)) = entry.caret {
            writeln!(file, "@caret {line} {col}").map_err(|e| e.to_string())?;
        }
        if entry.active {
            writeln!(file, "@active").map_err(|e| e.to_string())?;
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
    let mut meta_ok = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("@folds") {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.fold_headers = parse_fold_headers(rest.trim());
                }
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("@caret") {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.caret = parse_caret(rest.trim());
                }
            }
            continue;
        }
        if trimmed == "@active" {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.active = true;
                }
            }
            continue;
        }
        let p = PathBuf::from(trimmed);
        if p.as_os_str().is_empty() {
            meta_ok = false;
            continue;
        }
        if p.exists() {
            out.push(SessionEntry {
                path: p,
                fold_headers: Vec::new(),
                caret: None,
                active: false,
            });
            meta_ok = true;
        } else {
            // Missing path: skip it and any following meta lines.
            meta_ok = false;
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

fn parse_caret(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.split_whitespace();
    let line = parts.next()?.parse::<usize>().ok()?;
    let col = parts.next()?.parse::<usize>().ok()?;
    Some((line, col))
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
        assert_eq!(entries[0].caret, None);
        assert!(!entries[0].active);
        assert_eq!(entries[1].path, b);
        assert_eq!(entries[1].fold_headers, vec![2]);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_caret_and_active() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-caret-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("a.rs");
        let b = dir.join("b.rs");
        fs::write(&a, "line0\nline1\n").unwrap();
        fs::write(&b, "x\n").unwrap();

        let text = format!(
            "{}\n@caret 1 2\n{}\n@folds 0\n@caret 0 0\n@active\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].caret, Some((1, 2)));
        assert!(!entries[0].active);
        assert_eq!(entries[1].fold_headers, vec![0]);
        assert_eq!(entries[1].caret, Some((0, 0)));
        assert!(entries[1].active);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_ignores_orphan_meta() {
        let text = "@folds 1,2\n@caret 0 1\n@active\n";
        assert!(parse_session_text(text).is_empty());
    }

    #[test]
    fn parse_ignores_bad_caret() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-bad-caret-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("a.rs");
        fs::write(&a, "x\n").unwrap();
        let text = format!("{}\n@caret nope\n@caret 3\n", a.display());
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].caret, None);
        let _ = fs::remove_dir_all(&dir);
    }
}
