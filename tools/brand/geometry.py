"""Geometry of the tenant marks and wordmarks, in the construction of Madar's own logo.

Madar's wordmark (MadarDashboard/site/src/assets/brand/wordmark-primary.svg) is monoline:
stems, elliptical arches and circles on a baseline of 72 with an x-height line at 30, round caps
and joins. The tenant wordmarks below use the same skeleton (baseline 72, x-height 30, ascender
10, descender 92) with a lighter stroke (WORD_SW), so they read as siblings of Madar without
copying it. Arabic is drawn on the same baseline, right to left (x decreases), with the joins,
teeth, bowls and dots of IBM Plex Sans Arabic's skeleton, simplified to the same primitives.

Every function returns plain SVG path data (stroke-only) plus dots (filled circles), so the
build script can colour, scale and place them. Units are arbitrary; the marks live in a
100 x 100 box.
"""

from __future__ import annotations

import math
from dataclasses import dataclass, field

WORD_SW = 11.0  # wordmark stroke (Madar's own is 13.5 on the same 42-unit x-height)
MARK_SW = 10.0  # mark stroke in its 100-unit box
DOT_R = WORD_SW * 0.62  # i-dot / Arabic dot radius: a touch larger than the stroke reads equal


def f(v: float) -> str:
    """Compact number for SVG: two decimals, no trailing zeros."""
    s = f"{v:.2f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


@dataclass
class Shape:
    """Stroked paths + filled dots, with a tag per dot so a dot can take its own colour."""

    paths: list[str] = field(default_factory=list)
    dots: list[tuple[float, float, float, str]] = field(default_factory=list)  # x, y, r, tag
    thin: list[tuple[str, float]] = field(default_factory=list)  # (path, stroke width)

    def extend(self, other: "Shape") -> "Shape":
        self.paths += other.paths
        self.dots += other.dots
        self.thin += other.thin
        return self


def ell(cx: float, cy: float, rx: float, ry: float, deg: float) -> tuple[float, float]:
    t = math.radians(deg)
    return cx + rx * math.cos(t), cy + ry * math.sin(t)


# --------------------------------------------------------------------------------------------
# Latin letters. Each takes the x of its left edge (stroke centre line) and returns (paths, right).
# --------------------------------------------------------------------------------------------
ARCH_RX, ARCH_RY = 15.0, 11.0  # n / u: Madar's m arch is A14 10 on a 28 pitch; ours is a touch rounder


def lat_n(x):
    return [f"M{f(x)} 30V72", f"M{f(x)} {f(30 + ARCH_RY)}A{f(ARCH_RX)} {f(ARCH_RY)} 0 0 1 {f(x + 2 * ARCH_RX)} {f(30 + ARCH_RY)}V72"], x + 2 * ARCH_RX


def lat_u(x):
    y = 72 - ARCH_RY
    return [f"M{f(x)} 30V{f(y)}A{f(ARCH_RX)} {f(ARCH_RY)} 0 0 0 {f(x + 2 * ARCH_RX)} {f(y)}", f"M{f(x + 2 * ARCH_RX)} 30V72"], x + 2 * ARCH_RX


def lat_i(x):
    cy = 10 - WORD_SW / 2 + DOT_R  # dot top on the ascender line
    return [f"M{f(x)} 30V72"], (x, cy, DOT_R)


def lat_l(x):
    return [f"M{f(x)} 10V72"], x


def lat_o(xl, r=21.0):
    cx = xl + r
    return [f"M{f(cx + r)} 51A{f(r)} {f(r)} 0 1 0 {f(cx - r)} 51A{f(r)} {f(r)} 0 1 0 {f(cx + r)} 51"], cx + r


def lat_e(xl, r=21.0, aperture=38.0):
    cx = xl + r
    ex, ey = ell(cx, 51, r, r, aperture)
    return [f"M{f(xl)} 51H{f(cx + r)}A{f(r)} {f(r)} 0 1 0 {f(ex)} {f(ey)}"], cx + r


def lat_q(xl, r=21.0):
    paths, right = lat_o(xl, r)
    return paths + [f"M{f(right)} 30V92"], right


def lat_s(xl, rxu=12.5, ryu=10.0, rxl=13.5, ryl=11.0, top=-32.0, bottom=148.0):
    """Two stacked half-ellipses joined at a horizontal tangent (upper bowl a little smaller)."""
    cx = xl + rxl
    cu, cl = 30 + ryu, 72 - ryl
    sx, sy = ell(cx, cu, rxu, ryu, top)
    ex, ey = ell(cx, cl, rxl, ryl, bottom)
    return [
        f"M{f(sx)} {f(sy)}A{f(rxu)} {f(ryu)} 0 1 0 {f(cx)} {f(cu + ryu)}"
        f"A{f(rxl)} {f(ryl)} 0 1 1 {f(ex)} {f(ey)}"
    ], cx + rxl


# Spacing on stroke centre lines (white = gap - WORD_SW): straight|straight, straight|round, round|round.
GAP_SS, GAP_SR, GAP_RR = 22.0, 19.5, 16.5


def word_nile() -> Shape:
    s = Shape()
    p, x = lat_n(0)
    s.paths += p
    x += GAP_SS
    p, (dx, dy, dr) = lat_i(x)
    s.paths += p
    s.dots.append((dx, dy, dr, "accent"))  # the tittle carries the tenant colour
    x += GAP_SS
    p, x = lat_l(x)
    s.paths += p
    x += GAP_SR
    p, x = lat_e(x)
    s.paths += p
    return s


def word_souq() -> Shape:
    s = Shape()
    p, x = lat_s(0)
    s.paths += p
    x += GAP_RR
    p, x = lat_o(x)
    s.paths += p
    x += GAP_SR
    p, x = lat_u(x)
    s.paths += p
    x += GAP_SR
    p, x = lat_q(x)
    s.paths += p
    return s


# --------------------------------------------------------------------------------------------
# Arabic. Baseline 72; teeth rise to 50, alef / lam to 26 (Madar's alef is 44 over a 42 x-height).
# x decreases right to left; every word starts at x = 0 on its right edge.
# --------------------------------------------------------------------------------------------
TOOTH = 50.0
TALL = 26.0
TURN = 8.0  # radius of the turn from a tooth or stem into the baseline


def dot_pair(cx, cy, r=DOT_R, gap=2.6):
    off = r + gap / 2
    return [(cx - off, cy, r, "ink"), (cx + off, cy, r, "ink")]


def word_nayl() -> Shape:
    """نايل: noon (initial) joined to alef (final); yeh (initial) joined to lam (final)."""
    s = Shape()
    n = 0.0  # noon tooth
    a = n - 24.0  # alef
    s.paths.append(
        f"M{f(n)} {f(TOOTH)}V{f(72 - TURN)}Q{f(n)} 72 {f(n - TURN)} 72"
        f"H{f(a + TURN)}Q{f(a)} 72 {f(a)} {f(72 - TURN)}V{f(TALL)}"
    )
    s.dots.append((n, TOOTH - WORD_SW / 2 - 5.0 - DOT_R, DOT_R, "ink"))
    y = a - 21.0  # yeh tooth (alef does not join to the left)
    lam = y - 26.0
    bowl = 15.0
    s.paths.append(f"M{f(y)} {f(TOOTH)}V{f(72 - TURN)}Q{f(y)} 72 {f(y - TURN)} 72H{f(lam)}")
    s.paths.append(f"M{f(lam)} {f(TALL)}V72A{f(bowl)} {f(bowl)} 0 0 1 {f(lam - 2 * bowl)} 72V{f(72 - 9)}")
    s.dots += dot_pair(y - 1.0, 72 + WORD_SW / 2 + 5.0 + DOT_R)
    return s


def word_suq() -> Shape:
    """سوق: seen (initial, three teeth) joined to waw (final); qaf isolated."""
    s = Shape()
    tooth = 52.0
    w = 16.0  # tooth pitch
    s1, s2, s3 = 0.0, -w, -2 * w
    cup = 8.0
    for t in (s1, s2):
        s.paths.append(f"M{f(t)} {f(tooth)}V{f(72 - cup)}")
    s.paths.append(f"M{f(s1)} {f(72 - cup)}A{f(w / 2)} {f(cup)} 0 0 1 {f(s2)} {f(72 - cup)}")
    s.paths.append(f"M{f(s2)} {f(72 - cup)}A{f(w / 2)} {f(cup)} 0 0 1 {f(s3)} {f(72 - cup)}")
    # waw: loop sitting on the baseline, tail sweeping down-left from its right side
    wr = 11.0
    wx = s3 - 21.0
    s.paths.append(f"M{f(s3)} {f(tooth)}V{f(72 - TURN)}Q{f(s3)} 72 {f(s3 - TURN)} 72H{f(wx + wr)}")
    s.paths.append(f"M{f(wx + wr)} {f(72 - wr)}A{f(wr)} {f(wr)} 0 1 0 {f(wx - wr)} {f(72 - wr)}A{f(wr)} {f(wr)} 0 1 0 {f(wx + wr)} {f(72 - wr)}")
    s.paths.append(f"M{f(wx + wr)} {f(72 - wr)}V74Q{f(wx + wr)} 89 {f(wx - 4)} 93")
    # qaf: loop + deep bowl + two dots
    qr, qR = 11.0, 19.0
    qx = wx - 37.0
    s.paths.append(f"M{f(qx + qr)} {f(72 - qr)}A{f(qr)} {f(qr)} 0 1 0 {f(qx - qr)} {f(72 - qr)}A{f(qr)} {f(qr)} 0 1 0 {f(qx + qr)} {f(72 - qr)}")
    s.paths.append(f"M{f(qx + qr)} {f(72 - qr)}V72A{f(qR)} {f(qR)} 0 0 1 {f(qx + qr - 2 * qR)} 72V{f(72 - 9)}")
    s.dots += dot_pair(qx, 72 - 2 * qr - WORD_SW / 2 - 5.0 - DOT_R)
    return s


# --------------------------------------------------------------------------------------------
# Marks (100 x 100 box, stroke MARK_SW)
# --------------------------------------------------------------------------------------------
def meander_n(x0=22.0, x1=78.0, ytop=22.0, ybot=78.0, rc=14.0):
    """The Nile: a river bend that is also an N. Left stem rises, turns over (radius rc), runs down
    the tangent between the two turns, turns under, and rises again."""
    c1 = (x0 + rc, ytop + rc)
    c2 = (x1 - rc, ybot - rc)
    mx, my = (c1[0] + c2[0]) / 2, (c1[1] + c2[1]) / 2
    dx, dy = mx - c1[0], my - c1[1]
    dist = math.hypot(dx, dy)
    a1 = math.atan2(dy, dx) - math.acos(rc / dist)
    t1 = (c1[0] + rc * math.cos(a1), c1[1] + rc * math.sin(a1))
    t2 = (2 * mx - t1[0], 2 * my - t1[1])
    d = (
        f"M{f(x0)} {f(ybot)}V{f(c1[1])}A{f(rc)} {f(rc)} 0 0 1 {f(t1[0])} {f(t1[1])}"
        f"L{f(t2[0])} {f(t2[1])}A{f(rc)} {f(rc)} 0 0 0 {f(x1)} {f(c2[1])}V{f(ytop)}"
    )
    return d, c1, t1, t2


def mark_nile() -> Shape:
    x0, x1, ytop, ybot, rc = 22.0, 78.0, 22.0, 78.0, 11.0
    d, c1, t1, t2 = meander_n(x0, x1, ytop, ybot, rc)
    s = Shape(paths=[d])
    # The sun / satellite: rests in the bend's upper pocket, top on the stems' cap line, equally
    # clear of the right stem and of the over-turn (solved numerically).
    r = 6.6
    cy = ytop - MARK_SW / 2 + r
    lo, hi = c1[0], x1
    for _ in range(60):
        cx = (lo + hi) / 2
        to_stem = (x1 - MARK_SW / 2) - (cx + r)
        # nearest distance to the over-turn arc / diagonal centre line, minus half stroke and r
        d_arc = math.hypot(cx - c1[0], cy - c1[1]) - rc
        # distance to the diagonal segment t1->t2
        vx, vy = t2[0] - t1[0], t2[1] - t1[1]
        u = max(0.0, min(1.0, ((cx - t1[0]) * vx + (cy - t1[1]) * vy) / (vx * vx + vy * vy)))
        d_seg = math.hypot(cx - (t1[0] + u * vx), cy - (t1[1] + u * vy))
        # only the arc part beyond t1 counts as arc (angles from -90 to a1); the segment covers the rest
        to_turn = min(d_arc if cy <= t1[1] else 1e9, d_seg) - MARK_SW / 2 - r
        if to_stem > to_turn:
            lo = cx
        else:
            hi = cx
    s.dots.append((cx, cy, r, "mark"))
    return s


def keel(cx=50.0, apex=21.5, half=24.0, phi_deg=60.0, base=78.5):
    """Fatimid keel arch (Cairo, al-Aqmar / Bab Zuwayla): quarter-round shoulders of radius
    `half` centred on the springing line, then straight tangents that meet in a shallow keel."""
    phi = math.radians(phi_deg)
    r = half
    spring = apex + r / math.sin(phi)
    lx, ly = cx - r * math.cos(phi), spring - r * math.sin(phi)
    d = (
        f"M{f(cx - r)} {f(base)}V{f(spring)}A{f(r)} {f(r)} 0 0 1 {f(lx)} {f(ly)}L{f(cx)} {f(apex)}"
        f"L{f(2 * cx - lx)} {f(ly)}A{f(r)} {f(r)} 0 0 1 {f(cx + r)} {f(spring)}V{f(base)}"
    )
    return d, spring, phi


def mark_souq() -> Shape:
    apex = 21.5
    d, spring, phi = keel(apex=apex)
    s = Shape(paths=[d])
    inner_apex = apex + (MARK_SW / 2) / math.sin(phi)
    r = 7.0
    cy = 46.5
    s.thin.append((f"M50 {f(inner_apex - 1.5)}V{f(cy - r + 1)}", 3.6))  # the lantern's chain
    s.dots.append((50.0, cy, r, "mark"))
    return s


# The band each word is centred on in a lockup: from its tallest regular feature (ascender, alef,
# the qaf's dots) down to the baseline's stroke edge. Descenders hang below it, as in type.
BASE_EDGE = 72 + WORD_SW / 2
WORDS = {
    "nile": {
        "en": (word_nile, (10 - WORD_SW / 2, BASE_EDGE)),
        "ar": (word_nayl, (TALL - WORD_SW / 2, BASE_EDGE)),
    },
    "souq": {
        "en": (word_souq, (30 - WORD_SW / 2, BASE_EDGE)),
        "ar": (word_suq, (72 - 2 * 11.0 - WORD_SW / 2 - 5.0 - 2 * DOT_R, BASE_EDGE)),
    },
}
MARKS = {"nile": mark_nile, "souq": mark_souq}
