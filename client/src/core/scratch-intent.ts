// Dev only (features/core.ts declares it under __DEV__): a dummy on-intent feature for the
// /core-scratch page, counting imports and mounts so lazyOnIntent can be checked in a browser.
import { register } from "./mount";

const w = window as unknown as { __scratch?: { loaded: number; mounts: number; unmounts: number } };
const s = (w.__scratch ??= { loaded: 0, mounts: 0, unmounts: 0 });
s.loaded++;

register("scratch-intent", (el) => {
  s.mounts++;
  el.dataset.mounted = "yes";
  return () => {
    s.unmounts++;
    el.dataset.mounted = "no";
  };
});
