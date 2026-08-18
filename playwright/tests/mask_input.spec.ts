import { test, expect, type Page } from "@playwright/test";

async function openMaskInput(page: Page) {
  await page.goto("/components/mask_input", { timeout: 30_000 });
}

test("formats input and reports raw, masked, and complete values", async ({
  page,
}) => {
  await openMaskInput(page);

  const phone = page.getByRole("textbox", { name: "Phone number" }).first();
  await phone.fill("1234567890");

  await expect(phone).toHaveValue("(123) 456-7890");
  await expect(page.locator("#mask-raw").first()).toHaveText("Raw: 1234567890");
  await expect(page.locator("#mask-masked").first()).toHaveText(
    "Masked: (123) 456-7890",
  );
  await expect(page.locator("#mask-complete").first()).toHaveText(
    "Complete: true",
  );
});

test("filters invalid characters and applies custom token transforms", async ({
  page,
}) => {
  await openMaskInput(page);

  const expiry = page.getByRole("textbox", { name: "Expiry" }).first();
  await expiry.fill("1a2/3");
  await expect(expiry).toHaveValue("12 / 3_");

  const license = page.getByRole("textbox", { name: "License plate" }).first();
  await license.fill("aB1x23");
  await expect(license).toHaveValue("AB-123");
});

test("preserves a literal-aware caret while replacing, deleting, and pasting", async ({
  page,
}) => {
  await openMaskInput(page);

  const phone = page.getByRole("textbox", { name: "Phone number" }).first();
  await phone.fill("1234567890");
  await phone.evaluate((input: HTMLInputElement) =>
    input.setSelectionRange(6, 9),
  );
  await page.keyboard.type("999");
  await expect(phone).toHaveValue("(123) 999-7890");

  await phone.fill("1234567890");
  await phone.evaluate((input: HTMLInputElement) =>
    input.setSelectionRange(9, 9),
  );
  await page.keyboard.press("Backspace");
  await expect(phone).toHaveValue("(123) 457-890_");

  await phone.fill("1234567890");
  await phone.evaluate((input: HTMLInputElement) =>
    input.setSelectionRange(10, 10),
  );
  await page.keyboard.press("Delete");
  await expect(phone).toHaveValue("(123) 456-890_");

  await phone.fill("1234567890");
  await phone.evaluate((input: HTMLInputElement) =>
    input.setSelectionRange(6, 9),
  );
  await page.keyboard.insertText("999");
  await expect(phone).toHaveValue("(123) 999-7890");
});

test("shows configured slots on focus and clears an incomplete value on blur", async ({
  page,
}) => {
  await openMaskInput(page);

  await expect(
    page.getByRole("textbox", { name: "Always show mask" }).first(),
  ).toHaveValue("____-____");
  await expect(
    page.getByRole("textbox", { name: "Custom slot character" }).first(),
  ).toHaveValue("··/··/····");

  const autoClear = page
    .getByRole("textbox", { name: "Auto clear when incomplete" })
    .first();
  await autoClear.fill("12");
  await expect(autoClear).toHaveValue("(12_) ___-____");
  await autoClear.blur();
  await expect(autoClear).toHaveValue("");
});

test("initializes a value and exposes an imperative reset", async ({
  page,
}) => {
  await openMaskInput(page);

  const input = page.locator("#mask-ref-controlled").first();
  await expect(input).toHaveValue("1234");
  await input.fill("9876");
  await expect(input).toHaveValue("9876");
  await page.locator("#mask-ref-reset").first().click();
  await expect(input).toHaveValue("");
});

test("forwards validation, form, state, label, and ARIA attributes", async ({
  page,
}) => {
  await openMaskInput(page);

  const account = page.locator("#mask-field-id").first();
  await expect(
    page.locator('label[for="mask-field-id"]').first(),
  ).toContainText("Account code");
  await expect(account).toHaveAttribute("name", "account-code");
  await expect(account).toHaveAttribute("form", "mask-fixture-form");
  await expect(account).toHaveAttribute("autocomplete", "one-time-code");
  await expect(account).toHaveAttribute("required", "true");
  await expect(account).toHaveAttribute("aria-invalid", "true");
  await expect
    .poll(() => account.evaluate((input: HTMLInputElement) => input.form?.id))
    .toBe("mask-fixture-form");
  const describedBy = (await account.getAttribute("aria-describedby"))!
    .split(/\s+/)
    .filter(Boolean);
  expect(new Set(describedBy)).toEqual(
    new Set([
      "mask-field-id-description",
      "mask-field-id-error",
      "mask-extra-description",
    ]),
  );
  expect(describedBy).toHaveLength(3);
  for (const id of describedBy) {
    await expect(page.locator(`#${id}`).first()).toBeVisible();
  }
  await expect(
    page.locator("#mask-field-id-description").first(),
  ).toBeVisible();
  await expect(page.locator("#mask-field-id-error").first()).toBeVisible();

  await expect(page.locator("#mask-disabled").first()).toBeDisabled();
  await expect(page.locator("#mask-readonly").first()).toHaveAttribute(
    "readonly",
    "true",
  );
});
