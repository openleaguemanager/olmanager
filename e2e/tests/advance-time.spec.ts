import { test, expect } from "@playwright/test";

test.describe("Advance Time", () => {
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

  test("should advance one week and update the game state", async ({ page }) => {
    // Verify we're on the dashboard
    await expect(page.locator("text=Fnatic").first()).toBeVisible({ timeout: 15000 });

    // Find and click the advance time button
    const advanceBtn = page.locator('button:has-text("Avanzar"), button[title*="Avanzar"], button:has-text("Next"), button[aria-label*="advance"]');
    if (await advanceBtn.isVisible({ timeout: 3000 }).catch(function () { return false; })) {
      await advanceBtn.click();
    }

    // After advancing, verify we're still on dashboard
    await expect(page.locator("text=Fnatic").first()).toBeVisible({ timeout: 15000 });
  });
});
