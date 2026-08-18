import { test, expect } from "@playwright/test";

test.describe("toggle group", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/components/toggle_group/block#main");
  });
  const preview = (page: import("@playwright/test").Page) =>
    page.locator("#dx-preview-block-root");

  test("exposes group and item semantics, state, orientation, and global attributes", async ({
    page,
  }) => {
    const group = page
      .locator("#dx-preview-block-root")
      .getByRole("group", { name: "Text formatting" })
      .first();
    await expect(group).toHaveAttribute("id", "multiple-group");
    await expect(group).toHaveAttribute("data-orientation", "horizontal");
    await expect(group).toHaveAttribute("data-allow-multiple-pressed", "true");

    for (const name of ["Bold", "Italic", "Underline"]) {
      const button = group.getByRole("button", { name });
      await expect(button).toHaveAttribute("aria-pressed", "false");
      await expect(button).toHaveAttribute("data-state", "off");
      await expect(button).toHaveAttribute("data-orientation", "horizontal");
    }
  });

  test("supports multiple selection and toggling an item off", async ({
    page,
  }) => {
    const group = preview(page).getByTestId("multiple-group").first();
    const bold = group.getByRole("button", { name: "Bold" });
    const italic = group.getByRole("button", { name: "Italic" });

    await bold.click();
    await italic.click();
    await expect(bold).toHaveAttribute("aria-pressed", "true");
    await expect(bold).toHaveAttribute("data-state", "on");
    await expect(italic).toHaveAttribute("aria-pressed", "true");
    await bold.click();
    await expect(bold).toHaveAttribute("aria-pressed", "false");
    await expect(bold).toHaveAttribute("data-state", "off");
    await expect(italic).toHaveAttribute("aria-pressed", "true");
  });

  test("keeps controlled single selection exclusive and reports each real click once", async ({
    page,
  }) => {
    const group = preview(page).getByTestId("controlled-single").first();
    const left = group.getByRole("button", { name: "Left" });
    const center = group.getByRole("button", { name: "Center" });
    const right = group.getByRole("button", { name: "Right" });
    const value = preview(page).getByTestId("controlled-value").first();
    const count = preview(page).getByTestId("callback-count").first();

    await expect(left).toHaveAttribute("aria-pressed", "true");
    await expect(value).toHaveText("0");
    await expect(count).toHaveText("0");
    await center.click();
    await expect(center).toHaveAttribute("aria-pressed", "true");
    await expect(left).toHaveAttribute("aria-pressed", "false");
    await expect(right).toHaveAttribute("aria-pressed", "false");
    await expect(value).toHaveText("1");
    await expect(count).toHaveText("1");
    await center.click();
    await expect(center).toHaveAttribute("aria-pressed", "false");
    await expect(value).toHaveText("empty");
    await expect(count).toHaveText("2");
    await right.click();
    await expect(right).toHaveAttribute("aria-pressed", "true");
    await expect(value).toHaveText("2");
    await expect(count).toHaveText("3");
  });

  test("navigates vertically past disabled items", async ({ page }) => {
    const group = preview(page).getByTestId("vertical-group").first();
    const top = group.getByRole("button", { name: "Top" });
    const middle = group.getByRole("button", { name: "Middle" });
    const bottom = group.getByRole("button", { name: "Bottom" });
    await expect(group).toHaveAttribute("data-orientation", "vertical");
    await expect(middle).toBeDisabled();
    await top.focus();
    await expect(group.locator('button[tabindex="0"]')).toHaveCount(1);
    await page.keyboard.press("ArrowDown");
    await expect(bottom).toBeFocused();
    await expect(group.locator('button[tabindex="0"]')).toHaveCount(1);
    await page.keyboard.press("ArrowUp");
    await expect(top).toBeFocused();
  });

  test("clamps keyboard focus at no-loop boundaries", async ({ page }) => {
    const group = preview(page).getByTestId("no-loop-group").first();
    const first = group.getByRole("button", { name: "First" });
    const last = group.getByRole("button", { name: "Last" });
    await expect(group.locator('button[tabindex="0"]')).toHaveCount(1);
    await first.focus();
    await page.keyboard.press("ArrowLeft");
    await expect(first).toBeFocused();
    await expect(group.locator('button[tabindex="0"]')).toHaveCount(1);
    await last.focus();
    await page.keyboard.press("ArrowRight");
    await expect(last).toBeFocused();
  });

  test("reverses horizontal keyboard navigation in RTL", async ({ page }) => {
    const group = preview(page).getByTestId("rtl-group").first();
    const one = group.getByRole("button", { name: "One" });
    const two = group.getByRole("button", { name: "Two" });
    const three = group.getByRole("button", { name: "Three" });
    await expect(group).toHaveAttribute("dir", "rtl");
    await two.focus();
    await page.keyboard.press("ArrowRight");
    await expect(one).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(two).toBeFocused();
    await page.keyboard.press("ArrowLeft");
    await expect(three).toBeFocused();
  });

  test("disables the root and every item", async ({ page }) => {
    const group = preview(page).getByTestId("disabled-group").first();
    await expect(group).toHaveAttribute("aria-disabled", "true");
    await expect(group).toHaveAttribute("data-disabled", "true");
    await expect(group).toHaveAttribute("data-orientation", "horizontal");
    for (const name of ["Disabled one", "Disabled two"]) {
      await expect(group.getByRole("button", { name })).toBeDisabled();
      await expect(group.getByRole("button", { name })).toHaveAttribute(
        "data-disabled",
        "true",
      );
    }
  });
});
