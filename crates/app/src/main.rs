//! npp-rs — Notepad++-inspired OS-agnostic text editor (MVP).

mod backup;
mod commands;
mod diff;
mod editor;
mod fold;
mod menu_data;
mod recent;
mod search_util;
mod session;
mod theme;
mod ui;
mod ui_paint;

use eframe::egui;
use std::io::Write;
use std::path::PathBuf;
use ui::{CliOptions, EditorApp};

fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let msg = format!("npp-rs panic at {}\n{info}\n", chrono_stamp());
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

enum CliAction {
    Help,
    Version,
    Run(CliOptions),
}

fn parse_args() -> Result<CliAction, String> {
    let mut opts = CliOptions::default();
    let mut args = std::env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => return Ok(CliAction::Help),
            "--version" | "-V" => return Ok(CliAction::Version),
            "--read-only" | "-ro" => opts.read_only = true,
            "--line" | "-n" => {
                let Some(v) = args.next() else {
                    return Err("missing value for --line / -n".into());
                };
                let n: usize = v.parse().map_err(|_| format!("invalid line number: {v}"))?;
                if n == 0 {
                    return Err("line number must be >= 1".into());
                }
                opts.goto_line = Some(n);
            }
            a if a.starts_with("-n") && a.len() > 2 && a.as_bytes()[2].is_ascii_digit() => {
                let v = &a[2..];
                let n: usize = v.parse().map_err(|_| format!("invalid line number: {v}"))?;
                if n == 0 {
                    return Err("line number must be >= 1".into());
                }
                opts.goto_line = Some(n);
            }
            "--" => {
                for p in args {
                    opts.paths.push(PathBuf::from(p));
                }
                break;
            }
            a if a.starts_with('-') => {
                return Err(format!("unknown option: {a} (try --help)"));
            }
            _ => opts.paths.push(PathBuf::from(arg)),
        }
    }
    Ok(CliAction::Run(opts))
}

fn print_usage() {
    eprintln!(
        "Usage: npp-rs [OPTIONS] [FILE]...\n\
         \n\
         Open each existing FILE in a tab.\n\
         Missing paths are skipped (status line shows which).\n\
         \n\
         Options:\n\
           -h, --help         Show this help and exit\n\
           -V, --version      Print version and exit\n\
           -n <N>, --line <N> Go to 1-based line N in the first opened file\n\
           -n<N>              Same as -n <N> (Notepad++ style)\n\
           -ro, --read-only   Open argv files as read-only\n\
           --                 End of options; remaining args are paths\n\
         \n\
         Also: ? → Command Line Arguments... in the app."
    );
}

fn main() -> eframe::Result<()> {
    install_panic_hook();
    match parse_args() {
        Ok(CliAction::Help) => {
            print_usage();
            Ok(())
        }
        Ok(CliAction::Version) => {
            eprintln!("npp-rs {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Ok(CliAction::Run(opts)) => {
            let options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_inner_size([1100.0, 720.0])
                    .with_title("npp-rust"),
                ..Default::default()
            };
            eframe::run_native(
                "npp-rust",
                options,
                Box::new(move |cc| Ok(Box::new(EditorApp::new(cc, opts)))),
            )
        }
        Err(e) => {
            eprintln!("npp-rs: {e}");
            print_usage();
            std::process::exit(2);
        }
    }
}
