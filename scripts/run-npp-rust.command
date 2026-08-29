#!/bin/zsh
cd "$(dirname "$0")/.."
exec ./target/release/npp-rs
