import { chromium } from "@playwright/test";

const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
const page = await context.newPage();
await page.addInitScript({ path: "e2e/mocks/tauri.js" });
await page.goto("http://localhost:1420");

await page.locator('button:has-text("Nueva Partida")').click({ timeout: 15000 });
await page.locator("#create-manager-field-firstName").waitFor({ timeout: 5000 });
await page.locator("#create-manager-field-firstName input").fill("John");
await page.locator("#create-manager-field-lastName input").fill("Doe");
await page.locator("#create-manager-field-nickname input").fill("JD");
await page.locator("input[inputmode='numeric']").nth(0).fill("15");
await page.locator("input[inputmode='numeric']").nth(1).fill("2000");
await page.locator("#create-manager-field-dob button").first().click();
await page.locator(".max-h-48 button").first().click();
await page.locator("#create-manager-field-nationality button[type='button']").first().click();
await page.locator("#create-manager-field-nationality input[type='text']").fill("ES");
await page.locator("#create-manager-field-nationality [class*='max-h'] button").first().waitFor({ timeout: 5000 });
await page.locator("#create-manager-field-nationality [class*='max-h'] button").first().click({ force: true });
await page.locator('button[type="submit"]').click();
await page.waitForURL("**/select-team", { timeout: 30000 });
await page.locator("text=Fnatic").first().click();
await page.locator('button:has-text("Dirigir")').click();
await page.waitForURL("**/dashboard", { timeout: 30000 });
await page.waitForTimeout(3000);

await page.screenshot({ path: "e2e/tests/dashboard-debug.png", fullPage: true });
console.log("Screenshot saved");

const labels = await page.locator("button[aria-label]").evaluateAll(
  (els) => els.map((el) => el.getAttribute("aria-label"))
);
console.log("ARIA labels:", JSON.stringify(labels));

const allText = await page.locator("body").innerText();
console.log("Page text sample:", allText.substring(0, 500));

await browser.close();
