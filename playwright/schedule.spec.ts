import { test, expect, type Locator, type Page } from "@playwright/test";

const BASE = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:8080";
const TEST_TIMEOUT = 90 * 1000;
const NAVIGATION_TIMEOUT = 15 * 1000;
const ROOT_TIMEOUT = 10 * 1000;
const LOAD_ATTEMPTS = 3;
const ROOT_SELECTOR = "[data-schedule-root]:visible";

test.describe.configure({ mode: "serial" });
test.setTimeout(TEST_TIMEOUT);

const mainUrl = `${BASE}/component/?name=schedule&`;
const variantUrl = (variant: string) =>
  `${BASE}/component/block/?name=schedule&variant=${variant}&`;

async function loadMain(page: Page) {
  return await loadSchedulePage(page, mainUrl);
}

async function loadVariant(page: Page, variant: string) {
  return await loadSchedulePage(page, variantUrl(variant));
}

async function loadSchedulePage(page: Page, url: string) {
  const root = page.locator(ROOT_SELECTOR).first();
  let lastError: unknown;

  for (let attempt = 0; attempt < LOAD_ATTEMPTS; attempt += 1) {
    await page.goto(url, { timeout: NAVIGATION_TIMEOUT, waitUntil: "load" });

    try {
      await expect(root).toBeVisible({ timeout: ROOT_TIMEOUT });
      return root;
    } catch (error) {
      lastError = error;
    }
  }

  const lastBodyText = await page.locator("body").innerText().catch(() => "");
  throw new Error(
    `Timed out waiting for a visible schedule root at ${url}. Last body text: ${lastBodyText.slice(0, 400)}`,
    { cause: lastError },
  );
}

function header(root: Locator) {
  return root.locator("[data-schedule-header]").first();
}

function viewButton(root: Locator, view: "day" | "week" | "month" | "year") {
  return root
    .getByRole("button", { name: new RegExp(`^${view}$`, "i") })
    .first();
}

function visibleEvent(root: Locator, name: string) {
  return root
    .locator("[data-schedule-event]:not([data-schedule-resize-preview])")
    .filter({ hasText: name })
    .first();
}

function eventsByName(root: Locator, name: string) {
  return root
    .locator("[data-schedule-event]:not([data-schedule-resize-preview])")
    .filter({ hasText: name });
}

async function timeSlotHeights(root: Locator) {
  return await root
    .locator("[data-schedule-time-slot]")
    .evaluateAll((slots) =>
      slots.slice(0, 28).map((slot) => slot.getBoundingClientRect().height),
    );
}

async function startResizeEventEnd(root: Locator, event: Locator, targetSlot: Locator) {
  const handle = event.locator("[data-schedule-resize-handle='end']").first();
  const beforeHeights = await timeSlotHeights(root);

  await event.hover();
  await expect(handle).toBeVisible();
  await handle.dispatchEvent("pointerdown");
  await expect(root).toHaveAttribute("data-resizing", "true");
  await expect(root).toHaveAttribute("data-dragging", "false");
  await expect(event).toHaveAttribute("data-draggable", "false");
  await expect(event).toHaveCSS("visibility", "hidden");

  await targetSlot.hover();
  const afterHeights = await timeSlotHeights(root);
  expect(afterHeights).toEqual(beforeHeights);
}

async function finishResizeEventEnd(targetSlot: Locator) {
  await targetSlot.dispatchEvent("pointerup");
}

test("preview page loads with header, controls, and events", async ({
  page,
}) => {
  const root = await loadMain(page);

  await expect(root).toHaveAttribute("data-view", "week");
  await expect(root).toHaveAttribute("data-mode", "default");
  await expect(root).toHaveAttribute("data-layout", "default");
  await expect(root).toHaveAttribute("data-locale", "en-US");
  await expect(header(root)).toBeVisible();
  await expect(root.getByRole("button", { name: "Previous" })).toBeVisible();
  await expect(root.getByRole("button", { name: "Today" })).toBeVisible();
  await expect(root.getByRole("button", { name: "Next" })).toBeVisible();
  await expect(
    root.getByRole("navigation", { name: "Schedule views" }),
  ).toBeVisible();
  await expect(viewButton(root, "day")).toHaveAttribute("data-active", "false");
  await expect(viewButton(root, "week")).toHaveAttribute("data-active", "true");
  await expect(viewButton(root, "month")).toHaveAttribute("data-active", "false");
  await expect(viewButton(root, "year")).toHaveAttribute("data-active", "false");
  await expect(root.locator("[data-schedule-desktop]")).toBeVisible();
  await expect(visibleEvent(root, "Launch window")).toBeVisible();
  await expect(visibleEvent(root, "Team onsite")).toHaveAttribute(
    "data-all-day",
    "true",
  );
  const resizeHandle = visibleEvent(root, "Launch window")
    .locator("[data-schedule-resize-handle='end']")
    .first();
  await expect(resizeHandle).toBeHidden();
  await visibleEvent(root, "Launch window").hover();
  await expect(resizeHandle).toBeVisible();

  await visibleEvent(root, "Launch window").click();
  await expect(page.locator("[data-schedule-main-status]")).toContainText(
    "Clicked event Launch window",
  );

  await root.locator("[data-schedule-time-slot]").nth(1).click();
  await expect(page.locator("[data-schedule-main-status]")).toContainText(
    "Clicked time slot",
  );

  await root.locator("[data-schedule-all-day-slot]").first().click();
  await expect(page.locator("[data-schedule-main-status]")).toContainText(
    "Clicked all-day slot",
  );
});

test("week view renders day headers above the all-day lane and timed grid", async ({
  page,
}) => {
  const root = await loadMain(page);
  const allDayRow = root.locator("[data-schedule-all-day-row]");
  const allDaySlots = allDayRow.locator("[data-schedule-all-day-slot]");
  const firstDayHeader = root.locator("[data-schedule-day-header]").first();
  const firstAllDaySlot = allDaySlots.first();
  const firstTimeSlot = root.locator("[data-schedule-time-slot]").first();
  const allDayEvent = visibleEvent(root, "Team onsite");
  const timedEvent = visibleEvent(root, "Launch window");
  const visibleAllDayLabel = allDayRow.getByText("All day", { exact: true });

  const [dayHeaderBox, allDaySlotBox, timeSlotBox] = await Promise.all([
    firstDayHeader.boundingBox(),
    firstAllDaySlot.boundingBox(),
    firstTimeSlot.boundingBox(),
  ]);

  expect(dayHeaderBox).not.toBeNull();
  expect(allDaySlotBox).not.toBeNull();
  expect(timeSlotBox).not.toBeNull();
  expect(dayHeaderBox!.y).toBeLessThan(allDaySlotBox!.y);
  expect(allDaySlotBox!.y).toBeLessThan(timeSlotBox!.y);
  await expect(firstAllDaySlot).toContainText("All day");
  await expect(allDaySlots).toHaveCount(7);
  await expect(allDaySlots.first()).toHaveAttribute("aria-label", /All day .+/);
  await expect(allDaySlots.nth(1)).toHaveAttribute("aria-label", /All day .+/);
  const slotTexts = await allDaySlots.evaluateAll((elements) =>
    elements.map((element) => element.textContent?.trim() ?? ""),
  );
  expect(slotTexts.filter((text) => text === "All day")).toHaveLength(1);
  expect(slotTexts.indexOf("All day")).toBe(0);

  await expect(
    allDayEvent.locator("xpath=ancestor::*[@data-schedule-all-day-events][1]"),
  ).toBeVisible();
  await expect(
    timedEvent.locator("xpath=ancestor::*[@data-schedule-timed-events][1]"),
  ).toBeVisible();
});

test("primary navigation controls are keyboard focusable", async ({ page }) => {
  const root = await loadMain(page);
  const prev = root.getByRole("button", { name: "Previous" });
  const next = root.getByRole("button", { name: "Next" });
  const today = root.getByRole("button", { name: "Today" });
  const day = viewButton(root, "day");
  const week = viewButton(root, "week");

  await prev.focus();
  await expect(prev).toBeFocused();

  await next.focus();
  await expect(next).toBeFocused();

  await today.focus();
  await expect(today).toBeFocused();

  await day.focus();
  await expect(day).toBeFocused();

  await week.focus();
  await expect(week).toBeFocused();
});

test("view switching, date navigation, and year-to-month transition work", async ({
  page,
}) => {
  const root = await loadMain(page);
  const datePicker = root.locator("[data-schedule-date-picker]");
  await expect(datePicker).toContainText("2026");
  await expect(datePicker).toContainText("05");
  await expect(datePicker).toContainText("18");

  await viewButton(root, "day").click();
  await expect(root).toHaveAttribute("data-view", "day");
  await expect(root.locator("[data-schedule-view='day']")).toBeVisible();

  await viewButton(root, "week").click();
  await expect(root).toHaveAttribute("data-view", "week");
  await expect(root.locator("[data-schedule-view='week']")).toBeVisible();

  await root.getByRole("button", { name: "Next" }).click();
  await expect(datePicker).toContainText("25");
  await expect(eventsByName(root, "Launch window")).toHaveCount(0);

  await root.getByRole("button", { name: "Previous" }).click();
  await expect(datePicker).toContainText("18");
  await expect(visibleEvent(root, "Launch window")).toBeVisible();

  await viewButton(root, "year").click();
  await expect(root).toHaveAttribute("data-view", "year");
  await expect(root.locator("[data-schedule-view='year']")).toBeVisible();

  await root.locator("[data-schedule-year-month='10']").click();
  await expect(root).toHaveAttribute("data-view", "month");
  await expect(root.locator("[data-schedule-view='month']")).toBeVisible();
  await expect(datePicker).toContainText("10");
});

test("time slots, all-day slots, and drag-selection signals are visible", async ({
  page,
}) => {
  const root = await loadVariant(page, "slot_selection");
  const selected = root.locator("xpath=preceding-sibling::div[1]");
  const firstSlot = root.locator("[data-schedule-time-slot]").nth(0);
  const allDaySlot = root.locator("[data-schedule-all-day-slot]").first();

  await expect(firstSlot).toBeVisible();
  await expect(firstSlot).toHaveAttribute("data-slot-select-enabled", "true");
  await expect(allDaySlot).toBeVisible();

  await firstSlot.click();
  await expect(selected).toContainText("Created");

  await allDaySlot.click();
  await expect(allDaySlot).toBeVisible();
});

test("event drag/drop and resize callbacks are reflected in the preview", async ({
  page,
}) => {
  const dragRoot = await loadVariant(page, "drag_and_drop");
  const draggableEvent = visibleEvent(dragRoot, "Launch window");
  const dropTarget = dragRoot.locator("[data-schedule-time-slot]").first();
  const allDayTarget = dragRoot.locator("[data-schedule-all-day-slot]").first();
  const allDayEvents = dragRoot.locator("[data-schedule-all-day-events]").first();
  const timedEvents = dragRoot.locator("[data-schedule-timed-events]").first();
  const timedLaunchWindow = timedEvents
    .locator("[data-schedule-event]")
    .filter({ hasText: "Launch window" })
    .first();

  await expect(draggableEvent).toHaveAttribute("data-draggable", "true");
  await expect(draggableEvent).toHaveAttribute("data-resizable", "false");

  await draggableEvent.dragTo(dropTarget);
  await expect(page.getByText("Dropped Launch window")).toBeVisible();
  await expect(timedEvents).toContainText("Launch window");
  await expect(allDayEvents).not.toContainText("Launch window");

  await timedLaunchWindow.dragTo(allDayTarget);
  await expect(page.getByText("Dropped Launch window")).toBeVisible();
  await expect(allDayEvents).toContainText("Launch window");
  await expect(
    allDayEvents
      .locator("[data-schedule-event]")
      .filter({ hasText: "Launch window" })
      .first(),
  ).toHaveAttribute("data-all-day", "true");
  await expect(dropTarget).not.toContainText("Launch window");

  await allDayEvents
    .locator("[data-schedule-event]")
    .filter({ hasText: "Launch window" })
    .first()
    .dragTo(dropTarget);
  await expect(page.getByText("Dropped Launch window")).toBeVisible();
  await expect(
    timedLaunchWindow,
  ).toHaveAttribute(
    "data-all-day",
    "false",
  );
  await expect(timedEvents).toContainText("Launch window");
  await expect(allDayEvents).not.toContainText("Launch window");

  const resizeRoot = await loadVariant(page, "resize");
  const resizeHandle = resizeRoot
    .locator("[data-schedule-resize-handle='end']")
    .first();
  const startResizeHandle = resizeRoot
    .locator("[data-schedule-resize-handle='start']")
    .first();
  const resizableEvent = visibleEvent(resizeRoot, "Launch window");
  const laterSlot = resizeRoot.locator("[data-schedule-time-slot]").nth(5);

  await expect(resizableEvent).toHaveAttribute("data-resizable", "true");
  await expect(resizeHandle).toBeHidden();
  await expect(startResizeHandle).toBeHidden();
  await resizableEvent.hover();
  await expect(resizeHandle).toBeVisible();
  await expect(startResizeHandle).toBeVisible();
  await expect(startResizeHandle).toHaveCSS("top", "-2px");
  await expect(resizeHandle).toHaveCSS("bottom", "-2px");
  await startResizeEventEnd(resizeRoot, resizableEvent, laterSlot);
  await expect(resizeRoot.locator("[data-schedule-resize-preview]")).toBeVisible();
  await finishResizeEventEnd(laterSlot);
  await expect(page.getByText("Resized Launch window")).toBeVisible();
  await expect(page.getByText("Resized Launch window")).not.toContainText(
    "10:30",
  );
});

test("dragging one recurring occurrence detaches only that occurrence", async ({
  page,
}) => {
  const root = await loadVariant(page, "drag_and_drop");
  const recurringEvents = root
    .locator("[data-schedule-event]")
    .filter({ hasText: "Daily team sync" });
  const targetSlot = root.locator("[data-schedule-time-slot]").nth(0);
  const originalTimeEvents = recurringEvents.filter({
    hasText: "9 AM - 9:30 AM",
  });
  const movedTimeEvents = recurringEvents.filter({ hasText: "7 AM - 7:30 AM" });
  const initialOriginalTimeCount = await originalTimeEvents.count();
  const initialMovedTimeCount = await movedTimeEvents.count();

  await expect
    .poll(async () => recurringEvents.count())
    .toBeGreaterThan(1);
  await expect(initialOriginalTimeCount).toBeGreaterThan(1);

  await recurringEvents.first().dragTo(targetSlot);
  await expect(page.getByText("Dropped Daily team sync")).toBeVisible();
  await expect
    .poll(async () => recurringEvents.count())
    .toBeGreaterThan(1);
  await expect(originalTimeEvents).toHaveCount(initialOriginalTimeCount - 1);
  await expect(movedTimeEvents).toHaveCount(initialMovedTimeCount + 1);
});

test("external drops expose external data in the preview", async ({ page }) => {
  const root = await loadVariant(page, "external_drop");
  const source = page.locator("[data-schedule-external-source]");
  const target = root.locator("[data-schedule-time-slot]").first();
  const message = page.locator("[data-schedule-external-drop-status]");

  await expect(source).toBeVisible();
  await source.dragTo(target);
  await expect(message).toContainText("Dropped");
  await expect(message).toContainText("External planning task");
});

test("controlled state, custom event rendering, and recurrence are observable", async ({
  page,
}) => {
  const controlledRoot = await loadVariant(page, "controlled");
  await viewButton(controlledRoot, "month").click();
  await expect(page.locator("[data-schedule-controlled-status]")).toContainText(
    "View changed to Month",
  );

  const customRoot = await loadVariant(page, "custom_event");
  await expect(
    customRoot.locator("[data-schedule-custom-event='launch']"),
  ).toBeVisible();
  await expect(
    customRoot.locator("[data-schedule-custom-event]").first(),
  ).toContainText("custom body");

  const recurringRoot = await loadVariant(page, "recurring");
  await expect
    .poll(async () =>
      recurringRoot
        .locator("[data-schedule-event]")
        .filter({ hasText: "Daily team sync" })
        .count(),
    )
    .toBeGreaterThan(1);
});

test("static mode keeps navigation but disables drag and resize affordances", async ({
  page,
}) => {
  const root = await loadVariant(page, "static");
  const event = visibleEvent(root, "Launch window");

  await expect(root).toHaveAttribute("data-mode", "static");
  await expect(root.getByRole("button", { name: "Previous" })).toBeVisible();
  await expect(root.getByRole("button", { name: "Next" })).toBeVisible();
  await expect(event).toHaveAttribute("data-draggable", "false");
  await expect(event).toHaveAttribute("data-resizable", "false");
  await expect(root.locator("[data-schedule-resize-handle]")).toHaveCount(0);
});

test("responsive layout renders the mobile container and swaps to mobile month at small widths", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  const root = await loadVariant(page, "responsive");

  await expect(root).toHaveAttribute("data-layout", "responsive");
  await expect(root.locator("[data-schedule-desktop]")).not.toBeVisible();
  await expect(root.locator("[data-schedule-mobile]")).toBeVisible();
  await expect(
    root.locator("[data-schedule-mobile] [data-schedule-view='mobile-month']"),
  ).toBeVisible();
  await expect(
    root.locator("[data-schedule-mobile] [data-mobile-month-view]"),
  ).toBeVisible();
  await expect(
    root.locator("[data-schedule-mobile] [data-schedule-view='week']"),
  ).toHaveCount(0);
});

test("mobile month and year views remain reachable in responsive mode", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  const root = await loadVariant(page, "responsive");

  await viewButton(root, "year").click();
  await expect(root).toHaveAttribute("data-view", "year");
  await expect(
    root.locator("[data-schedule-mobile] [data-schedule-view='year']"),
  ).toBeVisible();

  await root.locator("[data-schedule-mobile] [data-schedule-year-month='10']").click();
  await expect(root).toHaveAttribute("data-view", "month");
  await expect(
    root.locator("[data-schedule-mobile] [data-schedule-view='mobile-month']"),
  ).toBeVisible();
});
