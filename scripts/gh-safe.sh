#!/usr/bin/env bash
# Safe GitHub CLI wrapper for public repos.
# Forces issue/PR bodies through scripts/redact_public_text.py before posting.
#
# Examples:
#   ./scripts/gh-safe.sh issue comment 12 --body "Thanks — looking into it."
#   ./scripts/gh-safe.sh issue comment 12 --body-file ./note.md
#   ./scripts/gh-safe.sh pr create --title "x" --body-file ./pr.md
#
# Any --body / --body-file content is scanned. If secrets/PII patterns match,
# the command is aborted (exit 1) unless NPP_GH_SOFT_REDACT=1 (posts redacted text).

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REDACT="${ROOT}/scripts/redact_public_text.py"
GH_REPO="${NPP_GH_REPO:-raro42/npp-rust}"

if [[ ! -x "$(command -v python3)" ]]; then
  echo "gh-safe: python3 required" >&2
  exit 2
fi
if [[ ! -f "$REDACT" ]]; then
  echo "gh-safe: missing $REDACT" >&2
  exit 2
fi

args=("$@")
body=""
body_file=""
new_args=()
i=0
while [[ $i -lt ${#args[@]} ]]; do
  a="${args[$i]}"
  if [[ "$a" == "--body" ]]; then
    i=$((i + 1))
    body="${args[$i]:-}"
  elif [[ "$a" == --body=* ]]; then
    body="${a#--body=}"
  elif [[ "$a" == "--body-file" ]]; then
    i=$((i + 1))
    body_file="${args[$i]:-}"
  elif [[ "$a" == --body-file=* ]]; then
    body_file="${a#--body-file=}"
  else
    new_args+=("$a")
  fi
  i=$((i + 1))
done

if [[ -n "$body_file" ]]; then
  if [[ "$body_file" == "-" ]]; then
    body="$(cat)"
  else
    body="$(cat "$body_file")"
  fi
fi

tmp=""
cleanup() {
  [[ -n "$tmp" && -f "$tmp" ]] && rm -f "$tmp"
}
trap cleanup EXIT

if [[ -n "$body" ]]; then
  tmp="$(mktemp)"
  soft=()
  if [[ "${NPP_GH_SOFT_REDACT:-0}" == "1" ]]; then
    soft=(--soft-redact)
  fi
  # Always store redacted copy; hard-fail if findings unless soft mode.
  if ! printf '%s' "$body" | python3 "$REDACT" --redact "${soft[@]}" >"$tmp" 2>"${tmp}.err"; then
    cat "${tmp}.err" >&2 || true
    echo "gh-safe: refusing to post — private/sensitive patterns detected." >&2
    echo "gh-safe: fix the text or set NPP_GH_SOFT_REDACT=1 to post a redacted version (not recommended)." >&2
    exit 1
  fi
  if [[ -s "${tmp}.err" ]]; then
    cat "${tmp}.err" >&2 || true
  fi
  # Ensure repo flag for issue/pr if missing
  has_repo=0
  for a in "${new_args[@]}"; do
    [[ "$a" == "--repo" || "$a" == --repo=* ]] && has_repo=1
  done
  if [[ $has_repo -eq 0 ]]; then
    new_args+=(--repo "$GH_REPO")
  fi
  exec gh "${new_args[@]}" --body-file "$tmp"
fi

# No body — still default repo when useful
has_repo=0
for a in "${new_args[@]}"; do
  [[ "$a" == "--repo" || "$a" == --repo=* ]] && has_repo=1
done
if [[ $has_repo -eq 0 ]]; then
  case "${new_args[0]:-}" in
    issue|pr) new_args+=(--repo "$GH_REPO") ;;
  esac
fi
exec gh "${new_args[@]}"
