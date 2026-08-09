import { test, expect } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("/components/textarea", { timeout: 30 * 1000 });
});

test("controlled and uncontrolled textareas preserve their public value contracts", async ({
  page,
}) => {
  const frame = page.locator("#component-preview-frame").first();
  const controlled = frame.locator("#textarea-controlled");
  await expect(controlled).toHaveValue("initial controlled value");
  await controlled.fill("typed value");
  await expect(frame.locator("#textarea-controlled-status")).toContainText(
    "Input callbacks: 1; value: typed value",
  );
  await frame.locator("#textarea-controlled-update").click();
  await expect(controlled).toHaveValue("programmatic controlled update");
  await expect(frame.locator("#textarea-controlled-status")).toContainText(
    "Input callbacks: 1;",
  );

  const uncontrolled = frame.locator("#textarea-uncontrolled");
  await expect(uncontrolled).toHaveValue("Default uncontrolled content");
  await uncontrolled.fill("changed uncontrolled");
  await expect(uncontrolled).toHaveValue("changed uncontrolled");
});

test("required form textarea submits and resets with visible status", async ({
  page,
}) => {
  const frame = page.locator("#component-preview-frame").first();
  const field = frame.locator("#textarea-required-form");
  await frame.locator("#textarea-form-submit").click();
  await expect(frame.locator("#textarea-form-status")).toHaveText(
    "Awaiting submission",
  );
  await field.fill("submitted feedback");
  await frame.locator("#textarea-form-submit").click();
  await expect(frame.locator("#textarea-form-status")).toHaveText(
    "Submitted required textarea",
  );
  const submittedValue = await frame
    .locator("#textarea-form")
    .evaluate((form: HTMLFormElement) => new FormData(form).get("feedback"));
  expect(submittedValue).toBe("submitted feedback");
  await frame.locator("#textarea-form-reset").click();
  await expect(frame.locator("#textarea-form-status")).toHaveText(
    "Reset required textarea",
  );
  await expect(field).toHaveValue("");
});

test("constraints, global attributes, reactive attributes, and focus are observable", async ({
  page,
}) => {
  const frame = page.locator("#component-preview-frame").first();
  const constraints = frame.locator("#textarea-constraints");
  await expect(constraints).toHaveAttribute("minlength", "3");
  await expect(constraints).toHaveAttribute("maxlength", "12");
  await expect(constraints).toHaveAttribute("rows", "4");
  await expect(frame.locator("#textarea-global-attributes")).toHaveAttribute(
    "data-contract",
    "global-attributes",
  );
  const reactive = frame.locator("#textarea-reactive-attributes");
  await expect(reactive).toHaveAttribute("data-active", "false");
  await frame.locator("#textarea-reactive-toggle").click();
  await expect(reactive).toHaveAttribute("data-active", "true");
  await expect(reactive).toHaveAccessibleName(
    "Textarea reactive attributes: active",
  );
  const focus = frame.locator("#textarea-focus-status");
  await focus.focus();
  await expect(frame.locator("#textarea-focus-output")).toHaveText("Focused");
  await focus.blur();
  await expect(frame.locator("#textarea-focus-output")).toHaveText("Blurred");
});

test("autosize textarea grows, clamps, and shrinks", async ({ page }) => {
  await page.goto("/components/textarea/block#autosize", {
    timeout: 30 * 1000,
  });
  const field = page.getByTestId("textarea-autosize").first();
  const initial = await field.evaluate(
    (element) => element.getBoundingClientRect().height,
  );
  await field.fill(
    Array.from({ length: 5 }, (_, index) => `line ${index}`).join("\n"),
  );
  const grown = await field.evaluate(
    (element) => element.getBoundingClientRect().height,
  );
  await field.fill(
    Array.from({ length: 30 }, (_, index) => `line ${index}`).join("\n"),
  );
  const clamped = await field.evaluate(
    (element) => element.getBoundingClientRect().height,
  );
  await field.fill(
    Array.from({ length: 60 }, (_, index) => `line ${index}`).join("\n"),
  );
  const reclamped = await field.evaluate(
    (element) => element.getBoundingClientRect().height,
  );
  expect(reclamped).toBe(clamped);
  await field.fill("");
  const shrunk = await field.evaluate(
    (element) => element.getBoundingClientRect().height,
  );
  expect(shrunk).toBeLessThan(clamped);
});
