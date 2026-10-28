//! npp-rs — Notepad++-inspired OS-agnostic text editor (MVP).

mod backup;
mod clipboard_history;
mod commands;
mod diff;
mod editor;
mod fold;
mod menu_data;
mod ollama;
mod recent;
mod search_util;
mod session;
mod shortcut_chord;
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

#[derive(Debug)]
enum CliAction {
    Help,
    Version,
    Run(CliOptions),
}

fn parse_args() -> Result<CliAction, String> {
    parse_args_iter(std::env::args().skip(1))
}

fn parse_args_iter<I, S>(args: I) -> Result<CliAction, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut opts = CliOptions::default();
    let mut args = args.into_iter().peekable();
    while let Some(arg_owned) = args.next() {
        let arg = arg_owned.as_ref();
        match arg {
            "--help" | "-h" => return Ok(CliAction::Help),
            "--version" | "-V" => return Ok(CliAction::Version),
            "--read-only" | "-ro" | "-readonly" => opts.read_only = true,
            "--no-session" | "-nosession" => opts.no_session = true,
            "--line" | "-n" => {
                let Some(v) = args.next() else {
                    return Err("missing value for --line / -n".into());
                };
                opts.goto_line = Some(parse_positive_usize(v.as_ref(), "line number")?);
            }
            a if a.starts_with("-n") && a.len() > 2 && a.as_bytes()[2].is_ascii_digit() => {
                opts.goto_line = Some(parse_positive_usize(&a[2..], "line number")?);
            }
            "--column" | "-c" => {
                let Some(v) = args.next() else {
                    return Err("missing value for --column / -c".into());
                };
                opts.goto_column = Some(parse_positive_usize(v.as_ref(), "column number")?);
            }
            a if a.starts_with("-c") && a.len() > 2 && a.as_bytes()[2].is_ascii_digit() => {
                opts.goto_column = Some(parse_positive_usize(&a[2..], "column number")?);
            }
            "--" => {
                for p in args {
                    opts.paths.push(PathBuf::from(p.as_ref()));
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

fn parse_positive_usize(v: &str, label: &str) -> Result<usize, String> {
    let n: usize = v.parse().map_err(|_| format!("invalid {label}: {v}"))?;
    if n == 0 {
        return Err(format!("{label} must be >= 1"));
    }
    Ok(n)
}

fn print_usage() {
    eprintln!(
        "Usage: npp-rs [OPTIONS] [FILE]...\n\
         \n\
         Open each existing FILE in a tab.\n\
         Missing paths are skipped (status line shows which).\n\
         \n\
         Options:\n\
           -h, --help              Show this help and exit\n\
           -V, --version           Print version and exit\n\
           -n <N>, --line <N>      Go to 1-based line N in the first opened file\n\
           -n<N>                   Same as -n <N> (Notepad++ style)\n\
           -c <N>, --column <N>    Go to 1-based column N in the first opened file\n\
           -c<N>                   Same as -c <N> (Notepad++ style)\n\
           -ro, --read-only, -readonly  Open argv files as read-only\n\
           -nosession, --no-session  Skip session restore (empty start)\n\
           --                      End of options; remaining args are paths\n\
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

#[cfg(test)]
mod tests {
    use super::*;

    fn run_opts(args: &[&str]) -> CliOptions {
        match parse_args_iter(args.iter().copied()).unwrap() {
            CliAction::Run(o) => o,
            CliAction::Help | CliAction::Version => panic!("expected Run"),
        }
    }

    #[test]
    fn parses_line_and_column_glued() {
        let o = run_opts(&["-n42", "-c7", "file.txt"]);
        assert_eq!(o.goto_line, Some(42));
        assert_eq!(o.goto_column, Some(7));
        assert_eq!(o.paths, vec![PathBuf::from("file.txt")]);
    }

    #[test]
    fn parses_line_and_column_spaced() {
        let o = run_opts(&["--line", "3", "--column", "9", "a.rs"]);
        assert_eq!(o.goto_line, Some(3));
        assert_eq!(o.goto_column, Some(9));
    }

    #[test]
    fn parses_no_session_and_readonly_aliases() {
        let o = run_opts(&["-nosession", "-readonly", "x"]);
        assert!(o.no_session);
        assert!(o.read_only);
    }

    #[test]
    fn rejects_zero_column() {
        let err = parse_args_iter(["-c0"].iter().copied()).unwrap_err();
        assert!(err.contains(">= 1"));
    }

    #[test]
    fn help_and_version_short_circuit() {
        assert!(matches!(
            parse_args_iter(["-h"].iter().copied()).unwrap(),
            CliAction::Help
        ));
        assert!(matches!(
            parse_args_iter(["--version"].iter().copied()).unwrap(),
            CliAction::Version
        ));
    }
}
