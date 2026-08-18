import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "/components/radio_group";

async function loadRadioGroup(page: Page) {
  await page.goto(URL, { timeout: 30 * 1000, waitUntil: "networkidle" });
}

function group(page: Page, name: string): Locator {
  if (name === "Preferred color") return page.locator("#preferred-color-group");
  if (name === "Density") return page.locator("#density-group");
  return page.getByRole("radiogroup", { name, exact: true }).first();
}

function radio(groupLocator: Locator, name: string): Locator {
  return groupLocator.getByRole("radio", { name, exact: true }).first();
}

test("radio group exposes its accessible name, description, ARIA, and global attributes", async ({
  page,
}) => {
  await loadRadioGroup(page);
  const preferred = group(page, "Preferred color");

  await expect(preferred).toHaveRole("radiogroup");
  await expect(preferred).toHaveAttribute("id", "preferred-color-group");
  await expect(preferred).toHaveAttribute(
    "data-testid",
    "preferred-color-group",
  );
  await expect(preferred).toHaveAttribute(
    "aria-label",
    "Preferred color choices",
  );
  await expect(preferred).toHaveAttribute(
    "data-fixture",
    "controlled-radio-group",
  );
  await expect(preferred).toHaveAttribute("aria-required", "true");
  const describedBy = (await preferred.getAttribute("aria-describedby"))!.split(
    /\s+/,
  );
  expect(describedBy).toContain("radio-help");
  const generatedDescriptionId = describedBy.find((id) => id !== "radio-help");
  expect(generatedDescriptionId).toBeTruthy();
  await expect(page.locator(`#${generatedDescriptionId}`)).toContainText(
    "Choose one available color",
  );
});

test("clicking a controlled radio updates selection, callback once, and focus", async ({
  page,
}) => {
  await loadRadioGroup(page);
  const preferred = group(page, "Preferred color");
  const red = radio(preferred, "Red");

  await red.click();
  await expect(red).toBeFocused();
  await expect(red).toHaveAttribute("aria-checked", "true");
  await expect(page.getByTestId("preferred-color-value")).toHaveText("red");
  await expect(page.getByTestId("preferred-color-callback-count")).toHaveText(
    "1",
  );

  await red.click();
  await expect(page.getByTestId("preferred-color-callback-count")).toHaveText(
    "1",
  );
});

test("horizontal arrows do not loop", async ({ page }) => {
  await loadRadioGroup(page);
  const density = group(page, "Density");
  const compact = radio(density, "Compact");
  const comfortable = radio(density, "Comfortable");
  const spacious = radio(density, "Spacious");

  await expect(density).toHaveAttribute("data-orientation", "horizontal");
  await compact.focus();
  await compact.press("ArrowLeft");
  await expect(compact).toBeFocused();
  await compact.press("ArrowRight");
  await expect(comfortable).toBeFocused();
  await comfortable.press("End");
  await spacious.press("ArrowRight");
  await expect(spacious).toBeFocused();
});

test("required named radio group contributes its selected value to FormData", async ({
  page,
}) => {
  await loadRadioGroup(page);
  const form = page.getByTestId("radio-form");
  const entries = await form.evaluate((element) =>
    Array.from(new FormData(element as HTMLFormElement).entries()).map(
      ([name, value]) => [name, String(value)],
    ),
  );
  expect(entries).toContainEqual(["preferred-color", "blue"]);
});
