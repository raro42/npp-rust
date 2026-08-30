#!/usr/bin/env bash
# Point this clone at .githooks so pushes run ci-local.sh first.
set -euo pipefail
cd "$(dirname "$0")/.."
git config core.hooksPath .githooks
chmod +x .githooks/pre-push
echo "OK — core.hooksPath=.githooks (pre-push runs ./scripts/ci-local.sh)"
echo "Note: Windows-only #[cfg] code is still only fully checked on GitHub Windows CI."
