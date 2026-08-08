import { test, expect, type Page } from "@playwright/test";

const datePickerFrame = (page: Page) =>
  page.locator("#component-preview-frame").first();

const showMay2026 = async (frame: ReturnType<typeof datePickerFrame>) => {
  await frame.getByRole("combobox", { name: "Year" }).selectOption("2026");
  await frame.getByRole("combobox", { name: "Month" }).selectOption("5");
};

test("exposes the inline calendar labels and ARIA semantics", async ({ page }) => {
  await page.goto("/components/date_picker/block#main", { timeout: 30 * 1000 });

  const frame = datePickerFrame(page);
  await expect(frame).toBeVisible({ timeout: 30 * 1000 });
  await expect(frame.getByRole("group", { name: "Date" })).toBeVisible();

  const calendar = frame.getByRole("application", { name: "Calendar" });
  await expect(calendar).toBeVisible();
  await expect(calendar.getByRole("combobox", { name: "Month" })).toBeVisible();
  await expect(calendar.getByRole("combobox", { name: "Year" })).toBeVisible();
  await showMay2026(frame);
  await expect(calendar.getByRole("button", { name: "Friday, May 15, 2026" }))
    .toHaveAttribute("data-selected", "false");
});

test("selects and deselects a date in the controlled picker", async ({ page }) => {
  await page.goto("/components/date_picker/block#main", { timeout: 30 * 1000 });

  const frame = datePickerFrame(page);
  await showMay2026(frame);
  const day = frame.getByRole("button", { name: "Friday, May 15, 2026" });

  await day.click();
  await expect(day).toHaveAttribute("data-selected", "true");
  await day.click();
  await expect(day).toHaveAttribute("data-selected", "false");
});

test("activates a date from the keyboard and moves focus by day", async ({ page }) => {
  await page.goto("/components/date_picker/block#main", { timeout: 30 * 1000 });

  const frame = datePickerFrame(page);
  await showMay2026(frame);
  const day = frame.getByRole("button", { name: "Monday, May 18, 2026" });
  const nextDay = frame.getByRole("button", { name: "Tuesday, May 19, 2026" });

  await day.focus();
  await expect(day).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(day).toHaveAttribute("data-selected", "true");
  await page.keyboard.press("ArrowRight");
  await expect(nextDay).toBeFocused();
});

test("keeps unavailable dates disabled and unselectable", async ({ page }) => {
  await page.goto("/components/date_picker/block#unavailable_dates", {
    timeout: 30 * 1000,
  });

  const frame = datePickerFrame(page);
  await showMay2026(frame);

  const unavailable = frame.getByRole("button", { name: "Friday, May 15, 2026" });
  await expect(unavailable).toHaveAttribute("data-disabled", "true");
  await expect(unavailable).toHaveAttribute("data-unavailable", "true");
  await expect(unavailable).toHaveAttribute("data-selected", "false");
  await unavailable.click({ force: true });
  await expect(unavailable).toHaveAttribute("data-selected", "false");
});

test("renders two calendar grids in the multi-month picker", async ({ page }) => {
  await page.goto("/components/date_picker/block#multi_month", {
    timeout: 30 * 1000,
  });

  const frame = datePickerFrame(page);
  await expect(frame).toBeVisible({ timeout: 30 * 1000 });
  await expect(frame.getByRole("application", { name: "Calendar" })).toHaveCount(1);
  await expect(frame.getByRole("grid")).toHaveCount(2);
});
