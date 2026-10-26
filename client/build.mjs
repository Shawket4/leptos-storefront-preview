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
//
// One chunk for the first load. esbuild groups modules into chunks by which entry points (the
// entry and each dynamic import) reach them, so the code every page loads at first came out as ~11
// small files, each compressed on its own: 17.8 KB gzip where the same code in one file is 14.8.
// esbuild has no manual chunks, so the build runs twice: the first pass finds the first-load
// modules (the entry's static imports, transitively); the second makes every on-demand feature
// reach all of them too (side-effect imports, added on its first line so source maps keep their
// lines), which puts them in one shared chunk. Nothing more is downloaded or run: those modules
// load and evaluate at boot anyway, and importing an evaluated module does nothing.
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

const options = {
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
};

// Pass 1 (not written): the first-load modules, and the modules dynamic imports load.
const probe = (await build({ ...options, write: false })).metafile;
const abs = (p) => path.resolve(p);
const entryOut = Object.keys(probe.outputs).find((f) => probe.outputs[f].entryPoint && abs(probe.outputs[f].entryPoint) === entry);
const firstLoad = new Set([entryOut]);
for (const f of firstLoad) for (const i of probe.outputs[f].imports) if (i.kind === "import-statement" && !i.external) firstLoad.add(i.path);
const firstLoadModules = [...firstLoad].flatMap((f) => Object.keys(probe.outputs[f].inputs)).map(abs).filter((m) => m !== entry);
const onDemand = new Set(Object.values(probe.outputs).map((o) => o.entryPoint && abs(o.entryPoint)).filter((m) => m && m !== entry));

// Pass 2: every on-demand feature also reaches the first-load modules → one shared chunk.
const firstLoadChunk = {
  name: "first-load-chunk",
  setup(b) {
    b.onLoad({ filter: /\.tsx?$/ }, (args) => {
      if (!onDemand.has(abs(args.path))) return undefined;
      const reach = firstLoadModules.map((m) => `import ${JSON.stringify(m)};`).join("");
      return { contents: reach + fs.readFileSync(args.path, "utf8"), loader: "ts", resolveDir: path.dirname(args.path) };
    });
  },
};
fs.rmSync(outdir, { recursive: true, force: true });
const result = await build({ ...options, plugins: [firstLoadChunk] });

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
