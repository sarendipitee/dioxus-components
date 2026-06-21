import { expect, test, type Page } from "@playwright/test";

const SHEET_DEMO_URL = "/components/sheet/block#main";

async function gotoSheetDemo(page: Page) {
  await page.goto(SHEET_DEMO_URL, {
    timeout: 20 * 60 * 1000,
    waitUntil: "load",
  });
}

async function openSheet(page: Page, side: "Top" | "Right" | "Bottom" | "Left") {
  await page.getByRole("button", { name: side }).click();

  const dialog = page.getByRole("dialog", { name: "Sheet Title" });
  await expect(dialog).toBeVisible();

  return dialog;
}

test("sheet basic interactions", async ({ page }) => {
  await gotoSheetDemo(page);

  const dialog = await openSheet(page, "Right");
  const sheetContent = page.locator('[data-slot="sheet-content"]').first();
  const nameInput = dialog.locator("#sheet-demo-name");
  const usernameInput = dialog.locator("#sheet-demo-username");
  const saveButton = dialog.getByRole("button", { name: "Save changes" });
  const cancelButton = dialog.getByRole("button", { name: "Cancel" });
  const closeButton = dialog.locator(".dx_sheet_close");

  await expect(sheetContent).toHaveAttribute("data-side", "right");
  await expect(nameInput).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(usernameInput).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(saveButton).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(cancelButton).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(closeButton).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(nameInput).toBeFocused();

  await page.keyboard.press("Escape");
  await expect(dialog).toBeHidden();

  const reopenedDialog = await openSheet(page, "Right");
  const reopenedCloseButton = reopenedDialog.locator(".dx_sheet_close");
  await reopenedCloseButton.focus();
  await expect(reopenedCloseButton).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(reopenedDialog).toBeHidden();
});

test("sheet opens from different sides", async ({ page }) => {
  await gotoSheetDemo(page);

  for (const [buttonName, side] of [
    ["Top", "top"],
    ["Bottom", "bottom"],
    ["Left", "left"],
  ] as const) {
    await openSheet(page, buttonName);
    await expect(page.locator('[data-slot="sheet-content"]').first()).toHaveAttribute(
      "data-side",
      side,
    );
    await page.keyboard.press("Escape");
    await expect(page.getByRole("dialog", { name: "Sheet Title" })).toBeHidden();
  }
});
