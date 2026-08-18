import { test, expect, type Page } from "@playwright/test";

const controlledDemoUrl = "/components/pagination/block#controlled";

// The component page renders every pagination demo, so scope to the controlled one.
const root = (page: Page) => page.getByTestId("pagination-controlled-demo");

const caption = (page: Page) =>
  root(page).getByTestId("pagination-controlled-value");

// PaginationLink renders an <a> without href, so it has no implicit link role;
// target the slot by its aria-label instead.
const pageLink = (page: Page, n: number) =>
  root(page).locator(
    `[data-slot="pagination-link"][aria-label="Go to page ${n}"]`,
  );

const control = (page: Page, name: string) =>
  root(page).locator(`[data-slot="pagination-link"][aria-label="${name}"]`);

test("renders the data-backed range and reflects the active page", async ({
  page,
}) => {
  await page.goto(controlledDemoUrl, { timeout: 30 * 1000 });
  await page.waitForLoadState("networkidle");

  await expect(caption(page)).toHaveText("Page 3 of 10");

  // active=3 -> 1 2 3 4 5 … 10
  await expect(pageLink(page, 3)).toHaveAttribute("aria-current", "page");
  await expect(pageLink(page, 5)).toBeVisible();
  await expect(pageLink(page, 1)).toBeVisible();
  await expect(pageLink(page, 10)).toBeVisible();
  // The gap between 5 and 10 is truncated.
  await expect(pageLink(page, 7)).toHaveCount(0);
});

test("clicking a page and the next control updates the controlled state", async ({
  page,
}) => {
  await page.goto(controlledDemoUrl, { timeout: 30 * 1000 });
  await page.waitForLoadState("networkidle");

  await pageLink(page, 5).click();
  await expect(caption(page)).toHaveText("Page 5 of 10");
  await expect(pageLink(page, 5)).toHaveAttribute("aria-current", "page");

  await control(page, "Go to next page").click();
  await expect(caption(page)).toHaveText("Page 6 of 10");
  await expect(pageLink(page, 6)).toHaveAttribute("aria-current", "page");
});

test("edge controls jump to the boundaries and disable there", async ({
  page,
}) => {
  await page.goto(controlledDemoUrl, { timeout: 30 * 1000 });
  await page.waitForLoadState("networkidle");

  await control(page, "Go to last page").click();
  await expect(caption(page)).toHaveText("Page 10 of 10");
  await expect(control(page, "Go to next page")).toHaveAttribute(
    "aria-disabled",
    "true",
  );
  await expect(control(page, "Go to next page")).toHaveAttribute(
    "tabindex",
    "-1",
  );
  await expect(control(page, "Go to last page")).toHaveAttribute(
    "aria-disabled",
    "true",
  );
  await expect(control(page, "Go to last page")).toHaveAttribute(
    "tabindex",
    "-1",
  );

  await control(page, "Go to first page").click();
  await expect(caption(page)).toHaveText("Page 1 of 10");
  await expect(control(page, "Go to previous page")).toHaveAttribute(
    "aria-disabled",
    "true",
  );
  await expect(control(page, "Go to previous page")).toHaveAttribute(
    "tabindex",
    "-1",
  );
});

test("exposes the labeled navigation and forwarded global attributes", async ({
  page,
}) => {
  await page.goto(controlledDemoUrl, { timeout: 30 * 1000 });
  await page.waitForLoadState("networkidle");

  const nav = page.getByTestId("pagination-controlled-nav");
  await expect(nav).toHaveRole("navigation");
  await expect(nav).toHaveAccessibleName("Account pages");
  await expect(nav).toHaveAttribute("data-testid", "pagination-controlled-nav");
});

test("previous and next controls disable at their boundaries", async ({
  page,
}) => {
  await page.goto(controlledDemoUrl, { timeout: 30 * 1000 });
  await page.waitForLoadState("networkidle");

  const previous = control(page, "Go to previous page");
  const next = control(page, "Go to next page");
  await expect(previous).toHaveAttribute("aria-disabled", "false");
  await expect(previous).toHaveAttribute("tabindex", "0");

  await control(page, "Go to first page").click();
  await expect(caption(page)).toHaveText("Page 1 of 10");
  await expect(previous).toHaveAttribute("aria-disabled", "true");
  await expect(previous).toHaveAttribute("tabindex", "-1");
  await next.click();
  await expect(caption(page)).toHaveText("Page 2 of 10");

  await control(page, "Go to last page").click();
  await expect(caption(page)).toHaveText("Page 10 of 10");
  await expect(next).toHaveAttribute("aria-disabled", "true");
  await expect(next).toHaveAttribute("tabindex", "-1");
  await previous.click();
  await expect(caption(page)).toHaveText("Page 9 of 10");
});

test("keyboard activation updates the controlled page and callback count", async ({
  page,
}) => {
  await page.goto(controlledDemoUrl, { timeout: 30 * 1000 });
  await page.waitForLoadState("networkidle");

  const changes = root(page).getByTestId("pagination-controlled-changes");
  const target = pageLink(page, 5);
  await expect(changes).toHaveText("Changes: 0");
  await target.focus();
  await expect(target).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(caption(page)).toHaveText("Page 5 of 10");
  await expect(changes).toHaveText("Changes: 1");

  await pageLink(page, 6).focus();
  await page.keyboard.press("Space");
  await expect(caption(page)).toHaveText("Page 6 of 10");
  await expect(changes).toHaveText("Changes: 2");
});

test("disabled pagination controls stay inert and do not invoke the callback", async ({
  page,
}) => {
  await page.goto(controlledDemoUrl, { timeout: 30 * 1000 });
  await page.waitForLoadState("networkidle");

  const changes = root(page).getByTestId("pagination-controlled-changes");
  const disabled = root(page).getByTestId("pagination-controlled-disabled");
  await expect(changes).toHaveText("Changes: 0");
  await disabled.click();

  const controls = root(page).locator('[data-slot="pagination-link"]');
  await expect(controls).toHaveCount(10);
  for (let index = 0; index < (await controls.count()); index += 1) {
    await expect(controls.nth(index)).toHaveAttribute("aria-disabled", "true");
    await expect(controls.nth(index)).toHaveAttribute("tabindex", "-1");
  }

  await control(page, "Go to page 5").click({ force: true });
  await expect(caption(page)).toHaveText("Page 3 of 10");
  await expect(changes).toHaveText("Changes: 0");
  await controls.nth(0).focus();
  await page.keyboard.press("Enter");
  await page.keyboard.press("Space");
  await expect(caption(page)).toHaveText("Page 3 of 10");
  await expect(changes).toHaveText("Changes: 0");
});
