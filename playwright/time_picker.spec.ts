import { test, expect, type Page } from "@playwright/test";

async function fixture(page: Page, id: string) {
  const route = id.includes("12-hour")
    ? "seconds_12_hour"
    : id.includes("duration")
      ? "duration"
      : id.includes("presets")
        ? "presets"
        : id.includes("clearable")
          ? "clearable"
          : "main";
  await page.goto(`/components/time_picker/${route}`, { timeout: 30 * 1000 });
  const root = page.getByTestId(id);
  await expect(root).toBeVisible({ timeout: 30 * 1000 });
  return root;
}

test("clock columns expose named listboxes and selected options", async ({ page }) => {
  const root = await fixture(page, "time-picker-bounded-steps");
  const hour = root.getByRole("listbox", { name: "Hour" });
  const minute = root.getByRole("listbox", { name: "Minute" });
  await expect(hour).toBeVisible();
  await expect(minute).toBeVisible();
  await expect(hour.getByRole("option")).toHaveCount(24);
  await expect(minute.getByRole("option")).toHaveCount(4);
  await expect(hour.getByRole("option", { name: "09" })).toHaveAttribute("aria-selected", "true");
  await expect(minute.getByRole("option", { name: "30" })).toHaveAttribute("aria-selected", "true");
});

test("selecting a 24-hour option updates controlled state", async ({ page }) => {
  const root = await fixture(page, "time-picker-bounded-steps");
  await root.getByRole("option", { name: "10" }).click();
  await expect(root.getByRole("option", { name: "10" })).toHaveAttribute("aria-selected", "true");
  await expect(root.getByTestId("time-picker-bounded-steps-state")).toContainText("10:30");
});

test("12-hour seconds picker exposes period and second columns", async ({ page }) => {
  const root = await fixture(page, "time-picker-12-hour-seconds");
  await expect(root.getByRole("listbox", { name: "Hour" })).toBeVisible();
  await expect(root.getByRole("listbox", { name: "Minute" })).toBeVisible();
  await expect(root.getByRole("listbox", { name: "Second" })).toBeVisible();
  const period = root.getByRole("listbox", { name: "AM/PM" });
  await expect(period).toBeVisible();
  await expect(period.getByRole("option", { name: "morning" })).toHaveAttribute("aria-selected", "true");
  await period.getByRole("option", { name: "afternoon" }).click();
  await expect(period.getByRole("option", { name: "afternoon" })).toHaveAttribute("aria-selected", "true");
  await expect(root.getByTestId("time-picker-12-hour-seconds-state")).toContainText("14:30:45");
});


test("duration picker keeps hours segmented and emits duration state", async ({ page }) => {
  const root = await fixture(page, "time-picker-duration");
  await expect(root.getByRole("spinbutton", { name: "hour" })).toBeVisible();
  await expect(root.getByRole("spinbutton", { name: "minute" })).toBeVisible();
  await expect(root.getByRole("spinbutton", { name: "second" })).toBeVisible();
  await expect(root.getByRole("spinbutton", { name: "hour" })).toHaveText("036");
  await expect(root.getByTestId("time-picker-duration-state")).toContainText("36:15:30");
});

test("presets select a manual preset", async ({ page }) => {
  const root = await fixture(page, "time-picker-presets-manual");
  await root.getByRole("button", { name: "9:00 AM" }).click();
  await expect(root.getByTestId("time-picker-presets-manual-state")).toContainText("09:00");
});

test("clearable picker clears its controlled value", async ({ page }) => {
  const root = await fixture(page, "time-picker-clearable");
  await root.getByRole("button", { name: "Clear" }).click();
  await expect(root.getByTestId("time-picker-clearable-state")).toContainText("none");
});

test("minimum, maximum, and steps disable invalid options", async ({ page }) => {
  const root = await fixture(page, "time-picker-bounded-steps");
  const minutes = root.getByRole("listbox", { name: "Minute" });
  await expect(minutes.getByRole("option", { name: "15" })).toBeEnabled();
  await expect(minutes.getByRole("option", { name: "16" })).toHaveCount(0);
  await expect(root.getByRole("option", { name: "11" })).toBeDisabled();
});

test("disabled and read-only pickers expose state and prevent selection", async ({ page }) => {
  const root = await fixture(page, "time-picker-disabled");
  const readOnly = page.getByTestId("time-picker-read-only");
  await expect(root).toHaveAttribute("data-disabled", "true");
  await expect(readOnly).toHaveAttribute("data-readonly", "true");
  await expect(root.getByRole("option").first()).toBeDisabled();
  await expect(readOnly.getByRole("option").first()).toBeDisabled();
});

test("custom labels and global attributes reach the picker group", async ({ page }) => {
  const root = await fixture(page, "time-picker-custom-labels");
  await expect(root).toHaveAttribute("aria-label", "Appointment time");
  await expect(root).toHaveAttribute("data-time-picker", "custom-attributes");
  await expect(root.getByRole("listbox", { name: "Appointment hour" })).toBeVisible();
  await expect(root.getByRole("button", { name: "Clear appointment" })).toBeVisible();
});
