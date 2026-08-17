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

test("named variant buttons expose the five canonical styles", async ({
  page,
}) => {
  const root = await loadDemo(page, BUTTON_DEMO_URL);
  const variants = [
    ["Default", "default"],
    ["Destructive", "destructive"],
    ["Outline", "outline"],
    ["Ghost", "ghost"],
    ["Link", "link"],
  ] as const;

  for (const [name, style] of variants) {
    await expect(
      root.getByRole("button", { name, exact: true }),
    ).toHaveAttribute("data-style", style);
  }
});

test("size demo exposes six exact size contracts and named icon controls", async ({
  page,
}) => {
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
