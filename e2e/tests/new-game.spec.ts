import { test, expect } from "@playwright/test";

test.describe("New Game Flow", () => {
  test.beforeEach(async ({ page }) => {
    // Inject Tauri mock before any page load
    await page.addInitScript({ path: "e2e/mocks/tauri.js" });
    await page.goto("/");
  });

  test("should display the main menu with new game button", async ({ page }) => {
    // Main menu should show the "Nueva Partida" button (menuState === "main")
    const newGameBtn = page.locator('button:has-text("Nueva Partida")');
    await expect(newGameBtn).toBeVisible({ timeout: 15000 });

    // Click to open the create manager form
    await newGameBtn.click();

    // Form fields should now be visible
    await expect(page.locator("#create-manager-field-firstName")).toBeVisible({ timeout: 5000 });
    await expect(page.locator("#create-manager-field-lastName")).toBeVisible();
  });

  test("should create a new game and navigate to team selection", async ({ page }) => {
    // Click "Nueva Partida" to open the create manager form
    await page.locator('button:has-text("Nueva Partida")').click({ timeout: 15000 });

    // Wait for form to appear
    await expect(page.locator("#create-manager-field-firstName")).toBeVisible({ timeout: 5000 });

    // Fill manager form
    await page.locator("#create-manager-field-firstName input").fill("John");
    await page.locator("#create-manager-field-lastName input").fill("Doe");
    await page.locator("#create-manager-field-nickname input").fill("JD");

    // Fill date of birth (custom DatePicker: 3 separate inputs)
    const dobFields = page.locator("#create-manager-field-dob input[inputmode='numeric']");
    // Day
    await dobFields.nth(0).fill("15");
    // Year
    await dobFields.nth(1).fill("2000");

    // Month — click the dropdown button inside the datepicker
    await page.locator("#create-manager-field-dob button[type='button']").first().click();
    // Select first month from the dropdown list (any language)
    await page.locator("#create-manager-field-dob [class*='max-h-48'] button").first().click();

    // Select nationality from the searchable dropdown
    // Click the dropdown trigger button
    await page.locator("#create-manager-field-nationality button[type='button']").click();
    // Type search text
    await page.locator("#create-manager-field-nationality input[type='text']").fill("Spain");
    // Click the Spain option
    await page.locator("#create-manager-field-nationality button:has-text('Spain')").click();

    // Click "Comenzar" / "Start Career"
    await page.locator('button:has-text("Comenzar")').click();

    // Should navigate to /select-team
    await page.waitForURL("**/select-team", { timeout: 30000 });

    // Verify team selection shows teams
    await expect(page.locator("text=Fnatic").first()).toBeVisible({ timeout: 15000 });

    // Select Fnatic
    await page.locator('button:has-text("Fnatic")').first().click();

    // Verify confirm button is enabled
    const confirmBtn = page.locator('button:has-text("Confirmar")');
    await expect(confirmBtn).toBeEnabled();

    // Confirm selection
    await confirmBtn.click();

    // Should navigate to dashboard
    await page.waitForURL("**/dashboard", { timeout: 30000 });
    await expect(page.locator("text=Dashboard").first()).toBeVisible({ timeout: 15000 });
  });
});
