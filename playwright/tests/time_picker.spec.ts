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
