import { test, expect, type Page } from "@playwright/test";

const SIDEBAR_RENDER_TIMEOUT = 30 * 1000;

async function gotoSidebarBlock(page: Page) {
  await page.goto("/components/sidebar/block#main", {
    timeout: 30 * 1000,
    waitUntil: 'load'
  });

  await expect(page.locator('[data-slot="sidebar-wrapper"]')).toBeVisible({
    timeout: SIDEBAR_RENDER_TIMEOUT,
  });
}

async function gotoFloatingSidebar(page: Page) {
  await page.goto("/components/sidebar/block#floating", {
    timeout: 30 * 1000,
    waitUntil: 'load'
  });

  await expect(page.locator('[data-slot="sidebar-wrapper"]')).toBeVisible({
    timeout: SIDEBAR_RENDER_TIMEOUT,
  });
}


test("floating offcanvas: reveals on edge hover on both sides", async ({ page }) => {
  await gotoFloatingSidebar(page);

  const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])');
  const gap = sidebar.locator('[data-slot="sidebar-gap"]');
  const inner = sidebar.locator('[data-slot="sidebar-inner"]');
  const container = sidebar.locator('[data-slot="sidebar-container"]');
  const hotzone = sidebar.locator('[data-slot="sidebar-hotzone"]');
  const inset = page.locator('[data-slot="sidebar-inset"]');

  for (const side of ["left", "right"] as const) {
    await page.getByRole("button", { name: side === "left" ? "Left" : "Right" }).click();
    await page.getByRole("button", { name: "Offcanvas" }).click();
    await expect(sidebar).toHaveAttribute("data-variant", "floating");
    await expect(sidebar).toHaveAttribute("data-collapsible", "");
    await expect(sidebar).toHaveAttribute("data-side", side);
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
    await expect(hotzone).toHaveAttribute("aria-hidden", "true");

    const viewport = await page.evaluate(() => ({ width: innerWidth, height: innerHeight }));
    const expandedInner = await inner.boundingBox();
    const expandedGap = await gap.boundingBox();
    expect(expandedInner).not.toBeNull();
    expect(expandedGap).not.toBeNull();
    expect(expandedInner!.width).toBeGreaterThan(0);
    expect(expandedInner!.height).toBeGreaterThan(0);
    expect(expandedGap!.width).toBeGreaterThan(0);
    expect(expandedInner!.x).toBeGreaterThanOrEqual(0);

    await page.keyboard.press("Control+b");
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect(sidebar).toHaveAttribute("data-collapsible", "offcanvas");
    await expect(gap).toHaveCSS("width", "0px");
    await expect.poll(async () => {
      const box = await inner.boundingBox();
      return box
        ? side === "left"
          ? box.x + box.width <= 0
          : box.x >= viewport.width
        : false;
    }).toBe(true);
    const collapsedInset = await inset.boundingBox();
    expect(collapsedInset).not.toBeNull();
    expect(Math.round(collapsedInset!.x)).toBe(0);
    expect(Math.round(collapsedInset!.width)).toBe(viewport.width);

    const edgeX = side === "left" ? 1 : viewport.width - 1;
    await page.mouse.move(edgeX, Math.round(viewport.height / 2));
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? Math.abs(box.x) <= 1
        : Math.abs(box.x + box.width - viewport.width) <= 1;
    }).toBe(true);
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    const revealedInner = await inner.boundingBox();
    expect(revealedInner).not.toBeNull();
    await page.mouse.move(
      Math.round(revealedInner!.x + revealedInner!.width / 2),
      Math.round(revealedInner!.y + revealedInner!.height / 2),
    );
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? Math.abs(box.x) <= 1
        : Math.abs(box.x + box.width - viewport.width) <= 1;
    }).toBe(true);

    await page.mouse.move(Math.round(viewport.width / 2), Math.round(viewport.height / 2));
    await expect.poll(async () => {
      const box = await inner.boundingBox();
      return box
        ? side === "left"
          ? box.x + box.width <= 0
          : box.x >= viewport.width
        : false;
    }).toBe(true);
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await page.keyboard.press("Control+b");
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
  }
});
test("offcanvas: collapsed rail drag stops when expansion threshold is crossed", async ({ page }) => {
  await gotoSidebarBlock(page);

  const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])').first();
  const rail = sidebar.locator('[data-slot="sidebar-rail"]');
  const gap = sidebar.locator('[data-slot="sidebar-gap"]');
  const trigger = page.locator('[data-slot="sidebar-trigger"]').first();

  if ((await sidebar.getAttribute("data-state")) === "expanded") {
    await trigger.click();
  }
  await expect(gap).toHaveCSS("width", "0px");
  await expect(sidebar).toHaveAttribute("data-state", "collapsed");

  const railBox = await rail.boundingBox();
  expect(railBox).not.toBeNull();
  const startX = railBox!.x + railBox!.width / 2;
  const y = railBox!.y + railBox!.height / 2;

  await page.mouse.move(startX, y);
  await page.mouse.down();
  await page.mouse.move(startX + 35, y, { steps: 2 });
  await expect(sidebar).toHaveAttribute("data-state", "expanded");
  await expect(gap).toHaveCSS("width", "220px");

  await page.mouse.move(startX + 180, y, { steps: 4 });
  await expect(gap).toHaveCSS("width", "220px");
  await page.mouse.up();

  await expect(sidebar).toHaveAttribute("data-state", "expanded");
  await expect(gap).toHaveCSS("width", "220px");
});
test("offcanvas: rail collapse can reopen from trigger and rail", async ({ page }) => {
  await gotoSidebarBlock(page);

  const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])').first();
  const container = sidebar.locator('[data-slot="sidebar-container"]');
  const gap = sidebar.locator('[data-slot="sidebar-gap"]');
  const rail = sidebar.locator('[data-slot="sidebar-rail"]');
  const trigger = page.locator('[data-slot="sidebar-trigger"]').first();

  const collapseByRail = async () => {
    const railBox = await rail.boundingBox();
    expect(railBox).not.toBeNull();
    const y = railBox!.y + railBox!.height / 2;
    await page.mouse.move(railBox!.x + railBox!.width / 2, y);
    await page.mouse.down();
    await page.mouse.move(1, y);
    await page.mouse.up();
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect(gap).toHaveCSS("width", "0px");
  };

  await collapseByRail();
  await trigger.click();
  await expect(sidebar).toHaveAttribute("data-state", "expanded");
  await expect(gap).toHaveCSS("width", "220px");
  await expect(container).toHaveCSS("left", "0px");

  await collapseByRail();
  await rail.click();
  await expect(sidebar).toHaveAttribute("data-state", "expanded");
  await expect(gap).toHaveCSS("width", "220px");
  await expect(container).toHaveCSS("left", "0px");
});

test("icon: trigger collapses to icon width after rail resize", async ({ page }) => {
  await gotoSidebarBlock(page);

  const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])').first();
  const container = sidebar.locator('[data-slot="sidebar-container"]');
  const gap = sidebar.locator('[data-slot="sidebar-gap"]');
  const rail = sidebar.locator('[data-slot="sidebar-rail"]');
  const trigger = page.locator('[data-slot="sidebar-trigger"]').first();

  const railBox = await rail.boundingBox();
  expect(railBox).not.toBeNull();
  const y = railBox!.y + railBox!.height / 2;
  await page.mouse.move(railBox!.x + railBox!.width / 2, y);
  await page.mouse.down();
  await page.mouse.move(1, y);
  await page.mouse.up();
  await expect(sidebar).toHaveAttribute("data-state", "collapsed");

  await page.getByRole("button", { name: "Icon", exact: true }).click();
  await expect(sidebar).toHaveAttribute("data-collapsible", "icon");
  await expect(gap).toHaveCSS("width", "48px");
  await expect(container).toHaveCSS("width", "48px");

  await trigger.click();
  await expect(sidebar).toHaveAttribute("data-state", "expanded");
  await expect(gap).toHaveCSS("width", "220px");
  await trigger.click();
  await expect(sidebar).toHaveAttribute("data-state", "collapsed");
  await expect(gap).toHaveCSS("width", "48px");
  await expect(container).toHaveCSS("width", "48px");
});


test("floating offcanvas: collapsed rail drag never expands", async ({ page }) => {
  await gotoFloatingSidebar(page);

  const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])').first();
  const rail = sidebar.locator('[data-slot="sidebar-rail"]');
  const viewport = page.viewportSize();
  expect(viewport).not.toBeNull();

  if ((await sidebar.getAttribute("data-state")) === "expanded") {
    await page.keyboard.press("Control+b");
  }
  await expect(sidebar).toHaveAttribute("data-state", "collapsed");

  await page.mouse.move(1, Math.round(viewport!.height / 2));
  const railBox = await rail.boundingBox();
  expect(railBox).not.toBeNull();
  const startX = railBox!.x + railBox!.width / 2;
  const y = railBox!.y + railBox!.height / 2;
  await rail.click();
  await expect(sidebar).toHaveAttribute("data-state", "expanded");
  await page.keyboard.press("Control+b");
  await expect(sidebar).toHaveAttribute("data-state", "collapsed");
  await page.mouse.move(1, y);

  await page.mouse.move(startX, y);
  await page.mouse.down();
  await page.mouse.move(startX + 60, y);
  await expect(sidebar).toHaveAttribute("data-state", "collapsed");
  await page.mouse.up();
  await expect(sidebar).toHaveAttribute("data-state", "collapsed");
});

test("floating offcanvas: trigger hover reveals transiently and click toggles on both sides", async ({ page }) => {
  await gotoFloatingSidebar(page);

  const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])');
  const trigger = page.locator('[data-slot="sidebar-trigger"]');
  const gap = sidebar.locator('[data-slot="sidebar-gap"]');
  const container = sidebar.locator('[data-slot="sidebar-container"]');

  for (const side of ["left", "right"] as const) {
    await page.getByRole("button", { name: side === "left" ? "Left" : "Right" }).click();
    await page.getByRole("button", { name: "Offcanvas" }).click();
    await expect(sidebar).toHaveAttribute("data-variant", "floating");
    await expect(sidebar).toHaveAttribute("data-collapsible", "");
    await expect(sidebar).toHaveAttribute("data-side", side);
    await expect(sidebar).toHaveAttribute("data-state", "expanded");

    await trigger.click();
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect(gap).toHaveCSS("width", "0px");

    const viewport = await page.evaluate(() => ({ width: innerWidth, height: innerHeight }));
    const triggerBox = await trigger.boundingBox();
    expect(triggerBox).not.toBeNull();
    await page.mouse.move(
      triggerBox!.x + triggerBox!.width / 2,
      triggerBox!.y + triggerBox!.height / 2,
    );
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? Math.abs(box.x) <= 1
        : Math.abs(box.x + box.width - viewport.width) <= 1;
    }).toBe(true);
    await expect(gap).toHaveCSS("width", "0px");

    const revealedContainer = await container.boundingBox();
    expect(revealedContainer).not.toBeNull();
    await page.mouse.move(
      revealedContainer!.x + revealedContainer!.width / 2,
      revealedContainer!.y + revealedContainer!.height / 2,
    );
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? Math.abs(box.x) <= 1
        : Math.abs(box.x + box.width - viewport.width) <= 1;
    }).toBe(true);
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect(gap).toHaveCSS("width", "0px");

    await page.mouse.move(Math.round(viewport.width / 2), 1);
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? box.x + box.width <= 0
        : box.x >= viewport.width;
    }).toBe(true);
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");

    const secondTriggerBox = await trigger.boundingBox();
    expect(secondTriggerBox).not.toBeNull();
    await page.mouse.move(
      secondTriggerBox!.x + secondTriggerBox!.width / 2,
      secondTriggerBox!.y + secondTriggerBox!.height / 2,
    );
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? Math.abs(box.x) <= 1
        : Math.abs(box.x + box.width - viewport.width) <= 1;
    }).toBe(true);

    await trigger.evaluate((element: HTMLElement) => element.click());
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
    await expect.poll(async () => (await gap.boundingBox())?.width ?? 0).toBeGreaterThan(0);
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? Math.abs(box.x) <= 1
        : Math.abs(box.x + box.width - viewport.width) <= 1;
    }).toBe(true);

    await trigger.click();
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect(gap).toHaveCSS("width", "0px");
    await trigger.hover();
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? Math.abs(box.x) <= 1
        : Math.abs(box.x + box.width - viewport.width) <= 1;
    }).toBe(true);
    await page.mouse.move(Math.round(viewport.width / 2), 1);
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? box.x + box.width <= 0
        : box.x >= viewport.width;
    }).toBe(true);

    await page.mouse.move(Math.round(viewport.width / 2), 1);
    await expect.poll(async () => {
      const box = await container.boundingBox();
      if (!box) return false;
      return side === "left"
        ? box.x + box.width <= 0
        : box.x >= viewport.width;
    }).toBe(true);
    await page.keyboard.press("Control+b");
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
  }
});
test("sidebar: preview page renders block", async ({ page }) => {
  await page.goto("/components/sidebar", {
    timeout: 30 * 1000,
    waitUntil: 'load'
  });
  const iframe = page.locator("iframe").first();
  await expect(iframe).toBeVisible({ timeout: SIDEBAR_RENDER_TIMEOUT });
  await expect(iframe).toHaveAttribute(
    "src",
    /components\/sidebar\/block#main/,
    { timeout: SIDEBAR_RENDER_TIMEOUT },
  );

  await expect(
    page.frameLocator("iframe").first().locator('[data-slot="sidebar-wrapper"]'),
  ).toBeVisible({ timeout: SIDEBAR_RENDER_TIMEOUT });
});

test.describe("sidebar: block route", () => {
  test("desktop: toggles via button and Ctrl+B", async ({ page }) => {
    await gotoSidebarBlock(page);

    const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])');
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
    const trigger = page.locator('[data-slot="sidebar-trigger"]');
    await expect(trigger).toHaveAccessibleName("Toggle Sidebar");

    // Toggle via button.
    await trigger.click();
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await trigger.click();
    await expect(sidebar).toHaveAttribute("data-state", "expanded");

    // Toggle via keyboard shortcut (⌘/Ctrl+B).
    await page.keyboard.press("Control+b");
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await page.keyboard.press("Control+b");
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
  });

  test("desktop: rail drag resizes within configured bounds", async ({ page }) => {
    await gotoSidebarBlock(page);

    const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])');
    const rail = sidebar.locator('[data-slot="sidebar-rail"]');
    await expect(rail).toHaveAccessibleName("Resize Sidebar");

    const railBox = await rail.boundingBox();
    expect(railBox).not.toBeNull();
    await page.mouse.move(railBox!.x + railBox!.width / 2, railBox!.y + railBox!.height / 2);
    await page.mouse.down();
    await page.mouse.move(railBox!.x + 500, railBox!.y + railBox!.height / 2);
    await page.mouse.up();
    await expect(sidebar.locator('[data-slot="sidebar-gap"]')).toHaveCSS("width", "360px");

    const resizedRailBox = await rail.boundingBox();
    expect(resizedRailBox).not.toBeNull();
    await page.mouse.move(
      resizedRailBox!.x + resizedRailBox!.width / 2,
      resizedRailBox!.y + resizedRailBox!.height / 2,
    );
    await page.mouse.down();
    await page.mouse.move(100, resizedRailBox!.y + resizedRailBox!.height / 2);
    await page.mouse.up();
    await expect(sidebar.locator('[data-slot="sidebar-gap"]')).toHaveCSS("width", "220px");
  });

  test("desktop: rail drag collapses near either viewport edge", async ({ page }) => {
    await gotoSidebarBlock(page);

    const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])');
    const rail = sidebar.locator('[data-slot="sidebar-rail"]');
    const viewport = page.viewportSize();
    expect(viewport).not.toBeNull();

    for (const side of ["left"] as const) {
      await page.getByRole("button", { name: side === "left" ? "Left" : "Right" }).click();
      await expect(sidebar).toHaveAttribute("data-side", side);
      await expect(sidebar).toHaveAttribute("data-state", "expanded");

      const railBox = await rail.boundingBox();
      expect(railBox).not.toBeNull();
      const y = railBox!.y + railBox!.height / 2;
      await page.mouse.move(railBox!.x + railBox!.width / 2, y);
      await page.mouse.down();
      await page.mouse.move(side === "left" ? 40 : viewport!.width - 40, y);
      await page.mouse.up();

      await expect(sidebar).toHaveAttribute("data-state", "collapsed");
      await expect(sidebar).toHaveCSS("--dx-sidebar-width", "220px");
      await page.locator('[data-slot="sidebar-trigger"]').click();
      await expect(sidebar).toHaveAttribute("data-state", "expanded");
    }
  });

  test("desktop: collapsed inset rail drag reopens the sidebar", async ({ page }) => {
    await page.goto("/components/sidebar/block#inset", {
      timeout: 30 * 1000,
      waitUntil: "load",
    });

    const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])').first();
    const rail = sidebar.locator('[data-slot="sidebar-rail"]');
    const viewport = page.viewportSize();
    expect(viewport).not.toBeNull();

    await page.getByRole("button", { name: "Left" }).click();
    await expect(sidebar).toHaveAttribute("data-side", "left");
    if ((await sidebar.getAttribute("data-state")) === "expanded") {
      await page.keyboard.press("Control+b");
    }
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");

    await rail.click();
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
    await page.keyboard.press("Control+b");
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");

    const railBox = await rail.boundingBox();
    expect(railBox).not.toBeNull();
    const dragStartX = railBox!.x + railBox!.width / 2;
    const y = railBox!.y + railBox!.height / 2;
    await page.mouse.move(dragStartX, y);
    await page.mouse.down();
    await page.mouse.move(dragStartX + 29, y);
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await page.mouse.move(dragStartX + 31, y);
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
    await page.mouse.move(300, y);
    await page.mouse.up();

    await expect(sidebar).toHaveAttribute("data-state", "expanded");
  });

  test("desktop: side switch updates data-side", async ({ page }) => {
    await gotoSidebarBlock(page);

    const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])');
    await expect(sidebar).toHaveAttribute("data-side", "left");

    await page.getByRole("button", { name: "Right" }).click();
    await expect(sidebar).toHaveAttribute("data-side", "right");
    await page.getByRole("button", { name: "Left" }).click();
    await expect(sidebar).toHaveAttribute("data-side", "left");
  });

  test("desktop: icon collapse shows tooltip on focus and preserves accessible names", async ({
    page,
  }) => {
    await gotoSidebarBlock(page);

    const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])');
    const trigger = page.locator('[data-slot="sidebar-trigger"]');

    await page.getByRole("button", { name: "Icon" }).click();
    await trigger.click();

    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect(sidebar).toHaveAttribute("data-collapsible", "icon");

    // In icon-collapsed mode, tooltips should appear on keyboard focus.
    const playground = page
      .locator('[data-sidebar="menu-button"]')
      .filter({ hasText: "Playground" })
      .first();

    await playground.focus();

    const tooltip = page.getByRole("tooltip");
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toContainText("Playground");

    // Even when labels are visually hidden in icon mode, the control should still have an accessible name.
    await expect(playground).toHaveAccessibleName("Playground");
  });

  test("mobile: opens as a sheet and closes with Escape (focus restored)", async ({
    page,
  }) => {
    await gotoSidebarBlock(page);

    const trigger = page.locator('[data-slot="sidebar-trigger"]');
    await trigger.tap();

    const sheet = page.locator('[data-slot="sheet-root"]');
    await expect(sheet).toHaveAttribute("data-state", "open");
    await page.keyboard.press("Escape");
    await expect(sheet).toHaveCount(0);
    await expect(trigger).toBeFocused();
  });
});

test.describe("sidebar: focused accessibility and modes", () => {
  test("desktop trigger exposes relationship and keyboard activation", async ({ page }) => {
    await gotoSidebarBlock(page);
    const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])').first();
    const trigger = page.locator('[data-slot="sidebar-trigger"]').first();

    await expect(trigger).toHaveAccessibleName("Toggle Sidebar");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    const controls = await trigger.getAttribute("aria-controls");
    expect(controls).not.toBeNull();
    await expect(page.locator(`#${controls}`)).toHaveAttribute("data-slot", "sidebar");

    await trigger.focus();
    await page.keyboard.press("Enter");
    await expect(sidebar).toHaveAttribute("data-state", "collapsed");
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    await page.keyboard.press(" ");
    await expect(sidebar).toHaveAttribute("data-state", "expanded");
  });

  test("desktop collapse variants and public state attrs are observable", async ({ page }) => {
    await gotoSidebarBlock(page);
    const sidebar = page.locator('[data-slot="sidebar"]:not([data-mobile="true"])').first();
    const trigger = page.locator('[data-slot="sidebar-trigger"]').first();
    await expect(sidebar).toHaveAttribute("data-variant", "sidebar");
    await expect(sidebar).toHaveAttribute("data-side", "left");

    for (const variant of ["Offcanvas", "Icon"] as const) {
      await page.getByRole("button", { name: variant, exact: true }).click();
      await expect(sidebar).toHaveAttribute("data-state", "expanded");
      await expect(sidebar).toHaveAttribute("data-collapsible", "");
      await trigger.click();
      await expect(sidebar).toHaveAttribute("data-state", "collapsed");
      await expect(sidebar).toHaveAttribute("data-collapsible", variant.toLowerCase());
      await trigger.click();
    }

    await page.getByRole("button", { name: "None", exact: true }).click();
    await expect(sidebar).not.toHaveAttribute("data-state", /.+/);
    await expect(sidebar).not.toHaveAttribute("data-collapsible", /.+/);
  });

  test("Ctrl+B updates controlled callback-visible state", async ({ page }) => {
    await gotoSidebarBlock(page);
    const state = page.getByTestId("sidebar-open-state");
    await expect(state).toHaveText("expanded");
    await page.keyboard.press("Control+b");
    await expect(state).toHaveText("collapsed");
    await page.getByRole("button", { name: "Set sidebar expanded", exact: true }).click();
    await expect(state).toHaveText("expanded");
    await page.getByRole("button", { name: "Set sidebar collapsed", exact: true }).click();
    await expect(state).toHaveText("collapsed");
  });

  test("forwards global attributes to the sidebar root", async ({ page }) => {
    await gotoSidebarBlock(page);
    const root = page.getByTestId("sidebar-demo-root");
    await expect(root).toHaveAttribute("aria-label", "Demo sidebar");
    await expect(root).toHaveAttribute("data-testid", "sidebar-demo-root");
  });

  test("mobile sidebar uses dialog semantics, side mapping, and Escape restoration", async ({ page }) => {
    await page.setViewportSize({ width: 767, height: 800 });
    await gotoSidebarBlock(page);
    const trigger = page.locator('[data-slot="sidebar-trigger"]').first();
    await expect(page.locator('[data-slot="sidebar"][data-mobile="true"]')).toHaveCount(0);
    await trigger.focus();
    await trigger.press("Enter");

    const mobile = page.locator('[data-slot="sidebar"][data-mobile="true"]');
    await expect(mobile).toBeVisible();
    await expect(mobile).toHaveAttribute("role", "dialog");
    await expect(mobile).toHaveAttribute("data-side", "left");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await page.keyboard.press("Escape");
    await expect(mobile).toBeHidden();
    await expect(trigger).toBeFocused();

    await page.getByRole("button", { name: "Right", exact: true }).click();
    await trigger.click();
    await expect(page.locator('[data-slot="sidebar"][data-mobile="true"]')).toHaveAttribute("data-side", "right");
  });
});
