import { expect, test, type Page } from "@playwright/test";

const TYPOGRAPHY_DEMO_URL = "/components/typography/block#main";

async function gotoTypographyDemo(page: Page) {
  await page.goto(TYPOGRAPHY_DEMO_URL, {
    timeout: 30 * 1000,
    waitUntil: "load",
  });
}

test("typography preserves semantic heading hierarchy independently of visual variants", async ({ page }) => {
  await gotoTypographyDemo(page);

  const preview = page.locator("#dx-preview-block-root");
  await expect(preview.getByRole("heading", { level: 2, name: "Shared typography" })).toBeVisible();
  await expect(preview.getByRole("heading", { level: 3, name: "Semantic h3 with medium visual size" })).toBeVisible();

  const fixture = preview.locator("#typography-semantic-fixture");
  await expect(fixture.getByRole("heading", { level: 1, name: "Typography level one heading" })).toHaveText("Heading level one");
  await expect(fixture.getByRole("heading", { level: 3, name: "Heading level three" })).toBeVisible();
  await expect(fixture.getByRole("heading", { level: 6, name: "Heading level six" })).toBeVisible();
});

test("text selects supported paragraph, span, div, and label elements", async ({ page }) => {
  await gotoTypographyDemo(page);

  const preview = page.locator("#dx-preview-block-root");
  await expect(preview.locator("p", { hasText: "Use Text and Heading" })).toBeVisible();
  await expect(preview.locator("span", { hasText: "Inline accent span" })).toBeVisible();

  const fixture = preview.locator("#typography-semantic-fixture");
  await expect(fixture.locator('div[data-typography-text-element="div"]')).toHaveText(
    "Text rendered as a semantic division with forwarded global attributes.",
  );
  const input = fixture.getByLabel("Fixture input label");
  await expect(input).toHaveAttribute("id", "typography-semantic-input");
  await expect(input).toHaveAttribute("data-typography-input", "associated");
});

test("typography forwards global attributes while retaining supplied classes", async ({ page }) => {
  await gotoTypographyDemo(page);

  const fixture = page.locator("#dx-preview-block-root #typography-semantic-fixture");
  const heading = fixture.locator("#typography-heading-h1");
  await expect(heading).toHaveAttribute("data-typography-heading-level", "h1");
  await expect(heading).toHaveClass(/typography-semantic-heading/);
  await expect(heading).toHaveAttribute("aria-label", "Typography level one heading");

  const division = fixture.locator("#typography-semantic-div");
  await expect(division).toHaveAttribute("data-typography-text-element", "div");
  await expect(division).toHaveClass(/typography-semantic-div/);
  await expect(division).toHaveAttribute("aria-label", "Typography semantic division");
});

test("typography exposes truncation and line-clamp state on the rendered elements", async ({ page }) => {
  await gotoTypographyDemo(page);

  const fixture = page.locator("#dx-preview-block-root #typography-semantic-fixture");
  const truncated = fixture.locator('p[data-typography-text-element="paragraph"]');
  await expect(truncated).toHaveCSS("overflow", "hidden");
  await expect(truncated).toHaveCSS("text-overflow", "ellipsis");
  await expect(truncated).toHaveCSS("white-space", "nowrap");

  const clamped = fixture.locator('span[data-typography-text-element="span"]');
  await expect(clamped).toHaveCSS("overflow", "hidden");
  await expect(clamped).toHaveCSS("-webkit-line-clamp", "2");
});