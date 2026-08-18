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


test("12-hour mode exposes seconds and keyboard-toggleable AM/PM", async ({
  page,
}) => {
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

test("controlled edits update visible value and callback count", async ({
  page,
}) => {
  const root = await fixture(page, "time-input-default");
  const minute = root.getByRole("spinbutton", { name: "minute" });

  await minute.focus();
  await page.keyboard.press("ArrowUp");
  await expect(page.getByTestId("time-input-value")).toContainText("14:46");
  await expect(page.getByTestId("time-input-change-count")).toHaveText(
    "Changes: 1",
  );
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

test("clear button clears the controlled value", async ({ page }) => {
  const root = await fixture(page, "time-input-clearable");
  await root.getByRole("button", { name: "Clear" }).click();
  await expect(page.getByTestId("time-input-clear-value")).toHaveText(
    "Value: None",
  );
});

test("global attributes forward and react on the primitive root", async ({
  page,
}) => {
  const fixtureRoot = await fixture(page, "time-input-reactive");
  const picker = fixtureRoot.locator("[data-state='off']");
  await expect(picker).toHaveClass(/reactive-off/);
  await page.getByTestId("time-input-reactive-toggle").click();
  await expect(fixtureRoot.locator("[data-state='on']")).toHaveClass(
    /reactive-on/,
  );
});
