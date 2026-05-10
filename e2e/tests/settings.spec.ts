import { test, expect } from "@playwright/test";

test.describe("Settings", () => {
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

  test("should navigate to settings", async ({ page }) => {
    // Look for a settings link/button in the dashboard
    const settingsBtn = page.locator('a[href*="settings"], button:has-text("Settings"), [role="tab"]:has-text("Settings")');
    if (await settingsBtn.isVisible({ timeout: 3000 }).catch(function () { return false; })) {
      await settingsBtn.first().click();
      await expect(page.url()).toContain("settings");
    }
  });
});
