import { test, expect } from "@playwright/test";

test.describe("Squad", () => {
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

  test("should navigate to squad and show roster", async ({ page }) => {
    // Click the Squad tab
    const squadTab = page.locator('a:has-text("Squad"), button:has-text("Squad"), [role="tab"]:has-text("Squad")');
    await squadTab.first().click();

    // Verify we see team players
    await expect(page.locator("text=Bwipo").first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator("text=Razork").first()).toBeVisible({ timeout: 5000 });
    await expect(page.locator("text=Humanoid").first()).toBeVisible({ timeout: 5000 });
  });
});
