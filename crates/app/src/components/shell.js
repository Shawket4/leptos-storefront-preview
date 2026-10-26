// The shell's progressive enhancement (inlined once by components::shell, ~2 KB, no wasm).
// Everything is a delegated listener, so it keeps working while the islands router patches pages.
(() => {
  const d = document, w = window, H = history;
  const reduced = () => w.matchMedia("(prefers-reduced-motion: reduce)").matches;

  // ── Keep the router's page diff aligned ──────────────────────────────────
  // The islands router walks the old and new documents in lockstep, <head> included. Pages
  // carry different head tags (title, meta, JSON-LD after leptos_meta's <!--HEAD--> marker),
  // and when their count differs the walk slips and the next page lands inside the old <head>.
  // So: apply the new page's head tags here, then hand the router HTML whose <head> matches the
  // live one, and its body diff lines up again.
  const ofetch = w.fetch;
  const headTail = (head) => {
    const out = [];
    let seen = false;
    for (const n of head.childNodes) {
      if (seen) out.push(n);
      else if (n.nodeType === 8 && n.data.trim() === "HEAD") seen = true;
    }
    return seen ? out : null;
  };
  w.fetch = async function (input) {
    const resp = await ofetch.apply(this, arguments);
    const routed = input instanceof Request && input.headers.get("Islands-Router") === "true";
    const type = resp.headers.get("content-type") || "";
    if (!routed || !resp.ok || !type.includes("text/html")) return resp;
    const html = await resp.text();
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

  // ── Scroll on router navigations ─────────────────────────────────────────
  // The islands router swaps pages without touching the scroll position. New page → top;
  // #hash → that section; back/forward → where you were. Prefs links keep their place.
  H.scrollRestoration = "manual";
  const key = () => "sy:" + location.pathname + location.search;
  const save = () => { try { sessionStorage.setItem(key(), String(Math.round(scrollY))); } catch (_) {} };
  const saved = () => { try { return +sessionStorage.getItem(key()) || 0; } catch (_) { return 0; } };
  let pending = null;
  const after = () => {
    const p = pending;
    pending = null;
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
    if (!a || a.target || a.hasAttribute("download") || /\bexternal\b/.test(a.rel)) return;
    const u = new URL(a.href, location.href);
    if (u.origin !== location.origin) return;
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

  // ── Popovers: <details data-popover> ─────────────────────────────────────
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

  // ── Rails: prev/next buttons and their end states ────────────────────────
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
