//! Shared find helpers (literal or linear-time regex).

use std::path::{Path, PathBuf};

use regex::RegexBuilder;

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
    /// True when `use_regex` was set and the query failed to compile.
    pub invalid_regex: bool,
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
    use_regex: bool,
    include: &[String],
    exclude: &[String],
    caps: FindInFilesCaps,
) -> FindInFilesReport {
    let mut report = FindInFilesReport::default();
    if query.is_empty() || !root.is_dir() {
        return report;
    }

    let compiled = if use_regex {
        match compile_find_regex(query, match_case) {
            Ok(re) => Some(re),
            Err(_) => {
                report.invalid_regex = true;
                return report;
            }
        }
    } else {
        None
    };

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
                let hit = if let Some(re) = compiled.as_ref() {
                    re.is_match(line)
                } else {
                    line_has_query(line, query, match_case)
                };
                if hit {
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

/// Case / whole-word / regex flags for in-file find.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FindFlags {
    pub match_case: bool,
    pub whole_word: bool,
    pub use_regex: bool,
}
pub fn compile_find_regex(query: &str, match_case: bool) -> Result<regex::Regex, String> {
    RegexBuilder::new(query)
        .case_insensitive(!match_case)
        .multi_line(true)
        .size_limit(1 << 20)
        .dfa_size_limit(1 << 20)
        .build()
        .map_err(|e| e.to_string())
}

/// `Some` when `query` is not a valid find regex.
pub fn find_regex_error(query: &str, match_case: bool) -> Option<String> {
    compile_find_regex(query, match_case).err()
}

fn whole_word_ok(chars: &[char], start: usize, end: usize) -> bool {
    let before_ok = start == 0 || !is_word_char(chars[start - 1]);
    let after_ok = end >= chars.len() || !is_word_char(chars[end]);
    before_ok && after_ok
}

fn char_index_to_byte(text: &str, char_idx: usize) -> usize {
    text.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(text.len())
}

fn parse_ascii_u32(chars: &[char], start: usize, end: usize) -> usize {
    let mut n = 0usize;
    for &c in &chars[start..end] {
        n = n
            .saturating_mul(10)
            .saturating_add(u32::from(c as u8 - b'0') as usize);
    }
    n
}

fn capture_group<'a>(caps: &regex::Captures<'a>, n: usize) -> &'a str {
    caps.get(n).map(|m| m.as_str()).unwrap_or("")
}

/// Expand `$n` / `${n}` / `$&` / `$$` and `\n` / `\t` / `\1` in a regex replacement.
fn expand_regex_template(template: &str, caps: &regex::Captures<'_>) -> String {
    let chars: Vec<char> = template.chars().collect();
    let mut out = String::with_capacity(template.len());
    let cap_n = caps.len();
    let mut i = 0usize;
    while i < chars.len() {
        match chars[i] {
            '$' => {
                if i + 1 >= chars.len() {
                    out.push('$');
                    i += 1;
                    continue;
                }
                match chars[i + 1] {
                    '$' => {
                        out.push('$');
                        i += 2;
                    }
                    '&' => {
                        out.push_str(capture_group(caps, 0));
                        i += 2;
                    }
                    '{' => {
                        let mut j = i + 2;
                        while j < chars.len() && chars[j].is_ascii_digit() {
                            j += 1;
                        }
                        if j > i + 2 && j < chars.len() && chars[j] == '}' {
                            let n = parse_ascii_u32(&chars, i + 2, j);
                            if n < cap_n {
                                out.push_str(capture_group(caps, n));
                            }
                            i = j + 1;
                        } else {
                            out.push('$');
                            i += 1;
                        }
                    }
                    c if c.is_ascii_digit() => {
                        let mut j = i + 1;
                        while j < chars.len() && chars[j].is_ascii_digit() {
                            j += 1;
                        }
                        let n = parse_ascii_u32(&chars, i + 1, j);
                        if n < cap_n {
                            out.push_str(capture_group(caps, n));
                        }
                        i = j;
                    }
                    _ => {
                        out.push('$');
                        i += 1;
                    }
                }
            }
            '\\' => {
                if i + 1 >= chars.len() {
                    out.push('\\');
                    i += 1;
                    continue;
                }
                match chars[i + 1] {
                    '\\' => {
                        out.push('\\');
                        i += 2;
                    }
                    'n' => {
                        out.push('\n');
                        i += 2;
                    }
                    't' => {
                        out.push('\t');
                        i += 2;
                    }
                    'r' => {
                        out.push('\r');
                        i += 2;
                    }
                    '$' => {
                        out.push('$');
                        i += 2;
                    }
                    c if c.is_ascii_digit() => {
                        let mut j = i + 1;
                        while j < chars.len() && chars[j].is_ascii_digit() {
                            j += 1;
                        }
                        let n = parse_ascii_u32(&chars, i + 1, j);
                        if n < cap_n {
                            out.push_str(capture_group(caps, n));
                        }
                        i = j;
                    }
                    c => {
                        out.push(c);
                        i += 2;
                    }
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}

/// Expand a replacement for one match `[start, end)` (char indices). Literal when regex is off.
pub fn expand_replacement(
    text: &str,
    query: &str,
    replacement: &str,
    match_start: usize,
    match_end: usize,
    flags: FindFlags,
) -> String {
    if !flags.use_regex {
        return replacement.to_string();
    }
    let Ok(re) = compile_find_regex(query, flags.match_case) else {
        return replacement.to_string();
    };
    let byte = char_index_to_byte(text, match_start);
    let Some(cap) = re.captures_at(text, byte) else {
        return replacement.to_string();
    };
    let Some(m) = cap.get(0) else {
        return replacement.to_string();
    };
    let end_byte = char_index_to_byte(text, match_end);
    if m.start() != byte || m.end() != end_byte {
        return replacement.to_string();
    }
    expand_regex_template(replacement, &cap)
}

fn replace_all_regex(
    text: &str,
    query: &str,
    replacement: &str,
    flags: FindFlags,
) -> (String, usize) {
    let Ok(re) = compile_find_regex(query, flags.match_case) else {
        return (text.to_string(), 0);
    };
    let word_chars: Vec<char> = if flags.whole_word {
        text.chars().collect()
    } else {
        Vec::new()
    };
    let mut out = String::with_capacity(text.len());
    let mut last = 0usize;
    let mut count = 0usize;
    for cap in re.captures_iter(text) {
        let Some(m) = cap.get(0) else {
            continue;
        };
        if m.start() == m.end() {
            continue;
        }
        if flags.whole_word {
            let start = text[..m.start()].chars().count();
            let end = start + text[m.start()..m.end()].chars().count();
            if !whole_word_ok(&word_chars, start, end) {
                continue;
            }
        }
        out.push_str(&text[last..m.start()]);
        out.push_str(&expand_regex_template(replacement, &cap));
        last = m.end();
        count += 1;
    }
    out.push_str(&text[last..]);
    (out, count)
}

fn find_all_matches_regex(
    text: &str,
    query: &str,
    match_case: bool,
    whole_word: bool,
) -> Vec<(usize, usize)> {
    let Ok(re) = compile_find_regex(query, match_case) else {
        return Vec::new();
    };
    let chars: Vec<char> = if whole_word {
        text.chars().collect()
    } else {
        Vec::new()
    };
    let mut out = Vec::new();
    for m in re.find_iter(text) {
        if m.start() == m.end() {
            continue;
        }
        let start = text[..m.start()].chars().count();
        let end = start + text[m.start()..m.end()].chars().count();
        if whole_word && !whole_word_ok(&chars, start, end) {
            continue;
        }
        out.push((start, end));
    }
    out
}

/// All non-overlapping matches as char ranges `[start, end)`.
pub fn find_all_matches(text: &str, query: &str, flags: FindFlags) -> Vec<(usize, usize)> {
    if query.is_empty() {
        return Vec::new();
    }
    if flags.use_regex {
        return find_all_matches_regex(text, query, flags.match_case, flags.whole_word);
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
        let matched = if flags.match_case {
            chars[i..i + qlen] == q[..]
        } else {
            chars[i..i + qlen]
                .iter()
                .zip(q.iter())
                .all(|(a, b)| a.eq_ignore_ascii_case(b))
        };
        if matched {
            let ok = if flags.whole_word {
                whole_word_ok(&chars, i, i + qlen)
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
    flags: FindFlags,
) -> Option<(usize, usize)> {
    let all = find_all_matches(text, query, flags);
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
    flags: FindFlags,
) -> Option<(usize, usize)> {
    let all = find_all_matches(text, query, flags);
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

/// Character slice `[lo, hi)` of `text`.
pub fn char_span(text: &str, lo: usize, hi: usize) -> String {
    let n = text.chars().count();
    let lo = lo.min(n);
    let hi = hi.min(n).max(lo);
    text.chars().skip(lo).take(hi - lo).collect()
}

/// Matches of `query` whose spans lie in `[lo, hi)` (absolute char indices).
pub fn find_all_matches_in(
    text: &str,
    query: &str,
    lo: usize,
    hi: usize,
    flags: FindFlags,
) -> Vec<(usize, usize)> {
    let slice = char_span(text, lo, hi);
    find_all_matches(&slice, query, flags)
        .into_iter()
        .map(|(s, e)| (s + lo, e + lo))
        .collect()
}

/// Next match at or after `from` inside `span` (`[lo, hi)`). Wraps within the span.
pub fn find_next_in(
    text: &str,
    query: &str,
    from: usize,
    wrap: bool,
    span: (usize, usize),
    flags: FindFlags,
) -> Option<(usize, usize)> {
    let (lo, hi) = span;
    let slice = char_span(text, lo, hi);
    let from_rel = from.max(lo).saturating_sub(lo);
    find_next(&slice, query, from_rel, wrap, flags).map(|(s, e)| (s + lo, e + lo))
}

/// Previous match ending at or before `from` inside `span` (`[lo, hi)`). Wraps within the span.
pub fn find_prev_in(
    text: &str,
    query: &str,
    from: usize,
    wrap: bool,
    span: (usize, usize),
    flags: FindFlags,
) -> Option<(usize, usize)> {
    let (lo, hi) = span;
    let slice = char_span(text, lo, hi);
    let from_rel = from.min(hi).saturating_sub(lo);
    find_prev(&slice, query, from_rel, wrap, flags).map(|(s, e)| (s + lo, e + lo))
}

/// Replace matches inside `[lo, hi)`. Returns `(new_text, count, new_hi)`.
pub fn replace_all_in(
    text: &str,
    query: &str,
    replacement: &str,
    lo: usize,
    hi: usize,
    flags: FindFlags,
) -> (String, usize, usize) {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let lo = lo.min(n);
    let hi = hi.min(n).max(lo);
    let slice: String = chars[lo..hi].iter().collect();
    let (new_slice, count) = replace_all(&slice, query, replacement, flags);
    let new_hi = lo + new_slice.chars().count();
    let mut out = String::with_capacity(lo + new_slice.len() + (n - hi));
    out.extend(chars[..lo].iter());
    out.push_str(&new_slice);
    out.extend(chars[hi..].iter());
    (out, count, new_hi)
}

/// Replace all matches. Returns `(new_text, replacement_count)`.
///
/// With `use_regex`, the replacement expands `$n` / `\n` capture tokens.
pub fn replace_all(
    text: &str,
    query: &str,
    replacement: &str,
    flags: FindFlags,
) -> (String, usize) {
    if query.is_empty() {
        return (text.to_string(), 0);
    }
    if flags.use_regex {
        return replace_all_regex(text, query, replacement, flags);
    }
    let matches = find_all_matches(text, query, flags);
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
    use std::time::{Instant, SystemTime, UNIX_EPOCH};

    const fn flags(match_case: bool, whole_word: bool, use_regex: bool) -> FindFlags {
        FindFlags {
            match_case,
            whole_word,
            use_regex,
        }
    }

    #[test]
    fn case_insensitive_and_whole_word() {
        let text = "Foo foo food Foo";
        let all = find_all_matches(text, "foo", flags(false, true, false));
        assert_eq!(all, vec![(0, 3), (4, 7), (13, 16)]);
    }

    #[test]
    fn replace_all_counts() {
        let (out, n) = replace_all("a a aa", "a", "b", flags(true, true, false));
        assert_eq!(n, 2);
        assert_eq!(out, "b b aa");
    }

    #[test]
    fn find_in_span_skips_outside() {
        let text = "xx foo yy foo zz";
        let lit = flags(true, false, false);
        let all = find_all_matches_in(text, "foo", 0, 8, lit);
        assert_eq!(all, vec![(3, 6)]);
        let next = find_next_in(text, "foo", 6, true, (0, 8), lit);
        assert_eq!(next, Some((3, 6)));
        assert_eq!(find_next_in(text, "foo", 6, false, (0, 8), lit), None);
        let (out, n, new_hi) = replace_all_in(text, "foo", "BAR", 8, text.chars().count(), lit);
        assert_eq!(n, 1);
        assert_eq!(out, "xx foo yy BAR zz");
        assert_eq!(new_hi, out.chars().count());
    }

    #[test]
    fn regex_digits_skip_invalid_and_empty() {
        let text = "a12 b3";
        let re = flags(true, false, true);
        assert_eq!(find_all_matches(text, r"\d+", re), vec![(1, 3), (5, 6)]);
        assert!(find_all_matches(text, "[", re).is_empty());
        assert!(find_regex_error("[", true).is_some());
        assert_eq!(find_all_matches("aaa", "a+", re), vec![(0, 3)]);
        let (out, n) = replace_all("a12 b3", r"\d+", "N", re);
        assert_eq!(n, 2);
        assert_eq!(out, "aN bN");
        let (swapped, n) = replace_all("ab xy", r"(\w)(\w)", r"$2$1", re);
        assert_eq!(n, 2);
        assert_eq!(swapped, "ba yx");
        let (grp, n) = replace_all("a12", r"(\d+)", r"[$1]", re);
        assert_eq!(n, 1);
        assert_eq!(grp, "a[12]");
        let (dollar, n) = replace_all("a", "a", r"$$", re);
        assert_eq!(n, 1);
        assert_eq!(dollar, "$");
        let (nl, n) = replace_all("ab", "(a)(b)", r"\1\n\2", re);
        assert_eq!(n, 1);
        assert_eq!(nl, "a\nb");
        let lit = flags(true, false, false);
        let (kept, n) = replace_all("ab", "ab", r"$1", lit);
        assert_eq!(n, 1);
        assert_eq!(kept, "$1");
    }

    #[test]
    fn regex_nested_quantifiers_stay_linear() {
        let text = "a".repeat(50);
        let start = Instant::now();
        let hits = find_all_matches(&text, "(a+)+$", flags(true, false, true));
        assert!(start.elapsed().as_millis() < 500);
        assert_eq!(hits, vec![(0, 50)]);
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
            false,
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

    #[test]
    fn recursive_scan_regex_and_invalid() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("npp-fif-re-{stamp}"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.rs"), "alpha 12\nbeta\nalpha 3\n").unwrap();

        let include = split_filters("*.rs");
        let exclude = split_filters(&default_find_files_exclude());
        let report = find_in_files_scan(
            &root,
            r"alpha \d+",
            true,
            true,
            &include,
            &exclude,
            FindInFilesCaps::default(),
        );
        assert!(!report.invalid_regex);
        assert_eq!(report.hits.len(), 2);
        assert_eq!(report.hits[0].line_no, 1);
        assert_eq!(report.hits[1].line_no, 3);

        let bad = find_in_files_scan(
            &root,
            "[",
            true,
            true,
            &include,
            &exclude,
            FindInFilesCaps::default(),
        );
        let _ = fs::remove_dir_all(&root);
        assert!(bad.invalid_regex);
        assert!(bad.hits.is_empty());
        assert_eq!(bad.files_scanned, 0);
    }
}
