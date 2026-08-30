#!/usr/bin/env bash
# Mirror GitHub CI gates locally (fmt → clippy → test → release build).
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== fmt =="
cargo fmt --all -- --check

echo "== clippy =="
cargo clippy --workspace --all-targets -- -D warnings

echo "== test =="
cargo test --workspace

echo "== release build =="
cargo build -p app --release

echo "OK — same gates as .github/workflows/ci.yml"
