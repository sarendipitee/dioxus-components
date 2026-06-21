import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('/components/checkbox', { timeout: 20 * 60 * 1000, waitUntil: 'networkidle' }); // Increase timeout to 20 minutes
  const checkbox = page
    .getByRole('checkbox', { name: 'Accept terms and conditions', exact: true })
    .first();
  await expect(checkbox).toBeVisible();
  // The checkbox should not be checked initially
  await expect(checkbox).toHaveAttribute('data-state', 'unchecked');
  // Clicking the checkbox should check it
  await checkbox.click();
  await expect(checkbox).toHaveAttribute('data-state', 'checked');
  // Pressing space should also toggle the checkbox.
  await checkbox.focus();
  await checkbox.press('Space');
  await expect(checkbox).toHaveAttribute('data-state', 'unchecked');
});
