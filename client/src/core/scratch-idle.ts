// Dev only: a dummy `idle` on-intent feature for /core-scratch.
import { register } from "./mount";

const w = window as unknown as { __scratchIdle?: { loaded: number; mounts: number } };
const s = (w.__scratchIdle ??= { loaded: 0, mounts: 0 });
s.loaded++;

register("scratch-idle", (el) => {
  s.mounts++;
  el.dataset.mounted = "yes";
});
