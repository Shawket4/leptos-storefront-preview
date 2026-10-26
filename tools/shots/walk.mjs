#!/usr/bin/env node
// End-to-end walk of the storefront under phone conditions, logging every request the page makes.
// It checks the architecture's promise: state changes (add to cart, variant, quantity) send no
// request; only data needs do (navigations, the cart quote, search suggestions, order status).
//
//   node tools/shots/walk.mjs --base http://souq.localhost:3000 [--out <dir>] [--fast]
//
// --fast skips the network throttling (200 ms latency, 3 Mbit/s down, 1 Mbit/s up).

import { chromium } from "playwright";
import fs from "node:fs";
import path from "node:path";

const args = Object.fromEntries(
  process.argv.slice(2).reduce((acc, a, i, all) => {
    if (a.startsWith("--")) acc.push([a.slice(2), all[i + 1]?.startsWith("--") || all[i + 1] === undefined ? "1" : all[i + 1]]);
    return acc;
  }, []),
);
const base = args.base ?? "http://souq.localhost:3000";
const out = args.out ?? "/tmp/walk";
fs.mkdirSync(out, { recursive: true });

const browser = await chromium.launch();
const ctx = await browser.newContext({ viewport: { width: 390, height: 844 }, deviceScaleFactor: 2, isMobile: true, hasTouch: true });
const page = await ctx.newPage();
if (!args.fast) {
  const cdp = await ctx.newCDPSession(page);
  await cdp.send("Network.enable");
  await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: 200, downloadThroughput: 3e6 / 8, uploadThroughput: 1e6 / 8 });
}

const problems = [];
const requests = [];
page.on("console", (m) => m.type() === "error" && problems.push(`console: ${m.text().slice(0, 200)}`));
page.on("pageerror", (e) => problems.push(`pageerror: ${String(e).slice(0, 200)}`));
page.on("requestfailed", (r) => problems.push(`failed: ${r.method()} ${r.url().replace(base, "")} ${r.failure()?.errorText}`));
page.on("request", (r) => {
  const u = r.url();
  if (!u.startsWith(base)) return;
  const p = u.replace(base, "");
  if (/\.(webp|woff2|css|js|png|svg|ico)(\?|$)/.test(p) || p.startsWith("/media/") || p.startsWith("/fonts/")) return;
  requests.push(`${r.method()} ${p}`);
});

let step = 0;
const results = [];
async function check(name, fn, { expectRequests } = {}) {
  step++;
  const before = requests.length;
  let ok = true, note = "";
  try {
    note = (await fn()) ?? "";
  } catch (e) {
    ok = false;
    note = String(e).split("\n")[0].slice(0, 160);
  }
  await page.waitForTimeout(400);
  const made = requests.slice(before);
  if (expectRequests !== undefined && !expectRequests(made)) {
    ok = false;
    note += ` | unexpected requests: ${JSON.stringify(made)}`;
  }
  const shot = path.join(out, `${String(step).padStart(2, "0")}-${name.replace(/\W+/g, "-")}.png`);
  await page.screenshot({ path: shot }).catch(() => {});
  results.push({ step, name, ok, requests: made.length, note });
  console.log(`${ok ? "✓" : "✗"} ${String(step).padStart(2)} ${name.padEnd(34)} req=${made.length} ${note}`);
}

const badge = () => page.evaluate(() => document.querySelector('[data-ts~="cart-badge"]')?.textContent?.trim() ?? "?");
const toastText = () => page.evaluate(() => [...document.querySelectorAll("#toaster [data-toast-id]")].map((t) => t.textContent.trim()).join(" / "));
const none = (r) => r.length === 0;
const only = (re) => (r) => r.length > 0 && r.every((x) => re.test(x));

await check("home loads", async () => {
  await page.goto(base + "/", { waitUntil: "domcontentloaded" });
  await page.waitForLoadState("networkidle");
  return `badge=${await badge()}`;
});
await check("quick add (no request)", async () => {
  await page.locator('[data-ts~="quick-add"]').first().tap();
  await page.waitForTimeout(500);
  return `badge=${await badge()} toast="${await toastText()}"`;
}, { expectRequests: none });
await check("open a category (router)", async () => {
  await page.locator("main a[href^='/c/']").first().tap();
  await page.waitForURL(/\/c\//);
  return page.url().replace(base, "");
}, { expectRequests: only(/^GET \/c\//) });
await check("filter (one navigation)", async () => {
  const f = page.locator('[data-drawer-open], [data-ts~="filters"] input[type=checkbox]').first();
  await f.tap();
  await page.waitForTimeout(300);
  const box = page.locator('[data-ts~="drawer"] input[type=checkbox], [data-ts~="filters"] input[type=checkbox]').first();
  if (await box.count()) await box.tap();
  await page.waitForTimeout(1200);
  return page.url().replace(base, "");
});
await check("open a product (router)", async () => {
  await page.locator("main a[href^='/p/']").first().tap();
  await page.waitForURL(/\/p\//);
  return page.url().replace(base, "");
}, { expectRequests: only(/^GET \/p\//) });
await check("switch variant (no request)", async () => {
  const opt = page.locator('[data-ts~="variant-picker"] a, [data-ts~="variant-picker"] input, [data-ts~="variant-picker"] button').nth(1);
  if (!(await opt.count())) return "single-variant product";
  await opt.tap();
  return page.url().replace(base, "");
}, { expectRequests: none });
await check("add to cart from product (no request)", async () => {
  await page.locator('main button[type=submit], main [data-ts~="add-to-cart"]').first().tap();
  await page.waitForTimeout(500);
  return `badge=${await badge()} toast="${await toastText()}"`;
}, { expectRequests: none });
await check("open drawer (one quote)", async () => {
  await page.locator('[data-ts~="cart-badge"]').first().tap();
  await page.waitForTimeout(1500);
  return page.evaluate(() => document.querySelector("#cart-drawer")?.textContent?.replace(/\s+/g, " ").slice(0, 120));
}, { expectRequests: only(/^GET \/api\/cart\/quote/) });
await check("+1 in drawer (quote after debounce)", async () => {
  await page.locator('#cart-drawer [data-ts~="stepper"] button').last().tap();
  await page.waitForTimeout(1200);
  return `badge=${await badge()}`;
}, { expectRequests: (r) => r.every((x) => /^GET \/api\/cart\/quote/.test(x)) && r.length <= 1 });
await check("back (from cache)", async () => {
  await page.keyboard.press("Escape");
  await page.goBack();
  await page.waitForTimeout(800);
  return page.url().replace(base, "");
}, { expectRequests: (r) => r.length <= 1 });
await check("/cart page", async () => {
  await page.goto(base + "/cart", { waitUntil: "networkidle" });
  return page.evaluate(() => document.querySelector("main")?.textContent?.replace(/\s+/g, " ").slice(0, 140));
});
await check("search suggestions", async () => {
  const box = page.locator('[data-ts~="search-box"] input').first();
  await box.tap();
  await box.fill(base.includes("souq") ? "رز" : "phone");
  await page.waitForTimeout(1500);
  return page.evaluate(() => [...document.querySelectorAll('[data-ts~="search-box"] [role=option]')].slice(0, 3).map((o) => o.textContent.trim()).join(" | "));
}, { expectRequests: (r) => r.every((x) => /^GET \/api\/search\/suggest/.test(x)) && r.length >= 1 });

console.log(`\n${results.filter((r) => r.ok).length}/${results.length} steps ok; problems: ${problems.length ? "\n  " + problems.join("\n  ") : "none"}`);
await browser.close();
process.exit(results.every((r) => r.ok) && problems.length === 0 ? 0 : 1);
