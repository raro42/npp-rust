# Testing

**Date:** 2026-08-28

## Does `cargo build` run tests?

**No.** `cargo build` / `cargo build --release` only compiles.

## What runs tests

```bash
cargo test --workspace
```

GitHub Actions CI (`.github/workflows/ci.yml`) runs on Ubuntu, Windows, and macOS for pushes to `main`/`dev` and PRs to `main`:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo build -p app --release`

Toolchain: `rust-toolchain.toml` pins stable with `rustfmt` and `clippy`.

## Menu parity test

`menu_data::tests::menu_matches_notepad_plus_plus_reference_export` asserts the embedded menu tree matches the Notepad++ `Notepad_plus.rc` export:

- Top menus: File, Edit, Search, View, Encoding, Language ×2, Settings, Tools, Macro, Run, Plugins, Window, ?
- **574** command items

Source export: `crates/app/data/npp_menu.json` (from `reference/notepad-plus-plus/.../Notepad_plus.rc`).
