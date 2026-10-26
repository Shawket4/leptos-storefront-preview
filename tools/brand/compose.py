"""SVG composition helpers: colour a Shape, measure what resvg actually draws, frame it tightly.

Measuring the rendered alpha (instead of computing stroke outlines by hand) keeps every viewBox
exact: round caps, joins and dots are all included, and nothing is clipped.
"""

from __future__ import annotations

import os
import subprocess
import tempfile

from PIL import Image

from geometry import Shape, f

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
# Lettering (OG taglines) is set in Cairo (OFL, Arabic + Latin), the storefront's UI face. resvg
# reads only this folder, never system fonts, so renders are the same on every machine that has it.
FONTS = os.environ.get("BRAND_FONTS_DIR", os.path.expanduser("~/.local/share/fonts/cairo"))


def resvg(svg_path: str, png_path: str, width: int | None = None, height: int | None = None) -> None:
    cmd = ["resvg", svg_path, png_path, "--use-fonts-dir", FONTS, "--skip-system-fonts"]
    if width:
        cmd += ["-w", str(width)]
    if height:
        cmd += ["-h", str(height)]
    subprocess.run(cmd, check=True, capture_output=True)


def shape_svg(shape: Shape, stroke: str, sw: float, dot_colors: dict[str, str], transform: str = "") -> str:
    t = f' transform="{transform}"' if transform else ""
    out = [f"<g{t}>"]
    if shape.paths:
        out.append(
            f'<g fill="none" stroke="{stroke}" stroke-width="{f(sw)}" stroke-linecap="round" stroke-linejoin="round">'
            + "".join(f'<path d="{p}"/>' for p in shape.paths)
            + "</g>"
        )
    for p, w in shape.thin:
        out.append(f'<path d="{p}" fill="none" stroke="{dot_colors.get("mark", stroke)}" stroke-width="{f(w)}" stroke-linecap="round"/>')
    by_color: dict[str, list[str]] = {}
    for x, y, r, tag in shape.dots:
        by_color.setdefault(dot_colors.get(tag, stroke), []).append(f'<circle cx="{f(x)}" cy="{f(y)}" r="{f(r)}"/>')
    for color, circles in by_color.items():
        out.append(f'<g fill="{color}">' + "".join(circles) + "</g>")
    out.append("</g>")
    return "".join(out)


def doc(body: str, vb: tuple[float, float, float, float], width: float | None = None, height: float | None = None, title: str = "", desc: str = "") -> str:
    x, y, w, h = vb
    width = width if width is not None else w
    height = height if height is not None else h
    head = f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{f(x)} {f(y)} {f(w)} {f(h)}" width="{f(width)}" height="{f(height)}">'
    meta = (f"<title>{title}</title>" if title else "") + (f"<desc>{desc}</desc>" if desc else "")
    return head + meta + body + "</svg>\n"


def measure(body: str, vb=(-300.0, -150.0, 800.0, 400.0), scale: float = 4.0) -> tuple[float, float, float, float]:
    """Bounding box (x0, y0, x1, y1) in user units of everything `body` paints."""
    x, y, w, h = vb
    with tempfile.TemporaryDirectory() as tmp:
        sp, pp = os.path.join(tmp, "m.svg"), os.path.join(tmp, "m.png")
        with open(sp, "w") as fh:
            fh.write(doc(body, vb))
        resvg(sp, pp, width=int(w * scale))
        im = Image.open(pp).convert("RGBA")
        bb = im.getchannel("A").point(lambda a: 255 if a > 2 else 0).getbbox()
    if not bb:
        raise ValueError("nothing drawn")
    sx = im.width / w
    sy = im.height / h
    return (x + bb[0] / sx, y + bb[1] / sy, x + bb[2] / sx, y + bb[3] / sy)
