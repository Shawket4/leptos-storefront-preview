#!/usr/bin/env node
// Screenshot a storefront page across widths / themes / languages, and print console errors.
//
//   node tools/shots/shoot.mjs --url http://nile.localhost:3000/ --out shots/home \
//        [--widths 360,768,1280] [--themes light,dark] [--langs en,ar] [--num latn|arab]
//        [--js off] [--full] [--wait 1200] [--height 800]
//
// Writes <out>-<width>-<lang>-<theme>.png and prints one line per shot plus any console errors,
// page errors and failed requests. Uses the Chromium that Playwright 1.61 cached locally.

import { chromium } from "playwright";
import fs from "node:fs";
import path from "node:path";

const args = Object.fromEntries(
  process.argv.slice(2).reduce((acc, a, i, all) => {
    if (a.startsWith("--")) acc.push([a.slice(2), all[i + 1]?.startsWith("--") || all[i + 1] === undefined ? "1" : all[i + 1]]);
    return acc;
  }, []),
);
if (!args.url || !args.out) {
  console.error("usage: shoot.mjs --url URL --out PREFIX [--widths 360,768,1280] [--themes light,dark] [--langs en,ar]");
  process.exit(2);
}
const widths = (args.widths ?? "360,1280").split(",").map(Number);
const themes = (args.themes ?? "light").split(",");
const langs = (args.langs ?? "en").split(",");
const wait = Number(args.wait ?? 1200);
const height = Number(args.height ?? 800);
const url = new URL(args.url);
fs.mkdirSync(path.dirname(path.resolve(args.out)), { recursive: true });

const browser = await chromium.launch();
let failures = 0;
for (const width of widths) {
  for (const lang of langs) {
    for (const theme of themes) {
      const ctx = await browser.newContext({
        viewport: { width, height },
        deviceScaleFactor: width <= 480 ? 2 : 1,
        javaScriptEnabled: args.js !== "off",
        colorScheme: theme === "dark" ? "dark" : "light",
        hasTouch: width <= 480,
        isMobile: width <= 480,
      });
      await ctx.addCookies(
        [
          ["lang", lang],
          ["theme", theme],
          ["num", args.num ?? "latn"],
        ].map(([name, value]) => ({ name, value, domain: url.hostname, path: "/" })),
      );
      const page = await ctx.newPage();
      const problems = [];
      page.on("console", (m) => m.type() === "error" && problems.push(`console: ${m.text()}`));
      page.on("pageerror", (e) => problems.push(`pageerror: ${e.message}`));
      page.on("requestfailed", (r) => problems.push(`requestfailed: ${r.url()} ${r.failure()?.errorText}`));
      page.on("response", (r) => r.status() >= 400 && problems.push(`http ${r.status()}: ${r.url()}`));
      const t0 = Date.now();
      const resp = await page.goto(args.url, { waitUntil: "networkidle" }).catch((e) => (problems.push(`goto: ${e.message}`), null));
      await page.waitForTimeout(wait);
      const file = `${args.out}-${width}-${lang}-${theme}.png`;
      await page.screenshot({ path: file, fullPage: args.full === "1" || args.full === "true" });
      const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
      console.log(`${file}  status=${resp?.status() ?? "-"}  ${Date.now() - t0}ms${overflow > 0 ? `  HORIZONTAL-OVERFLOW=${overflow}px` : ""}`);
      for (const p of problems) console.log("   ! " + p);
      failures += problems.length + (overflow > 0 ? 1 : 0);
      await ctx.close();
    }
  }
}
await browser.close();
process.exit(failures ? 1 : 0);
