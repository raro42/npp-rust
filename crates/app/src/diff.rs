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

/// Exclusive end line of the change hunk that starts at `start`.
///
/// `None` when `start` is not a hunk start.
pub fn hunk_end_exclusive(tags: &[LineKind], start: usize) -> Option<usize> {
    if !is_hunk_start(tags, start) {
        return None;
    }
    let mut end = start + 1;
    while end < tags.len() && tags[end] != LineKind::Equal {
        end += 1;
    }
    Some(end)
}

/// Inclusive-start / exclusive-end line range for a 1-based hunk ordinal.
pub fn hunk_line_range(tags: &[LineKind], ordinal_1based: usize) -> Option<(usize, usize)> {
    let start = hunk_start_at_ordinal(tags, ordinal_1based)?;
    let end = hunk_end_exclusive(tags, start)?;
    Some((start, end))
}

const UNIFIED_CONTEXT: usize = 3;

#[derive(Clone, Copy)]
enum AlignOp {
    Equal { left: usize, right: usize },
    Delete { left: usize },
    Insert { right: usize, left_at: usize },
}

fn align_ops(left_tags: &[LineKind], right_tags: &[LineKind]) -> Option<Vec<AlignOp>> {
    let n = left_tags.len();
    let m = right_tags.len();
    let mut ops = Vec::with_capacity(n + m);
    let mut i = 0usize;
    let mut j = 0usize;
    while i < n || j < m {
        let l = left_tags.get(i).copied();
        let r = right_tags.get(j).copied();
        match (l, r) {
            (Some(LineKind::Equal), Some(LineKind::Equal)) => {
                ops.push(AlignOp::Equal { left: i, right: j });
                i += 1;
                j += 1;
            }
            (Some(LineKind::Delete), _) => {
                ops.push(AlignOp::Delete { left: i });
                i += 1;
            }
            (_, Some(LineKind::Insert)) => {
                ops.push(AlignOp::Insert {
                    right: j,
                    left_at: i,
                });
                j += 1;
            }
            (Some(LineKind::Insert), _)
            | (_, Some(LineKind::Delete))
            | (Some(LineKind::Equal), None)
            | (None, Some(LineKind::Equal))
            | (None, None) => return None,
        }
    }
    Some(ops)
}

fn op_is_change(op: AlignOp) -> bool {
    !matches!(op, AlignOp::Equal { .. })
}

/// Unified diff of tagged line lists (GNU-style, 3 lines of context).
///
/// `None` when tag lengths do not match the lines, or tags cannot be aligned.
pub fn unified_diff(
    left: &[&str],
    right: &[&str],
    left_name: &str,
    right_name: &str,
    left_tags: &[LineKind],
    right_tags: &[LineKind],
) -> Option<String> {
    if left.len() != left_tags.len() || right.len() != right_tags.len() {
        return None;
    }
    let ops = align_ops(left_tags, right_tags)?;
    let mut out = String::new();
    out.push_str(&format!("--- {left_name}\n+++ {right_name}\n"));
    if !ops.iter().copied().any(op_is_change) {
        return Some(out);
    }
    let mut i = 0usize;
    while i < ops.len() {
        if !op_is_change(ops[i]) {
            i += 1;
            continue;
        }
        let start = i.saturating_sub(UNIFIED_CONTEXT);
        let mut end = i + 1;
        loop {
            while end < ops.len() && op_is_change(ops[end]) {
                end += 1;
            }
            let mut peek = end;
            let mut equals = 0usize;
            while peek < ops.len() && !op_is_change(ops[peek]) && equals < UNIFIED_CONTEXT * 2 {
                peek += 1;
                equals += 1;
            }
            if peek < ops.len() && op_is_change(ops[peek]) && equals <= UNIFIED_CONTEXT * 2 {
                end = peek + 1;
                continue;
            }
            end = (end + UNIFIED_CONTEXT).min(ops.len());
            break;
        }
        emit_unified_hunk(&mut out, &ops[start..end], left, right);
        i = end;
    }
    Some(out)
}

fn emit_unified_hunk(out: &mut String, hunk: &[AlignOp], left: &[&str], right: &[&str]) {
    let mut old_count = 0usize;
    let mut new_count = 0usize;
    let mut old_start = 0usize;
    let mut new_start = 0usize;
    let mut saw_old = false;
    let mut saw_new = false;
    for op in hunk {
        match *op {
            AlignOp::Equal { left: l, right: r } => {
                old_count += 1;
                new_count += 1;
                if !saw_old {
                    old_start = l + 1;
                    saw_old = true;
                }
                if !saw_new {
                    new_start = r + 1;
                    saw_new = true;
                }
            }
            AlignOp::Delete { left: l } => {
                old_count += 1;
                if !saw_old {
                    old_start = l + 1;
                    saw_old = true;
                }
            }
            AlignOp::Insert { right: r, left_at } => {
                new_count += 1;
                if !saw_new {
                    new_start = r + 1;
                    saw_new = true;
                }
                if !saw_old {
                    old_start = left_at;
                }
            }
        }
    }
    if !saw_new {
        new_start = match hunk.first() {
            Some(AlignOp::Delete { left: l }) => *l,
            Some(AlignOp::Equal { right: r, .. }) => *r,
            _ => 0,
        };
    }
    out.push_str(&format!(
        "@@ -{old_start},{old_count} +{new_start},{new_count} @@\n"
    ));
    for op in hunk {
        match *op {
            AlignOp::Equal { left: l, .. } => {
                out.push(' ');
                out.push_str(left.get(l).copied().unwrap_or(""));
                out.push('\n');
            }
            AlignOp::Delete { left: l } => {
                out.push('-');
                out.push_str(left.get(l).copied().unwrap_or(""));
                out.push('\n');
            }
            AlignOp::Insert { right: r, .. } => {
                out.push('+');
                out.push_str(right.get(r).copied().unwrap_or(""));
                out.push('\n');
            }
        }
    }
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
        assert_eq!(hunk_end_exclusive(&tags, 1), Some(3));
        assert_eq!(hunk_end_exclusive(&tags, 4), Some(5));
        assert_eq!(hunk_end_exclusive(&tags, 2), None); // mid-hunk
        assert_eq!(hunk_line_range(&tags, 1), Some((1, 3)));
        assert_eq!(hunk_line_range(&tags, 2), Some((4, 5)));
        assert_eq!(hunk_line_range(&tags, 3), None);
        // Trailing hunk to EOF.
        let trail = vec![Equal, Delete, Delete];
        assert_eq!(hunk_line_range(&trail, 1), Some((1, 3)));
    }

    #[test]
    fn unified_diff_insert_and_identical() {
        let left = ["a", "c"];
        let right = ["a", "b", "c"];
        let (l, r) = diff_line_tags(&left, &right);
        let text = unified_diff(&left, &right, "old.txt", "new.txt", &l, &r).unwrap();
        assert!(text.starts_with("--- old.txt\n+++ new.txt\n"));
        assert!(text.contains("+b\n"));
        assert!(text.contains(" a\n"));
        assert!(unified_diff(&left, &right, "a", "b", &l, &[]).is_none());
        let (le, re) = diff_line_tags(&left, &left);
        let ident = unified_diff(&left, &left, "a", "b", &le, &re).unwrap();
        assert_eq!(ident, "--- a\n+++ b\n");
    }

    #[test]
    fn unified_diff_delete_replace_and_empty() {
        let left = ["keep", "gone", "tail"];
        let right = ["keep", "here", "tail"];
        let (l, r) = diff_line_tags(&left, &right);
        let text = unified_diff(&left, &right, "L", "R", &l, &r).unwrap();
        assert!(text.contains("-gone\n"));
        assert!(text.contains("+here\n"));
        let empty: [&str; 0] = [];
        let added = ["x"];
        let (l0, r0) = diff_line_tags(&empty, &added);
        let t = unified_diff(&empty, &added, "e", "f", &l0, &r0).unwrap();
        assert!(t.contains("@@ -0,0 +1,1 @@\n"));
        assert!(t.contains("+x\n"));
        let (l1, r1) = diff_line_tags(&added, &empty);
        let t2 = unified_diff(&added, &empty, "e", "f", &l1, &r1).unwrap();
        assert!(t2.contains("-x\n"));
    }
}
