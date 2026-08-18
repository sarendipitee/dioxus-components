import { test, expect, type Page } from "@playwright/test";

const loadNavbar = async (page: Page) => {
  await page.goto("/components/navbar", { timeout: 30 * 1000 });
  const navigation = page
    .locator("#component-preview-frame")
    .first()
    .locator('[role="navigation"][aria-label="Components"]');
  await expect(navigation).toBeVisible();
  return navigation;
};

test("mobile navigation uses a narrow viewport and fits the viewport", async ({
  page,
}) => {
  await page.setViewportSize({ width: 375, height: 667 });
  const navigation = await loadNavbar(page);
  expect(
    await page.evaluate(() => document.documentElement.scrollWidth),
  ).toBeLessThanOrEqual(375);
  const trigger = navigation.getByRole("menuitem", { name: "Inputs" });
  await trigger.tap();
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(page.getByRole("menuitem", { name: "Calendar" })).toBeVisible();
  await page.getByRole("menuitem", { name: "Calendar" }).tap();
  await expect(page).toHaveURL(/\/components\/calendar\?/);
});

test("Escape dismisses open content and restores trigger focus", async ({
  page,
}) => {
  const navigation = await loadNavbar(page);
  const menubar = navigation.getByRole("menubar");
  const trigger = navigation.getByRole("menuitem", { name: "Inputs" });
  await menubar.focus();
  await page.keyboard.press("Enter");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await page.keyboard.press("Escape");
  await expect(trigger).toBeFocused();
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(page.locator("#navbar-inputs-content")).toHaveAttribute(
    "data-state",
    "closed",
  );
});

test("outside click and Tab dismiss open content", async ({ page }) => {
  const navigation = await loadNavbar(page);
  const trigger = navigation.getByRole("menuitem", { name: "Inputs" });
  await trigger.click();
  await page.mouse.click(2, 2);
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await trigger.click();
  await page.keyboard.press("Tab");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
});
