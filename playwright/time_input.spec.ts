import { test, expect, type Page } from "@playwright/test";

async function fixture(page: Page, id: string) {
  await page.goto("/components/time_input", { timeout: 30 * 1000 });
  return page.getByTestId(id);
}

test("segments expose names and valid ARIA ranges", async ({ page }) => {
  const root = await fixture(page, "time-input-default");
  const hour = root.getByRole("spinbutton", { name: "hour" });
  const minute = root.getByRole("spinbutton", { name: "minute" });

  await expect(hour).toHaveAttribute("aria-valuemin", "0");
  await expect(hour).toHaveAttribute("aria-valuemax", "23");
  await expect(hour).toHaveAttribute("aria-valuenow", "14");
  await expect(minute).toHaveAttribute("aria-valuemin", "0");
  await expect(minute).toHaveAttribute("aria-valuemax", "59");
  await expect(minute).toHaveAttribute("aria-valuenow", "45");
  await expect(root.getByRole("spinbutton", { name: "second" })).toHaveCount(0);
});

test("segments support digits, arrows, Home, End, and Tab", async ({ page }) => {
  const root = await fixture(page, "time-input-default");
  const hour = root.getByRole("spinbutton", { name: "hour" });
  const minute = root.getByRole("spinbutton", { name: "minute" });

  await hour.focus();
  await page.keyboard.type("09");
  await expect(hour).toHaveText("09");
  await page.keyboard.press("ArrowUp");
  await expect(hour).toHaveText("10");
  await page.keyboard.press("ArrowDown");
  await expect(hour).toHaveText("09");
  await page.keyboard.press("Home");
  await expect(hour).toHaveText("00");
  await page.keyboard.press("End");
  await expect(hour).toHaveText("23");
  await page.keyboard.press("Tab");
  await expect(minute).toBeFocused();
});

test("12-hour mode exposes seconds and keyboard-toggleable AM/PM", async ({ page }) => {
  const root = await fixture(page, "time-input-12h-seconds");
  const hour = root.getByRole("spinbutton", { name: "hour" });
  const second = root.getByRole("spinbutton", { name: "second" });
  const period = root.getByRole("spinbutton", { name: "AM/PM" });

  await expect(hour).toHaveAttribute("aria-valuemin", "1");
  await expect(hour).toHaveAttribute("aria-valuemax", "12");
  await expect(second).toHaveText("30");
  await expect(period).toHaveText("PM");
  await period.focus();
  await page.keyboard.press("ArrowDown");
  await expect(period).toHaveText("AM");
  await page.keyboard.type("p");
  await expect(period).toHaveText("PM");
});

test("bounded input exposes and respects minimum and maximum hours", async ({ page }) => {
  const root = await fixture(page, "time-input-bounded");
  const hour = root.getByRole("spinbutton", { name: "hour" });

  await expect(hour).toHaveAttribute("aria-valuemin", "9");
  await expect(hour).toHaveAttribute("aria-valuemax", "17");
  await hour.focus();
  await page.keyboard.press("Home");
  await expect(hour).toHaveText("09");
  await page.keyboard.press("End");
  await expect(hour).toHaveText("17");
  await page.keyboard.press("ArrowUp");
  await expect(hour).toHaveText("17");
});

test("controlled edits update visible value and callback count", async ({ page }) => {
  const root = await fixture(page, "time-input-default");
  const minute = root.getByRole("spinbutton", { name: "minute" });

  await minute.focus();
  await page.keyboard.press("ArrowUp");
  await expect(page.getByTestId("time-input-value")).toContainText("14:46");
  await expect(page.getByTestId("time-input-change-count")).toHaveText("Changes: 1");
});

test("disabled and read-only segments do not mutate", async ({ page }) => {
  const disabled = await fixture(page, "time-input-disabled");
  const disabledHour = disabled.getByRole("spinbutton", { name: "hour" });
  await expect(disabledHour).toHaveAttribute("aria-disabled", "true");
  await disabledHour.focus();
  await page.keyboard.press("ArrowUp");
  await expect(disabledHour).toHaveText("10");

  const readonly = page.getByTestId("time-input-readonly");
  const readonlyHour = readonly.getByRole("spinbutton", { name: "hour" });
  await expect(readonlyHour).toHaveAttribute("aria-readonly", "true");
  await readonlyHour.focus();
  await page.keyboard.press("ArrowUp");
  await expect(readonlyHour).toHaveText("11");
});

test("required invalid field is labelled and described by helper and error", async ({ page }) => {
  const root = await fixture(page, "time-input-validation");
  const hour = root.getByRole("spinbutton", { name: "hour" });

  await expect(hour).toHaveAttribute("aria-required", "true");
  await expect(hour).toHaveAttribute("aria-invalid", "true");
  const describedBy = (await hour.getAttribute("aria-describedby")) ?? "";
  const ids = describedBy.split(/\s+/).filter(Boolean);
  expect(ids).toHaveLength(2);
  await expect(page.locator(`#${ids[0]}`)).toHaveText("Choose a time for the appointment.");
  await expect(page.locator(`#${ids[1]}`)).toHaveText("A time is required.");
  await expect(root.getByText("Required time", { exact: true })).toBeVisible();
});

test("clear button clears the controlled value", async ({ page }) => {
  const root = await fixture(page, "time-input-clearable");
  await root.getByRole("button", { name: "Clear" }).click();
  await expect(page.getByTestId("time-input-clear-value")).toHaveText("Value: None");
});

test("global attributes forward and react on the primitive root", async ({ page }) => {
  const fixtureRoot = await fixture(page, "time-input-reactive");
  const picker = fixtureRoot.locator("[data-state='off']");
  await expect(picker).toHaveClass(/reactive-off/);
  await page.getByTestId("time-input-reactive-toggle").click();
  await expect(fixtureRoot.locator("[data-state='on']")).toHaveClass(/reactive-on/);
});