import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "/components/collapsible";
const LOAD_TIMEOUT = 30 * 1000;

async function loadCollapsible(page: Page, id: string) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT, waitUntil: "networkidle" });
  const collapsible = page.locator(`#${id}`);
  await expect(collapsible).toBeVisible({ timeout: LOAD_TIMEOUT });
  return collapsible;
}

function content(collapsible: Locator) {
  return collapsible.getByTestId("collapsible-content");
}

test("closed collapsible exposes its initial state and accessible trigger", async ({
  page,
}) => {
  const collapsible = await loadCollapsible(page, "uncontrolled-collapsible");
  const trigger = collapsible.getByTestId("collapsible-trigger");
  const panel = content(collapsible);

  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(panel).toHaveAttribute("data-open", "false");
  await expect(
    panel.getByText("Fixed a bug in the collapsible component"),
  ).toBeHidden();
  await expect(trigger).toHaveRole("button");
  await expect(trigger).toHaveAttribute("type", "button");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(trigger).toHaveAttribute(
    "aria-controls",
    "recent-activity-content",
  );
});

test("clicking the trigger opens and closes the collapsible", async ({
  page,
}) => {
  const collapsible = await loadCollapsible(page, "uncontrolled-collapsible");
  const trigger = collapsible.getByTestId("collapsible-trigger");
  const panel = content(collapsible);

  await trigger.click();
  await expect(collapsible).toHaveAttribute("data-open", "true");
  await expect(panel).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(
    panel.getByText("Fixed a bug in the collapsible component"),
  ).toBeVisible();

  await trigger.click();
  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(panel).toHaveAttribute("data-open", "false");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(
    panel.getByText("Fixed a bug in the collapsible component"),
  ).toBeHidden();
});

test("Enter opens and Space closes the collapsible", async ({ page }) => {
  const collapsible = await loadCollapsible(page, "uncontrolled-collapsible");
  const trigger = collapsible.getByTestId("collapsible-trigger");

  await trigger.focus();
  await trigger.press("Enter");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(collapsible).toHaveAttribute("data-open", "true");

  await trigger.press("Space");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(collapsible).toHaveAttribute("data-open", "false");
});

test("default-open collapsible starts open", async ({ page }) => {
  const collapsible = await loadCollapsible(page, "default-open-collapsible");
  const trigger = collapsible.getByRole("button", {
    name: "Default open details",
  });

  await expect(collapsible).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(
    collapsible.getByText("This content starts open."),
  ).toBeVisible();
});

test("controlled collapsible reports changes and can be opened externally", async ({
  page,
}) => {
  const collapsible = await loadCollapsible(page, "controlled-collapsible");
  const trigger = collapsible.getByRole("button", {
    name: "Controlled details",
  });

  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(
    page.locator("p", { hasText: "Controlled state: closed" }),
  ).toBeVisible();

  await trigger.click();
  await expect(collapsible).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(
    page.locator("p", { hasText: "Controlled state: open" }),
  ).toBeVisible();

  await trigger.click();
  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(
    page.locator("p", { hasText: "Controlled state: closed" }),
  ).toBeVisible();

  await page.getByRole("button", { name: "Set controlled open" }).click();
  await expect(collapsible).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(
    page.locator("p", { hasText: "Controlled state: open" }),
  ).toBeVisible();
  await expect(
    collapsible.getByText("This content is controlled."),
  ).toBeVisible();
});

test("disabled collapsible cannot be opened", async ({ page }) => {
  const collapsible = await loadCollapsible(page, "disabled-collapsible");
  const trigger = collapsible.getByRole("button", { name: "Disabled details" });

  await expect(collapsible).toHaveAttribute("data-disabled", "true");
  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(trigger).toBeDisabled();
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await trigger.click({ force: true });
  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(
    collapsible.getByText("Disabled content should stay hidden."),
  ).toBeHidden();
});
