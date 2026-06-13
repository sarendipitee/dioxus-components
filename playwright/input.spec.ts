import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("/component/?name=input&", {
    timeout: 20 * 60 * 1000,
  }); // Increase timeout to 20 minutes

  await page.getByRole('textbox', { name: 'Enter your name' }).fill('name');
  await expect(page.locator('#input-greeting')).toContainText('Hello, name!');
});

test("shared input shell wires descriptions and sections", async ({ page }) => {
  await page.goto("/component/?name=input&", {
    timeout: 20 * 60 * 1000,
  });

  const nameInput = page.getByRole("textbox", { name: "Name" });
  const nameInputId = await nameInput.getAttribute("id");
  const describedBy = await nameInput.getAttribute("aria-describedby");
  expect(nameInputId).toBeTruthy();
  expect(describedBy).toBeTruthy();
  const describedByIds = describedBy!.split(/\s+/).filter(Boolean);
  expect(describedByIds).toContain(`${nameInputId}-description`);
  await expect(page.locator(`#${nameInputId}-description`)).toHaveText(
    "TextInput keeps the native text-entry API.",
  );

  const searchWrapper = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: page.getByRole("textbox", { name: "Search" }) });
  await expect(searchWrapper.locator("[data-slot='input-left-section']")).toContainText("S");
  await expect(searchWrapper.locator("[data-slot='input-right-section']")).toBeVisible();
  await expect(
    searchWrapper.getByRole("button", { name: "Clear search", exact: true }),
  ).toBeVisible();

  const customShell = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: page.getByText("Custom shell content", { exact: true }) });
  await expect(customShell.locator("[data-slot='input-left-section']")).toContainText("#");
  await expect(customShell.locator("[data-slot='input-control']")).toContainText(
    "Arbitrary input-like content",
  );
});

test("picker inputs wire generated ids and descriptions to controls", async ({ page }) => {
  await page.goto("/component/?name=color_input&", {
    timeout: 20 * 60 * 1000,
  });

  const colorInput = page.getByRole("textbox", { name: "Accent color" });
  const colorId = await colorInput.getAttribute("id");
  const colorDescribedBy = await colorInput.getAttribute("aria-describedby");
  expect(colorId).toBeTruthy();
  expect(colorDescribedBy).toContain(`${colorId}-description`);
  await expect(page.locator(`#${colorId}-description`)).toHaveText(
    "Shared input shell with ColorPicker in the popover.",
  );
  await colorInput.focus();
  await expect(colorInput).toHaveAttribute("aria-expanded", "true");
  await expect(colorInput).toHaveValue("#9B80FF");
  await expect(page.locator("[data-state='open']").getByText("Hue")).toBeVisible();
  await colorInput.fill("#123456");
  await expect(colorInput).toHaveValue("#123456");
  await expect(
    page
      .locator("[data-slot='input-control']")
      .filter({ has: page.getByRole("textbox", { name: "Accent color" }) }),
  ).not.toContainText("Hue");

  await page.goto("/component/?name=date_input&", {
    timeout: 20 * 60 * 1000,
  });

  const dueDateLabel = page.getByText("Due date", { exact: true });
  const dueDateInputId = await dueDateLabel.getAttribute("for");
  expect(dueDateInputId).toBeTruthy();
  const dueDateInput = page.locator(`#${dueDateInputId}`);
  await expect(dueDateInput).toHaveAttribute(
    "aria-describedby",
    `${dueDateInputId}-description`,
  );
  await expect(page.locator(`#${dueDateInputId}-description`)).toHaveText(
    "Single-date input composition.",
  );
  const dueDateShell = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: page.locator(`#${dueDateInputId}`) });
  const dueDateChevron = dueDateShell.getByRole("button", { name: "Show Calendar" });
  await expect(dueDateChevron).toBeVisible();
  await expect(dueDateShell.locator("[data-slot='input-right-section']")).toBeVisible();
  await dueDateChevron.click();
  const dateDialog = page.getByRole("dialog");
  await expect(dateDialog).toContainText("Su");
  await dueDateInput.focus();
  await expect(dateDialog).toContainText("Su");

  const rangeLabel = page.getByText("Booking range", { exact: true });
  const rangeInputId = await rangeLabel.getAttribute("for");
  expect(rangeInputId).toBeTruthy();
  const rangeInput = page.locator(`#${rangeInputId}`);
  await expect(rangeInput).toBeVisible();
  const rangeShell = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: page.locator(`#${rangeInputId}`) });
  await expect(rangeShell.getByRole("button", { name: "Show Calendar" })).toBeVisible();
  await rangeInput.focus();
  await expect(page.getByRole("dialog")).toContainText("Su");

  await page.goto("/component/?name=time_input&", {
    timeout: 20 * 60 * 1000,
  });

  const timeLabel = page.getByText("Start time", { exact: true });
  const timeInputId = await timeLabel.getAttribute("for");
  expect(timeInputId).toBeTruthy();
  const timeInput = page.locator(`#${timeInputId}`);
  await expect(timeInput).toHaveAttribute("aria-describedby", `${timeInputId}-description`);
  await expect(page.locator(`#${timeInputId}-description`)).toHaveText(
    "Shared input chrome with primitive time editing.",
  );
});
