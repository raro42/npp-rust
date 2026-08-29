//! Viewport text metrics and highlighted line paint (hot path).

use eframe::egui::{self, Color32, FontId, Pos2};
use highlight::color_for;

pub(crate) fn text_width(ui: &egui::Ui, font_id: &FontId, text: &str) -> f32 {
    ui.fonts(|f| f.layout_no_wrap(text.to_owned(), font_id.clone(), Color32::WHITE).size().x)
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
) {
    if line_text.is_empty() {
        return;
    }

    if language == "plain" || spans.is_empty() {
        painter.text(
            Pos2::new(x0, y),
            egui::Align2::LEFT_TOP,
            line_text,
            font_id.clone(),
            Color32::from_rgb(220, 220, 220),
        );
        return;
    }

    let mut x = x0;
    let chars: Vec<char> = line_text.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let global = line_start_char + i;
        let name = spans
            .iter()
            .find(|s| s.start <= global && global < s.end)
            .map(|s| s.name.as_str())
            .unwrap_or("");
        let (r, g, b) = if name.is_empty() {
            (0.85, 0.85, 0.85)
        } else {
            color_for(name)
        };
        let ch = chars[i].to_string();
        let w = text_width(ui, font_id, &ch);
        painter.text(
            Pos2::new(x, y),
            egui::Align2::LEFT_TOP,
            &ch,
            font_id.clone(),
            Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8),
        );
        x += w;
        i += 1;
    }
}
