//! Shared plain-text find helpers (match case / whole word).

use std::path::{Path, PathBuf};

/// True when `c` is a word character (alphanumeric or `_`).
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Default Find-in-Files exclude list (comma-separated directory / file names).
pub fn default_find_files_exclude() -> String {
    "target,node_modules,dist,build,.git".into()
}

/// Split a comma/semicolon filter list into trimmed non-empty patterns.
pub fn split_filters(raw: &str) -> Vec<String> {
    raw.split([',', ';'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

/// Simple `*` / `?` glob against `name` (ASCII case-insensitive).
pub fn glob_match(pattern: &str, name: &str) -> bool {
    let p: Vec<char> = pattern.chars().map(|c| c.to_ascii_lowercase()).collect();
    let n: Vec<char> = name.chars().map(|c| c.to_ascii_lowercase()).collect();
    glob_match_chars(&p, &n)
}

fn glob_match_chars(pat: &[char], name: &[char]) -> bool {
    let (mut i, mut j) = (0usize, 0usize);
    let mut star_i = None;
    let mut star_j = 0usize;
    while j < name.len() {
        if i < pat.len() && (pat[i] == '?' || pat[i] == name[j]) {
            i += 1;
            j += 1;
        } else if i < pat.len() && pat[i] == '*' {
            star_i = Some(i);
            star_j = j;
            i += 1;
        } else if let Some(si) = star_i {
            i = si + 1;
            star_j += 1;
            j = star_j;
        } else {
            return false;
        }
    }
    while i < pat.len() && pat[i] == '*' {
        i += 1;
    }
    i == pat.len()
}

/// True when `name` matches any pattern (empty patterns → false).
pub fn name_matches_any(name: &str, patterns: &[String]) -> bool {
    patterns.iter().any(|p| glob_match(p, name))
}

/// Caps for recursive Find in Files.
#[derive(Clone, Copy, Debug)]
pub struct FindInFilesCaps {
    pub max_file_bytes: u64,
    pub max_matches: usize,
    pub max_files: usize,
    pub max_depth: usize,
}

impl Default for FindInFilesCaps {
    fn default() -> Self {
        Self {
            max_file_bytes: 512 * 1024,
            max_matches: 500,
            max_files: 2000,
            max_depth: 32,
        }
    }
}

/// One hit line from a workspace scan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FindInFilesHit {
    pub rel_path: String,
    pub line_no: usize,
    pub line: String,
}

/// Result of a recursive workspace scan.
#[derive(Clone, Debug, Default)]
pub struct FindInFilesReport {
    pub hits: Vec<FindInFilesHit>,
    pub files_scanned: usize,
    pub truncated: bool,
}

fn should_skip_dir(name: &str, exclude: &[String]) -> bool {
    if name.starts_with('.') {
        return true;
    }
    name_matches_any(name, exclude)
}

fn file_allowed(name: &str, include: &[String], exclude: &[String]) -> bool {
    if name.starts_with('.') {
        return false;
    }
    if name_matches_any(name, exclude) {
        return false;
    }
    if include.is_empty() || include.iter().any(|p| p == "*") {
        return true;
    }
    name_matches_any(name, include)
}

fn line_has_query(line: &str, query: &str, match_case: bool) -> bool {
    if match_case {
        line.contains(query)
    } else {
        let q = query.to_ascii_lowercase();
        line.to_ascii_lowercase().contains(&q)
    }
}

/// Recursively scan `root` for `query`. Paths in hits are relative to `root`.
pub fn find_in_files_scan(
    root: &Path,
    query: &str,
    match_case: bool,
    include: &[String],
    exclude: &[String],
    caps: FindInFilesCaps,
) -> FindInFilesReport {
    let mut report = FindInFilesReport::default();
    if query.is_empty() || !root.is_dir() {
        return report;
    }

    let mut stack: Vec<(PathBuf, usize)> = vec![(root.to_path_buf(), 0)];
    while let Some((dir, depth)) = stack.pop() {
        if report.truncated || report.hits.len() >= caps.max_matches {
            report.truncated = true;
            break;
        }
        let Ok(rd) = std::fs::read_dir(&dir) else {
            continue;
        };
        let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            if report.hits.len() >= caps.max_matches || report.files_scanned >= caps.max_files {
                report.truncated = true;
                break;
            }
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            let Ok(meta) = entry.metadata() else {
                continue;
            };
            if meta.file_type().is_symlink() {
                continue;
            }
            if meta.is_dir() {
                if depth >= caps.max_depth || should_skip_dir(&name, exclude) {
                    continue;
                }
                stack.push((path, depth + 1));
                continue;
            }
            if !meta.is_file() || !file_allowed(&name, include, exclude) {
                continue;
            }
            if meta.len() > caps.max_file_bytes {
                continue;
            }
            let Ok(bytes) = std::fs::read(&path) else {
                continue;
            };
            if bytes.contains(&0) {
                continue;
            }
            let Ok(text) = String::from_utf8(bytes) else {
                continue;
            };
            report.files_scanned += 1;
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            for (li, line) in text.lines().enumerate() {
                if report.hits.len() >= caps.max_matches {
                    report.truncated = true;
                    break;
                }
                if line_has_query(line, query, match_case) {
                    report.hits.push(FindInFilesHit {
                        rel_path: rel.clone(),
                        line_no: li + 1,
                        line: line.to_string(),
                    });
                }
            }
        }
    }
    report
}

/// All non-overlapping matches as char ranges `[start, end)`.
pub fn find_all_matches(
    text: &str,
    query: &str,
    match_case: bool,
    whole_word: bool,
) -> Vec<(usize, usize)> {
    if query.is_empty() {
        return Vec::new();
    }
    let chars: Vec<char> = text.chars().collect();
    let q: Vec<char> = query.chars().collect();
    let qlen = q.len();
    if qlen == 0 || qlen > chars.len() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + qlen <= chars.len() {
        let matched = if match_case {
            chars[i..i + qlen] == q[..]
        } else {
            chars[i..i + qlen]
                .iter()
                .zip(q.iter())
                .all(|(a, b)| a.eq_ignore_ascii_case(b))
        };
        if matched {
            let ok = if whole_word {
                let before_ok = i == 0 || !is_word_char(chars[i - 1]);
                let after_ok = i + qlen >= chars.len() || !is_word_char(chars[i + qlen]);
                before_ok && after_ok
            } else {
                true
            };
            if ok {
                out.push((i, i + qlen));
                i += qlen;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Next match at or after `from` (char index). Wraps when `wrap` is true.
pub fn find_next(
    text: &str,
    query: &str,
    from: usize,
    wrap: bool,
    match_case: bool,
    whole_word: bool,
) -> Option<(usize, usize)> {
    let all = find_all_matches(text, query, match_case, whole_word);
    if all.is_empty() {
        return None;
    }
    if let Some(m) = all.iter().find(|(s, _)| *s >= from) {
        return Some(*m);
    }
    if wrap {
        return all.first().copied();
    }
    None
}

/// Previous match ending at or before `from`. Wraps when `wrap` is true.
pub fn find_prev(
    text: &str,
    query: &str,
    from: usize,
    wrap: bool,
    match_case: bool,
    whole_word: bool,
) -> Option<(usize, usize)> {
    let all = find_all_matches(text, query, match_case, whole_word);
    if all.is_empty() {
        return None;
    }
    if let Some(m) = all.iter().rev().find(|(_, e)| *e <= from) {
        return Some(*m);
    }
    if wrap {
        return all.last().copied();
    }
    None
}

/// Replace all matches. Returns `(new_text, replacement_count)`.
pub fn replace_all(
    text: &str,
    query: &str,
    replacement: &str,
    match_case: bool,
    whole_word: bool,
) -> (String, usize) {
    let matches = find_all_matches(text, query, match_case, whole_word);
    if matches.is_empty() {
        return (text.to_string(), 0);
    }
    let chars: Vec<char> = text.chars().collect();
    let repl: Vec<char> = replacement.chars().collect();
    let mut out = Vec::with_capacity(chars.len());
    let mut i = 0usize;
    let mut count = 0usize;
    for (s, e) in matches {
        if i < s {
            out.extend_from_slice(&chars[i..s]);
        }
        out.extend_from_slice(&repl);
        count += 1;
        i = e;
    }
    if i < chars.len() {
        out.extend_from_slice(&chars[i..]);
    }
    (out.into_iter().collect(), count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn case_insensitive_and_whole_word() {
        let text = "Foo foo food Foo";
        let all = find_all_matches(text, "foo", false, true);
        assert_eq!(all, vec![(0, 3), (4, 7), (13, 16)]);
    }

    #[test]
    fn replace_all_counts() {
        let (out, n) = replace_all("a a aa", "a", "b", true, true);
        assert_eq!(n, 2);
        assert_eq!(out, "b b aa");
    }

    #[test]
    fn glob_star_and_question() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(!glob_match("*.rs", "main.md"));
        assert!(glob_match("foo?.txt", "foo1.txt"));
        assert!(!glob_match("foo?.txt", "foo12.txt"));
    }

    #[test]
    fn split_filters_commas() {
        assert_eq!(
            split_filters(" *.rs ; *.md , "),
            vec!["*.rs".to_string(), "*.md".to_string()]
        );
    }

    #[test]
    fn recursive_scan_skips_target_and_respects_include() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("npp-fif-{stamp}"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("target/debug")).unwrap();
        fs::write(root.join("src/a.rs"), "needle here\nother\n").unwrap();
        fs::write(root.join("src/b.md"), "needle in md\n").unwrap();
        fs::write(root.join("target/debug/x.rs"), "needle in target\n").unwrap();

        let include = split_filters("*.rs");
        let exclude = split_filters(&default_find_files_exclude());
        let report = find_in_files_scan(
            &root,
            "needle",
            true,
            &include,
            &exclude,
            FindInFilesCaps::default(),
        );
        let _ = fs::remove_dir_all(&root);

        assert_eq!(report.files_scanned, 1);
        assert_eq!(report.hits.len(), 1);
        assert_eq!(report.hits[0].rel_path, "src/a.rs");
        assert_eq!(report.hits[0].line_no, 1);
    }
}
