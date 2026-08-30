//! Viewport text metrics and highlighted line paint (hot path).

use eframe::egui::{self, Color32, FontId, Pos2};
use std::collections::BTreeSet;

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

/// Draw a narrow gutter tick for a changed line (right of the bookmark slot).
/// `saved`: true = green (saved session), false = amber (unsaved).
pub(crate) fn paint_change_history_tick(
    painter: &egui::Painter,
    gutter_left: f32,
    y: f32,
    row_height: f32,
    saved: bool,
) {
    let color = if saved {
        change_history_saved_color()
    } else {
        change_history_unsaved_color()
    };
    painter.rect_filled(
        egui::Rect::from_min_max(
            Pos2::new(gutter_left + 12.0, y + 4.0),
            Pos2::new(gutter_left + 16.0, y + row_height - 4.0),
        ),
        0.5,
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
}
