import { test, expect, type Page } from '@playwright/test';

const loadNavbar = async (page: Page) => {
  await page.goto('/components/navbar', { timeout: 30 * 1000 });
  const navigation = page.locator('#component-preview-frame').first().locator('[role="navigation"][aria-label="Components"]');
  await expect(navigation).toBeVisible();
  return navigation;
};

test('landmark, menubar, links, and forwarded attributes', async ({ page }) => {
  const navigation = await loadNavbar(page);
  const menubar = navigation.getByRole('menubar');
  await expect(menubar).toHaveAttribute('data-testid', 'navbar');
  await expect(menubar).toHaveAttribute('data-navbar', 'components');
  await expect(menubar).toBeVisible();

  const trigger = navigation.getByRole('menuitem', { name: 'Inputs' });
  const inputsNav = navigation.getByRole('menu').filter({ has: trigger }).first();
  await expect(trigger).toHaveAttribute('aria-expanded', 'false');
  await expect(inputsNav).toHaveAttribute('data-state', 'closed');
  await expect(trigger).toHaveAttribute('aria-controls', 'navbar-inputs-content');
  expect(await trigger.getAttribute('aria-controls')).toBeTruthy();
  const content = page.locator('#navbar-inputs-content');
  await expect(content).toHaveAttribute('id', await trigger.getAttribute('aria-controls') as string);
  await expect(content.getByRole('menuitem', { name: 'Calendar' })).toHaveAttribute('href', /calendar/);
  await expect(content.getByRole('menuitem', { name: 'Slider' })).toHaveAttribute('data-disabled', 'true');
});

test('hover navigation', async ({ page }) => {
  const navigation = await loadNavbar(page);
  const inputsNav = navigation.getByRole('menu').filter({ has: navigation.getByRole('menuitem', { name: 'Inputs' }) }).first();
  await inputsNav.hover();
  await expect(inputsNav).toHaveAttribute('data-state', 'open');
  const trigger = navigation.getByRole('menuitem', { name: 'Inputs' });
  await expect(trigger).toHaveAttribute('aria-expanded', 'true');
  const calendar = page.getByRole('menuitem', { name: 'Calendar' });
  await calendar.click();
  await expect(page).toHaveURL(/\/components\/calendar\?/);
});

test('mobile navigation uses a narrow viewport and fits the viewport', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 667 });
  const navigation = await loadNavbar(page);
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(375);
  const trigger = navigation.getByRole('menuitem', { name: 'Inputs' });
  await trigger.tap();
  await expect(trigger).toHaveAttribute('aria-expanded', 'true');
  await expect(page.getByRole('menuitem', { name: 'Calendar' })).toBeVisible();
  await page.getByRole('menuitem', { name: 'Calendar' }).tap();
  await expect(page).toHaveURL(/\/components\/calendar\?/);
});

test('keyboard navigation roves focus and skips disabled items', async ({ page }) => {
  const navigation = await loadNavbar(page);
  const menubar = navigation.getByRole('menubar');
  await menubar.focus();
  await page.keyboard.press('ArrowRight');
  await expect(navigation.getByRole('menuitem', { name: 'Information' })).toBeFocused();
  await page.keyboard.press('ArrowLeft');
  await expect(navigation.getByRole('menuitem', { name: 'Inputs' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('menuitem', { name: 'Calendar' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('menuitem', { name: 'Checkbox' })).toBeFocused();
  await page.keyboard.press('Home');
  await expect(page.getByRole('menuitem', { name: 'Calendar' })).toBeFocused();
  await page.keyboard.press('End');
  await expect(page.getByRole('menuitem', { name: 'Radio Group' })).toBeFocused();
  await expect(page.getByRole('menuitem', { name: 'Slider' })).toHaveAttribute('data-disabled', 'true');
});

test('Escape dismisses open content and restores trigger focus', async ({ page }) => {
  const navigation = await loadNavbar(page);
  const menubar = navigation.getByRole('menubar');
  const trigger = navigation.getByRole('menuitem', { name: 'Inputs' });
  await menubar.focus();
  await page.keyboard.press('Enter');
  await expect(trigger).toHaveAttribute('aria-expanded', 'true');
  await page.keyboard.press('Escape');
  await expect(trigger).toBeFocused();
  await expect(trigger).toHaveAttribute('aria-expanded', 'false');
  await expect(page.locator('#navbar-inputs-content')).toHaveAttribute('data-state', 'closed');
});

test('outside click and Tab dismiss open content', async ({ page }) => {
  const navigation = await loadNavbar(page);
  const trigger = navigation.getByRole('menuitem', { name: 'Inputs' });
  await trigger.click();
  await page.mouse.click(2, 2);
  await expect(trigger).toHaveAttribute('aria-expanded', 'false');
  await trigger.click();
  await page.keyboard.press('Tab');
  await expect(trigger).toHaveAttribute('aria-expanded', 'false');
});
