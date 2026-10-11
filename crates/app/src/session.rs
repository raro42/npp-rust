//! Persist open-file session paths under the app config dir.
//!
//! Format (backward compatible):
//! - one absolute path per line
//! - optional `@folds h1,h2,…` line after a path lists folded **header** line indices
//! - optional `@bookmarks l1,l2,…` line after a path lists bookmarked line indices
//! - optional `@caret LINE COL` line after a path (0-based line and column)
//! - optional `@scroll LINE` line after a path (0-based first visible editor line; every tab)
//! - optional `@lang ID` line after a path restores Language-menu highlight id
//! - optional `@encoding TOKEN` line after a path restores save encoding
//! - optional `@readonly` marker when Edit → Toggle Read-Only was on
//! - optional `@pin` marker when the tab was pinned
//! - optional `@active` marker on the entry that should be focused on restore

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use doc::FileEncoding;

const SESSION_FILE: &str = "session.txt";

/// Portable label for status (never a home absolute path).
pub const SESSION_REL: &str = "npp-rs/session.txt";

/// One session tab: path plus optional fold headers, bookmarks, caret, scroll, language, encoding, read-only, pin, and active flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEntry {
    pub path: PathBuf,
    pub fold_headers: Vec<usize>,
    /// 0-based bookmarked line indices.
    pub bookmarks: Vec<usize>,
    /// 0-based line and column when present.
    pub caret: Option<(usize, usize)>,
    /// 0-based first visible editor line when present.
    pub scroll: Option<usize>,
    /// Language id for highlight / fold / comments when present.
    pub language: Option<String>,
    /// Save encoding when present (overrides open detect).
    pub encoding: Option<FileEncoding>,
    /// Edit → Toggle Read-Only was on for this tab.
    pub read_only: bool,
    pub pinned: bool,
    pub active: bool,
}

fn session_store_path() -> Result<PathBuf, ()> {
    let base = crate::recent::config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(SESSION_FILE))
}

/// Write paths (and optional `@folds` / `@bookmarks` / `@caret` / `@scroll` / `@lang` / `@encoding` / `@readonly` / `@pin` / `@active` lines) for restore.
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
        if !entry.bookmarks.is_empty() {
            let joined = entry
                .bookmarks
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<_>>()
                .join(",");
            writeln!(file, "@bookmarks {joined}").map_err(|e| e.to_string())?;
        }
        if let Some((line, col)) = entry.caret {
            writeln!(file, "@caret {line} {col}").map_err(|e| e.to_string())?;
        }
        if let Some(line) = entry.scroll {
            writeln!(file, "@scroll {line}").map_err(|e| e.to_string())?;
        }
        if let Some(lang) = entry.language.as_deref() {
            if let Some(safe) = sanitize_language_id(lang) {
                writeln!(file, "@lang {safe}").map_err(|e| e.to_string())?;
            }
        }
        if let Some(enc) = entry.encoding {
            writeln!(file, "@encoding {}", encoding_token(enc)).map_err(|e| e.to_string())?;
        }
        if entry.read_only {
            writeln!(file, "@readonly").map_err(|e| e.to_string())?;
        }
        if entry.pinned {
            writeln!(file, "@pin").map_err(|e| e.to_string())?;
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
                    last.fold_headers = parse_line_list(rest.trim());
                }
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("@bookmarks") {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.bookmarks = parse_line_list(rest.trim());
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
        if let Some(rest) = trimmed.strip_prefix("@scroll") {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.scroll = parse_scroll(rest.trim());
                }
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("@lang") {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.language = sanitize_language_id(rest.trim());
                }
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("@encoding") {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.encoding = parse_encoding_token(rest.trim());
                }
            }
            continue;
        }
        if trimmed == "@readonly" || trimmed == "@ro" {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.read_only = true;
                }
            }
            continue;
        }
        if trimmed == "@pin" {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.pinned = true;
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
                bookmarks: Vec::new(),
                caret: None,
                scroll: None,
                language: None,
                encoding: None,
                read_only: false,
                pinned: false,
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

fn parse_line_list(s: &str) -> Vec<usize> {
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

fn parse_scroll(s: &str) -> Option<usize> {
    let mut parts = s.split_whitespace();
    let line = parts.next()?.parse::<usize>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(line)
}

/// Accept a short lowercase language id (`rust`, `python`, `plain`).
fn sanitize_language_id(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() || t.len() > 32 {
        return None;
    }
    let mut out = String::with_capacity(t.len());
    for ch in t.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '+' {
            out.push(ch.to_ascii_lowercase());
        } else {
            return None;
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

/// Stable session token for [`FileEncoding`].
pub fn encoding_token(enc: FileEncoding) -> &'static str {
    match enc {
        FileEncoding::Utf8 => "utf8",
        FileEncoding::Utf8Bom => "utf8-bom",
        FileEncoding::Utf16Le => "utf16-le",
        FileEncoding::Utf16Be => "utf16-be",
        FileEncoding::Windows1252 => "windows-1252",
    }
}

/// Parse `@encoding` tokens (aliases accepted; unknown → `None`).
pub fn parse_encoding_token(s: &str) -> Option<FileEncoding> {
    let t = s.trim().to_ascii_lowercase();
    if t.is_empty() || t.len() > 32 {
        return None;
    }
    match t.as_str() {
        "utf8" | "utf-8" => Some(FileEncoding::Utf8),
        "utf8-bom" | "utf-8-bom" | "utf8bom" => Some(FileEncoding::Utf8Bom),
        "utf16-le" | "utf16le" | "utf-16-le" => Some(FileEncoding::Utf16Le),
        "utf16-be" | "utf16be" | "utf-16-be" => Some(FileEncoding::Utf16Be),
        "windows-1252" | "windows1252" | "ansi" => Some(FileEncoding::Windows1252),
        _ => None,
    }
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
        assert!(entries[0].bookmarks.is_empty());
        assert_eq!(entries[0].caret, None);
        assert_eq!(entries[0].scroll, None);
        assert_eq!(entries[0].language, None);
        assert_eq!(entries[0].encoding, None);
        assert!(!entries[0].read_only);
        assert!(!entries[0].pinned);
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
    fn parse_bookmarks() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-bookmarks-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("a.rs");
        fs::write(&a, "line0\nline1\nline2\n").unwrap();

        let text = format!(
            "{}\n@bookmarks 2,0,2\n@folds 1\n@caret 0 0\n@active\n",
            a.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].bookmarks, vec![0, 2]);
        assert_eq!(entries[0].fold_headers, vec![1]);
        assert!(!entries[0].pinned);
        assert!(entries[0].active);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_pin() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-pin-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("a.rs");
        let b = dir.join("b.rs");
        fs::write(&a, "line0\n").unwrap();
        fs::write(&b, "x\n").unwrap();

        let text = format!(
            "{}\n@pin\n@caret 0 0\n{}\n@active\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert!(entries[0].pinned);
        assert!(!entries[0].active);
        assert!(!entries[1].pinned);
        assert!(entries[1].active);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_lang() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-lang-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("notes.txt");
        fs::write(&a, "print(1)\n").unwrap();

        let text = format!("{}\n@lang Python\n@caret 0 0\n@active\n", a.display());
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].language.as_deref(), Some("python"));
        assert!(entries[0].active);

        let bad = format!("{}\n@lang weird lang!\n@lang\n", a.display());
        let entries = parse_session_text(&bad);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].language, None);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_encoding() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-encoding-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("notes.txt");
        fs::write(&a, "hello\n").unwrap();

        let text = format!(
            "{}\n@encoding UTF-8-BOM\n@lang plain\n@active\n",
            a.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].encoding, Some(FileEncoding::Utf8Bom));
        assert_eq!(entries[0].language.as_deref(), Some("plain"));
        assert!(entries[0].active);

        let aliases = format!(
            "{}\n@encoding ansi\n{}\n@encoding utf16le\n",
            a.display(),
            a.display()
        );
        let entries = parse_session_text(&aliases);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].encoding, Some(FileEncoding::Windows1252));
        assert_eq!(entries[1].encoding, Some(FileEncoding::Utf16Le));

        let bad = format!("{}\n@encoding not-a-real-enc\n@encoding\n", a.display());
        let entries = parse_session_text(&bad);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].encoding, None);

        assert_eq!(encoding_token(FileEncoding::Utf8), "utf8");
        assert_eq!(
            parse_encoding_token(encoding_token(FileEncoding::Utf16Be)),
            Some(FileEncoding::Utf16Be)
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_readonly() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-readonly-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("notes.txt");
        let b = dir.join("other.txt");
        fs::write(&a, "hello\n").unwrap();
        fs::write(&b, "x\n").unwrap();

        let text = format!(
            "{}\n@readonly\n@encoding utf8\n@active\n{}\n@ro\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert!(entries[0].read_only);
        assert_eq!(entries[0].encoding, Some(FileEncoding::Utf8));
        assert!(entries[0].active);
        assert!(entries[1].read_only);
        assert!(!entries[1].active);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_scroll() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-scroll-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("notes.txt");
        let b = dir.join("other.txt");
        fs::write(&a, "hello\n").unwrap();
        fs::write(&b, "x\n").unwrap();

        let text = format!(
            "{}\n@caret 10 0\n@scroll 42\n@active\n{}\n@scroll 0\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].caret, Some((10, 0)));
        assert_eq!(entries[0].scroll, Some(42));
        assert!(entries[0].active);
        assert_eq!(entries[1].scroll, Some(0));
        assert!(!entries[1].active);

        let bad = format!("{}\n@scroll\n@scroll nope\n@scroll 1 2\n", a.display());
        let entries = parse_session_text(&bad);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].scroll, None);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_ignores_orphan_meta() {
        let text =
            "@folds 1,2\n@bookmarks 3\n@caret 0 1\n@scroll 9\n@lang rust\n@encoding utf8\n@readonly\n@pin\n@active\n";
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
