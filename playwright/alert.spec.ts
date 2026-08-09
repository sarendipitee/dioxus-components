import { test, expect, type Locator, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const ALERT_DEMO_URL = "/components/alert/block#main";
const PREVIEW_ROOT = "#dx-preview-block-root";

async function loadAlertDemo(
  page: Page,
): Promise<{ root: Locator; alerts: Locator }> {
  await page.goto(ALERT_DEMO_URL, { timeout: 30 * 1000, waitUntil: "load" });

  const root = page.locator(PREVIEW_ROOT);
  const alerts = root.getByRole("alert");
  await expect(alerts).toHaveCount(4);
  await expect(alerts.first()).toBeVisible();
  await expect(
    root.getByText("System maintenance window", { exact: true }),
  ).toBeVisible();

  return { root, alerts };
}

test("renders four alert landmarks with their preview titles and descriptions", async ({
  page,
}) => {
  const { alerts } = await loadAlertDemo(page);
  const content = [
    [
      "System maintenance window",
      "Routine infrastructure work is scheduled for tonight from 11:00 PM to 11:30 PM.",
    ],
    [
      "Payment failed",
      "We could not process the last invoice. Update the billing method to avoid service interruption.",
    ],
    [
      "Profile change pending review",
      "Your updated organization details are saved and waiting for approval from an administrator.",
    ],
    [
      "Backup completed",
      "A new encrypted restore point is available and has been replicated to the secondary region.",
    ],
  ] as const;

  for (const [index, [title, description]] of content.entries()) {
    const alert = alerts.nth(index);
    await expect(alert).toBeVisible();
    await expect(alert).toContainText(title);
    await expect(alert).toContainText(description);
  }
});

test("exposes enabled names for actionable alert buttons", async ({ page }) => {
  const { alerts } = await loadAlertDemo(page);
  const buttons = alerts.getByRole("button");

  await expect(buttons).toHaveCount(2);
  await expect(buttons.nth(0)).toHaveAccessibleName("View status");
  await expect(buttons.nth(0)).toBeEnabled();
  await expect(buttons.nth(1)).toHaveAccessibleName("Fix billing");
  await expect(buttons.nth(1)).toBeEnabled();
});

test("hides decorative alert icons from the accessibility tree", async ({
  page,
}) => {
  const { root } = await loadAlertDemo(page);
  const icons = root.locator('[data-slot="alert-icon"]');

  await expect(icons).toHaveCount(4);
  for (let index = 0; index < 4; index += 1) {
    await expect(icons.nth(index)).toHaveAttribute("aria-hidden", "true");
  }
});

test("alert demo has no automatically detectable accessibility violations", async ({
  page,
}) => {
  const { root } = await loadAlertDemo(page);
  const accessibilityScanResults = await new AxeBuilder({ page })
    .include(PREVIEW_ROOT)
    .disableRules("color-contrast")
    .analyze();

  expect(accessibilityScanResults.violations).toEqual([]);
  await expect(root).toBeVisible();
});
