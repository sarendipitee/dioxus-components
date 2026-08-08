import { test, expect } from "@playwright/test";

const MAIN_PREVIEW_ROOT = "#component-preview-frame";
const POSITIONING_PREVIEW_ROOT = "#dx-preview-block-root";

test("controlled popover exposes trigger state and linked content", async ({ page }) => {
  await page.goto("/components/popover");
  const root = page.locator(MAIN_PREVIEW_ROOT).first();
  const trigger = root.locator('[data-testid="popover-trigger"]');
  const popoverButton = trigger.getByRole("button", { name: "Open popover" });

  await expect(popoverButton).toBeVisible();
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  const contentId = await trigger.getAttribute("aria-controls");
  expect(contentId).toBeTruthy();
  await expect(root.locator('[data-testid="popover-content"]')).toHaveCount(0);

  await popoverButton.click();
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  const dialog = page.locator('[data-testid="popover-content"]');
  await expect(dialog).toBeVisible();
  await expect(dialog).toHaveAttribute("id", contentId!);
  await expect(dialog).toHaveAttribute("aria-modal", "true");
  await expect(dialog).toHaveAccessibleName("Open popover");
  await expect(root.locator('[data-testid="popover-state"]')).toHaveText("Popover is open");
  await expect(dialog).toContainText("Details");
  await expect(dialog).toContainText("This is the popover content.");

  await popoverButton.click();
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(dialog).toHaveCount(0);
  await expect(root.locator('[data-testid="popover-state"]')).toHaveText("Popover is closed");
});

test("native button keyboard activation controls the popover", async ({ page }) => {
  await page.goto("/components/popover");
  const root = page.locator(MAIN_PREVIEW_ROOT).first();
  const trigger = root.locator('[data-testid="popover-trigger"]');
  const button = trigger.getByRole("button", { name: "Open popover" });

  await button.focus();
  await button.press("Enter");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(page.locator('[data-testid="popover-content"]')).toBeVisible();

  await button.press("Space");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(page.locator('[data-testid="popover-content"]')).toHaveCount(0);
});

test("modal popover enters, traps, and returns focus", async ({ page }) => {
  await page.goto("/components/popover");
  const popoverButton = page
    .locator(MAIN_PREVIEW_ROOT)
    .first()
    .getByRole("button", { name: "Open popover", exact: true });
  await popoverButton.click();

  const dialog = page.getByRole("dialog", { name: "Open popover", exact: true });
  const first = dialog.getByRole("button", { name: "First action" });
  const second = dialog.getByRole("button", { name: "Second action" });
  await expect(first).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(second).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(first).toBeFocused();
  await page.keyboard.press("Shift+Tab");
  await expect(second).toBeFocused();

  await page.keyboard.press("Escape");
  await expect(dialog).toHaveCount(0);
  await expect(popoverButton).toBeFocused();
});

test("non-modal popover omits modal semantics and does not trap focus", async ({ page }) => {
  await page.goto("/components/popover");
  const root = page.locator(MAIN_PREVIEW_ROOT).first();
  const trigger = root.getByRole("button", { name: "Open non-modal popover" });
  await trigger.click();

  const dialog = page.getByRole("dialog", { name: "Open non-modal popover" });
  await expect(dialog).not.toHaveAttribute("aria-modal", "true");
  await trigger.focus();
  await page.keyboard.press("Shift+Tab");
  await expect(root.getByRole("button", { name: "Second action" })).toBeFocused();
});
test("popover dismisses when clicking outside", async ({ page }) => {
  await page.goto("/components/popover");
  const popoverButton = page
    .locator(MAIN_PREVIEW_ROOT)
    .first()
    .getByRole("button", { name: "Open popover" });
  await popoverButton.click();
  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();
  await page.mouse.click(2, 2);
  await expect(dialog).toHaveCount(0);
});

test("positioning exposes all side and alignment choices", async ({ page }) => {
  await page.goto("/components/popover/block#positioning");
  const root = page.locator(POSITIONING_PREVIEW_ROOT);
  const placements = [
    "top-start", "top-center", "top-end",
    "right-start", "right-center", "right-end",
    "bottom-start", "bottom-center", "bottom-end",
    "left-start", "left-center", "left-end",
  ];

  await expect(root.getByRole("button")).toHaveCount(12);
  const trigger = root.getByText("trigger", { exact: true });
  await expect(trigger).toBeVisible();

  for (const placement of placements) {
    await root.getByRole("button", { name: placement, exact: true }).click();
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();
    await expect(dialog).toContainText(placement);
    await expect(dialog).toHaveAttribute("data-state", "open");
    await expect(dialog).toHaveAttribute("data-side", placement.split("-")[0]);
    await expect(dialog).toHaveAttribute("data-align", placement.split("-")[1]);
  }
});
