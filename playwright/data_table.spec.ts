import { test, expect, type Page } from "@playwright/test";

const VIRTUALIZED_URL =
  "/components/data_table/block#virtualized";

// The virtualized demo renders 5,000 rows but should only mount a small
// window of them at a time.
test("virtualized table mounts only a small window of rows", async ({
  page,
}) => {
  await page.goto(VIRTUALIZED_URL, { timeout: 20 * 60 * 1000 });

  const surface = page.locator('[data-slot="data-table-surface"]').first();
  await expect(surface).toBeVisible({ timeout: 60000 });

  const rows = page.locator("tr[data-virtual-index]");
  await expect(rows.first()).toBeVisible({ timeout: 30000 });

  // Far fewer than 5,000 rows should be in the DOM.
  const mountedCount = await rows.count();
  expect(mountedCount).toBeGreaterThan(0);
  expect(mountedCount).toBeLessThan(200);

  // The surface should actually be scrollable (bounded viewport over a tall body).
  const { scrollHeight, clientHeight } = await surface.evaluate((el) => ({
    scrollHeight: el.scrollHeight,
    clientHeight: el.clientHeight,
  }));
  expect(scrollHeight).toBeGreaterThan(clientHeight + 100);
});

test("virtualized table mounts off-screen rows on scroll", async ({ page }) => {
  await page.goto(VIRTUALIZED_URL, { timeout: 20 * 60 * 1000 });

  const surface = page.locator('[data-slot="data-table-surface"]').first();
  await expect(surface).toBeVisible({ timeout: 60000 });
  await expect(page.locator("tr[data-virtual-index]").first()).toBeVisible({
    timeout: 60000,
  });

  // Initially only low indices are mounted.
  const maxIndexBefore = await page.evaluate(() => {
    const indices = Array.from(
      document.querySelectorAll("tr[data-virtual-index]")
    ).map((el) =>
      parseInt(el.getAttribute("data-virtual-index") ?? "0", 10)
    );
    return Math.max(...indices);
  });
  expect(maxIndexBefore).toBeLessThan(100);

  // Scroll deep into the list. Jitter the target each retry so scrollTop always
  // changes — a no-op assignment fires no scroll event, so the virtualizer would
  // never re-trigger. Dispatch a scroll event too, in case the engine coalesces.
  let attempt = 0;
  await expect(async () => {
    attempt += 1;
    const target = 60000 + attempt * 211;
    await surface.evaluate((el, top) => {
      el.scrollTop = top;
      el.dispatchEvent(new Event("scroll"));
    }, target);
    await page.waitForTimeout(500);

    const indices = await page.evaluate(() =>
      Array.from(document.querySelectorAll("tr[data-virtual-index]")).map(
        (el) => parseInt(el.getAttribute("data-virtual-index") ?? "0", 10)
      )
    );
    const maxIndex = Math.max(...indices);
    // After scrolling far down, rows with high indices must have mounted, and
    // the original top rows must have unmounted.
    expect(maxIndex).toBeGreaterThan(500);
    expect(Math.min(...indices)).toBeGreaterThan(50);
  }).toPass({ timeout: 25000 });
});

// Column-width stability is the key risk for table virtualization: as the
// mounted row set changes during scroll, column widths must not jitter or the
// header desyncs from the body.
test("virtualized table keeps column widths stable during scroll", async ({
  page,
}) => {
  await page.goto(VIRTUALIZED_URL, { timeout: 20 * 60 * 1000 });

  const surface = page.locator('[data-slot="data-table-surface"]').first();
  await expect(surface).toBeVisible({ timeout: 60000 });
  await expect(page.locator("tr[data-virtual-index]").first()).toBeVisible({
    timeout: 60000,
  });

  const layoutSnapshot = async () =>
    page.evaluate(() => {
      const surf = document.querySelector(
        '[data-slot="data-table-surface"]'
      ) as HTMLElement;
      const table = document.querySelector(
        '[data-slot="data-table-table"]'
      ) as HTMLElement;
      const widths = Array.from(
        document.querySelectorAll("th[data-column-key]")
      ).map((el) => (el as HTMLElement).getBoundingClientRect().width);
      return {
        widths,
        tableWidth: table.getBoundingClientRect().width,
        surfaceWidth: surf.clientWidth,
      };
    });

  // Let the table settle before capturing the baseline. During WASM hydration
  // the table briefly renders narrower than its container; measuring then would
  // produce a bogus baseline. The table is laid out once it fills its surface.
  let baseline: number[] = [];
  await expect(async () => {
    const snap = await layoutSnapshot();
    expect(snap.tableWidth).toBeGreaterThanOrEqual(snap.surfaceWidth - 4);
    baseline = snap.widths;
  }).toPass({ timeout: 15000 });
  expect(baseline.length).toBeGreaterThan(0);

  // During steady-state scrolling the mounted row set changes constantly; with
  // fixed layout the column widths must not move.
  const maxScroll = await surface.evaluate(
    (el) => el.scrollHeight - el.clientHeight
  );
  for (let i = 1; i <= 8; i++) {
    await surface.evaluate((el, top) => {
      el.scrollTop = top;
    }, Math.round((maxScroll / 8) * i));
    await page.waitForTimeout(150);
    const { widths } = await layoutSnapshot();
    expect(widths.length).toBe(baseline.length);
    for (let col = 0; col < baseline.length; col++) {
      // The header and body share one fixed-layout table, so they can never
      // desync. The failure this guards against is the table reverting to auto
      // layout and reflowing columns by tens-to-hundreds of px as rows mount.
      // A few px of scrollbar-gutter / sub-pixel variance is expected and fine.
      expect(Math.abs(widths[col] - baseline[col])).toBeLessThanOrEqual(5);
    }
  }
});

// The "+ Filter" control opens a Popover containing filter inputs. Clicking the
// popover's own (non-focusable) background must NOT dismiss it: doing so blurs
// the focused input and the browser moves focus to the nearest focusable
// ancestor — which still contains the popover — and that must not be treated as
// an outside focus-out. Regression test for the popover light-dismiss logic.
const FILTER_URL = "/components/data_table";

async function openFilterPopover(page: Page) {
  const trigger = page.getByRole("button", { name: "Filter" }).first();
  await expect(trigger).toBeVisible({ timeout: 60000 });
  await trigger.click();
  const dialog = page.getByRole("dialog").first();
  await expect(dialog).toBeVisible();
  return dialog;
}

test("filter popover stays open when clicking its background", async ({
  page,
}) => {
  await page.goto(FILTER_URL);
  await openFilterPopover(page);

  // Find an in-viewport pixel whose top hit-test element is the dialog itself
  // (the popover background, not a control).
  const target = await page.evaluate(async () => {
    const d = document.querySelector('[role="dialog"][data-state="open"]');
    d.scrollIntoView({ block: "center" });
    await new Promise((r) => requestAnimationFrame(() => r(null)));
    const r = d.getBoundingClientRect();
    for (let y = r.top + 2; y < r.bottom - 2; y += 3) {
      if (y < 0 || y > window.innerHeight) continue;
      for (let x = r.left + 2; x < r.right - 2; x += 3) {
        if (x < 0 || x > window.innerWidth) continue;
        if (document.elementFromPoint(x, y) === d) return { x, y, ok: true };
      }
    }
    return { x: 0, y: 0, ok: false };
  });
  expect(target.ok, "found a background pixel of the popover").toBe(true);

  await page.mouse.click(target.x, target.y);
  await page.waitForTimeout(300);

  await expect(
    page.locator('[role="dialog"][data-state="open"]'),
    "background click must not dismiss the popover",
  ).toHaveCount(1);
});

test("filter popover still dismisses when clicking outside", async ({
  page,
}) => {
  await page.goto(FILTER_URL);
  await openFilterPopover(page);

  // Far corner of the document — genuinely outside the popover.
  await page.mouse.click(2, 2);
  await page.waitForTimeout(300);

  await expect(
    page.locator('[role="dialog"][data-state="open"]'),
    "outside click must dismiss the popover",
  ).toHaveCount(0);
});
