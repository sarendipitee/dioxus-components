import { expect, test, type Page } from "@playwright/test";

async function gotoSplitPane(page: Page, demo: string) {
  await page.goto(`/components/split_pane/block#${demo}`, {
    timeout: 30 * 1000,
    waitUntil: "load",
  });
}

function splitPaneDivider(page: Page, index = 0) {
  return page.locator('[role="separator"]:visible').nth(index);
}

function paneByIndex(page: Page, index: number) {
  return page.locator(`[data-pane-index="${index}"]:visible`).first();
}

async function readLeftPaneSizeStatus(page: Page) {
  const status = page.locator("text=Left pane:");
  await expect(status).toBeVisible();
  const text = (await status.textContent()) ?? "";
  const match = text.match(/Left pane: (\d+)px/);
  if (!match) throw new Error(`unexpected status text: ${text}`);
  return Number(match[1]);
}

async function readDividerValue(divider: ReturnType<typeof splitPaneDivider>) {
  const value = await divider.getAttribute("aria-valuenow");
  if (!value) throw new Error("split pane divider is missing aria-valuenow");
  return Number(value);
}

test("split pane divider exposes separator semantics and focus", async ({
  page,
}) => {
  await gotoSplitPane(page, "main");

  const divider = splitPaneDivider(page);

  await expect(
    page.locator('[role="group"][data-orientation="horizontal"]').first(),
  ).toHaveAttribute("data-resizable", "true");
  await expect(divider).toHaveAttribute("role", "separator");
  await expect(divider).toHaveAttribute("tabindex", "0");
  await expect(divider).toHaveAttribute("aria-orientation", "vertical");

  await divider.focus();
  await expect(divider).toBeFocused();
});

test("omitted divider size stays stable while the split pane geometry changes", async ({
  page,
}) => {
  await gotoSplitPane(page, "main");

  const root = page.locator("[data-split-pane-id]:visible").first();
  const divider = splitPaneDivider(page);
  const first = paneByIndex(page, 0);
  const second = paneByIndex(page, 1);
  const initialRootWidth = await root.evaluate(
    (element) => element.getBoundingClientRect().width,
  );
  const initialDividerWidth = await divider.evaluate(
    (element) => element.getBoundingClientRect().width,
  );
  const initialPaneTotal =
    (await first.evaluate((element) => element.getBoundingClientRect().width)) +
    (await second.evaluate((element) => element.getBoundingClientRect().width));
  const resizedWidth = initialRootWidth - 96;

  await root.evaluate((element, width) => {
    element.style.width = `${width}px`;
  }, resizedWidth);

  await expect
    .poll(() =>
      root.evaluate((element) => element.getBoundingClientRect().width),
    )
    .toBeCloseTo(resizedWidth, 0);
  await expect
    .poll(() =>
      divider.evaluate((element) => element.getBoundingClientRect().width),
    )
    .toBe(initialDividerWidth);
  await expect
    .poll(
      async () =>
        (await first.evaluate(
          (element) => element.getBoundingClientRect().width,
        )) +
        (await second.evaluate(
          (element) => element.getBoundingClientRect().width,
        )),
    )
    .toBeCloseTo(initialPaneTotal - 96, 0);
});

test("horizontal keyboard resize changes the committed pane size", async ({
  page,
}) => {
  await gotoSplitPane(page, "main");

  const divider = splitPaneDivider(page);
  const initialSize = await readDividerValue(divider);

  await divider.focus();
  await page.keyboard.press("ArrowRight");
  const afterRight = await readDividerValue(divider);
  expect(afterRight).toBeGreaterThan(initialSize);

  await page.keyboard.press("ArrowLeft");
  const afterLeft = await readDividerValue(divider);
  expect(afterLeft).toBeLessThan(afterRight);

  const statusSize = await readLeftPaneSizeStatus(page);
  expect(statusSize).toBe(Math.round(afterLeft));
});

test("controlled example commits divider resize updates back into the slider and label", async ({
  page,
}) => {
  await gotoSplitPane(page, "controlled");

  const divider = splitPaneDivider(page);
  const slider = page.getByRole("slider", { name: "Controlled pane size" });
  const label = page.getByText(/Sidebar \d+%/);

  await expect(slider).toHaveAttribute("aria-valuenow", "40");
  await expect(label).toHaveText("Sidebar 40%");

  await divider.focus();
  await expect(divider).toBeFocused();
  await page.keyboard.press("ArrowRight");

  await expect(slider).not.toHaveAttribute("aria-valuenow", "40");
  await expect(label).not.toHaveText("Sidebar 40%");
  await expect
    .poll(async () =>
      Number((await slider.getAttribute("aria-valuenow")) ?? "0"),
    )
    .toBeGreaterThan(40);
});

test("multi-pane layout keeps both dividers interactive", async ({ page }) => {
  await gotoSplitPane(page, "multi_pane");

  const dividers = page.locator('[role="separator"]:visible');
  await expect(dividers).toHaveCount(2);

  const firstDivider = dividers.first();
  const secondDivider = dividers.nth(1);
  const inspectorPane = page.locator('[data-pane-index="2"]:visible').first();

  await firstDivider.focus();
  await expect(firstDivider).toBeFocused();
  await page.keyboard.press("ArrowRight");

  await secondDivider.focus();
  await expect(secondDivider).toBeFocused();
  await expect(inspectorPane).toBeVisible();
});

test("forwards split pane accessible names and pane divider attributes", async ({
  page,
}) => {
  await gotoSplitPane(page, "main");

  await expect(
    page.locator('[role="group"][aria-label="Primary workspace"]:visible'),
  ).toBeVisible();
  await expect(
    page.locator('[aria-label="Navigator pane"]:visible'),
  ).toBeVisible();
  await expect(
    page.locator('[aria-label="Preview pane"]:visible'),
  ).toBeVisible();
  await expect(
    page.locator(
      '[role="separator"][aria-label="Resize navigator and preview"]:visible',
    ),
  ).toBeVisible();
});

test("pointer dragging changes adjacent pane geometry while conserving pair extent", async ({
  page,
}) => {
  await gotoSplitPane(page, "main");

  const first = paneByIndex(page, 0);
  const second = paneByIndex(page, 1);
  const divider = page.locator(
    '[role="separator"][aria-label="Resize navigator and preview"]:visible',
  );
  const beforeFirst = await first.boundingBox();
  const beforeSecond = await second.boundingBox();
  const handle = await divider.boundingBox();
  if (!beforeFirst || !beforeSecond || !handle)
    throw new Error("split pane geometry unavailable");

  await page.mouse.move(
    handle.x + handle.width / 2,
    handle.y + handle.height / 2,
  );
  await page.mouse.down();
  await page.mouse.move(
    handle.x + handle.width / 2 + 48,
    handle.y + handle.height / 2,
    { steps: 10 },
  );
  await page.mouse.up();

  const afterFirst = await first.boundingBox();
  const afterSecond = await second.boundingBox();
  if (!afterFirst || !afterSecond)
    throw new Error("split pane geometry unavailable after drag");
  expect(afterFirst.width).toBeGreaterThan(beforeFirst.width);
  expect(afterSecond.width).toBeLessThan(beforeSecond.width);
  expect(afterFirst.width + afterSecond.width).toBeCloseTo(
    beforeFirst.width + beforeSecond.width,
    0,
  );
});

test("Home and End honor min and max while Escape restores keyboard-start geometry", async ({
  page,
}) => {
  await gotoSplitPane(page, "constraints");

  const divider = page.locator(
    '[role="separator"][aria-label="Resize constrained panes"]:visible',
  );
  await divider.focus();
  const start = await readDividerValue(divider);
  await page.keyboard.press("End");
  await expect(divider).toHaveAttribute("aria-valuenow", "320");
  await page.keyboard.press("Home");
  await expect(divider).toHaveAttribute("aria-valuenow", "160");
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("Escape");
  expect(await readDividerValue(divider)).toBe(start);
});

test("vertical orientation and ArrowDown resize the first pane vertically", async ({
  page,
}) => {
  await gotoSplitPane(page, "vertical");

  const root = page.locator(
    '[role="group"][data-orientation="vertical"]:visible',
  );
  const divider = splitPaneDivider(page);
  await expect(root).toBeVisible();
  await expect(divider).toHaveAttribute("aria-orientation", "horizontal");
  const pane = paneByIndex(page, 0);
  const before = await pane.boundingBox();
  if (!before) throw new Error("vertical pane geometry unavailable");
  await divider.focus();
  await page.keyboard.press("ArrowDown");
  const after = await pane.boundingBox();
  if (!after)
    throw new Error("vertical pane geometry unavailable after resize");
  expect(after.height).toBeGreaterThan(before.height);
});

test("disabling resizing removes the divider tab stop and blocks keyboard and pointer input", async ({
  page,
}) => {
  await gotoSplitPane(page, "constraints");

  const toggle = page.getByRole("button", { name: "Disable resizing" });
  const divider = page.locator(
    '[role="separator"][aria-label="Resize constrained panes"]:visible',
  );
  await expect(
    page.getByText("Resizing enabled", { exact: true }),
  ).toBeVisible();
  await toggle.click();
  await expect(
    page.getByRole("button", { name: "Enable resizing" }),
  ).toBeVisible();
  await expect(
    page.getByText("Resizing disabled", { exact: true }),
  ).toBeVisible();
  await expect(divider).toHaveAttribute("tabindex", "-1");
  const before = await readDividerValue(divider);
  await divider.focus();
  await page.keyboard.press("ArrowRight");
  expect(await readDividerValue(divider)).toBe(before);
  const handle = await divider.boundingBox();
  if (!handle) throw new Error("disabled divider geometry unavailable");
  await page.mouse.move(
    handle.x + handle.width / 2,
    handle.y + handle.height / 2,
  );
  await page.mouse.down();
  await page.mouse.move(
    handle.x + handle.width / 2 + 40,
    handle.y + handle.height / 2,
    { steps: 10 },
  );
  await page.mouse.up();
  expect(await readDividerValue(divider)).toBe(before);
});

test("nested split panes retain independent orientations and divider interaction", async ({
  page,
}) => {
  await gotoSplitPane(page, "nested");

  await expect(
    page.locator('[role="group"][data-orientation="horizontal"]:visible'),
  ).toHaveCount(1);
  await expect(
    page.locator('[role="group"][data-orientation="vertical"]:visible'),
  ).toHaveCount(1);
  const dividers = page.locator('[role="separator"]:visible');
  await expect(dividers).toHaveCount(2);
  await expect(dividers.nth(0)).toHaveAttribute("aria-orientation", "vertical");
  await expect(dividers.nth(1)).toHaveAttribute(
    "aria-orientation",
    "horizontal",
  );
  await dividers.nth(1).focus();
  const innerPane = page
    .locator('[role="group"][data-orientation="vertical"]')
    .locator('[data-pane-index="0"]');
  const before = await innerPane.boundingBox();
  if (!before) throw new Error("nested pane geometry unavailable");
  await page.keyboard.press("ArrowDown");
  const after = await innerPane.boundingBox();
  if (!after) throw new Error("nested pane geometry unavailable after resize");
  expect(after.height).toBeGreaterThan(before.height);
});
