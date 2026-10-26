// The shell's progressive enhancement (inlined once by components::shell, no wasm). Everything is
// a delegated listener, so it keeps working while the islands router patches pages in.
(() => {
  const d = document, w = window, H = history;
  const reduced = () => w.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const ofetch = w.fetch.bind(w);
  const keyOf = (u) => { const x = new URL(u, location.href); x.hash = ""; return x.href; };
  // A link the islands router will follow (same origin, plain left click, no target/download).
  const routable = (a) => {
    if (!a || !a.getAttribute("href") || a.target || a.hasAttribute("download") || /\bexternal\b/.test(a.rel)) return null;
    const u = new URL(a.href, location.href);
    return u.origin === location.origin ? u : null;
  };

  // ── Prefetch: Madar's gesture model (MadarDashboard/src/lib/prefetch/intent.ts) ──────────
  // One delegated listener set for every router-eligible link, so no link wires anything:
  // - a mouse or pen hover warms after HOVER_DELAY_MS (a pass-over warms nothing); one pending
  //   hover at a time — entering another target or leaving cancels it;
  // - keyboard focus waits the same; blur cancels;
  // - pointerdown warms at once (a phone's only signal; on desktop it buys the ~100 ms to the
  //   click) and is never throttled;
  // - the same href is warmed at most once per DEDUPE_MS; hovers and focus warm at most
  //   HOVER_BUDGET_PER_S targets a second; nothing is warmed on Save-Data or 2G.
  // Warming is reading: the router's own GET (Islands-Router header), kept fresh 30 s (Madar's
  // staleTime) and single-use — the router's fetch of that href gets it and it is dropped.
  // /__ links set preferences (state-changing GETs): never warmed. A failed warm costs nothing.
  const HOVER_DELAY_MS = 65, DEDUPE_MS = 2000, HOVER_BUDGET_PER_S = 6, STALE_MS = 30000;
  const stats = { warmed: 0, reused: 0, skipped: 0 };
  w.__lspPrefetch = { HOVER_DELAY_MS, DEDUPE_MS, HOVER_BUDGET_PER_S, stats };
  const connectionAllowsPrefetch = () => {
    const c = navigator.connection;
    return !c || (!c.saveData && !/(^|-)2g$/.test(c.effectiveType || ""));
  };
  const warmable = (a) => {
    const u = routable(a);
    if (!u || u.pathname.startsWith("/__")) return null;
    // Not the page we're on (nor a hash-only jump within it).
    if (u.pathname === location.pathname && u.search === location.search) return null;
    return keyOf(u.href);
  };
  // Warmed pages are whole documents (Madar's are small JSON), so few are kept: the stale and
  // then the oldest are dropped, their unread bodies cancelled.
  const PRE_MAX = 8;
  const pre = new Map(); // href → { at, promise }, oldest first
  const drop = (entry) => entry.promise.then((r) => { if (r.body) r.body.cancel().catch(() => {}); }, () => {});
  const keep = (key, entry) => {
    const old = pre.get(key);
    if (old) { pre.delete(key); drop(old); }
    for (const [k, v] of pre) if (entry.at - v.at >= STALE_MS) { pre.delete(k); drop(v); }
    while (pre.size >= PRE_MAX) { const [k, v] = pre.entries().next().value; pre.delete(k); drop(v); }
    pre.set(key, entry);
  };
  const lastRun = new Map();
  const warmOnce = (key) => {
    if (!connectionAllowsPrefetch()) { stats.skipped++; return; }
    const now = Date.now();
    const at = lastRun.get(key);
    if (at !== undefined && now - at < DEDUPE_MS) { stats.skipped++; return; }
    lastRun.set(key, now);
    if (lastRun.size > 500) for (const [k, t] of lastRun) if (now - t >= DEDUPE_MS) lastRun.delete(k);
    const hit = pre.get(key);
    if (hit && now - hit.at < STALE_MS) return; // still fresh: nothing to do
    const promise = ofetch(new Request(key, { headers: { "Islands-Router": "true" }, cache: "no-cache" }));
    promise.catch(() => { if (pre.get(key)?.promise === promise) pre.delete(key); });
    keep(key, { at: now, promise });
    stats.warmed++;
  };
  let hoverPending = null;
  const cancelPending = () => { if (hoverPending !== null) { clearTimeout(hoverPending); hoverPending = null; } };
  const hoverWarms = [];
  const hoverBudget = () => {
    const now = Date.now();
    while (hoverWarms.length && now - hoverWarms[0] >= 1000) hoverWarms.shift();
    if (hoverWarms.length >= HOVER_BUDGET_PER_S) return false;
    hoverWarms.push(now);
    return true;
  };
  const schedule = (key) => {
    cancelPending();
    hoverPending = setTimeout(() => {
      hoverPending = null;
      if (hoverBudget()) warmOnce(key);
      else stats.skipped++;
    }, HOVER_DELAY_MS);
  };
  const linkOf = (t) => (t && t.closest ? t.closest("a[href]") : null);
  let hovered = null;
  d.addEventListener("pointerover", (e) => {
    // Touch has no hover: its pointerenter comes with the press, handled below.
    if (e.pointerType === "touch") return;
    const a = linkOf(e.target);
    if (a === hovered) return; // moving within the same link is not a new enter
    hovered = a;
    cancelPending();
    const key = warmable(a);
    if (key) schedule(key);
  }, true);
  d.addEventListener("pointerout", (e) => {
    const a = linkOf(e.target);
    if (a && a === hovered && !a.contains(e.relatedTarget)) { hovered = null; cancelPending(); }
  }, true);
  d.addEventListener("pointerdown", (e) => {
    cancelPending();
    const key = warmable(linkOf(e.target));
    if (key) warmOnce(key);
  }, { capture: true, passive: true });
  d.addEventListener("focusin", (e) => {
    const key = warmable(linkOf(e.target));
    if (key) schedule(key);
  });
  d.addEventListener("focusout", () => cancelPending());

  // ── Navigation progress ───────────────────────────────────────────────────
  // A thin bar at the top, only when a navigation takes longer than 150 ms; it completes when
  // the new page is swapped in (static under reduced motion, see shell.css).
  let barTimer = 0, barShown = false;
  const bar = () => d.querySelector(".nav-progress");
  const navBegin = () => {
    clearTimeout(barTimer);
    barShown = false;
    barTimer = setTimeout(() => { const b = bar(); if (b) { b.dataset.state = "loading"; barShown = true; } }, 150);
  };
  const navEnd = (ok) => {
    clearTimeout(barTimer);
    const b = bar();
    if (!b || !barShown) return;
    barShown = false;
    b.dataset.state = ok ? "done" : "idle";
    if (ok) setTimeout(() => { if (b.dataset.state === "done") b.dataset.state = "idle"; }, 320);
  };

  // ── A navigation that fails ───────────────────────────────────────────────
  // Offline, a 5xx, or 12 s without an answer: don't leave the visitor on the old page with no
  // word. Say so, offer Retry (re-navigate); on a second failure, fall back to a full navigation.
  const fails = new Map();
  const tag = (err) => { try { err.__nav = true; } catch (_) {} return err; };
  w.addEventListener("unhandledrejection", (e) => { if (e.reason && e.reason.__nav) e.preventDefault(); });
  const go = (url) => {
    const a = d.createElement("a");
    a.href = url;
    a.hidden = true;
    d.body.appendChild(a);
    a.click();
    a.remove();
  };
  const navFailed = (key, method, why) => {
    navEnd(false);
    const n = (fails.get(key) || 0) + 1;
    fails.set(key, n);
    if (n >= 2 && method === "GET") { location.assign(key); return; }
    showAlert(key, method, why);
  };
  const showAlert = (key, method, why) => {
    const el = d.querySelector(".nav-alert");
    if (!el) return;
    el.dataset.open = "true";
    const retry = el.querySelector(".nav-alert__retry");
    if (retry) {
      retry.hidden = method !== "GET";
      retry.onclick = () => { el.dataset.open = "false"; go(key); };
    }
    const close = el.querySelector(".nav-alert__close");
    if (close) close.onclick = () => { el.dataset.open = "false"; };
    // Set the reason after showing, so screen readers announce it.
    requestAnimationFrame(() => {
      const why_el = el.querySelector(".nav-alert__why");
      if (why_el) why_el.textContent = el.getAttribute("data-why-" + why) || "";
    });
  };

  // The design system's page (/ui) stages these states with buttons; nothing really fails.
  d.addEventListener("click", (e) => {
    const b = e.target.closest && e.target.closest("button[data-preview]");
    if (!b) return;
    const [what, arg] = b.dataset.preview.split(":");
    if (what === "progress") { navBegin(); setTimeout(() => navEnd(true), +arg || 1500); }
    else if (what === "alert") showAlert(location.href, "GET", arg || "network");
    else if (what === "offline" || what === "online") w.dispatchEvent(new Event(what));
  });

  // ── Keep the router's page diff aligned ───────────────────────────────────
  // The islands router walks the old and new documents in lockstep, <head> included. Pages carry
  // different head tags (title, meta, JSON-LD after leptos_meta's <!--HEAD--> marker), and when
  // their count differs the walk slips and the next page lands inside the old <head>. So: apply
  // the new page's head tags here, then hand the router HTML whose <head> matches the live one.
  const headTail = (head) => {
    const out = [];
    let seen = false;
    for (const n of head.childNodes) {
      if (seen) out.push(n);
      else if (n.nodeType === 8 && n.data.trim() === "HEAD") seen = true;
    }
    return seen ? out : null;
  };
  // <noscript> content is text in this (scripting) document but elements in the router's
  // DOMParser copy, so the diff would move them in. With JS running they are inert: empty them.
  const quiet = () => { for (const n of d.querySelectorAll("noscript")) n.textContent = ""; };
  quiet();
  w.fetch = async function (input) {
    const routed = input instanceof Request && input.headers.get("Islands-Router") === "true";
    if (!routed) return ofetch.apply(null, arguments);
    const key = keyOf(input.url);
    const method = input.method;
    navBegin();
    const ctrl = new AbortController();
    const timer = setTimeout(() => ctrl.abort(), 12000);
    const timeout = new Promise((_, no) => ctrl.signal.addEventListener("abort", () => no(Object.assign(new Error("timeout"), { name: "AbortError" }))));
    let resp = null;
    try {
      // A warm for this href, still fresh: hand the router a clone, once.
      const hit = method === "GET" ? pre.get(key) : null;
      pre.delete(key);
      if (hit && Date.now() - hit.at < STALE_MS) {
        try {
          resp = (await Promise.race([hit.promise, timeout])).clone();
          stats.reused++;
        } catch (err) {
          if (err.name === "AbortError" && ctrl.signal.aborted) throw err;
          resp = null; // a failed warm: fetch as usual
        }
      }
      // Pages vary by the language/numeral/theme cookies: router navigations always revalidate.
      if (!resp) resp = await Promise.race([ofetch(new Request(input, { cache: "no-cache", signal: ctrl.signal })), timeout]);
    } catch (err) {
      clearTimeout(timer);
      navFailed(key, method, !navigator.onLine ? "offline" : ctrl.signal.aborted ? "timeout" : "network");
      throw tag(err);
    }
    clearTimeout(timer);
    if (resp.status >= 500) {
      navFailed(key, method, "server");
      throw tag(new Error("HTTP " + resp.status));
    }
    fails.delete(key);
    const type = resp.headers.get("content-type") || "";
    if (!resp.ok || !type.includes("text/html")) return resp;
    quiet();
    const html = (await resp.text()).replace(/(<noscript\b[^>]*>)[\s\S]*?(<\/noscript>)/gi, "$1$2");
    let fixed = html;
    try {
      const doc = new DOMParser().parseFromString(html, "text/html");
      const oldTail = headTail(d.head), newTail = headTail(doc.head);
      const a = html.indexOf("<head>"), b = html.indexOf("</head>");
      if (oldTail && newTail && a >= 0 && b > a) {
        for (const n of oldTail) n.remove();
        for (const n of newTail) d.head.appendChild(d.importNode(n, true));
        // Splice the live head into the response text; everything else stays byte-identical.
        fixed = html.slice(0, a + 6) + d.head.innerHTML + html.slice(b);
      }
    } catch (_) {}
    return new Proxy(resp, {
      get(t, p) {
        if (p === "text") return () => Promise.resolve(fixed);
        const v = Reflect.get(t, p);
        return typeof v === "function" ? v.bind(t) : v;
      },
    });
  };

  // ── Scroll on router navigations ──────────────────────────────────────────
  // The islands router swaps pages without touching the scroll position. New page → top;
  // #hash → that section; back/forward → where you were. Prefs links keep their place.
  H.scrollRestoration = "manual";
  const skey = () => "sy:" + location.pathname + location.search;
  const save = () => { try { sessionStorage.setItem(skey(), String(Math.round(scrollY))); } catch (_) {} };
  const saved = () => { try { return +sessionStorage.getItem(skey()) || 0; } catch (_) { return 0; } };
  let pending = null;
  const after = () => {
    const p = pending;
    pending = null;
    navEnd(true);
    const alert = d.querySelector(".nav-alert");
    if (alert) alert.dataset.open = "false";
    if (p && p.hash) d.getElementById(p.hash)?.scrollIntoView({ block: "start", behavior: "instant" });
    else if (p && p.top) w.scrollTo({ top: 0, left: 0, behavior: "instant" });
    else if (p && p.y != null) w.scrollTo({ top: p.y, left: 0, behavior: "instant" });
    rails();
  };
  const svt = d.startViewTransition;
  if (svt) {
    // Run `after` inside the transition, right after the swap, so the new snapshot is in place.
    d.startViewTransition = function (arg) {
      const run = typeof arg === "function" ? arg : arg && arg.update;
      return svt.call(d, async () => { if (run) await run(); after(); });
    };
  }
  for (const m of ["pushState", "replaceState"]) {
    const orig = H[m];
    H[m] = function () { const r = orig.apply(this, arguments); if (!svt && pending) after(); return r; };
  }
  const nav = performance.getEntriesByType("navigation")[0];
  if (nav && (nav.type === "reload" || nav.type === "back_forward")) w.scrollTo(0, saved());
  let sr = 0;
  w.addEventListener("scroll", () => { if (!sr) sr = requestAnimationFrame(() => { sr = 0; save(); }); }, { passive: true });
  w.addEventListener("popstate", () => { pending = { y: saved() }; });

  d.addEventListener("click", (e) => {
    if (e.defaultPrevented || e.button || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
    const a = e.target.closest && e.target.closest("a[href]");
    const u = routable(a);
    if (!u) return;
    // A product card: name only its image for the card → product morph (a product can be on a
    // page twice, and duplicate view-transition names abort the transition).
    const media = a.closest(".pcard")?.querySelector("[data-vt]");
    if (media) {
      for (const x of d.querySelectorAll("[data-vt-named]")) { x.style.viewTransitionName = ""; x.removeAttribute("data-vt-named"); }
      media.style.viewTransitionName = media.dataset.vt;
      media.style.viewTransitionClass = "product-media";
      media.setAttribute("data-vt-named", "");
    }
    if (u.pathname.startsWith("/__")) return;
    save();
    pending = u.hash ? { hash: decodeURIComponent(u.hash.slice(1)) } : u.pathname !== location.pathname ? { top: true } : null;
  }, true);
  d.addEventListener("submit", (e) => {
    const f = e.target;
    if (!(f instanceof HTMLFormElement)) return;
    const u = new URL(f.action, location.href);
    if (u.origin !== location.origin) return;
    save();
    if (u.pathname !== location.pathname) pending = { top: true };
  }, true);

  // ── The button that submitted a form ──────────────────────────────────────
  // The islands router posts `new FormData(form)`, which leaves out the pressed button (a
  // `<button name="op" value="remove">`); without JS the browser sends it. Add it as a hidden
  // field for this one submission. (A "dialog" form isn't the router's: the browser submits it,
  // button included.)
  d.addEventListener("submit", (e) => {
    const f = e.target, b = e.submitter;
    if (!(f instanceof HTMLFormElement) || !b || !b.name || f.method === "dialog") return;
    const i = d.createElement("input");
    i.type = "hidden";
    i.name = b.name;
    i.value = b.value;
    f.appendChild(i);
    setTimeout(() => i.remove(), 0);
  }, true);

  // ── Preference and tenant links come back to where you are ────────────────
  // Their `next` is the page as the server rendered it, and an island may have moved the address
  // since (the listing filters push their state). At press time, point `next` at the live
  // address, so switching the language keeps the filters; the server's href stays the no-JS
  // link. A tenant link that sends elsewhere on purpose (the other shop's home, from a deep
  // page) keeps its `next`.
  const freshNext = (e) => {
    const a = e.target.closest && e.target.closest('a[href^="/__prefs"], a[href^="/__tenant/"]');
    if (!a) return;
    const u = new URL(a.href, location.href);
    const next = u.searchParams.get("next");
    if (next == null) return;
    const n = new URL(next, location.href);
    if (u.pathname.startsWith("/__tenant/") && n.pathname !== location.pathname) return;
    u.searchParams.set("next", location.pathname + location.search + n.hash);
    a.setAttribute("href", u.pathname + u.search);
  };
  d.addEventListener("pointerdown", freshNext, true);
  d.addEventListener("click", freshNext, true);

  // ── Popovers: <details data-popover> ──────────────────────────────────────
  // Light dismiss (outside press, Escape), one open at a time, and an exit that runs the entry
  // backwards (CSS [data-closing]) before the element really closes.
  const open = () => d.querySelectorAll("details[data-popover][open]");
  const close = (det, focus) => {
    if (!det.open || det.hasAttribute("data-closing")) return;
    const done = () => { det.removeAttribute("data-closing"); det.open = false; if (focus) det.querySelector("summary")?.focus(); };
    if (reduced()) return done();
    det.setAttribute("data-closing", "");
    setTimeout(done, 140);
  };
  d.addEventListener("pointerdown", (e) => { for (const det of open()) if (!det.contains(e.target)) close(det); }, true);
  d.addEventListener("keydown", (e) => { if (e.key === "Escape") for (const det of open()) close(det, true); });
  d.addEventListener("click", (e) => {
    const s = e.target.closest && e.target.closest("details[data-popover] > summary");
    if (s && s.parentElement.open) { e.preventDefault(); close(s.parentElement); return; }
    const a = e.target.closest && e.target.closest("details[data-popover] a[href]");
    if (a) close(a.closest("details"));
  });
  d.addEventListener("toggle", (e) => {
    const t = e.target;
    if (t.matches && t.matches("details[data-popover]") && t.open) for (const det of open()) if (det !== t) close(det);
  }, true);

  // ── Rails: prev/next buttons and their end states ─────────────────────────
  const railState = (r) => {
    const t = r && r.querySelector(".rail__track");
    if (!t) return;
    const max = t.scrollWidth - t.clientWidth, x = Math.abs(t.scrollLeft);
    r.dataset.at = max <= 2 ? "none" : x <= 2 ? "start" : x >= max - 2 ? "end" : "mid";
  };
  const rails = () => d.querySelectorAll("[data-rail]").forEach(railState);
  d.addEventListener("click", (e) => {
    const b = e.target.closest && e.target.closest("[data-rail-step]");
    const t = b && b.closest("[data-rail]")?.querySelector(".rail__track");
    if (!t) return;
    const dir = getComputedStyle(t).direction === "rtl" ? -1 : 1;
    t.scrollBy({ left: +b.dataset.railStep * t.clientWidth * 0.85 * dir, behavior: reduced() ? "auto" : "smooth" });
  });
  let rr = 0;
  d.addEventListener("scroll", (e) => {
    const t = e.target;
    if (t.classList && t.classList.contains("rail__track") && !rr) rr = requestAnimationFrame(() => { rr = 0; railState(t.closest("[data-rail]")); });
  }, true);
  w.addEventListener("resize", rails);
  rails();
})();
