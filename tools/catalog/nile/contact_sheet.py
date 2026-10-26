#!/usr/bin/env python3
"""Contact sheets for eyeballing images: a grid of labelled cells on a checkerboard (shows alpha).

    python3 -I tools/catalog/nile/contact_sheet.py OUT.png TILE COLS @cells.txt
        cells.txt: one "label|path" per line; cells flow left to right, top to bottom.
    Optional env-free flag: a 5th arg "white" draws on plain white instead of the checkerboard.
"""

from __future__ import annotations

import sys

from PIL import Image, ImageDraw, ImageFont

FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"


def checker(w: int, h: int) -> Image.Image:
    im = Image.new("RGB", (w, h), (236, 236, 236))
    d = ImageDraw.Draw(im)
    step = 12
    for y in range(0, h, step):
        for x in range(0, w, step):
            if (x // step + y // step) % 2:
                d.rectangle((x, y, x + step - 1, y + step - 1), fill=(214, 214, 214))
    return im


def main() -> int:
    out, tile, cols, spec = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    white = len(sys.argv) > 5 and sys.argv[5] == "white"
    with open(spec.lstrip("@"), encoding="utf-8") as f:
        cells = [line.rstrip("\n").split("|", 1) for line in f if line.strip()]
    font = ImageFont.truetype(FONT, 12)
    lab_h, pad = 16, 6
    rows = (len(cells) + cols - 1) // cols
    W = cols * (tile + pad) + pad
    H = rows * (tile + lab_h + pad) + pad
    sheet = Image.new("RGB", (W, H), (255, 255, 255))
    d = ImageDraw.Draw(sheet)
    for i, (label, path) in enumerate(cells):
        r, c = divmod(i, cols)
        x = pad + c * (tile + pad)
        y = pad + r * (tile + lab_h + pad)
        d.text((x, y), label[:34], fill=(20, 20, 20), font=font)
        try:
            im = Image.open(path)
            im.load()
            im = im.convert("RGBA")
            im.thumbnail((tile, tile), Image.LANCZOS)
            cell = Image.new("RGB", (tile, tile), (255, 255, 255)) if white else checker(tile, tile)
            cell.paste(im, ((tile - im.size[0]) // 2, (tile - im.size[1]) // 2), im)
        except Exception as exc:
            cell = Image.new("RGB", (tile, tile), (255, 220, 220))
            ImageDraw.Draw(cell).text((4, 4), f"ERR {exc}"[:40], fill=(200, 0, 0), font=font)
        sheet.paste(cell, (x, y + lab_h))
    sheet.save(out)
    print(out, sheet.size)
    return 0


if __name__ == "__main__":
    sys.exit(main())
