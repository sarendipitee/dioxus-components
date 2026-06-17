import { test, expect } from '@playwright/test';

test('opens and closes on cancel', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();

  await page.getByRole('button', { name: 'Stay' }).click();
  await expect(dialog).toHaveCount(0);
});

test('fires on_click and closes on action', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();

  await page.getByRole('button', { name: 'Leave' }).click();
  await expect(dialog).toHaveCount(0);
  await expect(page.getByText('You left the page.')).toBeVisible();
});

test('closes on escape', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();

  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
});

test('does not close on backdrop click', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();

  // AlertDialog must not dismiss on backdrop click — user must use a button
  await page.mouse.click(2, 2);
  await expect(dialog).toBeVisible();
});
