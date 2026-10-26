"""Small curve helpers used while drawing the illustrations (run to print path data)."""

from __future__ import annotations

import math
import sys


def _n(v):
    s = f"{v:.2f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def catmull_rom(points, closed=False, tension=0.5):
    """Smooth path through points: Catmull-Rom converted to cubic Bezier segments."""
    pts = list(points)
    if closed:
        pts = [pts[-1]] + pts + [pts[0], pts[1]]
    else:
        pts = [pts[0]] + pts + [pts[-1]]
    out = [f"M{_n(pts[1][0])} {_n(pts[1][1])}"]
    k = tension / 3 * 2  # 1/6 * 2 * tension*... (0.5 -> standard Catmull-Rom, 1/6)
    for i in range(1, len(pts) - 2):
        p0, p1, p2, p3 = pts[i - 1], pts[i], pts[i + 1], pts[i + 2]
        c1 = (p1[0] + (p2[0] - p0[0]) * k / 2, p1[1] + (p2[1] - p0[1]) * k / 2)
        c2 = (p2[0] - (p3[0] - p1[0]) * k / 2, p2[1] - (p3[1] - p1[1]) * k / 2)
        out.append(f"C{_n(c1[0])} {_n(c1[1])} {_n(c2[0])} {_n(c2[1])} {_n(p2[0])} {_n(p2[1])}")
    return "".join(out)


def iso_box(front, a, b, h):
    """Corners of an isometric (30 degree) box from its front-bottom corner."""
    c = math.cos(math.radians(30))
    fx, fy = front
    r0 = (fx + a * c, fy - a / 2)
    l0 = (fx - b * c, fy - b / 2)
    return {
        "F0": front, "R0": r0, "L0": l0,
        "F1": (fx, fy - h), "R1": (r0[0], r0[1] - h), "L1": (l0[0], l0[1] - h),
        "B1": (fx + a * c - b * c, fy - a / 2 - b / 2 - h),
    }


if __name__ == "__main__":
    pts = [tuple(map(float, p.split(","))) for p in sys.argv[1:]]
    print(catmull_rom(pts))
