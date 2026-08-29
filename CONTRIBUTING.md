# Contributing

Thanks for helping with **npp-rs** / **npp-rust**.

## Layout

| Path | Role |
|------|------|
| `README.md` | Project intro |
| `LICENSE` | MIT |
| `CONTRIBUTING.md` | This file |
| `docs/` | All other documentation |
| `crates/` | Rust workspace crates |
| `Cargo.toml` | Workspace root (keep here) |
| `agents/` | Issue agent loop (`agents/AGENTS.md`) |
| `scripts/` | Helper scripts |

## Cargo stays at the repository root

Do **not** move `Cargo.toml` / `Cargo.lock` into a subfolder.

Cargo expects the workspace manifest at the project root. Moving it breaks the usual `cargo build`, CI, and editor tooling unless every command uses `--manifest-path`. That does not help clarity.

Crates already live under `crates/` — that is the right place for package code.

## Docs

Write or update notes under `docs/`. See [docs/README.md](docs/README.md).

## Code

1. Prefer branch `dev`.
2. Run `cargo test --workspace` before a PR.
3. Do not commit secrets, home paths, or private data (public repo).
4. Do not commit `reference/` (upstream Notepad++ clone). Clone it locally if you need it; it is gitignored.

## Upstream credit

This project is **Notepad++ inspired**. Official Notepad++: [github.com/notepad-plus-plus/notepad-plus-plus](https://github.com/notepad-plus-plus/notepad-plus-plus), created and maintained by **Don Ho**.
