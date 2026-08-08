import { expect, test, type Page } from "@playwright/test";

const ITEM_BASE_URL = "/components/item/block";

async function loadItemDemo(page: Page, demo: string) {
  await page.goto(`${ITEM_BASE_URL}#${demo}`, { timeout: 30 * 1000, waitUntil: "domcontentloaded" });
}

test("main Item exposes noninteractive default structure and disabled action", async ({ page }) => {
  await loadItemDemo(page, "main");
  const basic = page.locator('#dx-preview-block-root [data-testid="basic-item"]');
  await expect(basic).toHaveCount(1);
  await expect(basic).toHaveJSProperty("tagName", "DIV");
  await expect(basic).not.toHaveAttribute("role");
  await expect(basic).not.toHaveAttribute("tabindex");
  await expect(basic).toHaveAttribute("id", "basic-item");
  await expect(basic).toHaveAttribute("aria-label", "Basic item summary");
  await expect(basic.locator('[data-slot="item-title"]')).toHaveJSProperty("tagName", "DIV");
  await expect(basic.locator('[data-slot="item-title"]')).toHaveText("Basic Item");
  await expect(basic.locator('[data-slot="item-description"]')).toHaveJSProperty("tagName", "P");
  await expect(basic.locator('[data-slot="item-description"]')).toHaveText(
    "A simple item with title and description.",
  );
  const action = basic.getByRole("button", { name: "Action", exact: true });
  await expect(action).toBeDisabled();
});

test("verified Item is a native focused anchor with icon media and Enter activation", async ({ page }) => {
  await loadItemDemo(page, "main");
  const link = page.locator('[data-testid="verified-item-link"]');
  await expect(link).toHaveJSProperty("tagName", "A");
  await expect(link).toHaveAttribute("href", "#");
  await expect(link).toHaveAttribute("data-size", "sm");
  await expect(link).toHaveAttribute("data-variant", "outline");
  await expect(link).toHaveAttribute("aria-label", "Open verified profile");
  await expect(link.locator('[data-slot="item-media"]')).toHaveAttribute("data-variant", "icon");
  const pathname = new URL(page.url()).pathname;
  await link.focus();
  await expect(link).toBeFocused();
  await link.press("Enter");
  await expect.poll(() => new URL(page.url()).pathname).toBe(pathname);
  await expect.poll(() => new URL(page.url()).hash).toBe("");
});

test("variant demo renders default, outline, and muted Items with content and actions", async ({ page }) => {
  await loadItemDemo(page, "variant");
  const items = page.locator('#dx-preview-block-root [data-slot="item"]');
  await expect(items).toHaveCount(3);
  await expect(items.evaluateAll((nodes) => nodes.map((node) => node.getAttribute("data-variant")))).resolves.toEqual(["default", "outline", "muted"]);
  for (const [index, title] of ["Default Variant", "Outline Variant", "Muted Variant"].entries()) {
    await expect(items.nth(index).locator('[data-slot="item-title"]')).toHaveText(title);
  }
  await expect(items.locator('[data-slot="item-description"]')).toHaveCount(3);
  await expect(items.getByRole("button", { name: "Open", exact: true })).toHaveCount(3);
});

test("size demo exposes default and small Item sizing with media variants", async ({ page }) => {
  await loadItemDemo(page, "size");
  const items = page.locator('#dx-preview-block-root [data-slot="item"]');
  await expect(items).toHaveCount(2);
  await expect(items.nth(0)).toHaveAttribute("data-size", "default");
  await expect(items.nth(1)).toHaveAttribute("data-size", "sm");
  await expect(items.nth(0)).toHaveAttribute("data-variant", "outline");
  await expect(items.nth(1)).toHaveAttribute("data-variant", "outline");
  await expect(items.nth(0).locator('[data-slot="item-media"]')).toHaveCount(0);
  await expect(items.nth(1).locator('[data-slot="item-media"]')).toHaveAttribute(
    "data-variant",
    "default",
  );
});

test("image demo renders three linked image Items with accessible metadata", async ({ page }) => {
  await loadItemDemo(page, "image");
  const items = page.locator('#dx-preview-block-root [data-slot="item"]');
  await expect(items).toHaveCount(3);
  await expect(items.evaluateAll((nodes) => nodes.every((node) => node.tagName === "A"))).resolves.toBe(true);
  for (const [index, alt] of ["Midnight City Lights", "Coffee Shop Conversations", "Digital Rain"].entries()) {
    await expect(items.nth(index).locator('[data-slot="item-media"] img')).toHaveAttribute("alt", alt);
  }
  for (const [index, title] of ["Midnight City Lights — Electric Nights", "Coffee Shop Conversations — Urban Stories", "Digital Rain — Binary Beats"].entries()) {
    await expect(items.nth(index).locator('[data-slot="item-title"]')).toHaveText(title);
  }
  for (const [index, [description, duration]] of [
    ["Neon Dreams", "3:45"],
    ["The Morning Brew", "4:05"],
    ["Cyber Symphony", "3:30"],
  ].entries()) {
    const descriptions = items.nth(index).locator('[data-slot="item-description"]');
    await expect(descriptions).toHaveCount(2);
    await expect(descriptions.nth(0)).toHaveText(description);
    await expect(descriptions.nth(1)).toHaveText(duration);
  }
});

test("group demo preserves list semantics, decorative separators, avatars, and keyboard actions", async ({ page }) => {
  await loadItemDemo(page, "group");
  const group = page.locator('#dx-preview-block-root [data-slot="item-group"]');
  await expect(group).toHaveCount(1);
  await expect(group).toHaveJSProperty("tagName", "H3");
  await expect(group).toHaveAttribute("role", "list");
  await expect(group.locator(':scope > [data-slot="item"]')).toHaveCount(3);
  const separators = group.locator(':scope > [data-slot="item-separator"]');
  await expect(separators).toHaveCount(2);
  for (let index = 0; index < 2; index += 1) {
    await expect(separators.nth(index)).toHaveAttribute("role", "none");
    await expect(separators.nth(index)).toHaveAttribute("data-orientation", "horizontal");
  }
  for (const [index, alt] of ["jkelleyrtp", "ealmloff", "DioxusLabs"].entries()) {
    await expect(group.locator("img").nth(index)).toHaveAttribute("alt", alt);
  }
  const addButtons = group.getByRole("button");
  for (const [index, name] of ["Add jkelleyrtp", "Add ealmloff", "Add DioxusLabs"].entries()) {
    await expect(addButtons.nth(index)).toHaveAccessibleName(name);
  }
  await addButtons.first().focus();
  await expect(addButtons.first()).toBeFocused();
  await addButtons.first().press("Tab");
  await expect(addButtons.nth(1)).toBeFocused();
});
