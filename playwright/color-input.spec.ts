import { expect, test, type Page } from "@playwright/test";

const PAGE_URL = "/components/color_input/block#main";
const PREVIEW_ROOT = "#dx-preview-block-root";

async function loadColorInput(page: Page) {
  await page.goto(PAGE_URL, { timeout: 30 * 1000, waitUntil: "load" });
  const root = page.locator(PREVIEW_ROOT);
  await expect(root).toBeVisible();
  return { root, input: root.getByLabel("Accent color", { exact: true }) };
}

test("exposes native text and accessible field semantics", async ({ page }) => {
  const { root, input } = await loadColorInput(page);
  await expect(input).toHaveAttribute("type", "text");
  await expect(input).toHaveValue("#9B80FF");
  await expect(input).toHaveAttribute("autocomplete", "off");
  await expect(input).toHaveAttribute("spellcheck", "false");
  await expect(input).toHaveAttribute("data-testid", "accent-color-input");

  const id = await input.getAttribute("id");
  expect(id).toBeTruthy();
  await expect(root.locator(`label[for="${id}"]`)).toHaveText("Accent color");
  await expect(input).toHaveAttribute("aria-describedby", `${id}-description`);
});

test("focus opens the controlled accessible dialog", async ({ page }) => {
  const { input } = await loadColorInput(page);
  const controlledId = await input.getAttribute("aria-controls");
  expect(controlledId).toBeTruthy();

  await input.focus();
  await expect(input).toHaveAttribute("aria-expanded", "true");
  const dialog = page.locator(`#${controlledId}`);
  await expect(dialog).toBeVisible();
  await expect(dialog.getByRole("slider", { name: "Hue" })).toBeVisible();
});

test("valid shorthand invokes the callback and canonicalizes on blur", async ({
  page,
}) => {
  const { root, input } = await loadColorInput(page);
  await expect(root.getByTestId("accent-callback-count")).toHaveText("0");

  await input.fill("  #abc  ");
  await expect(root.getByTestId("accent-callback-count")).not.toHaveText("0");
  await expect(root.getByTestId("accent-value")).toHaveText("#AABBCC");
  await input.blur();
  await expect(input).toHaveValue("#AABBCC");
});

test("invalid drafts do not invoke the callback and revert on blur", async ({
  page,
}) => {
  const { root, input } = await loadColorInput(page);
  const callbacks = root.getByTestId("accent-callback-count");
  const initialCount = await callbacks.textContent();

  await input.fill("not-a-color");
  await expect(input).toHaveValue("not-a-color");
  await expect(callbacks).toHaveText(initialCount ?? "0");
  await input.blur();
  await expect(root.getByTestId("accent-value")).toHaveText("#9B80FF");
});

test("disabled fields reject interaction and are excluded from form data", async ({
  page,
}) => {
  const { root } = await loadColorInput(page);
  const input = root.getByLabel("Disabled color", { exact: true });
  await expect(input).toBeDisabled();
  await expect(input).toHaveAttribute("aria-expanded", "false");

  const entries = await root
    .getByTestId("color-input-form")
    .evaluate((form) =>
      Array.from(new FormData(form as HTMLFormElement).entries()),
    );
  expect(entries).not.toContainEqual(["disabled-color", "#FF0000"]);
});

test("read-only fields remain focusable and form-associated without opening", async ({
  page,
}) => {
  const { root } = await loadColorInput(page);
  const input = root.getByLabel("Read-only color", { exact: true });
  await expect(input).toHaveAttribute("readonly", "true");
  const initialValue = await input.inputValue();
  await input.focus();
  await expect(input).toBeFocused();
  await expect(input).toHaveAttribute("aria-expanded", "false");
  await page.keyboard.press("ControlOrMeta+A");
  await page.keyboard.type("#000000");
  await expect(input).toHaveValue(initialValue);

  const entries = await root
    .getByTestId("color-input-form")
    .evaluate((form) =>
      Array.from(new FormData(form as HTMLFormElement).entries()),
    );
  expect(entries).toContainEqual(["readonly-color", await input.inputValue()]);
});

test("forwards native name, form association, and custom attributes", async ({
  page,
}) => {
  const { root, input } = await loadColorInput(page);
  await expect(input).toHaveAttribute("name", "accent");
  await expect(input).toHaveAttribute("form", "color-input-form");
  await expect(input).toHaveAttribute("data-color-field", "accent");

  const entries = await root
    .getByTestId("color-input-form")
    .evaluate((form) =>
      Array.from(new FormData(form as HTMLFormElement).entries()),
    );
  expect(entries).toContainEqual(["accent", "#9B80FF"]);
});
