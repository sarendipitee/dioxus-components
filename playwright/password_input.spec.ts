import { test, expect } from "@playwright/test";

// The component page renders every demo's live preview inside a
// `#component-preview-frame` panel. The static build serves a prerendered copy
// alongside the hydrated app, so each frame appears twice — `.first()` pins the
// queries to a single, self-consistent instance.

test("visibility toggle reveals and hides the value", async ({ page }) => {
  await page.goto("/components/password_input", {
    timeout: 20 * 60 * 1000,
  }); // Increase timeout to 20 minutes

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
    timeout: 20 * 60 * 1000,
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
    timeout: 20 * 60 * 1000,
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
