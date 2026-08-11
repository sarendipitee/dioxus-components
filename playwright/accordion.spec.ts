import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "/components/accordion/block";
const LOAD_TIMEOUT = 30 * 1000;

async function loadAccordion(page: Page, id: string) {
  const demo = id.replace(/-accordion$/, "");
  await page.goto(`${URL}?demo=${demo}#${demo}`, {
    timeout: LOAD_TIMEOUT,
    waitUntil: "networkidle",
  });
  const accordion = page.locator(`#${id}`);
  await expect(accordion).toBeVisible({ timeout: LOAD_TIMEOUT });
  return accordion;
}

function items(accordion: Locator) {
  return accordion.locator(":scope > [data-open]");
}

test("main accordion has visible width", async ({ page }) => {
  const accordion = await loadAccordion(page, "single-accordion");
  const box = await accordion.boundingBox();

  expect(box).not.toBeNull();
  expect(box!.width).toBeGreaterThan(0);
});

test("content padding participates in collapse animation", async ({ page }) => {
  const accordion = await loadAccordion(page, "single-accordion");
  const account = accordion.getByRole("button", { name: "Account settings" });

  await account.click();
  const contentId = await account.getAttribute("aria-controls");
  expect(contentId).toBeTruthy();
  const region = page.locator(`#${contentId}`);
  const clip = region.locator('[data-slot="accordion-content-inner"]');
  const body = region.locator('[data-slot="accordion-content-body"]');
  await expect(body).toHaveCSS("padding-top", "24px");
  await expect(body).toHaveCSS("padding-right", "24px");

  await account.click();
  await expect(region).toBeHidden();
  await account.click();
  await expect(body).toHaveCSS("padding-top", "24px");
  await expect(body).toHaveCSS("padding-bottom", "24px");
});

test("single accordion exposes state and switches the visible panel", async ({
  page,
}) => {
  const accordion = await loadAccordion(page, "single-accordion");
  const accordionItems = items(accordion);
  const account = accordion.getByRole("button", { name: "Account settings" });
  const billing = accordion.getByRole("button", { name: "Billing" });

  await expect(account).toHaveAttribute("aria-expanded", "false");
  const accountIcon = account.locator("svg");
  await expect(accountIcon).toBeVisible();
  await expect(accountIcon).toHaveAttribute("aria-hidden", "true");
  await account.click();
  await expect(account).toHaveAttribute("aria-expanded", "true");
  await expect(
    accordion.getByText("Update your profile and account preferences."),
  ).toBeVisible();
  await expect(accordionItems.first()).toHaveAttribute("data-open", "true");

  const contentId = await account.getAttribute("aria-controls");
  const triggerId = await account.getAttribute("id");
  expect(contentId).toBeTruthy();
  expect(triggerId).toBeTruthy();
  const region = page.locator(`#${contentId}`);
  await expect(region).toHaveRole("region");
  await expect(region).toHaveAttribute("aria-labelledby", triggerId!);

  await billing.click();
  await expect(billing).toHaveAttribute("aria-expanded", "true");
  await expect(account).toHaveAttribute("aria-expanded", "false");
  await expect(
    accordion.getByText("Update your profile and account preferences."),
  ).toBeHidden();
  await expect(
    accordion.getByText("Review invoices and payment methods."),
  ).toBeVisible();

  await billing.click();
  await expect(billing).toHaveAttribute("aria-expanded", "false");
  await expect(
    accordion.getByText("Review invoices and payment methods."),
  ).toBeHidden();
});

test("multiple accordion keeps panels open and honors non-collapsible default", async ({
  page,
}) => {
  const accordion = await loadAccordion(page, "multiple-accordion");
  const shipping = accordion.getByRole("button", { name: "Shipping details" });
  const returns = accordion.getByRole("button", { name: "Returns policy" });

  await expect(shipping).toHaveAttribute("aria-expanded", "true");
  await expect(
    accordion.getByText("Orders are shipped within two business days."),
  ).toBeVisible();
  await shipping.click();
  await expect(shipping).toHaveAttribute("aria-expanded", "true");

  await returns.click();
  await expect(returns).toHaveAttribute("aria-expanded", "true");
  await expect(shipping).toHaveAttribute("aria-expanded", "true");
  await expect(
    accordion.getByText("Returns are accepted within thirty days."),
  ).toBeVisible();
});

test("disabled items and disabled accordions cannot be activated", async ({
  page,
}) => {
  const accordion = await loadAccordion(page, "single-accordion");
  const archived = accordion.getByRole("button", { name: "Archived projects" });
  await expect(archived).toBeDisabled();
  await archived.click({ force: true });
  await expect(archived).toHaveAttribute("aria-expanded", "false");
  await expect(
    accordion.getByText("Archived project settings are unavailable."),
  ).toBeHidden();

  const disabledAccordion = await loadAccordion(page, "disabled-accordion");
  const disabledTrigger = disabledAccordion.getByRole("button", {
    name: "Disabled accordion",
  });
  await expect(disabledTrigger).toBeDisabled();
  await expect(items(disabledAccordion).first()).toHaveAttribute(
    "data-disabled",
    "true",
  );
});

test("keyboard activation and vertical focus navigation skip disabled items", async ({
  page,
}) => {
  const accordion = await loadAccordion(page, "single-accordion");
  const account = accordion.getByRole("button", { name: "Account settings" });
  const billing = accordion.getByRole("button", { name: "Billing" });
  const notifications = accordion.getByRole("button", {
    name: "Notifications",
  });

  await account.focus();
  await page.keyboard.press("Enter");
  await expect(account).toHaveAttribute("aria-expanded", "true");
  await page.keyboard.press("Space");
  await expect(account).toHaveAttribute("aria-expanded", "false");

  await billing.focus();
  await page.keyboard.press("ArrowDown");
  await expect(notifications).toBeFocused();
  await page.keyboard.press("ArrowUp");
  await expect(billing).toBeFocused();
  await page.keyboard.press("End");
  await expect(notifications).toBeFocused();
  await page.keyboard.press("Home");
  await expect(account).toBeFocused();
});

test("horizontal accordion uses horizontal arrow keys", async ({ page }) => {
  const accordion = await loadAccordion(page, "horizontal-accordion");
  const overview = accordion.getByRole("button", { name: "Overview" });
  const activity = accordion.getByRole("button", { name: "Activity" });

  await overview.focus();
  await page.keyboard.press("ArrowRight");
  await expect(activity).toBeFocused();
  await page.keyboard.press("ArrowLeft");
  await expect(overview).toBeFocused();
});

test("styled accordion supports trigger slots and chevron configuration", async ({
  page,
}) => {
  await page.goto("/components/accordion/block#customization", {
    timeout: LOAD_TIMEOUT,
    waitUntil: "networkidle",
  });

  const custom = page.locator("#custom-slots-accordion");
  await expect(custom).toHaveAttribute("data-chevron-position", "right");
  const customTrigger = custom.getByRole("button", {
    name: "Can I replace trigger visuals?",
  });
  await expect(customTrigger.getByTestId("leading-icon")).toBeVisible();
  const customChevron = customTrigger.getByTestId("custom-chevron");
  await expect(customChevron).toBeVisible();
  await expect(customChevron).toHaveAttribute("width", "20");
  const rightChevronSlot = customTrigger.locator(
    ':scope > span[data-rotate="true"]',
  );
  await expect(rightChevronSlot).toHaveAttribute("aria-hidden", "true");

  const left = page.locator("#left-chevron-accordion");
  await expect(left).toHaveAttribute("data-chevron-position", "left");
  const leftTrigger = left.getByRole("button", {
    name: "Left aligned without rotation",
  });
  const leftChevronSlot = leftTrigger.locator(
    ':scope > span[data-rotate="false"]',
  );
  await expect(leftChevronSlot).toBeVisible();
  await expect(leftChevronSlot.locator("svg")).toHaveAttribute("width", "16px");

  const rotateBefore = await leftChevronSlot.evaluate(
    (element) => getComputedStyle(element).rotate,
  );
  await leftTrigger.click();
  await expect(leftTrigger).toHaveAttribute("aria-expanded", "true");
  await page.waitForTimeout(350);
  await expect(leftChevronSlot).toHaveCSS("rotate", rotateBefore);
});
