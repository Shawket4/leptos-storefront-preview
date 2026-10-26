// Gesture motion, the Apple way (`.claude/skills/apple-design`): springs instead of fixed curves,
// interruptible (a new target starts from the live value and velocity), flicks projected to where
// they'd come to rest, edges that rubber-band instead of stopping hard. Used by every gesture-driven
// piece (gallery swipe, drawers, sheets) — features don't write their own animation loops.

import { prefersReducedMotion } from "./dom";

export interface SpringConfig {
  /** 1 = no overshoot; below 1 bounces (0.8 only after a flick). */
  damping: number;
  /** Seconds to (nearly) settle. */
  response: number;
}

/** The design tokens' springs (`--spring-*` in tokens.css). */
export const SPRING = {
  default: { damping: 1, response: 0.35 },
  snappy: { damping: 1, response: 0.25 },
  gentle: { damping: 1, response: 0.5 },
  bouncy: { damping: 0.8, response: 0.3 },
} as const satisfies Record<string, SpringConfig>;

/** Advance displacement `x` (from the target) with velocity `v` by `dt` s — solved exactly. */
export function step(cfg: SpringConfig, x: number, v: number, dt: number): [number, number] {
  const w0 = (2 * Math.PI) / Math.max(cfg.response, 0.01);
  const z = Math.min(Math.max(cfg.damping, 0), 1);
  if (z >= 0.9999) {
    const b = v + w0 * x;
    const e = Math.exp(-w0 * dt);
    return [(x + b * dt) * e, (v - w0 * b * dt) * e];
  }
  const wd = w0 * Math.sqrt(1 - z * z);
  const e = Math.exp(-z * w0 * dt);
  const s = Math.sin(wd * dt), c = Math.cos(wd * dt);
  const b = (v + z * w0 * x) / wd;
  return [e * (x * c + b * s), e * ((b * wd - z * w0 * x) * c - (x * wd + z * w0 * b) * s)];
}

/** Where a flick at `velocity` px/s would come to rest (0.998 ≈ scrolling, 0.99 snappier). */
export const project = (velocity: number, deceleration = 0.998): number =>
  ((velocity / 1000) * deceleration) / (1 - deceleration);

/** How far an element follows a drag `overshoot` px past an edge (Apple's constant 0.55). */
export const rubberband = (overshoot: number, dimension: number, constant = 0.55): number => {
  const d = Math.max(dimension, 1);
  return (overshoot * d * constant) / (d + constant * Math.abs(overshoot));
};

/** Pointer velocity (px/s) from the last 100 ms of positions. */
export class VelocityTracker {
  private samples: Array<[t: number, x: number]> = [];
  add(x: number, timeStamp: number): void {
    this.samples.push([timeStamp, x]);
    while (this.samples.length > 2 && timeStamp - this.samples[0]![0] > 100) this.samples.shift();
  }
  velocity(): number {
    const first = this.samples[0], last = this.samples[this.samples.length - 1];
    if (!first || !last || last[0] === first[0]) return 0;
    return ((last[1] - first[1]) / (last[0] - first[0])) * 1000;
  }
  reset(): void {
    this.samples = [];
  }
}

/** A number animated by a spring; `onUpdate` draws each frame. */
export class SpringValue {
  private x: number;
  private v = 0;
  private target: number;
  private frame = 0;
  private last = 0;
  private cfg: SpringConfig = SPRING.default;
  private onRest: (() => void) | undefined;

  constructor(
    value: number,
    private readonly onUpdate: (value: number) => void,
  ) {
    this.x = this.target = value;
  }

  get value(): number {
    return this.x;
  }
  get velocity(): number {
    return this.v;
  }

  /** Follow a finger: set the value now, no animation (velocity comes from a tracker on release). */
  jump(value: number): void {
    this.stop();
    this.x = this.target = value;
    this.v = 0;
    this.onUpdate(value);
  }

  /** Spring to `target`, starting from the live value and velocity (or the release velocity). */
  to(target: number, opts: { velocity?: number; spring?: SpringConfig; onRest?: () => void } = {}): void {
    this.target = target;
    if (opts.velocity !== undefined) this.v = opts.velocity;
    this.cfg = opts.spring ?? SPRING.default;
    this.onRest = opts.onRest;
    if (prefersReducedMotion()) {
      this.jump(target);
      opts.onRest?.();
      return;
    }
    if (!this.frame) {
      this.last = performance.now();
      this.frame = requestAnimationFrame(this.tick);
    }
  }

  stop(): void {
    cancelAnimationFrame(this.frame);
    this.frame = 0;
  }

  private tick = (now: number): void => {
    const dt = Math.min((now - this.last) / 1000, 1 / 30);
    this.last = now;
    const [dx, v] = step(this.cfg, this.x - this.target, this.v, dt);
    this.x = this.target + dx;
    this.v = v;
    if (Math.abs(dx) < 0.5 && Math.abs(v) < 5) {
      this.x = this.target;
      this.v = 0;
      this.frame = 0;
      this.onUpdate(this.x);
      this.onRest?.();
      return;
    }
    this.onUpdate(this.x);
    this.frame = requestAnimationFrame(this.tick);
  };
}
