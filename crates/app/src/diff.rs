//! Line-oriented 2-way diff (LCS). No system `diff` — works on all OSes.

use std::collections::BTreeSet;

/// Per-line tag for one side of a compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    Equal,
    /// Present only on the left (removed vs right).
    Delete,
    /// Present only on the right (added vs left).
    Insert,
}

/// Exclusive-end character range on one compare line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharRange {
    pub start: usize,
    pub end: usize,
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

/// Stronger wash for intra-line char diffs on a replace hunk.
pub fn line_kind_inline_bg(kind: LineKind) -> Option<eframe::egui::Color32> {
    use eframe::egui::Color32;
    match kind {
        LineKind::Equal => None,
        LineKind::Delete => Some(Color32::from_rgba_unmultiplied(210, 40, 40, 150)),
        LineKind::Insert => Some(Color32::from_rgba_unmultiplied(30, 150, 60, 150)),
    }
}

/// Max lines per side for the MVP LCS (O(n·m) memory).
pub const MAX_COMPARE_LINES: usize = 3_000;

/// Skip intra-line LCS when a side is longer than this (chars).
pub const MAX_INLINE_CHARS: usize = 256;

/// LCS match mask: `true` where the item is aligned as equal.
fn lcs_match_mask<T: Eq>(left: &[T], right: &[T]) -> (Vec<bool>, Vec<bool>) {
    let n = left.len();
    let m = right.len();
    let mut left_m = vec![false; n];
    let mut right_m = vec![false; m];
    if n == 0 || m == 0 {
        return (left_m, right_m);
    }
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
    let mut i = n;
    let mut j = m;
    while i > 0 && j > 0 {
        if left[i - 1] == right[j - 1] {
            left_m[i - 1] = true;
            right_m[j - 1] = true;
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    (left_m, right_m)
}

/// Tag each line on left and right using LCS of exact line strings.
pub fn diff_line_tags(left: &[&str], right: &[&str]) -> (Vec<LineKind>, Vec<LineKind>) {
    let (lm, rm) = lcs_match_mask(left, right);
    let left_tags = lm
        .into_iter()
        .map(|eq| {
            if eq {
                LineKind::Equal
            } else {
                LineKind::Delete
            }
        })
        .collect();
    let right_tags = rm
        .into_iter()
        .map(|eq| {
            if eq {
                LineKind::Equal
            } else {
                LineKind::Insert
            }
        })
        .collect();
    (left_tags, right_tags)
}

fn unmatched_runs(matched: &[bool]) -> Vec<CharRange> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < matched.len() {
        if matched[i] {
            i += 1;
            continue;
        }
        let start = i;
        while i < matched.len() && !matched[i] {
            i += 1;
        }
        out.push(CharRange { start, end: i });
    }
    out
}

/// Char-index ranges (exclusive end) that differ between two lines (char LCS).
///
/// `None` when either side is over [`MAX_INLINE_CHARS`] (caller keeps line wash only).
pub fn char_change_spans(left: &str, right: &str) -> Option<(Vec<CharRange>, Vec<CharRange>)> {
    let lc: Vec<char> = left.chars().collect();
    let rc: Vec<char> = right.chars().collect();
    if lc.len() > MAX_INLINE_CHARS || rc.len() > MAX_INLINE_CHARS {
        return None;
    }
    let (lm, rm) = lcs_match_mask(&lc, &rc);
    Some((unmatched_runs(&lm), unmatched_runs(&rm)))
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Word / separator token ranges as char-index half-open spans.
fn tokenize_word_ranges(chars: &[char]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < chars.len() {
        let start = i;
        if is_word_char(chars[i]) {
            i += 1;
            while i < chars.len() && is_word_char(chars[i]) {
                i += 1;
            }
        } else {
            // One separator char per token so spaces and punctuation stay aligned.
            i += 1;
        }
        out.push((start, i));
    }
    out
}

fn merge_char_ranges(mut ranges: Vec<CharRange>) -> Vec<CharRange> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_by_key(|r| r.start);
    let mut out = Vec::with_capacity(ranges.len());
    let mut cur = ranges[0];
    for r in ranges.into_iter().skip(1) {
        if r.start <= cur.end {
            cur.end = cur.end.max(r.end);
        } else {
            out.push(cur);
            cur = r;
        }
    }
    out.push(cur);
    out
}

/// Word-aware intra-line spans: LCS on tokens, char refine on 1:1 token replaces.
///
/// `None` when either side is over [`MAX_INLINE_CHARS`].
pub fn word_change_spans(left: &str, right: &str) -> Option<(Vec<CharRange>, Vec<CharRange>)> {
    let lc: Vec<char> = left.chars().collect();
    let rc: Vec<char> = right.chars().collect();
    if lc.len() > MAX_INLINE_CHARS || rc.len() > MAX_INLINE_CHARS {
        return None;
    }
    let lt = tokenize_word_ranges(&lc);
    let rt = tokenize_word_ranges(&rc);
    if lt.is_empty() && rt.is_empty() {
        return Some((Vec::new(), Vec::new()));
    }
    let lkeys: Vec<String> = lt.iter().map(|&(s, e)| lc[s..e].iter().collect()).collect();
    let rkeys: Vec<String> = rt.iter().map(|&(s, e)| rc[s..e].iter().collect()).collect();
    let (lm, rm) = lcs_match_mask(&lkeys, &rkeys);
    let left_tags: Vec<LineKind> = lm
        .into_iter()
        .map(|eq| {
            if eq {
                LineKind::Equal
            } else {
                LineKind::Delete
            }
        })
        .collect();
    let right_tags: Vec<LineKind> = rm
        .into_iter()
        .map(|eq| {
            if eq {
                LineKind::Equal
            } else {
                LineKind::Insert
            }
        })
        .collect();
    let Some(ops) = align_ops(&left_tags, &right_tags) else {
        // Fall back to char LCS if token tags cannot align.
        return char_change_spans(left, right);
    };
    let mut left_sp = Vec::new();
    let mut right_sp = Vec::new();
    let mut i = 0usize;
    while i < ops.len() {
        if !op_is_change(ops[i]) {
            i += 1;
            continue;
        }
        let mut dels = Vec::new();
        let mut ins = Vec::new();
        while i < ops.len() && op_is_change(ops[i]) {
            match ops[i] {
                AlignOp::Delete { left } => dels.push(left),
                AlignOp::Insert { right, .. } => ins.push(right),
                AlignOp::Equal { .. } => {}
            }
            i += 1;
        }
        let n = dels.len().min(ins.len());
        for k in 0..n {
            let di = dels[k];
            let ii = ins[k];
            let (ls, le) = lt[di];
            let (rs, re) = rt[ii];
            let l_txt: String = lc[ls..le].iter().collect();
            let r_txt: String = rc[rs..re].iter().collect();
            if let Some((ls_rel, rs_rel)) = char_change_spans(&l_txt, &r_txt) {
                for r in ls_rel {
                    left_sp.push(CharRange {
                        start: ls + r.start,
                        end: ls + r.end,
                    });
                }
                for r in rs_rel {
                    right_sp.push(CharRange {
                        start: rs + r.start,
                        end: rs + r.end,
                    });
                }
            } else {
                left_sp.push(CharRange { start: ls, end: le });
                right_sp.push(CharRange { start: rs, end: re });
            }
        }
        for &di in dels.iter().skip(n) {
            let (ls, le) = lt[di];
            left_sp.push(CharRange { start: ls, end: le });
        }
        for &ii in ins.iter().skip(n) {
            let (rs, re) = rt[ii];
            right_sp.push(CharRange { start: rs, end: re });
        }
    }
    Some((merge_char_ranges(left_sp), merge_char_ranges(right_sp)))
}

/// Intra-line spans for paired delete/insert lines in each change hunk.
///
/// Uses word-aware LCS (with char refine on 1:1 token replaces). Unpaired
/// insert/delete lines stay empty (full-line wash only).
pub fn inline_change_spans(
    left: &[&str],
    right: &[&str],
    left_tags: &[LineKind],
    right_tags: &[LineKind],
) -> (Vec<Vec<CharRange>>, Vec<Vec<CharRange>>) {
    let mut left_sp = vec![Vec::new(); left.len()];
    let mut right_sp = vec![Vec::new(); right.len()];
    if left.len() != left_tags.len() || right.len() != right_tags.len() {
        return (left_sp, right_sp);
    }
    let Some(ops) = align_ops(left_tags, right_tags) else {
        return (left_sp, right_sp);
    };
    let mut i = 0usize;
    while i < ops.len() {
        if !op_is_change(ops[i]) {
            i += 1;
            continue;
        }
        let mut dels = Vec::new();
        let mut ins = Vec::new();
        while i < ops.len() && op_is_change(ops[i]) {
            match ops[i] {
                AlignOp::Delete { left } => dels.push(left),
                AlignOp::Insert { right, .. } => ins.push(right),
                AlignOp::Equal { .. } => {}
            }
            i += 1;
        }
        let n = dels.len().min(ins.len());
        for k in 0..n {
            let li = dels[k];
            let ri = ins[k];
            let Some(l_txt) = left.get(li).copied() else {
                continue;
            };
            let Some(r_txt) = right.get(ri).copied() else {
                continue;
            };
            if let Some((ls, rs)) = word_change_spans(l_txt, r_txt) {
                left_sp[li] = ls;
                right_sp[ri] = rs;
            }
        }
    }
    (left_sp, right_sp)
}

/// Count insert/delete tags.
pub fn count_changes(left: &[LineKind], right: &[LineKind]) -> (usize, usize) {
    let del = left.iter().filter(|k| **k == LineKind::Delete).count();
    let ins = right.iter().filter(|k| **k == LineKind::Insert).count();
    (del, ins)
}

/// Equal lines kept visible on each side of a change when hide-equal is on.
pub const COMPARE_HIDE_EQUAL_CONTEXT: usize = 3;

/// Hidden Equal counts after subtracting click-expanded (revealed) lines.
///
/// Pass empty revealed sets for the raw hide-equal totals.
pub fn hidden_equal_counts_with_revealed(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    left_revealed: &BTreeSet<usize>,
    right_revealed: &BTreeSet<usize>,
    context: usize,
) -> (usize, usize) {
    let left = equal_line_indices_to_hide_with_context(left_tags, context);
    let right = equal_line_indices_to_hide_with_context(right_tags, context);
    (
        left.difference(left_revealed).count(),
        right.difference(right_revealed).count(),
    )
}

/// Drop revealed indices that are no longer in the hide-equal set (after re-diff).
pub fn prune_compare_hide_revealed(
    revealed: &mut BTreeSet<usize>,
    tags: &[LineKind],
    context: usize,
) {
    let hide = equal_line_indices_to_hide_with_context(tags, context);
    revealed.retain(|i| hide.contains(i));
}

/// Skipped document lines between two consecutive visible rows (hide-equal gap).
pub fn compare_visible_gap(prev_doc_line: usize, cur_doc_line: usize) -> Option<usize> {
    compare_gap_line_range(prev_doc_line, cur_doc_line).map(|(a, b)| b - a + 1)
}

/// Inclusive document-line range collapsed between two consecutive visible rows.
pub fn compare_gap_line_range(prev_doc_line: usize, cur_doc_line: usize) -> Option<(usize, usize)> {
    if cur_doc_line > prev_doc_line + 1 {
        Some((prev_doc_line + 1, cur_doc_line - 1))
    } else {
        None
    }
}

/// Hidden Equal run nearest `caret_line` given consecutive visible document rows.
///
/// Prefers a gap that contains the caret. Otherwise the gap below the caret's
/// visible row, then the gap above (same hairline a click on ···N would hit).
pub fn nearest_compare_hide_gap(
    caret_line: usize,
    visible_lines: &[usize],
) -> Option<(usize, usize)> {
    if visible_lines.len() < 2 {
        return None;
    }
    for pair in visible_lines.windows(2) {
        if let Some(gap) = compare_gap_line_range(pair[0], pair[1]) {
            if caret_line >= gap.0 && caret_line <= gap.1 {
                return Some(gap);
            }
        }
    }
    let idx = visible_lines.iter().position(|&l| l == caret_line)?;
    let below = visible_lines
        .get(idx + 1)
        .and_then(|&next| compare_gap_line_range(visible_lines[idx], next));
    if below.is_some() {
        return below;
    }
    idx.checked_sub(1)
        .and_then(|prev| compare_gap_line_range(visible_lines[prev], visible_lines[idx]))
}

/// All hide-equal gaps from consecutive visible rows, in document order.
pub fn list_compare_hide_gaps(visible_lines: &[usize]) -> Vec<(usize, usize)> {
    let mut gaps = Vec::new();
    for pair in visible_lines.windows(2) {
        if let Some(gap) = compare_gap_line_range(pair[0], pair[1]) {
            gaps.push(gap);
        }
    }
    gaps
}

/// Visible park line for a hide-equal gap (row above the ···N cue).
pub fn compare_hide_gap_park_line(gap: (usize, usize)) -> usize {
    gap.0.saturating_sub(1)
}

fn caret_on_compare_hide_gap(caret_line: usize, gap: (usize, usize)) -> bool {
    let park = compare_hide_gap_park_line(gap);
    caret_line == park || (caret_line >= gap.0 && caret_line <= gap.1)
}

/// Next collapsed Equal gap after `caret_line` (wraps). `None` if no gaps.
pub fn next_compare_hide_gap(caret_line: usize, gaps: &[(usize, usize)]) -> Option<(usize, usize)> {
    if gaps.is_empty() {
        return None;
    }
    if let Some(i) = gaps
        .iter()
        .position(|&g| caret_on_compare_hide_gap(caret_line, g))
    {
        return Some(gaps[(i + 1) % gaps.len()]);
    }
    let idx = gaps
        .iter()
        .position(|&g| compare_hide_gap_park_line(g) > caret_line)
        .unwrap_or(0);
    Some(gaps[idx])
}

/// Previous collapsed Equal gap before `caret_line` (wraps). `None` if no gaps.
pub fn prev_compare_hide_gap(caret_line: usize, gaps: &[(usize, usize)]) -> Option<(usize, usize)> {
    if gaps.is_empty() {
        return None;
    }
    if let Some(i) = gaps
        .iter()
        .position(|&g| caret_on_compare_hide_gap(caret_line, g))
    {
        let prev = if i == 0 { gaps.len() - 1 } else { i - 1 };
        return Some(gaps[prev]);
    }
    let idx = gaps
        .iter()
        .rposition(|&g| compare_hide_gap_park_line(g) < caret_line)
        .unwrap_or(gaps.len() - 1);
    Some(gaps[idx])
}

/// 1-based ordinal of `gap` in `gaps`, or `None` if missing.
pub fn compare_hide_gap_ordinal(
    gaps: &[(usize, usize)],
    gap: (usize, usize),
) -> Option<(usize, usize)> {
    let idx = gaps.iter().position(|&g| g == gap)?;
    Some((idx + 1, gaps.len()))
}

/// Insert every line in an inclusive gap into `revealed`. Returns how many were new.
pub fn reveal_compare_gap(revealed: &mut BTreeSet<usize>, gap: (usize, usize)) -> usize {
    let mut n = 0usize;
    for i in gap.0..=gap.1 {
        if revealed.insert(i) {
            n += 1;
        }
    }
    n
}

/// Insert each index in `lines` into `revealed`. Returns how many were new.
pub fn reveal_compare_lines(revealed: &mut BTreeSet<usize>, lines: &[usize]) -> usize {
    let mut n = 0usize;
    for &i in lines {
        if revealed.insert(i) {
            n += 1;
        }
    }
    n
}

/// Reveal every Equal line currently hidden by hide-equal. Returns how many were new.
pub fn reveal_all_compare_hidden(
    revealed: &mut BTreeSet<usize>,
    tags: &[LineKind],
    context: usize,
) -> usize {
    let hide = equal_line_indices_to_hide_with_context(tags, context);
    let mut n = 0usize;
    for i in hide {
        if revealed.insert(i) {
            n += 1;
        }
    }
    n
}

/// Clear click/menu-expanded Equal lines so hide-equal collapses them again.
///
/// Returns how many indices were dropped. Hide-equal preference is unchanged.
pub fn collapse_all_compare_revealed(revealed: &mut BTreeSet<usize>) -> usize {
    let n = revealed.len();
    revealed.clear();
    n
}

/// Contiguous revealed Equal run nearest `caret_line`.
///
/// Prefers a run that contains the caret. Otherwise the closest revealed index
/// (ties prefer below), expanded to its contiguous block.
pub fn nearest_compare_revealed_run(
    caret_line: usize,
    revealed: &BTreeSet<usize>,
) -> Option<(usize, usize)> {
    if revealed.is_empty() {
        return None;
    }
    let pick = if revealed.contains(&caret_line) {
        caret_line
    } else {
        let below = revealed.range(caret_line..).next().copied();
        let above = revealed.range(..caret_line).next_back().copied();
        match (above, below) {
            (Some(a), Some(b)) => {
                if caret_line - a <= b - caret_line {
                    a
                } else {
                    b
                }
            }
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => return None,
        }
    };
    let mut lo = pick;
    let mut hi = pick;
    while lo > 0 && revealed.contains(&(lo - 1)) {
        lo -= 1;
    }
    while revealed.contains(&(hi + 1)) {
        hi += 1;
    }
    Some((lo, hi))
}

/// Remove every line in an inclusive gap from `revealed`. Returns how many were dropped.
pub fn collapse_compare_gap(revealed: &mut BTreeSet<usize>, gap: (usize, usize)) -> usize {
    let mut n = 0usize;
    for i in gap.0..=gap.1 {
        if revealed.remove(&i) {
            n += 1;
        }
    }
    n
}

/// Remove each index in `lines` from `revealed`. Returns how many were dropped.
pub fn collapse_compare_lines(revealed: &mut BTreeSet<usize>, lines: &[usize]) -> usize {
    let mut n = 0usize;
    for &i in lines {
        if revealed.remove(&i) {
            n += 1;
        }
    }
    n
}

/// Partner Equal line indices on the other pane for a hide-equal gap.
///
/// Uses LCS alignment so a ···N click can expand the matching Equal run on both sides.
pub fn aligned_equal_partner_lines(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    gap: (usize, usize),
    gap_is_left: bool,
) -> Vec<usize> {
    let Some(ops) = align_ops(left_tags, right_tags) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for op in ops {
        let AlignOp::Equal { left, right } = op else {
            continue;
        };
        let mine = if gap_is_left { left } else { right };
        if mine >= gap.0 && mine <= gap.1 {
            out.push(if gap_is_left { right } else { left });
        }
    }
    out
}

/// LCS-aligned Equal partner line on the other pane for a single Equal line.
///
/// `None` when `line` is not Equal on the focus side, tags cannot be aligned, or
/// there is no Equal partner (should not happen for well-formed Equal tags).
pub fn aligned_equal_partner_line(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    line: usize,
    from_left: bool,
) -> Option<usize> {
    let focus_tags = if from_left { left_tags } else { right_tags };
    if focus_tags.get(line) != Some(&LineKind::Equal) {
        return None;
    }
    let ops = align_ops(left_tags, right_tags)?;
    for op in ops {
        let AlignOp::Equal { left, right } = op else {
            continue;
        };
        if from_left && left == line {
            return Some(right);
        }
        if !from_left && right == line {
            return Some(left);
        }
    }
    None
}

/// Equal-tagged line indices to hide when "Hide Unchanged Lines" is on.
///
/// Keeps `context` Equal lines before/after each change line so hunks stay
/// readable. Returns empty when the side has no changes (identical / empty),
/// so the pane does not go blank.
pub fn equal_line_indices_to_hide_with_context(
    tags: &[LineKind],
    context: usize,
) -> BTreeSet<usize> {
    if !tags.iter().any(|k| *k != LineKind::Equal) {
        return BTreeSet::new();
    }
    let last = tags.len().saturating_sub(1);
    let mut keep_equal = BTreeSet::new();
    for (i, kind) in tags.iter().enumerate() {
        if *kind == LineKind::Equal {
            continue;
        }
        let start = i.saturating_sub(context);
        let end = (i + context).min(last);
        for (j, kind) in tags.iter().enumerate().take(end + 1).skip(start) {
            if *kind == LineKind::Equal {
                keep_equal.insert(j);
            }
        }
    }
    tags.iter()
        .enumerate()
        .filter(|(i, k)| **k == LineKind::Equal && !keep_equal.contains(i))
        .map(|(i, _)| i)
        .collect()
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

/// True when a compare key (or raw line) is blank / whitespace-only.
pub fn is_compare_blank_line(line: &str) -> bool {
    line.is_empty() || line.chars().all(|c| c.is_whitespace())
}

/// Line tags with blank lines forced to [`LineKind::Equal`] (skipped in LCS).
///
/// Non-blank lines still LCS-align among themselves. Extra blank lines on either
/// side do not create insert/delete tags when `ignore_blank` is on.
pub fn diff_line_tags_ignore_blank(
    left: &[&str],
    right: &[&str],
    ignore_blank: bool,
) -> (Vec<LineKind>, Vec<LineKind>) {
    if !ignore_blank {
        return diff_line_tags(left, right);
    }
    let left_idx: Vec<usize> = left
        .iter()
        .enumerate()
        .filter(|(_, s)| !is_compare_blank_line(s))
        .map(|(i, _)| i)
        .collect();
    let right_idx: Vec<usize> = right
        .iter()
        .enumerate()
        .filter(|(_, s)| !is_compare_blank_line(s))
        .map(|(i, _)| i)
        .collect();
    let left_f: Vec<&str> = left_idx.iter().map(|&i| left[i]).collect();
    let right_f: Vec<&str> = right_idx.iter().map(|&i| right[i]).collect();
    let (lt_f, rt_f) = diff_line_tags(&left_f, &right_f);
    let mut left_tags = vec![LineKind::Equal; left.len()];
    let mut right_tags = vec![LineKind::Equal; right.len()];
    for (k, &i) in left_idx.iter().enumerate() {
        left_tags[i] = lt_f[k];
    }
    for (k, &i) in right_idx.iter().enumerate() {
        right_tags[i] = rt_f[k];
    }
    (left_tags, right_tags)
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

fn hunk_line_for_copy(tags: &[LineKind], line: usize) -> Option<usize> {
    if tags.get(line).is_some_and(|k| *k != LineKind::Equal) {
        let (ord, _) = hunk_ordinal(tags, line)?;
        hunk_start_at_ordinal(tags, ord)
    } else {
        next_hunk_start(tags, line)
    }
}

fn change_run_containing(
    ops: &[AlignOp],
    focus_left: bool,
    start_line: usize,
) -> Option<(usize, usize)> {
    let idx = ops.iter().position(|op| match *op {
        AlignOp::Delete { left } if focus_left && left == start_line => true,
        AlignOp::Insert { right, .. } if !focus_left && right == start_line => true,
        _ => false,
    })?;
    let mut start = idx;
    while start > 0 && op_is_change(ops[start - 1]) {
        start -= 1;
    }
    let mut end = idx + 1;
    while end < ops.len() && op_is_change(ops[end]) {
        end += 1;
    }
    Some((start, end))
}

fn expand_equal_context(ops: &[AlignOp], run_start: usize, run_end: usize) -> (usize, usize) {
    let mut start = run_start;
    let mut taken = 0usize;
    while start > 0 && taken < UNIFIED_CONTEXT && !op_is_change(ops[start - 1]) {
        start -= 1;
        taken += 1;
    }
    let mut end = run_end;
    taken = 0;
    while end < ops.len() && taken < UNIFIED_CONTEXT && !op_is_change(ops[end]) {
        end += 1;
        taken += 1;
    }
    (start, end)
}

/// Unified diff of one change hunk (3 equal context lines, not merged with neighbors).
///
/// `focus_left` + `line` pick the hunk (caret line on that side). Equal lines use the
/// next hunk (wraps). `None` when there is no change hunk or tags cannot be aligned.
#[allow(clippy::too_many_arguments)]
pub fn unified_diff_hunk(
    left: &[&str],
    right: &[&str],
    left_name: &str,
    right_name: &str,
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    focus_left: bool,
    line: usize,
) -> Option<String> {
    if left.len() != left_tags.len() || right.len() != right_tags.len() {
        return None;
    }
    let tags = if focus_left { left_tags } else { right_tags };
    let start_line = hunk_line_for_copy(tags, line)?;
    let ops = align_ops(left_tags, right_tags)?;
    let (run_start, run_end) = change_run_containing(&ops, focus_left, start_line)?;
    let (start, end) = expand_equal_context(&ops, run_start, run_end);
    let mut out = String::new();
    out.push_str(&format!("--- {left_name}\n+++ {right_name}\n"));
    emit_unified_hunk(&mut out, &ops[start..end], left, right);
    Some(out)
}

/// Ordinal of the hunk that `unified_diff_hunk` would copy for this caret line.
pub fn hunk_ordinal_for_copy(tags: &[LineKind], line: usize) -> Option<(usize, usize)> {
    let start = hunk_line_for_copy(tags, line)?;
    hunk_ordinal(tags, start)
}

/// After applying hunk `applied_ord` (1-based), which remaining hunk to park on.
/// Returns `None` when the pair is identical (`remaining == 0`).
pub fn next_hunk_after_apply(applied_ord: usize, remaining: usize) -> Option<usize> {
    if remaining == 0 || applied_ord == 0 {
        None
    } else {
        Some(applied_ord.min(remaining))
    }
}

/// Line range to replace on the focused side, and the other side's source lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HunkApply {
    pub dest_start: usize,
    pub dest_end: usize,
    pub src_start: usize,
    pub src_end: usize,
    pub ordinal: usize,
    pub total: usize,
}

fn grow_line_span(lo: &mut Option<usize>, hi: &mut Option<usize>, i: usize) {
    *lo = Some(lo.map_or(i, |x: usize| x.min(i)));
    *hi = Some(hi.map_or(i + 1, |x: usize| x.max(i + 1)));
}

fn left_pos_before_op(ops: &[AlignOp], idx: usize) -> usize {
    if idx == 0 {
        return 0;
    }
    match ops[idx - 1] {
        AlignOp::Equal { left, .. } | AlignOp::Delete { left } => left + 1,
        AlignOp::Insert { left_at, .. } => left_at,
    }
}

fn right_pos_before_op(ops: &[AlignOp], idx: usize) -> usize {
    if idx == 0 {
        return 0;
    }
    match ops[idx - 1] {
        AlignOp::Equal { right, .. } | AlignOp::Insert { right, .. } => right + 1,
        AlignOp::Delete { .. } => right_pos_before_op(ops, idx - 1),
    }
}

fn hunk_apply_from_run(
    ops: &[AlignOp],
    run_start: usize,
    run_end: usize,
    focus_left: bool,
    ordinal: usize,
    total: usize,
) -> Option<HunkApply> {
    let mut dest_lo = None;
    let mut dest_hi = None;
    let mut src_lo = None;
    let mut src_hi = None;
    for op in &ops[run_start..run_end] {
        match *op {
            AlignOp::Equal { .. } => {}
            AlignOp::Delete { left: li } => {
                if focus_left {
                    grow_line_span(&mut dest_lo, &mut dest_hi, li);
                } else {
                    grow_line_span(&mut src_lo, &mut src_hi, li);
                }
            }
            AlignOp::Insert { right: ri, .. } => {
                if focus_left {
                    grow_line_span(&mut src_lo, &mut src_hi, ri);
                } else {
                    grow_line_span(&mut dest_lo, &mut dest_hi, ri);
                }
            }
        }
    }
    let (dest_start, dest_end) = match (dest_lo, dest_hi) {
        (Some(a), Some(b)) => (a, b),
        (None, None) => {
            let at = if focus_left {
                left_pos_before_op(ops, run_start)
            } else {
                right_pos_before_op(ops, run_start)
            };
            (at, at)
        }
        _ => return None,
    };
    let (src_start, src_end) = match (src_lo, src_hi) {
        (Some(a), Some(b)) => (a, b),
        (None, None) => (0, 0),
        _ => return None,
    };
    Some(HunkApply {
        dest_start,
        dest_end,
        src_start,
        src_end,
        ordinal,
        total,
    })
}

fn change_runs(ops: &[AlignOp]) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut i = 0usize;
    while i < ops.len() {
        if !op_is_change(ops[i]) {
            i += 1;
            continue;
        }
        let start = i;
        let mut end = i + 1;
        while end < ops.len() && op_is_change(ops[end]) {
            end += 1;
        }
        runs.push((start, end));
        i = end;
    }
    runs
}

/// Replace the focused pane's change hunk with the other pane's lines.
///
/// Caret on an equal line uses the next hunk (wraps), matching copy-hunk.
/// Insert-only / delete-only hunks yield an empty dest or src range.
pub fn hunk_apply_from_other(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    focus_left: bool,
    line: usize,
) -> Option<HunkApply> {
    hunk_apply_for_hunk(left_tags, right_tags, focus_left, line, focus_left)
}

/// Replace the other pane's change hunk with the focused pane's lines.
///
/// Same caret / next-hunk rules as [`hunk_apply_from_other`], but the destination
/// is the non-focused side (push instead of pull).
pub fn hunk_apply_to_other(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    focus_left: bool,
    line: usize,
) -> Option<HunkApply> {
    hunk_apply_for_hunk(left_tags, right_tags, focus_left, line, !focus_left)
}

fn hunk_apply_for_hunk(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    focus_left: bool,
    line: usize,
    dest_left: bool,
) -> Option<HunkApply> {
    let focus_tags = if focus_left { left_tags } else { right_tags };
    let other_tags = if focus_left { right_tags } else { left_tags };
    let (run_focus_left, start_line) = if let Some(start) = hunk_line_for_copy(focus_tags, line) {
        (focus_left, start)
    } else {
        let start = hunk_line_for_copy(other_tags, line)?;
        (!focus_left, start)
    };
    let ops = align_ops(left_tags, right_tags)?;
    let (run_start, run_end) = change_run_containing(&ops, run_focus_left, start_line)?;
    let (ordinal, total) = hunk_ordinal_for_copy(focus_tags, line)
        .or_else(|| hunk_ordinal_for_copy(other_tags, line))?;
    hunk_apply_from_run(&ops, run_start, run_end, dest_left, ordinal, total)
}

/// Every change hunk as an apply spec (file order, 1-based ordinals).
///
/// Destination is the focused pane (pull from the other side).
/// `None` when tags cannot be aligned or there are no change hunks.
pub fn hunk_apply_all_from_other(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    focus_left: bool,
) -> Option<Vec<HunkApply>> {
    hunk_apply_all_for_dest(left_tags, right_tags, focus_left)
}

/// Every change hunk as an apply spec pushing onto the other pane.
///
/// Same run order as [`hunk_apply_all_from_other`], but destination is the
/// non-focused side.
pub fn hunk_apply_all_to_other(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    focus_left: bool,
) -> Option<Vec<HunkApply>> {
    hunk_apply_all_for_dest(left_tags, right_tags, !focus_left)
}

fn hunk_apply_all_for_dest(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    dest_left: bool,
) -> Option<Vec<HunkApply>> {
    let ops = align_ops(left_tags, right_tags)?;
    let runs = change_runs(&ops);
    if runs.is_empty() {
        return None;
    }
    let total = runs.len();
    let mut specs = Vec::with_capacity(total);
    for (i, (run_start, run_end)) in runs.into_iter().enumerate() {
        specs.push(hunk_apply_from_run(
            &ops,
            run_start,
            run_end,
            dest_left,
            i + 1,
            total,
        )?);
    }
    Some(specs)
}

/// Delete/insert counts for the change run of a hunk copy (no context equals).
pub fn hunk_copy_change_counts(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    focus_left: bool,
    line: usize,
) -> Option<(usize, usize)> {
    let tags = if focus_left { left_tags } else { right_tags };
    let start_line = hunk_line_for_copy(tags, line)?;
    let ops = align_ops(left_tags, right_tags)?;
    let (run_start, run_end) = change_run_containing(&ops, focus_left, start_line)?;
    let mut del = 0usize;
    let mut ins = 0usize;
    for op in &ops[run_start..run_end] {
        match *op {
            AlignOp::Delete { .. } => del += 1,
            AlignOp::Insert { .. } => ins += 1,
            AlignOp::Equal { .. } => {}
        }
    }
    Some((del, ins))
}

/// One paired change hunk for a Compare summary list (0-based exclusive ends).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompareSummaryHunk {
    pub ordinal: usize,
    pub left_start: usize,
    pub left_end: usize,
    pub right_start: usize,
    pub right_end: usize,
    pub deletes: usize,
    pub inserts: usize,
}

/// Paired change hunks in LCS order (`None` when tags cannot be aligned).
pub fn compare_summary_hunks(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
) -> Option<Vec<CompareSummaryHunk>> {
    let ops = align_ops(left_tags, right_tags)?;
    let runs = change_runs(&ops);
    let total = runs.len();
    let mut out = Vec::with_capacity(total);
    for (i, (run_start, run_end)) in runs.into_iter().enumerate() {
        let mut left_lo = None;
        let mut left_hi = None;
        let mut right_lo = None;
        let mut right_hi = None;
        let mut deletes = 0usize;
        let mut inserts = 0usize;
        for op in &ops[run_start..run_end] {
            match *op {
                AlignOp::Delete { left } => {
                    grow_line_span(&mut left_lo, &mut left_hi, left);
                    deletes += 1;
                }
                AlignOp::Insert { right, .. } => {
                    grow_line_span(&mut right_lo, &mut right_hi, right);
                    inserts += 1;
                }
                AlignOp::Equal { .. } => {}
            }
        }
        let left_at = left_pos_before_op(&ops, run_start);
        let right_at = right_pos_before_op(&ops, run_start);
        out.push(CompareSummaryHunk {
            ordinal: i + 1,
            left_start: left_lo.unwrap_or(left_at),
            left_end: left_hi.unwrap_or(left_at),
            right_start: right_lo.unwrap_or(right_at),
            right_end: right_hi.unwrap_or(right_at),
            deletes,
            inserts,
        });
    }
    Some(out)
}

fn format_summary_span(start: usize, end: usize) -> String {
    if start >= end || end == start + 1 {
        format!("{}", start + 1)
    } else {
        format!("{}–{}", start + 1, end)
    }
}

/// Max display chars for one Compare Summary preview line.
pub const SUMMARY_PREVIEW_MAX: usize = 72;

/// Max preview lines per side (− / +) under each Compare Summary hunk.
pub const SUMMARY_PREVIEW_LINES: usize = 3;

fn summary_preview_line(line: &str) -> String {
    let flat: String = line
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    let trimmed = flat.trim_end();
    if trimmed.chars().count() <= SUMMARY_PREVIEW_MAX {
        return trimmed.to_string();
    }
    let mut s: String = trimmed
        .chars()
        .take(SUMMARY_PREVIEW_MAX.saturating_sub(1))
        .collect();
    s.push('…');
    s
}

fn push_summary_preview(out: &mut String, marker: char, line: &str) {
    out.push_str("   ");
    out.push(marker);
    out.push(' ');
    out.push_str(&summary_preview_line(line));
    out.push('\n');
}

/// Push up to [`SUMMARY_PREVIEW_LINES`] from `lines[start..end]`, then
/// `… (+N more)` when the hunk side is longer.
fn push_summary_side_previews(
    out: &mut String,
    marker: char,
    lines: &[&str],
    start: usize,
    end: usize,
) {
    if start >= end {
        return;
    }
    let total = end - start;
    let show = total.min(SUMMARY_PREVIEW_LINES);
    for i in 0..show {
        if let Some(line) = lines.get(start + i) {
            push_summary_preview(out, marker, line);
        }
    }
    if total > show {
        out.push_str(&format!("   … (+{} more)\n", total - show));
    }
}

/// Count summary hunks by kind (`delete` / `insert` / `replace`).
fn summary_kind_tallies(hunks: &[CompareSummaryHunk]) -> (usize, usize, usize) {
    let mut deletes = 0usize;
    let mut inserts = 0usize;
    let mut replaces = 0usize;
    for h in hunks {
        match (h.deletes, h.inserts) {
            (0, _) => inserts += 1,
            (_, 0) => deletes += 1,
            _ => replaces += 1,
        }
    }
    (deletes, inserts, replaces)
}

fn format_summary_kind_tallies(hunks: &[CompareSummaryHunk]) -> String {
    let (deletes, inserts, replaces) = summary_kind_tallies(hunks);
    let mut parts = Vec::with_capacity(3);
    if deletes > 0 {
        parts.push(format!("{deletes} delete"));
    }
    if inserts > 0 {
        parts.push(format!("{inserts} insert"));
    }
    if replaces > 0 {
        parts.push(format!("{replaces} replace"));
    }
    parts.join(", ")
}

/// Status/summary suffix for active Compare ignore toggles
/// (`""`, or e.g. `" · ignore ws+case"`).
pub fn compare_ignore_note(ignore_ws: bool, ignore_case: bool, ignore_blank: bool) -> String {
    let mut parts = Vec::new();
    if ignore_ws {
        parts.push("ws");
    }
    if ignore_case {
        parts.push("case");
    }
    if ignore_blank {
        parts.push("blank");
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!(" · ignore {}", parts.join("+"))
    }
}

/// Human-readable Compare hunk index with multi-line previews
/// (`None` when tags cannot be aligned).
///
/// Each side shows up to [`SUMMARY_PREVIEW_LINES`] truncated lines
/// (`-` / `+`, max [`SUMMARY_PREVIEW_MAX`] chars), then `… (+N more)`
/// when the hunk side is longer.
///
/// `ignore_note` is appended after the header paren (usually
/// [`compare_ignore_note`], or `""` when no ignore toggles are on).
pub fn compare_summary_text(
    left_name: &str,
    right_name: &str,
    left_lines: &[&str],
    right_lines: &[&str],
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    ignore_note: &str,
) -> Option<String> {
    if left_lines.len() != left_tags.len() || right_lines.len() != right_tags.len() {
        return None;
    }
    let hunks = compare_summary_hunks(left_tags, right_tags)?;
    let (del, ins) = count_changes(left_tags, right_tags);
    let mut out = String::new();
    let kind_bit = if hunks.is_empty() {
        String::new()
    } else {
        format!(": {}", format_summary_kind_tallies(&hunks))
    };
    out.push_str(&format!(
        "Compare summary: “{left_name}” | “{right_name}” (−{del} +{ins}, {} hunks{kind_bit}){ignore_note}\n",
        hunks.len()
    ));
    if hunks.is_empty() {
        out.push_str("(identical)\n");
        return Some(out);
    }
    for h in &hunks {
        let kind = match (h.deletes, h.inserts) {
            (0, _) => "insert",
            (_, 0) => "delete",
            _ => "replace",
        };
        out.push_str(&format!(
            "{}. L{} | R{} (−{} +{}) {}\n",
            h.ordinal,
            format_summary_span(h.left_start, h.left_end),
            format_summary_span(h.right_start, h.right_end),
            h.deletes,
            h.inserts,
            kind,
        ));
        if h.deletes > 0 {
            push_summary_side_previews(&mut out, '-', left_lines, h.left_start, h.left_end);
        }
        if h.inserts > 0 {
            push_summary_side_previews(&mut out, '+', right_lines, h.right_start, h.right_end);
        }
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
    fn char_change_spans_marks_middle() {
        let (l, r) = char_change_spans("hello", "hallo").unwrap();
        assert_eq!(l, vec![CharRange { start: 1, end: 2 }]);
        assert_eq!(r, vec![CharRange { start: 1, end: 2 }]);
    }

    #[test]
    fn word_change_spans_washes_whole_word() {
        let (l, r) = word_change_spans("the cat sat", "the dog sat").unwrap();
        assert_eq!(l, vec![CharRange { start: 4, end: 7 }]);
        assert_eq!(r, vec![CharRange { start: 4, end: 7 }]);
    }

    #[test]
    fn word_change_spans_refines_typo_inside_token() {
        let (l, r) = word_change_spans("hello", "hallo").unwrap();
        assert_eq!(l, vec![CharRange { start: 1, end: 2 }]);
        assert_eq!(r, vec![CharRange { start: 1, end: 2 }]);
    }

    #[test]
    fn inline_spans_pair_replace_not_pure_insert() {
        let left = ["keep", "abc", "tail"];
        let right = ["keep", "axc", "tail"];
        let (lt, rt) = diff_line_tags(&left, &right);
        let (ls, rs) = inline_change_spans(&left, &right, &lt, &rt);
        assert_eq!(ls[1], vec![CharRange { start: 1, end: 2 }]);
        assert_eq!(rs[1], vec![CharRange { start: 1, end: 2 }]);
        assert!(ls[0].is_empty() && rs[0].is_empty());

        let left2 = ["a", "c"];
        let right2 = ["a", "b", "c"];
        let (l2, r2) = diff_line_tags(&left2, &right2);
        let (ls2, rs2) = inline_change_spans(&left2, &right2, &l2, &r2);
        assert!(ls2.iter().all(|s| s.is_empty()));
        assert!(rs2[1].is_empty());
    }

    #[test]
    fn inline_spans_word_level_on_replace_line() {
        let left = ["keep", "foo bar baz", "tail"];
        let right = ["keep", "foo qux baz", "tail"];
        let (lt, rt) = diff_line_tags(&left, &right);
        let (ls, rs) = inline_change_spans(&left, &right, &lt, &rt);
        assert_eq!(ls[1], vec![CharRange { start: 4, end: 7 }]);
        assert_eq!(rs[1], vec![CharRange { start: 4, end: 7 }]);
    }

    #[test]
    fn char_change_spans_skips_long_lines() {
        let long: String = "x".repeat(MAX_INLINE_CHARS + 1);
        assert!(char_change_spans(&long, "y").is_none());
        assert!(word_change_spans(&long, "y").is_none());
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
    fn equal_line_indices_to_hide_skips_identical() {
        use LineKind::*;
        assert!(equal_line_indices_to_hide_with_context(
            &[Equal, Equal],
            COMPARE_HIDE_EQUAL_CONTEXT
        )
        .is_empty());
        // Context 0: hide every Equal when changes exist.
        let hide0 = equal_line_indices_to_hide_with_context(&[Equal, Delete, Equal, Insert], 0);
        assert_eq!(hide0.iter().copied().collect::<Vec<_>>(), vec![0, 2]);
        assert!(!hide0.contains(&1));
        assert!(!hide0.contains(&3));
        // Default ±3 context keeps nearby Equals; far Equals stay hidden.
        let tags = vec![
            Equal, Equal, Equal, Equal, Delete, Equal, Equal, Equal, Equal, Insert, Equal, Equal,
            Equal, Equal,
        ];
        let hide = equal_line_indices_to_hide_with_context(&tags, COMPARE_HIDE_EQUAL_CONTEXT);
        // Change at 4 keeps Equals 1..=3 and 5..=7; change at 9 keeps 6..=8 and 10..=12.
        // Far Equals 0 and 13 are hidden.
        assert_eq!(hide.iter().copied().collect::<Vec<_>>(), vec![0, 13]);
        assert!(!hide.contains(&3));
        assert!(!hide.contains(&5));
        assert!(!hide.contains(&10));
        let empty = BTreeSet::new();
        let (hl, hr) = hidden_equal_counts_with_revealed(
            &tags,
            &tags,
            &empty,
            &empty,
            COMPARE_HIDE_EQUAL_CONTEXT,
        );
        assert_eq!((hl, hr), (2, 2));
        assert_eq!(compare_visible_gap(3, 10), Some(6));
        assert_eq!(compare_visible_gap(3, 4), None);
        assert_eq!(compare_gap_line_range(3, 10), Some((4, 9)));
        let vis = [3usize, 4, 5, 12, 13];
        assert_eq!(nearest_compare_hide_gap(5, &vis), Some((6, 11)));
        assert_eq!(nearest_compare_hide_gap(12, &vis), Some((6, 11)));
        assert_eq!(nearest_compare_hide_gap(8, &vis), Some((6, 11)));
        assert_eq!(nearest_compare_hide_gap(3, &vis), None);
        assert_eq!(nearest_compare_hide_gap(13, &vis), None);
        let both = [0usize, 4, 8];
        assert_eq!(nearest_compare_hide_gap(4, &both), Some((5, 7)));
        assert_eq!(nearest_compare_hide_gap(0, &both), Some((1, 3)));
        let gaps = list_compare_hide_gaps(&both);
        assert_eq!(gaps, vec![(1, 3), (5, 7)]);
        assert_eq!(compare_hide_gap_park_line((1, 3)), 0);
        assert_eq!(compare_hide_gap_park_line((5, 7)), 4);
        assert_eq!(next_compare_hide_gap(0, &gaps), Some((5, 7)));
        assert_eq!(next_compare_hide_gap(4, &gaps), Some((1, 3))); // wrap
        assert_eq!(prev_compare_hide_gap(4, &gaps), Some((1, 3)));
        assert_eq!(prev_compare_hide_gap(0, &gaps), Some((5, 7))); // wrap
        assert_eq!(next_compare_hide_gap(2, &gaps), Some((5, 7))); // mid-gap
        assert_eq!(gaps.first().copied(), Some((1, 3)));
        assert_eq!(gaps.last().copied(), Some((5, 7)));
        assert_eq!(compare_hide_gap_ordinal(&gaps, (5, 7)), Some((2, 2)));
        assert_eq!(next_compare_hide_gap(0, &[]), None);
        let mut revealed = BTreeSet::new();
        assert_eq!(reveal_compare_gap(&mut revealed, (0, 0)), 1);
        let (hl2, hr2) = hidden_equal_counts_with_revealed(
            &tags,
            &tags,
            &revealed,
            &BTreeSet::new(),
            COMPARE_HIDE_EQUAL_CONTEXT,
        );
        assert_eq!((hl2, hr2), (1, 2));
        prune_compare_hide_revealed(&mut revealed, &tags, COMPARE_HIDE_EQUAL_CONTEXT);
        assert!(revealed.contains(&0));
        prune_compare_hide_revealed(&mut revealed, &[Equal, Equal], COMPARE_HIDE_EQUAL_CONTEXT);
        assert!(revealed.is_empty());
        let mut all = BTreeSet::new();
        assert_eq!(
            reveal_all_compare_hidden(&mut all, &tags, COMPARE_HIDE_EQUAL_CONTEXT),
            2
        );
        assert_eq!(
            reveal_all_compare_hidden(&mut all, &tags, COMPARE_HIDE_EQUAL_CONTEXT),
            0
        );
        let (hl3, hr3) =
            hidden_equal_counts_with_revealed(&tags, &tags, &all, &all, COMPARE_HIDE_EQUAL_CONTEXT);
        assert_eq!((hl3, hr3), (0, 0));
        assert_eq!(collapse_all_compare_revealed(&mut all), 2);
        assert!(all.is_empty());
        let (hl4, hr4) =
            hidden_equal_counts_with_revealed(&tags, &tags, &all, &all, COMPARE_HIDE_EQUAL_CONTEXT);
        assert_eq!((hl4, hr4), (2, 2));
        assert_eq!(collapse_all_compare_revealed(&mut all), 0);
        let mut rev = BTreeSet::from([0usize, 1, 5, 6, 7]);
        assert_eq!(nearest_compare_revealed_run(6, &rev), Some((5, 7)));
        assert_eq!(nearest_compare_revealed_run(3, &rev), Some((0, 1)));
        assert_eq!(nearest_compare_revealed_run(4, &rev), Some((5, 7)));
        assert_eq!(collapse_compare_gap(&mut rev, (5, 7)), 3);
        assert_eq!(rev.iter().copied().collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(collapse_compare_lines(&mut rev, &[0, 9]), 1);
        assert_eq!(rev.iter().copied().collect::<Vec<_>>(), vec![1]);
        assert_eq!(nearest_compare_revealed_run(0, &BTreeSet::new()), None);
    }

    #[test]
    fn aligned_equal_partner_lines_maps_hide_gap() {
        // Long matching prefix, one changed line, long matching suffix.
        let left: Vec<&str> = (0..12).map(|i| if i == 6 { "L" } else { "eq" }).collect();
        let right: Vec<&str> = (0..12).map(|i| if i == 6 { "R" } else { "eq" }).collect();
        let (lt, rt) = diff_line_tags(&left, &right);
        let hide_l = equal_line_indices_to_hide_with_context(&lt, COMPARE_HIDE_EQUAL_CONTEXT);
        let hide_r = equal_line_indices_to_hide_with_context(&rt, COMPARE_HIDE_EQUAL_CONTEXT);
        assert!(hide_l.contains(&0));
        assert!(hide_r.contains(&0));
        let partners = aligned_equal_partner_lines(&lt, &rt, (0, 0), true);
        assert_eq!(partners, vec![0]);
        let partners_r = aligned_equal_partner_lines(&lt, &rt, (0, 0), false);
        assert_eq!(partners_r, vec![0]);
        let mut left_rev = BTreeSet::new();
        let mut right_rev = BTreeSet::new();
        assert_eq!(reveal_compare_gap(&mut left_rev, (0, 0)), 1);
        let n_other = reveal_compare_lines(
            &mut right_rev,
            &aligned_equal_partner_lines(&lt, &rt, (0, 0), true),
        );
        assert_eq!(n_other, 1);
        assert!(right_rev.contains(&0));
        let (hl, hr) = hidden_equal_counts_with_revealed(
            &lt,
            &rt,
            &left_rev,
            &right_rev,
            COMPARE_HIDE_EQUAL_CONTEXT,
        );
        assert_eq!(hl, hide_l.len() - 1);
        assert_eq!(hr, hide_r.len() - 1);
    }

    #[test]
    fn aligned_equal_partner_line_maps_both_sides() {
        let left = ["a", "old", "c", "d"];
        let right = ["a", "new", "extra", "c", "d"];
        let (lt, rt) = diff_line_tags(&left, &right);
        assert_eq!(aligned_equal_partner_line(&lt, &rt, 0, true), Some(0));
        assert_eq!(aligned_equal_partner_line(&lt, &rt, 2, true), Some(3)); // "c"
        assert_eq!(aligned_equal_partner_line(&lt, &rt, 3, true), Some(4)); // "d"
        assert_eq!(aligned_equal_partner_line(&lt, &rt, 3, false), Some(2)); // right "c" → left
        assert_eq!(aligned_equal_partner_line(&lt, &rt, 1, true), None); // Delete
        assert_eq!(aligned_equal_partner_line(&lt, &rt, 1, false), None); // Insert
        assert_eq!(aligned_equal_partner_line(&lt, &rt, 99, true), None);
    }

    #[test]
    fn ignore_blank_skips_extra_blank_lines() {
        let left = ["a", "", "b"];
        let right = ["a", "b"];
        let (l0, r0) = diff_line_tags(&left, &right);
        assert!(l0.contains(&LineKind::Delete));
        assert_eq!(r0.iter().filter(|k| **k != LineKind::Equal).count(), 0);

        let (l, r) = diff_line_tags_ignore_blank(&left, &right, true);
        assert!(l.iter().all(|k| *k == LineKind::Equal));
        assert!(r.iter().all(|k| *k == LineKind::Equal));

        // Content diffs still surface; blanks stay Equal.
        let left2 = ["a", "  ", "x"];
        let right2 = ["a", "y"];
        let (l2, r2) = diff_line_tags_ignore_blank(&left2, &right2, true);
        assert_eq!(l2[0], LineKind::Equal);
        assert_eq!(l2[1], LineKind::Equal);
        assert_eq!(l2[2], LineKind::Delete);
        assert_eq!(r2[0], LineKind::Equal);
        assert_eq!(r2[1], LineKind::Insert);
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
    fn next_hunk_after_apply_parks_same_slot_or_none() {
        assert_eq!(next_hunk_after_apply(1, 3), Some(1));
        assert_eq!(next_hunk_after_apply(2, 3), Some(2));
        assert_eq!(next_hunk_after_apply(3, 2), Some(2)); // last applied → last remaining
        assert_eq!(next_hunk_after_apply(1, 0), None);
        assert_eq!(next_hunk_after_apply(0, 2), None);
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
    fn compare_summary_lists_hunks_and_identical() {
        let left = ["a", "gone", "b", "old", "c"];
        let right = ["a", "b", "new", "c", "tail"];
        let (l, r) = diff_line_tags(&left, &right);
        let hunks = compare_summary_hunks(&l, &r).unwrap();
        assert_eq!(hunks.len(), 3);
        assert_eq!(
            hunks[0],
            CompareSummaryHunk {
                ordinal: 1,
                left_start: 1,
                left_end: 2,
                right_start: 1,
                right_end: 1,
                deletes: 1,
                inserts: 0,
            }
        );
        assert_eq!(hunks[1].deletes, 1);
        assert_eq!(hunks[1].inserts, 1);
        assert_eq!(hunks[2].deletes, 0);
        assert_eq!(hunks[2].inserts, 1);
        let text = compare_summary_text("L", "R", &left, &right, &l, &r, "").unwrap();
        assert!(text.contains("Compare summary: “L” | “R”"));
        assert!(text.contains("3 hunks: 1 delete, 1 insert, 1 replace)\n"));
        assert!(text.contains("1. L2 | R2 (−1 +0) delete\n"));
        assert!(text.contains("   - gone\n"));
        assert!(text.contains("replace\n"));
        assert!(text.contains("   - old\n"));
        assert!(text.contains("   + new\n"));
        assert!(text.contains("insert\n"));
        assert!(text.contains("   + tail\n"));
        let (le, re) = diff_line_tags(&left, &left);
        let ident = compare_summary_text("a", "b", &left, &left, &le, &re, "").unwrap();
        assert!(ident.contains("(identical)\n"));
        assert!(ident.contains("0 hunks)\n"));
        assert!(!ident.contains("0 hunks:"));
        assert!(compare_summary_hunks(&l, &[]).is_none());
        assert!(compare_summary_text("L", "R", &left[..2], &right, &l, &r, "").is_none());
    }

    #[test]
    fn compare_summary_includes_ignore_note() {
        let left = ["A", "x"];
        let right = ["a", "y"];
        let (l, r) = diff_line_tags(&left, &right);
        let note = compare_ignore_note(true, true, false);
        assert_eq!(note, " · ignore ws+case");
        let text = compare_summary_text("L", "R", &left, &right, &l, &r, &note).unwrap();
        assert!(text.contains("hunks: 1 replace) · ignore ws+case\n"));
        assert_eq!(compare_ignore_note(false, false, false), "");
        assert_eq!(
            compare_ignore_note(true, true, true),
            " · ignore ws+case+blank"
        );
    }

    #[test]
    fn compare_summary_truncates_long_preview() {
        let long = "x".repeat(SUMMARY_PREVIEW_MAX + 8);
        let left = ["a", long.as_str()];
        let right = ["a"];
        let (l, r) = diff_line_tags(&left, &right);
        let text = compare_summary_text("L", "R", &left, &right, &l, &r, "").unwrap();
        assert!(text.contains('…'));
        let preview = text
            .lines()
            .find(|line| line.starts_with("   - "))
            .expect("delete preview");
        assert_eq!(preview.chars().count(), 5 + SUMMARY_PREVIEW_MAX);
        assert!(preview.ends_with('…'));
    }

    #[test]
    fn compare_summary_multi_line_previews_and_more() {
        let left = ["keep", "d1", "d2", "d3", "d4", "d5", "tail"];
        let right = ["keep", "i1", "i2", "i3", "i4", "tail"];
        let (l, r) = diff_line_tags(&left, &right);
        let text = compare_summary_text("L", "R", &left, &right, &l, &r, "").unwrap();
        assert!(text.contains("   - d1\n"));
        assert!(text.contains("   - d2\n"));
        assert!(text.contains("   - d3\n"));
        assert!(text.contains("   … (+2 more)\n"));
        assert!(!text.contains("   - d4\n"));
        assert!(text.contains("   + i1\n"));
        assert!(text.contains("   + i2\n"));
        assert!(text.contains("   + i3\n"));
        assert!(text.contains("   … (+1 more)\n"));
        assert!(!text.contains("   + i4\n"));
        assert_eq!(SUMMARY_PREVIEW_LINES, 3);
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

    #[test]
    fn unified_diff_hunk_picks_one_change() {
        let left = ["a", "gone1", "b", "gone2", "c"];
        let right = ["a", "b", "c"];
        let (l, r) = diff_line_tags(&left, &right);
        let first = unified_diff_hunk(&left, &right, "L", "R", &l, &r, true, 1).unwrap();
        assert!(first.contains("-gone1\n"));
        assert!(!first.contains("gone2"));
        let second = unified_diff_hunk(&left, &right, "L", "R", &l, &r, true, 3).unwrap();
        assert!(second.contains("-gone2\n"));
        assert!(!second.contains("gone1"));
        // Equal caret uses next hunk (wraps to first after last).
        let from_eq = unified_diff_hunk(&left, &right, "L", "R", &l, &r, true, 0).unwrap();
        assert!(from_eq.contains("-gone1\n"));
        assert!(!from_eq.contains("gone2"));
        assert_eq!(hunk_ordinal_for_copy(&l, 1), Some((1, 2)));
        assert_eq!(hunk_ordinal_for_copy(&l, 3), Some((2, 2)));
        assert_eq!(hunk_copy_change_counts(&l, &r, true, 1), Some((1, 0)));
        let (le, re) = diff_line_tags(&left, &left);
        assert!(unified_diff_hunk(&left, &left, "L", "R", &le, &re, true, 0).is_none());
    }

    #[test]
    fn unified_diff_hunk_replace_and_right_insert() {
        let left = ["keep", "old", "tail"];
        let right = ["keep", "new", "tail"];
        let (l, r) = diff_line_tags(&left, &right);
        let text = unified_diff_hunk(&left, &right, "L", "R", &l, &r, true, 1).unwrap();
        assert!(text.contains("-old\n"));
        assert!(text.contains("+new\n"));
        assert_eq!(hunk_copy_change_counts(&l, &r, true, 1), Some((1, 1)));
        let left2 = ["a", "c"];
        let right2 = ["a", "b", "c"];
        let (l2, r2) = diff_line_tags(&left2, &right2);
        let ins = unified_diff_hunk(&left2, &right2, "L", "R", &l2, &r2, false, 1).unwrap();
        assert!(ins.contains("+b\n"));
        assert!(!ins.contains("\n-"));
        assert_eq!(hunk_copy_change_counts(&l2, &r2, false, 1), Some((0, 1)));
    }

    fn splice_hunk(dest: &[&str], spec: HunkApply, src: &[&str]) -> Vec<String> {
        let mut out: Vec<String> = dest.iter().map(|s| (*s).to_string()).collect();
        let insert: Vec<String> = src[spec.src_start..spec.src_end]
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        out.splice(spec.dest_start..spec.dest_end, insert);
        out
    }

    #[test]
    fn hunk_apply_from_other_replace_insert_delete() {
        let left = ["keep", "old", "tail"];
        let right = ["keep", "new", "tail"];
        let (l, r) = diff_line_tags(&left, &right);
        let spec = hunk_apply_from_other(&l, &r, true, 1).unwrap();
        assert_eq!(spec.dest_start, 1);
        assert_eq!(spec.dest_end, 2);
        assert_eq!(spec.src_start, 1);
        assert_eq!(spec.src_end, 2);
        assert_eq!(spec.ordinal, 1);
        assert_eq!(splice_hunk(&left, spec, &right), ["keep", "new", "tail"]);

        let left2 = ["a", "c"];
        let right2 = ["a", "b", "c"];
        let (l2, r2) = diff_line_tags(&left2, &right2);
        let ins = hunk_apply_from_other(&l2, &r2, true, 0).unwrap();
        assert_eq!((ins.dest_start, ins.dest_end), (1, 1));
        assert_eq!((ins.src_start, ins.src_end), (1, 2));
        assert_eq!(splice_hunk(&left2, ins, &right2), ["a", "b", "c"]);
        let del_r = hunk_apply_from_other(&l2, &r2, false, 1).unwrap();
        assert_eq!((del_r.dest_start, del_r.dest_end), (1, 2));
        assert_eq!((del_r.src_start, del_r.src_end), (0, 0));
        assert_eq!(splice_hunk(&right2, del_r, &left2), ["a", "c"]);

        let left3 = ["a", "gone", "c"];
        let right3 = ["a", "c"];
        let (l3, r3) = diff_line_tags(&left3, &right3);
        let drop = hunk_apply_from_other(&l3, &r3, true, 1).unwrap();
        assert_eq!(splice_hunk(&left3, drop, &right3), ["a", "c"]);
        let add = hunk_apply_from_other(&l3, &r3, false, 0).unwrap();
        assert_eq!(splice_hunk(&right3, add, &left3), ["a", "gone", "c"]);

        let (le, re) = diff_line_tags(&left, &left);
        assert!(hunk_apply_from_other(&le, &re, true, 0).is_none());
    }

    #[test]
    fn hunk_apply_to_other_replace_insert_delete() {
        let left = ["keep", "old", "tail"];
        let right = ["keep", "new", "tail"];
        let (l, r) = diff_line_tags(&left, &right);
        let spec = hunk_apply_to_other(&l, &r, true, 1).unwrap();
        assert_eq!(spec.dest_start, 1);
        assert_eq!(spec.dest_end, 2);
        assert_eq!(spec.src_start, 1);
        assert_eq!(spec.src_end, 2);
        assert_eq!(spec.ordinal, 1);
        assert_eq!(splice_hunk(&right, spec, &left), ["keep", "old", "tail"]);

        let left2 = ["a", "c"];
        let right2 = ["a", "b", "c"];
        let (l2, r2) = diff_line_tags(&left2, &right2);
        // Push left (no insert) onto right → delete the inserted line.
        let drop = hunk_apply_to_other(&l2, &r2, true, 0).unwrap();
        assert_eq!((drop.dest_start, drop.dest_end), (1, 2));
        assert_eq!((drop.src_start, drop.src_end), (0, 0));
        assert_eq!(splice_hunk(&right2, drop, &left2), ["a", "c"]);
        // Push right's insert onto left.
        let add = hunk_apply_to_other(&l2, &r2, false, 1).unwrap();
        assert_eq!((add.dest_start, add.dest_end), (1, 1));
        assert_eq!((add.src_start, add.src_end), (1, 2));
        assert_eq!(splice_hunk(&left2, add, &right2), ["a", "b", "c"]);

        let (le, re) = diff_line_tags(&left, &left);
        assert!(hunk_apply_to_other(&le, &re, true, 0).is_none());
    }

    #[test]
    fn hunk_apply_all_from_other_two_hunks_last_first() {
        let left = ["keep", "old1", "mid", "old2", "tail"];
        let right = ["keep", "new1", "mid", "new2", "tail"];
        let (l, r) = diff_line_tags(&left, &right);
        let specs = hunk_apply_all_from_other(&l, &r, true).unwrap();
        assert_eq!(specs.len(), 2);
        assert_eq!(specs[0].ordinal, 1);
        assert_eq!(specs[1].ordinal, 2);
        assert_eq!(specs[0].total, 2);
        let mut dest: Vec<String> = left.iter().map(|s| (*s).to_string()).collect();
        for spec in specs.iter().rev() {
            let insert: Vec<String> = right[spec.src_start..spec.src_end]
                .iter()
                .map(|s| (*s).to_string())
                .collect();
            dest.splice(spec.dest_start..spec.dest_end, insert);
        }
        assert_eq!(dest, ["keep", "new1", "mid", "new2", "tail"]);
        let left2 = ["a", "c"];
        let right2 = ["a", "b", "c"];
        let (l2, r2) = diff_line_tags(&left2, &right2);
        let all = hunk_apply_all_from_other(&l2, &r2, true).unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(splice_hunk(&left2, all[0], &right2), ["a", "b", "c"]);
        let (le, re) = diff_line_tags(&left, &left);
        assert!(hunk_apply_all_from_other(&le, &re, true).is_none());
    }

    #[test]
    fn hunk_apply_all_to_other_two_hunks_last_first() {
        let left = ["keep", "old1", "mid", "old2", "tail"];
        let right = ["keep", "new1", "mid", "new2", "tail"];
        let (l, r) = diff_line_tags(&left, &right);
        // Push left onto right → right becomes left.
        let specs = hunk_apply_all_to_other(&l, &r, true).unwrap();
        assert_eq!(specs.len(), 2);
        assert_eq!(specs[0].ordinal, 1);
        assert_eq!(specs[1].ordinal, 2);
        let mut dest: Vec<String> = right.iter().map(|s| (*s).to_string()).collect();
        for spec in specs.iter().rev() {
            let insert: Vec<String> = left[spec.src_start..spec.src_end]
                .iter()
                .map(|s| (*s).to_string())
                .collect();
            dest.splice(spec.dest_start..spec.dest_end, insert);
        }
        assert_eq!(dest, ["keep", "old1", "mid", "old2", "tail"]);
        let left2 = ["a", "c"];
        let right2 = ["a", "b", "c"];
        let (l2, r2) = diff_line_tags(&left2, &right2);
        let all = hunk_apply_all_to_other(&l2, &r2, true).unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(splice_hunk(&right2, all[0], &left2), ["a", "c"]);
        let (le, re) = diff_line_tags(&left, &left);
        assert!(hunk_apply_all_to_other(&le, &re, true).is_none());
    }
}
