import { test, expect } from "@playwright/test";

test("pointer navigation", async ({ page }) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  await page.getByRole("button", { name: "right click here" }).first().click({
    button: "right",
  });

  // Assert the context menu is visible
  const contextMenu = page.getByRole("menu").first();
  await expect(contextMenu).toHaveAttribute("data-state", "open");
  await expect(
    page.locator(".dx_menu_label", { hasText: "Canvas" }).first(),
  ).toBeVisible();
  await expect(
    page.getByRole("menuitem", { name: "Edit" }).first(),
  ).toContainText("⌘E");
  await expect(page.getByRole("separator")).toHaveCount(2);
  await expect(
    page.getByRole("menuitemcheckbox", { name: "Show line numbers" }).first(),
  ).toHaveAttribute("data-state", "checked");
  const arrangeItem = page.getByRole("menuitem", { name: "Arrange" }).first();
  await arrangeItem.hover();
  const submenu = page.locator(".dx_menu_sub_content").first();
  await expect(submenu).toHaveAttribute("data-state", "open");
  const grace = submenu.locator("[data-menu-pointer-grace]");
  await expect(grace).toBeVisible();
  await expect(grace).toHaveCSS("pointer-events", "auto");
  await expect(grace).toHaveCSS("width", "64px");
  await expect(
    submenu.getByRole("menuitem", { name: "Bring to front" }),
  ).toBeVisible();
  const hoveredBg = await arrangeItem.evaluate(
    (el) => getComputedStyle(el).backgroundColor,
  );
  const restingBg = await page
    .getByRole("menuitem", { name: "Edit" })
    .first()
    .evaluate((el) => getComputedStyle(el).backgroundColor);
  expect(hoveredBg).not.toBe(restingBg);
  const arrangeBox = await arrangeItem.boundingBox();
  const submenuBox = await submenu.boundingBox();
  if (!arrangeBox || !submenuBox)
    throw new Error("submenu geometry unavailable");
  expect(submenuBox.x).toBeGreaterThanOrEqual(
    arrangeBox.x + arrangeBox.width - 8,
  );
  expect(Math.abs(submenuBox.y - arrangeBox.y)).toBeLessThanOrEqual(12);
  await page.mouse.move(
    arrangeBox.x + arrangeBox.width / 2,
    arrangeBox.y + arrangeBox.height / 2,
  );
  const gracePoint = {
    x: submenuBox.x - 20,
    y: arrangeBox.y + arrangeBox.height / 2,
  };
  await page.mouse.move(
    gracePoint.x,
    gracePoint.y,
    { steps: 10 },
  );
  await expect(submenu).toHaveAttribute("data-state", "open");
  await page.waitForTimeout(150);
  await expect(submenu).toHaveAttribute("data-state", "open");
  await page.mouse.move(
    submenuBox.x + 8,
    arrangeBox.y + arrangeBox.height / 2,
    { steps: 4 },
  );
  await expect(submenu).toHaveAttribute("data-state", "open");
  await submenu.getByRole("menuitem", { name: "Send to back" }).hover();
  await expect(contextMenu).toHaveAttribute("data-state", "open");
  await page.mouse.move(
    arrangeBox.x - 24,
    arrangeBox.y + arrangeBox.height / 2,
  );
  await expect(submenu).toHaveCount(0);
  await arrangeItem.hover();
  await expect(submenu).toHaveAttribute("data-state", "open");
  await submenu.getByRole("menuitem", { name: "Send to back" }).click();
  // Assert the context menu is closed after clicking a submenu item
  await expect(contextMenu).toHaveCount(0);
  await expect(page.getByText("Selected: Send to back")).toBeVisible();
});

test("flipped submenu keeps slow pointer travel through grace area", async ({
  page,
}) => {
  await page.setViewportSize({ width: 800, height: 600 });
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });

  const contextRoot = page.locator(".dx_context_menu").first();
  await contextRoot.evaluate((element) => {
    const root = element as HTMLElement;
    root.style.position = "fixed";
    root.style.left = "500px";
    root.style.top = "220px";
  });

  const trigger = page.getByRole("button", { name: "right click here" }).first();
  await trigger.click({ button: "right" });
  const contextMenu = page.getByRole("menu").first();
  await expect(contextMenu).toHaveAttribute("data-state", "open");

  const arrangeItem = contextMenu.getByRole("menuitem", { name: "Arrange" });
  await arrangeItem.hover();
  const submenu = page.locator(".dx_menu_sub_content").first();
  await expect(submenu).toHaveAttribute("data-state", "open");
  await expect(submenu).toHaveAttribute("data-side", "left");
  const grace = submenu.locator("[data-menu-pointer-grace]");
  await expect(grace).toBeVisible();
  await expect(grace).toHaveCSS("pointer-events", "auto");
  await expect(grace).toHaveCSS("width", "64px");

  const arrangeBox = await arrangeItem.boundingBox();
  const submenuBox = await submenu.boundingBox();
  if (!arrangeBox || !submenuBox) {
    throw new Error("flipped submenu geometry unavailable");
  }
  expect(submenuBox.x + submenuBox.width).toBeLessThanOrEqual(
    arrangeBox.x + 8,
  );

  await page.mouse.move(
    arrangeBox.x + arrangeBox.width / 2,
    arrangeBox.y + arrangeBox.height / 2,
  );
  await page.mouse.move(
    submenuBox.x + submenuBox.width - 20,
    submenuBox.y + submenuBox.height / 2,
    { steps: 24 },
  );
  await expect(submenu).toHaveAttribute("data-state", "open");
  await page.waitForTimeout(150);
  await expect(submenu).toHaveAttribute("data-state", "open");

  await page.mouse.move(20, 20);
  await expect(submenu).toHaveCount(0);
});

test("menu lands at the tap coordinates on touch long-press", async ({
  page,
}) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  // Push the trigger down so the tap point isn't at viewport (0, 0) — any
  // misalignment will then have a non-zero direction to detect.
  await page.evaluate(() => {
    const main = document.querySelector("main") ?? document.body;
    (main as HTMLElement).style.paddingTop = "300px";
    (main as HTMLElement).style.paddingLeft = "120px";
  });

  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();
  const contextMenu = page.getByRole("menu").first();
  const box = await trigger.boundingBox();
  if (!box) throw new Error("trigger has no bounding box");
  const tapX = box.x + box.width / 2;
  const tapY = box.y + box.height / 2;
  const pointerId = 7777;

  await trigger.evaluate(
    (el, { x, y, pointerId }) => {
      el.dispatchEvent(
        new PointerEvent("pointerdown", {
          pointerId,
          pointerType: "touch",
          isPrimary: true,
          clientX: x,
          clientY: y,
          button: 0,
          buttons: 1,
          bubbles: true,
          cancelable: true,
        }),
      );
    },
    { x: tapX, y: tapY, pointerId },
  );

  // Poll through the 500 ms long-press threshold instead of sleeping after it.
  await expect(contextMenu).toHaveAttribute("data-state", "open", {
    timeout: 1_500,
  });
  const menuBox = await contextMenu.boundingBox();
  if (!menuBox) throw new Error("menu has no bounding box");
  // The menu's top-left should be at the tap coords (give or take a px for
  // sub-pixel rounding). If it's off by tens of pixels, a viewport coord
  // system is mismatched somewhere.
  expect(Math.abs(menuBox.x - tapX)).toBeLessThan(2);
  expect(Math.abs(menuBox.y - tapY)).toBeLessThan(2);
});

test("touch long-press opens the context menu", async ({ page }) => {
  // iOS Safari does not fire `contextmenu` on long press, so the menu must
  // open from a held touch instead. Reproduces issue #262.
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();
  const contextMenu = page.getByRole("menu").first();

  const box = await trigger.boundingBox();
  if (!box) throw new Error("trigger has no bounding box");
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 4242;

  await trigger.evaluate(
    (el, { x, y, pointerId }) => {
      el.dispatchEvent(
        new PointerEvent("pointerdown", {
          pointerId,
          pointerType: "touch",
          isPrimary: true,
          clientX: x,
          clientY: y,
          button: 0,
          buttons: 1,
          bubbles: true,
          cancelable: true,
        }),
      );
    },
    { x, y, pointerId },
  );

  // Poll through the 500 ms long-press threshold instead of sleeping after it.
  await expect(contextMenu).toHaveAttribute("data-state", "open", {
    timeout: 1_500,
  });

  // Release the touch after the menu has opened; it should stay open.
  await trigger.evaluate(
    (el, { x, y, pointerId }) => {
      el.dispatchEvent(
        new PointerEvent("pointerup", {
          pointerId,
          pointerType: "touch",
          isPrimary: true,
          clientX: x,
          clientY: y,
          bubbles: true,
        }),
      );
    },
    { x, y, pointerId },
  );

  await expect(contextMenu).toHaveAttribute("data-state", "open");
});

test("pen long-press opens the context menu", async ({ page }) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();
  const contextMenu = page.getByRole("menu").first();

  const box = await trigger.boundingBox();
  if (!box) throw new Error("trigger has no bounding box");
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 4244;

  await trigger.evaluate(
    (el, { x, y, pointerId }) => {
      el.dispatchEvent(
        new PointerEvent("pointerdown", {
          pointerId,
          pointerType: "pen",
          isPrimary: true,
          clientX: x,
          clientY: y,
          button: 0,
          buttons: 1,
          bubbles: true,
          cancelable: true,
        }),
      );
    },
    { x, y, pointerId },
  );

  // Poll through the 500 ms long-press threshold instead of sleeping after it.
  await expect(contextMenu).toHaveAttribute("data-state", "open", {
    timeout: 1_500,
  });
});

test("mouse pointerdown does not arm the long-press timer", async ({
  page,
}) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();

  const box = await trigger.boundingBox();
  if (!box) throw new Error("trigger has no bounding box");
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 4245;

  await trigger.evaluate(
    (el, { x, y, pointerId }) => {
      el.dispatchEvent(
        new PointerEvent("pointerdown", {
          pointerId,
          pointerType: "mouse",
          isPrimary: true,
          clientX: x,
          clientY: y,
          button: 0,
          buttons: 1,
          bubbles: true,
          cancelable: true,
        }),
      );
    },
    { x, y, pointerId },
  );

  // This wait is intentional: absence cannot be polled to prove mouse did not
  // arm a timer, so observe beyond the 500 ms long-press threshold.
  await page.waitForTimeout(700);
  await expect(page.getByRole("menu")).toHaveCount(0);
});

test("touch tap outside closes the open menu", async ({ page }) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();
  const contextMenu = page.getByRole("menu").first();

  await trigger.click({ button: "right" });
  await expect(contextMenu).toHaveAttribute("data-state", "open");

  // Tap near the bottom-right of the viewport, well outside the menu.
  const viewport = page.viewportSize();
  if (!viewport) throw new Error("no viewport");
  const farX = viewport.width - 10;
  const farY = viewport.height - 10;
  await page.evaluate(
    ({ x, y }) => {
      const target = document.elementFromPoint(x, y);
      if (!target) throw new Error("no element at outside point");
      target.dispatchEvent(
        new PointerEvent("pointerdown", {
          pointerId: 5050,
          pointerType: "touch",
          isPrimary: true,
          clientX: x,
          clientY: y,
          button: 0,
          buttons: 1,
          bubbles: true,
          cancelable: true,
        }),
      );
    },
    { x: farX, y: farY },
  );

  await expect(contextMenu).toHaveCount(0);
});

test("pointerdown at the trigger location does not dismiss an open menu", async ({
  page,
}) => {
  // Regression for the long-press dismiss bug: on iOS Safari a fresh
  // pointerdown could land at the original touch coordinates right after the
  // menu opened (either from a topology-change re-dispatch under the active
  // touch, or from compat-mouse promotion). The dismiss listener must treat
  // the trigger as "inside" the menu's root and ignore it.
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();
  const contextMenu = page.getByRole("menu").first();

  await trigger.click({ button: "right" });
  await expect(contextMenu).toHaveAttribute("data-state", "open");

  await trigger.evaluate((el) => {
    const triggerRect = el.getBoundingClientRect();
    if (triggerRect.width === 0 || triggerRect.height === 0) {
      throw new Error("trigger has no bounding box");
    }

    const x = triggerRect.left + triggerRect.width / 2;
    const y = triggerRect.top + triggerRect.height / 2;

    if (
      x < triggerRect.left ||
      x > triggerRect.right ||
      y < triggerRect.top ||
      y > triggerRect.bottom
    ) {
      throw new Error("point is outside trigger bounds");
    }

    const root = el.parentElement;
    if (!root) throw new Error("trigger has no root");
    const rootRect = root.getBoundingClientRect();
    if (
      x < rootRect.left ||
      x > rootRect.right ||
      y < rootRect.top ||
      y > rootRect.bottom
    ) {
      throw new Error("point is outside context menu root bounds");
    }

    el.dispatchEvent(
      new PointerEvent("pointerdown", {
        pointerId: 6060,
        pointerType: "touch",
        isPrimary: true,
        clientX: x,
        clientY: y,
        button: 0,
        buttons: 1,
        bubbles: true,
        cancelable: true,
      }),
    );
  });

  await expect(contextMenu).toHaveAttribute("data-state", "open");
});

test("touch released before long-press threshold does not open the menu", async ({
  page,
}) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();

  const box = await trigger.boundingBox();
  if (!box) throw new Error("trigger has no bounding box");
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 4343;

  await trigger.evaluate(
    (el, { x, y, pointerId }) => {
      el.dispatchEvent(
        new PointerEvent("pointerdown", {
          pointerId,
          pointerType: "touch",
          isPrimary: true,
          clientX: x,
          clientY: y,
          button: 0,
          buttons: 1,
          bubbles: true,
          cancelable: true,
        }),
      );
    },
    { x, y, pointerId },
  );

  // This short wait is intentional: release before the 500 ms threshold so
  // the test exercises cancellation of an armed long press.
  await page.waitForTimeout(50);
  await trigger.evaluate(
    (el, { x, y, pointerId }) => {
      el.dispatchEvent(
        new PointerEvent("pointerup", {
          pointerId,
          pointerType: "touch",
          isPrimary: true,
          clientX: x,
          clientY: y,
          bubbles: true,
        }),
      );
    },
    { x, y, pointerId },
  );

  // This wait is intentional: absence cannot be polled to prove the canceled
  // timer stays canceled, so observe beyond the 500 ms long-press threshold.
  await page.waitForTimeout(700);
  await expect(page.getByRole("menu")).toHaveCount(0);
});

test("keyboard navigation", async ({ page }) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  await page.getByRole("button", { name: "right click here" }).first().click({
    button: "right",
  });

  // Assert the context menu is visible
  const contextMenu = page.getByRole("menu").first();
  await expect(contextMenu).toHaveAttribute("data-state", "open");
  // Hit escape to close the context menu
  await page.keyboard.press("Escape");
  // Assert the context menu is closed after pressing escape
  await expect(contextMenu).toHaveCount(0);

  // Reopen the context menu
  await page.getByRole("button", { name: "right click here" }).first().click({
    button: "right",
  });
  await page.keyboard.press("ArrowDown");
  // Assert the "Edit" menu item is focused
  await expect(
    page.getByRole("menuitem", { name: "Edit" }).first(),
  ).toBeFocused();
  await expect(
    page.getByRole("menuitem", { name: "Arrange" }).first(),
  ).toBeVisible();
  // Move down to the "Duplicate" menu item
  await page.keyboard.press("ArrowDown");
  await expect(
    page.getByRole("menuitem", { name: "Arrange" }).first(),
  ).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(
    page.getByRole("menuitem", { name: "Bring to front" }).first(),
  ).toBeFocused();
  await page.keyboard.press("Enter");
  // Assert the context menu is closed after selection
  await expect(contextMenu).toHaveCount(0);
  // Assert the selected item is displayed
  await expect(page.getByText("Selected: Bring to front")).toBeVisible();
});

test("exposes menu roles and ARIA states", async ({ page }) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();
  await trigger.click({ button: "right" });

  const menu = page.getByRole("menu").first();
  await expect(menu).toBeVisible();
  await expect(
    page.getByRole("menuitem", { name: "Edit" }).first(),
  ).toBeVisible();
  await expect(
    page.getByRole("menuitem", { name: "Undo" }).first(),
  ).toHaveAttribute("aria-disabled", "true");
  await expect(
    page.getByRole("menuitem", { name: "Arrange" }).first(),
  ).toBeVisible();
  await expect(
    page.getByRole("menuitemcheckbox", { name: "Show line numbers" }).first(),
  ).toHaveAttribute("aria-checked", "true");
  await expect(
    page.getByRole("menuitemradio", { name: "Preview" }).first(),
  ).toHaveAttribute("aria-checked", "true");
  await expect(
    page.getByRole("menuitemradio", { name: "Code" }).first(),
  ).toHaveAttribute("aria-checked", "false");
  await expect(page.getByRole("separator")).toHaveCount(2);
});

test("toggles Show line numbers with pointer and keyboard activation", async ({
  page,
}) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();
  const checkbox = page
    .getByRole("menuitemcheckbox", { name: "Show line numbers" })
    .first();

  await trigger.click({ button: "right" });
  await checkbox.click();
  await expect(checkbox).toHaveAttribute("aria-checked", "false");
  await expect(page.getByText("Line numbers: false")).toBeVisible();

  await trigger.click({ button: "right" });
  await checkbox.focus();
  await page.keyboard.press("Space");
  await expect(checkbox).toHaveAttribute("aria-checked", "true");
  await expect(page.getByText("Line numbers: true")).toBeVisible();
});

test("selects Code radio with pointer and keyboard activation", async ({
  page,
}) => {
  await page.goto("/components/context_menu", { timeout: 30 * 1000 });
  const trigger = page
    .getByRole("button", { name: "right click here" })
    .first();
  const code = page.getByRole("menuitemradio", { name: "Code" }).first();

  await trigger.click({ button: "right" });
  await code.dispatchEvent("pointerdown", { clientX: 1, clientY: 1 });
  await code.dispatchEvent("pointerup", { clientX: 1, clientY: 1 });
  await expect(code).toHaveAttribute("aria-checked", "true");
  await expect(page.getByText("Panel: code")).toBeVisible();

  await trigger.click({ button: "right" });
  await code.focus();
  await page.keyboard.press("Enter");
  await expect(code).toHaveAttribute("aria-checked", "true");
  await expect(page.getByText("Panel: code")).toBeVisible();
});
