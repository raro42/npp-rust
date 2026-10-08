#!/usr/bin/env bash
# Install the latest npp-rs macOS release (Apple Silicon).
#
# Strips the browser quarantine flag so Gatekeeper does not show
# "damaged and can't be opened" for this unsigned OSS build.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/raro42/npp-rust/main/scripts/install-macos.sh | bash
#   VERSION=v0.3.139 bash scripts/install-macos.sh
#
# Env:
#   VERSION   — release tag (default: latest)
#   DEST_DIR  — install directory (default: /Applications, else ~/Applications)

set -euo pipefail

REPO="raro42/npp-rust"
ASSET_STEM="npp-rs-macos-aarch64"
ASSET_TAR="${ASSET_STEM}.tar.gz"
INSTALL_NAME="npp-rs"

die() {
  echo "install-macos: $*" >&2
  exit 1
}

need() {
  command -v "$1" >/dev/null 2>&1 || die "need '$1' on PATH"
}

need curl
need tar
need uname

arch="$(uname -m)"
[[ "$arch" == "arm64" ]] || die "this release is Apple Silicon (arm64) only; got: $arch"

os="$(uname -s)"
[[ "$os" == "Darwin" ]] || die "this script is for macOS only; got: $os"

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/npp-rs-install.XXXXXX")"
cleanup() { rm -rf "$tmpdir"; }
trap cleanup EXIT

api="https://api.github.com/repos/${REPO}/releases"
if [[ -n "${VERSION:-}" ]]; then
  tag="${VERSION}"
  [[ "$tag" == v* ]] || tag="v${tag}"
  echo "Using release ${tag}"
  url="https://github.com/${REPO}/releases/download/${tag}/${ASSET_TAR}"
else
  echo "Resolving latest release…"
  json="$(curl -fsSL "${api}/latest")" || die "failed to fetch latest release metadata"
  tag="$(printf '%s\n' "$json" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -n1)"
  [[ -n "$tag" ]] || die "could not parse tag_name from GitHub API"
  url="https://github.com/${REPO}/releases/download/${tag}/${ASSET_TAR}"
  echo "Latest tag: ${tag}"
fi

archive="${tmpdir}/${ASSET_TAR}"
echo "Downloading ${url}"
curl -fL --retry 3 --retry-delay 1 -o "$archive" "$url" || die "download failed (check tag and asset name)"

echo "Extracting…"
tar -xzf "$archive" -C "$tmpdir"
bin="${tmpdir}/${ASSET_STEM}"
[[ -f "$bin" ]] || die "archive missing ${ASSET_STEM}"

chmod +x "$bin"
if command -v xattr >/dev/null 2>&1; then
  echo "Clearing macOS quarantine (Gatekeeper 'damaged' workaround)…"
  xattr -dr com.apple.quarantine "$bin" 2>/dev/null || true
  xattr -cr "$bin" 2>/dev/null || true
fi

if [[ -n "${DEST_DIR:-}" ]]; then
  dest_dir="${DEST_DIR}"
elif [[ -w /Applications ]] || [[ -d /Applications && -w /Applications ]]; then
  dest_dir="/Applications"
else
  dest_dir="${HOME}/Applications"
  mkdir -p "$dest_dir"
fi

dest="${dest_dir}/${INSTALL_NAME}"
echo "Installing to ${dest}"
if [[ -e "$dest" ]] && [[ ! -w "$dest_dir" ]]; then
  die "cannot write ${dest}; set DEST_DIR or run with write access"
fi

# Prefer install(1) when available; fall back to cp.
if command -v install >/dev/null 2>&1; then
  install -m 755 "$bin" "$dest"
else
  cp "$bin" "$dest"
  chmod 755 "$dest"
fi

if command -v xattr >/dev/null 2>&1; then
  xattr -dr com.apple.quarantine "$dest" 2>/dev/null || true
  xattr -cr "$dest" 2>/dev/null || true
fi

echo
echo "Installed ${tag} → ${dest}"
echo "Run: ${dest}"
echo
echo "If macOS still says the file is damaged, run:"
echo "  xattr -dr com.apple.quarantine ${dest}"
echo "Then open ${dest} again."
echo "More: https://github.com/${REPO}/blob/main/docs/macos-install.md"
