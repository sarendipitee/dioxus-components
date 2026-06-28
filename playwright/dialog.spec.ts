import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('/components/dialog/block#main', { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  await page.getByRole('button', { name: 'Open Dialog', exact: true }).click();
  // Assert the dialog is open
  const dialog = page.getByRole('dialog');
  await expect(dialog).toBeVisible();
  // Hitting escape should close the dialog
  await page.keyboard.press('Escape');
  // Assert the dialog can no longer be found
  await expect(dialog).toHaveCount(0);

  // Reopen the dialog
  await page.getByRole('button', { name: 'Open Dialog', exact: true }).click();
  await expect(dialog).toBeVisible();
  // Clicking far outside the dialog content should dismiss it.
  await page.mouse.click(2, 2);
  await expect(dialog).toHaveCount(0);
});

test('closing a nested dialog does not replay the outer open animation', async ({ page }) => {
  await page.goto('/components/dialog/block#nested', { timeout: 20 * 60 * 1000 });

  await page.getByRole('button', { name: 'Open Dialog', exact: true }).click();
  const outer = page.getByRole('dialog', { name: 'Manage task' });
  await expect(outer).toBeVisible();
  await page.waitForTimeout(400);

  await outer.getByRole('button', { name: 'Set Priority' }).click();
  const inner = page.getByRole('dialog', { name: 'Set priority' });
  await expect(inner).toBeVisible();
  await expect(outer).toHaveAttribute('data-overlay-depth', '1');
  await page.waitForTimeout(250);

  await inner.getByRole('button', { name: 'Cancel' }).click();
  await expect(inner).toHaveCount(0);
  await expect(outer).toHaveAttribute('data-overlay-depth', '0');
  await page.waitForTimeout(50);

  const activeAnimationNames = await outer.evaluate((element) =>
    element
      .getAnimations({ subtree: false })
      .filter((animation) => animation.playState !== 'finished')
      .map((animation) => (animation as CSSAnimation).animationName)
  );

  expect(activeAnimationNames).not.toContain('dx-dialog-animate-in');
});
