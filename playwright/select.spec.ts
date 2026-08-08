import { test, expect, type Locator, type Page } from "@playwright/test";

const fruitRoot = (page: Page) => page.getByTestId("fruit-select-root").first();
const fruitTrigger = (page: Page) => fruitRoot(page).locator("#fruit-select").first();
const listbox = (page: Page) => page.getByRole("listbox");
const option = (page: Page, name: string | RegExp) =>
    listbox(page).getByRole("option", { name });

const visualStyle = (target: Locator) =>
    target.evaluate((element) => {
        const style = getComputedStyle(element);
        return { backgroundColor: style.backgroundColor, borderColor: style.borderColor, boxShadow: style.boxShadow, color: style.color, outlineColor: style.outlineColor, outlineStyle: style.outlineStyle, outlineWidth: style.outlineWidth };
    });

test.beforeEach(async ({ page }) => {
    await page.goto("/components/select", { timeout: 30 * 1000 });
});

test("exposes trigger, listbox, groups, and option state through ARIA", async ({ page }) => {
    const trigger = fruitTrigger(page);
    await expect(trigger).toHaveAccessibleName("Fruit selection");
    await expect(trigger).toHaveAttribute("aria-haspopup", "listbox");
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    const controls = await trigger.getAttribute("aria-controls");
    expect(controls).toBeTruthy();
    await expect(fruitRoot(page)).toHaveAttribute("data-audit", "forwarded");
    await trigger.click();
    const menu = listbox(page);
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(menu).toHaveAttribute("id", controls!);
    await expect(menu).toHaveAttribute("aria-multiselectable", "false");
    const fruits = menu.getByRole("group", { name: "Fruits" });
    await expect(fruits).toHaveCount(1);
    await expect(menu.getByRole("group", { name: "Other" })).toHaveCount(1);
    await expect(fruits.getByRole("option")).toHaveCount(6);
    await expect(option(page, /Apple/i)).toHaveAttribute("aria-selected", "true");
    await expect(option(page, /^🍊 Orange$/i)).toHaveAttribute("aria-disabled", "true");
});

test("controlled selection updates value and calls callback exactly once", async ({ page }) => {
    const root = fruitRoot(page);
    const trigger = fruitTrigger(page);
    await expect(page.getByTestId("fruit-select-value")).toHaveText(/Apple/i);
    await expect(page.getByTestId("fruit-select-callback-count")).toHaveText("Callbacks: 0");
    await trigger.click();
    await option(page, /Banana/i).click();
    await expect(trigger).toContainText(/Banana/i);
    await expect(page.getByTestId("fruit-select-value")).toHaveText(/Banana/i);
    await expect(page.getByTestId("fruit-select-callback-count")).toHaveText("Callbacks: 1");
    await expect(listbox(page)).toHaveCount(0);
});

test("keyboard navigation skips disabled options and supports Home, End, and typeahead", async ({ page }) => {
    const trigger = fruitTrigger(page);
    await trigger.click();
    await page.keyboard.press("ArrowDown");
    await expect(option(page, /Apple/i)).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(option(page, /Banana/i)).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(option(page, /Orangeade/i)).toBeFocused();
    await page.keyboard.press("Home");
    await expect(option(page, /Apple/i)).toBeFocused();
    await page.keyboard.press("End");
    await expect(option(page, /Other/i)).toBeFocused();
    await page.keyboard.type("Straw");
    await expect(option(page, /Strawberry/i)).toBeFocused();
});

test("Escape restores trigger focus and outside click closes the menu", async ({ page }) => {
    const trigger = fruitTrigger(page);
    await trigger.click();
    await page.keyboard.press("Escape");
    await expect(listbox(page)).toHaveCount(0);
    await expect(trigger).toBeFocused();
    await trigger.click();
    await page.getByTestId("select-outside").click();
    await expect(listbox(page)).toHaveCount(0);
});

test("Tab closes the menu", async ({ page }) => {
    await fruitTrigger(page).click();
    await page.keyboard.press("Tab");
    await expect(listbox(page)).toHaveCount(0);
});

test("disabled group propagates aria-disabled and cannot be selected", async ({ page }) => {
    const root = page.getByTestId("disabled-group-select-root").first();
    await expect(root).toHaveCount(1);
    const trigger = root.locator("#disabled-group-select").first();
    await trigger.click();
    const group = listbox(page).getByRole("group", { name: "Unavailable fruits" });
    const disabledOptions = group.getByRole("option");
    await expect(group).toHaveCount(1);
    await expect(disabledOptions).toHaveCount(2);
    for (const item of await disabledOptions.all()) {
        await expect(item).toHaveAttribute("aria-disabled", "true");
        await expect(item).toBeDisabled();
    }
    await page.keyboard.press("ArrowDown");
    await expect(disabledOptions.first()).not.toBeFocused();
});

test("disabled root trigger cannot open", async ({ page }) => {
    const trigger = page.getByTestId("disabled-root-select-root").first().locator("#disabled-root-select").first();
    await expect(trigger).toHaveAccessibleName("Disabled root selection");
    await expect(trigger).toBeDisabled();
    await trigger.click({ force: true });
    await expect(listbox(page)).toHaveCount(0);
});

test("focused options have a visual cue", async ({ page }) => {
    const trigger = fruitTrigger(page);
    await trigger.click();
    const before = await visualStyle(option(page, /Banana/i));
    await page.keyboard.press("ArrowDown");
    await expect(option(page, /Apple/i)).toBeFocused();
    await expect.poll(() => visualStyle(option(page, /Apple/i))).not.toEqual(before);
});

test("multi-select retains its interaction coverage", async ({ page }) => {
    await page.goto("/components/select/block#multi", { timeout: 30 * 1000 });
    const trigger = page.getByRole("button").filter({ hasText: /Pepperoni|Mushroom|Onion/ });
    await trigger.click();
    const menu = listbox(page);
    await expect(menu).toHaveAttribute("aria-multiselectable", "true");
    const onion = menu.getByRole("option", { name: /Onion/i });
    await onion.click();
    await expect(menu).toHaveAttribute("data-state", "open");
    await expect(onion).toHaveAttribute("aria-selected", "true");
});
