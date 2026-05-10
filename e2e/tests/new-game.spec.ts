import { test, expect } from "@playwright/test";

test("complete game flow", async ({ page }) => {
  await page.addInitScript({ path: "e2e/mocks/tauri.js" });
  await page.goto("/");

  // ========== ONBOARDING ==========
  await page.locator('button:has-text("Nueva Partida")').click({ timeout: 15000 });
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

  // ========== DASHBOARD ==========
  await expect(page.locator("aside")).toBeVisible({ timeout: 10000 });
  await expect(page.locator('button[aria-label="Inicio"]')).toBeVisible();
  console.log("Dashboard loaded");

  // ========== SQUAD ==========
  await page.locator('button[aria-label="Plantilla"]').click();
  await page.waitForTimeout(2000);
  var squadText = await page.locator("body").innerText();
  console.log("Squad text sample:", squadText.substring(0, 300));
  await expect(page.locator("text=Bwipo").first()).toBeVisible({ timeout: 15000 });
  console.log("Squad: Bwipo found");

  // ========== INBOX ==========
  await page.locator('button[aria-label="Bandeja"]').click();
  await page.waitForTimeout(1000);
  console.log("Inbox navigated");

  // ========== NEWS ==========
  await page.locator('button[aria-label="Noticias"]').click();
  await page.waitForTimeout(1000);
  console.log("News navigated");

  // ========== SCHEDULE ==========
  await page.locator('button[aria-label="Calendario"]').click();
  await page.waitForTimeout(1000);
  console.log("Schedule navigated");

  // ========== TRAINING ==========
  await page.locator('button[aria-label="Entrenamiento"]').click();
  await page.waitForTimeout(1000);
  console.log("Training navigated");

  // ========== PLAYERS ==========
  await page.locator('button[aria-label="Jugadores"]').click();
  await page.waitForTimeout(1000);
  console.log("Players navigated");

  // ========== TEAMS ==========
  await page.locator('button[aria-label="Equipos"]').click();
  await page.waitForTimeout(1000);
  console.log("Teams navigated");

  // ========== SETTINGS ==========
  await page.locator('button[aria-label="Configuración"]').click();
  await page.waitForTimeout(1000);
  console.log("Settings navigated");

  console.log("All flows completed successfully");
});
