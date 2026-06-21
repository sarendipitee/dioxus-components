import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('/components/toast/block#main');
  // Create a toast
  await page.getByRole('button', { name: 'Info (60s)' }).click();
  // Create another toast
  await page.getByRole('button', { name: 'Info (60s)' }).click();
  const toasts = page.getByRole('alertdialog');
  const toast_close_buttons = toasts.getByRole('button', { name: 'close' });

  await expect(toasts).toHaveCount(2);
  // Hover and close the first toast
  await toasts.first().hover();
  await toast_close_buttons.first().click();
  await expect(toast_close_buttons).toHaveCount(1);

  // Hover and close the second toast
  await toasts.first().hover();
  await toast_close_buttons.first().click();
  await expect(toast_close_buttons).toHaveCount(0);
});
