#!/usr/bin/env bash
# Once-a-day local disk hygiene for npp-rust.
# Removes compile artifacts and local release piles. Does not touch other projects.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "===== daily-clean $(date -u +%Y-%m-%dT%H:%M:%SZ)"

if [[ -d target ]]; then
  echo "----- cargo clean"
  cargo clean
fi

for d in dist release-tmp releases-local .release-staging; do
  if [[ -e "$d" ]]; then
    echo "----- remove $d"
    rm -rf "$d"
  fi
done

# Scratch only under this repo or predictable temp names.
rm -f /tmp/npp-redact-*.txt 2>/dev/null || true
rm -rf /tmp/npp-release-* 2>/dev/null || true

echo "----- done"
