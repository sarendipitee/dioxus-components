import { test, expect, type Page } from "@playwright/test";

const controlledUrl = "/component/?name=pagination&variant=controlled&";

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
    await page.goto(controlledUrl, { timeout: 20 * 60 * 1000 });
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
    await page.goto(controlledUrl, { timeout: 20 * 60 * 1000 });
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
    await page.goto(controlledUrl, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("networkidle");

    await control(page, "Go to last page").click();
    await expect(caption(page)).toHaveText("Page 10 of 10");
    await expect(control(page, "Go to next page")).toHaveAttribute(
        "aria-disabled",
        "true",
    );
    await expect(control(page, "Go to last page")).toHaveAttribute(
        "aria-disabled",
        "true",
    );

    await control(page, "Go to first page").click();
    await expect(caption(page)).toHaveText("Page 1 of 10");
    await expect(control(page, "Go to previous page")).toHaveAttribute(
        "aria-disabled",
        "true",
    );
});
