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
            .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        let right_n: Vec<String> = right
            .iter()
            .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        let lref: Vec<&str> = left_n.iter().map(|s| s.as_str()).collect();
        let rref: Vec<&str> = right_n.iter().map(|s| s.as_str()).collect();
        let (l, r) = diff_line_tags(&lref, &rref);
        assert!(l.iter().all(|k| *k == LineKind::Equal));
        assert!(r.iter().all(|k| *k == LineKind::Equal));
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
    }
}
