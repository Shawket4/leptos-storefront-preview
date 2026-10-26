#!/usr/bin/env python3
"""Small derivative edits of licensed originals before process.py (Pillow only).

    python3 -I tools/catalog/nile/cutout.py crop SRC OUT x0 y0 x1 y1
        crop a box given as fractions of the width/height (keeps alpha)
    python3 -I tools/catalog/nile/cutout.py bg-white SRC OUT [thresh]
        make a near-white backdrop transparent: flood-fill from the border over pixels within
        `thresh` (default 14) of white, then feather the edge by 1 px
    python3 -I tools/catalog/nile/cutout.py fg-seed SRC OUT seedx seedy level
        keep the bright object containing the seed (fractions) on a dark backdrop: threshold at
        `level` (0–255 luma), flood-fill from the seed, fill interior holes, feather 1 px
"""

from __future__ import annotations

import sys

from PIL import Image, ImageDraw, ImageFilter

MARK = 128


def save(im: Image.Image, out: str) -> None:
    im.save(out, "PNG", optimize=True)
    print(out, im.size, im.mode)


def fill_holes(mask: Image.Image) -> Image.Image:
    """mask: L with 255 = object, 0 = background. Pixels not reachable from the border become object."""
    inv = mask.point(lambda v: 0 if v else 255)  # background = 255
    w, h = inv.size
    for xy in ((0, 0), (w - 1, 0), (0, h - 1), (w - 1, h - 1)):
        if inv.getpixel(xy) == 255:
            ImageDraw.floodfill(inv, xy, MARK)
    # MARK = real background, 255 = enclosed holes, 0 = object
    return inv.point(lambda v: 0 if v == MARK else 255)


def feather(mask: Image.Image) -> Image.Image:
    return mask.filter(ImageFilter.MinFilter(3)).filter(ImageFilter.GaussianBlur(1))


def main() -> int:
    mode, src, out = sys.argv[1], sys.argv[2], sys.argv[3]
    im = Image.open(src)
    im.load()
    if mode == "crop":
        x0, y0, x1, y1 = (float(v) for v in sys.argv[4:8])
        w, h = im.size
        save(im.crop((round(x0 * w), round(y0 * h), round(x1 * w), round(y1 * h))), out)
    elif mode == "bg-white":
        thresh = int(sys.argv[4]) if len(sys.argv) > 4 else 14
        rgb = im.convert("RGB")
        work = rgb.copy()
        w, h = work.size
        marker = (255, 0, 255)
        step = max(1, min(w, h) // 40)
        seeds = [(x, 0) for x in range(0, w, step)] + [(x, h - 1) for x in range(0, w, step)] + \
                [(0, y) for y in range(0, h, step)] + [(w - 1, y) for y in range(0, h, step)]
        for xy in seeds:
            px = work.getpixel(xy)
            if px != marker and min(px) >= 255 - thresh:
                ImageDraw.floodfill(work, xy, marker, thresh=thresh)
        bg = Image.new("L", (w, h), 0)
        bg.putdata([255 if p == marker else 0 for p in work.getdata()])
        mask = feather(bg.point(lambda v: 0 if v else 255))
        rgba = rgb.convert("RGBA")
        rgba.putalpha(mask)
        save(rgba, out)
    elif mode == "fg-seed":
        sx, sy, level = float(sys.argv[4]), float(sys.argv[5]), int(sys.argv[6])
        rgb = im.convert("RGB")
        w, h = rgb.size
        binary = rgb.convert("L").point(lambda v: 255 if v >= level else 0)
        seed = (round(sx * w), round(sy * h))
        if binary.getpixel(seed) != 255:
            raise SystemExit(f"seed {seed} is not above the threshold")
        ImageDraw.floodfill(binary, seed, MARK)
        obj = binary.point(lambda v: 255 if v == MARK else 0)
        mask = feather(fill_holes(obj))
        bbox = mask.getbbox()
        rgba = rgb.convert("RGBA")
        rgba.putalpha(mask)
        save(rgba.crop(bbox), out)
    else:
        raise SystemExit(f"unknown mode {mode}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
