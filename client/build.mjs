// Bundle src/main.ts into one ES module for the Rust server to embed. Called by crates/app/build.rs
// (after `tsc --noEmit` has type-checked it):
//
//   node build.mjs --out <file> [--release]
//
// Release: minified, no source map. Dev: readable, with an inline source map.
import { build } from "esbuild";
import { fileURLToPath } from "node:url";

const args = process.argv.slice(2);
const out = args[args.indexOf("--out") + 1];
if (!out) throw new Error("usage: node build.mjs --out <file> [--release]");
const release = args.includes("--release");

await build({
  entryPoints: [fileURLToPath(new URL("./src/main.ts", import.meta.url))],
  outfile: out,
  bundle: true,
  format: "esm",
  platform: "browser",
  target: ["es2020", "safari14"],
  minify: release,
  sourcemap: release ? false : "inline",
  legalComments: "none",
  define: { __DEV__: JSON.stringify(!release) },
  logLevel: "warning",
});
