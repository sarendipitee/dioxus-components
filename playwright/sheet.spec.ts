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

test("sheet root wrapper exists and reflects open state", async ({ page }) => {
  await gotoSheetDemo(page);

  const root = page.locator('[data-slot="sheet-root"]');

  // Root wrapper should always exist in DOM, initially closed
  await expect(root).toHaveAttribute("data-state", "closed");

  // Open sheet
  await page.getByRole("button", { name: "Right" }).click();
  await expect(root).toHaveAttribute("data-state", "open");

  // Close with Escape
  await page.keyboard.press("Escape");
  await expect(root).toHaveAttribute("data-state", "closed");
});

test("sheet panel appears on the correct side", async ({ page }) => {
  await gotoSheetDemo(page);
  const viewport = page.viewportSize()!;

  for (const [buttonName, side, edgeCheck] of [
    ["Right", "right", (box: { x: number; y: number; width: number; height: number }) => {
      // Right edge of sheet panel should be at viewport right edge
      expect(box.x + box.width).toBe(viewport.width);
    }],
    ["Left", "left", (box: { x: number; y: number; width: number; height: number }) => {
      expect(box.x).toBe(0);
    }],
    ["Top", "top", (box: { x: number; y: number; width: number; height: number }) => {
      expect(box.y).toBe(0);
    }],
    ["Bottom", "bottom", (box: { x: number; y: number; width: number; height: number }) => {
      expect(box.y + box.height).toBe(viewport.height);
    }],
  ] as const) {
    await openSheet(page, buttonName);

    const content = page.locator('[data-slot="sheet-content"]').first();
    // Wait for slide-in animation to finish (200ms ease-out)
    await page.waitForTimeout(300);

    const box = await content.boundingBox();
    expect(box, "sheet content should have non-zero bounding box").toBeTruthy();
    edgeCheck(box!);

    await page.keyboard.press("Escape");
    await expect(page.getByRole("dialog", { name: "Sheet Title" })).toBeHidden();
  }
});

test("sheet backdrop covers viewport and catches clicks", async ({ page }) => {
  await gotoSheetDemo(page);

  await openSheet(page, "Right");

  // The root wrapper should be fixed and cover the viewport
  const root = page.locator('[data-slot="sheet-root"]');
  await expect(root).toHaveCSS("position", "fixed");
  await expect(root).toHaveCSS("top", "0px");
  await expect(root).toHaveCSS("right", "0px");
  await expect(root).toHaveCSS("bottom", "0px");
  await expect(root).toHaveCSS("left", "0px");

  // Click far left of the sheet panel — on the backdrop
  await page.mouse.click(5, 200);
  await expect(page.getByRole("dialog", { name: "Sheet Title" })).toBeHidden();
});
