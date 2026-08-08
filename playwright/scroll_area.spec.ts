import { test, expect } from "@playwright/test";

const geometry = (element: HTMLElement) => {
  const style = getComputedStyle(element);
  return {
    overflowX: style.overflowX,
    overflowY: style.overflowY,
    scrollbarWidth: style.scrollbarWidth,
    horizontalRange: element.scrollWidth - element.clientWidth,
    verticalRange: element.scrollHeight - element.clientHeight,
  };
};

test.beforeEach(async ({ page }) => {
  await page.goto("/components/scroll_area");
});

test("forwards global attributes and exposes the vertical auto viewport", async ({ page }) => {
  const area = page.getByRole("region", { name: "Vertical auto scroll area" }).first();

  await expect(area).toBeVisible();
  await expect(area).toHaveAttribute("id", "scroll-area-forwarded-id");
  await expect(area).toHaveAttribute("data-scroll-direction", "vertical");
  await expect(area).toHaveAttribute("tabindex", "0");

  const state = await area.evaluate(geometry);
  expect(state.overflowX).toBe("hidden");
  expect(state.overflowY).toBe("auto");
  expect(state.horizontalRange).toBe(0);
  expect(state.verticalRange).toBeGreaterThan(0);
});

test("scrolls the focused vertical viewport with the keyboard", async ({ page }) => {
  const area = page.getByTestId("scroll-area-vertical-auto").first();

  await area.focus();
  await expect(area).toBeFocused();
  await expect
    .poll(async () => {
      await area.press("ArrowDown");
      return area.evaluate((element) => element.scrollTop);
    })
    .toBeGreaterThan(0);
});

test("uses an always-visible horizontal scrolling axis", async ({ page }) => {
  const area = page.getByRole("region", { name: "Horizontal always scroll area" }).first();

  await expect(area).toHaveAttribute("data-scroll-direction", "horizontal");
  const state = await area.evaluate(geometry);
  expect(state.overflowX).toBe("scroll");
  expect(state.overflowY).toBe("hidden");
  expect(state.horizontalRange).toBeGreaterThan(0);

  await area.hover();
  await expect
    .poll(async () => {
      await page.mouse.wheel(160, 0);
      return area.evaluate((element) => element.scrollLeft);
    })
    .toBeGreaterThan(0);
});

test("hides scrollbars while retaining both scrolling axes", async ({ page }) => {
  const area = page.getByRole("region", { name: "Both-axis hidden scroll area" }).first();

  await expect(area).toHaveAttribute("data-scroll-direction", "both");
  const state = await area.evaluate(geometry);
  expect(state.overflowX).toBe("scroll");
  expect(state.overflowY).toBe("scroll");
  expect(state.scrollbarWidth).toBe("none");
  expect(state.horizontalRange).toBeGreaterThan(0);
  expect(state.verticalRange).toBeGreaterThan(0);

  await area.hover();
  await expect
    .poll(async () => {
      await page.mouse.wheel(120, 160);
      return area.evaluate(
        (element) => element.scrollLeft > 0 && element.scrollTop > 0
      );
    })
    .toBe(true);
});
