import { test, expect, devices, type Locator, type Page } from "@playwright/test";

const URL = "/components/combobox";
const demoUrl = (demo: string) =>
    `/components/combobox/block#${demo}`;

const input = (page: Page) =>
    page.getByRole("combobox", { name: "Select framework" });

const content = (page: Page) =>
    page.locator("[role='listbox'][data-state='open']");

const list = (page: Page) =>
    page.locator("[role='listbox'][data-state='open']");

const visualStyle = (option: Locator) =>
    option.evaluate((element) => {
        const style = getComputedStyle(element);
        return {
            backgroundColor: style.backgroundColor,
            borderColor: style.borderColor,
            boxShadow: style.boxShadow,
            color: style.color,
            outlineColor: style.outlineColor,
            outlineStyle: style.outlineStyle,
            outlineWidth: style.outlineWidth,
        };
    });

test("opens from the focused input with the keyboard", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await expect(trigger).toBeVisible();
    await trigger.focus();
    await expect(trigger).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(content(page)).toBeVisible();
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(list(page).getByRole("option", { name: "Next.js" })).toHaveAttribute(
        "data-highlighted",
        "true",
    );
    const option = list(page).getByRole("option", { name: "Next.js" });
    await expect(trigger).toHaveAttribute("aria-activedescendant", await option.getAttribute("id"));
    await expect(option).toHaveAttribute("id", await trigger.getAttribute("aria-activedescendant"));
});

test("ArrowUp opens at the last option with a visual active item", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.focus();
    await page.keyboard.press("ArrowUp");

    const dioxus = list(page).getByRole("option", { name: "Dioxus" });
    const next = list(page).getByRole("option", { name: "Next.js" });
    const unhighlightedStyle = await visualStyle(next);

    await expect(content(page)).toBeVisible();
    await expect(dioxus).toHaveAttribute("data-highlighted", "true");
    await expect(next).toHaveAttribute("data-highlighted", "false");
    await expect.poll(() => visualStyle(dioxus)).not.toEqual(unhighlightedStyle);
    await expect(trigger).toBeFocused();
});

test("Home and End move active option and select it", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.focus();
    await page.keyboard.press("ArrowDown");

    const next = list(page).getByRole("option", { name: "Next.js" });
    const dioxus = list(page).getByRole("option", { name: "Dioxus" });

    await page.keyboard.press("End");
    await expect(dioxus).toHaveAttribute("data-highlighted", "true");
    await expect(trigger).toHaveAttribute(
        "aria-activedescendant",
        await dioxus.getAttribute("id"),
    );

    await page.keyboard.press("Home");
    await expect(next).toHaveAttribute("data-highlighted", "true");
    await expect(trigger).toHaveAttribute(
        "aria-activedescendant",
        await next.getAttribute("id"),
    );

    await page.keyboard.press("Enter");
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("Next.js");
});

test("filters and selects with the keyboard", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.click();
    await expect(content(page)).toBeVisible();

    await page.keyboard.type("sve");
    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    await expect(svelte).toBeVisible();

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("Enter");
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("SvelteKit");

    await trigger.click();
    await expect(svelte).toHaveAttribute("aria-selected", "true");

    await page.keyboard.press("Escape");
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("SvelteKit");
});

test("shows an empty state when no options match", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.click();
    await page.keyboard.type("zzz");

    await expect(list(page).getByText("No framework found.")).toBeVisible();
    await expect(list(page).getByRole("option")).toHaveCount(0);
});

test("arrow keys stay on visible filtered options", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.click();
    await page.keyboard.type("sve");
    await expect(trigger).toBeFocused();

    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    await expect(svelte).toBeVisible();
    await expect(svelte).not.toHaveAttribute("tabindex", /.+/);
    await expect(list(page)).not.toHaveAttribute("tabindex", /.+/);

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");
    await expect(trigger).toBeFocused();
    await expect(trigger).toHaveAttribute("aria-activedescendant", await svelte.getAttribute("id"));

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowUp");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");
});

test("keeps filtered options in source order", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.click();
    await page.keyboard.type("s");

    const next = list(page).getByRole("option", { name: "Next.js" });
    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    const solid = list(page).getByRole("option", { name: "SolidStart" });

    await expect(next).toBeVisible();
    await expect(svelte).toBeVisible();
    await expect(solid).toBeVisible();

    const nextBox = await next.boundingBox();
    const svelteBox = await svelte.boundingBox();
    expect(nextBox).not.toBeNull();
    expect(svelteBox).not.toBeNull();
    expect(nextBox!.y).toBeLessThan(svelteBox!.y);

    await page.keyboard.press("ArrowDown");
    await expect(next).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowUp");
    await expect(next).toHaveAttribute("data-highlighted", "true");
});

test("keeps filtered options during keyboard close animation", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.click();
    await page.keyboard.type("sve");

    const svelte = list(page).getByRole("option", { name: "SvelteKit" });
    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("Enter");

    const closingContent = page.locator("[role='listbox'][data-state='closed']");
    if (await closingContent.count()) {
        const closingList = closingContent.first();
        await expect(closingList).toBeVisible();
        await expect(closingList.getByRole("option", { name: "SvelteKit" })).toBeVisible();
        await expect(closingList.getByRole("option")).toHaveCount(1);
    }
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("SvelteKit");
});

test("clicking an option commits and closes", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.click();
    await list(page).getByRole("option", { name: "Dioxus" }).click();

    await expect(trigger).toHaveValue("Dioxus");
    await expect(content(page)).toHaveCount(0);
});

test("tabbing away closes the list", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = input(page);
    await trigger.click();
    await expect(content(page)).toBeVisible();

    await page.keyboard.press("Tab");
    await expect(content(page)).toHaveCount(0);
});

test("disabled options are exposed but skipped by keyboard selection", async ({ page }) => {
    await page.goto(demoUrl("disabled"), { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    await expect(page.getByRole("combobox", { name: "Disabled combobox" })).toBeDisabled();

    const trigger = page.getByRole("combobox", {
        name: "Framework with disabled option",
    });
    await trigger.click();

    const menu = list(page);
    const next = menu.getByRole("option", { name: "Next.js" });
    const svelte = menu.getByRole("option", { name: "SvelteKit" });
    const nuxt = menu.getByRole("option", { name: "Nuxt.js" });

    await expect(svelte).toHaveAttribute("aria-disabled", "true");

    await page.keyboard.press("ArrowDown");
    await expect(next).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowDown");
    await expect(svelte).toHaveAttribute("data-highlighted", "false");
    await expect(nuxt).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("ArrowUp");
    await expect(next).toHaveAttribute("data-highlighted", "true");
});

test("controlled value and controlled open stay in sync", async ({ page }) => {
    await page.goto(demoUrl("controlled"), { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = page.getByRole("combobox", { name: "Controlled framework" });
    const storedValue = page.getByTestId("combobox-controlled-value");

    await expect(trigger).toHaveValue("SvelteKit");
    await expect(storedValue).toHaveText("svelte");

    await page.getByRole("button", { name: "Set Astro" }).click();
    await expect(trigger).toHaveValue("Astro");
    await expect(storedValue).toHaveText("astro");

    await page.getByRole("button", { name: "Open", exact: true }).click();
    await expect(content(page)).toBeVisible();

    await list(page).getByRole("option", { name: "Dioxus" }).click();
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("Dioxus");
    await expect(storedValue).toHaveText("dioxus");
});

test("dynamic option mutation keeps list open, while an ordinary outside click dismisses it", async ({ page }) => {
    await page.goto(demoUrl("dynamic"), { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = page.getByRole("combobox", { name: "Dynamic framework" });
    const toggleSvelte = page.getByRole("button", { name: "Toggle SvelteKit" });
    await trigger.click();
    await page.keyboard.type("s");

    await expect(list(page).getByRole("option", { name: "SvelteKit" })).toBeVisible();
    await expect(list(page).getByRole("option", { name: "SolidStart" })).toBeVisible();
    await page.keyboard.press("ArrowDown");
    await expect(list(page).getByRole("option", { name: "Next.js" })).toHaveAttribute(
        "data-highlighted",
        "true",
    );
    await page.keyboard.press("ArrowDown");
    await expect(list(page).getByRole("option", { name: "SvelteKit" })).toHaveAttribute(
        "data-highlighted",
        "true",
    );

    await expect(trigger).toBeFocused();
    await toggleSvelte.click();
    await expect(list(page).getByRole("option", { name: "SvelteKit" })).toHaveCount(0);
    await expect(list(page).getByRole("option", { name: "SolidStart" })).toBeVisible();
    await expect(content(page)).toBeVisible();
    await expect(trigger).toBeFocused();

    await page.locator("body").dispatchEvent("pointerdown", { bubbles: true, cancelable: true });
    await expect(content(page)).toHaveCount(0);

    await trigger.click();
    await expect(content(page)).toBeVisible();
    await expect(trigger).toBeFocused();

    await page.keyboard.press("ArrowDown");
    const next = list(page).getByRole("option", { name: "Next.js" });
    await expect(next).toHaveAttribute("data-highlighted", "true");

    await page.keyboard.press("Enter");
    await expect(content(page)).toHaveCount(0);
    await expect(trigger).toHaveValue("Next.js");
});

test("delayed option arrival replaces empty state and supports keyboard selection", async ({ page }) => {
    await page.goto(demoUrl("dynamic"), { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = page.getByRole("combobox", { name: "Dynamic framework" });
    await expect(trigger).toBeVisible();
    await trigger.click();
    await expect(list(page)).toHaveAttribute(
        "style",
        /--overlay-z:\s*calc\(var\(--z-overlay-base\) \+ \d+ \* var\(--z-overlay-step\)\)/,
    );
    await page.keyboard.type("ada");

    await expect(content(page)).toHaveText("No framework found.");
    await page.getByRole("button", { name: "Load matching patient" }).click();
    await expect(page.getByTestId("dynamic-patient-loaded")).toBeVisible();

    const ada = list(page).getByRole("option", { name: "Ada Lovelace" });
    await expect(ada).toBeVisible();
    await expect(content(page).getByText("No framework found.")).toHaveCount(0);
    await expect(trigger).toBeFocused();

    await page.keyboard.press("ArrowDown");
    await expect(ada).toHaveAttribute("data-highlighted", "true");
    await page.keyboard.press("Enter");
    await expect(trigger).toHaveValue("Ada Lovelace");
    await expect(content(page)).toHaveCount(0);
});

test("virtualized demo shows visible options when opened", async ({ page }) => {
    await page.goto(demoUrl("virtualized"), { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = page.getByRole("combobox", { name: "Virtualized option picker" });
    await trigger.focus();
    await expect(trigger).toBeFocused();
    await page.keyboard.press("ArrowDown");

    const menu = list(page);
    await expect(menu).toBeVisible();
    await expect(menu.getByRole("option", { name: "Option 0", exact: true })).toBeVisible();
    await expect(menu.getByRole("option", { name: "Option 1", exact: true })).toBeVisible();
    const activeId = await trigger.getAttribute("aria-activedescendant");
    await expect(page.locator(`#${activeId}`)).toHaveText("Option 0");
});

test("virtualized ArrowUp opens at final logical option", async ({ page }) => {
    await page.goto(demoUrl("virtualized"), { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = page.getByRole("combobox", { name: "Virtualized option picker" });
    await trigger.focus();
    await page.keyboard.press("ArrowUp");

    const last = list(page).getByRole("option", { name: "Option 999", exact: true });
    await expect(last).toBeVisible();
    await expect(last).toHaveAttribute("data-highlighted", "true");
    await expect(trigger).toBeFocused();
    await expect(trigger).toHaveAttribute(
        "aria-activedescendant",
        await last.getAttribute("id"),
    );
});

test("virtualized demo keeps scrollHeight stable while scrolling", async ({ page }) => {
    await page.goto(demoUrl("virtualized"), { timeout: 20 * 60 * 1000 });
    await page.waitForLoadState("domcontentloaded");

    const trigger = page.getByRole("combobox", { name: "Virtualized option picker" });
    await trigger.focus();
    await expect(trigger).toBeFocused();
    await page.keyboard.press("ArrowDown");

    const menu = list(page);
    await expect(menu).toBeVisible();
    await page.waitForTimeout(500);

    const initialState = await menu.evaluate((el) => ({
        scrollHeight: el.scrollHeight,
        clientHeight: el.clientHeight,
        ratio: el.scrollHeight / el.clientHeight,
    }));

    const maxScroll = initialState.scrollHeight - initialState.clientHeight;
    const steps = 20;
    const stepSize = maxScroll / steps;
    const measurements: Array<{
        scrollTop: number;
        scrollHeight: number;
        clientHeight: number;
        ratio: number;
    }> = [];

    for (let i = 1; i <= steps; i++) {
        const targetScroll = Math.round(stepSize * i);

        await menu.evaluate((el, scroll) => {
            el.scrollTop = scroll;
        }, targetScroll);
        await page.waitForTimeout(100);

        measurements.push(await menu.evaluate((el) => ({
            scrollTop: el.scrollTop,
            scrollHeight: el.scrollHeight,
            clientHeight: el.clientHeight,
            ratio: el.scrollHeight / el.clientHeight,
        })));
    }

    const duringScrollMeasurements = measurements.slice(0, -1);
    const scrollHeights = duringScrollMeasurements.map((m) => m.scrollHeight);
    const clientHeights = duringScrollMeasurements.map((m) => m.clientHeight);
    const ratios = duringScrollMeasurements.map((m) => m.ratio);
    const minHeight = Math.min(...scrollHeights);
    const maxHeight = Math.max(...scrollHeights);
    const heightVariance = maxHeight - minHeight;
    const minClientHeight = Math.min(...clientHeights);
    const maxClientHeight = Math.max(...clientHeights);
    const clientHeightVariance = maxClientHeight - minClientHeight;
    const minRatio = Math.min(...ratios);
    const maxRatio = Math.max(...ratios);
    const ratioVariance = maxRatio - minRatio;

    expect(
        heightVariance,
        `combobox scrollHeight changed by ${heightVariance}px during scroll`
    ).toBeLessThan(100);
    expect(
        clientHeightVariance,
        `combobox clientHeight changed by ${clientHeightVariance}px during scroll`
    ).toBeLessThanOrEqual(1);
    expect(
        ratioVariance,
        `combobox scrollHeight/clientHeight ratio changed by ${ratioVariance} during scroll`
    ).toBeLessThan(0.5);

    const lastMeasurement = measurements.at(-1);
    expect(lastMeasurement).toBeDefined();

    await page.waitForTimeout(650);

    const settledState = await menu.evaluate((el) => ({
        scrollTop: el.scrollTop,
        scrollHeight: el.scrollHeight,
        clientHeight: el.clientHeight,
        ratio: el.scrollHeight / el.clientHeight,
    }));

    expect(
        Math.abs(settledState.scrollHeight - lastMeasurement!.scrollHeight),
        "combobox scrollHeight shifted after the 600ms scroll debounce settled"
    ).toBeLessThan(100);
    expect(
        Math.abs(settledState.clientHeight - lastMeasurement!.clientHeight),
        "combobox clientHeight changed after the 600ms scroll debounce settled"
    ).toBeLessThanOrEqual(1);
    expect(
        Math.abs(settledState.ratio - lastMeasurement!.ratio),
        "combobox scrollHeight/clientHeight ratio shifted after the 600ms scroll debounce settled"
    ).toBeLessThan(0.5);
    expect(
        Math.abs(settledState.scrollTop - lastMeasurement!.scrollTop),
        "combobox scrollTop drifted after the 600ms scroll debounce settled"
    ).toBeLessThanOrEqual(1);
});

test("touch selection commits and closes", async ({ browser, browserName }) => {
    test.skip(browserName === "firefox", "Firefox does not support mobile contexts");

    const { defaultBrowserType: _defaultBrowserType, ...iphone } = devices["iPhone 12"];
    const context = await browser.newContext(iphone);
    try {
        const page = await context.newPage();
        await page.goto(URL, { timeout: 20 * 60 * 1000 });
        await page.waitForLoadState("domcontentloaded");

        const trigger = input(page);
        await trigger.tap();
        await list(page).getByRole("option", { name: "Dioxus" }).tap();

        await expect(content(page)).toHaveCount(0);
        await expect(trigger).toHaveValue("Dioxus");
    } finally {
        await context.close();
    }
});
