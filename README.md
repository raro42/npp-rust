# npp-rs

> **Beta.** This project is early. Features may work — or they may not. Please [open an issue](https://github.com/raro42/npp-rust/issues/new) if something breaks, feels wrong, or is missing. Reports help a lot.

**A Notepad++–inspired text editor for every desktop — fast, local, and written in Rust.**

Cross-platform (macOS, Linux, Windows). Full upstream-style menu tree. MIT. Not affiliated with Notepad++.

[![GitHub release](https://img.shields.io/github/v/release/raro42/npp-rust?include_prereleases&style=flat-square)](https://github.com/raro42/npp-rust/releases/latest)
[![CI](https://img.shields.io/github/actions/workflow/status/raro42/npp-rust/ci.yml?branch=main&label=CI&style=flat-square)](https://github.com/raro42/npp-rust/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/actions/workflow/status/raro42/npp-rust/release.yml?event=release&label=release&style=flat-square)](https://github.com/raro42/npp-rust/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![egui](https://img.shields.io/badge/egui-1B1B1B?style=flat-square&logo=rust&logoColor=orange)](https://github.com/emilk/egui)
[![macOS](https://img.shields.io/badge/macOS-000000?style=flat-square&logo=apple&logoColor=white)](https://www.apple.com/macos/)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black)](https://kernel.org/)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows&logoColor=white)](https://www.microsoft.com/windows)

> Honest scope: this is **not** a drop-in Windows Notepad++ binary clone. See [docs/scope.md](docs/scope.md) and [docs/whats-missing.md](docs/whats-missing.md).

npp-rs takes inspiration from [Notepad++](https://github.com/notepad-plus-plus/notepad-plus-plus), created and maintained for over twenty years by **Don Ho**. Thank you for that work. This project is separate and is not affiliated with Notepad++.

📋 [Changelog](docs/changelog.md) · 🗺 [What’s missing](docs/whats-missing.md) · 📘 [Docs index](docs/README.md) · 🧪 [Testing](docs/testing.md) · 🤝 [Contributing](CONTRIBUTING.md)

## Quick start

**From source** (Rust stable + GUI deps on Linux):

```bash
git clone https://github.com/raro42/npp-rust.git
cd npp-rust
cargo run -p app --release
```

Binary name: `npp-rs`. On macOS you can also run `./scripts/run-npp-rust.command`.

**Prebuilt binaries:** [GitHub Releases](https://github.com/raro42/npp-rust/releases/latest) (Linux, Windows, macOS via the Release workflow).

```bash
cargo test --workspace
# or match CI:
./scripts/ci-local.sh
```

## Features

| Area | What you get |
|------|----------------|
| Menus | Full Notepad++-style menu tree (574 command IDs); teal = no Coming Soon stub |
| Editing | Multi-tab, undo/redo, find/replace, dual view, pin tabs, drag-reorder |
| Files | UTF-8 / UTF-8-BOM / ANSI (Windows-1252), atomic save, reload, opt-in session restore |
| View | Soft wrap, line numbers, document map, function list, 2-way compare |
| Logs | Tail / Monitoring for growing files; optional prompt when opening `*.log` |
| Ext | Builtin plugins (format, trim, case, EOL); Preferences → `npp-rs/settings.json` |

## Shortcuts (selected)

| Action | Shortcut |
|--------|----------|
| New / Open / Save | Ctrl/Cmd+N / O / S |
| Find / Replace | Ctrl/Cmd+F / Shift+F |
| Select all | Ctrl/Cmd+A |
| Duplicate line | Ctrl/Cmd+D |
| Format document | Ctrl/Cmd+Shift+I |
| Word jump | Alt+← / → |
| Indent / Outdent | Ctrl/Cmd+] / [ |
| Tail log | Ctrl/Cmd+Shift+T |

## Crates

| Crate | Role |
|-------|------|
| `buffer` | Rope text store, caret, undo |
| `doc` | Tabs and language detect |
| `fs` | Sync/async file I/O |
| `highlight` | Tree-sitter highlight |
| `format` | Language-aware format helpers |
| `plugins` | Builtin plugin host |
| `app` | egui UI and commands |

## Docs & agents

| Doc | Topic |
|-----|--------|
| [docs/scope.md](docs/scope.md) | Feature boundary |
| [docs/branch-main-only.md](docs/branch-main-only.md) | Work on `main` only |
| [docs/agent-loop.md](docs/agent-loop.md) | Issue agent loop |
| [docs/release.md](docs/release.md) | Version tags and CI release |

## License

[MIT](LICENSE). Separate project from Notepad++.
