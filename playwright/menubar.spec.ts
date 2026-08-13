import { test, expect } from "@playwright/test";

test("pointer navigation", async ({ page }) => {
  await page.emulateMedia({ colorScheme: "light" });
  await page.goto("/components/menubar", { timeout: 30 * 1000 });
  const fileMenuButton = page.getByRole("menuitem", { name: "File" }).first();
  await fileMenuButton.click();
  // Assert the menu is open
  const fileMenuContent = page
    .getByRole("menu")
    .filter({ has: page.getByRole("menuitem", { name: "New" }).first() })
    .first();
  await expect(fileMenuContent).toHaveAttribute("data-state", "open");
  await expect(
    fileMenuContent.locator(".dx_menu_label", { hasText: "File" }).first(),
  ).toBeVisible();
  await expect(
    fileMenuContent.getByRole("menuitem", { name: "New" }),
  ).toContainText("⌘N");
  await expect(fileMenuContent.getByRole("separator")).toHaveCount(1);
  await expect(
    fileMenuContent.getByRole("menuitemcheckbox", { name: "Status bar" }),
  ).toHaveAttribute("data-state", "checked");
  const shareItem = fileMenuContent.getByRole("menuitem", { name: "Share" });
  await expect(shareItem).toHaveAttribute("aria-haspopup", "menu");
  await expect(shareItem).toHaveAttribute("aria-expanded", "false");
  await shareItem.hover();
  const submenu = page.locator(".dx_menu_sub_content").first();
  await expect(submenu).toHaveAttribute("data-state", "open");
  const grace = submenu.locator("[data-menu-pointer-grace]");
  await expect(grace).toBeVisible();
  await expect(grace).toHaveCSS("pointer-events", "auto");
  await expect(grace).toHaveCSS("width", "128px");
  await expect(shareItem).toHaveAttribute("aria-expanded", "true");
  await expect(
    submenu.getByRole("menuitem", { name: "Copy link" }),
  ).toBeVisible();
  await fileMenuContent.getByRole("menuitem", { name: "New" }).hover();
  await expect(shareItem).toHaveAttribute("aria-expanded", "false");
  await shareItem.hover();
  await expect(submenu).toHaveAttribute("data-state", "open");
  await expect(shareItem).toHaveCSS("background-color", "rgb(247, 247, 247)");
  const shareBox = await shareItem.boundingBox();
  const submenuBox = await submenu.boundingBox();
  if (!shareBox || !submenuBox) throw new Error("submenu geometry unavailable");
  expect(submenuBox.x).toBeGreaterThanOrEqual(shareBox.x + shareBox.width - 8);
  expect(Math.abs(submenuBox.y - shareBox.y)).toBeLessThanOrEqual(12);
  await submenu.getByRole("menuitem", { name: "Invite" }).hover();
  await expect(fileMenuContent).toHaveAttribute("data-state", "open");
  await shareItem.hover();
  await expect(submenu).toHaveAttribute("data-state", "open");
  await submenu.getByRole("menuitem", { name: "Invite" }).click();
  await expect(page.getByText("Selected: Invite")).toBeVisible();

  await fileMenuButton.click();
  await expect(fileMenuContent).toHaveAttribute("data-state", "open");

  // After the menu is open, hover over the Edit menu item
  const editMenuButton = page.getByRole("menuitem", { name: "View" }).first();
  await editMenuButton.hover();
  // Assert the Edit menu content is open
  const editMenuContent = page
    .getByRole("menu")
    .filter({ has: page.getByRole("menuitemradio", { name: "Name" }).first() })
    .first();
  await expect(editMenuContent).toHaveAttribute("data-state", "open");
  // Assert the File menu content is closed
  await expect(fileMenuContent).toHaveCount(0);

  // Click the Date modified menu item
  const cutItem = editMenuContent.getByRole("menuitemradio", {
    name: "Date modified",
  });
  await cutItem.click();
  // Assert the menu is closed after clicking a menu item
  await expect(fileMenuContent).toHaveCount(0);
  await expect(page.getByText("Sort: date")).toBeVisible();
});

test("keyboard navigation", async ({ page }) => {
  await page.goto("/components/menubar", { timeout: 30 * 1000 });
  const menubar = page.getByRole("menubar").first();
  await menubar.focus();
  const fileMenuButton = page.getByRole("menuitem", { name: "File" }).first();
  // Go right with the keyboard
  await page.keyboard.press("ArrowRight");
  // Assert the focus is on the View menu item
  const editMenuButton = page.getByRole("menuitem", { name: "View" }).first();
  await expect(editMenuButton).toBeFocused();
  // Go left with the keyboard
  await page.keyboard.press("ArrowLeft");
  // Assert the focus is on the File menu item
  await expect(fileMenuButton).toBeFocused();
  // Open the File menu
  await page.keyboard.press("ArrowDown");
  // Assert the File menu content is open
  const fileMenuContent = page
    .getByRole("menu")
    .filter({ has: page.getByRole("menuitem", { name: "New" }).first() })
    .first();
  await expect(fileMenuContent).toHaveAttribute("data-state", "open");
  const shareItem = fileMenuContent.getByRole("menuitem", { name: "Share" });
  await expect(
    fileMenuContent.getByRole("menuitem", { name: "Open" }),
  ).toHaveAttribute("data-disabled", "true");
  await shareItem.hover();
  // Click the focused Save menu item
  await page.keyboard.press("ArrowRight");
  await expect(
    page.getByRole("menuitem", { name: "Copy link" }).first(),
  ).toBeFocused();
  await page.keyboard.press("Enter");
  // Assert the menu is closed after clicking a menu item
  await expect(fileMenuContent).toHaveCount(0);
  await expect(page.getByText("Selected: Copy link")).toBeVisible();
});

test("trigger exposes menu popup and expanded state", async ({ page }) => {
  await page.goto("/components/menubar/block#main", { timeout: 30 * 1000 });
  const demo = page.locator("#dx-preview-block-root");
  const fileTrigger = demo.getByRole("menuitem", { name: "File" }).first();
  await expect(fileTrigger).toHaveAttribute("aria-haspopup", "menu");
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "false");
  await fileTrigger.click({ force: true });
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "true");
  await fileTrigger.click();
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "false");
});

test("switching triggers closes the previous menu", async ({ page }) => {
  await page.goto("/components/menubar/block#main", { timeout: 30 * 1000 });
  const demo = page.locator("#dx-preview-block-root");
  const fileTrigger = demo.getByRole("menuitem", { name: "File" }).first();
  const viewTrigger = demo.getByRole("menuitem", { name: "View" }).first();
  await fileTrigger.click();
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "true");
  await viewTrigger.hover();
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "false");
  await expect(viewTrigger).toHaveAttribute("aria-expanded", "true");
  await expect(
    page.getByRole("menuitemradio", { name: "Date modified" }),
  ).toBeVisible();
});

test("checkbox toggles and radio selection update their outputs", async ({
  page,
}) => {
  await page.goto("/components/menubar/block#main", { timeout: 30 * 1000 });
  const demo = page.locator("#dx-preview-block-root");
  const fileTrigger = demo.getByRole("menuitem", { name: "File" }).first();
  await fileTrigger.click();
  let statusBar = page.getByRole("menuitemcheckbox", { name: "Status bar" });
  await expect(statusBar).toHaveAttribute("data-state", "checked");
  await statusBar.click();
  await expect(
    page.getByText("Status bar: false", { exact: true }),
  ).toBeVisible();

  statusBar = page.getByRole("menuitemcheckbox", { name: "Status bar" });
  await expect(statusBar).toHaveAttribute("data-state", "unchecked");

  const viewTrigger = page.getByRole("menuitem", { name: "View" }).first();
  await viewTrigger.hover();
  let viewMenu = page
    .getByRole("menu")
    .filter({ has: page.getByRole("menuitemradio", { name: "Name" }).first() })
    .first();
  let dateModified = viewMenu.getByRole("menuitemradio", {
    name: "Date modified",
  });
  await expect(dateModified).toHaveAttribute("data-state", "unchecked");
  await dateModified.click();
  await expect(page.getByText("Sort: date", { exact: true })).toBeVisible();
});

test("disabled item cannot activate", async ({ page }) => {
  await page.goto("/components/menubar/block#main", { timeout: 30 * 1000 });
  const demo = page.locator("#dx-preview-block-root");
  const fileTrigger = demo.getByRole("menuitem", { name: "File" }).first();
  await fileTrigger.click();
  const openItem = page.getByRole("menuitem", { name: "Open" });
  await expect(openItem).toHaveAttribute("data-disabled", "true");
  await openItem.click({ force: true });
  await expect(page.getByText("Selected: New", { exact: true })).toBeVisible();
  await expect(page.getByRole("menuitem", { name: "New" })).toBeVisible();
});

test("Escape dismisses menu and restores trigger focus", async ({ page }) => {
  await page.goto("/components/menubar/block#main", { timeout: 30 * 1000 });
  const demo = page.locator("#dx-preview-block-root");
  const fileTrigger = demo.getByRole("menuitem", { name: "File" }).first();
  await fileTrigger.click();
  await page.keyboard.press("Escape");
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "false");
  await expect(fileTrigger).toBeFocused();
});

test("Tab dismisses menu and moves focus outside menubar", async ({ page }) => {
  await page.goto("/components/menubar/block#main", { timeout: 30 * 1000 });
  const demo = page.locator("#dx-preview-block-root");
  const fileTrigger = demo.getByRole("menuitem", { name: "File" }).first();
  await fileTrigger.click();
  await page.keyboard.press("Tab");
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "false");
  await expect(fileTrigger).not.toBeFocused();
});

test("outside pointer dismisses an open menu", async ({ page }) => {
  await page.goto("/components/menubar/block#main", { timeout: 30 * 1000 });
  const demo = page.locator("#dx-preview-block-root");
  const fileTrigger = demo.getByRole("menuitem", { name: "File" }).first();
  await fileTrigger.click();
  await page.mouse.click(1, page.viewportSize()!.height - 1);
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByRole("menuitem", { name: "New" })).toHaveCount(0);
});
