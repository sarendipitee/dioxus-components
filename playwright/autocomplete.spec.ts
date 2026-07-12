import { expect, test } from "@playwright/test";

const URL = "/components/autocomplete/block#server_backed";

test("server-backed autocomplete replaces loading state with matching results", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const demo = page.locator("#dx-preview-block-root");
    const search = demo.getByRole("combobox", {
        name: "Server-backed people search",
    });
    await expect(search).toBeVisible();
    await search.fill("ada");

    await expect(demo.getByRole("status")).toHaveText("Searching server...");

    const ada = page.getByRole("option", { name: /Ada Lovelace/ });
    await expect(ada).toBeVisible();
    await expect(demo.getByText("Server returned 1 result(s).")).toBeVisible();

    await ada.click();
    await expect(search).toHaveValue("Ada Lovelace");
    await expect(demo.getByText("Selected: ada-lovelace")).toBeVisible();
    await expect(page.locator("[role='listbox'][data-state='open']")).toHaveCount(0);
});
