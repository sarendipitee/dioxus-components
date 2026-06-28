import { test, expect } from '@playwright/test';

test('dropdown menu shared wrappers', async ({ page }) => {
  await page.goto('/components/dropdown_menu');
  const mainDemo = page.locator('.dx-component-section').first();
  const menuButton = mainDemo.getByRole('button', { name: 'Open Menu' });
  const menuElement = menuButton.locator('xpath=..');
  const mainMenuContent = page.getByRole('menu').filter({ has: page.locator('.dx_menu_label', { hasText: 'Actions' }) }).first();
  const nestedDemo = page
    .locator('.dx-component-demo')
    .filter({ has: page.locator('.dx-component-demo-title', { hasText: 'nested_submenus' }) });
  const nestedMenuButton = nestedDemo.getByRole('button', { name: 'Move item' });
  const nestedMenuElement = nestedMenuButton.locator('xpath=..');

  // The menu should not be open initially
  await expect(menuElement).toHaveAttribute('data-state', 'closed');
  // Clicking the menu should open it
  await menuButton.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  await expect(mainMenuContent).toHaveAttribute('data-state', 'open');
  await expect(mainMenuContent.locator('.dx_menu_label', { hasText: 'Actions' })).toBeVisible();
  await expect(mainMenuContent.getByRole('menuitem', { name: 'Edit' })).toContainText('⌘E');
  await expect(mainMenuContent.getByRole('separator')).toHaveCount(2);
  await expect(mainMenuContent.getByRole('menuitemcheckbox', { name: 'Show Toolbar' })).toHaveAttribute('data-state', 'checked');
  await expect(mainMenuContent.getByRole('menuitemradio', { name: 'System' })).toHaveAttribute('data-state', 'checked');

  // Pressing down should focus the first item
  await page.keyboard.press('ArrowDown');
  await expect(mainMenuContent.getByRole('menuitem', { name: 'Edit' })).toBeFocused();
  await expect(mainMenuContent.getByRole('menuitem', { name: 'Undo' })).toHaveAttribute('data-disabled', 'true');
  await page.keyboard.press('ArrowDown');
  await expect(mainMenuContent.getByRole('menuitem', { name: 'Share' })).toBeFocused();
  // Keyboard navigation should open the submenu and focus its first item
  await page.keyboard.press('ArrowRight');
  await expect(page.getByRole('menuitem', { name: 'Copy link' }).first()).toBeFocused();
  await page.keyboard.press('Escape');
  await expect(menuElement).toHaveAttribute('data-state', 'closed');

  // Reopen the menu and verify pointer hover opens submenus too
  await menuButton.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  const shareItem = mainMenuContent.getByRole('menuitem', { name: 'Share' });
  await shareItem.hover();
  const submenu = page
    .locator('.dx_menu_sub_content')
    .filter({ has: page.getByRole('menuitem', { name: 'Copy link' }).first() })
    .first();
  await expect(submenu).toHaveAttribute('data-state', 'open');
  await expect(submenu.getByRole('menuitem', { name: 'Copy link' })).toBeVisible();
  await expect(shareItem).toHaveCSS('background-color', 'rgb(247, 247, 247)');
  const shareBox = await shareItem.boundingBox();
  const submenuBox = await submenu.boundingBox();
  if (!shareBox || !submenuBox) throw new Error('submenu geometry unavailable');
  expect(submenuBox.x).toBeGreaterThanOrEqual(shareBox.x + shareBox.width - 8);
  expect(Math.abs(submenuBox.y - shareBox.y)).toBeLessThanOrEqual(12);
  await submenu.getByRole('menuitem', { name: 'Invite teammate' }).hover();
  await page.mouse.move(submenuBox.x + submenuBox.width + 40, submenuBox.y + submenuBox.height + 40);
  await expect(submenu).toHaveAttribute('data-state', 'closed');
  await shareItem.hover();
  await expect(submenu).toHaveAttribute('data-state', 'open');
  await submenu.getByRole('menuitem', { name: 'Invite teammate' }).click();
  await expect(menuElement).toHaveAttribute('data-state', 'closed');
  await expect(page.getByText('Selected action: Invite teammate')).toBeVisible();

  // Reopen the menu
  await menuButton.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Pressing Escape should close the menu
  await page.keyboard.press('Escape');
  await expect(menuElement).toHaveAttribute('data-state', 'closed');

  // Reopen the menu
  await menuButton.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Pressing Tab should close the menu
  await page.keyboard.press('Tab');
  await expect(menuElement).toHaveAttribute('data-state', 'closed');

  // Reopen the menu
  await menuButton.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Clicking outside the menu should close it
  await nestedDemo.locator('.dx-component-demo-title').click();
  await expect(menuElement).toHaveAttribute('data-state', 'closed');

  // Reopen the menu
  await menuButton.click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  // Clicking a checkable item should toggle it without closing the menu
  await page.getByRole('menuitemcheckbox', { name: 'Show Toolbar' }).click();
  await expect(menuElement).toHaveAttribute('data-state', 'open');
  await expect(mainDemo.getByText('Toolbar visible: false')).toBeVisible();

  // Nested submenu demo should support a depth-3 path
  await nestedMenuButton.evaluate((element) => {
    const root = element.closest('.dx_dropdown_menu') as HTMLElement | null;
    if (!root) throw new Error('nested dropdown root unavailable');
    root.style.position = 'fixed';
    root.style.top = '120px';
    root.style.right = '24px';
    root.style.zIndex = '10';
  });
  await expect(nestedMenuElement).toHaveAttribute('data-state', 'closed');
  await nestedMenuButton.click();
  await expect(nestedMenuElement).toHaveAttribute('data-state', 'open');
  const workspaceAlpha = page.locator('.dx_menu_sub_trigger').filter({ hasText: /^Workspace Alpha$/ });
  await workspaceAlpha.press('ArrowRight');
  const alphaSubmenu = page
    .locator('.dx_menu_sub_content')
    .filter({ has: page.locator('.dx_menu_sub_trigger').filter({ hasText: /^Workspace Alpha \/ Projects$/ }) })
    .first();
  await expect(alphaSubmenu).toHaveAttribute('data-side', 'left');
  const workspaceAlphaBox = await workspaceAlpha.boundingBox();
  const alphaSubmenuBox = await alphaSubmenu.boundingBox();
  if (!workspaceAlphaBox || !alphaSubmenuBox) throw new Error('flipped submenu geometry unavailable');
  expect(alphaSubmenuBox.x + alphaSubmenuBox.width).toBeLessThanOrEqual(workspaceAlphaBox.x + 8);
  await page.locator('.dx_menu_sub_trigger').filter({ hasText: /^Workspace Alpha \/ Projects$/ }).press('ArrowRight');
  await page.locator('.dx_menu_sub_trigger').filter({ hasText: /^Workspace Alpha \/ Projects \/ Q3$/ }).press('ArrowRight');
  await page.getByRole('menuitem', { name: 'Workspace Alpha / Projects / Q3 / Launch' }).press('Enter');
  await expect(nestedDemo.getByText('Selected destination: Workspace Alpha / Projects / Q3 / Launch')).toBeVisible();
});
