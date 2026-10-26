#!/usr/bin/env node
// Report the client bundle size (raw / gzip -9 / brotli q11) of a cargo-leptos build.
//
//   cargo leptos build --release && node tools/wasm-size.mjs [target/site/pkg]
//
// Prints a Markdown table (used in the README) for every .wasm/.js/.css in the pkg dir.

import fs from "node:fs";
import path from "node:path";
import zlib from "node:zlib";

const dir = process.argv[2] ?? "target/site/pkg";
const files = fs
  .readdirSync(dir)
  .filter((f) => /\.(wasm|js|css)$/.test(f))
  .sort((a, b) => (a.endsWith(".wasm") ? -1 : b.endsWith(".wasm") ? 1 : a.localeCompare(b)));

const kb = (n) => (n / 1024).toFixed(1) + " KB";
const rows = [];
let total = { raw: 0, gz: 0, br: 0 };
for (const f of files) {
  const buf = fs.readFileSync(path.join(dir, f));
  const gz = zlib.gzipSync(buf, { level: 9 }).length;
  const br = zlib.brotliCompressSync(buf, {
    params: {
      [zlib.constants.BROTLI_PARAM_QUALITY]: 11,
      [zlib.constants.BROTLI_PARAM_SIZE_HINT]: buf.length,
    },
  }).length;
  rows.push(`| \`${f}\` | ${kb(buf.length)} | ${kb(gz)} | ${kb(br)} |`);
  if (!f.endsWith(".css")) {
    total.raw += buf.length;
    total.gz += gz;
    total.br += br;
  }
}
console.log("| File | Raw | gzip -9 | brotli 11 |");
console.log("|---|---:|---:|---:|");
for (const r of rows) console.log(r);
console.log(`| **JS + WASM total** | **${kb(total.raw)}** | **${kb(total.gz)}** | **${kb(total.br)}** |`);
