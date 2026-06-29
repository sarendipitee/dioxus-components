import { test, expect, type Locator } from '@playwright/test';

async function computedColorForCssColor(locator: Locator, color: string) {
  return locator.evaluate((element: Element, cssColor: string) => {
    const probe = document.createElement('span');
    probe.style.color = cssColor;
    probe.style.position = 'absolute';
    probe.style.visibility = 'hidden';
    element.appendChild(probe);
    const computedColor = getComputedStyle(probe).color;
    probe.remove();
    return computedColor;
  }, color);
}

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

test('dialog title and description keep primitive ARIA with shared typography', async ({
  page,
}) => {
  await page.goto('/components/dialog/block#main', {
    timeout: 20 * 60 * 1000,
    waitUntil: 'load',
  });
  await page.getByRole('button', { name: 'Open Dialog', exact: true }).click();

  const dialog = page.getByRole('dialog', { name: 'Item information' });
  const title = dialog.locator('[data-slot="dialog-title"]');
  const description = dialog.locator('[data-slot="dialog-description"]');

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
  await expect(description).toHaveAttribute('data-tone', 'muted');
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

  const [dialogColor, titleColor, descriptionColor, mutedColor] = await Promise.all([
    dialog.evaluate((element) => getComputedStyle(element).color),
    title.evaluate((element) => getComputedStyle(element).color),
    description.evaluate((element) => getComputedStyle(element).color),
    computedColorForCssColor(dialog, 'var(--fg-muted)'),
  ]);
  expect(titleColor).toBe(dialogColor);
  expect(descriptionColor).toBe(mutedColor);
});
