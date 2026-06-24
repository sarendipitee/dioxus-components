import { test, expect, type Page } from "@playwright/test";

const URL = "/components/table_of_contents";
const LOAD_TIMEOUT = 20 * 60 * 1000;

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
  const { links } = await loadPage(page);

  const overview = links.locator('[href="#overview"]');
  await expect(overview).toHaveAttribute("data-depth", "2");

  const config = links.locator('[href="#configuration"]');
  await expect(config).toHaveAttribute("data-depth", "3");

  const offsets = links.locator('[href="#offsets"]');
  await expect(offsets).toHaveAttribute("data-depth", "4");
});

test("active state changes on scroll", async ({ page }) => {
  const { links } = await loadPage(page);

  const scrollRegion = page.locator("[data-toc-demo-scroll-region]");

  const overviewLink = links.locator('[href="#overview"]');
  const installationLink = links.locator('[href="#installation"]');

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
