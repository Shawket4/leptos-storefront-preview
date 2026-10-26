// The interactive specimens on /ui (the design-system kit): `data-ts="kit-…"`, mounted on
// server-rendered markup, text written through `patch()` into `[data-slot]`s. Strings come from
// `strings/showcase.rs` through `t()`. (The /showcase tour's demos are features/showcase.ts.)

import { register } from "../core/mount";
import { $, offAll, on, patch, reveal } from "../core/dom";
import { digits, t } from "../core/page";
import { stepperValue } from "../core/stepper";
import { setTimeline, type TimelineState } from "../core/timeline";
import { retryIn, setLiveStatus } from "../core/live-status";
import { toast } from "../stores/toasts";
import { previewNet } from "./core";

register("kit-toasts", (root) => {
  reveal(root);
  return on(root, "click", "button[data-toast]", (_, b) => {
    const kind = b.dataset.toast;
    if (kind === "success") toast(t("showcase.toast_success"), "success");
    else if (kind === "error") toast(t("showcase.toast_error"), "error");
    else if (kind === "action")
      toast(t("showcase.toast_removed"), "info", { label: t("showcase.undo"), run: () => toast(t("showcase.toast_back"), "success") });
    else toast(t("showcase.toast_info", { n: digits(14) }), "info");
  });
});

// A stepper with a line total: the totals for each quantity come formatted from the server (the
// client never computes money); the stepper itself is core's.
register("kit-total", (root) => {
  let totals: Record<string, string> = {};
  try {
    totals = JSON.parse($("script[data-ts-json]", root)?.textContent ?? "{}") as Record<string, string>;
  } catch {
    totals = {};
  }
  const show = () => {
    const step = $("[data-ts~=stepper]", root);
    if (step) patch(root, { total: totals[String(stepperValue(step))] ?? "" });
  };
  return on(root, "change", "input", show);
});

register("kit-remove", (root) =>
  on(root, "change", "input", (_, input) => {
    if ((input as HTMLInputElement).valueAsNumber === 0) toast(t("showcase.toast_removed"), "info");
  }),
);

// Skeleton pairs: replay the load in place and measure the layout shift it causes (should be 0).
register("kit-skeletons", (root) => {
  reveal(root);
  let shift = 0;
  let timer = 0;
  const po =
    "PerformanceObserver" in self && PerformanceObserver.supportedEntryTypes?.includes("layout-shift")
      ? new PerformanceObserver((list) => {
          for (const e of list.getEntries() as Array<PerformanceEntry & { value: number; hadRecentInput: boolean }>)
            if (!e.hadRecentInput) shift += e.value;
        })
      : null;
  const play = () => {
    clearTimeout(timer);
    shift = 0;
    po?.observe({ type: "layout-shift", buffered: false });
    root.dataset.replay = "loading";
    patch(root, { cls: t("showcase.loading") });
    timer = window.setTimeout(() => {
      root.dataset.replay = "loaded";
      window.setTimeout(() => {
        po?.disconnect();
        patch(root, { cls: po ? t("showcase.cls", { n: digits(shift.toFixed(3)) }) : "—" });
      }, 300);
    }, 1200);
  };
  return offAll([() => clearTimeout(timer), () => po?.disconnect(), on(root, "click", "button[data-act=replay]", play)]);
});

// Network states: stage the real offline banner, failure alert and progress bar through core's
// `previewNet` (features/core.ts); nothing is really disconnected.
register("kit-net", (root) => {
  reveal(root);
  return on(root, "click", "button[data-net]", (_, b) => {
    const v = b.dataset.net ?? "";
    if (v === "offline" || v === "online") previewNet(v);
    else if (v.startsWith("fail:")) previewNet({ fail: v.slice(5) as "offline" | "timeout" | "server" });
    else previewNet({ progress: Number(v.split(":")[1]) || 1800 });
  });
});

// A notice the browser reveals later: rendered hidden with `slots`, then written with `patch()`.
register("kit-notice", (root) => {
  reveal(root);
  return on(root, "click", "button[data-act=reveal]", () => {
    const n = $('[data-state="added"]', root);
    if (!n) return;
    patch(n, { title: t("showcase.notice_title"), body: t("showcase.notice_body") });
    n.hidden = false;
  });
});

// The field's error line is always there, empty: filling it moves nothing on the page.
register("kit-check", (root) => {
  reveal(root);
  return on(root, "click", "button[data-act=check]", () => {
    const input = $<HTMLInputElement>("input", root);
    if (!input) return;
    const bad = input.value.trim().length < 3;
    const line = root.querySelector(`[id="${input.id}-error"]`);
    if (line) line.textContent = bad ? t("showcase.check_error") : "";
    input.setAttribute("aria-invalid", String(bad));
  });
});

// The order timeline, moved the way order tracking moves it (core/timeline.ts). Times come
// formatted from the server; the current stage says "Now".
register("kit-timeline", (root) => {
  let data: { times: string[]; now: string } = { times: [], now: "" };
  try {
    data = JSON.parse($("script[data-ts-json]", root)?.textContent ?? "{}") as typeof data;
  } catch {
    /* the server always renders it; keep the empty default */
  }
  const START = 2;
  let cur = START;
  const states = (): TimelineState[] => [0, 1, 2, 3, 4].map((i) => (i < cur ? "done" : i === cur ? "current" : "todo"));
  const times = () => [0, 1, 2, 3, 4].map((i) => (i < cur ? (data.times[i] ?? null) : i === cur ? data.now : null));
  const show = () => setTimeline(root, { progress: cur / 4, states: states(), times: times(), stopped: false });
  reveal(root);
  return on(root, "click", "button[data-act]", (_, b) => {
    if (b.dataset.act === "advance") cur = Math.min(cur + 1, 4);
    else if (b.dataset.act === "reset") cur = START;
    else return setTimeline(root, { stopped: true });
    show();
  });
});

// The live status, driven by core/live-status.ts: a failed poll counts down to its retry.
register("kit-live", (root) => {
  let cancel: (() => void) | null = null;
  const stop = () => {
    cancel?.();
    cancel = null;
  };
  reveal(root);
  return offAll([
    stop,
    on(root, "click", "button[data-act]", (_, b) => {
      stop();
      if (b.dataset.act === "retry") cancel = retryIn(root, 5000, () => setLiveStatus(root, "live"));
      else setLiveStatus(root, b.dataset.act === "offline" ? "offline" : "live");
    }),
  ]);
});
