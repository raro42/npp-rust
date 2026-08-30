# Testing

**Date:** 2026-08-30

## Does `cargo build` run tests?

**No.** `cargo build` / `cargo build --release` only compiles.

## Local compile (quick)

```bash
cargo check -p app
cargo build -p app --release
```

## Compile gates (when to run what)

| When | Gate | Who |
|------|------|-----|
| **Before commit** (Rust staged) | `fmt` + `clippy -D warnings` | `pre-commit` hook + coder |
| **Before push** | full `./scripts/ci-local.sh` (fmt + clippy + test + release build) | `pre-push` hook + coder |
| **Loop 002 coder** | fmt+clippy before commit; **ci-local before push / TEST-** | required |
| **Loop 003 tester** | **ci-local** must pass before DONE | required |
| **Loop 005 CI watch** | GitHub Actions after push | daily catch |
| **Loop 008 git flush** | ci-local if Rust files dirty | before flush push |

You do **not** need full `ci-local.sh` on every tiny docs-only commit. You **do** need it before any push that GitHub CI will build, and before marking a task DONE.

Enable hooks once per clone:

```bash
./scripts/install-git-hooks.sh
```

## Menu parity test

`menu_data::tests::menu_matches_notepad_plus_plus_reference_export` asserts the embedded menu tree matches the Notepad++ `Notepad_plus.rc` export:

- Top menus: File, Edit, Search, View, Encoding, Language ×2, Settings, Tools, Macro, Run, Plugins, Window, ?
- **574** command items

Source export: `crates/app/data/npp_menu.json` (from `reference/notepad-plus-plus/.../Notepad_plus.rc`).
