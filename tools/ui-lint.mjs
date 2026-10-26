#!/usr/bin/env node
// UI lint: the design system is used, not re-typed, and the Apple design rules hold.
//
//   node tools/ui-lint.mjs [repo-root]          exit 1 when anything is found
//
// 1. Pages never hand-write a shared component's markup: a page's Rust may not use a class that a
//    component stylesheet (style/components/*.css) owns at top level (`btn`, `badge`, `price`, …).
//    Use the component; pass data-*/aria-*/name/value with `attr:` (`<Button attr:data-x="">`).
//    A component that can't do what you need gets extended by its owner (core) — never re-typed.
// 2. Stylesheets follow the design rules (AGENTS.md): no gradients; colours only through tokens
//    (no hex/rgb/hsl outside tokens.css); corner radii only through `--r-*` (0 and 50% allowed);
//    easing only through the `--ease-*` / `--spring-*` tokens.
// 3. Client code: stores only in stores/ (and core/); no HTML strings with data (`innerHTML =`);
//    no money formatting (`Intl.NumberFormat`, `toLocaleString`) — money comes formatted.

import fs from "node:fs";
import path from "node:path";

const root = path.resolve(process.argv[2] ?? ".");
const rel = (p) => path.relative(root, p);
const walk = (dir, ext) =>
  fs.existsSync(dir)
    ? fs.readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
        const p = path.join(dir, e.name);
        return e.isDirectory() ? walk(p, ext) : ext.some((x) => p.endsWith(x)) ? [p] : [];
      })
    : [];
const problems = [];
const report = (file, line, rule, text) => problems.push({ file: rel(file), line, rule, text: text.trim().slice(0, 120) });
const lines = (file) => fs.readFileSync(file, "utf8").split("\n");

// ── 1. Component classes re-typed in pages ───────────────────────────────────────────────────
const owned = new Map(); // class → component stylesheet
for (const css of walk(path.join(root, "style/components"), [".css"])) {
  for (const l of lines(css)) {
    const m = /^\.([a-z][a-z0-9-]*)(?![a-z0-9_-])/.exec(l);
    if (m && !m[1].includes("__")) owned.set(m[1], path.basename(css, ".css"));
  }
}
for (const file of walk(path.join(root, "crates/app/src/pages"), [".rs"])) {
  lines(file).forEach((l, i) => {
    if (/^\s*\/\//.test(l)) return;
    for (const m of l.matchAll(/class(?:=|:)\s*"([^"]*)"/g)) {
      for (const c of m[1].split(/\s+/)) {
        const base = c.split("--")[0];
        if (owned.has(base))
          report(file, i + 1, `re-typed component markup (.${c} belongs to components/${owned.get(base)})`, l);
      }
    }
  });
}

// ── 2. Design rules in stylesheets ─────────────────────────────────────────────────────────────
const cssFiles = [...walk(path.join(root, "style/components"), [".css"]), ...walk(path.join(root, "style/pages"), [".css"])];
for (const file of cssFiles) {
  lines(file).forEach((l, i) => {
    const code = l.replace(/\/\*.*?\*\//g, "");
    if (/^\s*\/?\*/.test(l) && !/[;{}]/.test(code)) return; // comment lines
    if (/gradient\(/.test(code)) report(file, i + 1, "gradient (no gradients — AGENTS.md design rules)", l);
    if (/#[0-9a-fA-F]{3,8}\b|\b(rgba?|hsla?|oklch|oklab)\(/.test(code) && !/var\(--/.test(code.split(":").slice(1).join(":") || "x"))
      report(file, i + 1, "raw colour (use a token from tokens.css)", l);
    const r = /border(?:-[a-z]+)*-radius\s*:\s*([^;]+)/.exec(code);
    if (r && !/var\(--r-|^\s*(0|50%|inherit)\s*$/.test(r[1])) report(file, i + 1, "raw radius (use --r-* tokens; squircle scale)", l);
    if (/cubic-bezier\(|\b(ease-in-out|ease-in|ease-out|linear)\b|[:\s]ease[;\s,]/.test(code) && !/var\(--(ease|spring)/.test(code) && /transition|animation/.test(code))
      report(file, i + 1, "raw easing (use --ease-* / --spring-* tokens)", l);
  });
}

// ── 3. Client code rules ─────────────────────────────────────────────────────────────────────
for (const file of walk(path.join(root, "client/src"), [".ts"])) {
  const area = rel(file);
  lines(file).forEach((l, i) => {
    if (/^\s*\/\//.test(l)) return;
    if (/\bcreateStore\(/.test(l) && !/client\/src\/(stores|core)\//.test(area) && !/import/.test(l))
      report(file, i + 1, "store created outside stores/ (one store per concern, AGENTS.md)", l);
    if (/\.innerHTML\s*=(?!=)/.test(l)) report(file, i + 1, "innerHTML assignment (fill a server <template> instead)", l);
    if (/Intl\.NumberFormat|toLocaleString\(/.test(l)) report(file, i + 1, "client-side number/money formatting (money comes formatted; counts use digits())", l);
  });
}

const byRule = Object.groupBy ? Object.groupBy(problems, (p) => p.rule) : problems.reduce((m, p) => ((m[p.rule] ??= []).push(p), m), {});
for (const [rule, list] of Object.entries(byRule)) {
  console.log(`\n${rule} — ${list.length}`);
  for (const p of list.slice(0, 40)) console.log(`  ${p.file}:${p.line}  ${p.text}`);
  if (list.length > 40) console.log(`  … ${list.length - 40} more`);
}
console.log(problems.length ? `\n✗ ${problems.length} problems` : "✓ ui-lint clean");
process.exit(problems.length ? 1 : 0);
