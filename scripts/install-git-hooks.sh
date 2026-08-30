#!/usr/bin/env bash
# Point this clone at .githooks so commits/pushes run local gates.
set -euo pipefail
cd "$(dirname "$0")/.."
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit .githooks/pre-push
echo "OK — core.hooksPath=.githooks"
echo "  pre-commit → fmt + clippy (when Rust is staged)"
echo "  pre-push   → ./scripts/ci-local.sh (fmt + clippy + test + release)"
echo "Note: Windows-only #[cfg] code is still only fully checked on GitHub Windows CI."
