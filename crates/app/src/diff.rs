//! Line-oriented 2-way diff (LCS). No system `diff` — works on all OSes.

/// Per-line tag for one side of a compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    Equal,
    /// Present only on the left (removed vs right).
    Delete,
    /// Present only on the right (added vs left).
    Insert,
}

/// Soft background for a compare line (primary / secondary pane).
pub fn line_kind_bg(kind: LineKind) -> Option<eframe::egui::Color32> {
    use eframe::egui::Color32;
    match kind {
        LineKind::Equal => None,
        LineKind::Delete => Some(Color32::from_rgba_unmultiplied(180, 60, 60, 70)),
        LineKind::Insert => Some(Color32::from_rgba_unmultiplied(50, 140, 70, 70)),
    }
}

/// Max lines per side for the MVP LCS (O(n·m) memory).
pub const MAX_COMPARE_LINES: usize = 3_000;

/// Tag each line on left and right using LCS of exact line strings.
pub fn diff_line_tags(left: &[&str], right: &[&str]) -> (Vec<LineKind>, Vec<LineKind>) {
    let n = left.len();
    let m = right.len();
    if n == 0 && m == 0 {
        return (Vec::new(), Vec::new());
    }
    // dp[i][j] = LCS length of left[..i] and right[..j]
    let mut dp = vec![vec![0u32; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if left[i - 1] == right[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    let mut left_tags = vec![LineKind::Delete; n];
    let mut right_tags = vec![LineKind::Insert; m];
    let mut i = n;
    let mut j = m;
    while i > 0 && j > 0 {
        if left[i - 1] == right[j - 1] {
            left_tags[i - 1] = LineKind::Equal;
            right_tags[j - 1] = LineKind::Equal;
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    (left_tags, right_tags)
}

/// Count insert/delete tags.
pub fn count_changes(left: &[LineKind], right: &[LineKind]) -> (usize, usize) {
    let del = left.iter().filter(|k| **k == LineKind::Delete).count();
    let ins = right.iter().filter(|k| **k == LineKind::Insert).count();
    (del, ins)
}

/// Normalize a line for compare matching (ignore whitespace / case options).
pub fn compare_line_key(line: &str, ignore_ws: bool, ignore_case: bool) -> String {
    let key = if ignore_ws {
        line.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        line.to_string()
    };
    if ignore_case {
        key.to_lowercase()
    } else {
        key
    }
}

/// True when this line starts a change hunk (non-equal after equal, or file start).
fn is_hunk_start(tags: &[LineKind], i: usize) -> bool {
    tags.get(i).is_some_and(|k| *k != LineKind::Equal) && (i == 0 || tags[i - 1] == LineKind::Equal)
}

/// First line of the next change hunk after `from` (wraps). `None` if no changes.
pub fn next_hunk_start(tags: &[LineKind], from: usize) -> Option<usize> {
    let n = tags.len();
    if n == 0 || !tags.iter().any(|k| *k != LineKind::Equal) {
        return None;
    }
    let start = from.min(n.saturating_sub(1));
    for offset in 1..=n {
        let i = (start + offset) % n;
        if is_hunk_start(tags, i) {
            return Some(i);
        }
    }
    None
}

/// First line of the previous change hunk before `from` (wraps). `None` if no changes.
pub fn prev_hunk_start(tags: &[LineKind], from: usize) -> Option<usize> {
    let n = tags.len();
    if n == 0 || !tags.iter().any(|k| *k != LineKind::Equal) {
        return None;
    }
    let start = from.min(n.saturating_sub(1));
    for offset in 1..=n {
        let i = (start + n - offset) % n;
        if is_hunk_start(tags, i) {
            return Some(i);
        }
    }
    None
}

/// Whether a Next/Prev hunk jump wrapped past the end/beginning of the file.
///
/// For Next, wrap means the target line is at or before the caret line (circular).
/// For Prev, wrap means the target is at or after the caret line.
pub fn hunk_nav_wrapped(next: bool, from: usize, to: usize) -> bool {
    if next {
        to <= from
    } else {
        to >= from
    }
}

/// Line indices that start a change hunk (in order).
pub fn hunk_starts(tags: &[LineKind]) -> Vec<usize> {
    (0..tags.len())
        .filter(|&i| is_hunk_start(tags, i))
        .collect()
}

/// 1-based hunk ordinal and total for the hunk that contains `line`, or that
/// starts at `line`. `None` if there are no change hunks.
pub fn hunk_ordinal(tags: &[LineKind], line: usize) -> Option<(usize, usize)> {
    let starts = hunk_starts(tags);
    if starts.is_empty() {
        return None;
    }
    // Prefer the hunk that starts at `line`, else the hunk covering `line`.
    if let Some(idx) = starts.iter().position(|&s| s == line) {
        return Some((idx + 1, starts.len()));
    }
    if tags.get(line).is_some_and(|k| *k != LineKind::Equal) {
        let mut best = 0usize;
        for (i, &s) in starts.iter().enumerate() {
            if s <= line {
                best = i;
            } else {
                break;
            }
        }
        return Some((best + 1, starts.len()));
    }
    None
}

/// Line index of the 1-based hunk ordinal, if that hunk exists on this side.
pub fn hunk_start_at_ordinal(tags: &[LineKind], ordinal_1based: usize) -> Option<usize> {
    if ordinal_1based == 0 {
        return None;
    }
    hunk_starts(tags).get(ordinal_1based - 1).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_files() {
        let a = ["a", "b", "c"];
        let (l, r) = diff_line_tags(&a, &a);
        assert!(l.iter().all(|k| *k == LineKind::Equal));
        assert!(r.iter().all(|k| *k == LineKind::Equal));
    }

    #[test]
    fn insert_middle() {
        let left = ["a", "c"];
        let right = ["a", "b", "c"];
        let (l, r) = diff_line_tags(&left, &right);
        assert_eq!(l, vec![LineKind::Equal, LineKind::Equal]);
        assert_eq!(r, vec![LineKind::Equal, LineKind::Insert, LineKind::Equal]);
    }

    #[test]
    fn ignore_ws_keys_match() {
        let left = ["a  b", "c"];
        let right = ["a b", "c"];
        let left_n: Vec<String> = left
            .iter()
            .map(|s| compare_line_key(s, true, false))
            .collect();
        let right_n: Vec<String> = right
            .iter()
            .map(|s| compare_line_key(s, true, false))
            .collect();
        let lref: Vec<&str> = left_n.iter().map(|s| s.as_str()).collect();
        let rref: Vec<&str> = right_n.iter().map(|s| s.as_str()).collect();
        let (l, r) = diff_line_tags(&lref, &rref);
        assert!(l.iter().all(|k| *k == LineKind::Equal));
        assert!(r.iter().all(|k| *k == LineKind::Equal));
    }

    #[test]
    fn ignore_case_keys_match() {
        let left = ["Hello", "World"];
        let right = ["hello", "WORLD"];
        let left_n: Vec<String> = left
            .iter()
            .map(|s| compare_line_key(s, false, true))
            .collect();
        let right_n: Vec<String> = right
            .iter()
            .map(|s| compare_line_key(s, false, true))
            .collect();
        let lref: Vec<&str> = left_n.iter().map(|s| s.as_str()).collect();
        let rref: Vec<&str> = right_n.iter().map(|s| s.as_str()).collect();
        let (l, r) = diff_line_tags(&lref, &rref);
        assert!(l.iter().all(|k| *k == LineKind::Equal));
        assert!(r.iter().all(|k| *k == LineKind::Equal));
        // Combined with ignore-ws.
        assert_eq!(
            compare_line_key("A  B", true, true),
            compare_line_key("a b", true, true)
        );
    }

    #[test]
    fn hunk_nav_wraps_and_skips_mid_hunk() {
        use LineKind::*;
        let tags = vec![Equal, Delete, Delete, Equal, Insert, Equal];
        assert_eq!(next_hunk_start(&tags, 0), Some(1));
        assert_eq!(next_hunk_start(&tags, 1), Some(4)); // mid-hunk → next hunk
        assert_eq!(next_hunk_start(&tags, 4), Some(1)); // wrap
        assert_eq!(prev_hunk_start(&tags, 4), Some(1));
        assert_eq!(prev_hunk_start(&tags, 1), Some(4)); // wrap
        assert_eq!(next_hunk_start(&[Equal, Equal], 0), None);
        assert!(!hunk_nav_wrapped(true, 0, 1));
        assert!(!hunk_nav_wrapped(true, 1, 4));
        assert!(hunk_nav_wrapped(true, 4, 1));
        assert!(!hunk_nav_wrapped(false, 4, 1));
        assert!(hunk_nav_wrapped(false, 1, 4));
        assert!(hunk_nav_wrapped(true, 1, 1)); // single-hunk Next
        assert!(hunk_nav_wrapped(false, 1, 1)); // single-hunk Prev
    }

    #[test]
    fn hunk_ordinal_counts_and_mid_hunk() {
        use LineKind::*;
        let tags = vec![Equal, Delete, Delete, Equal, Insert, Equal];
        assert_eq!(hunk_starts(&tags), vec![1, 4]);
        assert_eq!(hunk_ordinal(&tags, 1), Some((1, 2)));
        assert_eq!(hunk_ordinal(&tags, 2), Some((1, 2))); // mid-hunk
        assert_eq!(hunk_ordinal(&tags, 4), Some((2, 2)));
        assert_eq!(hunk_ordinal(&tags, 0), None);
        assert_eq!(hunk_ordinal(&[Equal, Equal], 0), None);
        assert_eq!(hunk_start_at_ordinal(&tags, 1), Some(1));
        assert_eq!(hunk_start_at_ordinal(&tags, 2), Some(4));
        assert_eq!(hunk_start_at_ordinal(&tags, 3), None);
        assert_eq!(hunk_start_at_ordinal(&tags, 0), None);
        assert_eq!(hunk_starts(&tags).first().copied(), Some(1));
        assert_eq!(hunk_starts(&tags).last().copied(), Some(4));
        assert!(hunk_starts(&[Equal, Equal]).is_empty());
    }
}
