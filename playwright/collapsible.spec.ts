import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "/components/collapsible";
const LOAD_TIMEOUT = 30 * 1000;

async function loadCollapsible(page: Page, id: string, demo = "main") {
  await page.goto(`${URL}/block#${demo}`, {
    timeout: LOAD_TIMEOUT,
    waitUntil: "networkidle",
  });
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
  await expect(panel.getByText("dioxuslabs/components")).toBeHidden();
  await expect(trigger).toHaveRole("button");
  await expect(trigger).toHaveAttribute("type", "button");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(trigger).toHaveAttribute("aria-controls", "repository-content");
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
  await expect(panel.getByText("dioxuslabs/components")).toBeVisible();

  await trigger.click();
  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(panel).toHaveAttribute("data-open", "false");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(panel.getByText("dioxuslabs/components")).toBeHidden();
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
  const collapsible = await loadCollapsible(
    page,
    "default-open-collapsible",
    "default_open",
  );
  const trigger = collapsible.getByRole("button", {
    name: "What is included?",
  });

  await expect(collapsible).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(
    collapsible.getByText(/Accessible keyboard behavior/),
  ).toBeVisible();
});

test("controlled collapsible reports trigger and external changes", async ({
  page,
}) => {
  const collapsible = await loadCollapsible(
    page,
    "controlled-collapsible",
    "controlled",
  );
  const trigger = collapsible.getByRole("button", {
    name: "Version 0.3.1",
  });
  const state = page.getByTestId("controlled-state");

  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(state).toHaveText("Release notes are closed.");

  await trigger.click();
  await expect(collapsible).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(state).toHaveText("Release notes are open.");

  await trigger.click();
  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(state).toHaveText("Release notes are closed.");

  await page.getByRole("button", { name: "Show release notes" }).click();
  await expect(collapsible).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(state).toHaveText("Release notes are open.");
});

test("inline-actions trigger keeps sibling controls outside the disclosure button", async ({
  page,
}) => {
  const collapsible = await loadCollapsible(
    page,
    "inline-actions-collapsible",
    "inline_actions",
  );
  const trigger = collapsible.getByRole("button", { name: "Recents" });
  const moreActions = collapsible.getByRole("button", {
    name: "More recent actions",
  });
  const addItem = collapsible.getByRole("button", { name: "Add recent item" });
  const panel = collapsible.getByTestId("inline-actions-content");

  await expect(trigger.locator("svg")).toHaveCount(1);
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(moreActions).toHaveCount(1);
  await expect(addItem).toHaveCount(1);
  expect(
    await trigger.evaluate((node) => node.querySelectorAll("button").length),
  ).toBe(0);

  await addItem.click();
  await expect(collapsible).toHaveAttribute("data-open", "false");

  await trigger.click();
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(panel.getByText("Design system notes")).toBeVisible();
});

test("disabled collapsible cannot be opened", async ({ page }) => {
  const collapsible = await loadCollapsible(
    page,
    "disabled-collapsible",
    "disabled",
  );
  const trigger = collapsible.getByRole("button", { name: "Archived project" });

  await expect(collapsible).toHaveAttribute("data-disabled", "true");
  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(trigger).toBeDisabled();
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await trigger.click({ force: true });
  await expect(collapsible).toHaveAttribute("data-open", "false");
  await expect(
    collapsible.getByText("Archived project settings are unavailable."),
  ).toBeHidden();
});
