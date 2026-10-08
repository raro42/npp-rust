# macOS install (Gatekeeper)

Date: 2026-10-08  
Repo: [raro42/npp-rust](https://github.com/raro42/npp-rust)

## Why macOS says “damaged”

The release build is **not** Apple-notarized (no paid Developer ID yet).  
When you download from a browser, macOS adds `com.apple.quarantine`.  
Gatekeeper then often shows **“damaged and can’t be opened”**. The binary is not corrupt.

## Easiest path (one-liner)

```bash
curl -fsSL https://raw.githubusercontent.com/raro42/npp-rust/main/scripts/install-macos.sh | bash
```

The script downloads the latest `npp-rs-macos-aarch64` release, sets execute permission, clears quarantine, and installs to `/Applications/npp-rs` (or `~/Applications/npp-rs`).

Pin a version:

```bash
VERSION=v0.3.139 curl -fsSL https://raw.githubusercontent.com/raro42/npp-rust/main/scripts/install-macos.sh | bash
```

## Manual download

1. Get `npp-rs-macos-aarch64.tar.gz` from [Releases](https://github.com/raro42/npp-rust/releases/latest).
2. Extract, then run:

```bash
chmod +x npp-rs-macos-aarch64
xattr -dr com.apple.quarantine npp-rs-macos-aarch64
./npp-rs-macos-aarch64
```

## From source

```bash
git clone https://github.com/raro42/npp-rust.git
cd npp-rust
cargo run -p app --release
```

Or `./scripts/run-npp-rust.command` after a release build.

## Notes

- Current CI asset is **Apple Silicon (arm64)** only.
- Ad-hoc codesign in CI keeps the Mach-O consistent. It does **not** replace notarization.
- Apple Developer ID + notarization can remove this step later.
