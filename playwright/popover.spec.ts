import { test, expect, type Page } from "@playwright/test";
const PAGE_URL = "/components/popover/block#main";
const PREVIEW_ROOT = "#dx-preview-block-root";

async function loadPopover(page: Page) {
  await page.goto(PAGE_URL, { timeout: 30 * 1000, waitUntil: "load" });
  const root = page.locator(PREVIEW_ROOT);
  await expect(root).toBeVisible();
  return root;
}


test("defaults to bottom-centered intrinsic content", async ({ page }) => {
  const root = await loadPopover(page);
  const trigger = root.getByTestId("popover-trigger");
  await trigger.click();
  const content = page.getByTestId("popover-content");
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("data-side", "bottom");
});

test("accepts an explicit CSS width", async ({ page }) => {
  const root = await loadPopover(page);
  const trigger = root.getByTestId("explicit-width-trigger");
  await trigger.click();
  const content = page.getByTestId("explicit-width-content");
  await expect(content).toBeVisible();
  await expect(content).toHaveCSS("width", "320px");
});
