import { expect, test, type Locator, type Page } from "@playwright/test";

const URL = "/components/label";

async function loadLabel(
  page: Page,
): Promise<{ label: Locator; textbox: Locator }> {
  await page.goto(URL, { timeout: 30 * 1000, waitUntil: "networkidle" });

  const label = page.getByText("Name", { exact: true });
  const textbox = page.getByRole("textbox", { name: "Name", exact: true });

  await expect(label).toBeVisible();
  await expect(textbox).toBeVisible();

  return { label, textbox };
}

test("associates the native label with the textbox", async ({ page }) => {
  const { label, textbox } = await loadLabel(page);

  await expect(label).toHaveAttribute("for", "name");
  await expect(textbox).toHaveAttribute("id", "name");
});

test("uses visible label text as the textbox accessible name", async ({
  page,
}) => {
  const { textbox } = await loadLabel(page);

  await expect(textbox).toHaveAccessibleName("Name");
});

test("clicking the native label focuses its textbox", async ({ page }) => {
  const { label, textbox } = await loadLabel(page);

  await label.click();
  await expect(textbox).toBeFocused();
});

test("forwards global attributes to the native label", async ({ page }) => {
  await page.goto(URL, { timeout: 30 * 1000, waitUntil: "networkidle" });

  const label = page.getByTestId("name-label");
  await expect(label).toHaveAttribute("id", "name-label");
  await expect(label).toHaveAttribute("data-testid", "name-label");
});
