#!/usr/bin/env python3
"""Contact sheets for reviewing candidate images (labelled grid on a checkerboard, so transparency
and cut-off edges are visible).

    python3 -I tools/catalog/souq/sheet.py OUT_PREFIX TILE COLS FILE [FILE ...]
    python3 -I tools/catalog/souq/sheet.py OUT_PREFIX TILE COLS --dir DIR

Writes OUT_PREFIX-01.png, OUT_PREFIX-02.png … (ROWS rows per sheet).
"""

from __future__ import annotations

import os
import sys

from PIL import Image, ImageDraw, ImageFont

FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
ROWS = 6


def checker(size: int, cell: int = 12) -> Image.Image:
    im = Image.new("RGB", (size, size), (235, 235, 235))
    d = ImageDraw.Draw(im)
    for y in range(0, size, cell):
        for x in range(0, size, cell):
            if (x // cell + y // cell) % 2:
                d.rectangle([x, y, x + cell - 1, y + cell - 1], fill=(205, 205, 205))
    return im


def main() -> int:
    if len(sys.argv) < 5:
        print(__doc__)
        return 2
    prefix, tile, cols = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    rest = sys.argv[4:]
    if rest[0] == "--dir":
        d = rest[1]
        files = sorted(os.path.join(d, f) for f in os.listdir(d) if not f.endswith(".part"))
    else:
        files = rest
    try:
        font = ImageFont.truetype(FONT, 13)
    except OSError:
        font = ImageFont.load_default()
    label_h = 18
    per = cols * ROWS
    bg = checker(tile)
    for s in range(0, len(files), per):
        chunk = files[s : s + per]
        rows = (len(chunk) + cols - 1) // cols
        sheet = Image.new("RGB", (cols * (tile + 4), rows * (tile + label_h + 4)), (255, 255, 255))
        d = ImageDraw.Draw(sheet)
        for i, f in enumerate(chunk):
            x = (i % cols) * (tile + 4)
            y = (i // cols) * (tile + label_h + 4)
            cell = bg.copy()
            try:
                im = Image.open(f)
                im.load()
                im = im.convert("RGBA")
                im.thumbnail((tile, tile))
                cell.paste(im, ((tile - im.size[0]) // 2, (tile - im.size[1]) // 2), im)
            except Exception as exc:  # noqa: BLE001
                ImageDraw.Draw(cell).text((4, 4), f"ERR {exc}"[:40], fill=(200, 0, 0), font=font)
            sheet.paste(cell, (x, y))
            name = os.path.splitext(os.path.basename(f))[0]
            d.text((x + 2, y + tile + 2), name[: tile // 7], fill=(0, 0, 0), font=font)
        out = f"{prefix}-{s // per + 1:02d}.png"
        sheet.save(out)
        print(out, len(chunk))
    return 0


if __name__ == "__main__":
    sys.exit(main())
