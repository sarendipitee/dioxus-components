import { test, expect } from "@playwright/test";

test("popover opens and dismisses with Escape", async ({ page }) => {
  await page.goto("/components/popover");
  const popoverButton = page
    .locator("#component-preview-frame")
    .getByRole("button", { name: "Open popover" });
  await expect(popoverButton).toBeVisible();
  await popoverButton.click();

  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();
  await expect(dialog).toContainText("Details");
  await expect(dialog).toContainText("This is the popover content.");

  await page.keyboard.press("Escape");
  await expect(dialog).toHaveCount(0);
  await expect(popoverButton).toBeFocused();
});

test("popover dismisses when clicking outside", async ({ page }) => {
  await page.goto("/components/popover");
  const popoverButton = page
    .locator("#component-preview-frame")
    .getByRole("button", { name: "Open popover" });
  await popoverButton.click();
  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();
  // Click far outside the popover (corner of the document) — should dismiss.
  await page.mouse.click(2, 2);
  await expect(dialog).toHaveCount(0);
});
