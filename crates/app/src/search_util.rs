//! Shared plain-text find helpers (match case / whole word).

/// True when `c` is a word character (alphanumeric or `_`).
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
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
}
