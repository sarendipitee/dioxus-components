import { test, expect } from "@playwright/test";

// The main demo renders once as the page's primary preview (first in the DOM)
// and again in the Demos list, so anchor on the first "Quantity" field and walk
// to its own input wrapper to stay scoped to a single instance.
test("steppers and arrow keys adjust the controlled value", async ({ page }) => {
  await page.goto("/components/number_input", {
    timeout: 20 * 60 * 1000,
  }); // Increase timeout to 20 minutes

  const quantity = page.getByRole("textbox", { name: "Quantity" }).first();
  const value = page.locator("#number-value").first();
  const wrapper = quantity.locator(
    "xpath=ancestor::*[@data-slot='input-wrapper'][1]",
  );

  await expect(quantity).toHaveValue("42");
  await expect(value).toContainText("Value: 42");

  await wrapper.locator("[data-slot='number-input-increment']").click();
  await expect(quantity).toHaveValue("43");
  await expect(value).toContainText("Value: 43");

  await wrapper.locator("[data-slot='number-input-decrement']").click();
  await wrapper.locator("[data-slot='number-input-decrement']").click();
  await expect(quantity).toHaveValue("41");
  await expect(value).toContainText("Value: 41");

  await quantity.focus();
  await quantity.press("ArrowUp");
  await expect(quantity).toHaveValue("42");
  await expect(value).toContainText("Value: 42");
});

test("min/max bounds disable the steppers", async ({ page }) => {
  await page.goto("/components/number_input", {
    timeout: 20 * 60 * 1000,
  });

  const quantity = page.getByRole("textbox", { name: "Quantity" }).first();
  const wrapper = quantity.locator(
    "xpath=ancestor::*[@data-slot='input-wrapper'][1]",
  );

  await quantity.fill("100");
  await expect(
    wrapper.locator("[data-slot='number-input-increment']"),
  ).toBeDisabled();

  await quantity.fill("0");
  await expect(
    wrapper.locator("[data-slot='number-input-decrement']"),
  ).toBeDisabled();
});
