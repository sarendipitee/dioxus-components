import { test, expect, type Locator, type Page } from "@playwright/test";

async function sliderTrackPoint(track: Locator, frac: number) {
  const box = await track.boundingBox();
  if (!box) throw new Error("slider track has no bounding box");
  return { x: box.x + box.width * frac, y: box.y + box.height / 2 };
}

async function clickSliderTrack(page: Page, track: Locator, frac: number) {
  const point = await sliderTrackPoint(track, frac);
  await page.mouse.click(point.x, point.y);
}

function sliderGroup(page: Page, name: string | RegExp) {
  return page
    .getByRole("slider", { name })
    .first()
    .locator(
      'xpath=ancestor::*[@role="group" and @data-orientation="horizontal"][1]',
    );
}

function sliderTrack(slider: Locator) {
  return slider
    .locator('div[data-orientation="horizontal"]:has([role="slider"])')
    .first();
}

test("dynamic min/max", async ({ page }) => {
  await page.goto("/components/slider/block#dynamic_range", {
    timeout: 30 * 1000,
  });
  const thumb = page.getByRole("slider", { name: "Dynamic Range Slider" });

  // Initial state: percentage mode (0-100)
  await expect(thumb).toHaveAttribute("aria-valuemin", "0");
  await expect(thumb).toHaveAttribute("aria-valuemax", "100");

  // Switch to absolute number mode
  await page.getByRole("switch", { name: "Percentage" }).click();

  // Should now be absolute mode (0-1000)
  await expect(thumb).toHaveAttribute("aria-valuemin", "0");
  await expect(thumb).toHaveAttribute("aria-valuemax", "1000");

  // Click back to percentage mode
  await page.getByRole("switch", { name: "Percentage" }).click();

  // Should be back to percentage mode (0-100)
  await expect(thumb).toHaveAttribute("aria-valuemin", "0");
  await expect(thumb).toHaveAttribute("aria-valuemax", "100");
});

test("range two thumbs", async ({ page }) => {
  await page.goto("/components/slider/block#range", { timeout: 30 * 1000 });
  const thumbs = page.getByRole("slider", { name: "Range Slider" });
  await expect(thumbs).toHaveCount(2);
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);

  // Initial values
  await expect(t0).toHaveAttribute("aria-valuenow", "20");
  await expect(t1).toHaveAttribute("aria-valuenow", "80");

  // Per-thumb ARIA bounds reflect the live neighbor constraint
  await expect(t0).toHaveAttribute("aria-valuemax", "80");
  await expect(t1).toHaveAttribute("aria-valuemin", "20");

  // Keyboard nudges thumb 0
  await t0.focus();
  await page.keyboard.press("ArrowRight");
  await expect(t0).toHaveAttribute("aria-valuenow", "21");

  // Spam right past thumb 1's value — thumb 0 must clamp at 80, not cross
  for (let i = 0; i < 200; i++) await page.keyboard.press("ArrowRight");
  await expect(t0).toHaveAttribute("aria-valuenow", "80");
  await expect(t1).toHaveAttribute("aria-valuenow", "80");
  // After clamping, thumb 1's lower bound moves up to thumb 0's value
  await expect(t1).toHaveAttribute("aria-valuemin", "80");
});

test("range thumbs recover from collision", async ({ page }) => {
  await page.goto("/components/slider/block#range", { timeout: 30 * 1000 });
  const thumbs = page.getByRole("slider", { name: "Range Slider" });
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);

  // Drive both thumbs to 80
  await t0.focus();
  for (let i = 0; i < 200; i++) await page.keyboard.press("ArrowRight");
  await expect(t0).toHaveAttribute("aria-valuenow", "80");
  await expect(t1).toHaveAttribute("aria-valuenow", "80");

  // Thumb 1 must still be movable up; once it does, thumb 0's max should follow
  await t1.focus();
  await page.keyboard.press("ArrowRight");
  await expect(t1).toHaveAttribute("aria-valuenow", "81");
  await expect(t0).toHaveAttribute("aria-valuemax", "81");

  // And thumb 0 must still be movable down
  await t0.focus();
  await page.keyboard.press("ArrowLeft");
  await expect(t0).toHaveAttribute("aria-valuenow", "79");
  await expect(t1).toHaveAttribute("aria-valuemin", "79");
});

test("range track click activates closest thumb", async ({ page }) => {
  await page.goto("/components/slider/block#range", { timeout: 30 * 1000 });
  const thumbs = page.getByRole("slider", { name: "Range Slider" });
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);
  const slider = sliderGroup(page, "Range Slider");
  const track = sliderTrack(slider);

  await expect(t0).toHaveAttribute("aria-valuenow", "20");
  await expect(t1).toHaveAttribute("aria-valuenow", "80");

  // Click near the right edge — should activate thumb 1, jumping it close to 100
  await clickSliderTrack(page, track, 0.95);
  await expect(t0).toHaveAttribute("aria-valuenow", "20");
  await expect
    .poll(async () => Number(await t1.getAttribute("aria-valuenow")))
    .toBeGreaterThan(80);

  // Click near the left edge — should activate thumb 0, jumping it close to 0
  await clickSliderTrack(page, track, 0.05);
  await expect
    .poll(async () => Number(await t0.getAttribute("aria-valuenow")))
    .toBeLessThan(20);
});

test("range collided thumbs split by click direction", async ({ page }) => {
  await page.goto("/components/slider/block#range", { timeout: 30 * 1000 });
  const thumbs = page.getByRole("slider", { name: "Range Slider" });
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);
  const slider = sliderGroup(page, "Range Slider");
  const track = sliderTrack(slider);

  // Collide both thumbs at 80
  await t0.focus();
  for (let i = 0; i < 200; i++) await page.keyboard.press("ArrowRight");
  await expect(t0).toHaveAttribute("aria-valuenow", "80");
  await expect(t1).toHaveAttribute("aria-valuenow", "80");

  // Clicking to the RIGHT of the collision must activate thumb 1 (not thumb 0,
  // which would otherwise win the distance tie and leave thumb 1 stranded).
  await clickSliderTrack(page, track, 0.95);
  await expect(t0).toHaveAttribute("aria-valuenow", "80");
  await expect
    .poll(async () => Number(await t1.getAttribute("aria-valuenow")))
    .toBeGreaterThan(80);
});

test("range collided thumbs drag left from just below collision", async ({
  page,
}) => {
  await page.goto("/components/slider/block#range", { timeout: 30 * 1000 });
  const thumbs = page.getByRole("slider", { name: "Range Slider" });
  const t0 = thumbs.nth(0);
  const t1 = thumbs.nth(1);
  const slider = sliderGroup(page, "Range Slider");
  const track = sliderTrack(slider);

  // Collide both thumbs at 80
  await t0.focus();
  for (let i = 0; i < 200; i++) await page.keyboard.press("ArrowRight");
  await expect(t0).toHaveAttribute("aria-valuenow", "80");
  await expect(t1).toHaveAttribute("aria-valuenow", "80");

  // 79.6 snaps to 80. Thumb selection must still see the raw 79.6 position,
  // otherwise thumb 1 wins the tie and leftward dragging is clamped at 80.
  const start = await sliderTrackPoint(track, 0.796);
  const end = await sliderTrackPoint(track, 0.7);
  await page.mouse.move(start.x, start.y);
  await page.mouse.down();
  await page.mouse.move(end.x, end.y, { steps: 5 });
  await page.mouse.up();

  await expect(t0).toHaveAttribute("aria-valuenow", "70");
  await expect(t1).toHaveAttribute("aria-valuenow", "80");
});
