import { expect, test, type Page } from "@playwright/test";

const SKELETON_URL = "/components/skeleton/block#main";

async function gotoSkeletonDemo(page: Page) {
  await page.goto(SKELETON_URL, { timeout: 30_000, waitUntil: "load" });
}

test("exposes a busy loading region with decorative placeholders", async ({
  page,
}) => {
  await gotoSkeletonDemo(page);

  const region = page.getByRole("region", { name: "Profile loading preview" });
  await expect(region).toHaveAttribute("aria-busy", "true");
  await expect(page.locator("#skeleton-status")).toHaveText(
    "Profile is loading",
  );

  for (const id of [
    "skeleton-avatar",
    "skeleton-primary-line",
    "skeleton-secondary-line",
  ]) {
    await expect(page.locator(`#${id}`)).toHaveAttribute("aria-hidden", "true");
  }
});

test("forwards attributes and preserves requested placeholder geometry", async ({
  page,
}) => {
  await gotoSkeletonDemo(page);

  const avatar = page.locator("#skeleton-avatar");
  await expect(avatar).toHaveAttribute("data-shape", "circle");
  await expect(avatar).toHaveAttribute("title", "Circular avatar placeholder");
  await expect(avatar).toHaveCSS("width", "48px");
  await expect(avatar).toHaveCSS("height", "48px");
  await expect(avatar).toHaveCSS("border-radius", "50%");
  await expect(page.locator("#skeleton-primary-line")).toHaveCSS(
    "width",
    "186px",
  );
  await expect(page.locator("#skeleton-secondary-line")).toHaveCSS(
    "width",
    "136px",
  );
});

test("reactively replaces loading placeholders with loaded content", async ({
  page,
}) => {
  await gotoSkeletonDemo(page);

  await page.getByRole("button", { name: "Show loaded profile" }).click();

  await expect(
    page.getByRole("region", { name: "Profile loading preview" }),
  ).toHaveAttribute("aria-busy", "false");
  await expect(page.locator("#skeleton-status")).toHaveText("Profile loaded");
  await expect(page.getByText("Ada Lovelace", { exact: true })).toBeVisible();
  await expect(
    page.getByText("Computing pioneer", { exact: true }),
  ).toBeVisible();
  await expect(page.locator("#skeleton-avatar")).toHaveCount(0);
});

test("disables pulsing when reduced motion is requested", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await gotoSkeletonDemo(page);

  await expect(page.locator("#skeleton-avatar")).toHaveCSS(
    "animation-name",
    "none",
  );
});
