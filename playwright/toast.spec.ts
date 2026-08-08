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
    await page.locator('#dx-preview-block-root').getByRole('button', { name: button, exact: true }).click();

    const toast = page.locator(`[role="alertdialog"][data-type="${type}"]:visible`);
    await expect(toast).toBeVisible();
    // The variant icon is an svg rendered inside the leading toast-icon slot.
    await expect(toast.locator('[data-slot="toast-icon"] svg')).toBeVisible();
  }
});

test('loading toast shows the spinner instead of a leading icon', async ({ page }) => {
  await page.goto('/components/toast/block#loading');

  await page.locator('#dx-preview-block-root').getByRole('button', { name: 'Save (success)' }).click();

  const toast = page.locator('[role="alertdialog"][data-type="loading"]:visible');
  await expect(toast).toBeVisible();
  // Loading conveys state through the spinner, so it gets no leading icon.
  await expect(toast.locator('[data-slot="toast-icon"]')).toHaveCount(0);
});

test('toasts can be opened and dismissed individually', async ({ page }) => {
  await page.goto('/components/toast/block#permanent');

  // Permanent toasts never auto-dismiss, so the open/close flow stays stable.
  const trigger = page.locator('#dx-preview-block-root').getByRole('button', { name: 'Show permanent' });
  await trigger.click();
  await trigger.click();

  const toasts = page.locator('[role="alertdialog"]:visible');
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

test('collapsed stack keeps variable-height toasts at measured edge offsets', async ({ page }) => {
  await page.goto('/components/toast/block#main');

  await page.locator('#dx-preview-block-root').getByRole('button', { name: 'Info', exact: true }).click();
  await page.waitForTimeout(100);
  await page.locator('#dx-preview-block-root').getByRole('button', { name: 'Success', exact: true }).click();
  await page.waitForTimeout(500);

  const frontToast = page.locator('[role="alertdialog"]:visible').first();
  await expect(frontToast).toHaveAttribute('data-type', 'success');

  const frontBox = await frontToast.boundingBox();
  expect(frontBox).not.toBeNull();

  const olderToast = page.locator('[role="alertdialog"]:visible').nth(1);
  await expect(olderToast).toHaveAttribute('data-type', 'info');
  const olderBox = await olderToast.boundingBox();
  expect(olderBox).not.toBeNull();

  expect(olderBox!.height).toBeGreaterThan(0);
  expect(Math.abs((frontBox!.y - olderBox!.y) - 15)).toBeLessThanOrEqual(2);
});

test('expanded stack collapses without replaying entry animation', async ({ page }) => {
  await page.goto('/components/toast/block#permanent');

  const trigger = page.locator('#dx-preview-block-root').getByRole('button', { name: 'Show permanent' });
  await trigger.click();
  await trigger.click();

  const toasts = page.locator('[role="alertdialog"]:visible');
  await expect(toasts).toHaveCount(2);

  const topToast = toasts.first();
  await expect(topToast).not.toHaveAttribute('data-entering', 'true');

  await topToast.hover();
  await page.mouse.move(0, 0);

  await expect(topToast).not.toHaveCSS('animation-name', /dx-toast-slide-in/);
  await expect(topToast).not.toHaveCSS('opacity', '0');
});
test('description toasts expose exact content and linked accessible names', async ({ page }) => {
  await page.goto('/components/toast/block#with_description');
  await page.locator('#dx-preview-block-root').getByRole('button', { name: 'Success', exact: true }).click();
  const toast = page.locator('[role="alertdialog"]:visible').filter({ hasText: 'Event has been created' });
  await expect(toast).toContainText('Event has been created');
  await expect(toast).toContainText('Monday, January 3rd at 6:00pm');
  await expect(toast).toHaveAttribute('aria-modal', 'false');
  await expect(toast.locator('[role="alert"][aria-atomic="true"]')).toHaveCount(1);
  const labelledBy = await toast.getAttribute('aria-labelledby');
  const describedBy = await toast.getAttribute('aria-describedby');
  expect(labelledBy).toBeTruthy();
  expect(describedBy).toBeTruthy();
  await expect(page.locator(`#${labelledBy!}`)).toHaveText('Event has been created');
  await expect(page.locator(`#${describedBy!}`)).toHaveText('Monday, January 3rd at 6:00pm');
});

test('action provider forwards attributes, limits toasts, and reports callbacks', async ({ page }) => {
  await page.goto('/components/toast/block#with_action');
  const region = page.locator('#toast-audit-region:visible');
  const result = page.locator('[data-testid="toast-action-result"]');
  await expect(region).toHaveAttribute('data-testid', 'toast-audit-region');
  await expect(region).toHaveAttribute('data-position', 'top-left');
  await expect(region).toHaveAttribute('aria-label', '0 notifications');
  await expect(region).toHaveAttribute('tabindex', '-1');
  await expect(result).toHaveText('No action selected');
  const actionTrigger = page.locator('#dx-preview-block-root').getByRole('button', { name: 'Action', exact: true });
  await actionTrigger.click();
  await actionTrigger.click();
  await actionTrigger.click();
  await expect(region).toHaveAttribute('aria-label', '2 notifications');
  await expect(region.getByRole('alertdialog')).toHaveCount(2);
  await region.getByRole('button', { name: 'Undo', exact: true }).first().click();
  await expect(result).toHaveText('Undo selected');
  await expect(region.getByRole('alertdialog')).toHaveCount(1);
  const actionCancelTrigger = page.locator('#dx-preview-block-root').getByRole('button', { name: 'Action + cancel', exact: true });
  await actionCancelTrigger.click();
  await region.getByRole('button', { name: 'Save now', exact: true }).click();
  await expect(result).toHaveText('Save selected');
  await expect(region.getByRole('alertdialog')).toHaveCount(1);
  await actionCancelTrigger.click();
  await region.getByRole('button', { name: 'Discard', exact: true }).click();
  await expect(result).toHaveText('Discard selected');
  await expect(region.getByRole('alertdialog')).toHaveCount(1);
});

test('F6 focuses the notification region while a toast exists', async ({ page }) => {
  await page.goto('/components/toast/block#main');
  await page.locator('#dx-preview-block-root').getByRole('button', { name: 'Info', exact: true }).click();
  const region = page.locator('[role="region"][data-position]:visible');
  await expect(region).toBeVisible();
  await page.keyboard.press('F6');
  await expect(region).toBeFocused();
});

test('custom two-second toast pauses while hovered and resumes on leave', async ({ page }) => {
  await page.goto('/components/toast/block#custom_duration');
  await page.locator('#dx-preview-block-root').getByRole('button', { name: '2s — quick confirm', exact: true }).click();
  const toast = page.locator('[role="alertdialog"]:visible').filter({ hasText: 'Copied to clipboard' });
  await expect(toast).toBeVisible();
  await toast.hover();
  await page.waitForTimeout(2300);
  await expect(toast).toBeVisible();
  await page.mouse.move(0, 0);
  await expect(toast).toBeHidden({ timeout: 2500 });
});

test('promise toasts transition from loading to success and error', async ({ page }) => {
  await page.goto('/components/toast/block#loading');
  const block = page.locator('#dx-preview-block-root');
  await block.getByRole('button', { name: 'Save (success)', exact: true }).click();
  await expect(page.locator('[role="alertdialog"]:visible').filter({ hasText: 'Saving your changes…' })).toBeVisible();
  await expect(page.locator('[role="alertdialog"]:visible').filter({ hasText: 'Changes saved' })).toBeVisible({ timeout: 3500 });
  await block.getByRole('button', { name: 'Publish (error)', exact: true }).click();
  await expect(page.locator('[role="alertdialog"]:visible').filter({ hasText: 'Publishing…' })).toBeVisible();
  await expect(page.locator('[role="alertdialog"]:visible').filter({ hasText: 'Publish failed' })).toBeVisible({ timeout: 3500 });
  await expect(page.getByRole('alertdialog', { name: 'Publish failed' })).toBeVisible({ timeout: 3500 });
});
