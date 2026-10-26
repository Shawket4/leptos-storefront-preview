#!/usr/bin/env node
// The size budget (AGENTS.md, principle 6): the whole client under 30 KB gzip, CSS under 35 KB gzip.
//
//   node tools/size-budget.mjs            # after a build (cargo leptos build / watch wrote the CSS)
//
// Bundles the client in release mode (`node client/build.mjs --out <tmp> --release`, exactly what
// crates/app/build.rs does for a release build) and measures it, and the newest
// `target/site/pkg/*.css` minified the way a release build ships it (a dev build writes it
// unminified; esbuild's CSS minifier stands in for cargo-leptos's Lightning CSS): raw / gzip
// (level 9) / brotli (quality 11). Prints a table; exits 1 if a budget is exceeded (2 if there is
// nothing to measure).

import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import zlib from "node:zlib";
import { createRequire } from "node:module";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const BUDGET = { js: 30 * 1024, css: 35 * 1024 }; // gzip bytes

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "size-budget-"));
const jsOut = path.join(tmp, "app.js");
execFileSync("node", [path.join(root, "client/build.mjs"), "--out", jsOut, "--release"], { stdio: "inherit" });

// The newest stylesheet (hash-files leaves one per content hash).
const pkg = path.join(root, "target/site/pkg");
const css = (fs.existsSync(pkg) ? fs.readdirSync(pkg) : [])
  .filter((f) => f.endsWith(".css"))
  .map((f) => path.join(pkg, f))
  .sort((a, b) => fs.statSync(b).mtimeMs - fs.statSync(a).mtimeMs)
  .slice(0, 1);
if (!css.length) console.warn(`no CSS in ${path.relative(root, pkg)} — build first (cargo leptos build)`);
const esbuild = createRequire(path.join(root, "client/package.json"))("esbuild");
const cssMin = css.map((f) => {
  const out = path.join(tmp, path.basename(f));
  fs.writeFileSync(out, esbuild.transformSync(fs.readFileSync(f, "utf8"), { loader: "css", minify: true, logLevel: "error" }).code);
  return { src: f, out };
});

const measure = (file) => {
  const buf = fs.readFileSync(file);
  return {
    raw: buf.length,
    gzip: zlib.gzipSync(buf, { level: 9 }).length,
    brotli: zlib.brotliCompressSync(buf, {
      params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 11, [zlib.constants.BROTLI_PARAM_SIZE_HINT]: buf.length },
    }).length,
  };
};
const kb = (n) => (n / 1024).toFixed(1).padStart(7) + " KB";

const rows = [
  { kind: "js", name: "client (release bundle)", ...measure(jsOut) },
  ...cssMin.map(({ src, out }) => ({ kind: "css", name: path.basename(src) + " (min)", ...measure(out) })),
];
fs.rmSync(tmp, { recursive: true, force: true });

let over = 0;
console.log(`${"".padEnd(48)}${"raw".padStart(10)}${"gzip".padStart(10)}${"brotli".padStart(10)}   budget (gzip)`);
for (const r of rows) {
  const budget = BUDGET[r.kind];
  const ok = r.gzip <= budget;
  if (!ok) over++;
  console.log(
    `${(r.kind.toUpperCase() + "  " + r.name).padEnd(48)}${kb(r.raw)}${kb(r.gzip)}${kb(r.brotli)}   ${ok ? "ok" : "OVER"} (${(budget / 1024).toFixed(0)} KB, ${((r.gzip / budget) * 100).toFixed(0)}%)`,
  );
}
if (!rows.length) process.exit(2);
process.exit(over ? 1 : 0);
