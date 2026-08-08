import { test, expect, type Locator, type Page } from '@playwright/test';

const URL = '/components/toggle/block#main';
const PREVIEW_ROOT = '#dx-preview-block-root';

async function loadToggle(page: Page): Promise<Locator> {
  await page.goto(URL, { timeout: 30 * 1000, waitUntil: 'load' });
  const root = page.locator(PREVIEW_ROOT);
  await expect(root.getByTestId('toggle-basic').first()).toBeVisible();
  return root;
}

function toggle(root: Locator, name: string): Locator {
  return root.getByRole('button', { name, exact: true }).first();
}
test('toggle renders as an accessible named button', async ({ page }) => {
  const root = await loadToggle(page);
  const button = toggle(root, 'B');

  await expect(button).toBeVisible();
  await expect(button).toHaveRole('button');
  await expect(button).toHaveAttribute('aria-pressed', 'false');
});

test('toggle forwards global attributes', async ({ page }) => {
  const root = await loadToggle(page);
  const button = toggle(root, 'B');

  await expect(button).toHaveAttribute('id', 'toggle-basic');
  await expect(button).toHaveAttribute('data-testid', 'toggle-basic');
  await expect(button).toHaveAttribute('title', 'Toggle global attributes');
  await expect(button).toHaveAttribute('name', 'global-toggle');
  await expect(button).toHaveAttribute('data-audit', 'toggle-global-attributes');
});
test('controlled toggle synchronizes semantic and styling states and focuses on click', async ({ page }) => {
  const root = await loadToggle(page);
  const button = toggle(root, 'Controlled toggle');
  const offBackground = await button.evaluate((element) => getComputedStyle(element).backgroundColor);

  await expect(button).toHaveAttribute('aria-pressed', 'false');
  await expect(button).toHaveAttribute('data-state', 'off');
  await button.click();
  await expect(button).toBeFocused();
  await expect(button).toHaveAttribute('aria-pressed', 'true');
  await expect(button).toHaveAttribute('data-state', 'on');
  await expect.poll(() => button.evaluate((element) => getComputedStyle(element).backgroundColor)).not.toBe(offBackground);

  await button.click();
  await expect(button).toHaveAttribute('aria-pressed', 'false');
  await expect(button).toHaveAttribute('data-state', 'off');
});
test('controlled toggle activates once with Space and Enter', async ({ page }) => {
  const root = await loadToggle(page);
  const button = toggle(root, 'Controlled toggle');
  const count = root.locator('#controlled-count:visible').first();

  await button.focus();
  await button.press('Space');
  await expect(button).toHaveAttribute('aria-pressed', 'true');
  await expect(count).toHaveText('1');
  await button.press('Enter');
  await expect(button).toHaveAttribute('aria-pressed', 'false');
  await expect(count).toHaveText('2');
});
test('controlled callback reports each requested state exactly once', async ({ page }) => {
  const root = await loadToggle(page);
  const button = toggle(root, 'Controlled toggle');
  const count = root.locator('#controlled-count:visible').first();
  const state = root.locator('#controlled-state:visible').first();

  await expect(count).toHaveText('0');
  await expect(state).toHaveText('Off');
  await button.click();
  await expect(count).toHaveText('1');
  await expect(state).toHaveText('On');
  await button.click();
  await expect(count).toHaveText('2');
  await expect(state).toHaveText('Off');
});
test('uncontrolled toggle honors default pressed and reports each transition once', async ({ page }) => {
  const root = await loadToggle(page);
  const button = toggle(root, 'Default pressed toggle');
  const count = root.locator('#uncontrolled-count:visible').first();

  await expect(button).toHaveAttribute('aria-pressed', 'true');
  await expect(button).toHaveAttribute('data-state', 'on');
  await expect(count).toHaveText('0');
  await button.click();
  await expect(button).toHaveAttribute('aria-pressed', 'false');
  await expect(button).toHaveAttribute('data-state', 'off');
  await expect(count).toHaveText('1');
  await button.click();
  await expect(button).toHaveAttribute('aria-pressed', 'true');
  await expect(button).toHaveAttribute('data-state', 'on');
  await expect(count).toHaveText('2');
});
test('disabled toggles cannot receive focus or change state', async ({ page }) => {
  const root = await loadToggle(page);
  const off = toggle(root, 'Disabled off toggle');
  const on = toggle(root, 'Disabled on toggle');

  await expect(off).toBeDisabled();
  await expect(on).toBeDisabled();
  await expect(off).toHaveAttribute('data-disabled', 'true');
  await expect(on).toHaveAttribute('data-disabled', 'true');
  await expect(off).toHaveAttribute('aria-pressed', 'false');
  await expect(on).toHaveAttribute('aria-pressed', 'true');
  await off.focus();
  await on.focus();
  await expect(off).not.toBeFocused();
  await expect(on).not.toBeFocused();
  await off.click({ force: true });
  await on.click({ force: true });
  await expect(off).toHaveAttribute('data-state', 'off');
  await expect(on).toHaveAttribute('data-state', 'on');
  await expect(root.locator('#controlled-count:visible').first()).toHaveText('0');
});
