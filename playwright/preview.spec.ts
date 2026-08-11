import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test.describe("homepage", () => {
  test("hero should not have any automatically detectable accessibility issues", async ({
    page,
  }) => {
    await page.goto("/", { timeout: 30 * 1000 });

    await expect(page.locator("#hero")).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page })
      .include("#hero")
      .disableRules("color-contrast")
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});

test.describe("theme studio", () => {
  test("opens without losing its customizer context", async ({ page }) => {
    const pageErrors: Error[] = [];
    page.on("pageerror", (error) => pageErrors.push(error));

    await page.goto("/theme", { timeout: 30 * 1000 });
    await page.getByRole("button", { name: "Open theme studio" }).click();

    await expect(page.getByRole("heading", { name: "Theme studio", level: 2 })).toBeVisible();
    await expect(page.getByRole("heading", { name: "Core" })).toBeVisible();
    expect(pageErrors).toEqual([]);
  });
});

test.describe("details", () => {
  test("should not have any automatically detectable accessibility issues", async ({
    page,
  }) => {
    await page.goto("/components/calendar", { timeout: 30 * 1000 });

    await expect(
      page.getByRole("heading", { name: /^calendar$/i }).first(),
    ).toBeVisible();

    const accessibilityScanResults = await new AxeBuilder({ page })
      .exclude(".dx-component-props-table-wrap")
      .disableRules("color-contrast")
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});
