//! Viewport text metrics and highlighted line paint (hot path).

use eframe::egui::{self, Color32, FontId, Pos2};
use std::collections::BTreeSet;

/// Width reserved for fold −/+ markers when the fold margin is on.
pub(crate) const FOLD_MARGIN_W: f32 = 14.0;

/// Document line indices that are not folded/hidden (display order).
pub(crate) fn visible_line_indices(line_count: usize, hidden: &BTreeSet<usize>) -> Vec<usize> {
    if hidden.is_empty() {
        return (0..line_count).collect();
    }
    (0..line_count).filter(|i| !hidden.contains(i)).collect()
}

/// Display row for a document line (nearest visible row if the line is hidden).
pub(crate) fn display_row_for(visible: &[usize], doc_line: usize) -> usize {
    if visible.is_empty() {
        return 0;
    }
    match visible.binary_search(&doc_line) {
        Ok(i) => i,
        Err(0) => 0,
        Err(i) => i - 1,
    }
}

/// Soft background for Search → Style mark slots 1..=5.
pub(crate) fn style_mark_bg(style: u8) -> Color32 {
    match style {
        1 => Color32::from_rgba_unmultiplied(200, 170, 40, 55),
        2 => Color32::from_rgba_unmultiplied(40, 170, 90, 55),
        3 => Color32::from_rgba_unmultiplied(40, 150, 190, 55),
        4 => Color32::from_rgba_unmultiplied(190, 70, 150, 55),
        5 => Color32::from_rgba_unmultiplied(130, 90, 190, 55),
        _ => Color32::from_rgba_unmultiplied(80, 80, 80, 40),
    }
}

/// Amber tick for unsaved change-history lines.
pub(crate) fn change_history_unsaved_color() -> Color32 {
    Color32::from_rgb(210, 140, 40)
}

/// Green tick for saved change-history lines.
pub(crate) fn change_history_saved_color() -> Color32 {
    Color32::from_rgb(70, 160, 90)
}

/// Soft wash behind a changed line (Scintilla-style backup when margin is narrow).
pub(crate) fn change_history_wash(saved: bool) -> Color32 {
    if saved {
        Color32::from_rgba_unmultiplied(70, 160, 90, 28)
    } else {
        Color32::from_rgba_unmultiplied(210, 140, 40, 32)
    }
}

/// Whether `line` joins a same-colour neighbour for a continuous SC_MARK_BAR.
pub(crate) fn change_history_joins(
    line: usize,
    saved: bool,
    unsaved: &BTreeSet<usize>,
    saved_set: &BTreeSet<usize>,
) -> (bool, bool) {
    let same = |other: usize| -> bool {
        if saved {
            saved_set.contains(&other) && !unsaved.contains(&other)
        } else {
            unsaved.contains(&other)
        }
    };
    let above = line > 0 && same(line - 1);
    let below = same(line + 1);
    (above, below)
}

/// Draw a full-height gutter bar for a changed line (Scintilla SC_MARK_BAR style).
/// `saved`: true = green (saved session), false = amber (unsaved).
/// `join_above` / `join_below`: connect with neighbours for a continuous block.
pub(crate) fn paint_change_history_bar(
    painter: &egui::Painter,
    gutter_left: f32,
    y: f32,
    row_height: f32,
    saved: bool,
    join_above: bool,
    join_below: bool,
) {
    let color = if saved {
        change_history_saved_color()
    } else {
        change_history_unsaved_color()
    };
    // Full line height when joined; slight inset at block ends (end caps).
    let y0 = if join_above { y } else { y + 1.5 };
    let y1 = if join_below {
        y + row_height
    } else {
        y + row_height - 1.5
    };
    painter.rect_filled(
        egui::Rect::from_min_max(
            Pos2::new(gutter_left + 11.0, y0),
            Pos2::new(gutter_left + 16.0, y1.max(y0 + 2.0)),
        ),
        0.0,
        color,
    );
}

/// Draw a fold margin marker (`−` open, `+` folded) in the gutter strip.
#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_fold_marker(
    painter: &egui::Painter,
    font_id: &FontId,
    fold_left: f32,
    fold_w: f32,
    y: f32,
    row_height: f32,
    folded: bool,
    color: Color32,
) {
    let mark = if folded { "+" } else { "−" };
    painter.text(
        Pos2::new(
            fold_left + fold_w * 0.5,
            y + (row_height - font_id.size) * 0.35,
        ),
        egui::Align2::CENTER_TOP,
        mark,
        font_id.clone(),
        color,
    );
}

pub(crate) fn text_width(ui: &egui::Ui, font_id: &FontId, text: &str) -> f32 {
    ui.fonts(|f| {
        f.layout_no_wrap(text.to_owned(), font_id.clone(), Color32::WHITE)
            .size()
            .x
    })
}

pub(crate) fn col_from_x(ui: &egui::Ui, font_id: &FontId, line: &str, x: f32) -> usize {
    if x <= 0.0 || line.is_empty() {
        return 0;
    }
    let chars: Vec<char> = line.chars().collect();
    let mut best = 0usize;
    let mut best_dist = f32::MAX;
    for i in 0..=chars.len() {
        let prefix: String = chars.iter().take(i).collect();
        let w = text_width(ui, font_id, &prefix);
        let d = (w - x).abs();
        if d < best_dist {
            best_dist = d;
            best = i;
        }
        if w > x + 2.0 {
            break;
        }
    }
    best
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_line_text(
    painter: &egui::Painter,
    ui: &egui::Ui,
    font_id: &FontId,
    x0: f32,
    y: f32,
    line_text: &str,
    line_start_char: usize,
    spans: &[highlight::Span],
    language: &str,
    theme: &crate::theme::AppliedTheme,
    rtl: bool,
) {
    if line_text.is_empty() {
        return;
    }

    let align = if rtl {
        egui::Align2::RIGHT_TOP
    } else {
        egui::Align2::LEFT_TOP
    };

    if language == "plain" || spans.is_empty() {
        painter.text(
            Pos2::new(x0, y),
            align,
            line_text,
            font_id.clone(),
            theme.plain_fg,
        );
        return;
    }

    let mut x = if rtl {
        x0 - text_width(ui, font_id, line_text)
    } else {
        x0
    };
    let chars: Vec<char> = line_text.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let global = line_start_char + i;
        let name = spans
            .iter()
            .find(|s| s.start <= global && global < s.end)
            .map(|s| s.name.as_str())
            .unwrap_or("");
        let color = theme.token_color(name);
        let ch = chars[i].to_string();
        let w = text_width(ui, font_id, &ch);
        painter.text(
            Pos2::new(x, y),
            egui::Align2::LEFT_TOP,
            &ch,
            font_id.clone(),
            color,
        );
        x += w;
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn visible_lines_skip_hidden() {
        let hidden: BTreeSet<usize> = [1usize, 3].into_iter().collect();
        assert_eq!(visible_line_indices(5, &hidden), vec![0, 2, 4]);
    }

    #[test]
    fn display_row_nearest_when_hidden() {
        let visible = vec![0usize, 2, 4];
        assert_eq!(display_row_for(&visible, 2), 1);
        assert_eq!(display_row_for(&visible, 3), 1);
        assert_eq!(display_row_for(&visible, 0), 0);
    }

    #[test]
    fn joins_detect_block_edges() {
        let unsaved: BTreeSet<usize> = [2usize, 3, 4].into_iter().collect();
        let saved: BTreeSet<usize> = [10usize, 11].into_iter().collect();
        assert_eq!(
            change_history_joins(2, false, &unsaved, &saved),
            (false, true)
        );
        assert_eq!(
            change_history_joins(3, false, &unsaved, &saved),
            (true, true)
        );
        assert_eq!(
            change_history_joins(4, false, &unsaved, &saved),
            (true, false)
        );
        assert_eq!(
            change_history_joins(10, true, &unsaved, &saved),
            (false, true)
        );
        let both_u: BTreeSet<usize> = [5usize].into_iter().collect();
        let both_s: BTreeSet<usize> = [4usize, 6].into_iter().collect();
        assert_eq!(
            change_history_joins(5, false, &both_u, &both_s),
            (false, false)
        );
    }
}
