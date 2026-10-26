// Bundle src/main.ts for the Rust server to embed. Called by crates/app/build.rs (after `tsc
// --noEmit` has type-checked it):
//
//   node build.mjs --outdir <dir> [--release]
//
// ES modules with code splitting: the entry `app-<hash>.js`, the features main.ts loads on demand
// (`lazy(..., () => import("./features/x"))`) and the code they share as `chunk-<hash>.js` — names
// are content hashes, so every file can be cached forever. Also writes `<dir>/manifest.txt`, read
// by build.rs and tools/size-budget.mjs:
//
//   entry app-XXXX.js
//   preload chunk-AAAA.js          (a static import of the entry: fetched with it)
//   file app-XXXX.js 12345         (every output: name, bytes)
//   import app-XXXX.js chunk-AAAA.js static|dynamic
//   source chunk-BBBB.js features/product.ts   (the module a dynamic import loads; for the size report)
//
// Release: minified, no source maps. Dev: readable, inline source maps.
import { build } from "esbuild";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const args = process.argv.slice(2);
const at = args.indexOf("--outdir");
const outdir = at >= 0 ? args[at + 1] : undefined;
if (!outdir) throw new Error("usage: node build.mjs --outdir <dir> [--release]");
const release = args.includes("--release");
const entry = fileURLToPath(new URL("./src/main.ts", import.meta.url));

fs.rmSync(outdir, { recursive: true, force: true });
const result = await build({
  entryPoints: [entry],
  outdir,
  bundle: true,
  splitting: true,
  format: "esm",
  platform: "browser",
  target: ["es2020", "safari14"],
  entryNames: "app-[hash]",
  chunkNames: "chunk-[hash]",
  minify: release,
  sourcemap: release ? false : "inline",
  legalComments: "none",
  define: { __DEV__: JSON.stringify(!release) },
  metafile: true,
  logLevel: "warning",
});

const base = (p) => path.basename(p);
const lines = [];
let main = null;
for (const [file, out] of Object.entries(result.metafile.outputs)) {
  if (out.entryPoint && path.resolve(out.entryPoint) === entry) main = base(file);
  lines.push(`file ${base(file)} ${out.bytes}`);
  if (out.entryPoint) lines.push(`source ${base(file)} ${path.relative(path.dirname(entry), path.resolve(out.entryPoint))}`);
  for (const i of out.imports) {
    if (i.external) continue;
    lines.push(`import ${base(file)} ${base(i.path)} ${i.kind === "dynamic-import" ? "dynamic" : "static"}`);
  }
}
if (!main) throw new Error("build.mjs: no output for src/main.ts");
const preload = result.metafile.outputs[Object.keys(result.metafile.outputs).find((f) => base(f) === main)].imports
  .filter((i) => i.kind === "import-statement")
  .map((i) => `preload ${base(i.path)}`);
fs.writeFileSync(path.join(outdir, "manifest.txt"), [`entry ${main}`, ...preload, ...lines].join("\n") + "\n");
