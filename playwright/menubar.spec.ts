import { test, expect } from "@playwright/test";

test("pointer navigation", async ({ page }) => {
  await page.goto("/component/?name=menubar&", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  const fileMenuButton = page.getByRole("menuitem", { name: "File" }).first();
  await fileMenuButton.click();
  // Assert the menu is open
  const fileMenuContent = page.getByRole("menu").filter({ has: page.getByRole("menuitem", { name: "New" }).first() }).first();
  await expect(fileMenuContent).toHaveAttribute("data-state", "open");
  await expect(fileMenuContent.locator('.dx_menubar_label', { hasText: "File" }).first()).toBeVisible();
  await expect(fileMenuContent.getByRole("menuitem", { name: "New" })).toContainText("⌘N");
  await expect(fileMenuContent.getByRole("separator")).toHaveCount(1);
  await expect(fileMenuContent.getByRole("menuitemcheckbox", { name: "Status bar" })).toHaveAttribute("data-state", "checked");
  const shareItem = fileMenuContent.getByRole("menuitem", { name: "Share" });
  await shareItem.hover();
  const submenu = page.locator(".dx_menubar_sub_content").first();
  await expect(submenu).toHaveAttribute("data-state", "open");
  await expect(submenu.getByRole("menuitem", { name: "Copy link" })).toBeVisible();
  await expect(shareItem).toHaveCSS("background-color", "rgb(247, 247, 247)");
  const shareBox = await shareItem.boundingBox();
  const submenuBox = await submenu.boundingBox();
  if (!shareBox || !submenuBox) throw new Error("submenu geometry unavailable");
  expect(submenuBox.x).toBeGreaterThanOrEqual(shareBox.x + shareBox.width - 8);
  expect(Math.abs(submenuBox.y - shareBox.y)).toBeLessThanOrEqual(12);
  await submenu.getByRole("menuitem", { name: "Invite" }).hover();
  await expect(fileMenuContent).toHaveAttribute("data-state", "open");
  await page.mouse.move(shareBox.x - 24, shareBox.y + shareBox.height / 2);
  await expect(submenu).toHaveAttribute("data-state", "closed");
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
  const editMenuContent = page.getByRole("menu").filter({ has: page.getByRole("menuitemradio", { name: "Name" }).first() }).first();
  await expect(editMenuContent).toHaveAttribute("data-state", "open");
  // Assert the File menu content is closed
  await expect(fileMenuContent).toHaveCount(0);

  // Click the Date modified menu item
  const cutItem = editMenuContent.getByRole("menuitemradio", { name: "Date modified" });
  await cutItem.click();
  // Assert the menu is closed after clicking a menu item
  await expect(fileMenuContent).toHaveCount(0);
  await expect(page.getByText("Sort: date")).toBeVisible();
});

test("keyboard navigation", async ({ page }) => {
  await page.goto("/component/?name=menubar&", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole("menubar").first().focus();
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
  const fileMenuContent = page.getByRole("menu").filter({ has: page.getByRole("menuitem", { name: "New" }).first() }).first();
  await expect(fileMenuContent).toHaveAttribute("data-state", "open");

  // assert the new item is focused
  const newItem = fileMenuContent.getByRole("menuitem", { name: "New" });
  await expect(newItem).toBeFocused();
  await expect(fileMenuContent.getByRole("menuitem", { name: "Open" })).toHaveAttribute("data-disabled", "true");
  await page.keyboard.press("ArrowDown");
  await expect(fileMenuContent.getByRole("menuitem", { name: "Share" })).toBeFocused();
  // Click the focused Save menu item
  await page.keyboard.press("ArrowRight");
  await expect(page.getByRole("menuitem", { name: "Copy link" }).first()).toBeFocused();
  await page.keyboard.press("Enter");
  // Assert the menu is closed after clicking a menu item
  await expect(fileMenuContent).toHaveCount(0);
  await expect(page.getByText("Selected: Copy link")).toBeVisible();
});
