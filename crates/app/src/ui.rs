//! egui shell: menus, tabs, viewport editor, find bar.

use crate::editor::EditorState;
use crate::ui_paint::{
    change_history_joins, change_history_wash, col_from_x, display_row_for,
    paint_change_history_bar, paint_compare_hide_gap, paint_fold_marker,
    paint_inline_compare_spans, paint_line_text, style_mark_bg, text_width, visible_line_indices,
    FOLD_MARGIN_W,
};
use eframe::egui::{self, Color32, CursorIcon, FontId, Key, Pos2, Rect, RichText, Sense, Vec2};
use std::collections::BTreeSet;
use std::path::PathBuf;

/// Soft teal — ready menu items (works on light and dark themes).
const MENU_READY: Color32 = Color32::from_rgb(42, 148, 118);

/// Argv options passed from `main` into the UI shell.
#[derive(Debug, Clone, Default)]
pub struct CliOptions {
    pub paths: Vec<std::path::PathBuf>,
    /// Jump to this 1-based line in the first opened file.
    pub goto_line: Option<usize>,
    /// Mark argv-opened files read-only.
    pub read_only: bool,
}

/// Which dual-view pane receives keyboard edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditorPane {
    Primary,
    Secondary,
}

/// In-progress drag of selected text (move, or copy with Ctrl/Cmd).
struct SelTextDrag {
    tab: usize,
    drop_at: usize,
}

/// Choose the right-hand tab for Compare.
///
/// Order: marked partner → dual-view other pane → tab to the right → tab to the left.
fn pick_compare_right(
    n: usize,
    active: usize,
    partner: Option<usize>,
    dual_view: bool,
    other_view_tab: usize,
) -> Option<usize> {
    if n < 2 {
        return None;
    }
    if let Some(p) = partner {
        if p < n && p != active {
            return Some(p);
        }
    }
    if dual_view && other_view_tab < n && other_view_tab != active {
        return Some(other_view_tab);
    }
    if active + 1 < n {
        return Some(active + 1);
    }
    if active > 0 {
        return Some(active - 1);
    }
    None
}

/// Compare ignore toggles (ws / case / blank) for status wording.
#[derive(Clone, Copy)]
struct CompareIgnoreBits {
    ws: bool,
    case: bool,
    blank: bool,
}

/// Unified-diff payload for the caret hunk (copy / open tab).
struct CompareHunkPayload {
    text: String,
    left_name: String,
    right_name: String,
    ordinal: usize,
    total: usize,
    kind: &'static str,
    deletes: usize,
    inserts: usize,
}

/// Status line for an active compare pair (identical vs change counts).
fn compare_ignore_status_bit(bits: CompareIgnoreBits) -> String {
    crate::diff::compare_ignore_note(bits.ws, bits.case, bits.blank)
}

fn compare_hide_opt(
    on: bool,
    left_tags: &[crate::diff::LineKind],
    right_tags: &[crate::diff::LineKind],
    left_revealed: &BTreeSet<usize>,
    right_revealed: &BTreeSet<usize>,
    context: usize,
) -> Option<(usize, usize)> {
    if on {
        Some(crate::diff::hidden_equal_counts_with_revealed(
            left_tags,
            right_tags,
            left_revealed,
            right_revealed,
            context,
        ))
    } else {
        None
    }
}

/// Hit-test the ···N hairline between visible rows; returns inclusive doc-line gap.
fn compare_hide_gap_at_pointer(
    pos: Pos2,
    rect: Rect,
    scroll_line: f32,
    row_height: f32,
    visible_lines: &[usize],
) -> Option<(usize, usize)> {
    if row_height <= 0.0 || visible_lines.len() < 2 {
        return None;
    }
    let rel_y = pos.y - rect.top();
    if rel_y < 0.0 || pos.x < rect.left() || pos.x > rect.right() {
        return None;
    }
    let row_f = scroll_line + rel_y / row_height;
    let row = row_f.floor() as usize;
    let frac = row_f - row as f32;
    // Cue sits on the top edge of the lower row (also accept bottom of upper row).
    let (prev_row, cur_row) = if frac <= 0.35 {
        (row.checked_sub(1)?, row)
    } else if frac >= 0.85 {
        (row, row + 1)
    } else {
        return None;
    };
    let prev = *visible_lines.get(prev_row)?;
    let cur = *visible_lines.get(cur_row)?;
    crate::diff::compare_gap_line_range(prev, cur)
}

/// Hide-equal status for compare status-line bits.
#[derive(Clone, Copy)]
struct CompareHideStatus {
    hidden: Option<(usize, usize)>,
    context: usize,
}

/// Change counts + Equal match percent for the live Compare status line.
struct ComparePairCounts {
    del: usize,
    ins: usize,
    hunk_n: usize,
    equal_pct: u8,
    /// After hunk count (`""` or `": 1 delete, 2 insert"`).
    kind_bit: String,
}

/// `None` = hide-equal off; `Some((left, right))` = on with per-side hidden counts.
fn compare_hide_status_bit(hide: CompareHideStatus) -> String {
    let Some((hidden_left, hidden_right)) = hide.hidden else {
        return String::new();
    };
    let ctx = hide.context;
    let count_bit = if hidden_left == 0 && hidden_right == 0 {
        String::new()
    } else if hidden_left == hidden_right {
        format!(" · {hidden_left} hidden")
    } else {
        format!(" · L{hidden_left}|R{hidden_right} hidden")
    };
    format!(" · hide equal ±{ctx}{count_bit}")
}

fn compare_pair_status(
    lname: &str,
    rname: &str,
    counts: ComparePairCounts,
    ignore: CompareIgnoreBits,
    hide: CompareHideStatus,
) -> String {
    let ignore_bit = compare_ignore_status_bit(ignore);
    let hide_bit = compare_hide_status_bit(hide);
    let ComparePairCounts {
        del,
        ins,
        hunk_n,
        equal_pct,
        kind_bit,
    } = counts;
    if del == 0 && ins == 0 {
        return format!("Compare “{lname}” | “{rname}” (identical){ignore_bit}{hide_bit}");
    }
    let hunk_bit = if hunk_n > 0 {
        format!(
            " · {hunk_n} hunk{}{kind_bit}",
            if hunk_n == 1 { "" } else { "s" }
        )
    } else {
        String::new()
    };
    format!(
        "Compare “{lname}” | “{rname}” (−{del} +{ins}){hunk_bit} · {equal_pct}% equal{ignore_bit}{hide_bit}"
    )
}

/// Suffix when Compare parks on a hunk (start / ignore re-diff / hide-equal / swap / First / edit re-diff), e.g. ` · at L12 | R15 (1/5 replace −1 +1)`.
fn compare_at_hunk_status_bit(lr: &str, ordinal_bit: &str) -> String {
    if lr.is_empty() {
        format!(" · at {ordinal_bit}")
    } else {
        format!(" · at {lr} {ordinal_bit}")
    }
}

/// Live Compare status counts from current tags.
fn compare_pair_counts_from_tags(
    left_tags: &[crate::diff::LineKind],
    right_tags: &[crate::diff::LineKind],
) -> ComparePairCounts {
    let (del, ins) = crate::diff::count_changes(left_tags, right_tags);
    let hunk_n = crate::diff::hunk_starts(left_tags)
        .len()
        .max(crate::diff::hunk_starts(right_tags).len());
    let equal_pct = crate::diff::compare_equal_percent(left_tags, right_tags);
    let kind_bit = crate::diff::compare_kind_tally_bit(left_tags, right_tags);
    ComparePairCounts {
        del,
        ins,
        hunk_n,
        equal_pct,
        kind_bit,
    }
}

/// Visible rows for a pane, merging fold hides with optional Compare hide-equal.
fn visible_lines_with_compare_hide(
    line_count: usize,
    fold_hidden: &BTreeSet<usize>,
    compare_on: bool,
    hide_equal: bool,
    tags: &[crate::diff::LineKind],
    revealed: &BTreeSet<usize>,
    context: usize,
) -> Vec<usize> {
    if !compare_on || !hide_equal {
        return visible_line_indices(line_count, fold_hidden);
    }
    let mut equal_hide = crate::diff::equal_line_indices_to_hide_with_context(tags, context);
    for i in revealed {
        equal_hide.remove(i);
    }
    if equal_hide.is_empty() {
        return visible_line_indices(line_count, fold_hidden);
    }
    if fold_hidden.is_empty() {
        return visible_line_indices(line_count, &equal_hide);
    }
    let mut merged = fold_hidden.clone();
    merged.extend(equal_hide);
    visible_line_indices(line_count, &merged)
}

/// Status for Equal-line partner park (click/keyboard).
///
/// Example: `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal · ignore ws · hide equal ±3 · 5 hidden`.
fn compare_equal_park_status(
    l: usize,
    r: usize,
    counts: &ComparePairCounts,
    ignore: CompareIgnoreBits,
    hide: CompareHideStatus,
) -> String {
    let ignore_bit = compare_ignore_status_bit(ignore);
    let hide_bit = compare_hide_status_bit(hide);
    let ComparePairCounts {
        del,
        ins,
        hunk_n,
        equal_pct,
        kind_bit,
    } = counts;
    if *del == 0 && *ins == 0 {
        format!("Compare equal → L{l} | R{r} (identical){ignore_bit}{hide_bit}")
    } else {
        let hunk_word = if *hunk_n == 1 { "hunk" } else { "hunks" };
        format!(
            "Compare equal → L{l} | R{r} (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}) · {equal_pct}% equal{ignore_bit}{hide_bit}"
        )
    }
}

/// Status for Copy/Open Compare Diff, e.g. `Copied unified diff (−1 +2, 2 hunks: 1 delete, 1 insert) “a” | “b”`.
fn compare_open_or_copy_status(
    verb: &str,
    lname: &str,
    rname: &str,
    del: usize,
    ins: usize,
    hunk_n: usize,
    kind_bit: &str,
) -> String {
    if del == 0 && ins == 0 {
        format!("{verb} unified diff (identical) “{lname}” | “{rname}”")
    } else {
        let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
        format!(
            "{verb} unified diff (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}) “{lname}” | “{rname}”"
        )
    }
}

/// Status for Clear Compare, e.g. `Compare cleared (−1 +2, 2 hunks: 1 delete, 1 insert) “a” | “b”`.
fn compare_cleared_status(
    closed_saved_snapshot: bool,
    lname: &str,
    rname: &str,
    del: usize,
    ins: usize,
    hunk_n: usize,
    kind_bit: &str,
) -> String {
    let prefix = if closed_saved_snapshot {
        "Compare cleared (closed saved snapshot)"
    } else {
        "Compare cleared"
    };
    if del == 0 && ins == 0 {
        format!("{prefix} (identical) “{lname}” | “{rname}”")
    } else {
        let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
        format!("{prefix} (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}) “{lname}” | “{rname}”")
    }
}

/// Status for Copy/Open Compare Hunk, e.g. `Copied hunk (2/5 replace −1 +1) unified diff “a” | “b”`.
fn compare_hunk_copy_open_status(
    verb: &str,
    ordinal_bit: &str,
    lname: &str,
    rname: &str,
) -> String {
    format!("{verb} hunk {ordinal_bit} unified diff “{lname}” | “{rname}”")
}

/// Status for Copy/Open Compare Summary, e.g. `Copied compare summary (−1 +2, 2 hunks: 1 delete, 1 insert) “a” | “b”`.
fn compare_summary_copy_open_status(
    verb: &str,
    del: usize,
    ins: usize,
    hunk_n: usize,
    kind_bit: &str,
    lname: &str,
    rname: &str,
) -> String {
    let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
    format!(
        "{verb} compare summary (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}) “{lname}” | “{rname}”"
    )
}

/// Status for Bookmark Compare Differences.
///
/// Example: `Compare bookmarked differences (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, +3 new)`.
fn compare_bookmark_differences_status(
    del: usize,
    ins: usize,
    left_marks: usize,
    right_marks: usize,
    hunk_n: usize,
    kind_bit: &str,
    added: usize,
) -> String {
    let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
    format!(
        "Compare bookmarked differences (−{del} +{ins}, L{left_marks}|R{right_marks}, {hunk_n} {hunk_word}{kind_bit}, +{added} new)"
    )
}

/// Status for Clear Compare Difference Bookmarks.
///
/// Example: `Compare cleared difference bookmarks (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, −3 removed)`.
fn compare_clear_difference_bookmarks_status(
    del: usize,
    ins: usize,
    left_marks: usize,
    right_marks: usize,
    hunk_n: usize,
    kind_bit: &str,
    removed: usize,
) -> String {
    let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
    format!(
        "Compare cleared difference bookmarks (−{del} +{ins}, L{left_marks}|R{right_marks}, {hunk_n} {hunk_word}{kind_bit}, −{removed} removed)"
    )
}

/// Status for Expand All Unchanged Lines.
///
/// Example: `Compare expanded all unchanged lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 shown)`.
fn compare_expand_all_unchanged_status(
    del: usize,
    ins: usize,
    hunk_n: usize,
    kind_bit: &str,
    shown: usize,
) -> String {
    let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
    format!(
        "Compare expanded all unchanged lines (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}, {shown} shown)"
    )
}

/// Status for Collapse All Unchanged Lines.
///
/// Example: `Compare collapsed expanded equal lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 re-hidden, 8 hidden)`.
fn compare_collapse_all_unchanged_status(
    del: usize,
    ins: usize,
    hunk_n: usize,
    kind_bit: &str,
    rehidden: usize,
    remain: usize,
) -> String {
    let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
    format!(
        "Compare collapsed expanded equal lines (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}, {rehidden} re-hidden, {remain} hidden)"
    )
}

/// Status for Expand Unchanged at Caret / click ···N cue.
///
/// Example: `Compare expanded ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 still hidden)`.
fn compare_expand_gap_status(
    n: usize,
    both: &str,
    del: usize,
    ins: usize,
    hunk_n: usize,
    kind_bit: &str,
    remain: usize,
) -> String {
    let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
    let tail = if remain == 0 {
        "all equal lines shown".to_string()
    } else {
        format!("{remain} still hidden")
    };
    format!("Compare expanded ···{n}{both} (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}, {tail})")
}

/// Status for Collapse Unchanged at Caret.
///
/// Example: `Compare collapsed ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 hidden)`.
fn compare_collapse_gap_status(
    n: usize,
    both: &str,
    del: usize,
    ins: usize,
    hunk_n: usize,
    kind_bit: &str,
    remain: usize,
) -> String {
    let hunk_word = if hunk_n == 1 { "hunk" } else { "hunks" };
    format!(
        "Compare collapsed ···{n}{both} (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}, {remain} hidden)"
    )
}

/// Status for Next/Previous/First/Last Hidden Equal.
///
/// Example: `Compare Next hidden equal → ···5 (1/3) (−1 +2, 2 hunks: 1 delete, 1 insert) · wrapped`.
fn compare_hide_gap_nav_status(
    dir: &str,
    n: usize,
    ord: usize,
    total: usize,
    counts: &ComparePairCounts,
    wrapped: bool,
) -> String {
    let ComparePairCounts {
        del,
        ins,
        hunk_n,
        kind_bit,
        ..
    } = counts;
    let hunk_word = if *hunk_n == 1 { "hunk" } else { "hunks" };
    let wrap_bit = if wrapped { " · wrapped" } else { "" };
    format!(
        "Compare {dir} hidden equal → ···{n} ({ord}/{total}) (−{del} +{ins}, {hunk_n} {hunk_word}{kind_bit}){wrap_bit}"
    )
}

fn compare_buffer_eol(buf: &buffer::TextBuffer) -> &'static str {
    let n = buf.line_count();
    for i in 0..n {
        let line = buf.line(i);
        if line.ends_with("\r\n") {
            return "\r\n";
        }
        if line.ends_with('\n') {
            return "\n";
        }
        if line.ends_with('\r') {
            return "\r";
        }
    }
    "\n"
}

fn compare_line_range_chars(buf: &buffer::TextBuffer, start: usize, end: usize) -> (usize, usize) {
    let n = buf.line_count();
    let lo = if start >= n {
        buf.len_chars()
    } else {
        buf.line_to_char(start)
    };
    let hi = if end >= n {
        buf.len_chars()
    } else {
        buf.line_to_char(end)
    };
    (lo, hi.max(lo))
}

fn join_compare_lines(lines: &[String], eol: &str, trailing_eol: bool) -> String {
    if lines.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        out.push_str(line);
        if i + 1 < lines.len() || trailing_eol {
            out.push_str(eol);
        }
    }
    out
}

fn apply_compare_hunk_spec(
    buf: &mut buffer::TextBuffer,
    spec: &crate::diff::HunkApply,
    src_slice: &[String],
) -> (usize, usize) {
    let n = buf.line_count();
    let (lo, hi) = compare_line_range_chars(buf, spec.dest_start, spec.dest_end);
    let deleted = buf.slice(lo, hi);
    let trailing_eol = if spec.dest_start == spec.dest_end {
        spec.dest_start < n
    } else {
        deleted.ends_with('\n') || deleted.ends_with('\r')
    };
    let eol = compare_buffer_eol(buf);
    let replacement = join_compare_lines(src_slice, eol, trailing_eol);
    buf.set_selection(lo, hi);
    buf.insert(&replacement);
    let new_end = lo + replacement.chars().count();
    (lo, new_end)
}

/// Remap a tab index after `closed` was removed. `None` if that tab was closed.
fn index_after_tab_close(idx: usize, closed: usize) -> Option<usize> {
    if idx == closed {
        None
    } else if idx > closed {
        Some(idx - 1)
    } else {
        Some(idx)
    }
}

pub struct EditorApp {
    state: EditorState,
    find_focus_once: bool,
    /// Vertical scroll in lines.
    scroll_line: f32,
    /// When true, next paint scrolls so the caret stays in view.
    follow_caret: bool,
    /// Caret-follow for the secondary pane.
    follow_caret_other: bool,
    show_about: bool,
    show_preferences: bool,
    /// Checkbox state for the log-tail prompt.
    log_tail_remember: bool,
    /// Drag-select anchor (char index), while primary button is held.
    drag_anchor: Option<usize>,
    /// Alt+drag rectangular / column selection in progress.
    rect_drag: bool,
    /// Drag selected text to move (or Ctrl/Cmd+drag to copy).
    sel_text_drag: Option<SelTextDrag>,
    /// Tab bar drag-reorder: source index while the pointer drags a tab.
    tab_drag_from: Option<usize>,
    show_replace: bool,
    replace_with: String,
    /// Friendly dialog for menu items not wired yet.
    coming_soon: Option<crate::commands::ComingSoon>,
    /// Editor monospace size (zoom).
    font_size: f32,
    show_goto_line: bool,
    goto_line_input: String,
    show_summary: bool,
    show_doc_list: bool,
    show_project_panel: bool,
    /// Cached Project panel listing (`root` + filter → entries).
    project_panel_cache_key: Option<(PathBuf, String)>,
    project_panel_entries: Vec<PathBuf>,
    show_theme_picker: bool,
    show_doc_map: bool,
    show_func_list: bool,
    show_char_panel: bool,
    /// Last text copied via menu (`pending_clipboard`).
    last_app_clipboard: Option<String>,
    /// Next Paste replaces bookmarked lines.
    await_paste_bookmarks: bool,
    /// Second editor pane (writable).
    dual_view: bool,
    /// Tab index shown in the secondary pane.
    other_view_tab: usize,
    /// Pane that owns keyboard typing / caret keys.
    focused_pane: EditorPane,
    /// Vertical scroll for the secondary pane (lines).
    scroll_line_other: f32,
    /// Sync vertical scroll between panes.
    sync_scroll_v: bool,
    /// Sync flag for horizontal (MVP shares line scroll with V when either is on).
    sync_scroll_h: bool,
    /// Session flag: both panes share font size (always true in practice).
    zoom_sync: bool,
    /// 2-way compare mode (colours + dual view).
    compare_on: bool,
    compare_left_tab: usize,
    compare_right_tab: usize,
    compare_left_tags: Vec<crate::diff::LineKind>,
    compare_right_tags: Vec<crate::diff::LineKind>,
    /// Intra-line char ranges (exclusive end) for replace hunks.
    compare_left_inline: Vec<Vec<crate::diff::CharRange>>,
    compare_right_inline: Vec<Vec<crate::diff::CharRange>>,
    /// When set, wait until this instant before re-diff (debounce while typing).
    compare_refresh_at: Option<std::time::Instant>,
    /// Optional second tab for Compare (⌘/Ctrl-click a tab, or context menu).
    compare_partner_tab: Option<usize>,
    /// Equal lines temporarily shown after clicking a ···N hide-equal gap (left).
    compare_hide_revealed_left: BTreeSet<usize>,
    /// Equal lines temporarily shown after clicking a ···N hide-equal gap (right).
    compare_hide_revealed_right: BTreeSet<usize>,
}

impl EditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, cli: CliOptions) -> Self {
        let state = EditorState::new();
        let font_size = state.settings.font_size.clamp(8.0, 48.0);
        let mut app = Self {
            state,
            find_focus_once: false,
            scroll_line: 0.0,
            follow_caret: false,
            follow_caret_other: false,
            show_about: false,
            show_preferences: false,
            log_tail_remember: true,
            drag_anchor: None,
            rect_drag: false,
            sel_text_drag: None,
            tab_drag_from: None,
            show_replace: false,
            replace_with: String::new(),
            coming_soon: None,
            font_size,
            show_goto_line: false,
            goto_line_input: String::new(),
            show_summary: false,
            show_doc_list: false,
            show_project_panel: false,
            project_panel_cache_key: None,
            project_panel_entries: Vec::new(),
            show_theme_picker: false,
            show_doc_map: false,
            show_func_list: false,
            show_char_panel: false,
            last_app_clipboard: None,
            await_paste_bookmarks: false,
            dual_view: false,
            other_view_tab: 0,
            focused_pane: EditorPane::Primary,
            scroll_line_other: 0.0,
            sync_scroll_v: false,
            sync_scroll_h: false,
            zoom_sync: false,
            compare_on: false,
            compare_left_tab: 0,
            compare_right_tab: 0,
            compare_left_tags: Vec::new(),
            compare_right_tags: Vec::new(),
            compare_left_inline: Vec::new(),
            compare_right_inline: Vec::new(),
            compare_refresh_at: None,
            compare_partner_tab: None,
            compare_hide_revealed_left: BTreeSet::new(),
            compare_hide_revealed_right: BTreeSet::new(),
        };
        let had_argv = !cli.paths.is_empty();
        app.replace_with = app.state.settings.replace_with.clone();
        app.open_argv_paths(cli);
        if !had_argv && app.state.settings.restore_session {
            app.state.restore_session_from_disk();
        }
        app
    }

    /// Open existing argv paths; skip missing and report on the status line.
    fn open_argv_paths(&mut self, cli: CliOptions) {
        if cli.paths.is_empty() {
            return;
        }
        let mut opened = 0usize;
        let mut missing: Vec<String> = Vec::new();
        let mut first_opened_tab: Option<usize> = None;
        for path in cli.paths {
            if path.exists() {
                self.state.open_path(path);
                let tab = self.state.tabs.active_index();
                if cli.read_only {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.read_only = true;
                    }
                }
                if first_opened_tab.is_none() {
                    first_opened_tab = Some(tab);
                }
                opened += 1;
            } else {
                missing.push(path.display().to_string());
            }
        }
        if opened > 0 && self.state.tabs.len() > 1 {
            if let Some(doc) = self.state.tabs.get(0) {
                if doc.path.is_none() && !doc.dirty && doc.buffer.is_empty() {
                    self.state.close_tab(0);
                    if let Some(t) = first_opened_tab.as_mut() {
                        *t = t.saturating_sub(1);
                    }
                }
            }
        }
        if let (Some(tab), Some(line_1based)) = (first_opened_tab, cli.goto_line) {
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                let line = line_1based.saturating_sub(1);
                let max_line = doc.buffer.line_count().saturating_sub(1);
                let line = line.min(max_line);
                let pos = doc.buffer.line_to_char(line);
                doc.buffer.set_caret(pos);
                self.state.tabs.set_active(tab);
                self.follow_caret = true;
                self.scroll_line = line as f32;
            }
        }
        if !missing.is_empty() {
            let skip = missing.join(", ");
            self.state.status = if opened > 0 {
                format!("Opened {opened} file(s); skipped missing: {skip}")
            } else {
                format!("Skipped missing: {skip}")
            };
        } else if opened > 1 {
            self.state.status = format!("Opened {opened} file(s) from command line");
        }
        if cli.read_only && opened > 0 {
            self.state.status = format!("{} (read-only)", self.state.status);
        }
        if let Some(n) = cli.goto_line {
            if opened > 0 {
                self.state.status = format!("{} → line {n}", self.state.status);
            }
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            self.state.persist_session_if_enabled();
            let dirty = self.state.tabs.iter().any(|d| d.dirty);
            if dirty || self.state.pending_close.is_some() {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                if self.state.pending_close.is_none() {
                    let mut flags = crate::commands::UiFlags::default();
                    self.state.request_quit(&mut flags);
                    if flags.request_quit {
                        self.state.persist_session_if_enabled();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            }
        }

        self.state.poll_loads();
        if self.state.poll_tail() {
            self.follow_caret = true;
        }
        let _ = self.state.tick_autosave();
        if !self.state.pending.is_empty()
            || self.state.tabs.iter().any(|d| d.tail_follow)
            || self.state.settings.autosave_secs() > 0
        {
            // Steady poll while tailing or autosave is armed; avoid hammering every frame.
            ctx.request_repaint_after(std::time::Duration::from_millis(300));
        }

        self.apply_theme_visuals(ctx);
        self.handle_file_drops(ctx);
        self.handle_shortcuts(ctx);
        self.menu_bar(ctx);
        self.refresh_compare_if_stale(ctx);
        if self.compare_on && self.state.settings.compare_hide_equal {
            crate::diff::prune_compare_hide_revealed(
                &mut self.compare_hide_revealed_left,
                &self.compare_left_tags,
                self.state.settings.compare_hide_equal_context_lines(),
            );
            crate::diff::prune_compare_hide_revealed(
                &mut self.compare_hide_revealed_right,
                &self.compare_right_tags,
                self.state.settings.compare_hide_equal_context_lines(),
            );
        }
        self.tab_bar(ctx);
        if self.state.find_open || self.show_replace {
            self.find_replace_bar(ctx);
        }
        // Bottom panel before CentralPanel so the editor height excludes the status bar.
        self.status_bar(ctx);
        self.editor_pane(ctx);
        self.about_window(ctx);
        self.preferences_window(ctx);
        self.log_tail_prompt_window(ctx);
        self.encoding_notice_window(ctx);
        self.lossy_ansi_confirm_window(ctx);
        self.unsaved_close_window(ctx);
        self.unsaved_reload_window(ctx);
        self.coming_soon_window(ctx);
        self.goto_line_window(ctx);
        self.summary_window(ctx);
        self.doc_list_window(ctx);
        self.project_panel_window(ctx);
        self.theme_picker_window(ctx);
        self.doc_map_window(ctx);
        self.func_list_window(ctx);
        self.char_panel_window(ctx);
    }
}

impl EditorApp {
    /// Open paths dropped onto the window (skip folders / missing).
    fn handle_file_drops(&mut self, ctx: &egui::Context) {
        let hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
        if hovering {
            ctx.set_cursor_icon(CursorIcon::Copy);
            if self.state.status.is_empty() || !self.state.status.starts_with("Drop ") {
                self.state.status = "Drop files here to open…".into();
            }
        }

        let dropped: Vec<PathBuf> = ctx.input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone())
                .collect()
        });
        if dropped.is_empty() {
            return;
        }

        let mut opened = 0usize;
        let mut skipped = 0usize;
        for path in dropped {
            if path.is_file() {
                self.state.open_path(path);
                opened += 1;
            } else {
                skipped += 1;
            }
        }
        self.state.status = if opened == 0 && skipped == 0 {
            "Drop ignored (no local paths)".into()
        } else if skipped == 0 {
            format!("Opened {opened} dropped file(s)")
        } else if opened == 0 {
            format!("Skipped {skipped} dropped path(s) (not files)")
        } else {
            format!("Opened {opened} dropped file(s); skipped {skipped}")
        };
    }

    fn finish_sel_text_drag(&mut self, copy: bool) {
        let Some(drag) = self.sel_text_drag.take() else {
            return;
        };
        let Some(doc) = self.state.tabs.get_mut(drag.tab) else {
            return;
        };
        if doc.read_only {
            self.state.status = "Read-only — cannot move or copy selection".into();
            return;
        }
        let ok = doc.buffer.drag_selection_to(drag.drop_at, copy);
        if ok {
            self.state.mark_text_changed_at(drag.tab);
            self.state.status = if copy {
                "Copied selection by drag".into()
            } else {
                "Moved selection by drag".into()
            };
            if drag.tab == self.state.tabs.active_index() {
                self.follow_caret = true;
            } else {
                self.follow_caret_other = true;
            }
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| {
            (
                i.modifiers,
                i.key_pressed(Key::N),
                i.key_pressed(Key::O),
                i.key_pressed(Key::S),
                i.key_pressed(Key::F),
                i.key_pressed(Key::Z),
                i.key_pressed(Key::Y),
                i.key_pressed(Key::W),
                i.key_pressed(Key::G),
                i.key_pressed(Key::A),
                i.key_pressed(Key::D),
                i.key_pressed(Key::L),
                i.key_pressed(Key::I),
                i.key_pressed(Key::T),
                i.key_pressed(Key::H),
                i.key_pressed(Key::F2),
                i.key_pressed(Key::F3),
                i.key_pressed(Key::F7),
                i.key_pressed(Key::ArrowLeft),
                i.key_pressed(Key::ArrowRight),
                i.key_pressed(Key::Equals),
                i.key_pressed(Key::Minus),
                i.key_pressed(Key::Num0),
                i.key_pressed(Key::CloseBracket),
                i.key_pressed(Key::OpenBracket),
                i.raw_scroll_delta.y,
            )
        });
        let (
            mods,
            n,
            o,
            s,
            f,
            z,
            y,
            w,
            g,
            a,
            d,
            l,
            i_key,
            t,
            h,
            f2,
            f3,
            f7,
            arrow_left,
            arrow_right,
            equals,
            minus,
            num0,
            close_br,
            open_br,
            scroll_y,
        ) = input;
        let cmd = mods.command || mods.ctrl;

        if cmd && mods.shift && t && self.state.toggle_tail_follow() {
            self.follow_caret = true;
        }
        if cmd && n {
            self.state.new_file();
        }
        if cmd && o {
            self.state.open_dialog();
        }
        if cmd && s && mods.shift {
            self.state.save_as_dialog();
        } else if cmd && s {
            self.state.save();
        }
        if (cmd && h && !mods.shift) || (cmd && f && mods.shift) {
            self.show_replace = true;
            self.state.find_open = true;
            self.find_focus_once = true;
            self.state.prepare_find_bar_from_selection();
        } else if cmd && f {
            self.state.find_open = true;
            self.show_replace = false;
            self.find_focus_once = true;
            self.state.prepare_find_bar_from_selection();
        }
        if cmd && a {
            let tab = self.focused_edit_tab();
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                doc.buffer.select_all();
            }
        }
        if cmd && d {
            let tab = self.focused_edit_tab();
            self.state.prepare_edit_at(tab);
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                doc.buffer.duplicate_line();
            }
            self.state.mark_text_changed_at(tab);
            self.follow_focused_caret();
        }
        if cmd && l && mods.shift {
            let tab = self.focused_edit_tab();
            self.state.prepare_edit_at(tab);
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                doc.buffer.delete_line();
            }
            self.state.mark_text_changed_at(tab);
            self.follow_focused_caret();
        } else if cmd && l {
            self.open_goto_line_dialog();
        }
        if cmd && close_br {
            let tab = self.focused_edit_tab();
            self.state.prepare_edit_at(tab);
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                let n = self.state.settings.tab_width.max(1) as usize;
                let pad = " ".repeat(n);
                doc.buffer.indent_lines(&pad);
            }
            self.state.mark_text_changed_at(tab);
            self.follow_focused_caret();
        }
        if cmd && open_br {
            let tab = self.focused_edit_tab();
            self.state.prepare_edit_at(tab);
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                let n = self.state.settings.tab_width.max(1) as usize;
                doc.buffer.outdent_lines(n);
            }
            self.state.mark_text_changed_at(tab);
            self.follow_focused_caret();
        }
        if cmd && mods.shift && i_key {
            self.state.format_document();
            self.follow_caret = true;
        }

        if cmd && z && mods.shift {
            self.state.redo_at(self.focused_edit_tab());
        } else if cmd && z {
            self.state.undo_at(self.focused_edit_tab());
        }

        // Remappable word wrap (default Alt+Z; settings.shortcut_word_wrap).
        {
            use crate::shortcut_chord::{resolve_chord, DEFAULT_WORD_WRAP};
            let wrap_chord =
                resolve_chord(&self.state.settings.shortcut_word_wrap, DEFAULT_WORD_WRAP);
            // Collect pressed letter/function keys we already sampled; Z is enough for default.
            let wrap_key_pressed = match wrap_chord.key {
                Key::Z => z,
                Key::N => n,
                Key::O => o,
                Key::S => s,
                Key::F => f,
                Key::Y => y,
                Key::W => w,
                Key::G => g,
                Key::A => a,
                Key::D => d,
                Key::L => l,
                Key::I => i_key,
                Key::T => t,
                Key::H => h,
                Key::F2 => f2,
                Key::F3 => f3,
                Key::Equals => equals,
                Key::Minus => minus,
                Key::Num0 => num0,
                Key::CloseBracket => close_br,
                Key::OpenBracket => open_br,
                other => ctx.input(|i| i.key_pressed(other)),
            };
            if wrap_key_pressed && wrap_chord.matches(mods, wrap_chord.key) {
                self.state.word_wrap = !self.state.word_wrap;
                self.state.settings.word_wrap = self.state.word_wrap;
                self.state.settings.save();
                self.state.status = format!(
                    "Word wrap: {}",
                    if self.state.word_wrap { "on" } else { "off" }
                );
            }
        }
        if cmd && y {
            self.state.redo_at(self.focused_edit_tab());
        }
        if cmd && w {
            let idx = self.state.tabs.active_index();
            self.state.request_close_tab(idx);
            self.scroll_line = 0.0;
        }

        // Find next/prev: remappable chord (default F3; Shift flips to prev) + Cmd+G / Cmd+Shift+G.
        {
            use crate::shortcut_chord::{resolve_chord, DEFAULT_FIND_NEXT};
            let find_chord =
                resolve_chord(&self.state.settings.shortcut_find_next, DEFAULT_FIND_NEXT);
            let find_key_pressed = match find_chord.key {
                Key::Z => z,
                Key::N => n,
                Key::O => o,
                Key::S => s,
                Key::F => f,
                Key::Y => y,
                Key::W => w,
                Key::G => g,
                Key::A => a,
                Key::D => d,
                Key::L => l,
                Key::I => i_key,
                Key::T => t,
                Key::H => h,
                Key::F2 => f2,
                Key::F3 => f3,
                Key::Equals => equals,
                Key::Minus => minus,
                Key::Num0 => num0,
                Key::CloseBracket => close_br,
                Key::OpenBracket => open_br,
                other => ctx.input(|i| i.key_pressed(other)),
            };
            let remapped_prev =
                find_key_pressed && find_chord.flipped_shift().matches(mods, find_chord.key);
            let remapped_next = find_key_pressed && find_chord.matches(mods, find_chord.key);
            let find_prev_key = remapped_prev || (cmd && g && mods.shift);
            let find_next_key = remapped_next || (cmd && g && !mods.shift);
            if find_prev_key {
                if self.state.find_query.is_empty() {
                    self.seed_find_from_selection();
                }
                self.state.find_prev();
                self.follow_focused_caret();
            } else if find_next_key {
                if self.state.find_query.is_empty() {
                    self.seed_find_from_selection();
                }
                self.state.find_next();
                self.follow_focused_caret();
            }
        }

        // Bookmarks: F2 next, Shift+F2 prev, Cmd+F2 toggle.
        if f2 && cmd {
            self.run_shortcut_cmd("IDM_SEARCH_TOGGLE_BOOKMARK");
        } else if f2 && mods.shift {
            self.run_shortcut_cmd("IDM_SEARCH_PREV_BOOKMARK");
        } else if f2 {
            self.run_shortcut_cmd("IDM_SEARCH_NEXT_BOOKMARK");
        }

        // Compare hunks: F7 next, Shift+F7 previous, ⌘/Ctrl+F7 first, ⌘/Ctrl+Shift+F7 last.
        // Hidden Equal (···N gaps): Alt+F7 family (same modifiers as hunk nav).
        if f7 && mods.alt && cmd && mods.shift {
            self.run_shortcut_cmd("IDM_VIEW_COMPARE_LAST_HIDDEN_EQUAL");
        } else if f7 && mods.alt && cmd {
            self.run_shortcut_cmd("IDM_VIEW_COMPARE_FIRST_HIDDEN_EQUAL");
        } else if f7 && mods.alt && mods.shift {
            self.run_shortcut_cmd("IDM_VIEW_COMPARE_PREV_HIDDEN_EQUAL");
        } else if f7 && mods.alt {
            self.run_shortcut_cmd("IDM_VIEW_COMPARE_NEXT_HIDDEN_EQUAL");
        } else if f7 && cmd && mods.shift {
            self.run_shortcut_cmd("IDM_VIEW_LAST_DIFF");
        } else if f7 && cmd {
            self.run_shortcut_cmd("IDM_VIEW_FIRST_DIFF");
        } else if f7 && mods.shift {
            self.run_shortcut_cmd("IDM_VIEW_PREV_DIFF");
        } else if f7 {
            self.run_shortcut_cmd("IDM_VIEW_NEXT_DIFF");
        }

        // Compare apply hunk: ⌘/Ctrl+Alt+←/→ one hunk; +Shift applies all.
        // Alt alone stays word-jump in the editor; command+alt skips caret move there.
        if cmd && mods.alt && mods.shift && arrow_left {
            self.run_shortcut_cmd("IDM_VIEW_APPLY_ALL_COMPARE_HUNKS");
        } else if cmd && mods.alt && mods.shift && arrow_right {
            self.run_shortcut_cmd("IDM_VIEW_APPLY_ALL_COMPARE_HUNKS_TO_OTHER");
        } else if cmd && mods.alt && arrow_left {
            self.run_shortcut_cmd("IDM_VIEW_APPLY_COMPARE_HUNK");
        } else if cmd && mods.alt && arrow_right {
            self.run_shortcut_cmd("IDM_VIEW_APPLY_COMPARE_HUNK_TO_OTHER");
        }

        // Hide-equal context ±1: Alt+] increase, Alt+[ decrease (Cmd+[ / ] stay indent).
        if mods.alt && !cmd && close_br {
            self.run_shortcut_cmd("IDM_VIEW_COMPARE_HIDE_EQUAL_CONTEXT_INC");
        } else if mods.alt && !cmd && open_br {
            self.run_shortcut_cmd("IDM_VIEW_COMPARE_HIDE_EQUAL_CONTEXT_DEC");
        }

        // Zoom: Cmd+= / Cmd+- / Cmd+0, and Cmd+mouse wheel.
        if cmd && equals {
            self.font_size = (self.font_size + 1.0).min(48.0);
            self.persist_font_size();
        } else if cmd && minus {
            self.font_size = (self.font_size - 1.0).max(8.0);
            self.persist_font_size();
        } else if cmd && num0 {
            self.font_size = 14.0;
            self.persist_font_size();
        } else if cmd && scroll_y != 0.0 {
            if scroll_y > 0.0 {
                self.font_size = (self.font_size + 1.0).min(48.0);
            } else {
                self.font_size = (self.font_size - 1.0).max(8.0);
            }
            self.persist_font_size();
        }

        if (self.state.find_open || self.show_replace) && ctx.input(|i| i.key_pressed(Key::Escape))
        {
            self.state.find_open = false;
            self.show_replace = false;
        }
    }

    fn open_goto_line_dialog(&mut self) {
        self.show_goto_line = true;
        let line = self
            .state
            .tabs
            .active()
            .buffer
            .char_to_line(self.state.tabs.active().buffer.caret())
            + 1;
        self.goto_line_input = line.to_string();
    }

    fn run_shortcut_cmd(&mut self, cmd: &str) {
        let mut flags = crate::commands::UiFlags::default();
        let _ = self.dispatch_menu_cmd(cmd, &mut flags);
        if flags.follow_caret {
            self.follow_focused_caret();
        }
        if flags.show_goto_line {
            self.open_goto_line_dialog();
        }
        match flags.zoom_delta {
            Some(1) => {
                self.font_size = (self.font_size + 1.0).min(48.0);
                self.persist_font_size();
            }
            Some(-1) => {
                self.font_size = (self.font_size - 1.0).max(8.0);
                self.persist_font_size();
            }
            Some(0) => {
                self.font_size = 14.0;
                self.persist_font_size();
            }
            _ => {}
        }
        self.apply_dual_view_flags(&mut flags);
    }

    fn menu_bar(&mut self, ctx: &egui::Context) {
        let menu = crate::menu_data::load_npp_menu();
        let mut flags = crate::commands::UiFlags {
            show_about: self.show_about,
            show_preferences: self.show_preferences,
            find_open: self.state.find_open,
            show_replace: self.show_replace,
            find_focus_once: self.find_focus_once,
            follow_caret: self.follow_caret,
            last_copied: self.last_app_clipboard.clone(),
            ..Default::default()
        };
        let mut run_cmd: Option<String> = None;

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                for node in &menu {
                    self.render_menu_node(ui, node, &mut run_cmd, true);
                }
            });
        });

        if let Some(cmd) = run_cmd {
            // Edit/Format menus mutate the focused dual-view pane, not only the tab-bar active tab.
            let result = self.dispatch_menu_cmd(&cmd, &mut flags);
            if result == crate::commands::CmdResult::Stub {
                self.coming_soon = Some(crate::commands::coming_soon_for(&cmd));
                if let Some(cs) = self.coming_soon.as_ref() {
                    self.state.status = format!("Coming soon: {}", cs.feature);
                }
            }
            if flags.coming_soon.is_some() {
                self.coming_soon = flags.coming_soon.take();
            }
            self.show_about = flags.show_about;
            self.show_preferences = flags.show_preferences;
            self.state.find_open = flags.find_open;
            self.show_replace = flags.show_replace;
            self.find_focus_once = flags.find_focus_once;
            if flags.follow_caret {
                self.follow_focused_caret();
            } else {
                self.follow_caret = false;
            }
            if flags.show_goto_line {
                self.open_goto_line_dialog();
            }
            if flags.show_summary {
                self.show_summary = true;
            }
            if flags.show_doc_list {
                self.show_doc_list = true;
            }
            if flags.show_project_panel {
                self.show_project_panel = true;
            }
            if flags.show_theme_picker {
                self.show_theme_picker = true;
            }
            if flags.show_doc_map {
                self.show_doc_map = true;
            }
            if flags.show_func_list {
                self.show_func_list = true;
            }
            if flags.show_char_panel {
                self.show_char_panel = true;
            }
            match flags.zoom_delta {
                Some(1) => {
                    self.font_size = (self.font_size + 1.0).min(48.0);
                    self.persist_font_size();
                }
                Some(-1) => {
                    self.font_size = (self.font_size - 1.0).max(8.0);
                    self.persist_font_size();
                }
                Some(0) => {
                    self.font_size = 14.0;
                    self.persist_font_size();
                }
                _ => {}
            }
            self.apply_dual_view_flags(&mut flags);
            if let Some(on) = flags.always_on_top {
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(if on {
                    egui::WindowLevel::AlwaysOnTop
                } else {
                    egui::WindowLevel::Normal
                }));
            }
            if flags.fullscreen_toggle {
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
                    !ctx.input(|i| i.viewport().fullscreen.unwrap_or(false)),
                ));
            }
            if let Some(t) = flags.pending_clipboard.take() {
                self.last_app_clipboard = Some(t.clone());
                ctx.copy_text(t);
            } else if let Some(t) = flags.last_copied.take() {
                self.last_app_clipboard = Some(t);
            }
            if flags.await_paste_bookmarks {
                self.await_paste_bookmarks = true;
            }
            if flags.request_quit {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if flags.highlight_dirty_scroll_reset {
                self.scroll_line = 0.0;
            }
        }
    }

    fn render_menu_node(
        &mut self,
        ui: &mut egui::Ui,
        node: &crate::menu_data::MenuNode,
        run_cmd: &mut Option<String>,
        top_level: bool,
    ) {
        use crate::menu_data::MenuNode;
        match node {
            MenuNode::Separator => {
                ui.separator();
            }
            MenuNode::Item { label, cmd } => {
                let checked = match cmd.as_str() {
                    "IDM_VIEW_COMPARE_IGNORE_WS" => self.state.settings.compare_ignore_ws,
                    "IDM_VIEW_COMPARE_IGNORE_CASE" => self.state.settings.compare_ignore_case,
                    "IDM_VIEW_COMPARE_IGNORE_BLANK" => self.state.settings.compare_ignore_blank,
                    "IDM_VIEW_COMPARE_HIDE_EQUAL" => self.state.settings.compare_hide_equal,
                    _ => false,
                };
                let shown = if checked {
                    format!("✓ {label}")
                } else {
                    label.clone()
                };
                let text = if crate::commands::is_implemented(cmd) {
                    RichText::new(shown).color(MENU_READY)
                } else {
                    RichText::new(shown)
                };
                let response = ui.button(text);
                let response = match cmd.as_str() {
                    "IDM_OPEN_NPP_LOGS" => response.on_hover_text(
                        "Open logs/*.log relative to the process cwd (e.g. logs/panic.log)",
                    ),
                    "IDM_DEBUGINFO" => {
                        response.on_hover_text("Open a tab with version, OS, and log status")
                    }
                    "IDM_VIEW_COMPARE_TO_SAVED" => response.on_hover_text(
                        "Compare the active file to its last-saved disk contents (read-only snapshot)",
                    ),
                    "IDM_VIEW_COMPARE_IGNORE_WS" => response.on_hover_text(
                        "Toggle ignore whitespace for Compare (Preferences persist)",
                    ),
                    "IDM_VIEW_COMPARE_IGNORE_CASE" => response.on_hover_text(
                        "Toggle ignore letter case for Compare (Preferences persist)",
                    ),
                    "IDM_VIEW_COMPARE_IGNORE_BLANK" => response.on_hover_text(
                        "Toggle ignore blank lines for Compare (Preferences persist)",
                    ),
                    "IDM_VIEW_COMPARE_HIDE_EQUAL" => response.on_hover_text(
                        "Hide Equal (unchanged) lines while Compare is on; parks both panes on the first change hunk; context size is Preferences Hide-equal context; gutter ···N marks collapsed runs — click a cue to expand that run on both panes (Preferences persist)",
                    ),
                    "IDM_VIEW_COMPARE_HIDE_EQUAL_CONTEXT_INC" => response.on_hover_text(
                        "Increase Equal context lines kept around each change when Hide Unchanged Lines is on (Alt+]; 0–10; Preferences persist; while Compare is on parks both panes on the first change hunk)",
                    ),
                    "IDM_VIEW_COMPARE_HIDE_EQUAL_CONTEXT_DEC" => response.on_hover_text(
                        "Decrease Equal context lines kept around each change when Hide Unchanged Lines is on (Alt+[; 0–10; Preferences persist; while Compare is on parks both panes on the first change hunk)",
                    ),
                    "IDM_VIEW_COMPARE_EXPAND_HIDDEN_EQUAL_AT_CARET" => response.on_hover_text(
                        "While Hide Unchanged Lines is on, expand the collapsed Equal run nearest the caret on both panes (same as clicking that ···N cue)",
                    ),
                    "IDM_VIEW_COMPARE_COLLAPSE_HIDDEN_EQUAL_AT_CARET" => response.on_hover_text(
                        "While Hide Unchanged Lines is on, re-collapse the expanded Equal run nearest the caret on both panes (···N cue returns for that run)",
                    ),
                    "IDM_VIEW_COMPARE_NEXT_HIDDEN_EQUAL" => response.on_hover_text(
                        "While Hide Unchanged Lines is on, jump to the next ···N collapsed Equal gap (Alt+F7; wraps; parks both panes)",
                    ),
                    "IDM_VIEW_COMPARE_PREV_HIDDEN_EQUAL" => response.on_hover_text(
                        "While Hide Unchanged Lines is on, jump to the previous ···N collapsed Equal gap (Alt+Shift+F7; wraps; parks both panes)",
                    ),
                    "IDM_VIEW_COMPARE_FIRST_HIDDEN_EQUAL" => response.on_hover_text(
                        "While Hide Unchanged Lines is on, jump to the first ···N collapsed Equal gap (⌘/Ctrl+Alt+F7; parks both panes)",
                    ),
                    "IDM_VIEW_COMPARE_LAST_HIDDEN_EQUAL" => response.on_hover_text(
                        "While Hide Unchanged Lines is on, jump to the last ···N collapsed Equal gap (⌘/Ctrl+Alt+Shift+F7; parks both panes)",
                    ),
                    "IDM_VIEW_COMPARE_EXPAND_HIDDEN_EQUAL" => response.on_hover_text(
                        "While Hide Unchanged Lines is on, reveal every collapsed Equal run on both panes (···N cues clear; hide-equal preference stays on)",
                    ),
                    "IDM_VIEW_COMPARE_COLLAPSE_HIDDEN_EQUAL" => response.on_hover_text(
                        "While Hide Unchanged Lines is on, re-collapse every click/menu-expanded Equal run on both panes (···N cues return; hide-equal preference stays on)",
                    ),
                    "IDM_VIEW_COMPARE_BOOKMARK_DIFFS" => response.on_hover_text(
                        "Bookmark the start of every Compare change hunk on both panes (then F2 / Shift+F2)",
                    ),
                    "IDM_VIEW_COMPARE_CLEAR_DIFF_BOOKMARKS" => response.on_hover_text(
                        "Remove bookmarks at every Compare change-hunk start on both panes (other bookmarks stay)",
                    ),
                    "IDM_VIEW_COPY_COMPARE_DIFF" => response
                        .on_hover_text("Copy the Compare pair as a unified diff (clipboard)"),
                    "IDM_VIEW_OPEN_COMPARE_DIFF" => response.on_hover_text(
                        "Open the Compare pair as a unified-diff tab (clears Compare)",
                    ),
                    "IDM_VIEW_COPY_COMPARE_SUMMARY" => response.on_hover_text(
                        "Copy a text summary of every Compare change hunk (clipboard)",
                    ),
                    "IDM_VIEW_OPEN_COMPARE_SUMMARY" => response.on_hover_text(
                        "Open a tab listing every Compare change hunk with L|R line ranges (clears Compare)",
                    ),
                    "IDM_VIEW_COPY_COMPARE_HUNK" => response.on_hover_text(
                        "Copy the change hunk at the caret as a unified diff (clipboard)",
                    ),
                    "IDM_VIEW_OPEN_COMPARE_HUNK" => response.on_hover_text(
                        "Open the change hunk at the caret as a unified-diff tab (clears Compare)",
                    ),
                    "IDM_VIEW_APPLY_COMPARE_HUNK" => response.on_hover_text(
                        "Replace the focused pane's change hunk with the other pane (⌘/Ctrl+Alt+←, one undo)",
                    ),
                    "IDM_VIEW_APPLY_COMPARE_HUNK_TO_OTHER" => response.on_hover_text(
                        "Replace the other pane's change hunk with the focused pane (⌘/Ctrl+Alt+→, one undo)",
                    ),
                    "IDM_VIEW_APPLY_ALL_COMPARE_HUNKS" => response.on_hover_text(
                        "Replace every change hunk on the focused pane with the other pane (⌘/Ctrl+Alt+Shift+←, one undo)",
                    ),
                    "IDM_VIEW_APPLY_ALL_COMPARE_HUNKS_TO_OTHER" => response.on_hover_text(
                        "Replace every change hunk on the other pane with the focused pane (⌘/Ctrl+Alt+Shift+→, one undo)",
                    ),
                    _ => response,
                };
                if response.clicked() {
                    *run_cmd = Some(cmd.clone());
                    ui.close_menu();
                }
            }
            MenuNode::Popup { label, children } => {
                // Inject Recent Files into File menu (Notepad++ inserts this at runtime).
                let is_file = top_level && label == "File";
                let is_plugins = top_level && label == "Plugins";
                ui.menu_button(label, |ui| {
                    if is_file {
                        // Match N++: recent list near the end; we place after Open block via full tree,
                        // and also expose an explicit Recent submenu at the top of File for clarity.
                        ui.menu_button(RichText::new("Recent Files").color(MENU_READY), |ui| {
                            let recent_paths: Vec<_> = self.state.recent.paths().to_vec();
                            if recent_paths.is_empty() {
                                ui.label(RichText::new("(empty)").italics().weak());
                            } else {
                                let mut open_path = None;
                                for (i, path) in recent_paths.iter().enumerate() {
                                    let label = crate::recent::recent_label(path);
                                    let exists = path.exists();
                                    let text = if exists {
                                        RichText::new(format!("{}.  {label}", i + 1))
                                            .color(MENU_READY)
                                    } else {
                                        RichText::new(format!("{}.  {label}  (missing)", i + 1))
                                            .weak()
                                    };
                                    if ui
                                        .add_enabled(exists, egui::Button::new(text))
                                        .on_hover_text(path.display().to_string())
                                        .clicked()
                                    {
                                        open_path = Some(path.clone());
                                    }
                                }
                                ui.separator();
                                if ui
                                    .button(RichText::new("Clear Recent Files").color(MENU_READY))
                                    .clicked()
                                {
                                    self.state.clear_recent();
                                    ui.close_menu();
                                }
                                if let Some(path) = open_path {
                                    self.state.open_path(path);
                                    ui.close_menu();
                                }
                            }
                        });
                        ui.separator();
                    }
                    for child in children {
                        self.render_menu_node(ui, child, run_cmd, false);
                    }
                    if is_plugins {
                        ui.separator();
                        ui.label(RichText::new("npp-rs builtins").small().weak());
                        let host = plugins::PluginHost::new();
                        let mut run_id = None;
                        for p in host.list() {
                            if ui
                                .button(RichText::new(p.name()).color(MENU_READY))
                                .clicked()
                            {
                                run_id = Some(p.id().to_string());
                            }
                        }
                        if ui
                            .button(RichText::new("Format Document").color(MENU_READY))
                            .clicked()
                        {
                            self.state.format_document();
                            ui.close_menu();
                        }
                        if let Some(id) = run_id {
                            self.state.run_plugin(&id);
                            ui.close_menu();
                        }
                    }
                });
            }
        }
    }

    fn about_window(&mut self, ctx: &egui::Context) {
        if !self.show_about {
            return;
        }
        let mut open = true;
        egui::Window::new("About npp-rust")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size([440.0, 420.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.heading(
                        RichText::new("npp-rust")
                            .size(28.0)
                            .color(Color32::from_rgb(120, 200, 255)),
                    );
                    ui.label(
                        RichText::new("a Notepad++ inspired editor, rebuilt for fun")
                            .italics()
                            .color(Color32::from_rgb(180, 180, 190)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(format!(
                            "v{} · {} · Rust · macOS / Linux / Windows",
                            env!("CARGO_PKG_VERSION"),
                            env!("NPP_GIT_HASH")
                        ))
                        .small(),
                    );
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.hyperlink_to("GitHub", "https://github.com/raro42/npp-rust");
                        ui.label("·");
                        ui.hyperlink_to("Issues", "https://github.com/raro42/npp-rust/issues");
                        ui.label("·");
                        ui.hyperlink_to(
                            "Discussions",
                            "https://github.com/raro42/npp-rust/discussions",
                        );
                        ui.label("·");
                        ui.hyperlink_to("Wiki", "https://github.com/raro42/npp-rust/wiki");
                        ui.label("·");
                        ui.hyperlink_to("Releases", "https://github.com/raro42/npp-rust/releases");
                        ui.label("·");
                        ui.hyperlink_to(
                            "Changelog",
                            "https://github.com/raro42/npp-rust/blob/main/docs/changelog.md",
                        );
                    });
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                ui.label(RichText::new("Why it exists").strong());
                ui.label(
                    "Built as a side adventure — something nice to grow in the background \
while other work runs. Not a line-by-line port. A fresh editor with a rope buffer, \
Tree-sitter highlight, and a calm UI.",
                );

                ui.add_space(10.0);
                ui.label(RichText::new("Made by").strong());
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Ralf Roeber").strong());
                    ui.label("·");
                    ui.label("El Masnou (Barcelona), Catalonia");
                });
                ui.label(
                    RichText::new("Germany roots · Spain home · open source habit")
                        .small()
                        .color(Color32::from_rgb(150, 150, 160)),
                );

                ui.add_space(10.0);
                ui.label(RichText::new("Shortcuts").strong());
                let wrap_keys = crate::shortcut_chord::resolve_chord(
                    &self.state.settings.shortcut_word_wrap,
                    crate::shortcut_chord::DEFAULT_WORD_WRAP,
                )
                .display();
                let find_chord = crate::shortcut_chord::resolve_chord(
                    &self.state.settings.shortcut_find_next,
                    crate::shortcut_chord::DEFAULT_FIND_NEXT,
                );
                let find_keys = format!(
                    "{} / {}",
                    find_chord.display(),
                    find_chord.flipped_shift().display()
                );
                egui::Grid::new("about_shortcuts")
                    .num_columns(2)
                    .spacing([16.0, 4.0])
                    .show(ui, |ui| {
                        let rows: [(&str, &str); 27] = [
                            ("⌘/Ctrl N", "New file"),
                            ("⌘/Ctrl O", "Open"),
                            ("⌘/Ctrl S", "Save"),
                            ("⌘/Ctrl ⇧ S", "Save As"),
                            ("⌘/Ctrl F / H", "Find / Replace"),
                            (find_keys.as_str(), "Find next / prev"),
                            ("⌘/Ctrl G", "Find next (global)"),
                            ("⌘/Ctrl L", "Go to line"),
                            ("F2 / ⇧ F2", "Next / prev bookmark"),
                            ("F7 / ⇧ F7", "Compare next / prev diff"),
                            ("⌘/Ctrl F7 · ⌘/Ctrl ⇧ F7", "Compare first / last diff"),
                            ("Alt F7 / Alt ⇧ F7", "Compare next / prev hidden equal"),
                            (
                                "⌘/Ctrl Alt F7 · ⌘/Ctrl Alt ⇧ F7",
                                "Compare first / last hidden equal",
                            ),
                            ("⌘/Ctrl Alt ←/→", "Compare apply hunk from / to other"),
                            (
                                "⌘/Ctrl Alt ⇧ ←/→",
                                "Compare apply all hunks from / to other",
                            ),
                            ("Alt ] / [", "Compare hide-equal context ±1"),
                            ("⌘/Ctrl = / -", "Zoom in / out"),
                            (wrap_keys.as_str(), "Word wrap"),
                            ("⌘/Ctrl A", "Select all"),
                            ("⌘/Ctrl D", "Duplicate line"),
                            ("⌘/Ctrl ] / [", "Indent / Outdent"),
                            ("⌘/Ctrl ⇧ I", "Format document"),
                            ("⌘/Ctrl ⇧ T", "Toggle log tail"),
                            ("Alt ←/→", "Word jump"),
                            ("Double-click", "Select word"),
                            ("⌘/Ctrl Z / Y", "Undo / Redo"),
                            ("⌘/Ctrl W", "Close tab"),
                        ];
                        for (keys, action) in rows {
                            ui.monospace(keys);
                            ui.label(action);
                            ui.end_row();
                        }
                    });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Inspired by Notepad++ · MIT · separate project")
                            .small()
                            .color(Color32::from_rgb(140, 140, 150)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            self.show_about = false;
                        }
                    });
                });
            });
        if !open {
            self.show_about = false;
        }
    }

    fn preferences_window(&mut self, ctx: &egui::Context) {
        if !self.show_preferences {
            return;
        }
        use crate::recent::{DefaultEol, LogTailOnOpen};
        let mut open = true;
        let mut changed = false;
        egui::Window::new("Preferences")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(440.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(480.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new("When opening *.log files").strong());
                        ui.add_space(4.0);
                        let cur = &mut self.state.settings.log_tail_on_open;
                        if ui
                            .radio_value(cur, LogTailOnOpen::Ask, "Ask each time")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .radio_value(
                                cur,
                                LogTailOnOpen::Always,
                                "Always enable Monitoring (tail)",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .radio_value(
                                cur,
                                LogTailOnOpen::Never,
                                "Never ask — open as a normal file",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Editor").strong());
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("Font size");
                            if ui
                                .add(egui::Slider::new(&mut self.font_size, 8.0..=48.0))
                                .changed()
                            {
                                changed = true;
                                self.state.settings.font_size = self.font_size;
                            }
                        });
                        if ui
                            .checkbox(
                                &mut self.state.settings.show_line_numbers,
                                "Show line numbers",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.show_fold_margin,
                                "Show fold margin",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.horizontal(|ui| {
                            ui.label("Gutter extra");
                            let mut g = self.state.settings.gutter_extra as i32;
                            if ui.add(egui::Slider::new(&mut g, 0..=40)).changed() {
                                self.state.settings.gutter_extra = g as u8;
                                changed = true;
                            }
                        });
                        if ui
                            .checkbox(&mut self.state.settings.caret_blink, "Caret blink")
                            .changed()
                        {
                            changed = true;
                        }
                        ui.horizontal(|ui| {
                            ui.label("Tab width");
                            let mut tw = self.state.settings.tab_width as i32;
                            if ui.add(egui::Slider::new(&mut tw, 2..=8)).changed() {
                                self.state.settings.tab_width = tw as u8;
                                changed = true;
                            }
                        });
                        if ui
                            .checkbox(&mut self.state.settings.word_wrap, "Word wrap")
                            .changed()
                        {
                            self.state.word_wrap = self.state.settings.word_wrap;
                            changed = true;
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.show_whitespace,
                                "Show white space and TAB",
                            )
                            .changed()
                        {
                            self.state.show_whitespace = self.state.settings.show_whitespace;
                            changed = true;
                        }
                        if ui
                            .checkbox(&mut self.state.settings.show_eol, "Show EOL")
                            .changed()
                        {
                            self.state.show_eol = self.state.settings.show_eol;
                            changed = true;
                        }
                        if ui
                            .checkbox(&mut self.state.settings.show_npc, "Show NPC")
                            .changed()
                        {
                            self.state.show_npc = self.state.settings.show_npc;
                            changed = true;
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.show_indent_guide,
                                "Show indent guide",
                            )
                            .changed()
                        {
                            self.state.show_indent_guide = self.state.settings.show_indent_guide;
                            changed = true;
                        }
                        ui.horizontal(|ui| {
                            ui.label("Word wrap shortcut");
                            let edit = ui.add(
                                egui::TextEdit::singleline(
                                    &mut self.state.settings.shortcut_word_wrap,
                                )
                                .desired_width(120.0)
                                .hint_text("Alt+Z"),
                            );
                            // Persist only when focus leaves so half-typed chords are not saved.
                            if edit.lost_focus() {
                                let raw = self.state.settings.shortcut_word_wrap.trim().to_string();
                                if crate::shortcut_chord::parse_chord(&raw).is_none() {
                                    self.state.settings.shortcut_word_wrap =
                                        crate::shortcut_chord::DEFAULT_WORD_WRAP.into();
                                    self.state.status = format!(
                                        "Invalid shortcut; reset to {}",
                                        crate::shortcut_chord::DEFAULT_WORD_WRAP
                                    );
                                } else {
                                    self.state.settings.shortcut_word_wrap = raw;
                                }
                                changed = true;
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Find next shortcut");
                            let edit = ui.add(
                                egui::TextEdit::singleline(
                                    &mut self.state.settings.shortcut_find_next,
                                )
                                .desired_width(120.0)
                                .hint_text("F3"),
                            );
                            if edit.lost_focus() {
                                let raw = self.state.settings.shortcut_find_next.trim().to_string();
                                if crate::shortcut_chord::parse_chord(&raw).is_none() {
                                    self.state.settings.shortcut_find_next =
                                        crate::shortcut_chord::DEFAULT_FIND_NEXT.into();
                                    self.state.status = format!(
                                        "Invalid shortcut; reset to {}",
                                        crate::shortcut_chord::DEFAULT_FIND_NEXT
                                    );
                                } else {
                                    self.state.settings.shortcut_find_next = raw;
                                }
                                changed = true;
                            }
                        });
                        ui.label(
                            RichText::new(
                                "Example: F3, Alt+N. Shift + that key is Find previous. Cmd/Ctrl+G stays hard-wired.",
                            )
                            .small()
                            .weak(),
                        );
                        ui.label("Default EOL (Enter key)");
                        let eol = &mut self.state.settings.default_eol;
                        if ui
                            .radio_value(eol, DefaultEol::Lf, DefaultEol::Lf.label())
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .radio_value(eol, DefaultEol::Crlf, DefaultEol::Crlf.label())
                            .changed()
                        {
                            changed = true;
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Files").strong());
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("Recent file count");
                            let mut rm = self.state.settings.recent_max as i32;
                            if ui.add(egui::Slider::new(&mut rm, 5..=40)).changed() {
                                self.state.settings.recent_max = rm as u8;
                                changed = true;
                            }
                        });
                        if ui
                            .checkbox(
                                &mut self.state.settings.restore_session,
                                "Restore last session on launch",
                            )
                            .changed()
                        {
                            changed = true;
                            if self.state.settings.restore_session {
                                self.state.persist_session_if_enabled();
                            }
                        }
                        ui.label(
                            RichText::new(format!("Session file: {}", crate::session::SESSION_REL))
                                .small()
                                .weak(),
                        );
                        if ui
                            .checkbox(
                                &mut self.state.settings.backup_on_save,
                                "Backup on save (copy into config backup folder)",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.label(
                            RichText::new(format!(
                                "Backup folder: {} (mirrors path layout)",
                                crate::backup::BACKUP_REL
                            ))
                            .small()
                            .weak(),
                        );
                        ui.horizontal(|ui| {
                            ui.label("Autosave interval (sec)");
                            let mut secs = self.state.settings.autosave_interval_secs as i32;
                            if ui.add(egui::Slider::new(&mut secs, 0..=900)).changed() {
                                if secs > 0 && secs < 15 {
                                    secs = 15;
                                }
                                self.state.settings.autosave_interval_secs = secs as u32;
                                changed = true;
                            }
                        });
                        ui.label(
                            RichText::new(
                                "0 = off; otherwise 15–900. Dirty tabs with a path only (skip untitled).",
                            )
                            .small()
                            .weak(),
                        );
                        ui.add_space(10.0);
                        ui.label(RichText::new("Find").strong());
                        ui.add_space(4.0);
                        if ui
                            .checkbox(&mut self.state.settings.find_match_case, "Match case")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .checkbox(&mut self.state.settings.find_whole_word, "Whole word")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.find_in_selection,
                                "In selection",
                            )
                            .changed()
                        {
                            changed = true;
                            if self.state.settings.find_in_selection {
                                self.state.capture_find_scope();
                            } else {
                                self.state.find_scope = None;
                            }
                        }
                        if ui
                            .checkbox(&mut self.state.settings.find_wrap, "Wrap around")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .checkbox(&mut self.state.settings.find_regex, "Regular expression")
                            .on_hover_text(
                                "Find uses a linear-time regex. Replace expands $n / \\n capture groups",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Compare").strong());
                        ui.add_space(4.0);
                        if ui
                            .checkbox(
                                &mut self.state.settings.compare_ignore_ws,
                                "Ignore whitespace differences",
                            )
                            .changed()
                        {
                            changed = true;
                            if self.compare_on {
                                self.state.compare_stale = true;
                            }
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.compare_ignore_case,
                                "Ignore case differences",
                            )
                            .changed()
                        {
                            changed = true;
                            if self.compare_on {
                                self.state.compare_stale = true;
                            }
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.compare_ignore_blank,
                                "Ignore blank lines",
                            )
                            .changed()
                        {
                            changed = true;
                            if self.compare_on {
                                self.state.compare_stale = true;
                            }
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.compare_hide_equal,
                                "Hide unchanged lines",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.horizontal(|ui| {
                            ui.label("Hide-equal context");
                            let mut c = i32::from(self.state.settings.compare_hide_equal_context);
                            if ui
                                .add(egui::Slider::new(&mut c, 0..=10))
                                .on_hover_text(
                                    "Equal lines kept visible on each side of a change when Hide unchanged lines is on (0–10; default 3; while Compare is on parks both panes on the first change hunk)",
                                )
                                .changed()
                            {
                                self.set_compare_hide_equal_context(c.clamp(0, 10) as usize);
                                changed = true;
                            }
                        });
                        ui.add_space(10.0);
                        ui.label(RichText::new("Status bar").strong());
                        ui.add_space(4.0);
                        if ui
                            .checkbox(&mut self.state.settings.status_show_lang, "Show language")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.status_show_chars,
                                "Show character count",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Theme").strong());
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(
                                "JSON tokens + chrome; N++ XML subset (GlobalStyles + one lexer).",
                            )
                            .small()
                            .weak(),
                        );
                        let mut theme_id = self.state.settings.theme_id.clone();
                        for (id, label) in crate::theme::list_theme_choices() {
                            if ui.radio_value(&mut theme_id, id.clone(), label).changed() {
                                self.state.settings.theme_id = theme_id.clone();
                                changed = true;
                            }
                        }
                        if ui.button("Open Theme picker…").clicked() {
                            self.show_theme_picker = true;
                        }
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!("Saved to {}", crate::recent::SETTINGS_REL))
                                .small()
                                .weak(),
                        );
                        ui.add_space(8.0);
                        if ui.button("Close").clicked() {
                            self.show_preferences = false;
                        }
                    });
            });
        if changed {
            self.state.settings.font_size = self.font_size;
            self.state.settings.save();
            self.state.status = format!("Preferences saved ({})", crate::recent::SETTINGS_REL);
        }
        if !open {
            self.show_preferences = false;
        }
    }

    fn log_tail_prompt_window(&mut self, ctx: &egui::Context) {
        if !self.state.pending_log_tail_prompt {
            return;
        }
        let name = self
            .state
            .tabs
            .active()
            .path
            .as_ref()
            .map(|p| crate::recent::short_path_label(p))
            .unwrap_or_else(|| "*.log".into());
        let mut enable = false;
        let mut skip = false;
        let mut open = true;
        egui::Window::new("Follow this log?")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(360.0)
            .show(ctx, |ui| {
                ui.label(format!("You opened {name}."));
                ui.label("Enable Monitoring (tail -f) now?");
                ui.add_space(6.0);
                ui.checkbox(
                    &mut self.log_tail_remember,
                    "Remember for future *.log files",
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Enable tail").clicked() {
                        enable = true;
                    }
                    if ui.button("Not now").clicked() {
                        skip = true;
                    }
                });
                if ui
                    .small_button("Reset remembered preference to ask")
                    .on_hover_text(crate::recent::SETTINGS_REL)
                    .clicked()
                {
                    self.state.reset_log_tail_preference();
                }
            });
        if enable {
            self.state
                .resolve_log_tail_prompt(true, self.log_tail_remember);
            self.follow_caret = true;
        } else if skip {
            self.state
                .resolve_log_tail_prompt(false, self.log_tail_remember);
        } else if !open {
            // Window closed: dismiss once; do not persist.
            self.state.pending_log_tail_prompt = false;
        }
    }

    fn encoding_notice_window(&mut self, ctx: &egui::Context) {
        let Some(msg) = self.state.pending_encoding_notice.clone() else {
            return;
        };
        let mut dismiss = false;
        let mut open = true;
        egui::Window::new("Encoding notice")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(420.0)
            .show(ctx, |ui| {
                ui.label(&msg);
                ui.add_space(8.0);
                if ui.button("OK").clicked() {
                    dismiss = true;
                }
            });
        if dismiss || !open {
            self.state.pending_encoding_notice = None;
        }
    }

    fn unsaved_close_window(&mut self, ctx: &egui::Context) {
        if self.state.pending_close.is_none() {
            return;
        }
        let title = self.state.close_tab_title();
        let mut save = false;
        let mut discard = false;
        let mut cancel = false;
        let mut open = true;
        egui::Window::new("Save changes?")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(380.0)
            .show(ctx, |ui| {
                ui.label(format!("Save changes to \"{title}\"?"));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        save = true;
                    }
                    if ui.button("Don't Save").clicked() {
                        discard = true;
                    }
                    if ui.button("Cancel").clicked() {
                        cancel = true;
                    }
                });
            });
        if !open {
            cancel = true;
        }
        if save {
            if !self.state.confirm_close_save() {
                return;
            }
        } else if discard {
            self.state.confirm_close_discard();
        } else if cancel {
            self.state.confirm_close_cancel();
            return;
        } else {
            return;
        }
        if self.state.take_want_quit() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        self.apply_closed_tab_indices();
        self.scroll_line = 0.0;
    }

    fn unsaved_reload_window(&mut self, ctx: &egui::Context) {
        if self.state.pending_reload.is_none() {
            return;
        }
        let title = self.state.tabs.active().title.clone();
        let mut save = false;
        let mut discard = false;
        let mut cancel = false;
        let mut open = true;
        egui::Window::new("Reload from disk?")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.label(format!(
                    "\"{title}\" has unsaved changes. Reload and lose them?"
                ));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        save = true;
                    }
                    if ui.button("Don't Save").clicked() {
                        discard = true;
                    }
                    if ui.button("Cancel").clicked() {
                        cancel = true;
                    }
                });
            });
        if !open {
            cancel = true;
        }
        if save {
            let _ = self.state.confirm_reload_save();
        } else if discard {
            self.state.confirm_reload_discard();
            self.scroll_line = 0.0;
        } else if cancel {
            self.state.confirm_reload_cancel();
        }
    }

    fn coming_soon_window(&mut self, ctx: &egui::Context) {
        let Some(info) = self.coming_soon.clone() else {
            return;
        };
        let blurb = crate::commands::coming_soon_blurb(&info.cmd);
        let mut open = true;
        egui::Window::new("Coming soon")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(380.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(6.0);
                    ui.heading(
                        RichText::new("Not ready yet")
                            .size(22.0)
                            .color(Color32::from_rgb(255, 196, 120)),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("“{}”", info.feature))
                            .strong()
                            .size(16.0),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(blurb)
                            .italics()
                            .color(Color32::from_rgb(200, 200, 210)),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(
                            "We’re building npp-rs in the background — one honest menu at a time.",
                        )
                        .small()
                        .color(Color32::from_rgb(150, 150, 160)),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("Come back tomorrow. Bring a smile; we’ll bring a feature.")
                            .small()
                            .strong()
                            .color(Color32::from_rgb(140, 200, 160)),
                    );
                    ui.add_space(12.0);
                    if ui.button("Alright, see you tomorrow").clicked() {
                        self.coming_soon = None;
                    }
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(info.cmd)
                            .small()
                            .monospace()
                            .color(Color32::from_rgb(110, 110, 120)),
                    );
                });
            });
        if !open {
            self.coming_soon = None;
        }
    }

    fn goto_line_window(&mut self, ctx: &egui::Context) {
        if !self.show_goto_line {
            return;
        }
        let mut open = true;
        let mut go = false;
        let mut cancel = false;
        egui::Window::new("Go to line")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Line number (1-based):");
                let resp = ui.text_edit_singleline(&mut self.goto_line_input);
                if resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                    go = true;
                }
                ui.horizontal(|ui| {
                    if ui.button("Go").clicked() {
                        go = true;
                    }
                    if ui.button("Cancel").clicked() {
                        cancel = true;
                    }
                });
            });
        if go {
            if let Ok(n) = self.goto_line_input.trim().parse::<usize>() {
                if n >= 1 {
                    let line = (n - 1).min(
                        self.state
                            .tabs
                            .active()
                            .buffer
                            .line_count()
                            .saturating_sub(1),
                    );
                    let at = self.state.tabs.active().buffer.line_to_char(line);
                    self.state.tabs.active_mut().buffer.set_caret(at);
                    self.follow_caret = true;
                    self.state.status = format!("Go to line {n}");
                }
            }
            self.show_goto_line = false;
        }
        if cancel || !open {
            self.show_goto_line = false;
        }
    }

    fn summary_window(&mut self, ctx: &egui::Context) {
        if !self.show_summary {
            return;
        }
        let doc = self.state.tabs.active();
        let text = doc.buffer.to_string();
        let lines = doc.buffer.line_count();
        let chars = doc.buffer.len_chars();
        let bytes = text.len();
        let words = text.split_whitespace().count();
        let path = doc
            .path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "(untitled)".into());
        let title = doc.title.clone();
        let language = doc.language.clone();
        let dirty = doc.dirty;
        let read_only = doc.read_only;
        let marks = doc.bookmarks.len();
        let (chg_u, chg_s) = doc.change_history_counts();
        let mut open = self.show_summary;
        let mut close = false;
        egui::Window::new("Summary")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(format!("Title: {title}"));
                ui.label(format!("Path: {path}"));
                ui.label(format!("Language: {language}"));
                ui.label(format!("Lines: {lines}"));
                ui.label(format!("Words: {words}"));
                ui.label(format!("Characters: {chars}"));
                ui.label(format!("Bytes: {bytes}"));
                ui.label(format!("Dirty: {dirty}"));
                ui.label(format!("Read-only: {read_only}"));
                ui.label(format!("Bookmarks: {marks}"));
                ui.label(format!("Change history: {chg_u} unsaved, {chg_s} saved"));
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        self.show_summary = open && !close;
    }

    fn apply_theme_visuals(&mut self, ctx: &egui::Context) {
        let theme = crate::theme::resolve_theme(&self.state.settings.theme_id);
        ctx.set_visuals(theme.visuals());
    }

    fn current_theme(&self) -> crate::theme::AppliedTheme {
        crate::theme::resolve_theme(&self.state.settings.theme_id)
    }

    fn project_panel_window(&mut self, ctx: &egui::Context) {
        if !self.show_project_panel {
            return;
        }
        let mut open = self.show_project_panel;
        let mut open_path: Option<PathBuf> = None;
        let mut enter_dir: Option<PathBuf> = None;
        let mut reveal_path: Option<PathBuf> = None;
        let mut pick_folder = false;
        let mut close = false;
        let mut persist_filter = false;
        let mut refresh = false;
        let root = self.state.workspace_root.clone();
        let filter = self.state.settings.project_filter.clone();
        let cache_key = (root.clone(), filter.clone());
        if self.project_panel_cache_key.as_ref() != Some(&cache_key) {
            self.project_panel_entries = EditorState::list_project_panel_entries(&root, &filter);
            self.project_panel_cache_key = Some(cache_key);
        }
        let entries = self.project_panel_entries.clone();
        egui::Window::new("Project")
            .open(&mut open)
            .default_width(320.0)
            .default_height(420.0)
            .show(ctx, |ui| {
                ui.label(RichText::new(root.display().to_string()).small().weak());
                ui.horizontal(|ui| {
                    if ui.button("Pick folder…").clicked() {
                        pick_folder = true;
                    }
                    if ui
                        .small_button("Up")
                        .on_hover_text("Parent folder")
                        .clicked()
                    {
                        if let Some(parent) = root.parent() {
                            enter_dir = Some(parent.to_path_buf());
                        }
                    }
                    if ui
                        .small_button("Refresh")
                        .on_hover_text("Reload folder listing")
                        .clicked()
                    {
                        refresh = true;
                    }
                    if ui
                        .small_button("Reveal")
                        .on_hover_text("Open this folder in the file manager")
                        .clicked()
                    {
                        reveal_path = Some(root.clone());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Filter");
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.state.settings.project_filter)
                                .desired_width(180.0)
                                .hint_text("name contains…"),
                        )
                        .changed()
                    {
                        persist_filter = true;
                    }
                });
                ui.separator();
                egui::ScrollArea::vertical()
                    .max_height(340.0)
                    .show(ui, |ui| {
                        if entries.is_empty() {
                            ui.label(if filter.is_empty() {
                                "(empty folder)"
                            } else {
                                "(no matches)"
                            });
                        }
                        for p in &entries {
                            let is_dir = p.is_dir();
                            let name = p
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_else(|| p.display().to_string());
                            let label = if is_dir { format!("{name}/") } else { name };
                            let response = ui.selectable_label(false, label);
                            if response.clicked() {
                                if is_dir {
                                    enter_dir = Some(p.clone());
                                } else {
                                    open_path = Some(p.clone());
                                }
                            }
                            response.context_menu(|ui| {
                                if !is_dir && ui.button("Open").clicked() {
                                    open_path = Some(p.clone());
                                    ui.close_menu();
                                }
                                if is_dir && ui.button("Enter folder").clicked() {
                                    enter_dir = Some(p.clone());
                                    ui.close_menu();
                                }
                                if ui.button("Reveal in file manager").clicked() {
                                    reveal_path = Some(p.clone());
                                    ui.close_menu();
                                }
                            });
                        }
                    });
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        if refresh {
            self.project_panel_cache_key = None;
            self.project_panel_entries = EditorState::list_project_panel_entries(&root, &filter);
            self.project_panel_cache_key = Some((root.clone(), filter));
            self.state.status = "Project panel refreshed".into();
        }
        if pick_folder {
            self.state.pick_workspace_folder();
            self.project_panel_cache_key = None;
        }
        if let Some(dir) = enter_dir {
            self.state.workspace_root = dir;
            self.state.persist_workspace_root();
            self.project_panel_cache_key = None;
        }
        if persist_filter {
            self.state.settings.save();
            self.project_panel_cache_key = None;
        }
        if let Some(p) = reveal_path {
            self.state.reveal_path_in_os(&p);
        }
        if let Some(p) = open_path {
            self.state.open_path(p);
            self.scroll_line = 0.0;
        }
        self.show_project_panel = open && !close;
    }

    fn lossy_ansi_confirm_window(&mut self, ctx: &egui::Context) {
        let Some(path) = self.state.pending_lossy_ansi.clone() else {
            return;
        };
        let unmapped =
            ::fs::count_windows_1252_unmapped(&self.state.tabs.active().buffer.to_string());
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        let mut save = false;
        let mut cancel = false;
        egui::Window::new("ANSI save warning")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(format!(
                    "Saving “{name}” as Windows-1252 will turn {unmapped} character(s) into '?'."
                ));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Save anyway").clicked() {
                        save = true;
                    }
                    if ui.button("Cancel").clicked() {
                        cancel = true;
                    }
                });
            });
        if save {
            self.state.confirm_lossy_ansi_save();
        }
        if cancel {
            self.state.cancel_lossy_ansi_save();
        }
    }

    fn theme_picker_window(&mut self, ctx: &egui::Context) {
        if !self.show_theme_picker {
            return;
        }
        let mut open = self.show_theme_picker;
        let mut applied: Option<String> = None;
        let mut close = false;
        egui::Window::new("Themes")
            .open(&mut open)
            .default_width(420.0)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(
                        "Apply: egui visuals, editor chrome, selection/caret, and syntax tokens from themes/*.json or a Notepad++ XML subset (GlobalStyles + preferred lexer).",
                    )
                    .small()
                    .weak(),
                );
                ui.separator();
                let cur = self.state.settings.theme_id.clone();
                for (id, label) in crate::theme::list_theme_choices() {
                    if ui.selectable_label(cur == id, label).clicked() {
                        applied = Some(id);
                    }
                }
                ui.separator();
                if ui.button("Open themes/ folder").clicked() {
                    let dir = crate::theme::ensure_themes_dir();
                    crate::commands::common::open_path_in_os(&mut self.state, &dir);
                }
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        if let Some(id) = applied {
            self.state.settings.theme_id = id.clone();
            self.state.settings.save();
            let t = crate::theme::resolve_theme(&id);
            self.state.status = format!("Theme applied: {} ({})", t.label, id);
            ctx.set_visuals(t.visuals());
        }
        self.show_theme_picker = open && !close;
    }

    fn doc_list_window(&mut self, ctx: &egui::Context) {
        if !self.show_doc_list {
            return;
        }
        let mut open = self.show_doc_list;
        let mut switch_to = None;
        let mut close = false;
        egui::Window::new("Document List")
            .open(&mut open)
            .default_width(420.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(320.0)
                    .show(ui, |ui| {
                        for i in 0..self.state.tabs.len() {
                            let Some(doc) = self.state.tabs.get(i) else {
                                continue;
                            };
                            let mut label = doc.title.clone();
                            if doc.dirty {
                                label.push('*');
                            }
                            if ui
                                .selectable_label(i == self.state.tabs.active_index(), label)
                                .clicked()
                            {
                                switch_to = Some(i);
                            }
                        }
                    });
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        if let Some(i) = switch_to {
            self.state.switch_tab(i);
            close = true;
        }
        self.show_doc_list = open && !close;
    }

    fn doc_map_window(&mut self, ctx: &egui::Context) {
        if !self.show_doc_map {
            return;
        }
        let line_count = self.state.tabs.active().buffer.line_count().max(1);
        let max_scroll = (line_count.saturating_sub(1) as f32).max(0.0);
        let mut open = self.show_doc_map;
        let mut jump_line: Option<f32> = None;
        egui::Window::new("Document Map")
            .open(&mut open)
            .default_width(72.0)
            .default_height(360.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(format!("{line_count} lines"));
                let (resp, painter) = ui.allocate_painter(
                    Vec2::new(
                        ui.available_width().max(40.0),
                        ui.available_height().max(120.0),
                    ),
                    Sense::click_and_drag(),
                );
                let rect = resp.rect;
                painter.rect_filled(rect, 0.0, Color32::from_rgb(28, 28, 32));
                let sample_n = 128.min(line_count);
                for i in 0..sample_n {
                    let line_idx = i * line_count / sample_n;
                    let raw = self.state.tabs.active().buffer.line(line_idx);
                    let dens = (raw.trim_end_matches(['\n', '\r']).chars().count() as f32 / 80.0)
                        .clamp(0.05, 1.0);
                    let y0 = rect.top() + (i as f32 / sample_n as f32) * rect.height();
                    let y1 = rect.top() + ((i + 1) as f32 / sample_n as f32) * rect.height();
                    let g = (40.0 + dens * 140.0) as u8;
                    painter.rect_filled(
                        Rect::from_min_max(Pos2::new(rect.left(), y0), Pos2::new(rect.right(), y1)),
                        0.0,
                        Color32::from_rgb(g, g, g.saturating_add(8)),
                    );
                }
                if max_scroll > 0.0 {
                    let frac = (self.scroll_line / max_scroll).clamp(0.0, 1.0);
                    let mark_h = (rect.height() * 0.08).max(6.0);
                    let mark_y = rect.top() + frac * (rect.height() - mark_h);
                    painter.rect_stroke(
                        Rect::from_min_size(
                            Pos2::new(rect.left() + 1.0, mark_y),
                            Vec2::new(rect.width() - 2.0, mark_h),
                        ),
                        0.0,
                        egui::Stroke::new(1.5_f32, Color32::from_rgb(80, 180, 140)),
                        egui::StrokeKind::Outside,
                    );
                }
                if let Some(pos) = resp.interact_pointer_pos() {
                    if resp.clicked() || resp.dragged() {
                        let t = ((pos.y - rect.top()) / rect.height()).clamp(0.0, 1.0);
                        jump_line = Some(t * max_scroll);
                    }
                }
            });
        if let Some(line) = jump_line {
            self.scroll_line = line;
            self.follow_caret = false;
            self.state.status = format!("Document Map → line {}", line as usize + 1);
        }
        self.show_doc_map = open;
    }

    fn func_list_window(&mut self, ctx: &egui::Context) {
        if !self.show_func_list {
            return;
        }
        let entries = collect_func_like_lines(&self.state.tabs.active().buffer);
        let mut open = self.show_func_list;
        let mut jump: Option<usize> = None;
        egui::Window::new("Function List")
            .open(&mut open)
            .default_width(360.0)
            .default_height(320.0)
            .show(ctx, |ui| {
                if entries.is_empty() {
                    ui.label("No fn/class-like lines found.");
                } else {
                    egui::ScrollArea::vertical()
                        .max_height(280.0)
                        .show(ui, |ui| {
                            for (line_idx, preview) in &entries {
                                let label = format!("{}: {preview}", line_idx + 1);
                                if ui.selectable_label(false, label).clicked() {
                                    jump = Some(*line_idx);
                                }
                            }
                        });
                }
            });
        if let Some(line) = jump {
            let at = self.state.tabs.active().buffer.line_to_char(line);
            self.state.tabs.active_mut().buffer.set_caret(at);
            self.follow_caret = true;
            self.state.status = format!("Function List → line {}", line + 1);
        }
        self.show_func_list = open;
    }

    fn char_panel_window(&mut self, ctx: &egui::Context) {
        if !self.show_char_panel {
            return;
        }
        let mut open = self.show_char_panel;
        let mut insert: Option<char> = None;
        const CHARS: &[char] = &[
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{',
            '}', '\\', '|', ';', ':', '\'', '"', ',', '.', '<', '>', '/', '?', '~', '`', '©', '®',
            '™', '€', '£', '¥', '§', '¶', '°', '±', '×', '÷', '½', '¼', '¾', '…', '–', '—', '‘',
            '’', '“', '”', '«', '»', '•', '†', '‡', 'α', 'β', 'γ', 'δ', 'π', 'μ', 'Ω', '←', '→',
            '↑', '↓', '✓', '✗', '★', '☆', '♠', '♣', '♥', '♦', '☺', '☻', '♪', '♫', '∞', '≈', '≠',
            '≤', '≥', '√', '∑', '∏', '∫', '∂', '∆', '∈', '∉', '∩', '∪', '⊂',
        ];
        egui::Window::new("Character Panel")
            .open(&mut open)
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.label("Click a character to insert at the caret.");
                egui::ScrollArea::vertical()
                    .max_height(240.0)
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for &c in CHARS {
                                if ui
                                    .add_sized([28.0, 28.0], egui::Button::new(c.to_string()))
                                    .clicked()
                                {
                                    insert = Some(c);
                                }
                            }
                        });
                    });
            });
        if let Some(c) = insert {
            if self.state.tabs.active().read_only {
                self.state.status = "Document is read-only".into();
            } else {
                let s = c.to_string();
                self.state.prepare_edit();
                self.state.tabs.active_mut().buffer.insert(&s);
                self.state.mark_text_changed();
                self.follow_caret = true;
                self.state.status = format!("Inserted U+{:04X}", c as u32);
            }
        }
        self.show_char_panel = open;
    }

    fn tab_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let count = self.state.tabs.len();
                let mut switch_to = None;
                let mut close_idx = None;
                let mut toggle_pin = None;
                let mut pending_move: Option<(usize, usize)> = None;
                let mut start_compare_now = false;
                for i in 0..count {
                    let Some(doc) = self.state.tabs.get(i) else {
                        continue;
                    };
                    let pinned = doc.pinned;
                    let mut label = String::new();
                    if pinned {
                        label.push_str("[P] ");
                    }
                    label.push_str(&doc.title);
                    if doc.dirty {
                        label.push('*');
                    }
                    if doc.tail_follow {
                        label.push_str(" [tail]");
                    }
                    if doc.loading {
                        label.push_str(" …");
                    }
                    let selected = i == self.state.tabs.active_index();
                    let is_partner = self.compare_partner_tab == Some(i) && !selected;
                    if is_partner {
                        label.push_str(" ⇄");
                    }
                    let colour = match doc.tab_colour {
                        Some(1) => Some(Color32::from_rgb(180, 70, 70)),
                        Some(2) => Some(Color32::from_rgb(70, 140, 80)),
                        Some(3) => Some(Color32::from_rgb(70, 110, 180)),
                        Some(4) => Some(Color32::from_rgb(160, 120, 40)),
                        Some(5) => Some(Color32::from_rgb(140, 70, 160)),
                        _ => None,
                    };
                    let text = if let Some(c) = colour {
                        RichText::new(&label).color(c)
                    } else if is_partner {
                        RichText::new(&label).color(Color32::from_rgb(40, 120, 140))
                    } else {
                        RichText::new(&label)
                    };
                    let resp = ui.add(
                        egui::Button::new(text)
                            .selected(selected)
                            .sense(Sense::click_and_drag()),
                    );
                    if resp.drag_started() {
                        self.tab_drag_from = Some(i);
                        switch_to = Some(i);
                    }
                    if resp.dragged() {
                        ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
                    }
                    if self.tab_drag_from.is_some() && resp.hovered() {
                        if let Some(from) = self.tab_drag_from {
                            if from != i {
                                pending_move = Some((from, i));
                            }
                        }
                    }
                    if resp.clicked() {
                        let mod_pick = ui.input(|inp| inp.modifiers.command || inp.modifiers.ctrl);
                        if mod_pick && i != self.state.tabs.active_index() {
                            if self.compare_partner_tab == Some(i) {
                                self.compare_partner_tab = None;
                                self.state.status = "Compare partner cleared".into();
                            } else {
                                self.compare_partner_tab = Some(i);
                                self.state.status = format!(
                                    "Compare partner: “{}” — View → Compare with Other View",
                                    doc.title
                                );
                            }
                        } else {
                            switch_to = Some(i);
                        }
                    }
                    if resp.middle_clicked() {
                        close_idx = Some(i);
                    }
                    resp.context_menu(|ui| {
                        let pin_label = if pinned { "Unpin tab" } else { "Pin tab" };
                        if ui.button(pin_label).clicked() {
                            toggle_pin = Some(i);
                            ui.close_menu();
                        }
                        if i != self.state.tabs.active_index() {
                            let mark_label = if self.compare_partner_tab == Some(i) {
                                "Clear compare partner"
                            } else {
                                "Mark for compare"
                            };
                            if ui.button(mark_label).clicked() {
                                if self.compare_partner_tab == Some(i) {
                                    self.compare_partner_tab = None;
                                    self.state.status = "Compare partner cleared".into();
                                } else {
                                    self.compare_partner_tab = Some(i);
                                    self.state.status = format!(
                                        "Compare partner: “{}” — View → Compare with Other View",
                                        doc.title
                                    );
                                }
                                ui.close_menu();
                            }
                            if ui.button("Compare with this tab").clicked() {
                                self.compare_partner_tab = Some(i);
                                start_compare_now = true;
                                ui.close_menu();
                            }
                        }
                    });
                    ui.push_id(("pin_tab", i), |ui| {
                        let pin_btn = if pinned { "P" } else { "·" };
                        if ui
                            .small_button(pin_btn)
                            .on_hover_text(if pinned { "Unpin tab" } else { "Pin tab" })
                            .clicked()
                        {
                            toggle_pin = Some(i);
                        }
                    });
                    ui.push_id(("close_tab", i), |ui| {
                        if ui.small_button("×").on_hover_text("Close tab").clicked() {
                            close_idx = Some(i);
                        }
                    });
                }
                if let Some((from, to)) = pending_move {
                    if self.state.tabs.move_tab(from, to) {
                        self.remap_tab_indices(from, to);
                        self.tab_drag_from = Some(to);
                        self.state.status = "Tab moved".into();
                    }
                }
                if !ui.input(|i| i.pointer.any_down()) {
                    self.tab_drag_from = None;
                }
                if ui.button("+").clicked() {
                    self.state.new_file();
                }
                if self.dual_view {
                    ui.separator();
                    let other_title = self
                        .state
                        .tabs
                        .get(self.other_view_tab)
                        .map(|d| d.title.clone())
                        .unwrap_or_else(|| "?".into());
                    ui.label(
                        RichText::new(format!("| other: {other_title}"))
                            .small()
                            .weak(),
                    );
                    if ui
                        .small_button("×dual")
                        .on_hover_text("Close dual view")
                        .clicked()
                    {
                        self.dual_view = false;
                        self.focused_pane = EditorPane::Primary;
                        self.state.status = "Dual view closed".into();
                    }
                }
                if let Some(i) = toggle_pin {
                    if let Some(doc) = self.state.tabs.get_mut(i) {
                        doc.pinned = !doc.pinned;
                        let name = doc.title.clone();
                        let on = doc.pinned;
                        self.state.status = if on {
                            format!("Pinned “{name}”")
                        } else {
                            format!("Unpinned “{name}”")
                        };
                    }
                }
                if let Some(i) = switch_to {
                    self.state.tabs.set_active(i);
                    self.state.highlight_dirty = true;
                    self.scroll_line = 0.0;
                }
                if let Some(i) = close_idx {
                    self.state.request_close_tab(i);
                    self.scroll_line = 0.0;
                }
                if start_compare_now {
                    self.start_compare();
                }
            });
        });
    }

    fn remap_tab_indices(&mut self, from: usize, to: usize) {
        use doc::TabSet;
        self.other_view_tab = TabSet::remap_index(self.other_view_tab, from, to);
        self.compare_left_tab = TabSet::remap_index(self.compare_left_tab, from, to);
        self.compare_right_tab = TabSet::remap_index(self.compare_right_tab, from, to);
        if let Some(p) = self.compare_partner_tab {
            self.compare_partner_tab = Some(TabSet::remap_index(p, from, to));
        }
    }

    fn seed_find_from_selection(&mut self) {
        if let Some((s, e)) = self.state.tabs.active().buffer.selection() {
            let sel = self.state.tabs.active().buffer.slice(s, e);
            if !sel.is_empty() && !sel.contains('\n') && sel.chars().count() <= 200 {
                self.state.find_query = sel;
            }
        }
    }

    fn find_replace_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("find").show(ctx, |ui| {
            let mut persist = false;
            let mut run_find_files = false;
            ui.horizontal(|ui| {
                ui.label("Find:");
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.state.find_query)
                        .desired_width(180.0)
                        .hint_text("search text"),
                );
                if resp.changed() {
                    persist = true;
                }
                let enter = ui.input(|i| i.key_pressed(Key::Enter));
                if resp.has_focus() && enter {
                    self.state.find_next();
                    self.follow_caret = true;
                }
                if ui.button("Next").clicked() {
                    self.state.find_next();
                    self.follow_caret = true;
                }
                if ui.button("Prev").clicked() {
                    self.state.find_prev();
                    self.follow_caret = true;
                }
                if ui
                    .checkbox(&mut self.state.settings.find_match_case, "Case")
                    .changed()
                {
                    persist = true;
                }
                if ui
                    .checkbox(&mut self.state.settings.find_whole_word, "Word")
                    .changed()
                {
                    persist = true;
                }
                if ui
                    .checkbox(&mut self.state.settings.find_in_selection, "Sel")
                    .on_hover_text(
                        "Find and replace only in a captured range (auto on for multi-line selection)",
                    )
                    .changed()
                {
                    persist = true;
                    if self.state.settings.find_in_selection {
                        self.state.capture_find_scope();
                    } else {
                        self.state.find_scope = None;
                    }
                }
                if ui
                    .checkbox(&mut self.state.settings.find_wrap, "Wrap")
                    .on_hover_text("After the last match, Next starts again at the first")
                    .changed()
                {
                    persist = true;
                }
                if ui
                    .checkbox(&mut self.state.settings.find_regex, "Re")
                    .on_hover_text(
                        "Treat the query as a regular expression (linear-time). Replace: $n or \\n for groups",
                    )
                    .changed()
                {
                    persist = true;
                }
                let n = self.state.find_match_count();
                if self.state.find_query.is_empty() {
                    ui.label(RichText::new("0 matches").weak());
                } else {
                    ui.label(format!("{n} match{}", if n == 1 { "" } else { "es" }));
                }
                if self.show_replace {
                    ui.separator();
                    ui.label("Replace:");
                    let rresp = ui
                        .add(egui::TextEdit::singleline(&mut self.replace_with).desired_width(120.0))
                        .on_hover_text(
                            "With Re: $1 / \\1 insert a capture, $0 / $& the match, $$ a dollar",
                        );
                    if rresp.changed() {
                        persist = true;
                    }
                    if ui.button("Replace").clicked() {
                        let r = self.replace_with.clone();
                        self.state.replace_next(&r);
                        self.follow_caret = true;
                    }
                    if ui.button("Replace All").clicked() {
                        let r = self.replace_with.clone();
                        self.state.replace_all(&r);
                    }
                } else if ui.button("Replace…").clicked() {
                    self.show_replace = true;
                }
                if ui.button("Close").clicked() {
                    persist = true;
                    self.state.find_open = false;
                    self.show_replace = false;
                }
                if self.find_focus_once {
                    resp.request_focus();
                    self.find_focus_once = false;
                }
            });
            ui.horizontal(|ui| {
                ui.label("In files:");
                let inc = ui.add(
                    egui::TextEdit::singleline(&mut self.state.settings.find_files_include)
                        .desired_width(140.0)
                        .hint_text("*.rs,*.md (empty=all)"),
                );
                if inc.changed() {
                    persist = true;
                }
                ui.label("Exclude:");
                let exc = ui.add(
                    egui::TextEdit::singleline(&mut self.state.settings.find_files_exclude)
                        .desired_width(160.0)
                        .hint_text("target,node_modules"),
                );
                if exc.changed() {
                    persist = true;
                }
                if ui
                    .button("Find in Files")
                    .on_hover_text("Search workspace root recursively")
                    .clicked()
                {
                    persist = true;
                    run_find_files = true;
                }
            });
            if persist {
                self.state.settings.find_query = self.state.find_query.clone();
                self.state.settings.replace_with = self.replace_with.clone();
                self.state.settings.save();
            }
            if run_find_files {
                let mut flags = crate::commands::UiFlags {
                    find_open: self.state.find_open,
                    show_replace: self.show_replace,
                    find_focus_once: self.find_focus_once,
                    follow_caret: self.follow_caret,
                    ..Default::default()
                };
                let _ = self.dispatch_menu_cmd("IDM_SEARCH_FINDINFILES", &mut flags);
                self.state.find_open = flags.find_open;
                self.show_replace = flags.show_replace;
                self.find_focus_once = flags.find_focus_once;
                if flags.follow_caret {
                    self.follow_focused_caret();
                }
            }
        });
    }

    fn status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            let (lang, line, col, chars, on, status, chg_u, chg_s) = {
                let doc = self.state.tabs.active();
                let caret = doc.buffer.caret();
                let line = doc.buffer.char_to_line(caret);
                let col = caret - doc.buffer.line_to_char(line) + 1;
                let (chg_u, chg_s) = doc.change_history_counts();
                (
                    doc.language.clone(),
                    line + 1,
                    col,
                    doc.buffer.len_chars(),
                    doc.tail_follow,
                    self.state.status.clone(),
                    chg_u,
                    chg_s,
                )
            };
            ui.horizontal(|ui| {
                ui.label(&status);
                if crate::commands::edit::text_is_rtl() {
                    ui.separator();
                    ui.label(
                        RichText::new("RTL")
                            .strong()
                            .color(Color32::from_rgb(42, 148, 118)),
                    );
                }

                ui.separator();
                // Clickable tail toggle (same as View → Monitoring).
                let tail_text = if on {
                    RichText::new("TAIL").strong().color(MENU_READY)
                } else {
                    RichText::new("tail").weak()
                };
                if ui
                    .add(egui::Button::new(tail_text).frame(false))
                    .on_hover_text(if on {
                        "Tail ON — click to stop following the file"
                    } else {
                        "Click to tail this file (Monitoring / ⌘⇧T)"
                    })
                    .clicked()
                    && self.state.toggle_tail_follow()
                {
                    self.follow_caret = true;
                }
                if chg_u + chg_s > 0 {
                    ui.separator();
                    let label = format!("CHG {chg_u}/{chg_s}");
                    let color = if chg_u > 0 {
                        Color32::from_rgb(210, 140, 40)
                    } else {
                        Color32::from_rgb(70, 160, 90)
                    };
                    ui.label(RichText::new(label).color(color)).on_hover_text(
                        "Change history: unsaved (amber) / saved (green) line marks",
                    );
                }
                if self.state.settings.status_show_lang {
                    ui.separator();
                    ui.label(format!("Lang: {lang}"));
                }
                ui.separator();
                ui.label(format!("Ln {line}, Col {col}"));
                if self.state.settings.status_show_chars {
                    ui.separator();
                    ui.label(format!("{chars} chars"));
                }

                // Bottom-right: package version + git commit → GitHub.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let short = env!("NPP_GIT_HASH");
                    let full = env!("NPP_GIT_HASH_FULL");
                    let ver = env!("CARGO_PKG_VERSION");
                    let label = format!("v{ver} · {short}");
                    let url = format!("https://github.com/raro42/npp-rust/commit/{full}");
                    ui.hyperlink_to(RichText::new(label).small().weak(), url)
                        .on_hover_text("Open this build’s commit on GitHub (raro42/npp-rust)");
                });
            });
        });
    }

    fn editor_pane(&mut self, ctx: &egui::Context) {
        self.state
            .refresh_highlight_if_needed(self.scroll_line.floor() as usize);
        self.apply_closed_tab_indices();
        self.clamp_other_view_tab();
        self.sync_compare_panes();

        if self.dual_view {
            let title = self
                .state
                .tabs
                .get(self.other_view_tab)
                .map(|d| d.title.clone())
                .unwrap_or_else(|| "Other view".into());
            egui::SidePanel::right("dual_other_view")
                .resizable(true)
                .default_width(420.0)
                .min_width(180.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Other view: {title}")).strong());
                        if ui
                            .small_button("Switch")
                            .on_hover_text("Swap with active tab")
                            .clicked()
                        {
                            self.switch_other_view_now();
                        }
                    });
                    ui.separator();
                    self.paint_secondary_pane(ui);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.state.tabs.active().loading {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("Loading file…").size(18.0));
                });
                return;
            }

            let font_id = FontId::monospace(self.font_size);
            let row_height = ui.fonts(|f| f.row_height(&font_id)) + 2.0;
            let buf_line_count = self.state.tabs.active().buffer.line_count().max(1);
            let fold_hidden = self.state.tabs.active().hidden_lines.clone();
            let visible_lines = visible_lines_with_compare_hide(
                buf_line_count,
                &fold_hidden,
                self.compare_on,
                self.state.settings.compare_hide_equal,
                &self.compare_left_tags,
                &self.compare_hide_revealed_left,
                self.state.settings.compare_hide_equal_context_lines(),
            );
            let display_count = visible_lines.len().max(1);
            let avail = ui.available_size();
            let (rect, response) = ui.allocate_exact_size(avail, Sense::click_and_drag());
            // Do not steal focus every frame — that breaks Find (Ctrl/Cmd+F).
            if !self.state.find_open && !self.show_replace {
                if response.clicked() || response.drag_started() {
                    response.request_focus();
                    self.focused_pane = EditorPane::Primary;
                }
            } else if response.clicked() {
                // Clicking the editor closes find focus but keeps the bar open.
                response.request_focus();
                self.focused_pane = EditorPane::Primary;
            }

            if self.state.reset_view {
                self.scroll_line = 0.0;
                self.follow_caret = false;
                self.state.reset_view = false;
            }
            let visible_rows = {
                // One extra row of air above the status bar — last line must stay fully visible.
                let usable = (rect.height() - row_height).max(row_height);
                ((usable / row_height).floor() as usize).max(1)
            };
            let max_scroll = (display_count.saturating_sub(visible_rows) as f32).max(0.0);

            // Mouse-wheel scroll must not be overridden by caret-follow.
            // In dual view, only scroll this pane when the pointer is over it.
            // Cmd/Ctrl+wheel zooms (handled in handle_shortcuts); skip scroll then.
            let scroll = if !self.dual_view || response.hovered() {
                ui.input(|i| {
                    if i.modifiers.command || i.modifiers.ctrl {
                        0.0
                    } else {
                        i.raw_scroll_delta.y
                    }
                })
            } else {
                0.0
            };
            if scroll != 0.0 {
                self.follow_caret = false;
                self.scroll_line = (self.scroll_line - scroll / row_height).clamp(0.0, max_scroll);
                if self.dual_view && (self.sync_scroll_v || self.sync_scroll_h) {
                    self.scroll_line_other = self.scroll_line;
                }
            }

            let show_ln = self.state.settings.show_line_numbers;
            let show_fold = self.state.settings.show_fold_margin;
            let fold_w = if show_fold { FOLD_MARGIN_W } else { 0.0 };
            let gutter_w = if show_ln { 56.0 } else { 16.0 }
                + fold_w
                + f32::from(self.state.settings.gutter_extra);
            // Gap between line numbers and text (was flush before).
            let gutter_gap = 12.0;
            let text_left = rect.left() + gutter_w + gutter_gap;
            let gutter_right = rect.left() + gutter_w;
            let fold_left = gutter_right - fold_w;

            let hit_index = |ui: &egui::Ui,
                             pos: Pos2,
                             buf: &buffer::TextBuffer,
                             scroll: f32,
                             visible: &[usize]|
             -> usize {
                let first = scroll.floor() as usize;
                let row = first + ((pos.y - rect.top()) / row_height).floor().max(0.0) as usize;
                let row = row.min(visible.len().saturating_sub(1));
                let line = visible.get(row).copied().unwrap_or(0);
                let line_start = buf.line_to_char(line);
                let line_text = buf.line(line);
                let line_body = line_text.trim_end_matches(['\n', '\r']);
                let col = col_from_x(ui, &font_id, line_body, pos.x - text_left);
                line_start + col
            };

            let scroll_for_hit = self.scroll_line;
            let fold_line_at = |pos: Pos2| -> Option<usize> {
                if !show_fold || fold_w <= 0.0 || pos.x < fold_left || pos.x >= gutter_right {
                    return None;
                }
                let first = scroll_for_hit.floor() as usize;
                let row = first + ((pos.y - rect.top()) / row_height).floor().max(0.0) as usize;
                let row = row.min(visible_lines.len().saturating_sub(1));
                visible_lines.get(row).copied()
            };

            let mut fold_click = false;
            let mut hide_gap_click = false;
            if response.clicked() || response.drag_started() {
                if let Some(pos) = response.interact_pointer_pos() {
                    if self.try_expand_compare_hide_gap_at(
                        pos,
                        rect,
                        scroll_for_hit,
                        row_height,
                        &visible_lines,
                        true,
                    ) {
                        hide_gap_click = true;
                        self.drag_anchor = None;
                        self.rect_drag = false;
                        self.sel_text_drag = None;
                    } else if let Some(line) = fold_line_at(pos) {
                        let lang = self.state.tabs.active().language.clone();
                        let regions = crate::fold::compute_fold_regions(
                            lang.as_str(),
                            &self.state.tabs.active().buffer,
                        );
                        if let Some(region) = crate::fold::region_for_fold_action(&regions, line) {
                            let hidden = &mut self.state.tabs.active_mut().hidden_lines;
                            let was = crate::fold::is_folded(hidden, &region);
                            crate::fold::toggle_region(hidden, &region);
                            self.state.status = if was {
                                format!("Unfolded {} line(s)", region.end - region.header)
                            } else {
                                format!("Folded {} line(s)", region.end - region.header)
                            };
                            fold_click = true;
                            self.drag_anchor = None;
                            self.rect_drag = false;
                            self.sel_text_drag = None;
                        }
                    }
                }
            }

            // Double-click → word; triple-click → line; click → caret;
            // Alt+drag → rect/column multi-carets; drag inside selection → move/copy;
            // else drag → select.
            if fold_click || hide_gap_click {
                // Fold margin or hide-equal ···N cue consumed the pointer.
            } else if response.triple_clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    self.state.tabs.active_mut().clear_multi_sels();
                    self.state.tabs.active_mut().buffer.select_line_at(idx);
                    self.drag_anchor = None;
                    self.rect_drag = false;
                    self.sel_text_drag = None;
                    self.follow_caret = false;
                }
            } else if response.double_clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    self.state.tabs.active_mut().clear_multi_sels();
                    self.state.tabs.active_mut().buffer.select_word_at(idx);
                    self.drag_anchor = None;
                    self.rect_drag = false;
                    self.sel_text_drag = None;
                    self.follow_caret = false;
                }
            } else if response.drag_started() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    let (shift, alt) = ui.input(|i| (i.modifiers.shift, i.modifiers.alt));
                    let tab = self.state.tabs.active_index();
                    let doc = self.state.tabs.active();
                    let inside_sel = doc
                        .buffer
                        .selection()
                        .is_some_and(|(s, e)| idx >= s && idx < e);
                    let read_only = doc.read_only;
                    let sel_anchor = doc
                        .buffer
                        .selection()
                        .map(|(s, _)| s)
                        .unwrap_or_else(|| doc.buffer.caret());
                    if alt {
                        self.sel_text_drag = None;
                        self.rect_drag = true;
                        self.drag_anchor = Some(idx);
                        self.state.tabs.active_mut().set_rect_selection(idx, idx);
                        self.state.status = "Column select (Alt+drag)".into();
                    } else if !shift && inside_sel && !read_only {
                        self.state.tabs.active_mut().clear_multi_sels();
                        self.sel_text_drag = Some(SelTextDrag { tab, drop_at: idx });
                        self.drag_anchor = None;
                        self.rect_drag = false;
                    } else if shift {
                        self.state.tabs.active_mut().clear_multi_sels();
                        self.sel_text_drag = None;
                        self.rect_drag = false;
                        self.drag_anchor = Some(sel_anchor);
                        self.state
                            .tabs
                            .active_mut()
                            .buffer
                            .set_selection(sel_anchor, idx);
                    } else {
                        self.state.tabs.active_mut().clear_multi_sels();
                        self.sel_text_drag = None;
                        self.rect_drag = false;
                        self.drag_anchor = Some(idx);
                        self.state.tabs.active_mut().buffer.set_caret(idx);
                    }
                    self.follow_caret = false;
                }
            } else if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    if let Some(drag) = self.sel_text_drag.as_mut() {
                        if drag.tab == self.state.tabs.active_index() {
                            drag.drop_at = idx;
                            let copy = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);
                            ui.ctx().set_cursor_icon(if copy {
                                CursorIcon::Copy
                            } else {
                                CursorIcon::Grabbing
                            });
                        }
                    } else if let Some(anchor) = self.drag_anchor {
                        if self.rect_drag {
                            self.state.tabs.active_mut().set_rect_selection(anchor, idx);
                        } else {
                            self.state
                                .tabs
                                .active_mut()
                                .buffer
                                .set_selection(anchor, idx);
                        }
                        self.follow_caret = false;
                    }
                }
            } else if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    let shift = ui.input(|i| i.modifiers.shift);
                    if shift {
                        self.state.tabs.active_mut().clear_multi_sels();
                        let anchor = self
                            .state
                            .tabs
                            .active()
                            .buffer
                            .selection()
                            .map(|(s, _)| s)
                            .unwrap_or_else(|| self.state.tabs.active().buffer.caret());
                        self.state
                            .tabs
                            .active_mut()
                            .buffer
                            .set_selection(anchor, idx);
                    } else {
                        self.state.tabs.active_mut().clear_multi_sels();
                        self.state.tabs.active_mut().buffer.set_caret(idx);
                        if self.compare_on {
                            self.sync_compare_other_to_caret_hunk(true);
                        }
                    }
                    self.drag_anchor = None;
                    self.rect_drag = false;
                    self.sel_text_drag = None;
                    self.follow_caret = false;
                }
            }
            if ui.input(|i| i.pointer.any_released()) {
                if self.sel_text_drag.is_some() {
                    let copy = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);
                    self.finish_sel_text_drag(copy);
                }
                self.drag_anchor = None;
                self.rect_drag = false;
            }

            // Text input (arrows / typing may request caret follow)
            if response.has_focus()
                && self.focused_pane == EditorPane::Primary
                && !self.state.find_open
                && !self.show_replace
            {
                let tab = self.state.tabs.active_index();
                if self.handle_editor_input(ui, tab) {
                    self.follow_caret = true;
                    // Arrow / keyboard caret on a change line parks the other pane (same as click).
                    if self.compare_on
                        && (tab == self.compare_left_tab || tab == self.compare_right_tab)
                    {
                        self.sync_compare_other_to_caret_hunk(tab == self.compare_left_tab);
                    }
                }
            }

            // Only keep caret in view after caret motion — not after wheel scroll.
            if self.follow_caret {
                let caret_line = self
                    .state
                    .tabs
                    .active()
                    .buffer
                    .char_to_line(self.state.tabs.active().buffer.caret());
                let caret_row = display_row_for(&visible_lines, caret_line) as f32;
                if caret_row < self.scroll_line {
                    self.scroll_line = caret_row;
                } else if caret_row >= self.scroll_line + visible_rows as f32 {
                    self.scroll_line = caret_row - visible_rows as f32 + 1.0;
                }
                self.scroll_line = self.scroll_line.clamp(0.0, max_scroll);
                self.follow_caret = false;
            }

            let first_row = self.scroll_line.floor() as usize;
            let visible = visible_rows + 2;
            let last_row = (first_row + visible).min(display_count);

            let painter = ui.painter_at(rect);
            let theme = self.current_theme();
            painter.rect_filled(rect, 0.0, theme.editor_bg);
            // Gutter band + hairline so numbers stay separate from text.
            painter.rect_filled(
                Rect::from_min_max(
                    Pos2::new(rect.left(), rect.top()),
                    Pos2::new(gutter_right, rect.bottom()),
                ),
                0.0,
                theme.gutter_bg,
            );
            painter.vline(
                gutter_right,
                rect.y_range(),
                egui::Stroke::new(1.0_f32, theme.gutter_line),
            );

            let hl = &self.state.highlight_cache;
            let lang = self.state.tabs.active().language.clone();
            let bookmarks = self.state.tabs.active().bookmarks.clone();
            let changed_unsaved = self.state.tabs.active().changed_unsaved.clone();
            let changed_saved = self.state.tabs.active().changed_saved.clone();
            let style_marks = self.state.tabs.active().style_marks.clone();
            let hidden_lines = self.state.tabs.active().hidden_lines.clone();
            let fold_regions =
                crate::fold::compute_fold_regions(lang.as_str(), &self.state.tabs.active().buffer);
            let ln_right = if show_fold {
                fold_left - 2.0
            } else {
                gutter_right - 6.0
            };

            for row in first_row..last_row {
                let Some(&line_idx) = visible_lines.get(row) else {
                    break;
                };
                let y = rect.top() + (row as f32 - self.scroll_line) * row_height;
                let line_rect = Rect::from_min_size(
                    Pos2::new(rect.left(), y),
                    Vec2::new(rect.width(), row_height),
                );

                // Style-mark soft wash (first matching slot wins for paint).
                for (slot, marks) in style_marks.iter().enumerate() {
                    if marks.contains(&line_idx) {
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(text_left, y),
                                Pos2::new(rect.right(), y + row_height),
                            ),
                            0.0,
                            style_mark_bg((slot as u8) + 1),
                        );
                        break;
                    }
                }

                // 2-way compare wash (primary shows left side while comparing).
                if self.compare_on {
                    if let Some(kind) = self.compare_left_tags.get(line_idx) {
                        if let Some(bg) = crate::diff::line_kind_bg(*kind) {
                            painter.rect_filled(
                                Rect::from_min_max(
                                    Pos2::new(text_left, y),
                                    Pos2::new(rect.right(), y + row_height),
                                ),
                                0.0,
                                bg,
                            );
                        }
                    }
                    if self.state.settings.compare_hide_equal {
                        if let Some(&prev) = row.checked_sub(1).and_then(|r| visible_lines.get(r)) {
                            if let Some(skipped) = crate::diff::compare_visible_gap(prev, line_idx)
                            {
                                paint_compare_hide_gap(
                                    &painter,
                                    &font_id,
                                    text_left,
                                    rect.right(),
                                    y,
                                    skipped,
                                    theme.line_number_fg,
                                );
                            }
                        }
                    }
                }

                // Bookmark tick in the gutter.
                if bookmarks.contains(&line_idx) {
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(rect.left() + 4.0, y + 3.0),
                            Pos2::new(rect.left() + 10.0, y + row_height - 3.0),
                        ),
                        1.0,
                        Color32::from_rgb(80, 180, 220),
                    );
                }

                // Change-history bar in the gutter (amber = unsaved, green = saved).
                if changed_unsaved.contains(&line_idx) {
                    let (join_above, join_below) =
                        change_history_joins(line_idx, false, &changed_unsaved, &changed_saved);
                    paint_change_history_bar(
                        &painter,
                        rect.left(),
                        y,
                        row_height,
                        false,
                        join_above,
                        join_below,
                    );
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(text_left, y),
                            Pos2::new(rect.right(), y + row_height),
                        ),
                        0.0,
                        change_history_wash(false),
                    );
                } else if changed_saved.contains(&line_idx) {
                    let (join_above, join_below) =
                        change_history_joins(line_idx, true, &changed_unsaved, &changed_saved);
                    paint_change_history_bar(
                        &painter,
                        rect.left(),
                        y,
                        row_height,
                        true,
                        join_above,
                        join_below,
                    );
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(text_left, y),
                            Pos2::new(rect.right(), y + row_height),
                        ),
                        0.0,
                        change_history_wash(true),
                    );
                }

                // Line number — right-aligned inside the gutter (left of fold margin).
                if show_ln {
                    painter.text(
                        Pos2::new(ln_right, y),
                        egui::Align2::RIGHT_TOP,
                        format!("{}", line_idx + 1),
                        font_id.clone(),
                        theme.line_number_fg,
                    );
                }

                // Fold margin marker (− open / + folded).
                if show_fold {
                    if let Some(region) = crate::fold::region_at_header(&fold_regions, line_idx) {
                        let folded = crate::fold::is_folded(&hidden_lines, &region);
                        paint_fold_marker(
                            &painter,
                            &font_id,
                            fold_left,
                            fold_w,
                            y,
                            row_height,
                            folded,
                            theme.line_number_fg,
                        );
                    }
                }

                let line_start = self.state.tabs.active().buffer.line_to_char(line_idx);
                let raw = self.state.tabs.active().buffer.line(line_idx);
                let line_text = raw.trim_end_matches(['\n', '\r']);

                if self.compare_on {
                    if let Some(kind) = self.compare_left_tags.get(line_idx) {
                        if let Some(bg) = crate::diff::line_kind_inline_bg(*kind) {
                            if let Some(spans) = self.compare_left_inline.get(line_idx) {
                                paint_inline_compare_spans(
                                    &painter,
                                    ui,
                                    &font_id,
                                    Rect::from_min_size(
                                        Pos2::new(text_left, y),
                                        Vec2::new(rect.width(), row_height),
                                    ),
                                    line_text,
                                    spans,
                                    bg,
                                );
                            }
                        }
                    }
                }

                // Selection highlight on line
                if let Some((sel_s, sel_e)) = self.state.tabs.active().buffer.selection() {
                    let line_end = line_start + line_text.chars().count();
                    if sel_s < line_end && sel_e > line_start {
                        let local_s = sel_s
                            .saturating_sub(line_start)
                            .min(line_text.chars().count());
                        let local_e = sel_e
                            .saturating_sub(line_start)
                            .min(line_text.chars().count());
                        let x0 = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(local_s).collect::<String>(),
                            );
                        let x1 = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(local_e).collect::<String>(),
                            );
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(x0, y),
                                Pos2::new(x1.max(x0 + 2.0), y + row_height),
                            ),
                            0.0,
                            theme.selection_bg,
                        );
                    }
                }
                // Extra multi-caret / rect ranges (skip primary to avoid double paint)
                let primary_sel = self.state.tabs.active().buffer.selection();
                let multi = self.state.tabs.active().multi_sels.clone();
                for &(sel_s, sel_e) in &multi {
                    if primary_sel == Some((sel_s, sel_e)) {
                        continue;
                    }
                    let line_end = line_start + line_text.chars().count();
                    if sel_s == sel_e {
                        // Zero-width caret mark
                        if sel_s >= line_start && sel_s <= line_end {
                            let col = sel_s - line_start;
                            let prefix: String = line_text.chars().take(col).collect();
                            let cx = text_left + text_width(ui, &font_id, &prefix);
                            painter.line_segment(
                                [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                                egui::Stroke::new(1.0_f32, theme.caret_fg),
                            );
                        }
                        continue;
                    }
                    if sel_s < line_end && sel_e > line_start {
                        let local_s = sel_s
                            .saturating_sub(line_start)
                            .min(line_text.chars().count());
                        let local_e = sel_e
                            .saturating_sub(line_start)
                            .min(line_text.chars().count());
                        let x0 = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(local_s).collect::<String>(),
                            );
                        let x1 = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(local_e).collect::<String>(),
                            );
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(x0, y),
                                Pos2::new(x1.max(x0 + 2.0), y + row_height),
                            ),
                            0.0,
                            theme.selection_bg,
                        );
                    }
                }

                paint_line_text(
                    &painter,
                    ui,
                    &font_id,
                    text_left,
                    y,
                    line_text,
                    line_start,
                    hl,
                    &lang,
                    &theme,
                    crate::commands::edit::text_is_rtl(),
                );

                // Whitespace / NPC / EOL overlays
                let ws_color = theme.whitespace_fg;
                if self.state.show_whitespace || self.state.show_npc {
                    for (col, ch) in line_text.chars().enumerate() {
                        let x = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(col).collect::<String>(),
                            );
                        let mark = if self.state.show_whitespace && ch == ' ' {
                            Some("·")
                        } else if self.state.show_whitespace && ch == '\t' {
                            Some("→")
                        } else if self.state.show_npc && ch.is_control() {
                            Some("·")
                        } else {
                            None
                        };
                        if let Some(m) = mark {
                            painter.text(
                                Pos2::new(x, y),
                                egui::Align2::LEFT_TOP,
                                m,
                                font_id.clone(),
                                ws_color,
                            );
                        }
                    }
                }
                if self.state.show_eol {
                    let x = text_left + text_width(ui, &font_id, line_text);
                    painter.text(
                        Pos2::new(x, y),
                        egui::Align2::LEFT_TOP,
                        "¶",
                        font_id.clone(),
                        ws_color,
                    );
                }
                if self.state.show_indent_guide {
                    let avail = (rect.right() - text_left).max(0.0);
                    let col_w = text_width(ui, &font_id, " ");
                    if col_w > 0.0 {
                        let tab_w = self.state.settings.tab_width.max(1) as f32;
                        let mut gx = text_left + col_w * tab_w;
                        while gx < text_left + avail {
                            painter.line_segment(
                                [Pos2::new(gx, y), Pos2::new(gx, y + row_height)],
                                egui::Stroke::new(1.0_f32, theme.indent_guide),
                            );
                            gx += col_w * tab_w;
                        }
                    }
                }
                if self.state.word_wrap {
                    let max_w = (rect.right() - text_left - 8.0).max(40.0);
                    if text_width(ui, &font_id, line_text) > max_w {
                        painter.text(
                            Pos2::new(rect.right() - 14.0, y),
                            egui::Align2::LEFT_TOP,
                            "↩",
                            font_id.clone(),
                            ws_color,
                        );
                    }
                }

                // Caret
                let caret = self.state.tabs.active().buffer.caret();
                let line_end = line_start + line_text.chars().count();
                if caret >= line_start && caret <= line_end {
                    let blink_on = if self.state.settings.caret_blink {
                        ctx.request_repaint_after(std::time::Duration::from_millis(500));
                        ((ctx.input(|i| i.time) * 2.0_f64) as i64).rem_euclid(2) == 0
                    } else {
                        true
                    };
                    if blink_on {
                        let col = caret - line_start;
                        let prefix: String = line_text.chars().take(col).collect();
                        let cx = text_left + text_width(ui, &font_id, &prefix);
                        painter.line_segment(
                            [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                            egui::Stroke::new(1.0_f32, theme.caret_fg),
                        );
                    }
                }

                // Drop caret while dragging selected text.
                if let Some(drag) = self.sel_text_drag.as_ref() {
                    if drag.tab == self.state.tabs.active_index()
                        && drag.drop_at >= line_start
                        && drag.drop_at <= line_end
                    {
                        let col = drag.drop_at - line_start;
                        let prefix: String = line_text.chars().take(col).collect();
                        let cx = text_left + text_width(ui, &font_id, &prefix);
                        painter.line_segment(
                            [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                            egui::Stroke::new(2.0_f32, Color32::from_rgb(220, 140, 40)),
                        );
                    }
                }

                let _ = line_rect;
            }

            // Scrollbar thumb
            if max_scroll > 0.0 {
                let bar_w = 8.0;
                let bar_x = rect.right() - bar_w - 2.0;
                let frac = (self.scroll_line / max_scroll).clamp(0.0, 1.0);
                let thumb_h =
                    (rect.height() * (visible_rows as f32 / display_count as f32)).max(20.0);
                let thumb_y = rect.top() + frac * (rect.height() - thumb_h);
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(bar_x, thumb_y), Vec2::new(bar_w, thumb_h)),
                    2.0,
                    theme.gutter_line,
                );
            }
        });
    }

    fn persist_font_size(&mut self) {
        self.state.settings.font_size = self.font_size;
        self.state.settings.save();
    }

    fn clamp_other_view_tab(&mut self) {
        let n = self.state.tabs.len();
        if n == 0 {
            self.other_view_tab = 0;
            self.compare_partner_tab = None;
            return;
        }
        if self.other_view_tab >= n {
            self.other_view_tab = n - 1;
        }
        if let Some(p) = self.compare_partner_tab {
            if p >= n {
                self.compare_partner_tab = None;
            }
        }
    }

    /// Tab that receives typing / undo for the focused pane.
    fn focused_edit_tab(&self) -> usize {
        if self.dual_view && self.focused_pane == EditorPane::Secondary {
            self.other_view_tab
        } else {
            self.state.tabs.active_index()
        }
    }

    /// Run a menu command. Edit/Format IDs temporarily activate the focused dual-view tab.
    fn dispatch_menu_cmd(
        &mut self,
        cmd: &str,
        flags: &mut crate::commands::UiFlags,
    ) -> crate::commands::CmdResult {
        let retarget_menu =
            (cmd.starts_with("IDM_EDIT_") || cmd.starts_with("IDM_FORMAT_")) && self.dual_view;
        let focus = self.focused_edit_tab();
        let saved = self.state.tabs.active_index();
        let redirect = retarget_menu && focus != saved;
        if redirect {
            self.state.tabs.set_active(focus);
        }
        let result = crate::commands::dispatch(cmd, &mut self.state, flags);
        if redirect {
            let n = self.state.tabs.len();
            if n > 0 && saved < n {
                self.state.tabs.set_active(saved);
                self.state.highlight_dirty = true;
            }
            self.clamp_other_view_tab();
        }
        result
    }

    fn follow_focused_caret(&mut self) {
        if self.focused_pane == EditorPane::Secondary && self.dual_view {
            self.follow_caret_other = true;
            self.follow_caret = false;
        } else {
            self.follow_caret = true;
            self.follow_caret_other = false;
        }
    }

    fn ensure_other_view_tab(&mut self) {
        self.clamp_other_view_tab();
        let n = self.state.tabs.len();
        if n <= 1 {
            return;
        }
        let active = self.state.tabs.active_index();
        if self.other_view_tab == active {
            self.other_view_tab = (active + 1) % n;
        }
    }

    fn switch_other_view_now(&mut self) {
        self.dual_view = true;
        self.ensure_other_view_tab();
        let a = self.state.tabs.active_index();
        let b = self.other_view_tab;
        if a == b {
            self.state.status = "Other view shows the same tab".into();
            return;
        }
        self.state.tabs.set_active(b);
        self.other_view_tab = a;
        self.state.highlight_dirty = true;
        std::mem::swap(&mut self.scroll_line, &mut self.scroll_line_other);
        if self.compare_on {
            std::mem::swap(&mut self.compare_left_tab, &mut self.compare_right_tab);
            std::mem::swap(&mut self.compare_left_tags, &mut self.compare_right_tags);
            std::mem::swap(
                &mut self.compare_left_inline,
                &mut self.compare_right_inline,
            );
            std::mem::swap(
                &mut self.compare_hide_revealed_left,
                &mut self.compare_hide_revealed_right,
            );
        }
        self.state.status = "Switched to other view".into();
    }

    /// Remap dual-view / compare indices after tabs closed this frame.
    /// Closing a non-compared tab between the pair keeps Compare alive.
    fn apply_closed_tab_indices(&mut self) {
        let closed = self.state.take_closed_tabs();
        for idx in closed {
            self.after_tab_closed(idx);
        }
    }

    fn after_tab_closed(&mut self, closed: usize) {
        match self.compare_partner_tab {
            Some(p) if p == closed => self.compare_partner_tab = None,
            Some(p) => self.compare_partner_tab = index_after_tab_close(p, closed),
            None => {}
        }
        if let Some(other) = index_after_tab_close(self.other_view_tab, closed) {
            self.other_view_tab = other;
        } else {
            self.other_view_tab = 0;
        }
        if !self.compare_on {
            return;
        }
        let Some(left) = index_after_tab_close(self.compare_left_tab, closed) else {
            self.clear_compare();
            return;
        };
        let Some(right) = index_after_tab_close(self.compare_right_tab, closed) else {
            self.clear_compare();
            return;
        };
        self.compare_left_tab = left;
        self.compare_right_tab = right;
    }

    /// Keep compare panes on the compared pair (left = primary, right = other).
    fn sync_compare_panes(&mut self) {
        if !self.compare_on {
            return;
        }
        let n = self.state.tabs.len();
        if n == 0 {
            self.clear_compare();
            return;
        }
        if self.compare_left_tab >= n || self.compare_right_tab >= n {
            self.clear_compare();
            return;
        }
        if self.compare_left_tab == self.compare_right_tab {
            self.clear_compare();
            return;
        }
        self.dual_view = true;
        self.other_view_tab = self.compare_right_tab;
        if self.state.tabs.active_index() != self.compare_left_tab {
            self.state.tabs.set_active(self.compare_left_tab);
            self.state.highlight_dirty = true;
        }
    }

    fn apply_dual_view_flags(&mut self, flags: &mut crate::commands::UiFlags) {
        if let Some(on) = flags.sync_scroll_h {
            self.sync_scroll_h = on;
        }
        if let Some(on) = flags.sync_scroll_v {
            self.sync_scroll_v = on;
        }
        if let Some(on) = flags.zoom_sync {
            self.zoom_sync = on;
        }
        if let Some(on) = flags.dual_view {
            self.dual_view = on;
            if on {
                self.ensure_other_view_tab();
            } else {
                self.focused_pane = EditorPane::Primary;
            }
        }
        if let Some(idx) = flags.other_view_tab {
            self.other_view_tab = idx;
            self.dual_view = true;
            self.clamp_other_view_tab();
        }
        if flags.assign_other_view {
            self.other_view_tab = self.state.tabs.active_index();
            self.dual_view = true;
            let n = self.state.tabs.len();
            if n > 1 {
                let next = (self.other_view_tab + 1) % n;
                self.state.tabs.set_active(next);
                self.state.highlight_dirty = true;
            }
        }
        if flags.switch_other_view {
            self.switch_other_view_now();
        }
        if flags.clear_compare {
            self.clear_compare();
        }
        if flags.start_compare {
            self.start_compare();
        }
        if flags.compare_to_saved {
            self.compare_to_saved();
        }
        if flags.swap_compare {
            self.swap_compare_sides();
        }
        if let Some(nav) = flags.compare_nav {
            self.navigate_compare_hunk(nav);
        }
        if let Some(toggle) = flags.compare_ignore_toggle {
            self.toggle_compare_ignore(toggle);
        }
        if flags.compare_hide_equal_toggle {
            self.toggle_compare_hide_equal();
        }
        if let Some(delta) = flags.compare_hide_equal_context_delta {
            self.adjust_compare_hide_equal_context(delta);
        }
        if flags.compare_expand_hidden_equal_at_caret {
            self.expand_compare_hide_equal_at_caret();
        }
        if flags.compare_collapse_hidden_equal_at_caret {
            self.collapse_compare_hide_equal_at_caret();
        }
        if let Some(nav) = flags.compare_hide_gap_nav {
            self.navigate_compare_hide_gap(nav);
        }
        if flags.compare_expand_hidden_equal {
            self.expand_all_compare_hide_equal();
        }
        if flags.compare_collapse_hidden_equal {
            self.collapse_all_compare_hide_equal();
        }
        if flags.compare_bookmark_diffs {
            self.bookmark_compare_differences();
        }
        if flags.compare_clear_diff_bookmarks {
            self.clear_compare_difference_bookmarks();
        }
        if flags.copy_compare_diff {
            self.copy_compare_diff(flags);
        }
        if flags.open_compare_diff {
            self.open_compare_diff_tab();
        }
        if flags.copy_compare_summary {
            self.copy_compare_summary(flags);
        }
        if flags.open_compare_summary {
            self.open_compare_summary_tab();
        }
        if flags.copy_compare_hunk {
            self.copy_compare_hunk(flags);
        }
        if flags.open_compare_hunk {
            self.open_compare_hunk_tab();
        }
        if flags.apply_compare_hunk {
            self.apply_compare_hunk_from_other();
        }
        if flags.apply_compare_hunk_to_other {
            self.apply_compare_hunk_to_other();
        }
        if flags.apply_all_compare_hunks {
            self.apply_all_compare_hunks_from_other();
        }
        if flags.apply_all_compare_hunks_to_other {
            self.apply_all_compare_hunks_to_other();
        }
    }

    /// Toggle ignore-whitespace / ignore-case / ignore-blank from View menu; persist and re-diff.
    fn toggle_compare_ignore(&mut self, toggle: crate::commands::CompareIgnoreToggle) {
        match toggle {
            crate::commands::CompareIgnoreToggle::Whitespace => {
                self.state.settings.compare_ignore_ws = !self.state.settings.compare_ignore_ws;
            }
            crate::commands::CompareIgnoreToggle::Case => {
                self.state.settings.compare_ignore_case = !self.state.settings.compare_ignore_case;
            }
            crate::commands::CompareIgnoreToggle::BlankLines => {
                self.state.settings.compare_ignore_blank =
                    !self.state.settings.compare_ignore_blank;
            }
        }
        self.state.settings.save();
        let ignore = CompareIgnoreBits {
            ws: self.state.settings.compare_ignore_ws,
            case: self.state.settings.compare_ignore_case,
            blank: self.state.settings.compare_ignore_blank,
        };
        let hide_equal = self.state.settings.compare_hide_equal;
        if self.compare_on {
            // Refresh immediately so the toggle is visible without waiting for debounce.
            self.state.compare_stale = false;
            self.compare_refresh_at = None;
            let left = self.compare_left_tab;
            let right = self.compare_right_tab;
            if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
                crate::diff::prune_compare_hide_revealed(
                    &mut self.compare_hide_revealed_left,
                    &self.compare_left_tags,
                    self.state.settings.compare_hide_equal_context_lines(),
                );
                crate::diff::prune_compare_hide_revealed(
                    &mut self.compare_hide_revealed_right,
                    &self.compare_right_tags,
                    self.state.settings.compare_hide_equal_context_lines(),
                );
                let lname = self
                    .state
                    .tabs
                    .get(left)
                    .map(|d| d.title.clone())
                    .unwrap_or_else(|| "left".into());
                let rname = self
                    .state
                    .tabs
                    .get(right)
                    .map(|d| d.title.clone())
                    .unwrap_or_else(|| "right".into());
                let hide = compare_hide_opt(
                    hide_equal,
                    &self.compare_left_tags,
                    &self.compare_right_tags,
                    &self.compare_hide_revealed_left,
                    &self.compare_hide_revealed_right,
                    self.state.settings.compare_hide_equal_context_lines(),
                );
                let counts = compare_pair_counts_from_tags(
                    &self.compare_left_tags,
                    &self.compare_right_tags,
                );
                let hunk_n = counts.hunk_n;
                // Ignore toggles reshuffle hunks — re-park like Compare start.
                if hunk_n > 0 {
                    self.park_compare_hunk_ordinal(1);
                    self.select_compare_hunk_on_pane(true, 1);
                    self.select_compare_hunk_on_pane(false, 1);
                }
                let mut status = compare_pair_status(
                    &lname,
                    &rname,
                    counts,
                    ignore,
                    CompareHideStatus {
                        hidden: hide,
                        context: self.state.settings.compare_hide_equal_context_lines(),
                    },
                );
                if hunk_n > 0 {
                    let lr = self.compare_hunk_lr_label(1);
                    let ordinal = self.compare_hunk_ordinal_bit(1, hunk_n);
                    status.push_str(&compare_at_hunk_status_bit(&lr, &ordinal));
                }
                self.state.status = status;
                self.state.highlight_dirty = true;
            }
        } else {
            let which = match toggle {
                crate::commands::CompareIgnoreToggle::Whitespace => "Ignore whitespace",
                crate::commands::CompareIgnoreToggle::Case => "Ignore case",
                crate::commands::CompareIgnoreToggle::BlankLines => "Ignore blank lines",
            };
            let on = match toggle {
                crate::commands::CompareIgnoreToggle::Whitespace => ignore.ws,
                crate::commands::CompareIgnoreToggle::Case => ignore.case,
                crate::commands::CompareIgnoreToggle::BlankLines => ignore.blank,
            };
            self.state.status = format!(
                "{which}: {}{}",
                if on { "on" } else { "off" },
                compare_ignore_status_bit(ignore)
            );
        }
    }

    /// Click a ···N hide-equal cue to reveal that collapsed Equal run on both panes.
    fn try_expand_compare_hide_gap_at(
        &mut self,
        pos: Pos2,
        rect: Rect,
        scroll_line: f32,
        row_height: f32,
        visible_lines: &[usize],
        left_pane: bool,
    ) -> bool {
        if !self.compare_on || !self.state.settings.compare_hide_equal {
            return false;
        }
        let Some(gap) =
            compare_hide_gap_at_pointer(pos, rect, scroll_line, row_height, visible_lines)
        else {
            return false;
        };
        self.expand_compare_hide_gap(gap, left_pane)
    }

    /// Reveal one hide-equal gap on the focused pane and its aligned partners.
    fn expand_compare_hide_gap(&mut self, gap: (usize, usize), left_pane: bool) -> bool {
        let n = if left_pane {
            crate::diff::reveal_compare_gap(&mut self.compare_hide_revealed_left, gap)
        } else {
            crate::diff::reveal_compare_gap(&mut self.compare_hide_revealed_right, gap)
        };
        if n == 0 {
            return false;
        }
        let partners = crate::diff::aligned_equal_partner_lines(
            &self.compare_left_tags,
            &self.compare_right_tags,
            gap,
            left_pane,
        );
        let n_other = if left_pane {
            crate::diff::reveal_compare_lines(&mut self.compare_hide_revealed_right, &partners)
        } else {
            crate::diff::reveal_compare_lines(&mut self.compare_hide_revealed_left, &partners)
        };
        let hide = compare_hide_opt(
            true,
            &self.compare_left_tags,
            &self.compare_right_tags,
            &self.compare_hide_revealed_left,
            &self.compare_hide_revealed_right,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let remain = hide.map(|(l, r)| l + r).unwrap_or(0);
        let both = if n_other > 0 { " both panes" } else { "" };
        let counts =
            compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
        self.state.status = compare_expand_gap_status(
            n,
            both,
            counts.del,
            counts.ins,
            counts.hunk_n,
            &counts.kind_bit,
            remain,
        );
        true
    }

    /// Jump to the next/previous collapsed Equal (···N) gap on the focused pane.
    fn navigate_compare_hide_gap(&mut self, nav: crate::commands::CompareHideGapNav) {
        if !self.compare_on {
            self.state.status = "Compare off — start Compare to navigate hidden equal lines".into();
            return;
        }
        if !self.state.settings.compare_hide_equal {
            self.state.status =
                "Hide Unchanged Lines is off — turn it on to navigate ···N gaps".into();
            return;
        }
        let left_pane = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let tab = if left_pane {
            self.compare_left_tab
        } else {
            self.compare_right_tab
        };
        let Some(doc) = self.state.tabs.get(tab) else {
            return;
        };
        let caret_line = doc.buffer.char_to_line(doc.buffer.caret());
        let line_count = doc.buffer.line_count().max(1);
        let (tags, revealed) = if left_pane {
            (&self.compare_left_tags, &self.compare_hide_revealed_left)
        } else {
            (&self.compare_right_tags, &self.compare_hide_revealed_right)
        };
        let visible = visible_lines_with_compare_hide(
            line_count,
            &doc.hidden_lines,
            true,
            true,
            tags,
            revealed,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let gaps = crate::diff::list_compare_hide_gaps(&visible);
        let Some(gap) = (match nav {
            crate::commands::CompareHideGapNav::Next => {
                crate::diff::next_compare_hide_gap(caret_line, &gaps)
            }
            crate::commands::CompareHideGapNav::Prev => {
                crate::diff::prev_compare_hide_gap(caret_line, &gaps)
            }
            crate::commands::CompareHideGapNav::First => gaps.first().copied(),
            crate::commands::CompareHideGapNav::Last => gaps.last().copied(),
        }) else {
            self.state.status = "Compare: no hidden equal gaps".into();
            return;
        };
        let park = crate::diff::compare_hide_gap_park_line(gap);
        let n = gap.1 - gap.0 + 1;
        let (ord, total) = crate::diff::compare_hide_gap_ordinal(&gaps, gap).unwrap_or((1, 1));
        let wrapped = match nav {
            crate::commands::CompareHideGapNav::Next => {
                crate::diff::hunk_nav_wrapped(true, caret_line, park)
            }
            crate::commands::CompareHideGapNav::Prev => {
                crate::diff::hunk_nav_wrapped(false, caret_line, park)
            }
            crate::commands::CompareHideGapNav::First
            | crate::commands::CompareHideGapNav::Last => false,
        };
        if let Some(doc) = self.state.tabs.get_mut(tab) {
            let at = doc
                .buffer
                .line_to_char(park.min(doc.buffer.line_count().saturating_sub(1)));
            doc.buffer.set_caret(at);
        }
        if left_pane {
            self.follow_caret = true;
        } else {
            self.follow_caret_other = true;
        }
        self.sync_compare_other_to_equal_line(left_pane, park);
        let dir = match nav {
            crate::commands::CompareHideGapNav::Next => "Next",
            crate::commands::CompareHideGapNav::Prev => "Previous",
            crate::commands::CompareHideGapNav::First => "First",
            crate::commands::CompareHideGapNav::Last => "Last",
        };
        let counts =
            compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
        // Overwrite equal-park status with gap-nav ordinal + pair −/+/kind tallies.
        self.state.status = compare_hide_gap_nav_status(dir, n, ord, total, &counts, wrapped);
    }

    /// Expand the collapsed Equal run nearest the focused caret (menu / keyboard).
    fn expand_compare_hide_equal_at_caret(&mut self) {
        if !self.compare_on {
            self.state.status = "Compare off — start Compare to expand hidden equal lines".into();
            return;
        }
        if !self.state.settings.compare_hide_equal {
            self.state.status =
                "Hide Unchanged Lines is off — turn it on to collapse, then expand".into();
            return;
        }
        let left_pane = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let tab = if left_pane {
            self.compare_left_tab
        } else {
            self.compare_right_tab
        };
        let Some(doc) = self.state.tabs.get(tab) else {
            return;
        };
        let line = doc.buffer.char_to_line(doc.buffer.caret());
        let line_count = doc.buffer.line_count().max(1);
        let (tags, revealed) = if left_pane {
            (&self.compare_left_tags, &self.compare_hide_revealed_left)
        } else {
            (&self.compare_right_tags, &self.compare_hide_revealed_right)
        };
        let visible = visible_lines_with_compare_hide(
            line_count,
            &doc.hidden_lines,
            true,
            true,
            tags,
            revealed,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let Some(gap) = crate::diff::nearest_compare_hide_gap(line, &visible) else {
            self.state.status = "Compare: no hidden equal run at caret".into();
            return;
        };
        if !self.expand_compare_hide_gap(gap, left_pane) {
            self.state.status = "Compare: no hidden equal run at caret".into();
        }
    }

    /// Re-collapse the expanded Equal run nearest the focused caret (menu / keyboard).
    fn collapse_compare_hide_equal_at_caret(&mut self) {
        if !self.compare_on {
            self.state.status =
                "Compare off — start Compare to collapse expanded equal lines".into();
            return;
        }
        if !self.state.settings.compare_hide_equal {
            self.state.status =
                "Hide Unchanged Lines is off — turn it on to collapse unchanged lines".into();
            return;
        }
        let left_pane = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let tab = if left_pane {
            self.compare_left_tab
        } else {
            self.compare_right_tab
        };
        let Some(doc) = self.state.tabs.get(tab) else {
            return;
        };
        let line = doc.buffer.char_to_line(doc.buffer.caret());
        let gap = {
            let revealed = if left_pane {
                &self.compare_hide_revealed_left
            } else {
                &self.compare_hide_revealed_right
            };
            crate::diff::nearest_compare_revealed_run(line, revealed)
        };
        let Some(gap) = gap else {
            self.state.status = "Compare: no expanded equal run at caret".into();
            return;
        };
        if !self.collapse_compare_hide_gap(gap, left_pane) {
            self.state.status = "Compare: no expanded equal run at caret".into();
        }
    }

    /// Hide one previously expanded Equal run on the focused pane and its partners.
    fn collapse_compare_hide_gap(&mut self, gap: (usize, usize), left_pane: bool) -> bool {
        let n = if left_pane {
            crate::diff::collapse_compare_gap(&mut self.compare_hide_revealed_left, gap)
        } else {
            crate::diff::collapse_compare_gap(&mut self.compare_hide_revealed_right, gap)
        };
        if n == 0 {
            return false;
        }
        let partners = crate::diff::aligned_equal_partner_lines(
            &self.compare_left_tags,
            &self.compare_right_tags,
            gap,
            left_pane,
        );
        let n_other = if left_pane {
            crate::diff::collapse_compare_lines(&mut self.compare_hide_revealed_right, &partners)
        } else {
            crate::diff::collapse_compare_lines(&mut self.compare_hide_revealed_left, &partners)
        };
        let hide = compare_hide_opt(
            true,
            &self.compare_left_tags,
            &self.compare_right_tags,
            &self.compare_hide_revealed_left,
            &self.compare_hide_revealed_right,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let remain = hide.map(|(l, r)| l + r).unwrap_or(0);
        let both = if n_other > 0 { " both panes" } else { "" };
        let counts =
            compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
        self.state.status = compare_collapse_gap_status(
            n,
            both,
            counts.del,
            counts.ins,
            counts.hunk_n,
            &counts.kind_bit,
            remain,
        );
        true
    }

    /// Reveal every collapsed Equal run on both panes; hide-equal preference stays on.
    fn expand_all_compare_hide_equal(&mut self) {
        if !self.compare_on {
            self.state.status = "Compare off — start Compare to expand hidden equal lines".into();
            return;
        }
        if !self.state.settings.compare_hide_equal {
            self.state.status =
                "Hide Unchanged Lines is off — turn it on to collapse, then expand".into();
            return;
        }
        let n_left = crate::diff::reveal_all_compare_hidden(
            &mut self.compare_hide_revealed_left,
            &self.compare_left_tags,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let n_right = crate::diff::reveal_all_compare_hidden(
            &mut self.compare_hide_revealed_right,
            &self.compare_right_tags,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let n = n_left + n_right;
        self.state.status = if n == 0 {
            "Compare: no hidden equal lines".into()
        } else {
            let counts =
                compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
            compare_expand_all_unchanged_status(
                counts.del,
                counts.ins,
                counts.hunk_n,
                &counts.kind_bit,
                n,
            )
        };
    }

    /// Re-collapse click/menu-expanded Equal runs; hide-equal preference stays on.
    fn collapse_all_compare_hide_equal(&mut self) {
        if !self.compare_on {
            self.state.status =
                "Compare off — start Compare to collapse expanded equal lines".into();
            return;
        }
        if !self.state.settings.compare_hide_equal {
            self.state.status =
                "Hide Unchanged Lines is off — turn it on to collapse unchanged lines".into();
            return;
        }
        let n_left =
            crate::diff::collapse_all_compare_revealed(&mut self.compare_hide_revealed_left);
        let n_right =
            crate::diff::collapse_all_compare_revealed(&mut self.compare_hide_revealed_right);
        let n = n_left + n_right;
        let hide = compare_hide_opt(
            true,
            &self.compare_left_tags,
            &self.compare_right_tags,
            &self.compare_hide_revealed_left,
            &self.compare_hide_revealed_right,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let remain = hide.map(|(l, r)| l + r).unwrap_or(0);
        self.state.status = if n == 0 {
            if remain == 0 {
                "Compare: no expanded equal lines to collapse".into()
            } else {
                format!("Compare: equal lines already collapsed ({remain} hidden)")
            }
        } else {
            let counts =
                compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
            compare_collapse_all_unchanged_status(
                counts.del,
                counts.ins,
                counts.hunk_n,
                &counts.kind_bit,
                n,
                remain,
            )
        };
    }

    /// Bookmark every change-hunk start on both Compare panes (F2 / Shift+F2).
    fn bookmark_compare_differences(&mut self) {
        if !self.compare_on {
            self.state.status = "Compare off — start Compare to bookmark differences".into();
            return;
        }
        let left_starts = crate::diff::hunk_starts(&self.compare_left_tags);
        let right_starts = crate::diff::hunk_starts(&self.compare_right_tags);
        if left_starts.is_empty() && right_starts.is_empty() {
            self.state.status = "Compare: no differences to bookmark (identical)".into();
            return;
        }
        let left_tab = self.compare_left_tab;
        let right_tab = self.compare_right_tab;
        let mut added_l = 0usize;
        let mut added_r = 0usize;
        if let Some(doc) = self.state.tabs.get_mut(left_tab) {
            for &line in &left_starts {
                if doc.bookmarks.insert(line) {
                    added_l += 1;
                }
            }
        }
        if right_tab != left_tab {
            if let Some(doc) = self.state.tabs.get_mut(right_tab) {
                for &line in &right_starts {
                    if doc.bookmarks.insert(line) {
                        added_r += 1;
                    }
                }
            }
        }
        let counts =
            compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
        let added = added_l + added_r;
        self.state.status = compare_bookmark_differences_status(
            counts.del,
            counts.ins,
            left_starts.len(),
            right_starts.len(),
            counts.hunk_n,
            &counts.kind_bit,
            added,
        );
    }

    /// Clear bookmarks at each change-hunk start on both Compare panes.
    fn clear_compare_difference_bookmarks(&mut self) {
        if !self.compare_on {
            self.state.status = "Compare off — start Compare to clear difference bookmarks".into();
            return;
        }
        let left_starts = crate::diff::hunk_starts(&self.compare_left_tags);
        let right_starts = crate::diff::hunk_starts(&self.compare_right_tags);
        if left_starts.is_empty() && right_starts.is_empty() {
            self.state.status = "Compare: no difference bookmarks to clear (identical)".into();
            return;
        }
        let left_tab = self.compare_left_tab;
        let right_tab = self.compare_right_tab;
        let mut removed_l = 0usize;
        let mut removed_r = 0usize;
        if let Some(doc) = self.state.tabs.get_mut(left_tab) {
            for &line in &left_starts {
                if doc.bookmarks.remove(&line) {
                    removed_l += 1;
                }
            }
        }
        if right_tab != left_tab {
            if let Some(doc) = self.state.tabs.get_mut(right_tab) {
                for &line in &right_starts {
                    if doc.bookmarks.remove(&line) {
                        removed_r += 1;
                    }
                }
            }
        }
        let counts =
            compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
        let removed = removed_l + removed_r;
        self.state.status = compare_clear_difference_bookmarks_status(
            counts.del,
            counts.ins,
            left_starts.len(),
            right_starts.len(),
            counts.hunk_n,
            &counts.kind_bit,
            removed,
        );
    }

    /// Toggle hide-unchanged-lines for Compare; persist (no re-diff needed).
    fn toggle_compare_hide_equal(&mut self) {
        self.state.settings.compare_hide_equal = !self.state.settings.compare_hide_equal;
        self.state.settings.save();
        let hide_equal = self.state.settings.compare_hide_equal;
        if !hide_equal {
            self.compare_hide_revealed_left.clear();
            self.compare_hide_revealed_right.clear();
        }
        let ignore = CompareIgnoreBits {
            ws: self.state.settings.compare_ignore_ws,
            case: self.state.settings.compare_ignore_case,
            blank: self.state.settings.compare_ignore_blank,
        };
        if self.compare_on {
            let left = self.compare_left_tab;
            let right = self.compare_right_tab;
            let lname = self
                .state
                .tabs
                .get(left)
                .map(|d| d.title.clone())
                .unwrap_or_else(|| "left".into());
            let rname = self
                .state
                .tabs
                .get(right)
                .map(|d| d.title.clone())
                .unwrap_or_else(|| "right".into());
            let hide = compare_hide_opt(
                hide_equal,
                &self.compare_left_tags,
                &self.compare_right_tags,
                &self.compare_hide_revealed_left,
                &self.compare_hide_revealed_right,
                self.state.settings.compare_hide_equal_context_lines(),
            );
            let counts =
                compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
            let hunk_n = counts.hunk_n;
            // Hide toggle can orphan the caret on a now-hidden Equal line — re-park like
            // Compare start / ignore re-diff / swap.
            if hunk_n > 0 {
                self.park_compare_hunk_ordinal(1);
                self.select_compare_hunk_on_pane(true, 1);
                self.select_compare_hunk_on_pane(false, 1);
            }
            let mut status = compare_pair_status(
                &lname,
                &rname,
                counts,
                ignore,
                CompareHideStatus {
                    hidden: hide,
                    context: self.state.settings.compare_hide_equal_context_lines(),
                },
            );
            if hunk_n > 0 {
                let lr = self.compare_hunk_lr_label(1);
                let ordinal = self.compare_hunk_ordinal_bit(1, hunk_n);
                status.push_str(&compare_at_hunk_status_bit(&lr, &ordinal));
            }
            self.state.status = status;
        } else {
            self.state.status = format!(
                "Hide unchanged lines: {}{}",
                if hide_equal { "on" } else { "off" },
                if hide_equal {
                    format!(
                        " (±{} context; applies when Compare is on)",
                        self.state.settings.compare_hide_equal_context_lines()
                    )
                } else {
                    String::new()
                }
            );
        }
    }

    /// Bump Preferences hide-equal context by ±1 (clamped 0..=10).
    fn adjust_compare_hide_equal_context(&mut self, delta: i8) {
        let prev = self.state.settings.compare_hide_equal_context_lines();
        let next = (prev as i32 + i32::from(delta)).clamp(0, 10) as usize;
        self.set_compare_hide_equal_context(next);
    }

    /// Set Preferences hide-equal context (clamped 0..=10); clear ···N expansions.
    /// While Compare + Hide Unchanged is on, parks both panes on the first change hunk.
    fn set_compare_hide_equal_context(&mut self, next: usize) {
        let prev = self.state.settings.compare_hide_equal_context_lines();
        let next = next.clamp(0, 10);
        if next == prev {
            self.state.status = format!("Hide-equal context already ±{prev}");
            return;
        }
        self.state.settings.compare_hide_equal_context = next as u8;
        self.state.settings.save();
        let hide_equal = self.state.settings.compare_hide_equal;
        if self.compare_on && hide_equal {
            self.compare_hide_revealed_left.clear();
            self.compare_hide_revealed_right.clear();
        }
        let ctx = next;
        let ignore = CompareIgnoreBits {
            ws: self.state.settings.compare_ignore_ws,
            case: self.state.settings.compare_ignore_case,
            blank: self.state.settings.compare_ignore_blank,
        };
        if self.compare_on {
            let left = self.compare_left_tab;
            let right = self.compare_right_tab;
            let lname = self
                .state
                .tabs
                .get(left)
                .map(|d| d.title.clone())
                .unwrap_or_else(|| "left".into());
            let rname = self
                .state
                .tabs
                .get(right)
                .map(|d| d.title.clone())
                .unwrap_or_else(|| "right".into());
            let hide = compare_hide_opt(
                hide_equal,
                &self.compare_left_tags,
                &self.compare_right_tags,
                &self.compare_hide_revealed_left,
                &self.compare_hide_revealed_right,
                ctx,
            );
            let counts =
                compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
            let hunk_n = counts.hunk_n;
            // Context change can orphan the caret on a now-hidden Equal line — re-park like
            // hide-equal toggle / Compare start / ignore re-diff / swap.
            if hide_equal && hunk_n > 0 {
                self.park_compare_hunk_ordinal(1);
                self.select_compare_hunk_on_pane(true, 1);
                self.select_compare_hunk_on_pane(false, 1);
            }
            let mut status = compare_pair_status(
                &lname,
                &rname,
                counts,
                ignore,
                CompareHideStatus {
                    hidden: hide,
                    context: ctx,
                },
            );
            if hide_equal && hunk_n > 0 {
                let lr = self.compare_hunk_lr_label(1);
                let ordinal = self.compare_hunk_ordinal_bit(1, hunk_n);
                status.push_str(&compare_at_hunk_status_bit(&lr, &ordinal));
            }
            self.state.status = format!("Hide-equal context ±{ctx} — {status}");
        } else {
            self.state.status = format!(
                "Hide-equal context ±{ctx} (applies when Hide Unchanged Lines is on during Compare)"
            );
        }
    }

    fn tab_compare_lines(&self, tab: usize) -> Vec<String> {
        self.state
            .tabs
            .get(tab)
            .map(|d| {
                (0..d.buffer.line_count())
                    .map(|i| d.buffer.line(i).trim_end_matches(['\n', '\r']).to_string())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn compare_side_name(title: &str) -> String {
        title.replace(['\n', '\r'], " ")
    }

    /// Unified diff text for the current Compare pair.
    fn compare_unified_diff_text(
        &mut self,
    ) -> Option<(String, String, String, usize, usize, usize, String)> {
        if !self.compare_on {
            return None;
        }
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let left_lines = self.tab_compare_lines(left);
        let right_lines = self.tab_compare_lines(right);
        if left_lines.len() != self.compare_left_tags.len()
            || right_lines.len() != self.compare_right_tags.len()
        {
            if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
            } else {
                return None;
            }
        }
        let lname = self
            .state
            .tabs
            .get(left)
            .map(|d| Self::compare_side_name(&d.title))
            .unwrap_or_else(|| "left".into());
        let rname = self
            .state
            .tabs
            .get(right)
            .map(|d| Self::compare_side_name(&d.title))
            .unwrap_or_else(|| "right".into());
        let left_refs: Vec<&str> = left_lines.iter().map(String::as_str).collect();
        let right_refs: Vec<&str> = right_lines.iter().map(String::as_str).collect();
        let text = crate::diff::unified_diff(
            &left_refs,
            &right_refs,
            &lname,
            &rname,
            &self.compare_left_tags,
            &self.compare_right_tags,
        )?;
        let (del, ins) =
            crate::diff::count_changes(&self.compare_left_tags, &self.compare_right_tags);
        let hunk_n =
            crate::diff::compare_summary_hunks(&self.compare_left_tags, &self.compare_right_tags)
                .map(|h| h.len())
                .unwrap_or(0);
        let kind_bit =
            crate::diff::compare_kind_tally_bit(&self.compare_left_tags, &self.compare_right_tags);
        Some((text, lname, rname, del, ins, hunk_n, kind_bit))
    }

    /// Clipboard: unified diff of the current Compare pair.
    fn copy_compare_diff(&mut self, flags: &mut crate::commands::UiFlags) {
        if !self.compare_on {
            self.state.status = "Copy Compare Diff: Compare is off".into();
            return;
        }
        let Some((text, lname, rname, del, ins, hunk_n, kind_bit)) =
            self.compare_unified_diff_text()
        else {
            self.state.status = "Copy Compare Diff: could not build unified diff".into();
            return;
        };
        flags.pending_clipboard = Some(text);
        self.state.status =
            compare_open_or_copy_status("Copied", &lname, &rname, del, ins, hunk_n, &kind_bit);
    }

    /// New tab with the unified diff. Clears Compare so the tab is visible.
    fn open_compare_diff_tab(&mut self) {
        if !self.compare_on {
            self.state.status = "Open Compare Diff: Compare is off".into();
            return;
        }
        let Some((text, lname, rname, del, ins, hunk_n, kind_bit)) =
            self.compare_unified_diff_text()
        else {
            self.state.status = "Open Compare Diff: could not build unified diff".into();
            return;
        };
        self.clear_compare();
        self.dual_view = false;
        self.state.tabs.open_untitled();
        {
            let doc = self.state.tabs.active_mut();
            doc.title = "compare.diff".into();
            doc.buffer = buffer::TextBuffer::from_str(&text);
            doc.dirty = true;
            doc.language = "plain".into();
        }
        self.state.highlight_dirty = true;
        self.state.reset_view = true;
        self.state.status =
            compare_open_or_copy_status("Opened", &lname, &rname, del, ins, hunk_n, &kind_bit);
    }

    /// Build the Compare hunk-index text (refreshes stale tags when needed).
    fn compare_summary_payload(
        &mut self,
    ) -> Option<(String, String, String, usize, usize, usize, String)> {
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let left_lines = self.tab_compare_lines(left);
        let right_lines = self.tab_compare_lines(right);
        if left_lines.len() != self.compare_left_tags.len()
            || right_lines.len() != self.compare_right_tags.len()
        {
            let (lt, rt, _, _) = self.compute_compare_tags(left, right)?;
            self.compare_left_tags = lt;
            self.compare_right_tags = rt;
        }
        let lname = self
            .state
            .tabs
            .get(left)
            .map(|d| Self::compare_side_name(&d.title))
            .unwrap_or_else(|| "left".into());
        let rname = self
            .state
            .tabs
            .get(right)
            .map(|d| Self::compare_side_name(&d.title))
            .unwrap_or_else(|| "right".into());
        let hunks =
            crate::diff::compare_summary_hunks(&self.compare_left_tags, &self.compare_right_tags)?;
        let left_refs: Vec<&str> = left_lines.iter().map(String::as_str).collect();
        let right_refs: Vec<&str> = right_lines.iter().map(String::as_str).collect();
        let ignore_note = compare_ignore_status_bit(CompareIgnoreBits {
            ws: self.state.settings.compare_ignore_ws,
            case: self.state.settings.compare_ignore_case,
            blank: self.state.settings.compare_ignore_blank,
        });
        let text = crate::diff::compare_summary_text(
            &lname,
            &rname,
            &left_refs,
            &right_refs,
            &self.compare_left_tags,
            &self.compare_right_tags,
            &ignore_note,
        )?;
        let (del, ins) =
            crate::diff::count_changes(&self.compare_left_tags, &self.compare_right_tags);
        let kind_bit =
            crate::diff::compare_kind_tally_bit(&self.compare_left_tags, &self.compare_right_tags);
        Some((text, lname, rname, del, ins, hunks.len(), kind_bit))
    }

    /// Clipboard: text summary of every change hunk (keeps Compare on).
    fn copy_compare_summary(&mut self, flags: &mut crate::commands::UiFlags) {
        if !self.compare_on {
            self.state.status = "Copy Compare Summary: Compare is off".into();
            return;
        }
        let Some((text, lname, rname, del, ins, hunk_n, kind_bit)) = self.compare_summary_payload()
        else {
            self.state.status = "Copy Compare Summary: could not build summary".into();
            return;
        };
        flags.pending_clipboard = Some(text);
        self.state.status =
            compare_summary_copy_open_status("Copied", del, ins, hunk_n, &kind_bit, &lname, &rname);
    }

    /// New tab listing every change hunk (line ranges). Clears Compare so the tab is visible.
    fn open_compare_summary_tab(&mut self) {
        if !self.compare_on {
            self.state.status = "Open Compare Summary: Compare is off".into();
            return;
        }
        let Some((text, lname, rname, del, ins, hunk_n, kind_bit)) = self.compare_summary_payload()
        else {
            self.state.status = "Open Compare Summary: could not build summary".into();
            return;
        };
        self.clear_compare();
        self.dual_view = false;
        self.state.tabs.open_untitled();
        {
            let doc = self.state.tabs.active_mut();
            doc.title = "compare-summary.txt".into();
            doc.buffer = buffer::TextBuffer::from_str(&text);
            doc.dirty = true;
            doc.language = "plain".into();
        }
        self.state.highlight_dirty = true;
        self.state.reset_view = true;
        self.state.status =
            compare_summary_copy_open_status("Opened", del, ins, hunk_n, &kind_bit, &lname, &rname);
    }

    /// Unified diff for the hunk at the focused caret (or the next hunk).
    fn compare_caret_hunk_unified(&mut self) -> Result<CompareHunkPayload, &'static str> {
        if !self.compare_on {
            return Err("Compare is off");
        }
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let left_lines = self.tab_compare_lines(left);
        let right_lines = self.tab_compare_lines(right);
        if left_lines.len() != self.compare_left_tags.len()
            || right_lines.len() != self.compare_right_tags.len()
        {
            if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
            } else {
                return Err("could not build unified diff");
            }
        }
        let primary = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let tab = if primary { left } else { right };
        let Some(doc) = self.state.tabs.get(tab) else {
            return Err("tab missing");
        };
        let line = doc.buffer.char_to_line(doc.buffer.caret());
        let tags = if primary {
            &self.compare_left_tags
        } else {
            &self.compare_right_tags
        };
        let Some((ord, total)) = crate::diff::hunk_ordinal_for_copy(tags, line) else {
            return Err("no differences");
        };
        let lname = self
            .state
            .tabs
            .get(left)
            .map(|d| Self::compare_side_name(&d.title))
            .unwrap_or_else(|| "left".into());
        let rname = self
            .state
            .tabs
            .get(right)
            .map(|d| Self::compare_side_name(&d.title))
            .unwrap_or_else(|| "right".into());
        let left_refs: Vec<&str> = left_lines.iter().map(String::as_str).collect();
        let right_refs: Vec<&str> = right_lines.iter().map(String::as_str).collect();
        let Some(text) = crate::diff::unified_diff_hunk(
            &left_refs,
            &right_refs,
            &lname,
            &rname,
            &self.compare_left_tags,
            &self.compare_right_tags,
            primary,
            line,
        ) else {
            return Err("could not build unified diff");
        };
        let (kind, del, ins) = crate::diff::compare_hunk_kind_and_counts(
            &self.compare_left_tags,
            &self.compare_right_tags,
            ord,
        )
        .unwrap_or_else(|| {
            let (d, i) = crate::diff::hunk_copy_change_counts(
                &self.compare_left_tags,
                &self.compare_right_tags,
                primary,
                line,
            )
            .unwrap_or((0, 0));
            ("replace", d, i)
        });
        Ok(CompareHunkPayload {
            text,
            left_name: lname,
            right_name: rname,
            ordinal: ord,
            total,
            kind,
            deletes: del,
            inserts: ins,
        })
    }

    /// Clipboard: unified diff of the hunk at the focused caret (or the next hunk).
    fn copy_compare_hunk(&mut self, flags: &mut crate::commands::UiFlags) {
        match self.compare_caret_hunk_unified() {
            Ok(p) => {
                flags.pending_clipboard = Some(p.text);
                let ordinal = format!(
                    "({}/{} {} −{} +{})",
                    p.ordinal, p.total, p.kind, p.deletes, p.inserts
                );
                self.state.status =
                    compare_hunk_copy_open_status("Copied", &ordinal, &p.left_name, &p.right_name);
            }
            Err(why) => {
                self.state.status = format!("Copy Compare Hunk: {why}");
            }
        }
    }

    /// New tab with the caret hunk as a unified diff. Clears Compare so the tab is visible.
    fn open_compare_hunk_tab(&mut self) {
        match self.compare_caret_hunk_unified() {
            Ok(p) => {
                let ordinal = format!(
                    "({}/{} {} −{} +{})",
                    p.ordinal, p.total, p.kind, p.deletes, p.inserts
                );
                self.clear_compare();
                self.dual_view = false;
                self.state.tabs.open_untitled();
                {
                    let doc = self.state.tabs.active_mut();
                    doc.title = "compare-hunk.diff".into();
                    doc.buffer = buffer::TextBuffer::from_str(&p.text);
                    doc.dirty = true;
                    doc.language = "plain".into();
                }
                self.state.highlight_dirty = true;
                self.state.reset_view = true;
                self.state.status =
                    compare_hunk_copy_open_status("Opened", &ordinal, &p.left_name, &p.right_name);
            }
            Err(why) => {
                self.state.status = format!("Open Compare Hunk: {why}");
            }
        }
    }

    /// Replace the focused compare hunk with the other pane (one undo).
    fn apply_compare_hunk_from_other(&mut self) {
        if !self.compare_on {
            self.state.status = "Apply Compare Hunk: Compare is off".into();
            return;
        }
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let left_lines = self.tab_compare_lines(left);
        let right_lines = self.tab_compare_lines(right);
        if left_lines.len() != self.compare_left_tags.len()
            || right_lines.len() != self.compare_right_tags.len()
        {
            if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
            } else {
                return;
            }
        }
        let primary = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let dest_tab = if primary { left } else { right };
        let src_tab = if primary { right } else { left };
        if self.state.tabs.get(dest_tab).is_some_and(|d| d.read_only) {
            self.state.status = "Apply Compare Hunk: destination is read-only".into();
            return;
        }
        let Some(doc) = self.state.tabs.get(dest_tab) else {
            self.state.status = "Apply Compare Hunk: tab missing".into();
            return;
        };
        let line = doc.buffer.char_to_line(doc.buffer.caret());
        let Some(spec) = crate::diff::hunk_apply_from_other(
            &self.compare_left_tags,
            &self.compare_right_tags,
            primary,
            line,
        ) else {
            self.state.status = "Apply Compare Hunk: no differences".into();
            return;
        };
        let src_lines = self.tab_compare_lines(src_tab);
        if spec.src_end > src_lines.len() || spec.src_start > spec.src_end {
            self.state.status = "Apply Compare Hunk: hunk out of range".into();
            return;
        }
        let src_slice = &src_lines[spec.src_start..spec.src_end];
        let dest_lines = self.tab_compare_lines(dest_tab);
        if spec.dest_start > dest_lines.len() || spec.dest_end > dest_lines.len() {
            self.state.status = "Apply Compare Hunk: hunk out of range".into();
            return;
        }
        if spec.dest_end <= dest_lines.len()
            && dest_lines[spec.dest_start..spec.dest_end] == src_slice[..]
        {
            self.state.status = format!(
                "Apply Compare Hunk: hunk ({}/{}) already matches other view",
                spec.ordinal, spec.total
            );
            return;
        }
        let applied_ord = spec.ordinal;
        let applied_total = spec.total;
        // Capture kind/−/+ before re-diff (applied hunk disappears from tags).
        let applied_bit = self.compare_hunk_ordinal_bit(applied_ord, applied_total);
        let Some(doc) = self.state.tabs.get_mut(dest_tab) else {
            return;
        };
        apply_compare_hunk_spec(&mut doc.buffer, &spec, src_slice);
        self.state.mark_text_changed_at(dest_tab);
        self.state.compare_stale = false;
        self.compare_refresh_at = None;
        self.follow_caret = true;
        self.state.highlight_dirty = true;
        if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
            self.compare_left_tags = lt;
            self.compare_right_tags = rt;
        }
        // After a successful apply, park on the next remaining hunk (same ordinal
        // slot after re-diff) so the user can keep applying without F7 each time.
        let remaining = crate::diff::hunk_starts(&self.compare_left_tags)
            .len()
            .max(crate::diff::hunk_starts(&self.compare_right_tags).len());
        if let Some(next) = crate::diff::next_hunk_after_apply(applied_ord, remaining) {
            self.park_compare_hunk_ordinal(next);
            self.select_compare_hunk_on_pane(true, next);
            self.select_compare_hunk_on_pane(false, next);
            let lr = self.compare_hunk_lr_label(next);
            let lr_bit = if lr.is_empty() {
                String::new()
            } else {
                format!(" → {lr}")
            };
            let next_bit = self.compare_hunk_ordinal_bit(next, remaining);
            self.state.status =
                format!("Applied hunk {applied_bit} from other view{lr_bit} {next_bit}");
        } else {
            self.state.status = format!("Applied hunk {applied_bit} from other view · identical");
        }
    }

    /// Replace the other pane's compare hunk with the focused pane (one undo).
    fn apply_compare_hunk_to_other(&mut self) {
        if !self.compare_on {
            self.state.status = "Apply Hunk To Other View: Compare is off".into();
            return;
        }
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let left_lines = self.tab_compare_lines(left);
        let right_lines = self.tab_compare_lines(right);
        if left_lines.len() != self.compare_left_tags.len()
            || right_lines.len() != self.compare_right_tags.len()
        {
            if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
            } else {
                return;
            }
        }
        let primary = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let src_tab = if primary { left } else { right };
        let dest_tab = if primary { right } else { left };
        if self.state.tabs.get(dest_tab).is_some_and(|d| d.read_only) {
            self.state.status = "Apply Hunk To Other View: destination is read-only".into();
            return;
        }
        let Some(doc) = self.state.tabs.get(src_tab) else {
            self.state.status = "Apply Hunk To Other View: tab missing".into();
            return;
        };
        let line = doc.buffer.char_to_line(doc.buffer.caret());
        let Some(spec) = crate::diff::hunk_apply_to_other(
            &self.compare_left_tags,
            &self.compare_right_tags,
            primary,
            line,
        ) else {
            self.state.status = "Apply Hunk To Other View: no differences".into();
            return;
        };
        let src_lines = self.tab_compare_lines(src_tab);
        if spec.src_end > src_lines.len() || spec.src_start > spec.src_end {
            self.state.status = "Apply Hunk To Other View: hunk out of range".into();
            return;
        }
        let src_slice = &src_lines[spec.src_start..spec.src_end];
        let dest_lines = self.tab_compare_lines(dest_tab);
        if spec.dest_start > dest_lines.len() || spec.dest_end > dest_lines.len() {
            self.state.status = "Apply Hunk To Other View: hunk out of range".into();
            return;
        }
        if spec.dest_end <= dest_lines.len()
            && dest_lines[spec.dest_start..spec.dest_end] == src_slice[..]
        {
            self.state.status = format!(
                "Apply Hunk To Other View: hunk ({}/{}) already matches focused view",
                spec.ordinal, spec.total
            );
            return;
        }
        let applied_ord = spec.ordinal;
        let applied_total = spec.total;
        // Capture kind/−/+ before re-diff (applied hunk disappears from tags).
        let applied_bit = self.compare_hunk_ordinal_bit(applied_ord, applied_total);
        let Some(doc) = self.state.tabs.get_mut(dest_tab) else {
            return;
        };
        apply_compare_hunk_spec(&mut doc.buffer, &spec, src_slice);
        self.state.mark_text_changed_at(dest_tab);
        self.state.compare_stale = false;
        self.compare_refresh_at = None;
        self.follow_caret = true;
        self.state.highlight_dirty = true;
        if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
            self.compare_left_tags = lt;
            self.compare_right_tags = rt;
        }
        let remaining = crate::diff::hunk_starts(&self.compare_left_tags)
            .len()
            .max(crate::diff::hunk_starts(&self.compare_right_tags).len());
        if let Some(next) = crate::diff::next_hunk_after_apply(applied_ord, remaining) {
            self.park_compare_hunk_ordinal(next);
            self.select_compare_hunk_on_pane(true, next);
            self.select_compare_hunk_on_pane(false, next);
            let lr = self.compare_hunk_lr_label(next);
            let lr_bit = if lr.is_empty() {
                String::new()
            } else {
                format!(" → {lr}")
            };
            let next_bit = self.compare_hunk_ordinal_bit(next, remaining);
            self.state.status =
                format!("Applied hunk {applied_bit} to other view{lr_bit} {next_bit}");
        } else {
            self.state.status = format!("Applied hunk {applied_bit} to other view · identical");
        }
    }

    /// Replace every remaining change hunk on the focused pane (one undo).
    fn apply_all_compare_hunks_from_other(&mut self) {
        if !self.compare_on {
            self.state.status = "Apply All Compare Hunks: Compare is off".into();
            return;
        }
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let left_lines = self.tab_compare_lines(left);
        let right_lines = self.tab_compare_lines(right);
        if left_lines.len() != self.compare_left_tags.len()
            || right_lines.len() != self.compare_right_tags.len()
        {
            if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
            } else {
                return;
            }
        }
        let primary = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let dest_tab = if primary { left } else { right };
        let src_tab = if primary { right } else { left };
        if self.state.tabs.get(dest_tab).is_some_and(|d| d.read_only) {
            self.state.status = "Apply All Compare Hunks: destination is read-only".into();
            return;
        }
        if self.state.tabs.get(dest_tab).is_none() {
            self.state.status = "Apply All Compare Hunks: tab missing".into();
            return;
        }
        let Some(specs) = crate::diff::hunk_apply_all_from_other(
            &self.compare_left_tags,
            &self.compare_right_tags,
            primary,
        ) else {
            self.state.status = "Apply All Compare Hunks: no differences".into();
            return;
        };
        let src_lines = self.tab_compare_lines(src_tab);
        let dest_lines = self.tab_compare_lines(dest_tab);
        let mut pending: Vec<(crate::diff::HunkApply, Vec<String>)> = Vec::new();
        for spec in specs {
            if spec.src_end > src_lines.len() || spec.src_start > spec.src_end {
                self.state.status = "Apply All Compare Hunks: hunk out of range".into();
                return;
            }
            if spec.dest_start > dest_lines.len() || spec.dest_end > dest_lines.len() {
                self.state.status = "Apply All Compare Hunks: hunk out of range".into();
                return;
            }
            let src_slice = src_lines[spec.src_start..spec.src_end].to_vec();
            if spec.dest_end <= dest_lines.len()
                && dest_lines[spec.dest_start..spec.dest_end] == src_slice[..]
            {
                continue;
            }
            pending.push((spec, src_slice));
        }
        if pending.is_empty() {
            self.state.status = "Apply All Compare Hunks: already matches other view".into();
            return;
        }
        let applied = pending.len();
        // Capture kind/−/+ for applied ordinals before re-diff clears tags.
        let applied_ords: Vec<usize> = pending.iter().map(|(s, _)| s.ordinal).collect();
        let applied_bit = crate::diff::compare_apply_all_status_bit(
            &self.compare_left_tags,
            &self.compare_right_tags,
            &applied_ords,
        );
        {
            let Some(doc) = self.state.tabs.get_mut(dest_tab) else {
                return;
            };
            doc.buffer.with_transaction(|buf| {
                for (spec, src_slice) in pending.iter().rev() {
                    apply_compare_hunk_spec(buf, spec, src_slice);
                }
            });
            doc.buffer.set_caret(0);
        }
        self.state.mark_text_changed_at(dest_tab);
        self.state.compare_stale = false;
        self.compare_refresh_at = None;
        self.follow_caret = true;
        self.state.highlight_dirty = true;
        if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
            self.compare_left_tags = lt;
            self.compare_right_tags = rt;
        }
        let hunk_word = if applied == 1 { "hunk" } else { "hunks" };
        let remaining = crate::diff::hunk_starts(&self.compare_left_tags)
            .len()
            .max(crate::diff::hunk_starts(&self.compare_right_tags).len());
        if remaining == 0 {
            self.state.status =
                format!("Applied {applied} {hunk_word}{applied_bit} from other view · identical");
        } else {
            self.state.status =
                format!("Applied {applied} {hunk_word}{applied_bit} from other view");
        }
    }

    /// Replace every remaining change hunk on the other pane (one undo).
    fn apply_all_compare_hunks_to_other(&mut self) {
        if !self.compare_on {
            self.state.status = "Apply All Hunks To Other View: Compare is off".into();
            return;
        }
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let left_lines = self.tab_compare_lines(left);
        let right_lines = self.tab_compare_lines(right);
        if left_lines.len() != self.compare_left_tags.len()
            || right_lines.len() != self.compare_right_tags.len()
        {
            if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
            } else {
                return;
            }
        }
        let primary = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let src_tab = if primary { left } else { right };
        let dest_tab = if primary { right } else { left };
        if self.state.tabs.get(dest_tab).is_some_and(|d| d.read_only) {
            self.state.status = "Apply All Hunks To Other View: destination is read-only".into();
            return;
        }
        if self.state.tabs.get(dest_tab).is_none() {
            self.state.status = "Apply All Hunks To Other View: tab missing".into();
            return;
        }
        let Some(specs) = crate::diff::hunk_apply_all_to_other(
            &self.compare_left_tags,
            &self.compare_right_tags,
            primary,
        ) else {
            self.state.status = "Apply All Hunks To Other View: no differences".into();
            return;
        };
        let src_lines = self.tab_compare_lines(src_tab);
        let dest_lines = self.tab_compare_lines(dest_tab);
        let mut pending: Vec<(crate::diff::HunkApply, Vec<String>)> = Vec::new();
        for spec in specs {
            if spec.src_end > src_lines.len() || spec.src_start > spec.src_end {
                self.state.status = "Apply All Hunks To Other View: hunk out of range".into();
                return;
            }
            if spec.dest_start > dest_lines.len() || spec.dest_end > dest_lines.len() {
                self.state.status = "Apply All Hunks To Other View: hunk out of range".into();
                return;
            }
            let src_slice = src_lines[spec.src_start..spec.src_end].to_vec();
            if spec.dest_end <= dest_lines.len()
                && dest_lines[spec.dest_start..spec.dest_end] == src_slice[..]
            {
                continue;
            }
            pending.push((spec, src_slice));
        }
        if pending.is_empty() {
            self.state.status =
                "Apply All Hunks To Other View: already matches focused view".into();
            return;
        }
        let applied = pending.len();
        // Capture kind/−/+ for applied ordinals before re-diff clears tags.
        let applied_ords: Vec<usize> = pending.iter().map(|(s, _)| s.ordinal).collect();
        let applied_bit = crate::diff::compare_apply_all_status_bit(
            &self.compare_left_tags,
            &self.compare_right_tags,
            &applied_ords,
        );
        {
            let Some(doc) = self.state.tabs.get_mut(dest_tab) else {
                return;
            };
            doc.buffer.with_transaction(|buf| {
                for (spec, src_slice) in pending.iter().rev() {
                    apply_compare_hunk_spec(buf, spec, src_slice);
                }
            });
            doc.buffer.set_caret(0);
        }
        self.state.mark_text_changed_at(dest_tab);
        self.state.compare_stale = false;
        self.compare_refresh_at = None;
        self.follow_caret = true;
        self.state.highlight_dirty = true;
        if let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) {
            self.compare_left_tags = lt;
            self.compare_right_tags = rt;
        }
        let hunk_word = if applied == 1 { "hunk" } else { "hunks" };
        let remaining = crate::diff::hunk_starts(&self.compare_left_tags)
            .len()
            .max(crate::diff::hunk_starts(&self.compare_right_tags).len());
        if remaining == 0 {
            self.state.status =
                format!("Applied {applied} {hunk_word}{applied_bit} to other view · identical");
        } else {
            self.state.status = format!("Applied {applied} {hunk_word}{applied_bit} to other view");
        }
    }

    /// Park both compare panes on the same 1-based hunk ordinal (carets + follow).
    fn park_compare_hunk_ordinal(&mut self, ordinal_1based: usize) {
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        if let Some(line) =
            crate::diff::hunk_start_at_ordinal(&self.compare_left_tags, ordinal_1based)
        {
            if let Some(doc) = self.state.tabs.get_mut(left) {
                let at = doc
                    .buffer
                    .line_to_char(line.min(doc.buffer.line_count().saturating_sub(1)));
                doc.buffer.set_caret(at);
            }
            self.follow_caret = true;
        }
        if let Some(line) =
            crate::diff::hunk_start_at_ordinal(&self.compare_right_tags, ordinal_1based)
        {
            if let Some(doc) = self.state.tabs.get_mut(right) {
                let at = doc
                    .buffer
                    .line_to_char(line.min(doc.buffer.line_count().saturating_sub(1)));
                doc.buffer.set_caret(at);
            }
            self.follow_caret_other = true;
        }
    }

    /// Format `L12 | R15` for a hunk ordinal (omits a side that has no line).
    fn compare_hunk_lr_label(&self, ordinal_1based: usize) -> String {
        let left = crate::diff::hunk_start_at_ordinal(&self.compare_left_tags, ordinal_1based)
            .map(|l| format!("L{}", l + 1));
        let right = crate::diff::hunk_start_at_ordinal(&self.compare_right_tags, ordinal_1based)
            .map(|l| format!("R{}", l + 1));
        match (left, right) {
            (Some(l), Some(r)) => format!("{l} | {r}"),
            (Some(l), None) => l,
            (None, Some(r)) => r,
            (None, None) => String::new(),
        }
    }

    /// Landing on a change line (click or keyboard) parks the other pane on the same hunk ordinal.
    /// Equal lines park the other pane on the LCS-aligned partner (no hunk selection).
    fn sync_compare_other_to_caret_hunk(&mut self, primary: bool) {
        if !self.compare_on {
            return;
        }
        let tab = if primary {
            self.compare_left_tab
        } else {
            self.compare_right_tab
        };
        let Some(doc) = self.state.tabs.get(tab) else {
            return;
        };
        let line = doc.buffer.char_to_line(doc.buffer.caret());
        let Some((ord, total)) = ({
            let tags = if primary {
                &self.compare_left_tags
            } else {
                &self.compare_right_tags
            };
            crate::diff::hunk_ordinal(tags, line)
        }) else {
            self.sync_compare_other_to_equal_line(primary, line);
            return;
        };
        let other_tab = if primary {
            self.compare_right_tab
        } else {
            self.compare_left_tab
        };
        let other_line = {
            let tags = if primary {
                &self.compare_right_tags
            } else {
                &self.compare_left_tags
            };
            crate::diff::hunk_start_at_ordinal(tags, ord)
        };
        if let Some(oline) = other_line {
            if let Some(doc) = self.state.tabs.get_mut(other_tab) {
                let at = doc
                    .buffer
                    .line_to_char(oline.min(doc.buffer.line_count().saturating_sub(1)));
                doc.buffer.set_caret(at);
            }
            if primary {
                self.follow_caret_other = true;
            } else {
                self.follow_caret = true;
            }
        }
        // Select the hunk on both panes so click/keyboard landing shows both extents.
        self.select_compare_hunk_on_pane(true, ord);
        self.select_compare_hunk_on_pane(false, ord);
        let lr = self.compare_hunk_lr_label(ord);
        let ordinal = self.compare_hunk_ordinal_bit(ord, total);
        if lr.is_empty() {
            self.state.status = format!("Compare hunk {ordinal}");
        } else {
            self.state.status = format!("Compare hunk → {lr} {ordinal}");
        }
    }

    /// Status ordinal bit including hunk kind and −/+ counts, e.g. `(2/5 replace −1 +1)`.
    fn compare_hunk_ordinal_bit(&self, ord: usize, total: usize) -> String {
        match crate::diff::compare_hunk_kind_and_counts(
            &self.compare_left_tags,
            &self.compare_right_tags,
            ord,
        ) {
            Some((kind, del, ins)) => format!("({ord}/{total} {kind} −{del} +{ins})"),
            None => format!("({ord}/{total})"),
        }
    }

    /// Park the other Compare pane on the LCS-aligned Equal partner of `line`.
    fn sync_compare_other_to_equal_line(&mut self, primary: bool, line: usize) {
        let Some(oline) = self.park_compare_other_on_equal_line(primary, line) else {
            return;
        };
        let (l, r) = if primary {
            (line + 1, oline + 1)
        } else {
            (oline + 1, line + 1)
        };
        let counts =
            compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
        let ignore = CompareIgnoreBits {
            ws: self.state.settings.compare_ignore_ws,
            case: self.state.settings.compare_ignore_case,
            blank: self.state.settings.compare_ignore_blank,
        };
        let hide = compare_hide_opt(
            self.state.settings.compare_hide_equal,
            &self.compare_left_tags,
            &self.compare_right_tags,
            &self.compare_hide_revealed_left,
            &self.compare_hide_revealed_right,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        self.state.status = compare_equal_park_status(
            l,
            r,
            &counts,
            ignore,
            CompareHideStatus {
                hidden: hide,
                context: self.state.settings.compare_hide_equal_context_lines(),
            },
        );
    }

    /// Flip left/right compare panes; primary stays focused on the new left.
    fn swap_compare_sides(&mut self) {
        if !self.compare_on {
            self.state.status = "Compare is off — View → Compare with Other View first".into();
            return;
        }
        std::mem::swap(&mut self.compare_left_tab, &mut self.compare_right_tab);
        std::mem::swap(&mut self.scroll_line, &mut self.scroll_line_other);
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) else {
            // Restore orientation if re-diff cannot run (e.g. over line cap).
            std::mem::swap(&mut self.compare_left_tab, &mut self.compare_right_tab);
            std::mem::swap(&mut self.scroll_line, &mut self.scroll_line_other);
            return;
        };
        self.compare_left_tags = lt;
        self.compare_right_tags = rt;
        self.compare_hide_revealed_left.clear();
        self.compare_hide_revealed_right.clear();
        self.dual_view = true;
        self.other_view_tab = right;
        self.state.tabs.set_active(left);
        self.focused_pane = EditorPane::Primary;
        self.state.highlight_dirty = true;
        self.state.compare_stale = false;
        self.compare_refresh_at = None;
        let lname = self
            .state
            .tabs
            .get(left)
            .map(|d| d.title.clone())
            .unwrap_or_else(|| "left".into());
        let rname = self
            .state
            .tabs
            .get(right)
            .map(|d| d.title.clone())
            .unwrap_or_else(|| "right".into());
        let ignore = CompareIgnoreBits {
            ws: self.state.settings.compare_ignore_ws,
            case: self.state.settings.compare_ignore_case,
            blank: self.state.settings.compare_ignore_blank,
        };
        let hide = compare_hide_opt(
            self.state.settings.compare_hide_equal,
            &self.compare_left_tags,
            &self.compare_right_tags,
            &self.compare_hide_revealed_left,
            &self.compare_hide_revealed_right,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let counts =
            compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
        let hunk_n = counts.hunk_n;
        // Swap flips delete/insert colours — re-park like Compare start / ignore re-diff.
        if hunk_n > 0 {
            self.park_compare_hunk_ordinal(1);
            self.select_compare_hunk_on_pane(true, 1);
            self.select_compare_hunk_on_pane(false, 1);
        }
        let mut status = compare_pair_status(
            &lname,
            &rname,
            counts,
            ignore,
            CompareHideStatus {
                hidden: hide,
                context: self.state.settings.compare_hide_equal_context_lines(),
            },
        );
        if hunk_n > 0 {
            let lr = self.compare_hunk_lr_label(1);
            let ordinal = self.compare_hunk_ordinal_bit(1, hunk_n);
            status.push_str(&compare_at_hunk_status_bit(&lr, &ordinal));
        }
        self.state.status = format!("Swapped sides — {status}");
    }

    /// Jump caret (focused pane) to a compare change hunk.
    /// Also parks the other pane on the same hunk ordinal so both sides line up.
    fn navigate_compare_hunk(&mut self, nav: crate::commands::CompareNav) {
        if !self.compare_on {
            self.state.status = "Compare is off — View → Compare with Other View first".into();
            return;
        }
        let primary = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let tags = if primary {
            &self.compare_left_tags
        } else {
            &self.compare_right_tags
        };
        let tab = if primary {
            self.compare_left_tab
        } else {
            self.compare_right_tab
        };
        let Some(doc) = self.state.tabs.get(tab) else {
            self.state.status = "Compare: tab missing".into();
            return;
        };
        let from = doc.buffer.char_to_line(doc.buffer.caret());
        let starts = crate::diff::hunk_starts(tags);
        let target = match nav {
            crate::commands::CompareNav::Next => crate::diff::next_hunk_start(tags, from),
            crate::commands::CompareNav::Prev => crate::diff::prev_hunk_start(tags, from),
            crate::commands::CompareNav::First => starts.first().copied(),
            crate::commands::CompareNav::Last => starts.last().copied(),
        };
        let Some(line) = target else {
            self.state.status = "Compare: no differences".into();
            return;
        };
        let ordinal_pair = crate::diff::hunk_ordinal(tags, line);
        if let Some((ord, _)) = ordinal_pair {
            self.park_compare_hunk_ordinal(ord);
            // Select the whole hunk on both panes so each side shows the change extent
            // (Copy/Delete still apply to the focused pane).
            self.select_compare_hunk_on_pane(true, ord);
            self.select_compare_hunk_on_pane(false, ord);
        } else if let Some(doc) = self.state.tabs.get_mut(tab) {
            let at = doc.buffer.line_to_char(line);
            doc.buffer.set_caret(at);
        }
        if primary {
            self.state.tabs.set_active(tab);
            self.focused_pane = EditorPane::Primary;
            self.follow_caret = true;
            if self.sync_scroll_v {
                self.follow_caret_other = true;
            }
        } else {
            self.other_view_tab = tab;
            self.focused_pane = EditorPane::Secondary;
            self.follow_caret_other = true;
            if self.sync_scroll_v {
                self.follow_caret = true;
            }
        }
        let dir = match nav {
            crate::commands::CompareNav::Next => "Next",
            crate::commands::CompareNav::Prev => "Previous",
            crate::commands::CompareNav::First => "First",
            crate::commands::CompareNav::Last => "Last",
        };
        let ordinal = ordinal_pair
            .map(|(i, n)| format!(" {}", self.compare_hunk_ordinal_bit(i, n)))
            .unwrap_or_default();
        let lr = ordinal_pair
            .map(|(ord, _)| self.compare_hunk_lr_label(ord))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| format!("line {}", line + 1));
        let wrapped = match nav {
            crate::commands::CompareNav::Next => crate::diff::hunk_nav_wrapped(true, from, line),
            crate::commands::CompareNav::Prev => crate::diff::hunk_nav_wrapped(false, from, line),
            crate::commands::CompareNav::First | crate::commands::CompareNav::Last => false,
        };
        let wrap_bit = if wrapped { " · wrapped" } else { "" };
        self.state.status = format!("Compare {dir} difference → {lr}{ordinal}{wrap_bit}");
    }

    /// Select change lines for a hunk ordinal on one compare pane.
    fn select_compare_hunk_on_pane(&mut self, primary: bool, ordinal_1based: usize) {
        let (tab, range) = if primary {
            (
                self.compare_left_tab,
                crate::diff::hunk_line_range(&self.compare_left_tags, ordinal_1based),
            )
        } else {
            (
                self.compare_right_tab,
                crate::diff::hunk_line_range(&self.compare_right_tags, ordinal_1based),
            )
        };
        let Some((start, end)) = range else {
            return;
        };
        let Some(doc) = self.state.tabs.get_mut(tab) else {
            return;
        };
        let line_count = doc.buffer.line_count();
        if line_count == 0 {
            return;
        }
        let start = start.min(line_count.saturating_sub(1));
        let anchor = doc.buffer.line_to_char(start);
        let caret = if end >= line_count {
            doc.buffer.len_chars()
        } else {
            doc.buffer.line_to_char(end)
        };
        if caret > anchor {
            doc.buffer.set_selection(anchor, caret);
        } else {
            doc.buffer.set_caret(anchor);
        }
    }

    fn clear_compare(&mut self) {
        // Capture pair overview before dropping tags so Clear can report −/+/kind tallies.
        let overview = if self.compare_on {
            let left = self.compare_left_tab;
            let right = self.compare_right_tab;
            let lname = self
                .state
                .tabs
                .get(left)
                .map(|d| Self::compare_side_name(&d.title))
                .unwrap_or_else(|| "left".into());
            let rname = self
                .state
                .tabs
                .get(right)
                .map(|d| Self::compare_side_name(&d.title))
                .unwrap_or_else(|| "right".into());
            let counts =
                compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
            Some((lname, rname, counts))
        } else {
            None
        };
        // Drop the Compare-to-Saved snapshot with the colours so it does not linger.
        let snapshot = if self.compare_on {
            [self.compare_left_tab, self.compare_right_tab]
                .into_iter()
                .find(|&i| {
                    self.state
                        .tabs
                        .get(i)
                        .is_some_and(is_compare_saved_snapshot)
                })
        } else {
            None
        };
        self.compare_on = false;
        self.compare_left_tags.clear();
        self.compare_right_tags.clear();
        self.compare_left_inline.clear();
        self.compare_right_inline.clear();
        self.compare_hide_revealed_left.clear();
        self.compare_hide_revealed_right.clear();
        self.state.compare_stale = false;
        self.compare_refresh_at = None;
        let mut closed_saved = false;
        if let Some(i) = snapshot {
            if self
                .state
                .tabs
                .get(i)
                .is_some_and(is_compare_saved_snapshot)
            {
                self.state.close_tab(i);
                closed_saved = true;
            }
        }
        if let Some((lname, rname, counts)) = overview {
            self.state.status = compare_cleared_status(
                closed_saved,
                &lname,
                &rname,
                counts.del,
                counts.ins,
                counts.hunk_n,
                &counts.kind_bit,
            );
            return;
        }
        self.state.status = if closed_saved {
            "Compare cleared (closed saved snapshot)".into()
        } else {
            "Compare cleared".into()
        };
    }

    /// Rebuild compare colours after an edit (debounce ~200 ms while typing).
    fn refresh_compare_if_stale(&mut self, ctx: &egui::Context) {
        if !self.compare_on || !self.state.compare_stale {
            self.compare_refresh_at = None;
            return;
        }
        let now = std::time::Instant::now();
        let due = *self
            .compare_refresh_at
            .get_or_insert_with(|| now + std::time::Duration::from_millis(200));
        if now < due {
            ctx.request_repaint_after(due.saturating_duration_since(now));
            return;
        }
        self.compare_refresh_at = None;
        self.state.compare_stale = false;
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let n = self.state.tabs.len();
        if left >= n || right >= n || left == right {
            self.clear_compare();
            return;
        }
        match self.compute_compare_tags(left, right) {
            Some((lt, rt, _, _)) => {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
                crate::diff::prune_compare_hide_revealed(
                    &mut self.compare_hide_revealed_left,
                    &self.compare_left_tags,
                    self.state.settings.compare_hide_equal_context_lines(),
                );
                crate::diff::prune_compare_hide_revealed(
                    &mut self.compare_hide_revealed_right,
                    &self.compare_right_tags,
                    self.state.settings.compare_hide_equal_context_lines(),
                );
                let lname = self
                    .state
                    .tabs
                    .get(left)
                    .map(|d| d.title.clone())
                    .unwrap_or_else(|| "left".into());
                let rname = self
                    .state
                    .tabs
                    .get(right)
                    .map(|d| d.title.clone())
                    .unwrap_or_else(|| "right".into());
                let ignore = CompareIgnoreBits {
                    ws: self.state.settings.compare_ignore_ws,
                    case: self.state.settings.compare_ignore_case,
                    blank: self.state.settings.compare_ignore_blank,
                };
                let hide = compare_hide_opt(
                    self.state.settings.compare_hide_equal,
                    &self.compare_left_tags,
                    &self.compare_right_tags,
                    &self.compare_hide_revealed_left,
                    &self.compare_hide_revealed_right,
                    self.state.settings.compare_hide_equal_context_lines(),
                );
                let mut status = compare_pair_status(
                    &lname,
                    &rname,
                    compare_pair_counts_from_tags(
                        &self.compare_left_tags,
                        &self.compare_right_tags,
                    ),
                    ignore,
                    CompareHideStatus {
                        hidden: hide,
                        context: self.state.settings.compare_hide_equal_context_lines(),
                    },
                );
                // Keep the other pane on the focused caret's hunk/equal partner after
                // edit re-diff. Do not move the focused caret or force hunk selection
                // (that would fight typing).
                self.align_compare_partner_after_edit_rediff(&mut status);
                self.state.status = status;
            }
            None => {
                // Too many lines or missing tabs — leave prior tags; status already set.
            }
        }
    }

    /// After a debounce re-diff, park the other pane on the focused caret's hunk
    /// (or Equal partner) and append `· at L|R …` / `· equal L|R` landing bits.
    fn align_compare_partner_after_edit_rediff(&mut self, status: &mut String) {
        let primary = self.focused_pane == EditorPane::Primary || !self.dual_view;
        let tab = if primary {
            self.compare_left_tab
        } else {
            self.compare_right_tab
        };
        let Some(doc) = self.state.tabs.get(tab) else {
            return;
        };
        let line = doc.buffer.char_to_line(doc.buffer.caret());
        let Some((ord, total)) = ({
            let tags = if primary {
                &self.compare_left_tags
            } else {
                &self.compare_right_tags
            };
            crate::diff::hunk_ordinal(tags, line)
        }) else {
            let Some(oline) = self.park_compare_other_on_equal_line(primary, line) else {
                return;
            };
            let (l, r) = if primary {
                (line + 1, oline + 1)
            } else {
                (oline + 1, line + 1)
            };
            status.push_str(&format!(" · equal L{l} | R{r}"));
            return;
        };
        let other_tab = if primary {
            self.compare_right_tab
        } else {
            self.compare_left_tab
        };
        let other_line = {
            let tags = if primary {
                &self.compare_right_tags
            } else {
                &self.compare_left_tags
            };
            crate::diff::hunk_start_at_ordinal(tags, ord)
        };
        if let Some(oline) = other_line {
            if let Some(doc) = self.state.tabs.get_mut(other_tab) {
                let at = doc
                    .buffer
                    .line_to_char(oline.min(doc.buffer.line_count().saturating_sub(1)));
                doc.buffer.set_caret(at);
            }
            if primary {
                self.follow_caret_other = true;
            } else {
                self.follow_caret = true;
            }
        }
        let lr = self.compare_hunk_lr_label(ord);
        let ordinal = self.compare_hunk_ordinal_bit(ord, total);
        status.push_str(&compare_at_hunk_status_bit(&lr, &ordinal));
    }

    /// Park only the other Compare pane on the LCS-aligned Equal partner of `line`.
    ///
    /// When Hide Unchanged Lines collapses that partner (or the focused Equal),
    /// reveal those lines so the parked caret stays visible. Returns the partner
    /// document line when a park happened.
    fn park_compare_other_on_equal_line(&mut self, primary: bool, line: usize) -> Option<usize> {
        let oline = crate::diff::aligned_equal_partner_line(
            &self.compare_left_tags,
            &self.compare_right_tags,
            line,
            primary,
        )?;
        if self.state.settings.compare_hide_equal {
            let context = self.state.settings.compare_hide_equal_context_lines();
            let (focus_tags, other_tags, focus_revealed, other_revealed) = if primary {
                (
                    &self.compare_left_tags,
                    &self.compare_right_tags,
                    &mut self.compare_hide_revealed_left,
                    &mut self.compare_hide_revealed_right,
                )
            } else {
                (
                    &self.compare_right_tags,
                    &self.compare_left_tags,
                    &mut self.compare_hide_revealed_right,
                    &mut self.compare_hide_revealed_left,
                )
            };
            if crate::diff::compare_hide_line_is_collapsed(
                focus_tags,
                focus_revealed,
                context,
                line,
            ) {
                crate::diff::reveal_compare_lines(focus_revealed, &[line]);
            }
            if crate::diff::compare_hide_line_is_collapsed(
                other_tags,
                other_revealed,
                context,
                oline,
            ) {
                crate::diff::reveal_compare_lines(other_revealed, &[oline]);
            }
        }
        let other_tab = if primary {
            self.compare_right_tab
        } else {
            self.compare_left_tab
        };
        if let Some(doc) = self.state.tabs.get_mut(other_tab) {
            let at = doc
                .buffer
                .line_to_char(oline.min(doc.buffer.line_count().saturating_sub(1)));
            doc.buffer.set_caret(at);
        }
        if primary {
            self.follow_caret_other = true;
        } else {
            self.follow_caret = true;
        }
        Some(oline)
    }

    fn compute_compare_tags(
        &mut self,
        left: usize,
        right: usize,
    ) -> Option<(
        Vec<crate::diff::LineKind>,
        Vec<crate::diff::LineKind>,
        usize,
        usize,
    )> {
        let left_lines = self.tab_compare_lines(left);
        let right_lines = self.tab_compare_lines(right);
        if left_lines.len() > crate::diff::MAX_COMPARE_LINES
            || right_lines.len() > crate::diff::MAX_COMPARE_LINES
        {
            self.state.status = format!(
                "Compare MVP max is {} lines per side",
                crate::diff::MAX_COMPARE_LINES
            );
            return None;
        }
        let ignore_ws = self.state.settings.compare_ignore_ws;
        let ignore_case = self.state.settings.compare_ignore_case;
        let ignore_blank = self.state.settings.compare_ignore_blank;
        let left_keys: Vec<String> = left_lines
            .iter()
            .map(|s| crate::diff::compare_line_key(s, ignore_ws, ignore_case))
            .collect();
        let right_keys: Vec<String> = right_lines
            .iter()
            .map(|s| crate::diff::compare_line_key(s, ignore_ws, ignore_case))
            .collect();
        let left_refs: Vec<&str> = left_keys.iter().map(|s| s.as_str()).collect();
        let right_refs: Vec<&str> = right_keys.iter().map(|s| s.as_str()).collect();
        let (lt, rt) =
            crate::diff::diff_line_tags_ignore_blank(&left_refs, &right_refs, ignore_blank);
        let (del, ins) = crate::diff::count_changes(&lt, &rt);
        let left_orig: Vec<&str> = left_lines.iter().map(|s| s.as_str()).collect();
        let right_orig: Vec<&str> = right_lines.iter().map(|s| s.as_str()).collect();
        let (li, ri) = crate::diff::inline_change_spans(&left_orig, &right_orig, &lt, &rt);
        self.compare_left_inline = li;
        self.compare_right_inline = ri;
        Some((lt, rt, del, ins))
    }

    /// Pick the right-hand tab for Compare.
    ///
    /// Order: marked partner → dual-view other pane → tab to the right → tab to the left.
    fn resolve_compare_right(&self) -> Option<usize> {
        pick_compare_right(
            self.state.tabs.len(),
            self.state.tabs.active_index(),
            self.compare_partner_tab,
            self.dual_view,
            self.other_view_tab,
        )
    }

    /// Diff the active named tab against a read-only snapshot of its on-disk bytes.
    fn compare_to_saved(&mut self) {
        let left = self.state.tabs.active_index();
        let Some(doc) = self.state.tabs.get(left) else {
            return;
        };
        let Some(path) = doc.path.clone() else {
            self.state.status = "Compare to Saved: save the file first".into();
            return;
        };
        let language = doc.language.clone();
        let encoding = doc.encoding;
        let name = crate::recent::short_path_label(&path);
        let title = compare_saved_snapshot_title(&name);
        let content = match ::fs::read_file(&path) {
            Ok(r) => r.content,
            Err(_) => {
                self.state.status = format!("Compare to Saved: could not read “{name}”");
                return;
            }
        };
        let existing = self
            .state
            .tabs
            .iter()
            .enumerate()
            .find(|(_, d)| d.title == title && d.read_only && d.path.is_none())
            .map(|(i, _)| i);
        let saved_tab = if let Some(i) = existing {
            if let Some(snap) = self.state.tabs.get_mut(i) {
                snap.buffer = buffer::TextBuffer::from_str(&content);
                snap.language = language;
                snap.encoding = encoding;
                snap.read_only = true;
                snap.mark_clean();
            }
            i
        } else {
            self.state.tabs.open_untitled();
            let i = self.state.tabs.active_index();
            {
                let snap = self.state.tabs.active_mut();
                snap.title = title;
                snap.buffer = buffer::TextBuffer::from_str(&content);
                snap.language = language;
                snap.encoding = encoding;
                snap.read_only = true;
                snap.mark_clean();
            }
            i
        };
        self.state.tabs.set_active(left);
        self.compare_partner_tab = Some(saved_tab);
        self.state.highlight_dirty = true;
        self.start_compare();
    }

    fn start_compare(&mut self) {
        if self.state.tabs.len() < 2 {
            self.state.status = "Compare needs two open tabs".into();
            return;
        }
        let left = self.state.tabs.active_index();
        let Some(right) = self.resolve_compare_right() else {
            self.state.status = "Compare: pick a different tab first".into();
            return;
        };
        let Some((lt, rt, _, _)) = self.compute_compare_tags(left, right) else {
            return;
        };
        self.compare_on = true;
        self.state.compare_stale = false;
        self.compare_left_tab = left;
        self.compare_right_tab = right;
        self.compare_left_tags = lt;
        self.compare_right_tags = rt;
        self.compare_hide_revealed_left.clear();
        self.compare_hide_revealed_right.clear();
        self.compare_partner_tab = None;
        self.dual_view = true;
        self.sync_scroll_v = true;
        self.sync_scroll_h = true;
        self.other_view_tab = right;
        self.state.tabs.set_active(left);
        self.state.highlight_dirty = true;
        self.focused_pane = EditorPane::Primary;
        // Park + select both panes on the first change hunk (same ordinal; line numbers may differ).
        let counts =
            compare_pair_counts_from_tags(&self.compare_left_tags, &self.compare_right_tags);
        if counts.hunk_n > 0 {
            self.park_compare_hunk_ordinal(1);
            self.select_compare_hunk_on_pane(true, 1);
            self.select_compare_hunk_on_pane(false, 1);
        }
        let lname = self
            .state
            .tabs
            .get(left)
            .map(|d| d.title.clone())
            .unwrap_or_else(|| "left".into());
        let rname = self
            .state
            .tabs
            .get(right)
            .map(|d| d.title.clone())
            .unwrap_or_else(|| "right".into());
        let ignore = CompareIgnoreBits {
            ws: self.state.settings.compare_ignore_ws,
            case: self.state.settings.compare_ignore_case,
            blank: self.state.settings.compare_ignore_blank,
        };
        let hide = compare_hide_opt(
            self.state.settings.compare_hide_equal,
            &self.compare_left_tags,
            &self.compare_right_tags,
            &self.compare_hide_revealed_left,
            &self.compare_hide_revealed_right,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let hunk_n = counts.hunk_n;
        let mut status = compare_pair_status(
            &lname,
            &rname,
            counts,
            ignore,
            CompareHideStatus {
                hidden: hide,
                context: self.state.settings.compare_hide_equal_context_lines(),
            },
        );
        // Start parks both panes on the first change; surface that landing in status.
        if hunk_n > 0 {
            let lr = self.compare_hunk_lr_label(1);
            let ordinal = self.compare_hunk_ordinal_bit(1, hunk_n);
            status.push_str(&compare_at_hunk_status_bit(&lr, &ordinal));
        }
        self.state.status = status;
    }

    /// Writable secondary pane: edits `other_view_tab` when this pane has focus.
    fn paint_secondary_pane(&mut self, ui: &mut egui::Ui) {
        let tab = self.other_view_tab;
        let (loading, buf_line_count, hidden) = match self.state.tabs.get(tab) {
            None => {
                ui.label("No tab for other view");
                return;
            }
            Some(doc) => (
                doc.loading,
                doc.buffer.line_count().max(1),
                doc.hidden_lines.clone(),
            ),
        };
        if loading {
            ui.label("Loading…");
            return;
        }

        let font_id = FontId::monospace(self.font_size);
        let row_height = ui.fonts(|f| f.row_height(&font_id)) + 2.0;
        let visible_lines = visible_lines_with_compare_hide(
            buf_line_count,
            &hidden,
            self.compare_on,
            self.state.settings.compare_hide_equal,
            &self.compare_right_tags,
            &self.compare_hide_revealed_right,
            self.state.settings.compare_hide_equal_context_lines(),
        );
        let display_count = visible_lines.len().max(1);
        let avail = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(avail, Sense::click_and_drag());

        if response.clicked() || response.drag_started() {
            response.request_focus();
            self.focused_pane = EditorPane::Secondary;
        }

        let visible_rows = {
            let usable = (rect.height() - row_height).max(row_height);
            ((usable / row_height).floor() as usize).max(1)
        };
        let max_scroll = (display_count.saturating_sub(visible_rows) as f32).max(0.0);

        let sync = self.sync_scroll_v || self.sync_scroll_h;
        let mut scroll_line = if sync {
            self.scroll_line
        } else {
            self.scroll_line_other
        };

        let scroll = if response.hovered() {
            ui.input(|i| {
                if i.modifiers.command || i.modifiers.ctrl {
                    0.0
                } else {
                    i.raw_scroll_delta.y
                }
            })
        } else {
            0.0
        };
        if scroll != 0.0 {
            self.follow_caret_other = false;
            scroll_line = (scroll_line - scroll / row_height).clamp(0.0, max_scroll);
            if sync {
                self.scroll_line = scroll_line;
            }
            self.scroll_line_other = scroll_line;
        } else {
            scroll_line = scroll_line.clamp(0.0, max_scroll);
            if sync {
                self.scroll_line_other = self.scroll_line;
                scroll_line = self.scroll_line_other;
            }
        }

        let show_ln = self.state.settings.show_line_numbers;
        let show_fold = self.state.settings.show_fold_margin;
        let fold_w = if show_fold { FOLD_MARGIN_W } else { 0.0 };
        let gutter_w = if show_ln { 48.0 } else { 12.0 }
            + fold_w
            + f32::from(self.state.settings.gutter_extra);
        let gutter_gap = 8.0;
        let text_left = rect.left() + gutter_w + gutter_gap;
        let gutter_right = rect.left() + gutter_w;
        let fold_left = gutter_right - fold_w;

        let hit_index = |ui: &egui::Ui,
                         pos: Pos2,
                         buf: &buffer::TextBuffer,
                         scroll: f32,
                         visible: &[usize]|
         -> usize {
            let first = scroll.floor() as usize;
            let row = first + ((pos.y - rect.top()) / row_height).floor().max(0.0) as usize;
            let row = row.min(visible.len().saturating_sub(1));
            let line = visible.get(row).copied().unwrap_or(0);
            let line_start = buf.line_to_char(line);
            let line_text = buf.line(line);
            let line_body = line_text.trim_end_matches(['\n', '\r']);
            let col = col_from_x(ui, &font_id, line_body, pos.x - text_left);
            line_start + col
        };

        let fold_line_at = |pos: Pos2| -> Option<usize> {
            if !show_fold || fold_w <= 0.0 || pos.x < fold_left || pos.x >= gutter_right {
                return None;
            }
            let first = scroll_line.floor() as usize;
            let row = first + ((pos.y - rect.top()) / row_height).floor().max(0.0) as usize;
            let row = row.min(visible_lines.len().saturating_sub(1));
            visible_lines.get(row).copied()
        };

        let mut fold_click = false;
        let mut hide_gap_click = false;
        if response.clicked() || response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if self.try_expand_compare_hide_gap_at(
                    pos,
                    rect,
                    scroll_line,
                    row_height,
                    &visible_lines,
                    false,
                ) {
                    hide_gap_click = true;
                    self.drag_anchor = None;
                    self.rect_drag = false;
                    self.sel_text_drag = None;
                } else if let Some(line) = fold_line_at(pos) {
                    if let Some(doc) = self.state.tabs.get(tab) {
                        let lang = doc.language.clone();
                        let regions = crate::fold::compute_fold_regions(lang.as_str(), &doc.buffer);
                        if let Some(region) = crate::fold::region_for_fold_action(&regions, line) {
                            if let Some(doc) = self.state.tabs.get_mut(tab) {
                                let was = crate::fold::is_folded(&doc.hidden_lines, &region);
                                crate::fold::toggle_region(&mut doc.hidden_lines, &region);
                                self.state.status = if was {
                                    format!("Unfolded {} line(s)", region.end - region.header)
                                } else {
                                    format!("Folded {} line(s)", region.end - region.header)
                                };
                            }
                            fold_click = true;
                            self.drag_anchor = None;
                            self.rect_drag = false;
                            self.sel_text_drag = None;
                        }
                    }
                }
            }
        }

        if fold_click || hide_gap_click {
            // Fold margin or hide-equal ···N cue consumed the pointer.
        } else if response.triple_clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                        doc.buffer.select_line_at(idx);
                    }
                }
                self.drag_anchor = None;
                self.rect_drag = false;
                self.sel_text_drag = None;
                self.follow_caret_other = false;
            }
        } else if response.double_clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                        doc.buffer.select_word_at(idx);
                    }
                }
                self.drag_anchor = None;
                self.rect_drag = false;
                self.sel_text_drag = None;
                self.follow_caret_other = false;
            }
        } else if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    let (shift, alt) = ui.input(|i| (i.modifiers.shift, i.modifiers.alt));
                    let caret = doc.buffer.caret();
                    let sel_anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(caret);
                    let inside_sel = doc
                        .buffer
                        .selection()
                        .is_some_and(|(s, e)| idx >= s && idx < e);
                    let read_only = doc.read_only;
                    if alt {
                        self.sel_text_drag = None;
                        self.rect_drag = true;
                        self.drag_anchor = Some(idx);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            doc.set_rect_selection(idx, idx);
                        }
                        self.state.status = "Column select (Alt+drag)".into();
                    } else if !shift && inside_sel && !read_only {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            doc.clear_multi_sels();
                        }
                        self.sel_text_drag = Some(SelTextDrag { tab, drop_at: idx });
                        self.drag_anchor = None;
                        self.rect_drag = false;
                    } else if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                        self.sel_text_drag = None;
                        self.rect_drag = false;
                        if shift {
                            self.drag_anchor = Some(sel_anchor);
                            doc.buffer.set_selection(sel_anchor, idx);
                        } else {
                            self.drag_anchor = Some(idx);
                            doc.buffer.set_caret(idx);
                        }
                    }
                }
                self.follow_caret_other = false;
            }
        } else if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    if let Some(drag) = self.sel_text_drag.as_mut() {
                        if drag.tab == tab {
                            drag.drop_at = idx;
                            let copy = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);
                            ui.ctx().set_cursor_icon(if copy {
                                CursorIcon::Copy
                            } else {
                                CursorIcon::Grabbing
                            });
                        }
                    } else if let Some(anchor) = self.drag_anchor {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if self.rect_drag {
                                doc.set_rect_selection(anchor, idx);
                            } else {
                                doc.buffer.set_selection(anchor, idx);
                            }
                        }
                        self.follow_caret_other = false;
                    }
                }
            }
        } else if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    let shift = ui.input(|i| i.modifiers.shift);
                    let caret = doc.buffer.caret();
                    let sel_anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(caret);
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                        if shift {
                            doc.buffer.set_selection(sel_anchor, idx);
                        } else {
                            doc.buffer.set_caret(idx);
                        }
                    }
                    if !shift && self.compare_on {
                        self.sync_compare_other_to_caret_hunk(false);
                    }
                }
                self.drag_anchor = None;
                self.rect_drag = false;
                self.sel_text_drag = None;
                self.follow_caret_other = false;
            }
        }
        if ui.input(|i| i.pointer.any_released()) {
            if self.sel_text_drag.is_some() {
                let copy = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);
                self.finish_sel_text_drag(copy);
            }
            self.drag_anchor = None;
            self.rect_drag = false;
        }

        if response.has_focus()
            && self.focused_pane == EditorPane::Secondary
            && !self.state.find_open
            && !self.show_replace
            && self.handle_editor_input(ui, tab)
        {
            self.follow_caret_other = true;
            // Arrow / keyboard caret on a change line parks the other pane (same as click).
            if self.compare_on && (tab == self.compare_left_tab || tab == self.compare_right_tab) {
                self.sync_compare_other_to_caret_hunk(tab == self.compare_left_tab);
            }
        }

        if self.follow_caret_other {
            if let Some(doc) = self.state.tabs.get(tab) {
                let caret_line = doc.buffer.char_to_line(doc.buffer.caret());
                let caret_row = display_row_for(&visible_lines, caret_line) as f32;
                if caret_row < scroll_line {
                    scroll_line = caret_row;
                } else if caret_row >= scroll_line + visible_rows as f32 {
                    scroll_line = caret_row - visible_rows as f32 + 1.0;
                }
                scroll_line = scroll_line.clamp(0.0, max_scroll);
                self.scroll_line_other = scroll_line;
                if sync {
                    self.scroll_line = scroll_line;
                }
            }
            self.follow_caret_other = false;
        }

        let theme = self.current_theme();
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, theme.editor_bg);
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(rect.left(), rect.top()),
                Pos2::new(gutter_right, rect.bottom()),
            ),
            0.0,
            theme.gutter_bg,
        );
        if self.focused_pane == EditorPane::Secondary {
            painter.rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(1.0_f32, Color32::from_rgb(60, 100, 140)),
                egui::StrokeKind::Inside,
            );
        }

        let first_row = scroll_line.floor() as usize;
        let last_row = (first_row + visible_rows + 2).min(display_count);
        let plain = theme.plain_fg;
        let (changed_unsaved, changed_saved, fold_lang) = self
            .state
            .tabs
            .get(tab)
            .map(|d| {
                (
                    d.changed_unsaved.clone(),
                    d.changed_saved.clone(),
                    d.language.clone(),
                )
            })
            .unwrap_or_else(|| (Default::default(), Default::default(), "plain".into()));
        let fold_regions = self
            .state
            .tabs
            .get(tab)
            .map(|d| crate::fold::compute_fold_regions(fold_lang.as_str(), &d.buffer))
            .unwrap_or_default();
        let ln_right = if show_fold {
            fold_left - 2.0
        } else {
            gutter_right - 4.0
        };

        for row in first_row..last_row {
            let Some(&line_idx) = visible_lines.get(row) else {
                break;
            };
            let y = rect.top() + (row as f32 - scroll_line) * row_height;
            if self.compare_on {
                if let Some(kind) = self.compare_right_tags.get(line_idx) {
                    if let Some(bg) = crate::diff::line_kind_bg(*kind) {
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(text_left, y),
                                Pos2::new(rect.right(), y + row_height),
                            ),
                            0.0,
                            bg,
                        );
                    }
                }
                if self.state.settings.compare_hide_equal {
                    if let Some(&prev) = row.checked_sub(1).and_then(|r| visible_lines.get(r)) {
                        if let Some(skipped) = crate::diff::compare_visible_gap(prev, line_idx) {
                            paint_compare_hide_gap(
                                &painter,
                                &font_id,
                                text_left,
                                rect.right(),
                                y,
                                skipped,
                                theme.line_number_fg,
                            );
                        }
                    }
                }
            }
            if changed_unsaved.contains(&line_idx) {
                let (join_above, join_below) =
                    change_history_joins(line_idx, false, &changed_unsaved, &changed_saved);
                paint_change_history_bar(
                    &painter,
                    rect.left(),
                    y,
                    row_height,
                    false,
                    join_above,
                    join_below,
                );
                painter.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(text_left, y),
                        Pos2::new(rect.right(), y + row_height),
                    ),
                    0.0,
                    change_history_wash(false),
                );
            } else if changed_saved.contains(&line_idx) {
                let (join_above, join_below) =
                    change_history_joins(line_idx, true, &changed_unsaved, &changed_saved);
                paint_change_history_bar(
                    &painter,
                    rect.left(),
                    y,
                    row_height,
                    true,
                    join_above,
                    join_below,
                );
                painter.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(text_left, y),
                        Pos2::new(rect.right(), y + row_height),
                    ),
                    0.0,
                    change_history_wash(true),
                );
            }
            if show_ln {
                painter.text(
                    Pos2::new(ln_right, y),
                    egui::Align2::RIGHT_TOP,
                    format!("{}", line_idx + 1),
                    font_id.clone(),
                    theme.line_number_fg,
                );
            }
            if show_fold {
                if let Some(region) = crate::fold::region_at_header(&fold_regions, line_idx) {
                    let folded = self
                        .state
                        .tabs
                        .get(tab)
                        .is_some_and(|d| crate::fold::is_folded(&d.hidden_lines, &region));
                    paint_fold_marker(
                        &painter,
                        &font_id,
                        fold_left,
                        fold_w,
                        y,
                        row_height,
                        folded,
                        theme.line_number_fg,
                    );
                }
            }

            let Some(doc) = self.state.tabs.get(tab) else {
                break;
            };
            let line_start = doc.buffer.line_to_char(line_idx);
            let raw = doc.buffer.line(line_idx);
            let line_text = raw.trim_end_matches(['\n', '\r']);
            if self.compare_on {
                if let Some(kind) = self.compare_right_tags.get(line_idx) {
                    if let Some(bg) = crate::diff::line_kind_inline_bg(*kind) {
                        if let Some(spans) = self.compare_right_inline.get(line_idx) {
                            paint_inline_compare_spans(
                                &painter,
                                ui,
                                &font_id,
                                Rect::from_min_size(
                                    Pos2::new(text_left, y),
                                    Vec2::new(rect.width(), row_height),
                                ),
                                line_text,
                                spans,
                                bg,
                            );
                        }
                    }
                }
            }
            let primary_sel = doc.buffer.selection();
            let multi = doc.multi_sels.clone();

            if let Some((sel_s, sel_e)) = primary_sel {
                let line_end = line_start + line_text.chars().count();
                if sel_s < line_end && sel_e > line_start {
                    let local_s = sel_s
                        .saturating_sub(line_start)
                        .min(line_text.chars().count());
                    let local_e = sel_e
                        .saturating_sub(line_start)
                        .min(line_text.chars().count());
                    let x0 = text_left
                        + text_width(
                            ui,
                            &font_id,
                            &line_text.chars().take(local_s).collect::<String>(),
                        );
                    let x1 = text_left
                        + text_width(
                            ui,
                            &font_id,
                            &line_text.chars().take(local_e).collect::<String>(),
                        );
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(x0, y),
                            Pos2::new(x1.max(x0 + 2.0), y + row_height),
                        ),
                        0.0,
                        theme.selection_bg,
                    );
                }
            }
            for &(sel_s, sel_e) in &multi {
                if primary_sel == Some((sel_s, sel_e)) {
                    continue;
                }
                let line_end = line_start + line_text.chars().count();
                if sel_s == sel_e {
                    if sel_s >= line_start && sel_s <= line_end {
                        let col = sel_s - line_start;
                        let prefix: String = line_text.chars().take(col).collect();
                        let cx = text_left + text_width(ui, &font_id, &prefix);
                        painter.line_segment(
                            [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                            egui::Stroke::new(1.0_f32, theme.caret_fg),
                        );
                    }
                    continue;
                }
                if sel_s < line_end && sel_e > line_start {
                    let local_s = sel_s
                        .saturating_sub(line_start)
                        .min(line_text.chars().count());
                    let local_e = sel_e
                        .saturating_sub(line_start)
                        .min(line_text.chars().count());
                    let x0 = text_left
                        + text_width(
                            ui,
                            &font_id,
                            &line_text.chars().take(local_s).collect::<String>(),
                        );
                    let x1 = text_left
                        + text_width(
                            ui,
                            &font_id,
                            &line_text.chars().take(local_e).collect::<String>(),
                        );
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(x0, y),
                            Pos2::new(x1.max(x0 + 2.0), y + row_height),
                        ),
                        0.0,
                        theme.selection_bg,
                    );
                }
            }

            painter.text(
                Pos2::new(text_left, y),
                egui::Align2::LEFT_TOP,
                line_text,
                font_id.clone(),
                plain,
            );

            let caret = doc.buffer.caret();
            let line_end = line_start + line_text.chars().count();
            if caret >= line_start && caret <= line_end {
                let blink_on = if self.state.settings.caret_blink {
                    ui.ctx()
                        .request_repaint_after(std::time::Duration::from_millis(500));
                    ((ui.input(|i| i.time) * 2.0_f64) as i64).rem_euclid(2) == 0
                } else {
                    true
                };
                if blink_on {
                    let col = caret - line_start;
                    let prefix: String = line_text.chars().take(col).collect();
                    let cx = text_left + text_width(ui, &font_id, &prefix);
                    painter.line_segment(
                        [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                        egui::Stroke::new(1.0_f32, theme.caret_fg),
                    );
                }
            }

            if let Some(drag) = self.sel_text_drag.as_ref() {
                if drag.tab == tab && drag.drop_at >= line_start && drag.drop_at <= line_end {
                    let col = drag.drop_at - line_start;
                    let prefix: String = line_text.chars().take(col).collect();
                    let cx = text_left + text_width(ui, &font_id, &prefix);
                    painter.line_segment(
                        [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                        egui::Stroke::new(2.0_f32, Color32::from_rgb(220, 140, 40)),
                    );
                }
            }
        }
    }

    /// Returns true if the caret moved or text changed (caller should follow caret).
    fn handle_editor_input(&mut self, ui: &egui::Ui, tab: usize) -> bool {
        let mut changed = false;
        let mut caret_moved = false;
        let mut copy_text: Option<String> = None;
        let events: Vec<egui::Event> = ui.input(|i| i.events.clone());
        let mods = ui.input(|i| i.modifiers);
        let read_only = self
            .state
            .tabs
            .get(tab)
            .map(|d| d.read_only)
            .unwrap_or(true);

        for event in events {
            match event {
                egui::Event::Paste(t) => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                        continue;
                    }
                    if self.await_paste_bookmarks && tab == self.state.tabs.active_index() {
                        self.await_paste_bookmarks = false;
                        self.last_app_clipboard = Some(t.clone());
                        crate::commands::paste_over_bookmarked_lines(&mut self.state, &t);
                        changed = true;
                    } else {
                        self.state.prepare_edit_at(tab);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.insert_multi(&t) {
                                doc.buffer.insert(&t);
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Copy | egui::Event::Cut => {
                    if let Some(doc) = self.state.tabs.get(tab) {
                        let multi_text = doc.multi_sels_clipboard_text();
                        if let Some(text) = multi_text {
                            copy_text = Some(text);
                            if matches!(event, egui::Event::Cut) {
                                if read_only {
                                    self.state.status = "Document is read-only".into();
                                } else {
                                    self.state.prepare_edit_at(tab);
                                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                                        let _ = doc.delete_backward_multi();
                                        changed = true;
                                    }
                                }
                            }
                        } else if let Some((s, e)) = doc.buffer.selection() {
                            copy_text = Some(doc.buffer.slice(s, e));
                            if matches!(event, egui::Event::Cut) {
                                if read_only {
                                    self.state.status = "Document is read-only".into();
                                } else {
                                    self.state.prepare_edit_at(tab);
                                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                                        doc.buffer.delete_backward();
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
                egui::Event::Text(t) => {
                    if t.is_empty() || t.chars().all(|c| c.is_control()) {
                        continue;
                    }
                    if mods.command || mods.ctrl {
                        continue;
                    }
                    if read_only {
                        self.state.status = "Document is read-only".into();
                        continue;
                    }
                    self.state.prepare_edit_at(tab);
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        if !doc.insert_multi(&t) {
                            doc.buffer.insert(&t);
                        }
                        changed = true;
                    }
                }
                egui::Event::Key {
                    key: Key::Enter,
                    pressed: true,
                    modifiers,
                    ..
                } if !modifiers.command && !modifiers.ctrl => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                    } else {
                        self.state.prepare_edit_at(tab);
                        let eol = self.state.settings.default_eol.as_str().to_string();
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.insert_multi(&eol) {
                                doc.buffer.insert(&eol);
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::Tab,
                    pressed: true,
                    modifiers,
                    ..
                } if !modifiers.command && !modifiers.ctrl => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                    } else {
                        self.state.prepare_edit_at(tab);
                        let n = self.state.settings.tab_width.max(1) as usize;
                        let pad = " ".repeat(n);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.insert_multi(&pad) {
                                doc.buffer.insert(&pad);
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::Backspace,
                    pressed: true,
                    ..
                } => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                    } else {
                        self.state.prepare_edit_at(tab);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.delete_backward_multi() {
                                doc.buffer.delete_backward();
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::Delete,
                    pressed: true,
                    ..
                } => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                    } else {
                        self.state.prepare_edit_at(tab);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.delete_forward_multi() {
                                doc.buffer.delete_forward();
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::ArrowLeft,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    // Alt alone = word jump. Cmd/Ctrl+Alt(+Shift)+← is Compare apply (handle_shortcuts).
                    let cmd_mod = modifiers.command || modifiers.ctrl;
                    if modifiers.alt && !cmd_mod {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            doc.buffer.move_word(false, modifiers.shift);
                        }
                        caret_moved = true;
                    } else if !modifiers.alt {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            let c = doc.buffer.caret();
                            if c > 0 {
                                if modifiers.shift {
                                    let anchor =
                                        doc.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                                    doc.buffer.set_selection(anchor, c - 1);
                                } else {
                                    doc.buffer.set_caret(c - 1);
                                }
                                caret_moved = true;
                            }
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::ArrowRight,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    // Alt alone = word jump. Cmd/Ctrl+Alt(+Shift)+→ is Compare apply (handle_shortcuts).
                    let cmd_mod = modifiers.command || modifiers.ctrl;
                    if modifiers.alt && !cmd_mod {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            doc.buffer.move_word(true, modifiers.shift);
                        }
                        caret_moved = true;
                    } else if !modifiers.alt {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            let c = doc.buffer.caret();
                            let len = doc.buffer.len_chars();
                            if c < len {
                                if modifiers.shift {
                                    let anchor =
                                        doc.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                                    doc.buffer.set_selection(anchor, c + 1);
                                } else {
                                    doc.buffer.set_caret(c + 1);
                                }
                                caret_moved = true;
                            }
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::ArrowUp,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    if modifiers.command || modifiers.ctrl {
                        go_doc_start(&mut self.state, tab, modifiers.shift);
                    } else {
                        move_caret_vert(&mut self.state, tab, -1);
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::ArrowDown,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    if modifiers.command || modifiers.ctrl {
                        go_doc_end(&mut self.state, tab, modifiers.shift);
                    } else {
                        move_caret_vert(&mut self.state, tab, 1);
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::Home,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    if modifiers.command || modifiers.ctrl {
                        go_doc_start(&mut self.state, tab, modifiers.shift);
                    } else if let Some(doc) = self.state.tabs.get_mut(tab) {
                        let c = doc.buffer.caret();
                        let line = doc.buffer.char_to_line(c);
                        let line_start = doc.buffer.line_to_char(line);
                        if modifiers.shift {
                            let anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                            doc.buffer.set_selection(anchor, line_start);
                        } else {
                            doc.buffer.set_caret(line_start);
                        }
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::End,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if modifiers.command || modifiers.ctrl {
                        go_doc_end(&mut self.state, tab, modifiers.shift);
                    } else if let Some(doc) = self.state.tabs.get_mut(tab) {
                        let c = doc.buffer.caret();
                        let line = doc.buffer.char_to_line(c);
                        let raw = doc.buffer.line(line);
                        let n = raw.trim_end_matches(['\n', '\r']).chars().count();
                        let line_end = doc.buffer.line_to_char(line) + n;
                        if modifiers.shift {
                            let anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                            doc.buffer.set_selection(anchor, line_end);
                        } else {
                            doc.buffer.set_caret(line_end);
                        }
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::PageUp,
                    pressed: true,
                    ..
                } => {
                    move_caret_vert(&mut self.state, tab, -30);
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::PageDown,
                    pressed: true,
                    ..
                } => {
                    move_caret_vert(&mut self.state, tab, 30);
                    caret_moved = true;
                }
                _ => {}
            }
        }
        if let Some(t) = copy_text {
            self.last_app_clipboard = Some(t.clone());
            ui.ctx().copy_text(t);
        }
        if changed {
            self.state.mark_text_changed_at(tab);
        }
        changed || caret_moved
    }
}

/// Lines that look like fn / class / struct / def declarations (simple prefix match).
fn collect_func_like_lines(buf: &buffer::TextBuffer) -> Vec<(usize, String)> {
    const KEYS: &[&str] = &[
        "fn ",
        "def ",
        "function ",
        "class ",
        "struct ",
        "impl ",
        "impl<",
        "trait ",
        "interface ",
        "enum ",
        "mod ",
        "type ",
    ];
    const MODS: &[&str] = &[
        "pub ",
        "async ",
        "static ",
        "export ",
        "private ",
        "protected ",
        "public ",
        "crate ",
        "super ",
    ];
    let mut out = Vec::new();
    for i in 0..buf.line_count() {
        let raw = buf.line(i);
        let trimmed = raw.trim_end_matches(['\n', '\r']);
        let mut s = trimmed.trim_start();
        if s.is_empty() || s.starts_with("//") || s.starts_with('#') || s.starts_with("/*") {
            continue;
        }
        for _ in 0..4 {
            let mut hit = false;
            for m in MODS {
                if let Some(rest) = s.strip_prefix(m) {
                    s = rest.trim_start();
                    hit = true;
                    break;
                }
            }
            if !hit {
                break;
            }
        }
        let lower = s.to_ascii_lowercase();
        if KEYS.iter().any(|k| lower.starts_with(k)) {
            let preview: String = trimmed.chars().take(72).collect();
            out.push((i, preview));
        }
    }
    out
}

fn move_caret_vert(state: &mut EditorState, tab: usize, delta: i32) {
    let Some(b) = state.tabs.get_mut(tab) else {
        return;
    };
    let caret = b.buffer.caret();
    let line = b.buffer.char_to_line(caret) as i32 + delta;
    if line < 0 {
        b.buffer.set_caret(0);
        return;
    }
    let line = line as usize;
    if line >= b.buffer.line_count() {
        b.buffer.set_caret(b.buffer.len_chars());
        return;
    }
    let col = caret - b.buffer.line_to_char(b.buffer.char_to_line(caret));
    let raw = b.buffer.line(line);
    let n = raw.trim_end_matches(['\n', '\r']).chars().count();
    b.buffer.set_caret(b.buffer.line_to_char(line) + col.min(n));
}

fn go_doc_start(state: &mut EditorState, tab: usize, select: bool) {
    let Some(b) = state.tabs.get_mut(tab) else {
        return;
    };
    if select {
        let anchor = b
            .buffer
            .selection()
            .map(|(s, _)| s)
            .unwrap_or_else(|| b.buffer.caret());
        b.buffer.set_selection(anchor, 0);
    } else {
        b.buffer.set_caret(0);
    }
}

fn go_doc_end(state: &mut EditorState, tab: usize, select: bool) {
    let Some(b) = state.tabs.get_mut(tab) else {
        return;
    };
    let end = b.buffer.len_chars();
    if select {
        let anchor = b
            .buffer
            .selection()
            .map(|(s, _)| s)
            .unwrap_or_else(|| b.buffer.caret());
        b.buffer.set_selection(anchor, end);
    } else {
        b.buffer.set_caret(end);
    }
}

/// Tab title for a read-only on-disk snapshot used by Compare to Saved.
fn compare_saved_snapshot_title(file_name: &str) -> String {
    format!("{file_name} (saved)")
}

/// True for the untitled read-only tab created by Compare to Saved.
fn is_compare_saved_snapshot(doc: &doc::Document) -> bool {
    doc.read_only && doc.path.is_none() && doc.title.ends_with(" (saved)")
}

#[cfg(test)]
mod compare_pair_tests {
    use super::{
        compare_pair_status, compare_saved_snapshot_title, index_after_tab_close,
        is_compare_saved_snapshot, pick_compare_right, CompareHideStatus, ComparePairCounts,
    };

    #[test]
    fn saved_snapshot_title_uses_basename() {
        assert_eq!(compare_saved_snapshot_title("notes.md"), "notes.md (saved)");
        assert_eq!(compare_saved_snapshot_title("a"), "a (saved)");
    }

    #[test]
    fn saved_snapshot_detector_matches_compare_to_saved_tabs() {
        let mut snap = doc::Document::untitled(1, 1);
        snap.title = compare_saved_snapshot_title("notes.md");
        snap.read_only = true;
        assert!(is_compare_saved_snapshot(&snap));

        snap.read_only = false;
        assert!(!is_compare_saved_snapshot(&snap));

        let mut named = doc::Document::untitled(2, 2);
        named.title = "notes.md".into();
        named.read_only = true;
        assert!(!is_compare_saved_snapshot(&named));
    }

    #[test]
    fn needs_two_tabs() {
        assert_eq!(pick_compare_right(1, 0, None, false, 0), None);
    }

    fn counts(
        del: usize,
        ins: usize,
        hunk_n: usize,
        equal_pct: u8,
        kind_bit: &str,
    ) -> ComparePairCounts {
        ComparePairCounts {
            del,
            ins,
            hunk_n,
            equal_pct,
            kind_bit: kind_bit.to_string(),
        }
    }

    #[test]
    fn status_identical_vs_counts() {
        use super::CompareIgnoreBits;
        let none = CompareIgnoreBits {
            ws: false,
            case: false,
            blank: false,
        };
        assert_eq!(
            compare_pair_status(
                "a",
                "b",
                counts(0, 0, 0, 100, ""),
                none,
                CompareHideStatus {
                    hidden: None,
                    context: 3
                }
            ),
            "Compare “a” | “b” (identical)"
        );
        assert_eq!(
            compare_pair_status(
                "a",
                "b",
                counts(1, 2, 2, 50, ": 1 delete, 1 insert"),
                none,
                CompareHideStatus {
                    hidden: None,
                    context: 3
                }
            ),
            "Compare “a” | “b” (−1 +2) · 2 hunks: 1 delete, 1 insert · 50% equal"
        );
        assert_eq!(
            compare_pair_status(
                "a",
                "b",
                counts(0, 0, 0, 100, ""),
                CompareIgnoreBits {
                    ws: true,
                    case: true,
                    blank: false
                },
                CompareHideStatus {
                    hidden: None,
                    context: 3
                }
            ),
            "Compare “a” | “b” (identical) · ignore ws+case"
        );
        assert_eq!(
            compare_pair_status(
                "a",
                "b",
                counts(1, 0, 1, 80, ": 1 delete"),
                CompareIgnoreBits {
                    ws: true,
                    case: false,
                    blank: false
                },
                CompareHideStatus {
                    hidden: None,
                    context: 3
                }
            ),
            "Compare “a” | “b” (−1 +0) · 1 hunk: 1 delete · 80% equal · ignore ws"
        );
        assert_eq!(
            compare_pair_status(
                "a",
                "b",
                counts(0, 0, 0, 100, ""),
                CompareIgnoreBits {
                    ws: false,
                    case: false,
                    blank: true
                },
                CompareHideStatus {
                    hidden: None,
                    context: 3
                }
            ),
            "Compare “a” | “b” (identical) · ignore blank"
        );
        assert_eq!(
            compare_pair_status(
                "a",
                "b",
                counts(1, 0, 1, 75, ": 1 delete"),
                CompareIgnoreBits {
                    ws: true,
                    case: true,
                    blank: true
                },
                CompareHideStatus {
                    hidden: Some((4, 7)),
                    context: 3
                }
            ),
            "Compare “a” | “b” (−1 +0) · 1 hunk: 1 delete · 75% equal · ignore ws+case+blank · hide equal ±3 · L4|R7 hidden"
        );
        assert_eq!(
            compare_pair_status(
                "a",
                "b",
                counts(1, 0, 1, 90, ": 1 delete"),
                none,
                CompareHideStatus {
                    hidden: Some((5, 5)),
                    context: 3
                }
            ),
            "Compare “a” | “b” (−1 +0) · 1 hunk: 1 delete · 90% equal · hide equal ±3 · 5 hidden"
        );
    }

    #[test]
    fn at_hunk_status_bit_includes_lr_and_ordinal() {
        assert_eq!(
            super::compare_at_hunk_status_bit("L12 | R15", "(1/5 replace −1 +1)"),
            " · at L12 | R15 (1/5 replace −1 +1)"
        );
        assert_eq!(
            super::compare_at_hunk_status_bit("", "(1/2 delete −1 +0)"),
            " · at (1/2 delete −1 +0)"
        );
    }

    #[test]
    fn open_or_copy_status_matches_copy_wording() {
        assert_eq!(
            super::compare_open_or_copy_status("Copied", "a", "b", 0, 0, 0, ""),
            "Copied unified diff (identical) “a” | “b”"
        );
        assert_eq!(
            super::compare_open_or_copy_status("Opened", "L", "R", 2, 3, 2, ": 1 delete, 1 insert"),
            "Opened unified diff (−2 +3, 2 hunks: 1 delete, 1 insert) “L” | “R”"
        );
        assert_eq!(
            super::compare_open_or_copy_status("Copied", "a", "b", 1, 0, 1, ": 1 delete"),
            "Copied unified diff (−1 +0, 1 hunk: 1 delete) “a” | “b”"
        );
    }

    #[test]
    fn cleared_status_includes_kind_tallies() {
        assert_eq!(
            super::compare_cleared_status(false, "a", "b", 0, 0, 0, ""),
            "Compare cleared (identical) “a” | “b”"
        );
        assert_eq!(
            super::compare_cleared_status(true, "a", "b", 0, 0, 0, ""),
            "Compare cleared (closed saved snapshot) (identical) “a” | “b”"
        );
        assert_eq!(
            super::compare_cleared_status(false, "L", "R", 1, 2, 2, ": 1 delete, 1 insert"),
            "Compare cleared (−1 +2, 2 hunks: 1 delete, 1 insert) “L” | “R”"
        );
        assert_eq!(
            super::compare_cleared_status(true, "notes.md", "notes.md (saved)", 1, 0, 1, ": 1 delete"),
            "Compare cleared (closed saved snapshot) (−1 +0, 1 hunk: 1 delete) “notes.md” | “notes.md (saved)”"
        );
    }

    #[test]
    fn hunk_copy_open_status_includes_kind_and_counts() {
        assert_eq!(
            super::compare_hunk_copy_open_status("Copied", "(2/5 replace −1 +1)", "a", "b"),
            "Copied hunk (2/5 replace −1 +1) unified diff “a” | “b”"
        );
        assert_eq!(
            super::compare_hunk_copy_open_status("Opened", "(1/3 delete −2 +0)", "L", "R"),
            "Opened hunk (1/3 delete −2 +0) unified diff “L” | “R”"
        );
    }

    #[test]
    fn summary_copy_open_status_includes_kind_tallies() {
        assert_eq!(
            super::compare_summary_copy_open_status(
                "Copied",
                1,
                2,
                2,
                ": 1 delete, 1 insert",
                "a",
                "b"
            ),
            "Copied compare summary (−1 +2, 2 hunks: 1 delete, 1 insert) “a” | “b”"
        );
        assert_eq!(
            super::compare_summary_copy_open_status("Opened", 2, 0, 1, ": 1 delete", "L", "R"),
            "Opened compare summary (−2 +0, 1 hunk: 1 delete) “L” | “R”"
        );
        assert_eq!(
            super::compare_summary_copy_open_status("Copied", 0, 0, 0, "", "a", "b"),
            "Copied compare summary (−0 +0, 0 hunks) “a” | “b”"
        );
    }

    #[test]
    fn bookmark_difference_status_includes_kind_tallies() {
        assert_eq!(
            super::compare_bookmark_differences_status(1, 2, 2, 2, 2, ": 1 delete, 1 insert", 3),
            "Compare bookmarked differences (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, +3 new)"
        );
        assert_eq!(
            super::compare_bookmark_differences_status(1, 0, 1, 1, 1, ": 1 delete", 2),
            "Compare bookmarked differences (−1 +0, L1|R1, 1 hunk: 1 delete, +2 new)"
        );
        assert_eq!(
            super::compare_clear_difference_bookmarks_status(
                1,
                2,
                2,
                2,
                2,
                ": 1 delete, 1 insert",
                3
            ),
            "Compare cleared difference bookmarks (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, −3 removed)"
        );
        assert_eq!(
            super::compare_clear_difference_bookmarks_status(2, 0, 1, 1, 1, ": 1 delete", 0),
            "Compare cleared difference bookmarks (−2 +0, L1|R1, 1 hunk: 1 delete, −0 removed)"
        );
    }

    #[test]
    fn expand_collapse_all_unchanged_status_includes_kind_tallies() {
        assert_eq!(
            super::compare_expand_all_unchanged_status(1, 2, 2, ": 1 delete, 1 insert", 12),
            "Compare expanded all unchanged lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 shown)"
        );
        assert_eq!(
            super::compare_expand_all_unchanged_status(1, 0, 1, ": 1 delete", 4),
            "Compare expanded all unchanged lines (−1 +0, 1 hunk: 1 delete, 4 shown)"
        );
        assert_eq!(
            super::compare_collapse_all_unchanged_status(1, 2, 2, ": 1 delete, 1 insert", 12, 8),
            "Compare collapsed expanded equal lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 re-hidden, 8 hidden)"
        );
        assert_eq!(
            super::compare_collapse_all_unchanged_status(2, 0, 1, ": 1 delete", 3, 5),
            "Compare collapsed expanded equal lines (−2 +0, 1 hunk: 1 delete, 3 re-hidden, 5 hidden)"
        );
    }

    #[test]
    fn expand_collapse_gap_status_includes_kind_tallies() {
        assert_eq!(
            super::compare_expand_gap_status(5, " both panes", 1, 2, 2, ": 1 delete, 1 insert", 8),
            "Compare expanded ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 still hidden)"
        );
        assert_eq!(
            super::compare_expand_gap_status(3, "", 1, 0, 1, ": 1 delete", 0),
            "Compare expanded ···3 (−1 +0, 1 hunk: 1 delete, all equal lines shown)"
        );
        assert_eq!(
            super::compare_collapse_gap_status(
                5,
                " both panes",
                1,
                2,
                2,
                ": 1 delete, 1 insert",
                8
            ),
            "Compare collapsed ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 hidden)"
        );
        assert_eq!(
            super::compare_collapse_gap_status(2, "", 2, 0, 1, ": 1 delete", 5),
            "Compare collapsed ···2 (−2 +0, 1 hunk: 1 delete, 5 hidden)"
        );
    }

    #[test]
    fn equal_park_status_includes_kind_tallies_and_equal_pct() {
        use super::CompareIgnoreBits;
        let none = CompareIgnoreBits {
            ws: false,
            case: false,
            blank: false,
        };
        let no_hide = CompareHideStatus {
            hidden: None,
            context: 3,
        };
        let two = counts(1, 2, 2, 50, ": 1 delete, 1 insert");
        let one_del = counts(1, 0, 1, 80, ": 1 delete");
        let ident = counts(0, 0, 0, 100, "");
        assert_eq!(
            super::compare_equal_park_status(12, 12, &two, none, no_hide),
            "Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal"
        );
        assert_eq!(
            super::compare_equal_park_status(3, 5, &one_del, none, no_hide),
            "Compare equal → L3 | R5 (−1 +0, 1 hunk: 1 delete) · 80% equal"
        );
        assert_eq!(
            super::compare_equal_park_status(8, 8, &ident, none, no_hide),
            "Compare equal → L8 | R8 (identical)"
        );
        assert_eq!(
            super::compare_equal_park_status(
                12,
                12,
                &two,
                CompareIgnoreBits {
                    ws: true,
                    case: true,
                    blank: false
                },
                CompareHideStatus {
                    hidden: Some((5, 5)),
                    context: 3
                }
            ),
            "Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal · ignore ws+case · hide equal ±3 · 5 hidden"
        );
        assert_eq!(
            super::compare_equal_park_status(
                8,
                8,
                &ident,
                CompareIgnoreBits {
                    ws: false,
                    case: false,
                    blank: true
                },
                no_hide
            ),
            "Compare equal → L8 | R8 (identical) · ignore blank"
        );
    }

    #[test]
    fn hide_gap_nav_status_includes_kind_tallies() {
        let two = super::ComparePairCounts {
            del: 1,
            ins: 2,
            hunk_n: 2,
            equal_pct: 50,
            kind_bit: ": 1 delete, 1 insert".into(),
        };
        assert_eq!(
            super::compare_hide_gap_nav_status("Next", 5, 1, 3, &two, true),
            "Compare Next hidden equal → ···5 (1/3) (−1 +2, 2 hunks: 1 delete, 1 insert) · wrapped"
        );
        let del_only = super::ComparePairCounts {
            del: 1,
            ins: 0,
            hunk_n: 1,
            equal_pct: 80,
            kind_bit: ": 1 delete".into(),
        };
        assert_eq!(
            super::compare_hide_gap_nav_status("First", 3, 1, 2, &del_only, false),
            "Compare First hidden equal → ···3 (1/2) (−1 +0, 1 hunk: 1 delete)"
        );
        let ins_only = super::ComparePairCounts {
            del: 0,
            ins: 2,
            hunk_n: 1,
            equal_pct: 70,
            kind_bit: ": 1 insert".into(),
        };
        assert_eq!(
            super::compare_hide_gap_nav_status("Previous", 4, 2, 2, &ins_only, false),
            "Compare Previous hidden equal → ···4 (2/2) (−0 +2, 1 hunk: 1 insert)"
        );
    }

    #[test]
    fn prefers_tab_to_the_right() {
        assert_eq!(pick_compare_right(3, 0, None, false, 0), Some(1));
        assert_eq!(pick_compare_right(3, 1, None, false, 1), Some(2));
    }

    #[test]
    fn last_tab_uses_left_neighbor() {
        assert_eq!(pick_compare_right(3, 2, None, false, 2), Some(1));
    }

    #[test]
    fn marked_partner_wins() {
        assert_eq!(pick_compare_right(4, 0, Some(3), true, 1), Some(3));
    }

    #[test]
    fn dual_view_other_when_no_partner() {
        assert_eq!(pick_compare_right(4, 0, None, true, 2), Some(2));
    }

    #[test]
    fn close_middle_keeps_compare_pair_indices() {
        // Tabs [A,B,C], compare A|C (0,2); close B (1) → pair becomes (0,1).
        assert_eq!(index_after_tab_close(0, 1), Some(0));
        assert_eq!(index_after_tab_close(2, 1), Some(1));
        assert_eq!(index_after_tab_close(1, 1), None);
    }

    #[test]
    fn close_compared_side_maps_to_none() {
        assert_eq!(index_after_tab_close(0, 0), None);
        assert_eq!(index_after_tab_close(2, 2), None);
        assert_eq!(index_after_tab_close(0, 2), Some(0));
    }
}
