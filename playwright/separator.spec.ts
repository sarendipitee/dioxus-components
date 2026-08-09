import { expect, test, type Page } from "@playwright/test";

const SEPARATOR_URL = "/components/separator";

async function gotoSeparatorDemo(page: Page) {
  await page.goto(SEPARATOR_URL, { timeout: 30_000, waitUntil: "load" });
}

function separator(page: Page) {
  return page.locator("#separator-fixture");
}

test("exposes semantic horizontal separator and forwarded attributes", async ({
  page,
}) => {
  await gotoSeparatorDemo(page);
  const fixture = separator(page);

  await expect(fixture).toHaveRole("separator");
  await expect(fixture).toHaveAttribute("id", "separator-fixture");
  await expect(fixture).toHaveAttribute("aria-orientation", "horizontal");
  await expect(fixture).toHaveAttribute("data-orientation", "horizontal");
  await expect(fixture).toHaveAttribute("data-separator-fixture", "reactive");
  await expect(fixture).toHaveAttribute("class", /separator-fixture/);
  await expect(fixture).toHaveAttribute("title", "Reactive separator fixture");
  await expect(fixture).toHaveCSS("height", "1px");
  await expect(fixture).toHaveCSS("background-color", /.+/);
  await expect(page.locator("#separator-status")).toHaveAttribute(
    "aria-live",
    "polite",
  );
  await expect(page.locator("#separator-status")).toHaveText(
    "orientation: horizontal; decorative: false",
  );
});

test("reactively updates vertical orientation and decorative semantics", async ({
  page,
}) => {
  await gotoSeparatorDemo(page);
  const fixture = separator(page);

  await page
    .getByRole("button", { name: "Toggle orientation", exact: true })
    .click();
  await expect(fixture).toHaveAttribute("role", "separator");
  await expect(fixture).toHaveAttribute("aria-orientation", "vertical");
  await expect(fixture).toHaveAttribute("data-orientation", "vertical");
  await expect(fixture).toHaveCSS("width", "1px");
  await expect(page.locator("#separator-status")).toHaveText(
    "orientation: vertical; decorative: false",
  );

  await page
    .getByRole("button", { name: "Toggle decorative", exact: true })
    .click();
  await expect(fixture).toHaveAttribute("role", "none");
  await expect(fixture).not.toHaveAttribute("aria-orientation");
  await expect(fixture).toHaveAttribute("data-orientation", "vertical");
  await expect(page.locator("#separator-status")).toHaveText(
    "orientation: vertical; decorative: true",
  );

  await page
    .getByRole("button", { name: "Toggle decorative", exact: true })
    .click();
  await expect(fixture).toHaveAttribute("role", "separator");
  await expect(fixture).toHaveAttribute("aria-orientation", "vertical");
  await expect(page.locator("#separator-status")).toHaveText(
    "orientation: vertical; decorative: false",
  );
});
