import { test, expect } from '@playwright/test';

const TYPED_TOASTS = [
  { button: 'Success', type: 'success' },
  { button: 'Error', type: 'error' },
  { button: 'Warning', type: 'warning' },
  { button: 'Info', type: 'info' },
] as const;

test('each typed toast renders its leading variant icon', async ({ page }) => {
  await page.goto('/components/toast/block#main');

  for (const { button, type } of TYPED_TOASTS) {
    await page.getByRole('button', { name: button, exact: true }).click();

    const toast = page.locator(`[role="alertdialog"][data-type="${type}"]`);
    await expect(toast).toBeVisible();
    // The variant icon is an svg rendered inside the leading toast-icon slot.
    await expect(toast.locator('[data-slot="toast-icon"] svg')).toBeVisible();
  }
});

test('loading toast shows the spinner instead of a leading icon', async ({ page }) => {
  await page.goto('/components/toast/block#loading');

  await page.getByRole('button', { name: 'Save (success)' }).click();

  const toast = page.locator('[role="alertdialog"][data-type="loading"]');
  await expect(toast).toBeVisible();
  // Loading conveys state through the spinner, so it gets no leading icon.
  await expect(toast.locator('[data-slot="toast-icon"]')).toHaveCount(0);
});

test('toasts can be opened and dismissed individually', async ({ page }) => {
  await page.goto('/components/toast/block#permanent');

  // Permanent toasts never auto-dismiss, so the open/close flow stays stable.
  const trigger = page.getByRole('button', { name: 'Show permanent' });
  await trigger.click();
  await trigger.click();

  const toasts = page.getByRole('alertdialog');
  const closeButtons = toasts.getByRole('button', { name: 'close' });
  await expect(toasts).toHaveCount(2);

  // Hover pauses any timers and expands the stack so the close button is hittable.
  await toasts.first().hover();
  await closeButtons.first().click();
  await expect(closeButtons).toHaveCount(1);

  await toasts.first().hover();
  await closeButtons.first().click();
  await expect(closeButtons).toHaveCount(0);
});

test('expanded stack collapses without replaying entry animation', async ({ page }) => {
  await page.goto('/components/toast/block#permanent');

  const trigger = page.getByRole('button', { name: 'Show permanent' });
  await trigger.click();
  await trigger.click();

  const toasts = page.getByRole('alertdialog');
  await expect(toasts).toHaveCount(2);

  const topToast = toasts.first();
  await expect(topToast).not.toHaveAttribute('data-entering', 'true');

  await topToast.hover();
  await page.mouse.move(0, 0);

  await expect(topToast).not.toHaveCSS('animation-name', /dx-toast-slide-in/);
  await expect(topToast).not.toHaveCSS('opacity', '0');
});
