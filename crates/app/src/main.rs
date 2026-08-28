//! npp-rs — Notepad++-inspired OS-agnostic text editor (MVP).

mod editor;
mod recent;
mod ui;

use eframe::egui;
use ui::EditorApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_title("npp-rs"),
        ..Default::default()
    };
    eframe::run_native(
        "npp-rs",
        options,
        Box::new(|cc| Ok(Box::new(EditorApp::new(cc)))),
    )
}
