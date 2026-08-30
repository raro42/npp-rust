#!/usr/bin/env python3
"""Generate feature-tour storyboard PNGs for the npp-rs showcase GIF.

These frames are illustrated slides (not live window captures).
Replace docs/screens/frames/*.png with real screencaptures when ready,
then re-run scripts/make-showcase-gif.sh.
"""

from __future__ import annotations

import argparse
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


W, H = 1200, 720
BG = (28, 32, 38)
PANEL = (36, 42, 50)
TAB_BG = (48, 56, 66)
ACCENT = (42, 148, 118)
TEXT = (220, 225, 230)
MUTED = (140, 150, 160)
CODE = (190, 200, 210)


def font(size: int) -> ImageFont.ImageFont:
    for name in (
        "/System/Library/Fonts/Menlo.ttc",
        "/System/Library/Fonts/SFNSMono.ttf",
        "/Library/Fonts/Courier New.ttf",
    ):
        try:
            return ImageFont.truetype(name, size)
        except OSError:
            continue
    return ImageFont.load_default()


def draw_chrome(draw: ImageDraw.ImageDraw, title: str, tabs: list[str], active: int) -> None:
    draw.rectangle([0, 0, W, H], fill=BG)
    draw.rectangle([0, 0, W, 36], fill=(40, 44, 52))
    draw.text((16, 10), "npp-rs  ·  File  Edit  Search  View  Language  Settings  ?", fill=MUTED, font=font(14))
    draw.rectangle([0, 36, W, 72], fill=PANEL)
    x = 12
    for i, tab in enumerate(tabs):
        tw = 18 + len(tab) * 9
        fill = TAB_BG if i == active else (32, 36, 42)
        draw.rounded_rectangle([x, 42, x + tw, 66], radius=4, fill=fill)
        color = ACCENT if i == active else MUTED
        draw.text((x + 8, 46), tab, fill=color, font=font(13))
        x += tw + 6
    draw.text((16, H - 28), title, fill=ACCENT, font=font(16))


def frame_editor(path: Path) -> None:
    im = Image.new("RGB", (W, H), BG)
    d = ImageDraw.Draw(im)
    draw_chrome(d, "Editing · multi-tab · Tree-sitter highlight", ["sample.rs", "notes.md"], 0)
    lines = [
        (" 1", "//! Showcase sample — npp-rs"),
        (" 2", "fn main() {"),
        (" 3", '    let greeting = "hello, npp-rs";'),
        (" 4", "    println!(\"{greeting}\");"),
        (" 5", "    for i in 0..3 {"),
        (" 6", "        println!(\"tick {i}\");"),
        (" 7", "    }"),
        (" 8", "}"),
    ]
    y = 96
    for num, line in lines:
        d.text((24, y), num, fill=MUTED, font=font(15))
        d.text((64, y), line, fill=CODE, font=font(15))
        y += 26
    im.save(path)


def frame_find(path: Path) -> None:
    im = Image.new("RGB", (W, H), BG)
    d = ImageDraw.Draw(im)
    draw_chrome(d, "Find / Replace · match case · live count", ["sample.rs"], 0)
    d.rectangle([0, 72, W, 112], fill=(44, 52, 62))
    d.text((16, 84), "Find:  greeting     [Match case]  [Whole word]   2 matches", fill=TEXT, font=font(14))
    y = 130
    for num, line, hit in [
        (" 1", "//! Showcase sample — npp-rs", False),
        (" 2", "fn main() {", False),
        (" 3", '    let greeting = "hello, npp-rs";', True),
        (" 4", "    println!(\"{greeting}\");", True),
    ]:
        if hit:
            d.rectangle([56, y - 2, 560, y + 22], fill=(60, 90, 70))
        d.text((24, y), num, fill=MUTED, font=font(15))
        d.text((64, y), line, fill=CODE, font=font(15))
        y += 26
    im.save(path)


def frame_compare(path: Path) -> None:
    im = Image.new("RGB", (W, H), BG)
    d = ImageDraw.Draw(im)
    draw_chrome(
        d,
        "Compare · active vs tab to the right (or ⌘-click partner)",
        ["sample.rs", "sample_b.rs ⇄"],
        0,
    )
    mid = W // 2
    d.line([(mid, 72), (mid, H - 40)], fill=(60, 68, 78), width=2)
    left = [
        (False, '    let greeting = "hello, npp-rs";'),
        (True, "    for i in 0..3 {"),
    ]
    right = [
        (False, '    let greeting = "hello, world";'),
        (True, "    for i in 0..5 {"),
    ]
    y = 100
    d.text((24, y), "sample.rs", fill=ACCENT, font=font(13))
    d.text((mid + 24, y), "sample_b.rs", fill=ACCENT, font=font(13))
    y = 130
    for (ldel, ll), (rins, rl) in zip(left, right):
        if ldel:
            d.rectangle([16, y - 2, mid - 8, y + 22], fill=(90, 50, 50))
        if rins:
            d.rectangle([mid + 8, y - 2, W - 16, y + 22], fill=(50, 90, 60))
        d.text((24, y), ll, fill=CODE, font=font(14))
        d.text((mid + 24, y), rl, fill=CODE, font=font(14))
        y += 28
    d.text((24, H - 56), "Compare “sample.rs” | “sample_b.rs” (−1 +1)", fill=MUTED, font=font(13))
    im.save(path)


def frame_tail(path: Path) -> None:
    im = Image.new("RGB", (W, H), BG)
    d = ImageDraw.Draw(im)
    draw_chrome(d, "Tail / Monitoring · growing log files", ["app.log [tail]"], 0)
    lines = [
        "2026-08-30T08:00:01Z INFO  start",
        "2026-08-30T08:00:02Z INFO  load config",
        "2026-08-30T08:00:03Z WARN  cache miss",
        "2026-08-30T08:00:04Z INFO  ready",
        "2026-08-30T08:00:05Z INFO  request ok",
    ]
    y = 100
    for i, line in enumerate(lines, 1):
        d.text((24, y), f"{i:2d}", fill=MUTED, font=font(15))
        color = (200, 160, 80) if "WARN" in line else CODE
        d.text((64, y), line, fill=color, font=font(15))
        y += 26
    d.ellipse([W - 40, 90, W - 24, 106], fill=ACCENT)
    im.save(path)


def frame_prefs(path: Path) -> None:
    im = Image.new("RGB", (W, H), BG)
    d = ImageDraw.Draw(im)
    draw_chrome(d, "Preferences · font, session, compare options", ["settings"], 0)
    d.rounded_rectangle([80, 100, W - 80, H - 80], radius=8, fill=PANEL)
    d.text((110, 120), "Preferences", fill=TEXT, font=font(20))
    items = [
        "Font size: 14",
        "Restore session on start",
        "Recent files: 12",
        "Compare: ignore whitespace",
        "Default EOL: LF",
    ]
    y = 170
    for item in items:
        d.ellipse([120, y + 4, 132, y + 16], outline=ACCENT, width=2)
        d.text((148, y), item, fill=CODE, font=font(16))
        y += 36
    im.save(path)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", type=Path, required=True)
    args = ap.parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)
    # Clear prior storyboard only if names match our prefixes.
    specs = [
        ("01-editor.png", frame_editor),
        ("02-find.png", frame_find),
        ("03-compare.png", frame_compare),
        ("04-tail.png", frame_tail),
        ("05-prefs.png", frame_prefs),
    ]
    for name, fn in specs:
        fn(args.out_dir / name)
        print(f"wrote {args.out_dir / name}")


if __name__ == "__main__":
    main()
