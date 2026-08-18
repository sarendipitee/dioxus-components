import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "/components/switch";

async function loadSwitch(page: Page) {
  await page.goto(URL, { timeout: 30 * 1000, waitUntil: "networkidle" });
}

function controlled(page: Page): Locator {
  return page.getByRole("switch", { name: "Email notifications", exact: true });
}

function uncontrolled(page: Page): Locator {
  return page.getByRole("switch", { name: "Automatic updates", exact: true });
}

test("controlled switch exposes role, name, state, label, description, and attributes", async ({
  page,
}) => {
  await loadSwitch(page);
  const toggle = controlled(page);

  await expect(toggle).toBeVisible();
  await expect(toggle).toHaveRole("switch");
  await expect(toggle).toHaveAttribute("id", "email-notifications-switch");
  await expect(toggle).toHaveAttribute("data-testid", "controlled-switch");
  await expect(toggle).toHaveAttribute("data-fixture", "controlled-switch");
  await expect(toggle).toHaveAttribute("aria-checked", "false");
  await expect(toggle).toHaveAttribute("data-state", "unchecked");
  await expect(toggle).toHaveAttribute(
    "aria-describedby",
    "controlled-switch-description",
  );
  await expect(toggle).toHaveAttribute("value", "enabled");
  await expect(toggle).toHaveAttribute("aria-required", "true");
  const formControl = page.locator(
    'input[type="checkbox"][name="email-notifications"]',
  );
  await expect(formControl).toHaveValue("enabled");
  await expect(formControl).not.toBeChecked();
  await expect(
    page.getByText("Email notifications", { exact: true }),
  ).toBeVisible();
  await expect(
    page.getByText(
      "Receive an email when account activity needs your attention.",
      { exact: true },
    ),
  ).toBeVisible();
});

test("pointer click focuses and toggles controlled switch", async ({
  page,
}) => {
  await loadSwitch(page);
  const toggle = controlled(page);

  await toggle.click();
  await expect(toggle).toBeFocused();
  await expect(toggle).toHaveAttribute("aria-checked", "true");
  await expect(toggle).toHaveAttribute("data-state", "checked");
});

test("Space toggles switch and Enter does not toggle it", async ({ page }) => {
  await loadSwitch(page);
  const toggle = controlled(page);

  await toggle.focus();
  await toggle.press("Space");
  await expect(toggle).toHaveAttribute("aria-checked", "true");
  await toggle.press("Enter");
  await expect(toggle).toHaveAttribute("aria-checked", "true");
  await expect(toggle).toHaveAttribute("data-state", "checked");
});

test("controlled switch reports one callback per toggle and renders callback state", async ({
  page,
}) => {
  await loadSwitch(page);
  const toggle = controlled(page);
  const state = page.getByTestId("controlled-switch-state");
  const count = page.getByTestId("controlled-switch-callback-count");

  await expect(state).toHaveText("unchecked");
  await expect(count).toHaveText("0");
  await toggle.click();
  await expect(state).toHaveText("checked");
  await expect(count).toHaveText("1");
  await toggle.press("Space");
  await expect(state).toHaveText("unchecked");
  await expect(count).toHaveText("2");
});

test("uncontrolled switch honors its default checked state", async ({
  page,
}) => {
  await loadSwitch(page);
  const toggle = uncontrolled(page);

  await expect(toggle).toHaveAttribute("aria-checked", "true");
  await expect(toggle).toHaveAttribute("data-state", "checked");
  await toggle.click();
  await expect(toggle).toHaveAttribute("aria-checked", "false");
});

test("switch form includes checked custom value and omits unchecked switch", async ({
  page,
}) => {
  await loadSwitch(page);
  const form = page.getByTestId("switch-form");
  const toggle = controlled(page);

  const initial = await form.evaluate((element) =>
    Array.from(new FormData(element).entries()).map(([name, value]) => [
      name,
      String(value),
    ]),
  );
  expect(initial).not.toContainEqual(["email-notifications", "enabled"]);
  await toggle.click();
  const checked = await form.evaluate((element) =>
    Array.from(new FormData(element).entries()).map(([name, value]) => [
      name,
      String(value),
    ]),
  );
  expect(checked).toContainEqual(["email-notifications", "enabled"]);
  await toggle.click();
  const unchecked = await form.evaluate((element) =>
    Array.from(new FormData(element).entries()).map(([name, value]) => [
      name,
      String(value),
    ]),
  );
  expect(unchecked).not.toContainEqual(["email-notifications", "enabled"]);
});

test("disabled switches reject interaction and cannot receive focus", async ({
  page,
}) => {
  await loadSwitch(page);
  const unchecked = page.getByTestId("disabled-unchecked-switch");
  const checked = page.getByTestId("disabled-checked-switch");

  await expect(unchecked).toBeDisabled();
  await expect(checked).toBeDisabled();
  await unchecked.click({ force: true });
  await checked.click({ force: true });
  await expect(unchecked).toHaveAttribute("aria-checked", "false");
  await expect(checked).toHaveAttribute("aria-checked", "true");
  await unchecked.focus();
  await expect(unchecked).not.toBeFocused();
  await checked.focus();
  await expect(checked).not.toBeFocused();
});

test("controlled switch reacts to disabled and required controls", async ({
  page,
}) => {
  await loadSwitch(page);
  const toggle = controlled(page);
  const disable = page.getByRole("button", {
    name: "Disable controlled switch",
    exact: true,
  });
  const optional = page.getByRole("button", {
    name: "Make controlled switch optional",
    exact: true,
  });

  await expect(toggle).not.toBeDisabled();
  await expect(toggle).toHaveAttribute("aria-required", "true");
  await disable.click();
  await expect(toggle).toBeDisabled();
  await optional.click();
  await expect(toggle).toHaveAttribute("aria-required", "false");
  await disable.click();
  await expect(toggle).not.toBeDisabled();
});
