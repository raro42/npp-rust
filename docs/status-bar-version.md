# Status bar build link

Date: 2026-08-29

Bottom-right of the status bar shows `v{version} · {short-hash}`.

Click opens `https://github.com/raro42/npp-rust/commit/{full-hash}`.

Hash comes from `crates/app/build.rs` at compile time (`NPP_GIT_HASH` / `NPP_GIT_HASH_FULL`).
