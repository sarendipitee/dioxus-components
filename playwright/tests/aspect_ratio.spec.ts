import { expect, test, type Page } from "@playwright/test";

const ASPECT_RATIO_URL = "/components/aspect_ratio/block#main";

async function gotoAspectRatioDemo(page: Page) {
  await page.goto(ASPECT_RATIO_URL, {
    timeout: 30 * 1000,
    waitUntil: "load",
  });
}

test("renders the 4:3 landscape preview", async ({ page }) => {
  await gotoAspectRatioDemo(page);

  const aspectRatio = page.getByTestId("aspect-ratio");
  const content = page.getByTestId("aspect-ratio-content");

  await expect(aspectRatio).toBeVisible();
  await expect(content).toBeVisible();
  await expect(content).toHaveText("4:3 landscape preview");

  const box = await aspectRatio.boundingBox();
  expect(box).not.toBeNull();
  expect(box!.height).toBeGreaterThan(0);
  expect(box!.width / box!.height).toBeCloseTo(4 / 3, 1);
});

test("forwards preview attributes and accessible label", async ({ page }) => {
  await gotoAspectRatioDemo(page);

  const aspectRatio = page.getByTestId("aspect-ratio");

  await expect(aspectRatio).toHaveAttribute("id", "aspect-ratio-demo");
  await expect(aspectRatio).toHaveAttribute("data-ratio", "4:3");
  await expect(aspectRatio).toHaveAttribute(
    "aria-label",
    "4 by 3 landscape preview",
  );
  await expect(page.getByLabel("4 by 3 landscape preview")).toBeVisible();
});
