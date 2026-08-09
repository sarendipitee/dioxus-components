import { test, expect } from "@playwright/test";

test.describe("toolbar", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/components/toolbar");
  });

  test("exposes toolbar semantics, groups, separator, orientation, and root attributes", async ({
    page,
  }) => {
    const toolbar = page
      .getByRole("toolbar", { name: "Text formatting" })
      .first();
    await expect(toolbar).toHaveAttribute("aria-orientation", "horizontal");
    await expect(toolbar).toHaveAttribute("data-orientation", "horizontal");
    await expect(toolbar).toHaveAttribute("title", "Formatting controls");
    await expect(toolbar).toHaveAttribute("data-disabled", "false");
    await expect(
      toolbar.getByRole("group", { name: "Text style" }),
    ).toBeVisible();
    await expect(
      toolbar.getByRole("group", { name: "Alignment" }),
    ).toBeVisible();
    await expect(toolbar.getByRole("separator")).toHaveCount(1);
    await expect(
      page
        .getByRole("toolbar", { name: "Document actions" })
        .first()
        .getByRole("link", { name: "Help" }),
    ).toBeVisible();
    await expect(toolbar.locator("[tabindex='0']")).toHaveCount(1);
  });

  test("roves focus with arrows and clamps at Home/End boundaries", async ({
    page,
  }) => {
    const buttons = page
      .getByRole("toolbar", { name: "Text formatting" })
      .first()
      .getByRole("button");
    await page.locator("#component-preview-frame").focus();
    await page.keyboard.press("Tab");
    await expect(buttons.nth(0)).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(buttons.nth(0)).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(buttons.nth(1)).toBeFocused();
    await page.keyboard.press("Home");
    await expect(buttons.nth(0)).toBeFocused();
    await page.keyboard.press("End");
    await expect(buttons.nth(5)).toBeFocused();
    await page.keyboard.press("ArrowRight");
    await expect(buttons.nth(5)).toBeFocused();
    await expect(buttons.nth(5)).toHaveAttribute("tabindex", "0");
    await expect(buttons.nth(0)).toHaveAttribute("tabindex", "-1");
  });

  test("keeps formatting toggles independent and alignment mutually exclusive", async ({
    page,
  }) => {
    const bold = page.getByRole("button", { name: "Bold" }).first();
    const italic = page.getByRole("button", { name: "Italic" }).first();
    const left = page.getByRole("button", { name: "Align Left" }).first();
    const center = page.getByRole("button", { name: "Align Center" }).first();
    const status = page.getByTestId("formatting-status").first();
    await expect(bold).toHaveAttribute("aria-pressed", "false");
    await expect(left).toHaveAttribute("aria-pressed", "true");
    await bold.click();
    await italic.click();
    await expect(bold).toHaveAttribute("aria-pressed", "true");
    await expect(italic).toHaveAttribute("aria-pressed", "true");
    await center.click();
    await expect(center).toHaveAttribute("aria-pressed", "true");
    await expect(left).toHaveAttribute("aria-pressed", "false");
    await expect(status).toContainText("Alignment: center");
  });
});
