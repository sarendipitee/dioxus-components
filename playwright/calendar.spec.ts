import { test, expect, type Page } from "@playwright/test";

const calendarFrame = (page: Page) =>
  page.locator("#component-preview-frame, #dx-preview-block-root").first();

test("shows fixed May 2026 calendar without native selectors", async ({ page }) => {
  await page.goto("/components/calendar", { timeout: 30 * 1000 });

  const calendar = calendarFrame(page);
  await expect(calendar).toBeVisible({ timeout: 30 * 1000 });
  await expect(calendar.locator(".dx_calendar select")).toHaveCount(0);
  await expect(calendar.getByRole("button", { name: "Month May" })).toBeVisible();
  await expect(calendar.getByRole("button", { name: "Year 2026" })).toBeVisible();

  await calendar.getByRole("button", { name: "Month May" }).click();
  await expect(page.getByRole("menuitemradio", { name: "June" })).toBeVisible();

  const day = calendar.getByRole("button", { name: "Friday, May 15, 2026" });
  await expect(day).toHaveAttribute("data-selected", "false");
  await day.click();
  await expect(day).toHaveAttribute("data-selected", "true");
});
test("year dropdown panel is constrained and scrollable", async ({ page }) => {
  await page.goto("/components/calendar", { timeout: 30 * 1000 });

  const calendar = calendarFrame(page);
  await expect(calendar).toBeVisible({ timeout: 30 * 1000 });

  await calendar.getByRole("button", { name: "Year 2026" }).click();
  const yearMenu = page
    .getByRole("menu")
    .filter({ has: page.getByRole("menuitemradio", { name: "1995" }) })
    .last();
  await expect(yearMenu).toBeVisible();

  const panelMetrics = await yearMenu.evaluate((element) => {
    const styles = getComputedStyle(element);
    return {
      maxHeight: styles.maxHeight,
      overflowY: styles.overflowY,
      scrollHeight: element.scrollHeight,
      clientHeight: element.clientHeight,
    };
  });

  expect(panelMetrics.maxHeight).toBe("320px");
  expect(panelMetrics.overflowY).toBe("auto");
  expect(panelMetrics.scrollHeight).toBeGreaterThan(panelMetrics.clientHeight);
});

test("hovering year items does not cross menu signal scopes", async ({ page }) => {
  const scopeWarnings: string[] = [];
  page.on("console", (message) => {
    if (message.text().includes("Copy Value created in ScopeId")) {
      scopeWarnings.push(message.text());
    }
  });

  await page.goto("/components/calendar", { timeout: 30 * 1000 });

  const calendar = calendarFrame(page);
  await expect(calendar).toBeVisible({ timeout: 30 * 1000 });
  await calendar.getByRole("button", { name: "Year 2026" }).click();

  const year = page.getByRole("menuitemradio", { name: "2025", exact: true });
  await expect(year).toBeVisible();
  await year.hover();
  await expect(year).toHaveAttribute("data-focused", "true");
  expect(scopeWarnings).toEqual([]);
});

test("deselects the selected day when activated again", async ({ page }) => {
  await page.goto("/components/calendar", { timeout: 30 * 1000 });

  const calendar = calendarFrame(page);
  const day = calendar.getByRole("button", { name: "Wednesday, May 20, 2026" });

  await day.click();
  await expect(day).toHaveAttribute("data-selected", "true");
  await day.click();
  await expect(day).toHaveAttribute("data-selected", "false");
});

test("selects a day with keyboard activation", async ({ page }) => {
  await page.goto("/components/calendar", { timeout: 30 * 1000 });

  const calendar = calendarFrame(page);
  const day = calendar.getByRole("button", { name: "Monday, May 25, 2026" });

  await day.focus();
  await expect(day).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(day).toHaveAttribute("data-selected", "true");
});

test("navigates months with accessible controls", async ({ page }) => {
  await page.goto("/components/calendar", { timeout: 30 * 1000 });

  const calendar = calendarFrame(page);
  const application = calendar.getByRole("application", { name: "Calendar" });
  const month = calendar.getByRole("combobox", { name: "Month" });

  await expect(application).toBeVisible({ timeout: 30 * 1000 });
  await expect(month).toHaveValue("5");
  await calendar.getByRole("button", { name: "Previous month" }).click();
  await expect(month).toHaveValue("4");
  await calendar.getByRole("button", { name: "Next month" }).click();
  await expect(month).toHaveValue("5");
});

test("shift + arrow keys navigation", async ({ page }) => {
  await page.goto("/components/calendar", {
    timeout: 30 * 1000,
  });

  const calendar = page.locator("#component-preview-frame").first();
  const monthSelect = calendar.locator("select").first();
  const yearSelect = calendar.locator("select").nth(1);

  // Assert the calendar is displayed
  await expect(calendar).toBeVisible({ timeout: 30000 });

  // Get the initial month and year
  const initialMonth = await monthSelect.inputValue();
  const initialYear = await yearSelect.inputValue();
  const initialYearNumber = parseInt(initialYear, 10);
  const initialMonthNumber = parseInt(initialMonth, 10);

  // Move focus to the calendar
  const firstDay = calendar.locator('[data-month="current"]').first();
  await firstDay.focus();

  // Test Shift + ArrowDown - should move forward by one month
  await page.keyboard.press("Shift+ArrowDown");

  let currentMonth = await monthSelect.inputValue();
  let currentYear = await yearSelect.inputValue();
  let expectedMonth = initialMonthNumber === 12 ? 1 : initialMonthNumber + 1;
  let expectedYear =
    initialMonthNumber === 12 ? initialYearNumber + 1 : initialYearNumber;

  expect(parseInt(currentMonth, 10)).toBe(expectedMonth);
  expect(parseInt(currentYear, 10)).toBe(expectedYear);

  // Test Shift + ArrowUp - should move back to the initial month
  await page.keyboard.press("Shift+ArrowUp");

  currentMonth = await monthSelect.inputValue();
  currentYear = await yearSelect.inputValue();
  expect(currentMonth).toBe(initialMonth);
  expect(currentYear).toBe(initialYear);
});

async function testArrowKeyNavigation(
  page: any,
  arrowKey: "ArrowRight" | "ArrowLeft",
  startPosition: "first" | "last",
  expectedOrder: "ascending" | "descending",
) {
  await page.goto("/components/calendar", {
    timeout: 30 * 1000,
  });

  const calendar = page.locator("#component-preview-frame").first();
  const monthSelect = calendar.locator("select").first();
  const yearSelect = calendar.locator("select").nth(1);

  // Assert the calendar is displayed
  await expect(calendar).toBeVisible({ timeout: 30000 });

  // Get the current month and year to calculate days in month
  const currentMonthValue = await monthSelect.inputValue();
  const currentYearValue = await yearSelect.inputValue();
  const monthNumber = parseInt(currentMonthValue, 10);
  const yearNumber = parseInt(currentYearValue, 10);

  // Calculate the number of days in the current month
  const daysInMonth = new Date(yearNumber, monthNumber, 0).getDate();

  // Move focus to the starting day of the current month
  const startDay = calendar.locator('[data-month="current"]')[startPosition]();
  await startDay.focus();

  // Get the focused day selector
  const focusedDay = calendar.locator('[data-month="current"]:focus');

  // Array to track all days visited
  const daysVisited = [];

  // Get the starting day number
  let dayText = await focusedDay.first().textContent();
  let dayNumber = parseInt(dayText || "", 10);
  daysVisited.push(dayNumber);

  // Press arrow key to navigate through all remaining days of the month
  for (let i = 1; i < daysInMonth; i++) {
    await page.keyboard.press(arrowKey);

    // Get the new focused day
    dayText = await focusedDay.first().textContent();
    dayNumber = parseInt(dayText || "", 10);
    daysVisited.push(dayNumber);
  }

  // Assert that we visited the correct number of days
  expect(daysVisited.length).toBe(daysInMonth);

  // Sort the days visited to check we got all days from 1 to daysInMonth
  const sortedDays = [...daysVisited].sort((a, b) => a - b);

  // Create the expected array [1, 2, 3, ..., daysInMonth]
  const expectedDays = Array.from({ length: daysInMonth }, (_, i) => i + 1);

  // Assert that we visited every day exactly once
  expect(sortedDays).toEqual(expectedDays);

  // Verify we traversed in the expected order
  if (expectedOrder === "ascending") {
    expect(daysVisited).toEqual(expectedDays);
  } else {
    const expectedReverseDays = Array.from(
      { length: daysInMonth },
      (_, i) => daysInMonth - i,
    );
    expect(daysVisited).toEqual(expectedReverseDays);
  }
}

test("right arrow key navigates through all days of the month", async ({
  page,
}) => {
  await testArrowKeyNavigation(page, "ArrowRight", "first", "ascending");
});

test("left arrow key navigates through all days of the month in reverse", async ({
  page,
}) => {
  await testArrowKeyNavigation(page, "ArrowLeft", "last", "descending");
});
