import { test, expect } from "@playwright/test";

// The main demo renders once as the page's primary preview (first in the DOM)
// and again in the Demos list, so anchor on the first "Quantity" field and walk
// to its own input wrapper to stay scoped to a single instance.
test("steppers and arrow keys adjust the controlled value", async ({ page }) => {
  await page.goto("/components/number_input", {
    timeout: 30 * 1000,
  });

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
    timeout: 30 * 1000,
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
test("quantity sanitizes raw typing and reports the parsed callback value", async ({ page }) => {
  await page.goto("/components/number_input", { timeout: 30 * 1000 });

  const quantity = page.getByRole("textbox", { name: "Quantity" }).first();
  const value = page.locator("#number-value").first();

  await quantity.fill("abc12.3xyz");
  await expect(quantity).toHaveValue("12.3");
  await expect(value).toContainText("Value: 12.3");
});

test("strict bounds reject out-of-range typing and blur bounds clamp", async ({ page }) => {
  await page.goto("/components/number_input", { timeout: 30 * 1000 });

  const strict = page.getByRole("textbox", { name: "Strict (1–10)" }).first();
  await strict.fill("11");
  await expect(strict).toHaveValue("10");
  await strict.fill("1");
  await expect(strict).toHaveValue("1");
  await strict.fill("10");
  await expect(strict).toHaveValue("10");

  const blurClamp = page
    .getByRole("textbox", { name: "Clamp on blur (0–100)" })
    .first();
  await blurClamp.fill("150");
  await expect(blurClamp).toHaveValue("150");
  await blurClamp.blur();
  await expect(blurClamp).toHaveValue("100");
});

test("custom step preserves quarter precision for mouse and ArrowDown", async ({ page }) => {
  await page.goto("/components/number_input", { timeout: 30 * 1000 });

  const input = page.getByRole("textbox", { name: "Custom step" }).first();
  const wrapper = input.locator(
    "xpath=ancestor::*[@data-slot='input-wrapper'][1]",
  );

  await expect(input).toHaveValue("1.00");
  await wrapper.locator("[data-slot='number-input-increment']").click();
  await expect(input).toHaveValue("1.25");
  await input.press("ArrowDown");
  await expect(input).toHaveValue("1.00");
});

test("disabled Locked field disables native input and both steppers", async ({ page }) => {
  await page.goto("/components/number_input", { timeout: 30 * 1000 });

  const input = page.getByLabel("Locked").first();
  const wrapper = input.locator(
    "xpath=ancestor::*[@data-slot='input-wrapper'][1]",
  );

  await expect(input).toBeDisabled();
  await expect(
    wrapper.locator("[data-slot='number-input-increment']"),
  ).toBeDisabled();
  await expect(
    wrapper.locator("[data-slot='number-input-decrement']"),
  ).toBeDisabled();
});

test("Read-only amount preserves value, native readonly, and disabled steppers", async ({ page }) => {
  await page.goto("/components/number_input", { timeout: 30 * 1000 });

  const input = page.getByRole("textbox", { name: "Read-only amount" }).first();
  const wrapper = input.locator(
    "xpath=ancestor::*[@data-slot='input-wrapper'][1]",
  );
  const increment = wrapper.locator("[data-slot='number-input-increment']");
  const decrement = wrapper.locator("[data-slot='number-input-decrement']");

  await expect(input).toHaveValue("12");
  await expect(input).toHaveAttribute("readonly", "true");
  await expect(increment).toBeDisabled();
  await expect(decrement).toBeDisabled();
  await input.focus();
  await input.press("ArrowUp");
  await input.press("ArrowDown");
  await expect(input).toHaveValue("12");
});

test("number input wires labels, descriptions, errors, and form attributes", async ({ page }) => {
  await page.goto("/components/number_input", { timeout: 30 * 1000 });

  const seats = page.getByRole("textbox", { name: "Seats" }).first();
  const seatsId = await seats.getAttribute("id");
  expect(seatsId).toBeTruthy();
  await expect(page.locator(`label[for="${seatsId}"]`).first()).toContainText("Seats");
  await expect(seats).toHaveAttribute("required", /^(?:|true)$/);
  const seatsDescribedBy = (await seats.getAttribute("aria-describedby")) ?? "";
  expect(seatsDescribedBy.split(/\s+/)).toContain(`${seatsId}-description`);

  const error = page.getByRole("textbox", { name: "Quantity" }).last();
  const errorId = await error.getAttribute("id");
  expect(errorId).toBeTruthy();
  const errorDescribedBy = (await error.getAttribute("aria-describedby")) ?? "";
  expect(errorDescribedBy.split(/\s+/)).toContain(`${errorId}-error`);
  await expect(error).toHaveAttribute("aria-invalid", "true");

  const readOnly = page.getByRole("textbox", { name: "Read-only amount" }).first();
  await expect(readOnly).toHaveAttribute("name", "readonly-amount");
  await expect(readOnly).toHaveAttribute("form", "number-form");
});
