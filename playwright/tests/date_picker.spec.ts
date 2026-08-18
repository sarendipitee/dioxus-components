import { test, expect, type Page } from "@playwright/test";

const datePickerFrame = (page: Page) =>
  page.locator("#component-preview-frame").first();

const showMay2026 = async (frame: ReturnType<typeof datePickerFrame>) => {
  await frame.getByRole("combobox", { name: "Year" }).selectOption("2026");
  await frame.getByRole("combobox", { name: "Month" }).selectOption("5");
};
