# npp-rs

Notepad++ inspired text editor. OS-agnostic. Written in Rust.

See [docs/scope.md](docs/scope.md) for an honest feature boundary. This is **not** a drop-in Windows Notepad++ binary clone.

npp-rs takes inspiration from [Notepad++](https://github.com/notepad-plus-plus/notepad-plus-plus), created and maintained for over twenty years by **Don Ho**. Thank you for that work. This project is separate (MIT) and is not affiliated with Notepad++.

## Features

- **Full Notepad++ menu bar** (574 commands from upstream `Notepad_plus.rc`; unimplemented items show a status stub)
- CI runs `cargo test --workspace` on push (see [docs/testing.md](docs/testing.md))
- Multi-tab open / edit / save (UTF-8)
- **Open Recent** (persisted)
- Find + **Replace** / Replace All
- Double-click word, triple-click line, drag select
- Undo / redo, select all, indent / outdent, duplicate / delete line
- Syntax highlight: Rust, C, C++, **Python**, **SQL**, **Markdown**, JSON
- **Format Document** (Python / C++ / SQL / Markdown helpers)
- **Plugins** menu (in-process builtins: format, trim, case, EOL)
- Background open for files ≥ 2 MiB
- **Tail log** — **View → Monitoring (tail -f)** or ⌘/Ctrl+⇧+T; status bar **tail** / **TAIL** toggle

## Build and run

```bash
cargo run -p app --release
```

Binary name: `npp-rs`. Or: `./scripts/run-npp-rust.command`

```bash
cargo test --workspace
```

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
| `buffer` | Rope text store, caret, undo, word/line ops |
| `doc` | Tabs and language detect |
| `fs` | Sync/async file I/O |
| `highlight` | Tree-sitter highlight |
| `format` | Language-aware format helpers |
| `plugins` | Builtin plugin host |
| `app` | egui UI and commands |

## Docs

See [docs/README.md](docs/README.md). Contribute via [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. Separate project from Notepad++.
