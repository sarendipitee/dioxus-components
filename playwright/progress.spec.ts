import { expect, test, type Page } from "@playwright/test";

const PROGRESS_URL = "/components/progress";

async function gotoProgressDemo(page: Page) {
  await page.goto(PROGRESS_URL, { timeout: 30_000, waitUntil: "load" });
}

function progressbar(page: Page) {
  return page.getByRole("progressbar", { name: "Progressbar Demo", exact: true }).first();
}

test("exposes determinate semantics, text, and global attributes", async ({ page }) => {
  await gotoProgressDemo(page);
  const progress = progressbar(page);
  await expect(progress).toHaveAttribute("id", "progress-fixture");
  await expect(progress).toHaveAttribute("data-progress-fixture", "controlled");
  await expect(progress).toHaveAttribute("class", /progress-fixture/);
  await expect(progress).toHaveAttribute("title", "Controlled progress fixture");
  await expect(progress).toHaveAttribute("aria-valuemin", "0");
  await expect(progress).toHaveAttribute("aria-valuemax", "100");
  await expect(progress).toHaveAttribute("aria-valuenow", "25");
  await expect(progress).toHaveAttribute("aria-valuetext", "Progress fixture value");
  await expect(progress).toHaveAttribute("data-state", "loading");
  await expect(progress).toHaveAttribute("data-value", "25");
  await expect(progress).toHaveAttribute("data-max", "100");
  await expect(progress).toHaveAttribute("style", /--progress-value:\s*25%/);
  await expect(progress).toContainText("Progress fixture content");
});

test("reactively updates value and max", async ({ page }) => {
  await gotoProgressDemo(page);
  const progress = progressbar(page);
  await page.getByRole("button", { name: "Set 75", exact: true }).first().click();
  await expect(progress).toHaveAttribute("aria-valuenow", "75");
  await expect(progress).toHaveAttribute("style", /--progress-value:\s*75%/);
  await page.getByRole("button", { name: "Restore custom max/value", exact: true }).first().click();
  await expect(progress).toHaveAttribute("aria-valuemax", "200");
  await expect(progress).toHaveAttribute("aria-valuenow", "50");
  await expect(progress).toHaveAttribute("data-max", "200");
  await expect(progress).toHaveAttribute("style", /--progress-value:\s*25%/);
});

test("switches to indeterminate semantics", async ({ page }) => {
  await gotoProgressDemo(page);
  const progress = progressbar(page);
  await page.getByRole("button", { name: "Set indeterminate", exact: true }).first().click();
  await expect(progress).toHaveAttribute("data-state", "indeterminate");
  await expect(progress).not.toHaveAttribute("aria-valuenow", /.+/);
  await expect(progress).not.toHaveAttribute("data-value", /.+/);
  await expect(progress).not.toHaveAttribute("style", /--progress-value/);
});

test("clamps values to the supported range", async ({ page }) => {
  await gotoProgressDemo(page);
  const progress = progressbar(page);
  await page.getByRole("button", { name: "Set above max", exact: true }).first().click();
  await expect(progress).toHaveAttribute("aria-valuenow", "100");
  await expect(progress).toHaveAttribute("data-value", "100");
  await expect(progress).toHaveAttribute("style", /--progress-value:\s*100%/);
  await page.getByRole("button", { name: "Set below min", exact: true }).first().click();
  await expect(progress).toHaveAttribute("aria-valuenow", "0");
  await expect(progress).toHaveAttribute("data-value", "0");
  await expect(progress).toHaveAttribute("style", /--progress-value:\s*0%/);
});

test("uses zero progress when max is not positive", async ({ page }) => {
  await gotoProgressDemo(page);
  const progress = progressbar(page);
  await page.getByRole("button", { name: "Set max zero", exact: true }).first().click();
  await expect(progress).toHaveAttribute("aria-valuemax", "0");
  await expect(progress).toHaveAttribute("aria-valuenow", "0");
  await expect(progress).toHaveAttribute("data-value", "0");
  await expect(progress).toHaveAttribute("style", /--progress-value:\s*0%/);

  await page.getByRole("button", { name: "Set max negative", exact: true }).first().click();
  await expect(progress).toHaveAttribute("aria-valuemax", "0");
  await expect(progress).toHaveAttribute("aria-valuenow", "0");
  await expect(progress).toHaveAttribute("data-max", "0");
  await expect(progress).toHaveAttribute("style", /--progress-value:\s*0%/);
});