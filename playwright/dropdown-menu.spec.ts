import { test, expect } from "@playwright/test";

test("dropdown checkbox and radio keyboard state updates keep menu open", async ({
  page,
}) => {
  await page.goto("/components/dropdown_menu");
  const demo = page.locator(".dx-component-section").first();
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
