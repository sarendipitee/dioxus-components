import { expect, test, type Locator, type Page } from "@playwright/test";

const CARD_DEMO_URL = "/components/card/block#main";

async function gotoCardDemo(page: Page) {
  await page.goto(CARD_DEMO_URL, {
    timeout: 20 * 60 * 1000,
    waitUntil: "load",
  });
}

async function computedColorForCssColor(locator: Locator, color: string) {
  return locator.evaluate((element: Element, cssColor: string) => {
    const probe = document.createElement("span");
    probe.style.color = cssColor;
    probe.style.position = "absolute";
    probe.style.visibility = "hidden";
    element.appendChild(probe);
    const computedColor = getComputedStyle(probe).color;
    probe.remove();
    return computedColor;
  }, color);
}

test("card title and description use shared typography roles", async ({ page }) => {
  await gotoCardDemo(page);

  const cards = page.locator('#dx-preview-block-root [data-slot="card"]');
  const shorthandCard = cards.nth(0);
  const wrapperCard = cards.nth(1);

  await assertCardTypography(shorthandCard, "P");
  await assertCardTypography(wrapperCard, "DIV");
});

async function assertCardTypography(card: Locator, descriptionTagName: "P" | "DIV") {
  const title = card.locator('[data-slot="card-title"]');
  const description = card.locator('[data-slot="card-description"]');

  await expect(card).toBeVisible();
  await expect(title).toHaveCount(1);
  await expect(description).toHaveCount(1);
  await expect(title).toHaveJSProperty("tagName", "H3");
  await expect(description).toHaveJSProperty("tagName", descriptionTagName);
  await expect(title).toHaveClass(/dx_heading/);
  await expect(title).toHaveAttribute("data-size", "md");
  await expect(title).toHaveAttribute("data-weight", "semibold");
  await expect(title).toHaveAttribute("data-tone", "default");
  await expect(title).toHaveAttribute("data-wrap", "wrap");
  await expect(title).toHaveAttribute("data-truncate", "false");
  await expect(description).toHaveClass(/dx_text/);
  await expect(description).toHaveAttribute("data-size", "sm");
  await expect(description).toHaveAttribute("data-tone", "surface-muted");
  await expect(description).toHaveAttribute("data-weight", "inherit");
  await expect(description).toHaveAttribute("data-wrap", "wrap");
  await expect(description).toHaveAttribute("data-truncate", "false");
  await expect(title.locator("h1,h2,h3,h4,h5,h6,p")).toHaveCount(0);
  await expect(description.locator("h1,h2,h3,h4,h5,h6,p")).toHaveCount(0);

  const [cardColor, titleColor, descriptionColor, surfaceMutedColor, titleMetrics] = await Promise.all([
    card.evaluate((element) => getComputedStyle(element).color),
    title.evaluate((element) => getComputedStyle(element).color),
    description.evaluate((element) => getComputedStyle(element).color),
    computedColorForCssColor(card, "var(--surface-muted-fg)"),
    title.evaluate((element) => {
      const style = getComputedStyle(element);
      return {
        fontSize: Number.parseFloat(style.fontSize),
        lineHeight: Number.parseFloat(style.lineHeight),
      };
    }),
  ]);
  expect(titleColor).toBe(cardColor);
  expect(descriptionColor).toBe(surfaceMutedColor);
  expect(titleMetrics.lineHeight).toBeCloseTo(titleMetrics.fontSize, 5);
}
