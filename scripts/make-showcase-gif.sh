#!/usr/bin/env bash
# Build docs/screens/npp-rs-showcase.gif from PNG frames.
# Prefer real captures in docs/screens/frames/*.png (sorted).
# If frames/ is empty, generate a feature-tour storyboard with Python.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FRAMES="$ROOT/docs/screens/frames"
OUT="$ROOT/docs/screens/npp-rs-showcase.gif"
mkdir -p "$FRAMES"

count="$(find "$FRAMES" -maxdepth 1 -name '*.png' 2>/dev/null | wc -l | tr -d ' ')"
if [[ "$count" -eq 0 ]]; then
  echo "No PNGs in frames/ — generating storyboard frames…"
  python3 "$ROOT/scripts/gen-showcase-frames.py" --out-dir "$FRAMES"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
i=0
while IFS= read -r f; do
  printf -v name '%03d.png' "$i"
  cp "$f" "$tmpdir/$name"
  i=$((i + 1))
done < <(find "$FRAMES" -maxdepth 1 -name '*.png' | sort)

if [[ "$i" -lt 2 ]]; then
  echo "Need at least 2 PNG frames in $FRAMES" >&2
  exit 1
fi

if ! command -v ffmpeg >/dev/null 2>&1; then
  echo "ffmpeg not found" >&2
  exit 1
fi

# ~1.7s per still (0.6 fps → palette GIF). Width 960 for README.
ffmpeg -y -hide_banner -loglevel error \
  -framerate 0.6 -i "$tmpdir/%03d.png" \
  -vf "fps=6,scale=960:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128:stats_mode=diff[p];[s1][p]paletteuse=dither=bayer:bayer_scale=3" \
  -loop 0 "$OUT"

ls -lh "$OUT"
echo "Wrote $OUT"
