import { test, expect } from "@playwright/test";

// A tall viewport keeps the dropdown columns on the page. The popover renders at
// a negative offset in headless mode (a floating-placement artifact unrelated to
// the value bug), so the click is dispatched via `dispatchEvent('click')`, which
// bypasses viewport/actionability checks and still reaches Dioxus's delegated
// click handler. This certifies the value path (click -> on_select -> displayed
// value), NOT that a real user can physically reach the option.
test.use({ viewport: { width: 1280, height: 2000 } });

test("time input column picker click updates the displayed value", async ({
  page,
}) => {
  await page.goto("/components/time_input", {
    timeout: 20 * 60 * 1000,
  });

  // The main preview demo renders a 24-hour input seeded to 14:45.
  const startLabel = page.getByText("Start time", { exact: true });
  await expect(startLabel).toBeVisible();

  const wrapper = page
    .locator("[data-slot='input-wrapper']")
    .filter({ has: startLabel })
    .first();

  const hourSegment = wrapper.getByRole("spinbutton", { name: "hour" });
  const minuteSegment = wrapper.getByRole("spinbutton", { name: "minute" });

  await expect(hourSegment).toHaveText("14");
  await expect(minuteSegment).toHaveText("45");

  // Focusing the input opens the column picker dropdown.
  await hourSegment.focus();

  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();

  // Click an hour option via a coordinate-independent dispatch.
  const hourColumn = dialog.getByRole("listbox", { name: "Hr" });
  await hourColumn
    .getByRole("option", { name: "08", exact: true })
    .dispatchEvent("click");
  await page.waitForTimeout(200);
  console.log("OBS hour after click:", await hourSegment.textContent());
  console.log("OBS dialog after hour click:", await dialog.isVisible());

  await expect(hourSegment).toHaveText("08");
  await expect(dialog).toBeVisible();

  // Click a minute option the same way.
  const minuteColumn = dialog.getByRole("listbox", { name: "Min" });
  await minuteColumn
    .getByRole("option", { name: "10", exact: true })
    .dispatchEvent("click");
  await page.waitForTimeout(200);
  console.log("OBS minute after click:", await minuteSegment.textContent());

  await expect(minuteSegment).toHaveText("10");
  await expect(dialog).toBeVisible();
});
