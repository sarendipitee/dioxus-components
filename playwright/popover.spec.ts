import { test, expect, type Page } from "@playwright/test";
const MAIN_PAGE_URL = "/components/popover/block#main";
const SIZING_PAGE_URL = "/components/popover/block#sizing";
const PREVIEW_ROOT = "#dx-preview-block-root";

async function loadPopover(page: Page, url = MAIN_PAGE_URL) {
  await page.goto(url, { timeout: 30 * 1000, waitUntil: "load" });
  const root = page.locator(PREVIEW_ROOT);
  await expect(root).toBeVisible();
  return root;
}


test("opens basic bottom-centered content", async ({ page }) => {
  const root = await loadPopover(page);
  const trigger = root.getByTestId("popover-trigger");
  await trigger.click();
  const content = page.getByTestId("popover-content");
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("data-side", "bottom");
});

test("switches between intrinsic and extrinsic sizing", async ({ page }) => {
  const root = await loadPopover(page, SIZING_PAGE_URL);
  const trigger = root.getByTestId("sizing-popover-trigger");
  const content = page.getByTestId("sizing-popover-content");

  await trigger.click();
  await expect(content).toBeVisible();
  await expect(content).not.toHaveCSS("width", "320px");

  await page.keyboard.press("Escape");
  await root.getByTestId("extrinsic-size-option").click();
  await trigger.click();
  await expect(content).toBeVisible();
  await expect(content).toHaveCSS("width", "320px");
});
