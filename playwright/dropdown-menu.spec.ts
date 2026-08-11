import { test, expect } from "@playwright/test";

test("dropdown checkbox and radio keyboard state updates keep menu open", async ({
  page,
}) => {
  await page.goto("/components/dropdown_menu");
  const demo = page.locator("#dx-preview-block-root");
  const trigger = demo.getByRole("button", { name: "Open Menu" });
  const menu = page
    .getByRole("menu")
    .filter({ has: page.getByText("Actions") })
    .first();
  const checkbox = menu.getByRole("menuitemcheckbox", { name: "Show Toolbar" });
  const system = menu.getByRole("menuitemradio", { name: "System" });
  const light = menu.getByRole("menuitemradio", { name: "Light" });

  await trigger.press("ArrowDown");
  await page.keyboard.press("End");
  await page.keyboard.press("ArrowUp");
  await expect(light).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(light).toHaveAttribute("aria-checked", "true");
  await expect(system).toHaveAttribute("aria-checked", "false");
  await expect(demo.getByText("Theme: Light")).toBeVisible();
  await expect(menu).toHaveAttribute("data-state", "open");

  await checkbox.focus();
  await expect(checkbox).toHaveAttribute("aria-checked", "true");
  await page.keyboard.press("Space");
  await expect(checkbox).toHaveAttribute("aria-checked", "false");
  await expect(checkbox).toHaveAttribute("data-state", "unchecked");
  await expect(demo.getByText("Toolbar visible: false")).toBeVisible();
  await expect(menu).toHaveAttribute("data-state", "open");
});

test("first open anchors menu without scrolling trigger", async ({ page }) => {
  await page.goto("/components/dropdown_menu");
  const trigger = page
    .locator("#dx-preview-block-root")
    .getByRole("button", { name: "Open Menu" });

  await trigger.evaluate((element) => element.scrollIntoView({ block: "center" }));
  await page.waitForTimeout(50);

  await trigger.press("ArrowDown");
  const menu = page
    .getByRole("menu")
    .filter({ has: page.getByText("Actions") })
    .first();
  await expect(menu).toBeVisible();

  const triggerBox = await trigger.boundingBox();
  const menuBox = await menu.boundingBox();
  expect(triggerBox).not.toBeNull();
  expect(menuBox).not.toBeNull();
  expect(Math.abs(menuBox!.x - triggerBox!.x)).toBeLessThan(4);
  expect(menuBox!.y).toBeGreaterThanOrEqual(triggerBox!.y + triggerBox!.height);
});

test("nested submenu demo opens each submenu level", async ({ page }) => {
  await page.goto("/#nested_submenus");

  await page.getByRole("button", { name: "Move item" }).click();
  const rootMenu = page.locator('[role="menu"][data-state="open"]').first();
  await expect(rootMenu).toBeVisible();

  const alphaTrigger = rootMenu.getByRole("menuitem", {
    name: "Workspace Alpha",
    exact: true,
  });
  await alphaTrigger.hover();
  await expect(alphaTrigger).toHaveAttribute("aria-expanded", "true");
  const alphaMenu = page
    .getByRole("menu")
    .filter({ has: page.getByText("Alpha folders", { exact: true }) });
  await expect(alphaMenu).toBeVisible();

  await alphaMenu
    .getByRole("menuitem", { name: "Workspace Alpha / Projects", exact: true })
    .hover();
  const projectsMenu = page
    .getByRole("menu")
    .filter({ has: page.getByText("Project streams", { exact: true }) });
  await expect(projectsMenu).toBeVisible();
});

test("filterable menu restores items when query is deleted", async ({
  page,
}) => {
  await page.goto("/components/dropdown_menu");
  await page
    .getByRole("button", { name: "Actions", exact: true })
    .last()
    .click();

  const menu = page.locator('[role="menu"][data-state="open"]').first();
  const input = menu.getByRole("textbox", { name: "Filter actions" });
  await expect(input).toBeFocused();
  await menu.getByRole("menuitem", { name: "Create issue" }).hover();
  await expect(input).toBeFocused();

  await page.keyboard.press("v");
  await expect(
    menu.getByRole("menuitem", { name: "Assign reviewer" }),
  ).toBeVisible();
  await expect(
    menu.getByRole("menuitem", { name: "Create issue" }),
  ).toBeHidden();

  await input.press("Backspace");
  await expect(input).toHaveValue("");
  await expect(
    menu.getByRole("menuitem", { name: "Create issue" }),
  ).toBeVisible();
  await expect(
    menu.getByRole("menuitem", { name: "Assign reviewer" }),
  ).toBeVisible();
  await expect(
    menu.getByRole("menuitem", { name: "Copy review link" }),
  ).toBeVisible();
});
