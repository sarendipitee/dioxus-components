import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "/components/file_drop_zone";
const LOAD_TIMEOUT = 30 * 1000;

async function loadZone(page: Page, demo?: string) {
  await page.goto(demo ? `${URL}/block#${demo}` : URL, { timeout: LOAD_TIMEOUT });
  const zone = page.locator('[role="button"]:has(input[type="file"])').first();
  await expect(zone).toBeVisible();
  return zone;
}

function fileInput(zone: Locator) {
  return zone.locator('input[type="file"]');
}

const file = (name: string, mimeType: string, size = 4) => ({
  name,
  mimeType,
  buffer: Buffer.alloc(size, "x"),
});

test("exposes button and hidden input semantics", async ({ page }) => {
  const zone = await loadZone(page, "rejected");
  const input = fileInput(zone);
  await expect(zone).toHaveAccessibleName(/drop pdf files here or click to select/i);

  await expect(zone).toHaveAttribute("tabindex", "0");
  await expect(zone).not.toHaveAttribute("aria-disabled", "true");
  await expect(input).toHaveAttribute("accept", /application\/pdf/);
  await expect(input).toHaveAttribute("accept", /\.pdf/);
  await expect(input).toHaveAttribute("multiple", "true");
  await expect(input).not.toBeDisabled();
  await expect(input).toHaveAttribute("aria-hidden", "true");
});

for (const key of ["Enter", "Space"] as const) {
  test(`${key} activates the file picker`, async ({ page }) => {
    const zone = await loadZone(page);
    await zone.focus();
    const chooser = page.waitForEvent("filechooser");
    await zone.press(key);
    await chooser;
  });
}

test("accepts picker files and reports validated rejection paths", async ({ page }) => {
  const zone = await loadZone(page, "rejected");
  const input = fileInput(zone);

  await input.setInputFiles([
    file("mime-match.bin", "application/pdf"),
    file("extension-match.pdf", "application/octet-stream"),
    file("wrong.txt", "text/plain"),
    file("oversize.pdf", "application/pdf", 512 * 1024 + 1),
  ]);

  await expect(page.getByText("mime-match.bin", { exact: true })).toBeVisible();
  await expect(page.getByText("extension-match.pdf", { exact: true })).toBeVisible();
  await expect(page.getByText("wrong.txt", { exact: true })).toBeVisible();
  await expect(page.getByText("oversize.pdf", { exact: true })).toBeVisible();
  await expect(page.getByText(/\[file-invalid-type\]/)).toBeVisible();
  await expect(page.getByText(/\[file-too-large\]/)).toBeVisible();
});

test("accepts files delivered through a drop event", async ({ page }) => {
  const zone = await loadZone(page);
  await zone.evaluate((element) => {
    const transfer = new DataTransfer();
    transfer.items.add(new File(["drop contents"], "dropped.txt", { type: "text/plain" }));
    element.dispatchEvent(new DragEvent("drop", {
      bubbles: true,
      cancelable: true,
      dataTransfer: transfer,
    }));
  });
  await expect(page.getByText("dropped.txt", { exact: true })).toBeVisible();
});

test("enforces maximum count and single-file input semantics", async ({ page }) => {
  let zone = await loadZone(page, "max_count");
  await expect(fileInput(zone)).toHaveAttribute("multiple", "true");
  await fileInput(zone).setInputFiles([
    file("one.txt", "text/plain"),
    file("two.txt", "text/plain"),
    file("three.txt", "text/plain"),
    file("four.txt", "text/plain"),
  ]);
  await expect(page.getByText("3 of 3 accepted", { exact: true })).toBeVisible();
  await expect(page.getByText(/four\.txt.*maximum of 3 file\(s\)/i)).toBeVisible();

  zone = await loadZone(page, "single_file");
  await expect(fileInput(zone)).not.toHaveAttribute("multiple", "true");
  await fileInput(zone).setInputFiles(file("single.txt", "text/plain"));
  await expect(page.getByText("Selected: single.txt", { exact: true })).toBeVisible();
});

for (const demo of ["disabled", "loading"] as const) {
  test(`${demo} blocks picker and input interaction`, async ({ page }) => {
    const zone = await loadZone(page, demo);
    const input = fileInput(zone);
    await expect(zone).toHaveAttribute("aria-disabled", "true");
    await expect(zone).toHaveAttribute("tabindex", "-1");
    await expect(input).toBeDisabled();

    let chooserOpened = false;
    page.once("filechooser", () => { chooserOpened = true; });
    await zone.click({ force: true });
    await zone.press("Enter");
    await page.waitForTimeout(100);
    expect(chooserOpened).toBe(false);
  });
}

test("loading becomes interactive when loading finishes", async ({ page }) => {
  const zone = await loadZone(page, "loading");
  await page.getByRole("button", { name: "Stop loading" }).click();
  await expect(zone).not.toHaveAttribute("aria-disabled", "true");
  await expect(zone).toHaveAttribute("tabindex", "0");
  await expect(fileInput(zone)).not.toBeDisabled();
});