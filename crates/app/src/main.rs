//! npp-rs — Notepad++-inspired OS-agnostic text editor (MVP).

mod editor;
mod commands;
mod menu_data;
mod recent;
mod ui;

use eframe::egui;
use std::io::Write;
use ui::EditorApp;

fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let msg = format!(
            "npp-rs panic at {}\n{info}\n",
            chrono_stamp()
        );
        let _ = std::fs::create_dir_all("logs");
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/panic.log")
        {
            let _ = writeln!(f, "{msg}");
        }
        let _ = std::fs::write("/tmp/npp-rs-panic.log", &msg);
        eprintln!("{msg}");
        default_hook(info);
    }));
}

fn chrono_stamp() -> String {
    std::process::Command::new("date")
        .arg("+%Y-%m-%dT%H:%M:%S%z")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown-time".into())
}

fn main() -> eframe::Result<()> {
    install_panic_hook();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_title("npp-rust"),
        ..Default::default()
    };
    eframe::run_native(
        "npp-rust",
        options,
        Box::new(|cc| Ok(Box::new(EditorApp::new(cc)))),
    )
}
