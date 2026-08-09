import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("/components/input", {
    timeout: 30 * 1000,
  });

  const shellInput = page.getByPlaceholder("release-notes").first();
  await shellInput.fill("customer-portal");
  await expect(page.locator("#input-shell-value")).toContainText(
    "Shell value: customer-portal",
  );
});

test("shared input shell wires descriptions and sections", async ({ page }) => {
  await page.goto("/components/input", {
    timeout: 30 * 1000,
  });

  const labeledShellInput = page.getByPlaceholder("project-slug").first();
  const labeledShell = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: labeledShellInput });
  await expect(
    labeledShell.locator("[data-slot='input-left-section']"),
  ).toContainText("#");
  await expect(
    page.getByText("Labeled shell", { exact: true }).first(),
  ).toBeVisible();
  await expect(
    page
      .getByText("InputBase adds wrapper metadata around the same shell.", {
        exact: true,
      })
      .first(),
  ).toBeVisible();

  const searchInput = page.getByPlaceholder("Search a route").first();
  const clearButton = page.getByRole("button", {
    name: "Clear query",
    exact: true,
  });
  await expect(clearButton).toBeVisible();
  await clearButton.click();
  await expect(searchInput).toHaveValue("");

  const environmentInput = page
    .getByRole("textbox", { name: "Environment" })
    .first();
  const environmentInputId = await environmentInput.getAttribute("id");
  const environmentDescribedBy =
    await environmentInput.getAttribute("aria-describedby");
  expect(environmentInputId).toBeTruthy();
  expect(environmentDescribedBy).toBeTruthy();
  const environmentDescribedByIds = environmentDescribedBy!
    .split(/\s+/)
    .filter(Boolean);
  expect(environmentDescribedByIds).toContain(
    `${environmentInputId}-description`,
  );
  expect(environmentDescribedByIds).toContain(`${environmentInputId}-error`);
  await expect(page.locator(`#${environmentInputId}-description`)).toHaveText(
    "InputBase provides ids, described-by wiring, and shell state.",
  );
  await expect(page.locator(`#${environmentInputId}-error`)).toHaveText(
    "Only lowercase letters are allowed.",
  );
  await expect(
    page
      .locator("[data-slot='input-wrapper']")
      .filter({ has: environmentInput })
      .locator("[data-slot='input-left-section']"),
  ).toContainText("env");
});

test("picker inputs wire generated ids and descriptions to controls", async ({
  page,
}) => {
  await page.goto("/components/color_input", {
    timeout: 30 * 1000,
  });

  const colorLabel = page.getByText("Accent color", { exact: true });
  const colorInputId = await colorLabel.getAttribute("for");
  expect(colorInputId).toBeTruthy();
  const colorInput = page.locator(`#${colorInputId}`);
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
  await expect(
    page.locator("[data-state='open']").getByText("Hue"),
  ).toBeVisible();
  await colorInput.fill("#123456");
  await expect(colorInput).toHaveValue("#123456");
  await expect(
    page.locator("[data-slot='input-control']").filter({ has: colorInput }),
  ).not.toContainText("Hue");

  await page.goto("/components/date_input", {
    timeout: 30 * 1000,
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
  const dueDateChevron = dueDateShell.locator('[aria-label="Show Calendar"]');
  await expect(dueDateChevron).toBeVisible();
  await expect(
    dueDateShell.locator("[data-slot='input-right-section']"),
  ).toBeVisible();
  await dueDateChevron.click();
  const dateDialog = page.locator('[role="dialog"][data-state="open"]');
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
  await expect(
    rangeShell.locator('[aria-label="Show Calendar"]'),
  ).toBeVisible();
  await rangeInput.focus();
  await expect(
    page.locator('[role="dialog"][data-state="open"]').last(),
  ).toContainText("Su");

  await page.goto("/components/time_input", {
    timeout: 30 * 1000,
  });

  const timeLabel = page.getByText("Start time", { exact: true });
  const timeInputId = await timeLabel.getAttribute("for");
  await expect(page.locator(`#${timeInputId}-description`)).toHaveText(
    "Enter the start time.",
  );
});

test("text input controls expose labels, controlled output, and native attributes", async ({
  page,
}) => {
  await page.goto("/components/text_input", { timeout: 30 * 1000 });

  const email = page
    .getByRole("textbox", { name: "Email", exact: true })
    .first();
  await expect(email).toBeVisible();
  await expect(email).toHaveAttribute("name", "email");
  await expect(email).toHaveAttribute("data-input-purpose", "account-email");

  await email.fill("customer@example.com");
  await expect(page.locator("#text-input-value").first()).toContainText(
    "customer@example.com",
  );
});

test("text input distinguishes required and asterisk-only fields", async ({
  page,
}) => {
  await page.goto("/components/text_input", { timeout: 30 * 1000 });

  const required = page
    .getByRole("textbox", { name: "Display name", exact: true })
    .first();
  const requiredShell = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: required });
  await expect(required).toHaveAttribute("required", "true");
  await expect(requiredShell.getByText("*", { exact: true })).toBeVisible();

  const asteriskOnly = page
    .getByRole("textbox", { name: "Organization", exact: true })
    .first();
  const asteriskShell = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: asteriskOnly });
  await expect(asteriskOnly).not.toHaveAttribute("required");
  await expect(asteriskShell.getByText("*", { exact: true })).toBeVisible();
});

test("text input state props reach the native control and shell", async ({
  page,
}) => {
  await page.goto("/components/text_input", { timeout: 30 * 1000 });

  const disabled = page
    .getByRole("textbox", { name: "Disabled field", exact: true })
    .first();
  await expect(disabled).toBeDisabled();
  await expect(disabled).not.toBeEditable();

  const readOnly = page
    .getByRole("textbox", { name: "Read-only field", exact: true })
    .first();
  await expect(readOnly).toHaveAttribute("readonly", "true");
  await expect(readOnly).toHaveValue(/.+/);

  const loading = page
    .getByRole("textbox", { name: "Loading field", exact: true })
    .first();
  const loadingShell = page
    .locator("[data-slot='input']")
    .filter({ has: loading });
  await expect(loadingShell).toHaveAttribute("aria-busy", "true");
  await expect(loadingShell).toHaveAttribute("data-loading", "true");
  await expect(
    loadingShell.locator("[data-slot='input-spinner']"),
  ).toBeVisible();
});
