import { test, expect, type Page } from "@playwright/test";

const URL = "/components/table_of_contents";
const LOAD_TIMEOUT = 30 * 1000;

async function loadPage(page: Page) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT, waitUntil: "networkidle" });

  const nav = page.locator("nav[data-table-of-contents]").first();
  await expect(nav).toBeVisible({ timeout: 30000 });

  const links = nav.locator("a");
  await expect(links.first()).toBeVisible();

  return { nav, links };
}

test("renders styled TOC nav with CSS module class", async ({ page }) => {
  const { nav } = await loadPage(page);

  await expect(nav).toHaveAttribute("data-table-of-contents", "true");

  const allClasses = await nav.getAttribute("class");
  expect(allClasses).toContain("dx_table_of_contents");
});

test("exposes accessible navigation identity and active current state", async ({ page }) => {
  const { nav } = await loadPage(page);

  await expect(nav).toHaveAttribute("aria-label", "On this page");
  await expect(nav).toHaveAttribute("data-testid", "table-of-contents");

  await expect(nav.locator('a[href="#overview"]')).toHaveAttribute(
    "aria-current",
    "location",
  );
  await expect(nav.locator('a[href="#installation"]')).not.toHaveAttribute(
    "aria-current",
  );
});

test("indents nested headings by their hierarchy depth", async ({ page }) => {
  const { nav } = await loadPage(page);

  for (const [href, marginLeft] of [
    ["#overview", "20px"],
    ["#configuration", "40px"],
    ["#offsets", "60px"],
  ] as const) {
    await expect(nav.locator(`a[href="${href}"]`)).toHaveCSS(
      "margin-left",
      marginLeft,
    );
  }
});

test("clicking a TOC link updates the hash and scrolls the internal region", async ({ page }) => {
  const { nav } = await loadPage(page);
  const scrollRegion = page.locator("[data-toc-demo-scroll-region]");

  const link = nav.locator('a[href="#installation"]');
  const linkBox = await link.boundingBox();
  expect(linkBox).not.toBeNull();
  await page.mouse.click(
    linkBox!.x + linkBox!.width / 2,
    linkBox!.y + linkBox!.height / 2,
  );
  await expect(page).toHaveURL(/#installation$/);
  await expect
    .poll(async () => scrollRegion.evaluate((element) => element.scrollTop))
    .toBeGreaterThan(0);

});

test("supports keyboard focus-visible navigation and Enter activation", async ({ page }) => {
  const { nav } = await loadPage(page);
  const links = nav.locator("a");

  await links.first().focus();
  await page.keyboard.press("Tab");
  await expect(links.nth(1)).toBeFocused();
  await expect(links.nth(1)).toHaveCSS("box-shadow", /rgb\(/);
  await page.keyboard.press("Enter");
  await expect(page).toHaveURL(/#installation$/);
});

test("renders links for every heading", async ({ page }) => {
  const { links } = await loadPage(page);

  const expectedHrefs = [
    "#overview",
    "#installation",
    "#configuration",
    "#offsets",
    "#api",
    "#reinitialization",
    "#styling",
    "#accessibility",
    "#usage-notes",
  ];

  const hrefs = await links.evaluateAll((els) =>
    els.map((el) => el.getAttribute("href")),
  );

  for (const href of expectedHrefs) {
    expect(hrefs).toContain(href);
  }
});

test("links have data-depth attribute", async ({ page }) => {
  const { nav } = await loadPage(page);

  const overview = nav.locator('a[href="#overview"]');
  await expect(overview).toHaveAttribute("data-depth", "2");

  const config = nav.locator('a[href="#configuration"]');
  await expect(config).toHaveAttribute("data-depth", "3");

  const offsets = nav.locator('a[href="#offsets"]');
  await expect(offsets).toHaveAttribute("data-depth", "4");
});

test("active state changes on scroll", async ({ page }) => {
  const { nav } = await loadPage(page);

  const scrollRegion = page.locator("[data-toc-demo-scroll-region]");

  const overviewLink = nav.locator('a[href="#overview"]');
  const installationLink = nav.locator('a[href="#installation"]');

  await expect(overviewLink).toHaveAttribute("data-active", "true");

  await scrollRegion.evaluate((el) => {
    const heading = el.querySelector("#installation");
    if (heading) heading.scrollIntoView({ block: "start" });
    el.dispatchEvent(new Event("scroll"));
  });

  await expect(installationLink).toHaveAttribute("data-active", "true", {
    timeout: 5000,
  });
});

test("links are styled with theme tokens", async ({ page }) => {
  const { nav } = await loadPage(page);

  const link = nav.locator("a").first();
  await expect(link).toHaveCSS("text-decoration", "none");
  await expect(link).toHaveCSS("border-radius", "8px");
  await expect(link).toHaveCSS("color", "rgb(113, 113, 113)");
  await expect(link).toHaveCSS("font-size", "14px");
});

test("active link has accent-subtle styling", async ({ page }) => {
  const { nav } = await loadPage(page);

  const activeLink = nav.locator('a[data-active="true"]').first();
  await expect(activeLink).toBeVisible();
  await expect(activeLink).toHaveCSS("font-weight", "500");
});
