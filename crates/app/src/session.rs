//! Persist open-file session paths under the app config dir.
//!
//! Format (backward compatible):
//! - one absolute path per line
//! - optional `@folds h1,h2,…` line after a path lists folded **header** line indices
//! - optional `@bookmarks l1,l2,…` line after a path lists bookmarked line indices
//! - optional `@style1`…`@style5` lines list Search → Style mark line indices (per slot)
//! - optional `@caret LINE COL` line after a path (0-based line and column)
//! - optional `@sel A_LINE A_COL C_LINE C_COL` line after a path (anchor + caret; non-empty selection)
//! - optional `@scroll LINE` line after a path (0-based first visible editor line; every tab)
//! - optional `@lang ID` line after a path restores Language-menu highlight id
//! - optional `@encoding TOKEN` line after a path restores save encoding
//! - optional `@readonly` marker when Edit → Toggle Read-Only was on
//! - optional `@pin` marker when the tab was pinned
//! - optional `@colour N` / `@color N` (1..=5) when a tab colour was set
//! - optional `@tail` / `@logtail` marker when View → Monitoring (log-tail) was on
//! - optional `@other` / `@otherview` marker when that tab was in the dual-view other pane
//! - optional `@focus` / `@otherfocus` marker when the dual-view other pane had keyboard focus
//! - optional `@active` marker on the entry that should be focused on restore

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use doc::FileEncoding;

const SESSION_FILE: &str = "session.txt";

/// Portable label for status (never a home absolute path).
pub const SESSION_REL: &str = "npp-rs/session.txt";

/// Anchor `(line, col)` and caret `(line, col)` for a non-empty session selection.
pub type SessionSelection = ((usize, usize), (usize, usize));

/// One session tab: path plus optional fold headers, bookmarks, style marks, caret, selection, scroll, language, encoding, read-only, pin, colour, log-tail, other-view, other-pane focus, and active flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEntry {
    pub path: PathBuf,
    pub fold_headers: Vec<usize>,
    /// 0-based bookmarked line indices.
    pub bookmarks: Vec<usize>,
    /// Search → Style 1..=5 mark lines (index 0 = Style 1).
    pub style_marks: [Vec<usize>; 5],
    /// 0-based line and column when present.
    pub caret: Option<(usize, usize)>,
    /// Anchor `(line, col)` and caret `(line, col)` when a non-empty selection was present.
    pub selection: Option<SessionSelection>,
    /// 0-based first visible editor line when present.
    pub scroll: Option<usize>,
    /// Language id for highlight / fold / comments when present.
    pub language: Option<String>,
    /// Save encoding when present (overrides open detect).
    pub encoding: Option<FileEncoding>,
    /// Edit → Toggle Read-Only was on for this tab.
    pub read_only: bool,
    pub pinned: bool,
    /// View → Tab colour id 1..=5 when set.
    pub tab_colour: Option<u8>,
    /// View → Monitoring (log-tail follow) was on for this tab.
    pub tail_follow: bool,
    /// This tab was shown in the dual-view other pane.
    pub other_view: bool,
    /// Dual-view other pane had keyboard focus (meaningful with `other_view`).
    pub focus_other: bool,
    pub active: bool,
}

fn session_store_path() -> Result<PathBuf, ()> {
    let base = crate::recent::config_dir().ok_or(())?;
    Ok(base.join("npp-rs").join(SESSION_FILE))
}

/// Write paths (and optional `@folds` / `@bookmarks` / `@styleN` / `@caret` / `@sel` / `@scroll` / `@lang` / `@encoding` / `@readonly` / `@pin` / `@colour` / `@tail` / `@other` / `@focus` / `@active` lines) for restore.
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
        for (i, marks) in entry.style_marks.iter().enumerate() {
            if marks.is_empty() {
                continue;
            }
            let joined = marks
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<_>>()
                .join(",");
            writeln!(file, "@style{} {joined}", i + 1).map_err(|e| e.to_string())?;
        }
        if let Some((line, col)) = entry.caret {
            writeln!(file, "@caret {line} {col}").map_err(|e| e.to_string())?;
        }
        if let Some(((al, ac), (cl, cc))) = entry.selection {
            writeln!(file, "@sel {al} {ac} {cl} {cc}").map_err(|e| e.to_string())?;
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
        if let Some(n) = entry.tab_colour {
            if (1..=5).contains(&n) {
                writeln!(file, "@colour {n}").map_err(|e| e.to_string())?;
            }
        }
        if entry.tail_follow {
            writeln!(file, "@tail").map_err(|e| e.to_string())?;
        }
        if entry.other_view {
            writeln!(file, "@other").map_err(|e| e.to_string())?;
        }
        if entry.focus_other {
            writeln!(file, "@focus").map_err(|e| e.to_string())?;
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
        if let Some(rest) = trimmed.strip_prefix("@style") {
            // Consume `@style…` even when the slot is invalid so later meta stays attached.
            if meta_ok {
                if let Some((slot, lines)) = parse_style_slot(rest) {
                    if let Some(last) = out.last_mut() {
                        last.style_marks[slot] = lines;
                    }
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
        if let Some(rest) = trimmed.strip_prefix("@sel") {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.selection = parse_selection(rest.trim());
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
        if let Some(rest) = trimmed
            .strip_prefix("@colour")
            .or_else(|| trimmed.strip_prefix("@color"))
        {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.tab_colour = parse_tab_colour(rest.trim());
                }
            }
            continue;
        }
        if trimmed == "@tail" || trimmed == "@logtail" {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.tail_follow = true;
                }
            }
            continue;
        }
        if trimmed == "@other" || trimmed == "@otherview" {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.other_view = true;
                }
            }
            continue;
        }
        if trimmed == "@focus" || trimmed == "@otherfocus" {
            if meta_ok {
                if let Some(last) = out.last_mut() {
                    last.focus_other = true;
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
                style_marks: Default::default(),
                caret: None,
                selection: None,
                scroll: None,
                language: None,
                encoding: None,
                read_only: false,
                pinned: false,
                tab_colour: None,
                tail_follow: false,
                other_view: false,
                focus_other: false,
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

/// Parse `N` / `N l1,l2,…` after `@style` where N is 1..=5. Returns zero-based slot index.
fn parse_style_slot(rest: &str) -> Option<(usize, Vec<usize>)> {
    let mut chars = rest.chars();
    let digit = chars.next()?;
    if !digit.is_ascii_digit() {
        return None;
    }
    let n = digit.to_digit(10)? as usize;
    if !(1..=5).contains(&n) {
        return None;
    }
    let after = chars.as_str();
    if !after.is_empty() && !after.starts_with(|c: char| c.is_whitespace()) {
        // Reject `@style10` / `@style1x`.
        return None;
    }
    Some((n - 1, parse_line_list(after.trim())))
}

fn parse_caret(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.split_whitespace();
    let line = parts.next()?.parse::<usize>().ok()?;
    let col = parts.next()?.parse::<usize>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((line, col))
}

/// Parse `@sel A_LINE A_COL C_LINE C_COL` (anchor then caret). Zero-width → `None`.
fn parse_selection(s: &str) -> Option<SessionSelection> {
    let mut parts = s.split_whitespace();
    let al = parts.next()?.parse::<usize>().ok()?;
    let ac = parts.next()?.parse::<usize>().ok()?;
    let cl = parts.next()?.parse::<usize>().ok()?;
    let cc = parts.next()?.parse::<usize>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    if al == cl && ac == cc {
        return None;
    }
    Some(((al, ac), (cl, cc)))
}

fn parse_scroll(s: &str) -> Option<usize> {
    let mut parts = s.split_whitespace();
    let line = parts.next()?.parse::<usize>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(line)
}

/// Parse `@colour` / `@color` value: single integer in 1..=5.
fn parse_tab_colour(s: &str) -> Option<u8> {
    let mut parts = s.split_whitespace();
    let n = parts.next()?.parse::<u8>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    if (1..=5).contains(&n) {
        Some(n)
    } else {
        None
    }
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
        assert_eq!(entries[0].selection, None);
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
        assert_eq!(entries[0].selection, None);
        assert!(!entries[0].active);
        assert_eq!(entries[1].fold_headers, vec![0]);
        assert_eq!(entries[1].caret, Some((0, 0)));
        assert!(entries[1].active);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_selection_meta() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-sel-{}",
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
            "{}\n@caret 1 3\n@sel 0 0 1 3\n@active\n{}\n@sel 0 1 0 2\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].caret, Some((1, 3)));
        assert_eq!(entries[0].selection, Some(((0, 0), (1, 3))));
        assert!(entries[0].active);
        assert_eq!(entries[1].selection, Some(((0, 1), (0, 2))));

        let bad = format!(
            "{}\n@sel\n@sel 0 0\n@sel 0 0 0 0\n@sel 1 2 3 4 5\n@sel nope\n",
            a.display()
        );
        let entries = parse_session_text(&bad);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].selection, None);

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
    fn parse_style_marks() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-styles-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("a.rs");
        fs::write(&a, "line0\nline1\nline2\nline3\n").unwrap();

        let text = format!(
            "{}\n@style1 2,0,2\n@style3 1\n@style5 3,1\n@style0 9\n@style10 1\n@style6 1\n@bookmarks 0\n@active\n",
            a.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].style_marks[0], vec![0, 2]);
        assert!(entries[0].style_marks[1].is_empty());
        assert_eq!(entries[0].style_marks[2], vec![1]);
        assert!(entries[0].style_marks[3].is_empty());
        assert_eq!(entries[0].style_marks[4], vec![1, 3]);
        assert_eq!(entries[0].bookmarks, vec![0]);
        assert!(entries[0].active);

        let empty = format!("{}\n@style2\n@active\n", a.display());
        let entries = parse_session_text(&empty);
        assert_eq!(entries.len(), 1);
        assert!(entries[0].style_marks[1].is_empty());

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
    fn parse_tab_colour_meta() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-colour-{}",
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
            "{}\n@colour 3\n@pin\n{}\n@color 5\n@active\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].tab_colour, Some(3));
        assert!(entries[0].pinned);
        assert_eq!(entries[1].tab_colour, Some(5));
        assert!(entries[1].active);

        let bad = format!(
            "{}\n@colour\n@colour 0\n@colour 6\n@colour 2 2\n@color nope\n",
            a.display()
        );
        let entries = parse_session_text(&bad);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].tab_colour, None);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_tail_meta() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-tail-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("app.log");
        let b = dir.join("other.txt");
        fs::write(&a, "line0\n").unwrap();
        fs::write(&b, "x\n").unwrap();

        let text = format!(
            "{}\n@tail\n@pin\n{}\n@logtail\n@active\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert!(entries[0].tail_follow);
        assert!(entries[0].pinned);
        assert!(entries[1].tail_follow);
        assert!(entries[1].active);

        let none = format!("{}\n@caret 0 0\n", a.display());
        let entries = parse_session_text(&none);
        assert_eq!(entries.len(), 1);
        assert!(!entries[0].tail_follow);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_other_view_meta() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-other-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("left.rs");
        let b = dir.join("right.rs");
        fs::write(&a, "line0\n").unwrap();
        fs::write(&b, "x\n").unwrap();

        let text = format!("{}\n@active\n{}\n@other\n@pin\n", a.display(), b.display());
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert!(entries[0].active);
        assert!(!entries[0].other_view);
        assert!(entries[1].other_view);
        assert!(entries[1].pinned);
        assert!(!entries[1].focus_other);

        let alias = format!("{}\n@otherview\n{}\n@active\n", a.display(), b.display());
        let entries = parse_session_text(&alias);
        assert_eq!(entries.len(), 2);
        assert!(entries[0].other_view);
        assert!(entries[1].active);

        let none = format!("{}\n@caret 0 0\n", a.display());
        let entries = parse_session_text(&none);
        assert_eq!(entries.len(), 1);
        assert!(!entries[0].other_view);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_focus_other_meta() {
        let dir = std::env::temp_dir().join(format!(
            "npp-session-focus-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        let a = dir.join("left.rs");
        let b = dir.join("right.rs");
        fs::write(&a, "line0\n").unwrap();
        fs::write(&b, "x\n").unwrap();

        let text = format!(
            "{}\n@active\n{}\n@other\n@focus\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&text);
        assert_eq!(entries.len(), 2);
        assert!(entries[0].active);
        assert!(!entries[0].focus_other);
        assert!(entries[1].other_view);
        assert!(entries[1].focus_other);

        let alias = format!(
            "{}\n@active\n{}\n@other\n@otherfocus\n",
            a.display(),
            b.display()
        );
        let entries = parse_session_text(&alias);
        assert_eq!(entries.len(), 2);
        assert!(entries[1].focus_other);

        let none = format!("{}\n@other\n", b.display());
        let entries = parse_session_text(&none);
        assert_eq!(entries.len(), 1);
        assert!(entries[0].other_view);
        assert!(!entries[0].focus_other);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_ignores_orphan_meta() {
        let text =
            "@folds 1,2\n@bookmarks 3\n@caret 0 1\n@sel 0 0 0 1\n@scroll 9\n@lang rust\n@encoding utf8\n@readonly\n@pin\n@colour 2\n@tail\n@other\n@focus\n@active\n";
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
