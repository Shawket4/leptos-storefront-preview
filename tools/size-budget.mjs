#!/usr/bin/env node
// The size budget (AGENTS.md, principle 6), gzip: the JS every page loads first at most 15 KB; the
// JS any one page can load in all (first load + its feature chunks + every on-demand core chunk) at
// most 30 KB; CSS under 35 KB. The whole client (every file) is printed for information: since the
// split no page loads all of it, and per-file compression makes the sum larger than one bundle.
//
//   node tools/size-budget.mjs            # after a build (cargo leptos build / watch wrote the CSS)
//
// Bundles the client in release mode (`node client/build.mjs --outdir <tmp> --release`, exactly what
// crates/app/build.rs does for a release build) and reads its manifest: the initial set (the entry
// and the chunks it imports statically, which every page fetches) and, per on-demand feature, what
// it adds on the pages that have it. Each file is measured as served — raw / gzip (level 9) /
// brotli (quality 11) — so the sums are what goes over the wire. The newest
// `target/site/pkg/*.css` is measured minified, as a release build ships it (esbuild's CSS minifier
// stands in for cargo-leptos's Lightning CSS). Exits 1 when a budget is exceeded.

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import zlib from "node:zlib";
import { createRequire } from "node:module";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const KB = 1024;
const BUDGET = { initial: 15 * KB, page: 30 * KB, css: 35 * KB }; // gzip bytes

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "size-budget-"));
const jsDir = path.join(tmp, "js");
execFileSync("node", [path.join(root, "client/build.mjs"), "--outdir", jsDir, "--release"], { stdio: "inherit" });

const measure = (buf) => ({
  raw: buf.length,
  gzip: zlib.gzipSync(buf, { level: 9 }).length,
  brotli: zlib.brotliCompressSync(buf, {
    params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 11, [zlib.constants.BROTLI_PARAM_SIZE_HINT]: buf.length },
  }).length,
});
const sum = (list) => list.reduce((a, m) => ({ raw: a.raw + m.raw, gzip: a.gzip + m.gzip, brotli: a.brotli + m.brotli }), { raw: 0, gzip: 0, brotli: 0 });

// ── The client: the manifest's module graph ───────────────────────────────────────────────────
const manifest = fs.readFileSync(path.join(jsDir, "manifest.txt"), "utf8").trim().split("\n").map((l) => l.split(" "));
const entry = manifest.find(([k]) => k === "entry")[1];
const files = manifest.filter(([k]) => k === "file").map(([, f]) => f);
const edges = manifest.filter(([k]) => k === "import").map(([, from, to, kind]) => ({ from, to, kind }));
const size = new Map(files.map((f) => [f, measure(fs.readFileSync(path.join(jsDir, f)))]));
const closure = (start) => {
  const seen = new Set([start]);
  for (const f of seen) for (const e of edges) if (e.from === f && e.kind === "static") seen.add(e.to);
  return seen;
};
const initial = closure(entry);
// On-demand chunks: every target of a dynamic import, named by the module it loads (`source`
// lines). Page features are main.ts's `lazy([...], () => import("./features/x"))` list: a page
// loads the ones its markup names. Everything else loaded on demand (the drawer panel, rails,
// toggles, pull-to-refresh, the cart drawer, instant search…) is an extra that any page may load.
const source = new Map(manifest.filter(([k]) => k === "source").map(([, f, m]) => [f, m]));
const dynamic = [...new Set(edges.filter((e) => e.kind === "dynamic").map((e) => e.to))].filter((f) => !initial.has(f));
const mainTs = fs.readFileSync(path.join(root, "client/src/main.ts"), "utf8");
const loadedBy = (fn) => new Set([...mainTs.matchAll(new RegExp(`\\b${fn}\\(\\s*\\[[^\\]]*\\]\\s*,\\s*\\(\\)\\s*=>\\s*import\\("\\.\\/(features\\/[a-z-]+)"\\)`, "g"))].map((m) => m[1] + ".ts"));
// A feature also loaded on intent (lazyOnIntent: the cart drawer, search) can load on any page.
const onIntent = loadedBy("lazyOnIntent");
const pageFeatures = new Set([...loadedBy("lazy")].filter((m) => !onIntent.has(m)));
// A page feature that another on-demand chunk loads too can come on any page that one does (the
// cart drawer opens the line editor's sheet anywhere): an extra.
const loadedElsewhere = new Set(edges.filter((e) => e.kind === "dynamic" && e.from !== entry).map((e) => e.to));
const isFeature = (f) => pageFeatures.has(source.get(f)) && !loadedElsewhere.has(f);
const label = (f) => (source.get(f) ?? f).replace(/^features\//, "").replace(/\.ts$/, "");
const lazyChunks = [...dynamic.filter(isFeature), ...dynamic.filter((f) => !isFeature(f))];

const kb = (n) => (n / KB).toFixed(1).padStart(7) + " KB";
const row = (label, m, note = "") => console.log(`${label.padEnd(44)}${kb(m.raw)}${kb(m.gzip)}${kb(m.brotli)}   ${note}`);
console.log(`${"".padEnd(44)}${"raw".padStart(10)}${"gzip".padStart(10)}${"brotli".padStart(10)}`);

let over = 0;
const pct = (n, b) => `${((n / b) * 100).toFixed(0)}% of ${(b / KB).toFixed(0)} KB`;
const init = sum([...initial].map((f) => size.get(f)));
const initOk = init.gzip <= BUDGET.initial;
if (!initOk) over++;
row(`JS initial (${initial.size} files: entry + shared)`, init, `${initOk ? "ok" : "OVER"} (${pct(init.gzip, BUDGET.initial)})`);
for (const f of [...initial].sort((a, b) => (a === entry ? -1 : b === entry ? 1 : 0))) row(`   ${f}${f === entry ? " (entry)" : ""}`, size.get(f));
lazyChunks.forEach((chunk) => {
  const extra = [...closure(chunk)].filter((f) => !initial.has(f));
  const m = sum(extra.map((f) => size.get(f)));
  row(`JS + ${isFeature(chunk) ? "" : "extra: "}${label(chunk)} (on demand)`, m, `${extra.length} file${extra.length === 1 ? "" : "s"}; page total ${(((init.gzip + m.gzip) / KB)).toFixed(1)} KB gzip`);
});
// The heaviest page, as an upper bound: the first load + one page feature's chunks + every extra
// (any page may load them), each file counted once.
const extras = new Set(lazyChunks.filter((f) => !isFeature(f)).flatMap((c) => [...closure(c)]).filter((f) => !initial.has(f)));
let worst = { name: "no feature", m: sum([...initial, ...extras].map((f) => size.get(f))) };
for (const chunk of lazyChunks.filter(isFeature)) {
  const page = new Set([...initial, ...extras, ...closure(chunk)]);
  const m = sum([...page].map((f) => size.get(f)));
  if (m.gzip > worst.m.gzip) worst = { name: label(chunk), m };
}
const pageOk = worst.m.gzip <= BUDGET.page;
if (!pageOk) over++;
row(`JS heaviest page (${worst.name}, all extras)`, worst.m, `${pageOk ? "ok" : "OVER"} (${pct(worst.m.gzip, BUDGET.page)})`);
const all = sum(files.map((f) => size.get(f)));
row(`JS whole client (${files.length} files, info)`, all, "no page loads all of it");

// ── CSS ───────────────────────────────────────────────────────────────────────────────────────
const pkg = path.join(root, "target/site/pkg");
const css = (fs.existsSync(pkg) ? fs.readdirSync(pkg) : [])
  .filter((f) => f.endsWith(".css"))
  .map((f) => path.join(pkg, f))
  .sort((a, b) => fs.statSync(b).mtimeMs - fs.statSync(a).mtimeMs)[0];
if (css) {
  const esbuild = createRequire(path.join(root, "client/package.json"))("esbuild");
  const min = Buffer.from(esbuild.transformSync(fs.readFileSync(css, "utf8"), { loader: "css", minify: true, logLevel: "error" }).code);
  const m = measure(min);
  const ok = m.gzip <= BUDGET.css;
  if (!ok) over++;
  row(`CSS ${path.basename(css)} (min)`, m, `${ok ? "ok" : "OVER"} (${pct(m.gzip, BUDGET.css)})`);
} else console.warn(`no CSS in ${path.relative(root, pkg)} — build first (cargo leptos build)`);

fs.rmSync(tmp, { recursive: true, force: true });
process.exit(over ? 1 : 0);
