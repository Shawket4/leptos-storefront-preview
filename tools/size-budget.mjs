#!/usr/bin/env node
// The size budget (AGENTS.md, principle 6): JS a store page loads at first at most 15 KB gzip, the
// whole client under 30 KB gzip, CSS under 35 KB gzip.
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
const BUDGET = { initial: 15 * KB, js: 30 * KB, css: 35 * KB }; // gzip bytes

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
// On-demand chunks: main.ts's lazy features (dynamic imports of the entry, in its lazy() order),
// then any other dynamic import of the initial set (core's: the drawer panel, rails, toggles, PTR).
const fromEntry = edges.filter((e) => e.from === entry && e.kind === "dynamic").map((e) => e.to);
const fromShared = [...new Set(edges.filter((e) => e.from !== entry && initial.has(e.from) && e.kind === "dynamic").map((e) => e.to))].filter((f) => !fromEntry.includes(f));
const lazyChunks = [...fromEntry, ...fromShared];
// Name each on-demand chunk by the feature module it holds (dev build's paths aren't in release:
// main.ts's lazy() list order matches the entry's dynamic imports).
const mainTs = fs.readFileSync(path.join(root, "client/src/main.ts"), "utf8");
const lazyNames = [...mainTs.matchAll(/lazy\(\s*\[([^\]]*)\]\s*,\s*\(\)\s*=>\s*import\("\.\/features\/([a-z-]+)"\)/g)].map((m) => m[2]);

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
lazyChunks.forEach((chunk, i) => {
  const extra = [...closure(chunk)].filter((f) => !initial.has(f));
  const m = sum(extra.map((f) => size.get(f)));
  row(`JS + ${lazyNames[i] ?? "core: " + chunk} (on demand)`, m, `${extra.length} file${extra.length === 1 ? "" : "s"}; page total ${(((init.gzip + m.gzip) / KB)).toFixed(1)} KB gzip`);
});
const all = sum(files.map((f) => size.get(f)));
const allOk = all.gzip <= BUDGET.js;
if (!allOk) over++;
row(`JS whole client (${files.length} files)`, all, `${allOk ? "ok" : "OVER"} (${pct(all.gzip, BUDGET.js)})`);

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
