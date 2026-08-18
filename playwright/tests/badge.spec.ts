import { expect, test, type Locator, type Page } from "@playwright/test";

const BADGE_DEMO_URL = "/components/badge/block#main";
const PREVIEW_ROOT = "#dx-preview-block-root";

async function loadBadgeDemo(
  page: Page,
): Promise<{ root: Locator; badges: Locator }> {
  await page.goto(BADGE_DEMO_URL, { timeout: 30 * 1000, waitUntil: "load" });

  const root = page.locator(PREVIEW_ROOT);
  const badges = root.locator('[class*="dx_badge"]');
  await expect(badges).toHaveCount(5);
  await expect(badges.first()).toBeVisible();

  return { root, badges };
}

test("renders badges in order as noninteractive spans", async ({ page }) => {
  const { badges } = await loadBadgeDemo(page);
  const labels = [
    "Primary",
    "Secondary",
    "Destructive",
    "Outline",
    "Verified",
  ] as const;

  for (const [index, label] of labels.entries()) {
    const badge = badges.nth(index);
    await expect(badge).toBeVisible();
    await expect(badge).toHaveText(label, { exact: true });
    await expect(badge).toHaveJSProperty("tagName", "SPAN");
    await expect(badge).not.toHaveAttribute("role");
    await expect(badge).not.toHaveAttribute("tabindex");
    await expect(badge).toHaveClass(/dx_badge/);
  }
});

test("exposes the expected variants for the first four badges", async ({
  page,
}) => {
  const { badges } = await loadBadgeDemo(page);
  const variants = ["primary", "secondary", "destructive", "outline"] as const;

  for (const [index, variant] of variants.entries()) {
    await expect(badges.nth(index)).toHaveAttribute("data-variant", variant);
  }
});

test("forwards verified badge attributes and icon", async ({ page }) => {
  const { root } = await loadBadgeDemo(page);
  const verified = root.locator('[data-testid="verified-badge"]');

  await expect(verified).toHaveCount(1);
  await expect(verified).toHaveAttribute("id", "verified-status");
  await expect(verified).toHaveAttribute("aria-label", "Verified status");
  await expect(verified).toHaveJSProperty("tagName", "SPAN");
  await expect(verified).toHaveText("Verified", { exact: true });
  await expect(verified.locator("svg")).toHaveCount(1);
  await expect(verified.locator("svg")).toBeVisible();
});
