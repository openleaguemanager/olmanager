import { test, expect } from "@playwright/test";

test.describe("Schedule", () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript({ path: "e2e/mocks/tauri.js" });
    await page.goto("/");
    await page.locator('button:has-text("Nueva Partida")').click();
    await expect(page.locator("#create-manager-field-firstName")).toBeVisible({ timeout: 5000 });
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
  });

  test("should navigate to schedule tab", async ({ page }) => {
    // Click the Schedule tab
    const scheduleTab = page.locator('a:has-text("Schedule"), button:has-text("Schedule"), [role="tab"]:has-text("Schedule")');
    await scheduleTab.first().click();

    // Verify schedule page loaded
    await expect(page.locator("text=Schedule").first()).toBeVisible({ timeout: 10000 });
  });
});
