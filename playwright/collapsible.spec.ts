import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("/components/collapsible", { timeout: 30 * 1000 });
  const preview = page.locator("#component-preview-frame").first();
  await page.getByRole("button", { name: "Recent Activity" }).click();
  await expect(preview.getByText("Fixed a bug in the collapsible component")).toBeVisible();
});
