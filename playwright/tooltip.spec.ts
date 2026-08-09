import { test, expect, type Page } from "@playwright/test";

const FRAME = "#component-preview-frame";

function fixture(page: Page) {
  const frame = page.locator(FRAME).first();
  return {
    frame,
    root: frame.getByTestId("tooltip-root"),
    trigger: frame.getByTestId("tooltip-trigger"),
    content: page.getByTestId("tooltip-content"),
  };
}

test.describe("Tooltip", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/components/tooltip");
  });

  test("hover and focus open; pointer exit, blur, and Escape close", async ({
    page,
  }) => {
    const { frame, trigger, content } = fixture(page);

    await trigger.hover();
    await expect(content).toBeVisible();
    await frame.hover({ position: { x: 2, y: 2 } });
    await expect(content).toHaveCount(0);

    await trigger.focus();
    await expect(content).toBeVisible();
    await trigger.blur();
    await expect(content).toHaveCount(0);

    await trigger.focus();
    await expect(content).toBeVisible();
    await trigger.press("Escape");
    await expect(content).toHaveCount(0);
    await expect(trigger).toBeFocused();
    await expect(trigger).not.toHaveAttribute("aria-describedby");
  });
});
