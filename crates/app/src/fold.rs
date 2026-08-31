//! Lexer-aware fold regions and margin helpers.
//!
//! Brace languages use `{`/`}` nesting (strings / comments skipped lightly).
//! Indent languages use deeper-indent blocks (same idea as View → Fold Current).

use buffer::TextBuffer;
use std::collections::BTreeSet;

/// One foldable span: header stays visible; `(header+1)..=end` hide when folded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoldRegion {
    pub header: usize,
    pub end: usize,
    /// Nesting depth (1 = outermost).
    pub level: usize,
}

/// Languages that use brace nesting for folds.
pub fn uses_brace_folds(language: &str) -> bool {
    matches!(
        language,
        "rust" | "c" | "cpp" | "json" | "sql" | "javascript" | "typescript" | "java" | "go"
    )
}

/// Compute fold regions for the active language.
pub fn compute_fold_regions(language: &str, buf: &TextBuffer) -> Vec<FoldRegion> {
    if uses_brace_folds(language) {
        brace_fold_regions(buf)
    } else {
        indent_fold_regions(buf)
    }
}

/// Region whose header is `line`, if any (prefer innermost when several share a header).
pub fn region_at_header(regions: &[FoldRegion], line: usize) -> Option<FoldRegion> {
    regions
        .iter()
        .copied()
        .filter(|r| r.header == line && r.end > r.header)
        .max_by_key(|r| r.level)
}

/// True when the first child line of the region is hidden.
pub fn is_folded(hidden: &BTreeSet<usize>, region: &FoldRegion) -> bool {
    if region.end <= region.header {
        return false;
    }
    hidden.contains(&(region.header + 1))
}

/// Fold or unfold one region (toggle).
pub fn toggle_region(hidden: &mut BTreeSet<usize>, region: &FoldRegion) {
    if region.end <= region.header {
        return;
    }
    if is_folded(hidden, region) {
        for i in (region.header + 1)..=region.end {
            hidden.remove(&i);
        }
    } else {
        for i in (region.header + 1)..=region.end {
            hidden.insert(i);
        }
    }
}

/// Hide every fold region's body.
pub fn fold_all_regions(hidden: &mut BTreeSet<usize>, regions: &[FoldRegion]) -> usize {
    let mut added = 0usize;
    for r in regions {
        if r.end <= r.header {
            continue;
        }
        for i in (r.header + 1)..=r.end {
            if hidden.insert(i) {
                added += 1;
            }
        }
    }
    added
}

/// Hide bodies of regions with `level >= min_level`.
pub fn fold_regions_at_level(
    hidden: &mut BTreeSet<usize>,
    regions: &[FoldRegion],
    min_level: usize,
) -> usize {
    let mut added = 0usize;
    for r in regions {
        if r.level < min_level || r.end <= r.header {
            continue;
        }
        for i in (r.header + 1)..=r.end {
            if hidden.insert(i) {
                added += 1;
            }
        }
    }
    added
}

/// Show bodies of regions with exact `level`.
pub fn unfold_regions_at_level(
    hidden: &mut BTreeSet<usize>,
    regions: &[FoldRegion],
    level: usize,
) -> usize {
    let mut shown = 0usize;
    for r in regions {
        if r.level != level || r.end <= r.header {
            continue;
        }
        for i in (r.header + 1)..=r.end {
            if hidden.remove(&i) {
                shown += 1;
            }
        }
    }
    shown
}

/// Fold region covering the caret line (innermost), if any.
pub fn region_for_line(regions: &[FoldRegion], line: usize) -> Option<FoldRegion> {
    regions
        .iter()
        .copied()
        .filter(|r| r.header <= line && line <= r.end && r.end > r.header)
        .max_by_key(|r| r.level)
}

fn brace_fold_regions(buf: &TextBuffer) -> Vec<FoldRegion> {
    let mut regions = Vec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new(); // (header_line, level)
    let mut depth = 0usize;
    let mut in_str: Option<char> = None;
    let mut in_block_comment = false;

    for line_idx in 0..buf.line_count() {
        let raw = buf.line(line_idx);
        let chars: Vec<char> = raw.chars().collect();
        let mut i = 0usize;
        while i < chars.len() {
            let c = chars[i];
            let next = chars.get(i + 1).copied();

            if in_block_comment {
                if c == '*' && next == Some('/') {
                    in_block_comment = false;
                    i += 2;
                    continue;
                }
                i += 1;
                continue;
            }
            if let Some(q) = in_str {
                if c == '\\' && next.is_some() {
                    i += 2;
                    continue;
                }
                if c == q {
                    in_str = None;
                }
                i += 1;
                continue;
            }

            match c {
                '/' if next == Some('/') => {
                    break;
                }
                '/' if next == Some('*') => {
                    in_block_comment = true;
                    i += 2;
                    continue;
                }
                '"' | '\'' => {
                    in_str = Some(c);
                    i += 1;
                }
                '{' => {
                    depth += 1;
                    stack.push((line_idx, depth));
                    i += 1;
                }
                '}' => {
                    if let Some((header, level)) = stack.pop() {
                        if line_idx > header {
                            regions.push(FoldRegion {
                                header,
                                end: line_idx,
                                level,
                            });
                        }
                        depth = depth.saturating_sub(1);
                    }
                    i += 1;
                }
                _ => i += 1,
            }
        }
    }

    // Prefer listing by header then level for stable paint.
    regions.sort_by_key(|r| (r.header, r.level));
    regions
}

fn indent_fold_regions(buf: &TextBuffer) -> Vec<FoldRegion> {
    let n = buf.line_count();
    let mut levels: Vec<Option<usize>> = Vec::with_capacity(n);
    for i in 0..n {
        let raw = buf.line(i);
        if raw.trim().is_empty() {
            levels.push(None);
        } else {
            levels.push(Some(line_indent_units(&raw)));
        }
    }

    let mut regions = Vec::new();
    for i in 0..n {
        let Some(base) = levels[i] else {
            continue;
        };
        let mut end = i;
        let mut saw_child = false;
        for (j, level) in levels.iter().enumerate().skip(i + 1) {
            match level {
                None => {
                    end = j;
                }
                Some(lv) if *lv > base => {
                    end = j;
                    saw_child = true;
                }
                _ => break,
            }
        }
        if saw_child && end > i {
            regions.push(FoldRegion {
                header: i,
                end,
                level: base + 1,
            });
        }
    }
    regions
}

fn line_indent_units(line: &str) -> usize {
    let mut spaces = 0usize;
    for c in line.chars() {
        match c {
            ' ' => spaces += 1,
            '\t' => spaces += 4,
            _ => break,
        }
    }
    spaces / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buf(text: &str) -> TextBuffer {
        TextBuffer::from_str(text)
    }

    #[test]
    fn brace_folds_rust_fn() {
        let b = buf("fn main() {\n    let x = 1;\n}\n");
        let regions = compute_fold_regions("rust", &b);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].header, 0);
        assert_eq!(regions[0].end, 2);
        assert_eq!(regions[0].level, 1);
    }

    #[test]
    fn brace_folds_nested() {
        let b = buf("fn a() {\n    if true {\n        x();\n    }\n}\n");
        let regions = compute_fold_regions("rust", &b);
        assert_eq!(regions.len(), 2);
        let inner = region_at_header(&regions, 1).unwrap();
        assert_eq!(inner.end, 3);
        assert_eq!(inner.level, 2);
        let outer = regions.iter().find(|r| r.header == 0).unwrap();
        assert_eq!(outer.end, 4);
        assert_eq!(outer.level, 1);
    }

    #[test]
    fn brace_skips_string_braces() {
        let b = buf("fn a() {\n    let s = \"{\";\n}\n");
        let regions = compute_fold_regions("rust", &b);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].end, 2);
    }

    #[test]
    fn indent_folds_python() {
        let b = buf("def f():\n    x = 1\n    y = 2\nz = 3\n");
        let regions = compute_fold_regions("python", &b);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].header, 0);
        assert_eq!(regions[0].end, 2);
    }

    #[test]
    fn toggle_fold_hides_and_shows() {
        let region = FoldRegion {
            header: 0,
            end: 2,
            level: 1,
        };
        let mut hidden = BTreeSet::new();
        toggle_region(&mut hidden, &region);
        assert!(is_folded(&hidden, &region));
        assert!(hidden.contains(&1));
        assert!(hidden.contains(&2));
        toggle_region(&mut hidden, &region);
        assert!(!is_folded(&hidden, &region));
        assert!(hidden.is_empty());
    }
}
