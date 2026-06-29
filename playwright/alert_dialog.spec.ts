import { test, expect } from '@playwright/test';

test('opens and closes on cancel', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();
  await expect(dialog.getByRole('button', { name: 'Stay' })).toHaveAttribute('type', 'button');

  await page.getByRole('button', { name: 'Stay' }).click();
  await expect(dialog).toHaveCount(0);
});

test('fires on_click and closes on action', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();
  await expect(dialog.getByRole('button', { name: 'Leave', exact: true })).toHaveAttribute(
    'type',
    'button',
  );

  await page.getByRole('button', { name: 'Leave', exact: true }).click();
  await expect(dialog).toHaveCount(0);
  await expect(page.getByText('You left the page.', { exact: true }).first()).toBeVisible();
});

test('keeps primitive ARIA wrappers with shared typography', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  const alertDialogContent = page.locator('[data-slot="alert-dialog-content"]').first();
  const title = dialog.locator('[data-slot="alert-dialog-title"]');
  const description = dialog.locator('[data-slot="alert-dialog-description"]');

  await expect(dialog).toBeVisible();
  await expect(title).toHaveCount(1);
  await expect(description).toHaveCount(1);
  await expect(title).toHaveClass(/dx_heading/);
  await expect(description).toHaveClass(/dx_text/);
  await expect(title).toHaveAttribute('data-size', 'lg');
  await expect(title).toHaveAttribute('data-weight', 'bold');
  await expect(title).toHaveAttribute('data-tone', 'default');
  await expect(title).toHaveAttribute('data-wrap', 'wrap');
  await expect(title).toHaveAttribute('data-truncate', 'false');
  await expect(description).toHaveAttribute('data-size', 'md');
  await expect(description).toHaveAttribute('data-tone', 'default');
  await expect(description).toHaveAttribute('data-weight', 'inherit');
  await expect(description).toHaveAttribute('data-wrap', 'wrap');
  await expect(description).toHaveAttribute('data-truncate', 'false');

  const titleId = await title.getAttribute('id');
  const descriptionId = await description.getAttribute('id');
  expect(titleId).toBeTruthy();
  expect(descriptionId).toBeTruthy();
  await expect(dialog).toHaveAttribute('aria-labelledby', titleId!);
  await expect(dialog).toHaveAttribute('aria-describedby', descriptionId!);

  await expect(title).toHaveJSProperty('tagName', 'H2');
  await expect(description).toHaveJSProperty('tagName', 'P');
  await expect(title.locator('h1,h2,h3,h4,h5,h6,p')).toHaveCount(0);
  await expect(description.locator('h1,h2,h3,h4,h5,h6,p')).toHaveCount(0);

  const [contentColor, descriptionColor] = await Promise.all([
    alertDialogContent.evaluate((element) => getComputedStyle(element).color),
    description.evaluate((element) => getComputedStyle(element).color),
  ]);
  expect(descriptionColor).toBe(contentColor);
});

test('does not close on escape', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();

  await page.keyboard.press('Escape');
  await expect(dialog).toBeVisible();
});

test('does not close on backdrop click', async ({ page }) => {
  await page.goto('/components/alert_dialog', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Leave page' }).click();

  const dialog = page.getByRole('alertdialog');
  await expect(dialog).toBeVisible();

  // AlertDialog must not dismiss on backdrop click; user must use a button.
  await page.mouse.click(2, 2);
  await expect(dialog).toBeVisible();
});
