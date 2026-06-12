import { test, expect } from "@playwright/test";

test("date input popover keeps centered horizontal placement while opening", async ({
  page,
}) => {
  await page.goto("/component/?name=date_input&", {
    timeout: 20 * 60 * 1000,
  });

  const dueDateLabel = page.getByText("Due date", { exact: true });
  const dueDateInputId = await dueDateLabel.getAttribute("for");
  expect(dueDateInputId).toBeTruthy();

  const dueDateInput = page.locator(`#${dueDateInputId}`);
  const dueDateShell = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: dueDateInput });
  const showCalendar = dueDateShell.getByRole("button", {
    name: "Show Calendar",
  });

  await expect(dueDateInput).toBeVisible();
  await expect(showCalendar).toBeVisible();

  await showCalendar.click();

  const dialog = page.getByRole("dialog", { name: "Show Calendar" });
  await expect(dialog).toBeVisible();
  await expect
    .poll(async () => (await dialog.boundingBox())?.x ?? -1)
    .toBeGreaterThan(0);

  const openingBox = await dialog.boundingBox();
  expect(openingBox).toBeTruthy();

  await page.waitForTimeout(350);

  const settledBox = await dialog.boundingBox();
  expect(settledBox).toBeTruthy();

  expect(Math.abs(settledBox!.x - openingBox!.x)).toBeLessThanOrEqual(4);
});
