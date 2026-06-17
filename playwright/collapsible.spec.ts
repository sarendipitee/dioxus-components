import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("/components/collapsible", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  const preview = page.locator("#component-preview-frame").first();
  await page.getByRole("button", { name: "Recent Activity" }).click();
  await expect(preview.getByText("Fixed a bug in the collapsible component")).toBeVisible();
});
