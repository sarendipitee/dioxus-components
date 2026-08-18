import { test, expect } from "@playwright/test";

const route = "/components/hover_card";

function hoverCard(page: import("@playwright/test").Page) {
  const trigger = page.getByRole("button", { name: "Dioxus" });
  const tooltip = page.getByRole("tooltip");
  return { trigger, tooltip };
}

test.beforeEach(async ({ page }) => {
  await page.goto(route);
});

test("hover card is initially closed", async ({ page }) => {
  const { tooltip } = hoverCard(page);

  await expect(tooltip).toHaveCount(0);
});

test("focus opens the hover card and blur closes it", async ({ page }) => {
  const { trigger, tooltip } = hoverCard(page);

  await trigger.focus();
  await expect(tooltip).toBeVisible();

  await trigger.blur();
  await expect(tooltip).toHaveCount(0);
});

test("aria-describedby references the tooltip only while open", async ({
  page,
}) => {
  const { trigger, tooltip } = hoverCard(page);

  await expect(trigger).not.toHaveAttribute("aria-describedby", /.+/);

  await trigger.focus();
  await expect(tooltip).toBeVisible();
  const tooltipId = await tooltip.getAttribute("id");
  expect(tooltipId).toBeTruthy();
  await expect(trigger).toHaveAttribute("aria-describedby", tooltipId!);

  await trigger.blur();
  await expect(tooltip).toHaveCount(0);
  await expect(trigger).not.toHaveAttribute("aria-describedby", /.+/);
});

test("pointer entry opens the hover card and pointer exit closes it", async ({
  page,
}) => {
  const { trigger, tooltip } = hoverCard(page);

  await trigger.hover();
  await expect(tooltip).toBeVisible();

  await page
    .locator("#dx-preview-block-root")
    .hover({ position: { x: 2, y: 2 } });
  await expect(tooltip).toHaveCount(0);
});
