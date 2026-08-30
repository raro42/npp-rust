# Testing

**Date:** 2026-08-30

## Does `cargo build` run tests?

**No.** `cargo build` / `cargo build --release` only compiles.

## Local compile (quick)

```bash
cargo check -p app
cargo build -p app --release
```

## Match CI before push (required)

GitHub Actions (`.github/workflows/ci.yml`) runs on Ubuntu, Windows, and macOS for pushes to `main` and PRs to `main`. Run the same gates locally **before commit/push**:

```bash
./scripts/install-git-hooks.sh   # once per clone — pre-push runs the gates
./scripts/ci-local.sh
```

Or step by step:

```bash
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build -p app --release
```

`cargo check` alone does **not** catch rustfmt failures. That was why CI went red while local check looked fine.

**Windows-only code:** `#[cfg(windows)]` blocks are not clippy’d on macOS/Linux. Prefer expression style (no trailing `return`) in those blocks so Windows CI stays green.

Toolchain: `rust-toolchain.toml` pins stable with `rustfmt` and `clippy`.

## Menu parity test

`menu_data::tests::menu_matches_notepad_plus_plus_reference_export` asserts the embedded menu tree matches the Notepad++ `Notepad_plus.rc` export:

- Top menus: File, Edit, Search, View, Encoding, Language ×2, Settings, Tools, Macro, Run, Plugins, Window, ?
- **574** command items

Source export: `crates/app/data/npp_menu.json` (from `reference/notepad-plus-plus/.../Notepad_plus.rc`).
