import { expect, test, type Locator, type Page } from "@playwright/test";

const BUTTON_DEMO_URL = "/components/button/block#main";
const SIZE_DEMO_URL = "/components/button/block#size";
const PREVIEW_ROOT = "#dx-preview-block-root";

async function loadDemo(page: Page, url: string): Promise<Locator> {
  await page.goto(url, { timeout: 30 * 1000, waitUntil: "load" });
  const root = page.locator(PREVIEW_ROOT);
  await expect(root).toBeVisible();
  return root;
}

test("main demo buttons expose native semantics and forwarded attributes", async ({ page }) => {
  const root = await loadDemo(page, BUTTON_DEMO_URL);
  const activate = root.getByRole("button", { name: "Activate", exact: true });
  const submit = root.getByRole("button", { name: "Submit action", exact: true });

  await expect(activate).toHaveCount(1);
  await expect(activate).toHaveJSProperty("tagName", "BUTTON");
  await expect(activate).toHaveAttribute("id", "button-activation");
  await expect(activate).toHaveAttribute("data-testid", "button-activation");
  await expect(activate).toHaveAttribute("title", "Activates an observable counter");
  await expect(activate).toHaveAttribute("type", "button");
  await expect(submit).toHaveAttribute("type", "submit");
});

test("button activates by click, Enter, and Space", async ({ page }) => {
  const root = await loadDemo(page, BUTTON_DEMO_URL);
  const activate = root.getByRole("button", { name: "Activate", exact: true });
  const count = root.getByTestId("button-activation-count");

  await expect(count).toHaveText("Activations: 0");
  await activate.click();
  await expect(count).toHaveText("Activations: 1");
  await activate.press("Enter");
  await expect(count).toHaveText("Activations: 2");
  await activate.press("Space");
  await expect(count).toHaveText("Activations: 3");
});

test("disabled button cannot focus or activate", async ({ page }) => {
  const root = await loadDemo(page, BUTTON_DEMO_URL);
  const activate = root.getByRole("button", { name: "Activate", exact: true });
  const disabled = root.getByRole("button", { name: "Disabled", exact: true });
  const submit = root.getByRole("button", { name: "Submit action", exact: true });
  const count = root.getByTestId("button-activation-count");

  await expect(disabled).toBeDisabled();
  await activate.focus();
  await page.keyboard.press("Tab");
  await expect(submit).toBeFocused();
  await disabled.evaluate((element) => (element as HTMLButtonElement).click());
  await expect(count).toHaveText("Activations: 0");
});

test("named variant buttons expose the six canonical styles", async ({ page }) => {
  const root = await loadDemo(page, BUTTON_DEMO_URL);
  const variants = [
    ["Default", "default"],
    ["Secondary", "secondary"],
    ["Destructive", "destructive"],
    ["Outline", "outline"],
    ["Ghost", "ghost"],
    ["Link", "link"],
  ] as const;

  for (const [name, style] of variants) {
    await expect(root.getByRole("button", { name, exact: true })).toHaveAttribute("data-style", style);
  }
});

test("size demo exposes six exact size contracts and named icon controls", async ({ page }) => {
  const root = await loadDemo(page, SIZE_DEMO_URL);
  const buttons = root.getByRole("button");
  await expect(buttons).toHaveCount(6);

  const contracts = [
    ["Small", "sm"],
    ["Submit", "icon-sm"],
    ["Default", "default"],
    ["Submit", "icon"],
    ["Large", "lg"],
    ["Submit", "icon-lg"],
  ] as const;
  for (const [index, [name, size]] of contracts.entries()) {
    const button = buttons.nth(index);
    await expect(button).toHaveAttribute("data-size", size);
    if (name === "Submit") {
      await expect(button).toHaveAccessibleName("Submit");
      await expect(button).toHaveAttribute("aria-label", "Submit");
    } else {
      await expect(button).toHaveAccessibleName(name);
    }
  }
});
