import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "/components/checkbox";

async function loadCheckboxes(page: Page) {
  await page.goto(URL, { timeout: 30 * 1000, waitUntil: "networkidle" });
}

function checkbox(page: Page, name: string): Locator {
  return page.getByRole("checkbox", { name, exact: true }).first();
}

test("checkbox exposes its accessible name, role, and initial state", async ({ page }) => {
  await loadCheckboxes(page);
  const accept = checkbox(page, "Accept terms and conditions");

  await expect(accept).toBeVisible();
  await expect(accept).toHaveRole("checkbox");
  await expect(accept).toHaveAttribute("aria-checked", "false");
  await expect(accept).toHaveAttribute("data-state", "unchecked");
});

test("click toggles a checkbox and leaves it focused", async ({ page }) => {
  await loadCheckboxes(page);
  const accept = checkbox(page, "Accept terms and conditions");

  await accept.click();
  await expect(accept).toHaveAttribute("aria-checked", "true");
  await expect(accept).toHaveAttribute("data-state", "checked");
  await expect(accept).toBeFocused();
});

test("Space toggles and Enter does not toggle a checkbox", async ({ page }) => {
  await loadCheckboxes(page);
  const accept = checkbox(page, "Accept terms and conditions");

  await accept.focus();
  await accept.press("Space");
  await expect(accept).toHaveAttribute("aria-checked", "true");
  await accept.press("Enter");
  await expect(accept).toHaveAttribute("aria-checked", "true");
  await expect(accept).toHaveAttribute("data-state", "checked");
});

test("controlled indeterminate checkbox transitions through click and Space", async ({ page }) => {
  await loadCheckboxes(page);
  const selectAll = checkbox(page, "Select all visible rows");

  await expect(selectAll).toHaveAttribute("aria-checked", "mixed");
  await expect(selectAll).toHaveAttribute("data-state", "indeterminate");
  await selectAll.click();
  await expect(selectAll).toHaveAttribute("aria-checked", "false");
  await expect(selectAll).toHaveAttribute("data-state", "unchecked");
  await selectAll.press("Space");
  await expect(selectAll).toHaveAttribute("aria-checked", "true");
  await expect(selectAll).toHaveAttribute("data-state", "checked");
});

test("disabled unchecked and checked checkboxes reject interaction and focus", async ({ page }) => {
  await loadCheckboxes(page);
  const unchecked = checkbox(page, "Include archived projects");
  const checked = checkbox(page, "Enforce organization policy");

  await expect(unchecked).toBeDisabled();
  await expect(checked).toBeDisabled();
  await expect(unchecked).toHaveAttribute("aria-checked", "false");
  await expect(checked).toHaveAttribute("aria-checked", "true");
  await unchecked.click({ force: true });
  await checked.click({ force: true });
  await expect(unchecked).toHaveAttribute("data-state", "unchecked");
  await expect(checked).toHaveAttribute("data-state", "checked");
  await unchecked.focus();
  await expect(unchecked).not.toBeFocused();
  await checked.focus();
  await expect(checked).not.toBeFocused();
});

test("read-only checked checkbox remains checked while focusable", async ({ page }) => {
  await loadCheckboxes(page);
  const managed = checkbox(page, "Managed setting");

  await expect(managed).toHaveAttribute("aria-checked", "true");
  await expect(managed).toHaveAttribute("data-state", "checked");
  await expect(managed).toHaveAttribute("aria-readonly", "true");
  await expect(managed).toHaveAttribute("data-readonly", "true");
  await managed.click();
  await expect(managed).toBeFocused();
  await expect(managed).toHaveAttribute("aria-checked", "true");
  await managed.press("Space");
  await expect(managed).toHaveAttribute("aria-checked", "true");
  await expect(managed).toHaveAttribute("data-state", "checked");
});

test("checkbox form data omits unchecked and includes checked values", async ({ page }) => {
  await loadCheckboxes(page);
  const form = page.locator('form[data-testid="checkbox-form"]');
  const accept = checkbox(page, "Accept terms and conditions");

  const initialEntries = await form.evaluate((element) =>
    Array.from(new FormData(element).entries()).map(([name, value]) => [name, String(value)]),
  );
  expect(initialEntries).not.toContainEqual(["tos-check", "accepted"]);
  expect(initialEntries).toContainEqual(["managed-setting", "locked"]);

  await accept.click();
  const checkedEntries = await form.evaluate((element) =>
    Array.from(new FormData(element).entries()).map(([name, value]) => [name, String(value)]),
  );
  expect(checkedEntries).toContainEqual(["tos-check", "accepted"]);
  expect(checkedEntries).toContainEqual(["managed-setting", "locked"]);
});
