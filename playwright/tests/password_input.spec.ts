import { test, expect } from "@playwright/test";

// The component page renders every demo's live preview inside a
// `#component-preview-frame` panel. The static build serves a prerendered copy
// alongside the hydrated app, so each frame appears twice — `.first()` pins the
// queries to a single, self-consistent instance.

test("visibility toggle reveals and hides the value", async ({ page }) => {
  await page.goto("/components/password_input", {
    timeout: 30 * 1000,
  });

  const frame = page.locator("#component-preview-frame").first();
  const control = frame.locator('input[data-slot="password-input-control"]');
  const toggle = frame.locator('[data-slot="password-input-toggle"]');

  // Masked by default.
  await expect(control).toHaveAttribute("type", "password");
  await expect(toggle).toHaveAttribute("aria-label", "Show password");
  await expect(toggle).toHaveAttribute("aria-pressed", "false");

  // Revealing swaps the input type and the toggle's accessible state.
  await toggle.click();
  await expect(control).toHaveAttribute("type", "text");
  await expect(toggle).toHaveAttribute("aria-label", "Hide password");
  await expect(toggle).toHaveAttribute("aria-pressed", "true");

  // Hiding again restores the masked state.
  await toggle.click();
  await expect(control).toHaveAttribute("type", "password");
  await expect(toggle).toHaveAttribute("aria-pressed", "false");
});

test("controlled visibility drives a field without its own toggle", async ({
  page,
}) => {
  await page.goto("/components/password_input", {
    timeout: 30 * 1000,
  });

  const frame = page
    .locator("#component-preview-frame")
    .filter({ hasText: "Confirm password" })
    .first();
  const controls = frame.locator('input[data-slot="password-input-control"]');
  const password = controls.nth(0);
  const confirm = controls.nth(1);

  // The confirm field opts out of the bundled toggle.
  const confirmWrapper = frame
    .locator('[data-slot="input-wrapper"]')
    .filter({ has: confirm });
  await expect(
    confirmWrapper.locator('[data-slot="password-input-toggle"]'),
  ).toHaveCount(0);

  await expect(password).toHaveAttribute("type", "password");
  await expect(confirm).toHaveAttribute("type", "password");

  // The external button controls both fields at once.
  await frame.getByRole("button", { name: "Show both" }).click();
  await expect(password).toHaveAttribute("type", "text");
  await expect(confirm).toHaveAttribute("type", "text");

  await frame.getByRole("button", { name: "Hide both" }).click();
  await expect(password).toHaveAttribute("type", "password");
  await expect(confirm).toHaveAttribute("type", "password");
});

test("loading and disabled states adjust the trailing toggle", async ({
  page,
}) => {
  await page.goto("/components/password_input", {
    timeout: 30 * 1000,
  });

  const frame = page
    .locator("#component-preview-frame")
    .filter({ hasText: "With error" })
    .first();
  const wrappers = frame.locator('[data-slot="input-wrapper"]');

  // Error field starts revealed via default_visible.
  await expect(
    wrappers
      .filter({ hasText: "With error" })
      .locator('input[data-slot="password-input-control"]'),
  ).toHaveAttribute("type", "text");

  // Loading replaces the toggle with the shell spinner.
  const loadingWrapper = wrappers.filter({ hasText: "Loading" });
  await expect(
    loadingWrapper.locator('[data-slot="password-input-toggle"]'),
  ).toHaveCount(0);
  await expect(
    loadingWrapper.locator('[data-slot="input-spinner"]'),
  ).toBeVisible();

  // Disabled field disables its toggle button.
  const disabledWrapper = wrappers.filter({ hasText: "Disabled" });
  await expect(
    disabledWrapper.locator('[data-slot="password-input-toggle"]'),
  ).toBeDisabled();
});

test("main field exposes its label, description, and toggle relationship", async ({
  page,
}) => {
  await page.goto("/components/password_input", { timeout: 30 * 1000 });

  const frame = page.locator("#component-preview-frame").first();
  const control = frame
    .locator('input[data-slot="password-input-control"]')
    .first();
  const toggle = frame.locator('[data-slot="password-input-toggle"]').first();
  const inputId = await control.getAttribute("id");

  await expect(control).toHaveAccessibleName("Password");
  await expect(frame.locator(`label[for="${inputId}"]`)).toHaveText("Password");
  await expect(control).toHaveAttribute("aria-describedby", /.+/);
  await expect(toggle).toHaveAttribute("aria-controls", inputId);
});

test("toggle is keyboard reachable and keeps focus through reveal transitions", async ({
  page,
}) => {
  await page.goto("/components/password_input", { timeout: 30 * 1000 });

  const frame = page.locator("#component-preview-frame").first();
  const control = frame
    .locator('input[data-slot="password-input-control"]')
    .first();
  const toggle = frame.locator('[data-slot="password-input-toggle"]').first();

  await control.focus();
  await control.press("Tab");
  await expect(toggle).toBeFocused();
  await toggle.press("Enter");
  await expect(control).toHaveAttribute("type", "text");
  await expect(toggle).toBeFocused();
  await toggle.press("Space");
  await expect(control).toHaveAttribute("type", "password");
  await expect(toggle).toBeFocused();
});

test("callback-controlled field reports native input values and custom attributes", async ({
  page,
}) => {
  await page.goto("/components/password_input/block#states", {
    timeout: 30 * 1000,
  });

  const frame = page.locator("#dx-preview-block-root");
  const control = frame.locator("#callback-password");
  const toggle = frame.locator(
    'button[data-slot="password-input-toggle"][aria-controls="callback-password"]',
  );

  await expect(control).toHaveAccessibleName("Callback password");
  await expect(control).toHaveAttribute("placeholder", "Type a password");
  await expect(control).toHaveAttribute("name", "account_password");
  await expect(control).toHaveAttribute("form", "password-input-form");
  await expect(control).toHaveAttribute("data-password-field", "callback");
  await expect(toggle).toHaveAttribute("aria-label", "Reveal account password");
  await control.fill("updated-secret");
  await expect(frame.getByTestId("password-value")).toHaveText(
    "updated-secret",
  );
  await toggle.click();
  await expect(toggle).toHaveAttribute(
    "aria-label",
    "Conceal account password",
  );
});

test("required and error fields expose native validation and descriptions", async ({
  page,
}) => {
  await page.goto("/components/password_input/block#states", {
    timeout: 30 * 1000,
  });

  const frame = page.locator("#dx-preview-block-root");
  const required = frame.locator("#required-password");
  const error = frame.locator("#error-password");
  await expect(required).toHaveAttribute("required", "true");
  await expect(frame.locator('label[for="required-password"]')).toContainText(
    "*",
  );
  await expect(error).toHaveAttribute("aria-invalid", "true");
  await expect(error).toHaveAttribute("aria-describedby", /.+/);
  await expect(error).toHaveAttribute("aria-describedby", /error/);
});

test("readonly remains focusable and immutable while disabled and loading are unavailable", async ({
  page,
}) => {
  await page.goto("/components/password_input/block#states", {
    timeout: 30 * 1000,
  });

  const frame = page.locator("#dx-preview-block-root");
  const readonly = frame.getByLabel("Read-only password");
  await expect(readonly).toHaveValue("immutable");
  await expect(readonly).toHaveAttribute("readonly", "true");
  await readonly.focus();
  await expect(readonly).toBeFocused();
  await readonly.press("a");
  await expect(readonly).toHaveValue("immutable");

  const disabledWrapper = frame
    .locator('[data-slot="input-wrapper"]')
    .filter({ hasText: "Disabled" });
  await expect(
    disabledWrapper.locator('input[data-slot="password-input-control"]'),
  ).toBeDisabled();
  await expect(
    disabledWrapper.locator('[data-slot="password-input-toggle"]'),
  ).toBeDisabled();
  const loadingWrapper = frame
    .locator('[data-slot="input-wrapper"]')
    .filter({ hasText: "Loading" });
  await expect(loadingWrapper.locator('[data-slot="input"]')).toHaveAttribute(
    "aria-busy",
    "true",
  );
  await expect(
    loadingWrapper.locator('[data-slot="password-input-toggle"]'),
  ).toHaveCount(0);
  await expect(
    loadingWrapper.locator('[data-slot="input-spinner"]'),
  ).toBeVisible();
});

test("password form data includes readonly and callback fields but excludes disabled", async ({
  page,
}) => {
  await page.goto("/components/password_input/block#states", {
    timeout: 30 * 1000,
  });

  const frame = page.locator("#dx-preview-block-root");
  const entries = await frame
    .locator("#password-input-form")
    .evaluate((form) =>
      Array.from(new FormData(form as HTMLFormElement).entries()).map(
        ([name, value]) => [name, String(value)],
      ),
    );
  expect(entries).toContainEqual(["readonly_password", "immutable"]);
  expect(entries.some(([name]) => name === "account_password")).toBe(true);
  expect(entries.some(([name]) => name === "disabled_password")).toBe(false);
});
