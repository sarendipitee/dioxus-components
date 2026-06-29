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
  // After unifying Dialog/Sheet sub-components (1ed84d44), the close button
  // is the first child of DialogContent and therefore the first focusable
  // element in the focus trap. The class is now dx_dialog_close (not
  // dx_sheet_close which was removed in that unification).
  const closeButton = dialog.locator(".dx_dialog_close");

  await expect(sheetContent).toHaveAttribute("data-side", "right");
  // Focus trap starts at the first focusable element: the close button.
  await expect(closeButton).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(nameInput).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(usernameInput).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(saveButton).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(cancelButton).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(closeButton).toBeFocused();

  await page.keyboard.press("Escape");
  await expect(dialog).toBeHidden();

  const reopenedDialog = await openSheet(page, "Right");
  const reopenedCloseButton = reopenedDialog.locator(".dx_dialog_close");
  await reopenedCloseButton.focus();
  await expect(reopenedCloseButton).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(reopenedDialog).toBeHidden();
});

test("sheet title and description keep primitive typography wrappers", async ({ page }) => {
  await gotoSheetDemo(page);

  const dialog = await openSheet(page, "Right");
  const sheetContent = page.locator('[data-slot="sheet-content"]').first();
  const title = dialog.locator('[data-slot="sheet-title"]');
  const description = dialog.locator('[data-slot="sheet-description"]');

  await expect(title).toHaveCount(1);
  await expect(description).toHaveCount(1);
  await expect(title).toHaveClass(/dx_heading/);
  await expect(description).toHaveClass(/dx_text/);
  await expect(title).toHaveAttribute("data-tone", "default");
  await expect(description).toHaveAttribute("data-tone", "default");

  const titleId = await title.getAttribute("id");
  const descriptionId = await description.getAttribute("id");
  expect(titleId).toBeTruthy();
  expect(descriptionId).toBeTruthy();
  await expect(dialog).toHaveAttribute("aria-labelledby", titleId!);
  await expect(dialog).toHaveAttribute("aria-describedby", descriptionId!);

  await expect(title).toHaveJSProperty("tagName", "H2");
  await expect(description).toHaveJSProperty("tagName", "P");
  await expect(title.locator("h1,h2,h3,h4,h5,h6")).toHaveCount(0);
  await expect(description.locator("p")).toHaveCount(0);

  const [sheetColor, descriptionColor] = await Promise.all([
    sheetContent.evaluate((element) => getComputedStyle(element).color),
    description.evaluate((element) => getComputedStyle(element).color),
  ]);
  expect(descriptionColor).toBe(sheetColor);
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

  // The block demo renders inside #dx-preview-block-root; scope to it so the
  // live instance is targeted rather than the component definition-tree copy
  // that also mounts a [data-slot="sheet-root"] elsewhere on the page.
  const root = page.locator('#dx-preview-block-root [data-slot="sheet-root"]');

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

  // The backdrop should be fixed and cover the viewport. Sheet now renders its
  // dismiss backdrop through the shared dialog backdrop element, so assert
  // against that fixed, inset-0 layer rather than the (now static) root wrapper.
  const backdrop = page.locator(".dx_dialog_backdrop");
  await expect(backdrop).toHaveCSS("position", "fixed");
  await expect(backdrop).toHaveCSS("top", "0px");
  await expect(backdrop).toHaveCSS("right", "0px");
  await expect(backdrop).toHaveCSS("bottom", "0px");
  await expect(backdrop).toHaveCSS("left", "0px");

  // Click far left of the sheet panel — on the backdrop
  await page.mouse.click(5, 200);
  await expect(page.getByRole("dialog", { name: "Sheet Title" })).toBeHidden();
});

test("closing a nested sheet does not crash the page", async ({ page }) => {
  await page.goto("/components/sheet/block#nested", {
    timeout: 20 * 60 * 1000,
    waitUntil: "load",
  });

  // Open the first sheet
  await page.getByRole("button", { name: "Open Sheet" }).click();
  const sheet1 = page.getByRole("dialog", { name: "Sheet 1" });
  await expect(sheet1).toBeVisible();

  // Open the nested sheet from inside sheet 1
  await sheet1.getByRole("button", { name: "Open Sheet 2" }).click();
  const sheet2 = page.getByRole("dialog", { name: "Sheet 2" });
  await expect(sheet2).toBeVisible();

  // Close the inner sheet first — should not crash
  await page.keyboard.press("Escape");
  await expect(sheet2).toHaveCount(0);

  // The outer sheet must still be usable
  await expect(sheet1).toBeVisible();

  // Close the outer sheet
  await page.keyboard.press("Escape");
  await expect(sheet1).toHaveCount(0);
});

test("same-side nested sheets get sheet depth styling", async ({ page }) => {
  await page.goto("/components/sheet/block#nested", {
    timeout: 20 * 60 * 1000,
    waitUntil: "load",
  });

  await page.getByRole("button", { name: "Open Sheet" }).click();
  const sheet1 = page.getByRole("dialog", { name: "Sheet 1" });
  await expect(sheet1).toBeVisible();

  await sheet1.getByRole("button", { name: "Open Sheet 2" }).click();
  const sheet2 = page.getByRole("dialog", { name: "Sheet 2" });
  await expect(sheet2).toBeVisible();

  await expect(sheet1).toHaveAttribute("data-overlay-depth", "1");
  await expect(sheet1).toHaveAttribute("data-overlay-sheet-depth", "1");
  await expect(sheet2).toHaveAttribute("data-overlay-sheet-depth", "0");

  await page.waitForTimeout(300);
  const outerOpacity = await sheet1.evaluate((element) =>
    Number.parseFloat(getComputedStyle(element).opacity)
  );
  expect(outerOpacity).toBeCloseTo(0.88, 2);
});

test("opposite-side nested sheets do not get sheet depth styling", async ({ page }) => {
  await page.goto("/overlay-nesting", {
    timeout: 20 * 60 * 1000,
    waitUntil: "load",
  });

  await page.getByTestId("open-sheet-1").click();
  const outer = page.getByTestId("sheet-outer");
  await expect(outer).toBeVisible();

  await outer.getByTestId("open-sheet-2").click();
  const inner = page.getByTestId("sheet-inner");
  await expect(inner).toBeVisible();

  await expect(outer).toHaveAttribute("data-overlay-depth", "1");
  await expect(outer).toHaveAttribute("data-overlay-sheet-depth", "0");
  await expect(inner).toHaveAttribute("data-overlay-sheet-depth", "0");

  await page.waitForTimeout(300);
  const outerOpacity = await outer.evaluate((element) =>
    Number.parseFloat(getComputedStyle(element).opacity)
  );
  expect(outerOpacity).toBe(1);
});

test("tearing down outer scope while inner is still animating does not crash", async ({ page }) => {
  // This test exercises the UAF window: the inner DialogRoot and its signals
  // live inside the outer DialogPortalBody's children subtree. When the outer
  // scope is torn down (outer sheet closes), those signals are freed. If the
  // inner portaled body is still mounted (exit animation in progress) and reads
  // those signals reactively, that is a use-after-free — manifesting as a WASM
  // dlmalloc abort that kills the page.
  //
  // The inner sheet is modal and renders a pointer-blocking backdrop. Clicking
  // the outer sheet's close button through that backdrop is not possible (even
  // with Playwright's `force: true`, which moves the mouse to coordinates and
  // the backing element at that position intercepts). Instead, the correct
  // sequence to trigger the UAF window is:
  //   1. Close the INNER sheet (Escape) — starts its 150ms exit animation.
  //   2. Immediately close the OUTER sheet (Escape again) — tears down the
  //      outer scope while the inner portaled body is still mounted/animating.
  // The inner portaled body survives through the outer teardown. Without the
  // snapshot fix in dialog.rs, any reactive read of the freed inner DialogRoot
  // signals from the portaled body causes a heap-corruption abort.
  await page.goto("/components/sheet/block#nested", {
    timeout: 20 * 60 * 1000,
    waitUntil: "load",
  });

  const sheet1 = page.getByRole("dialog", { name: "Sheet 1" });
  const sheet2 = page.getByRole("dialog", { name: "Sheet 2" });

  // Open both sheets
  await page.getByRole("button", { name: "Open Sheet" }).click();
  await expect(sheet1).toBeVisible();
  await sheet1.getByRole("button", { name: "Open Sheet 2" }).click();
  await expect(sheet2).toBeVisible();

  // Close inner (sheet2) — its 150ms exit animation begins.
  await page.keyboard.press("Escape");
  // Immediately close outer (sheet1) — sheet2's portaled body is still mounted
  // and animating. Without the snapshot fix, WASM aborts here.
  await page.keyboard.press("Escape");

  // Both dialogs must disappear once animations finish (≤ 500ms).
  await expect(sheet1).toHaveCount(0, { timeout: 2000 });
  await expect(sheet2).toHaveCount(0, { timeout: 2000 });

  // Page-health assertion: interact with the WASM runtime after the close
  // sequence. A crashed runtime would leave the page unresponsive — reopening
  // the sheet and asserting it appears proves the runtime is still alive.
  await page.getByRole("button", { name: "Open Sheet" }).click();
  await expect(sheet1).toBeVisible({ timeout: 5000 });

  // Clean up
  await page.keyboard.press("Escape");
  await expect(sheet1).toHaveCount(0);
});
