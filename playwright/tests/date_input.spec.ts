import { test, expect, type Locator, type Page } from "@playwright/test";

test("date input popover keeps centered horizontal placement while opening", async ({
  page,
}) => {
  const root = await dateInputPage(page);
  const dueDateLabel = root.locator("label").filter({ hasText: "Due date" });
  const dueDateInputId = await dueDateLabel.getAttribute("for");
  expect(dueDateInputId).toBeTruthy();

  const dueDateInput = root.locator(`#${dueDateInputId}`);
  const dueDateShell = root
    .getByTestId("due-date-field")
    .locator(".dx_input_wrapper")
    .first();
  const showCalendar = dueDateShell.locator('[aria-label="Show Calendar"]');

  await expect(dueDateInput).toBeVisible();
  await expect(showCalendar).toBeVisible();

  await showCalendar.click();

  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();
  await expect(dialog).toHaveAttribute("data-state", "open");
  await expect(dialog).toHaveAttribute("data-align", "center");
  await expect(dialog).toHaveAttribute("data-side", "bottom");
  await expect
    .poll(async () => (await dialog.boundingBox())?.y ?? 0)
    .toBeGreaterThan(0);
  const shellBox = await dueDateShell.boundingBox();
  const dialogBox = await dialog.boundingBox();
  expect(shellBox).not.toBeNull();
  expect(dialogBox!.x).toBeGreaterThanOrEqual(0);
  expect(dialogBox!.x - shellBox!.x).toBeLessThanOrEqual(8);
  expect(dialogBox!.y).toBeGreaterThanOrEqual(shellBox!.y + shellBox!.height);
  const dialogId = await dialog.getAttribute("id");
  expect(dialogId).toBeTruthy();
  await expect(showCalendar).toHaveAttribute("aria-controls", dialogId!);
});

const dateInputPage = async (page: Page) => {
  await page.goto("/components/date_input/block#main", { timeout: 30 * 1000 });
  const root = page.locator("#dx-preview-block-root");
  await expect(root).toBeVisible();
  return root;
};

const dueDateWrapper = async (root: Locator) => {
  const label = root.locator("label").filter({ hasText: "Due date" });
  const id = await label.getAttribute("for");
  expect(id).toBeTruthy();
  return root
    .getByTestId("due-date-field")
    .locator("[data-slot='input-wrapper']");
};

test("date input exposes associated description and formatted date segments", async ({
  page,
}) => {
  const root = await dateInputPage(page);
  const label = root.locator("label").filter({ hasText: "Due date" });
  const id = await label.getAttribute("for");
  expect(id).toBeTruthy();
  const control = root.locator(`#${id}`);
  await expect(control).toHaveAttribute(
    "aria-describedby",
    `${id}-description`,
  );
  await expect(root.locator(`#${id}-description`)).toHaveText(
    "Single-date input composition.",
  );
  const wrapper = root
    .getByTestId("due-date-field")
    .locator("[data-slot='input-wrapper']");
  for (const [name, text, now] of [
    ["year", "2024", "2024"],
    ["month", "05", "5"],
    ["day", "15", "15"],
  ]) {
    const segment = wrapper.getByRole("spinbutton", { name });
    await expect(segment).toHaveText(text);
    await expect(segment).toHaveAttribute("aria-valuenow", now);
  }
});

test("calendar selection updates the controlled date status", async ({
  page,
}) => {
  const root = await dateInputPage(page);
  const wrapper = await dueDateWrapper(root);
  await wrapper.locator('[aria-label="Show Calendar"]').click();
  const dialog = page.locator('[role="dialog"][data-state="open"]');
  const date = dialog.getByRole("button", { name: "Thursday, May 16, 2024" });
  await wrapper.getByRole("spinbutton", { name: "year" }).focus();
  await expect(dialog).toBeVisible();
  await expect(date).toBeVisible();
  await date.click();
  await expect(root.getByTestId("due-date-value")).toHaveText("2024-05-16");
});

test("required error, disabled, and read-only date inputs expose state semantics", async ({
  page,
}) => {
  const root = await dateInputPage(page);
  const required = root.locator("label").filter({ hasText: "Required date" });
  const requiredId = await required.getAttribute("for");
  expect(requiredId).toBeTruthy();
  const requiredControl = root.locator(`#${requiredId}`);
  await expect(requiredControl).toHaveAttribute("aria-invalid", "true");
  await expect(requiredControl).toHaveAttribute(
    "aria-describedby",
    new RegExp(`${requiredId}-error`),
  );
  const disabled = root
    .getByTestId("disabled-date-field")
    .locator("[data-slot='input-wrapper']");
  await expect(disabled.locator('[aria-label="Show Calendar"]')).toBeDisabled();
  for (const segment of await disabled.getByRole("spinbutton").all()) {
    await expect(segment).toHaveAttribute("aria-disabled", "true");
    await expect(segment).toHaveAttribute("contenteditable", "false");
    const before = await segment.textContent();
    await segment.focus();
    await page.keyboard.press("ArrowUp");
    await expect(segment).toHaveText(before ?? "");
  }
  const readOnly = root
    .getByTestId("readonly-date-field")
    .locator("[data-slot='input-wrapper']");
  for (const segment of await readOnly.getByRole("spinbutton").all()) {
    await expect(segment).toHaveAttribute("aria-readonly", "true");
    await expect(segment).toHaveAttribute("contenteditable", "false");
    const before = await segment.textContent();
    await segment.focus();
    await page.keyboard.press("ArrowUp");
    await expect(segment).toHaveText(before ?? "");
  }
});
