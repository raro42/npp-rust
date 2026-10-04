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

/// Char-index ranges (exclusive end) that differ between two lines.
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

/// Intra-line spans for paired delete/insert lines in each change hunk.
///
/// Unpaired insert/delete lines stay empty (full-line wash only).
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
            if let Some((ls, rs)) = char_change_spans(l_txt, r_txt) {
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
    hunk_apply_from_run(&ops, run_start, run_end, focus_left, ordinal, total)
}

/// Every change hunk as an apply spec (file order, 1-based ordinals).
///
/// `None` when tags cannot be aligned or there are no change hunks.
pub fn hunk_apply_all_from_other(
    left_tags: &[LineKind],
    right_tags: &[LineKind],
    focus_left: bool,
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
            focus_left,
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
    fn char_change_spans_skips_long_lines() {
        let long: String = "x".repeat(MAX_INLINE_CHARS + 1);
        assert!(char_change_spans(&long, "y").is_none());
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
}
