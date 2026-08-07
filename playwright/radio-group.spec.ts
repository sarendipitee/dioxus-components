import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("/components/radio_group", { timeout: 30 * 1000 });
  await page.getByRole('radio', { name: 'Blue' }).click();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('radio', { name: 'Red' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('radio', { name: 'Blue' })).toBeFocused();
});
