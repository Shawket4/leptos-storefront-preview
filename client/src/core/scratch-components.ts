// Dev only (features/core.ts declares it under __DEV__): /core-scratch's shared components, driven
// the way the cart's features will — templates cloned with fill(), the timeline moved with
// setTimeline, the live status with setLiveStatus / retryIn. Values come from the page's JSON.
import { $, fill, on } from "./dom";
import { mountAll, register } from "./mount";
import { setTimeline } from "./timeline";
import { retryIn, setLiveStatus } from "./live-status";

interface Fill {
  sums: Record<string, string | null>;
  row: Record<string, string | null>;
  advance: { times: (string | null)[] };
}

register("scratch-components", (root) => {
  const data = JSON.parse($("#scratch-fill", root)?.textContent ?? "{}") as Fill;
  $("#sums-target", root)?.append(fill("scratch-sums", data.sums));
  const rows = $("#rows-target", root);
  if (rows) {
    rows.append(fill("scratch-row", data.row));
    mountAll(rows); // the stepper inside the clone
  }
  const live = $("#live-driven .live-status", root);
  return on(root, "click", "[data-scratch-action]", (_ev, b) => {
    const timeline = $("#sc-timeline", root);
    switch (b.dataset.scratchAction) {
      case "advance":
        if (timeline) setTimeline(timeline, { progress: 0.75, states: ["done", "done", "done", "current", "todo"], times: data.advance.times });
        break;
      case "stop":
        if (timeline) setTimeline(timeline, { stopped: true });
        break;
      case "retry":
        if (live) retryIn(live, 5000, () => setLiveStatus(live, "live"));
        break;
      case "offline":
        if (live) setLiveStatus(live, "offline");
        break;
      case "live":
        if (live) setLiveStatus(live, "live");
        break;
    }
  });
});
