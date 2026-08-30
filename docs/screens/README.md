# Screens & showcase GIF

Date: 2026-08-30

## README demo

| File | Role |
|------|------|
| [npp-rs-showcase.gif](npp-rs-showcase.gif) | Feature tour for the README |
| `frames/*.png` | Source stills (storyboard or real captures) |
| `fixtures/` | Sample files for manual capture |

## Rebuild the GIF

```bash
./scripts/make-showcase-gif.sh
```

If `frames/` has no PNGs, the script runs `scripts/gen-showcase-frames.py` (illustrated slides).

## Real window captures (better)

1. Build and open fixtures:

```bash
cargo run -p app --release -- \
  docs/screens/fixtures/sample.rs \
  docs/screens/fixtures/sample_b.rs \
  docs/screens/fixtures/app.log
```

2. Arrange each scene (editor, Find, Compare, Tail, Preferences).
3. Capture the window (macOS: `⌘⇧4` then space, or `screencapture -l <windowid>`).
4. Save as `docs/screens/frames/01-….png`, `02-….png`, … (sort order = play order).
5. Run `./scripts/make-showcase-gif.sh` again.

Needs **ffmpeg** on PATH (no new installs from the agent).

## Note

Current committed frames are a **storyboard**, not live egui captures. Replace them when you have real screenshots.
