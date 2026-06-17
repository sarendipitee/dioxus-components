import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test.describe("homepage", () => {
  test("should not have any automatically detectable accessibility issues", async ({
    page,
  }) => {
    await page.goto("/", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes

    await expect(page.locator("#hero")).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page })
      .disableRules("color-contrast")
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});


test.describe("details", () => {
  test("should not have any automatically detectable accessibility issues", async ({
    page,
  }) => {
    await page.goto("/components/calendar", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes

    await expect(
      page.getByRole("heading", { name: /^calendar$/i }).first(),
    ).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page })
      .disableRules("color-contrast")
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});
