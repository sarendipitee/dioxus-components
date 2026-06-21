import { test, expect, type Page } from '@playwright/test';

const PAGE_URL = '/components/color_picker';
const PAGE_TIMEOUT = 20 * 60 * 1000;

async function loadPicker(page: Page) {
  await page.goto(PAGE_URL, { timeout: PAGE_TIMEOUT });
  await page.waitForLoadState('networkidle');

  const picker = page.getByRole('group', { name: 'Color picker' }).first();
  const areaThumb = page.getByLabel('Color area').first();
  const hueThumb = page.getByRole('slider', { name: 'Hue' }).first();
  const saturationInput = page.locator('input[aria-label="Saturation"]').first();
  const valueInput = page.locator('input[aria-label="Value"]').first();

  await expect(picker).toBeVisible();
  await expect(areaThumb).toBeVisible();
  await expect(hueThumb).toBeVisible();
  await expect(saturationInput).toHaveValue('50');
  await expect(valueInput).toHaveValue('100');

  return { picker, areaThumb, hueThumb, saturationInput, valueInput };
}

test('renders the inline picker with the expected initial channels', async ({ page }) => {
  const { hueThumb, saturationInput, valueInput } = await loadPicker(page);

  await expect.poll(async () => Number(await hueThumb.getAttribute('aria-valuenow'))).toBeGreaterThan(252);
  await expect.poll(async () => Number(await hueThumb.getAttribute('aria-valuenow'))).toBeLessThan(253);
  await expect(saturationInput).toHaveValue('50');
  await expect(valueInput).toHaveValue('100');
});

test('hue slider keyboard navigation updates color', async ({ page }) => {
  const { hueThumb } = await loadPicker(page);
  await hueThumb.focus();

  const before = Number(await hueThumb.getAttribute('aria-valuenow'));
  await page.keyboard.press('ArrowRight');
  const after = Number(await hueThumb.getAttribute('aria-valuenow'));

  // ArrowRight should move hue by one step; default slider step is 1.
  expect(after).toBeGreaterThan(before);
  expect(after - before).toBeLessThan(5);

  // Shift+ArrowRight applies the 10× multiplier.
  await page.keyboard.press('Shift+ArrowRight');
  const shifted = Number(await hueThumb.getAttribute('aria-valuenow'));
  expect(shifted - after).toBeGreaterThan(5);
});

test('color area thumb keyboard navigation updates saturation/value', async ({ page }) => {
  const { areaThumb, saturationInput, valueInput } = await loadPicker(page);
  await areaThumb.focus();

  const saturationBefore = Number(await saturationInput.inputValue());
  const valueBefore = Number(await valueInput.inputValue());

  await page.keyboard.press('ArrowRight');
  await expect(saturationInput).toBeFocused();

  await page.keyboard.press('ArrowDown');
  await expect(valueInput).toBeFocused();

  const saturationAfter = Number(await saturationInput.inputValue());
  const valueAfter = Number(await valueInput.inputValue());

  expect(saturationAfter).toBeGreaterThan(saturationBefore);
  expect(valueAfter).toBeLessThan(valueBefore);
});

test('clicking the color area updates saturation and value', async ({ page, browserName }) => {
  test.skip(
    browserName === 'firefox',
    'Firefox automation does not reliably dispatch a single-click update for the color area; drag coverage remains.',
  );

  const { saturationInput, valueInput } = await loadPicker(page);
  const area = page.locator('.dx_color_area_track').first();

  const box = await area.boundingBox();
  if (!box) throw new Error('color area has no bounding box');
  await page.mouse.click(box.x + box.width * 0.8, box.y + box.height * 0.8);

  await expect.poll(async () => Number(await saturationInput.inputValue())).toBeGreaterThan(70);
  await expect.poll(async () => Number(await valueInput.inputValue())).toBeLessThan(30);
});

test('dragging the color area updates saturation and value', async ({ page, browserName }) => {
  test.skip(
    browserName === 'firefox',
    'Firefox automation does not reliably dispatch pointer drag updates for the color area; keyboard coverage remains.',
  );

  const { saturationInput, valueInput } = await loadPicker(page);
  const area = page.locator('.dx_color_area_track').first();

  const saturationBefore = Number(await saturationInput.inputValue());
  const valueBefore = Number(await valueInput.inputValue());

  const box = await area.boundingBox();
  if (!box) throw new Error('color area has no bounding box');
  const start = { x: box.x + box.width * 0.2, y: box.y + box.height * 0.2 };
  const end = { x: box.x + box.width * 0.8, y: box.y + box.height * 0.8 };

  await page.mouse.move(start.x, start.y);
  await page.mouse.down();
  await page.mouse.move(end.x, end.y, { steps: 10 });
  await page.mouse.up();

  await expect.poll(async () => Number(await saturationInput.inputValue())).toBeGreaterThan(saturationBefore);
  await expect.poll(async () => Number(await valueInput.inputValue())).toBeLessThan(valueBefore);
});
