// Wave 4b runtime verification for the unified overlay manager.
//
// Drives the /overlay-nesting matrix (all overlays rendered under a CSS-transformed
// ancestor that establishes a stacking-context + containing-block trap) and asserts
// the seven plan-§8 acceptance groups per case:
//   1. Renders at all via the single OverlayOutlet (proves portaling works).
//   2. Escapes the transform trap (correct viewport rect, not offset/clipped).
//   3. Z / paint order (nested above outer; floating above modal; toast above modal).
//   4. Independent dismiss (Escape / outside-click closes only the topmost overlay).
//   5. In-portal affordances (close buttons, menu items, select options, submenu).
//   6. Toast doesn't block the modal (toast container pointer-events:none).
//   7. Scroll-lock (locked while modal open, restored after, correct when nested).
//
// Run: BASE=http://127.0.0.1:PORT node playwright/overlay-nesting-verify.mjs
import { chromium } from 'playwright';

const BASE = process.env.BASE || 'http://localhost:8200';
const SHOT = process.env.SHOT ||
  '/Users/saren/tmp/claude-501/-Users-saren-Projects-dioxus-components/be7e04b1-dd31-4f54-a2ab-3818b9204dd0/scratchpad/shots';

const results = {};
const consoleErrors = [];

const tid = (t) => `[data-testid="${t}"]`;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
// Authoritative runtime-crash flag: any Rust panic / wasm `unreachable` aborts
// the Dioxus runtime. Track it so a case can report whether it crashed the app.
let runtimeCrashed = false;
const markCrash = (s) => { if (s.includes('panicked') || s.includes('unreachable')) runtimeCrashed = true; };
page.on('console', (m) => { if (m.type() === 'error') { const t = m.text().slice(0, 300); consoleErrors.push(t); markCrash(t); } });
page.on('pageerror', (e) => { const t = 'PAGEERROR: ' + String(e).slice(0, 300); consoleErrors.push(t); markCrash(t); });

async function shot(name) { await page.screenshot({ path: `${SHOT}/${name}.png` }); }

// Resolve a selector. Surfaces in the demo carry a `data-testid` whose value
// equals the documented `id` (the styled Dialog/Sheet routes the `id` prop to
// the logical root, not the portaled content div, so the content div ends up
// with an auto-generated id — the data-testid is the robust hook). So `#name`,
// `name`, and `[data-testid=...]` all resolve to the data-testid selector.
function sel(s) {
  if (s.startsWith('[')) return s;
  if (s.startsWith('#')) return tid(s.slice(1));
  return tid(s);
}

async function click(s) { await page.click(sel(s)); await page.waitForTimeout(350); }
async function present(s) { return (await page.locator(sel(s)).count()) > 0; }

async function rectOf(s) {
  return page.evaluate((q) => {
    const el = document.querySelector(q);
    if (!el) return null;
    const r = el.getBoundingClientRect();
    const cs = getComputedStyle(el);
    return {
      top: Math.round(r.top), left: Math.round(r.left),
      bottom: Math.round(r.bottom), right: Math.round(r.right),
      w: Math.round(r.width), h: Math.round(r.height),
      z: cs.zIndex, position: cs.position, visibility: cs.visibility,
      display: cs.display, pointerEvents: cs.pointerEvents,
      vw: innerWidth, vh: innerHeight,
    };
  }, sel(s));
}

// Effective paint order proxy: resolve the stacking z by walking computed
// z-index up the ancestor chain is unreliable across portals, so we use
// document.elementFromPoint at the element's center to verify what actually
// paints on top.
async function topAtCenterOf(s) {
  return page.evaluate((q) => {
    const el = document.querySelector(q);
    if (!el) return null;
    const r = el.getBoundingClientRect();
    const x = Math.round(r.left + r.width / 2);
    const y = Math.round(r.top + r.height / 2);
    const hit = document.elementFromPoint(x, y);
    if (!hit) return { hit: null };
    // Does the hit element belong to the queried element's subtree?
    const within = el.contains(hit) || hit.contains(el) || hit === el;
    // Identify which named surface the hit belongs to (closest id / testid).
    let node = hit, id = null, testid = null;
    while (node && node !== document.body) {
      if (!id && node.id) id = node.id;
      if (!testid && node.getAttribute && node.getAttribute('data-testid')) testid = node.getAttribute('data-testid');
      node = node.parentElement;
    }
    return { within, hitId: id, hitTestid: testid, x, y };
  }, sel(s));
}

async function bodyOverflow() {
  return page.evaluate(() => document.body.style.overflow || getComputedStyle(document.body).overflow);
}

// z-index numeric for a surface (its own computed z-index).
async function zOf(s) {
  return page.evaluate((q) => {
    const el = document.querySelector(q);
    if (!el) return null;
    const z = getComputedStyle(el).zIndex;
    return z === 'auto' ? null : Number.parseInt(z, 10);
  }, sel(s));
}

async function goto() {
  await page.goto(BASE + '/overlay-nesting', { waitUntil: 'domcontentloaded' });
  try { await page.waitForSelector(tid('overlay-nesting-transform-root'), { timeout: 30000 }); } catch (e) {}
  await page.waitForSelector(tid('open-dialog-1'), { timeout: 30000 });
  await page.waitForTimeout(800);
}

// The transformed trap root's bounding rect — overlays must NOT be offset by it.
async function trapRect() { return rectOf('overlay-nesting-transform-root'); }

function approxCentered(r, vw, vh, tol = 60) {
  const cx = (r.left + r.right) / 2;
  const cy = (r.top + r.bottom) / 2;
  return Math.abs(cx - vw / 2) <= tol && Math.abs(cy - vh / 2) <= Math.max(tol, vh / 3);
}
function inViewport(r) {
  return r.top >= -3 && r.left >= -3 && r.right <= r.vw + 3 && r.bottom <= r.vh + 3;
}

try {
  await goto();
  const trap = await trapRect();
  results._trapRoot = trap;

  // ============ CASE 1: Dialog-in-Dialog ============
  try {
    const c = {};
    await click('open-dialog-1');
    const r1 = await rectOf('#dialog-outer');
    c.renders = !!r1 && r1.visibility !== 'hidden';                       // G1
    c.outerRect = r1;
    // G2: portaled to body level, escapes transform. Outer dialog content div is
    // NOT a descendant of the transform root, and rect is centered in viewport.
    c.outerNotUnderTrap = await page.evaluate(() => {
      const d = document.querySelector('[data-testid="dialog-outer"]');
      const trapEl = document.querySelector('[data-testid="overlay-nesting-transform-root"]');
      return d && trapEl ? !trapEl.contains(d) : null;
    });
    c.outerCentered = r1 ? approxCentered(r1, r1.vw, r1.vh) : false;
    c.outerInViewport = r1 ? inViewport(r1) : false;
    await shot('case1-dialog-outer');

    // open inner
    await click('open-dialog-2');
    const r2 = await rectOf('#dialog-inner');
    c.innerRenders = !!r2 && r2.visibility !== 'hidden';
    c.innerRect = r2;
    // G3: inner paints above outer (elementFromPoint at inner center hits inner).
    c.innerOnTop = await topAtCenterOf('#dialog-inner');
    const zOuter = await zOf('#dialog-outer');
    const zInner = await zOf('#dialog-inner');
    c.zOuter = zOuter; c.zInner = zInner;
    c.zOrderOk = zInner != null && zOuter != null && zInner > zOuter;
    await shot('case1-dialog-in-dialog');

    // G7: scroll-lock with two modals open.
    c.overflowTwoOpen = await bodyOverflow();

    // G4: Escape closes only inner (topmost), outer stays.
    await page.keyboard.press('Escape'); await page.waitForTimeout(400);
    c.innerClosedAfterEsc = !(await present('#dialog-inner'));
    c.outerStillOpenAfterEsc = await present('#dialog-outer');
    c.overflowAfterInnerClose = await bodyOverflow();

    // G5: close button works on outer.
    await click('dialog-outer-close');
    c.outerClosedByButton = !(await present('#dialog-outer'));
    await page.waitForTimeout(300);
    c.overflowAfterAllClosed = await bodyOverflow();           // G7 restored

    results.dialog_in_dialog = c;
  } catch (e) { results.dialog_in_dialog = { ...(results.dialog_in_dialog || {}), _caseError: String(e).slice(0, 200) }; }

  // ============ CASE 2: Sheets ============
  try {
    const c = {};
    await click('open-sheet-1');
    const so = await rectOf('[data-testid="sheet-outer"]');
    c.outerRenders = !!so && so.visibility !== 'hidden';
    c.outerInViewport = so ? inViewport(so) : false;
    c.outerNotUnderTrap = await page.evaluate(() => {
      const d = document.querySelector('[data-testid="sheet-outer"]');
      const t = document.querySelector('[data-testid="overlay-nesting-transform-root"]');
      return d && t ? !t.contains(d) : null;
    });
    await shot('case2-sheet-outer');

    // sheet on sheet
    await click('open-sheet-2');
    const si = await rectOf('[data-testid="sheet-inner"]');
    c.innerRenders = !!si && si.visibility !== 'hidden';
    const zso = await zOf('[data-testid="sheet-outer"]');
    const zsi = await zOf('[data-testid="sheet-inner"]');
    c.zSheetOuter = zso; c.zSheetInner = zsi;
    c.sheetStackOk = zsi != null && zso != null && zsi >= zso;
    c.innerOnTop = await topAtCenterOf('[data-testid="sheet-inner"]');
    await shot('case2-sheet-on-sheet');

    // dismiss inner sheet only
    await page.keyboard.press('Escape'); await page.waitForTimeout(400);
    c.innerSheetClosedEsc = !(await present('[data-testid="sheet-inner"]'));
    c.outerSheetStillOpen = await present('[data-testid="sheet-outer"]');

    // dialog on sheet
    await click('open-dialog-on-sheet');
    const dos = await rectOf('#dialog-on-sheet');
    c.dialogOnSheetRenders = !!dos && dos.visibility !== 'hidden';
    const zsheet = await zOf('[data-testid="sheet-outer"]');
    const zdlg = await zOf('#dialog-on-sheet');
    c.zDialogOnSheet = zdlg; c.zSheetUnder = zsheet;
    c.dialogAboveSheet = zdlg != null && zsheet != null && zdlg > zsheet;
    c.dialogOnSheetOnTop = await topAtCenterOf('#dialog-on-sheet');
    await shot('case2-dialog-on-sheet');

    await click('dialog-on-sheet-close');
    c.dialogOnSheetClosed = !(await present('#dialog-on-sheet'));
    c.sheetStillOpenAfter = await present('[data-testid="sheet-outer"]');
    await click('sheet-outer-close');
    c.allSheetsClosed = !(await present('[data-testid="sheet-outer"]'));
    await page.waitForTimeout(300);
    c.overflowRestored = await bodyOverflow();
    results.sheets = c;
  } catch (e) { results.sheets = { ...(results.sheets || {}), _caseError: String(e).slice(0, 200) }; }

  // ============ CASE 3: Floating inside modal ============
  try {
    const c = {};
    await click('open-floating-host-dialog');
    c.hostDialogRenders = await present('#floating-host-dialog');
    c.overflowModalOpen = await bodyOverflow();

    // -- dropdown in dialog --
    await click('dropdown-in-dialog-trigger');
    const dd = await rectOf('[data-testid="dropdown-in-dialog-menu"]');
    c.dropdownRenders = !!dd && dd.visibility !== 'hidden';
    c.dropdownInViewport = dd ? inViewport(dd) : false;
    const zHost = await zOf('#floating-host-dialog');
    const zDd = await zOf('[data-testid="dropdown-in-dialog-menu"]');
    c.zHost = zHost; c.zDropdown = zDd;
    c.dropdownAboveDialog = zDd != null && zHost != null && zDd > zHost;
    c.dropdownOnTop = await topAtCenterOf('[data-testid="dropdown-in-dialog-menu"]');
    await shot('case3-dropdown-in-dialog');
    // G5: item selectable -> closes menu
    await click('dropdown-in-dialog-item-1');
    c.dropdownItemSelected = !(await present('[data-testid="dropdown-in-dialog-menu"]'));
    c.dialogStillOpenAfterItem = await present('#floating-host-dialog');

    // -- popover in dialog --
    await click('popover-in-dialog-trigger');
    const pv = await rectOf('[data-testid="popover-in-dialog-content"]');
    c.popoverRenders = !!pv && pv.visibility !== 'hidden';
    c.popoverInViewport = pv ? inViewport(pv) : false;
    const zPv = await zOf('[data-testid="popover-in-dialog-content"]');
    c.zPopover = zPv;
    c.popoverAboveDialog = zPv != null && zHost != null && zPv > zHost;
    c.popoverOnTop = await topAtCenterOf('[data-testid="popover-in-dialog-content"]');
    await shot('case3-popover-in-dialog');
    // G4: outside-click INSIDE the dialog (on the dialog title, outside the
    // popover panel) dismisses ONLY the popover; the dialog stays open. This is
    // the real union-predicate test: the press lands inside the dialog subtree
    // but outside the floating panel subtree.
    const titleBox = await page.evaluate(() => {
      const t = document.querySelector('[data-testid="floating-host-dialog"] .dx_dialog_title, [data-testid="floating-host-dialog"] h2');
      if (!t) return null;
      const r = t.getBoundingClientRect();
      return { x: Math.round(r.left + 10), y: Math.round(r.top + r.height / 2) };
    });
    if (titleBox) { await page.mouse.click(titleBox.x, titleBox.y); await page.waitForTimeout(350); }
    c.popoverDismissedByInsideOutsideClick = !(await present('[data-testid="popover-in-dialog-content"]'));
    c.dialogStillOpenAfterPopoverDismiss = await present('#floating-host-dialog');
    // also confirm Escape isolation: reopen, Escape closes popover not dialog
    if (await present('#floating-host-dialog')) {
      await click('popover-in-dialog-trigger');
      await page.keyboard.press('Escape'); await page.waitForTimeout(350);
      c.popoverDismissedEsc = !(await present('[data-testid="popover-in-dialog-content"]'));
      c.dialogStillOpenAfterEsc = await present('#floating-host-dialog');
    }

    // -- select in dialog --
    // Click the inner trigger button and scroll it into view first (the page
    // shell footer can overlap the select when it sits low in the dialog body).
    await page.evaluate(() => {
      const sel = document.querySelector('[data-testid="select-in-dialog"]');
      if (sel) sel.scrollIntoView({ block: 'center' });
    });
    await page.waitForTimeout(150);
    await page.click('[data-testid="select-in-dialog"] button'); await page.waitForTimeout(400);
    const slRect = await rectOf('[data-testid="select-in-dialog"]');
    // The listbox is a separate element; locate the popover/list rendered by Select.
    const selList = await page.evaluate(() => {
      const lists = [...document.querySelectorAll('[role="listbox"], .dx_select_list')];
      const vis = lists.find((l) => getComputedStyle(l).visibility !== 'hidden' && l.offsetParent !== null) || lists[0];
      if (!vis) return null;
      const r = vis.getBoundingClientRect();
      return { found: true, top: Math.round(r.top), left: Math.round(r.left), w: Math.round(r.width), h: Math.round(r.height), z: getComputedStyle(vis).zIndex, vw: innerWidth, vh: innerHeight };
    });
    c.selectListRenders = !!selList;
    c.selectList = selList;
    await shot('case3-select-in-dialog');
    // pick an option
    const picked = await page.evaluate(() => {
      const opts = [...document.querySelectorAll('[role="option"], .dx_select_option')].filter((o) => o.offsetParent !== null);
      if (!opts.length) return { ok: false, count: 0 };
      opts[0].click();
      return { ok: true, count: opts.length, text: (opts[0].textContent || '').trim() };
    });
    await page.waitForTimeout(350);
    c.selectOptionPick = picked;
    c.dialogStillOpenAfterSelect = await present('#floating-host-dialog');

    // -- nested dialog: floating above deeper modal --
    await click('open-nested-dialog');
    c.nestedDialogRenders = await present('#floating-nested-dialog');
    const zNested = await zOf('#floating-nested-dialog');
    await click('popover-in-nested-dialog-trigger');
    const zPvN = await zOf('[data-testid="popover-in-nested-dialog-content"]');
    c.zNestedDialog = zNested; c.zPopoverNested = zPvN;
    c.popoverAboveNestedDialog = zPvN != null && zNested != null && zPvN > zNested;
    c.popoverNestedOnTop = await topAtCenterOf('[data-testid="popover-in-nested-dialog-content"]');
    await shot('case3-popover-in-nested-dialog');
    await click('popover-in-nested-dialog-close');
    await click('floating-nested-dialog-close');
    c.nestedDialogClosed = !(await present('#floating-nested-dialog'));
    c.hostStillOpenAfterNested = await present('#floating-host-dialog');
    await click('floating-host-dialog-close');
    c.hostClosed = !(await present('#floating-host-dialog'));
    await page.waitForTimeout(300);
    c.overflowRestored = await bodyOverflow();
    results.floating_in_modal = c;
  } catch (e) { results.floating_in_modal = { ...(results.floating_in_modal || {}), _caseError: String(e).slice(0, 200) }; }

  // ============ CASE 5: Toast over modal ============
  // NOTE: run BEFORE the submenu case. The submenu-in-dialog case currently
  // crashes the wasm runtime on nested-item select (see escalation in the run
  // summary); running it last keeps every other group's evidence intact.
  try {
    const c = {};
    await click('open-toast-host-dialog');
    c.hostRenders = await present('#toast-host-dialog');
    await click('fire-toast');
    await page.waitForTimeout(500);
    const toast = await page.evaluate(() => {
      // This library's Toast renders with `dx_toast_container` / `dx_toast`
      // classes (it does NOT use the sonner data attributes).
      const cont = document.querySelector('.dx_toast_container, [data-sonner-toaster]');
      const t = document.querySelector('.dx_toast, [data-sonner-toast]');
      if (!cont) return { containerFound: false };
      const ccs = getComputedStyle(cont);
      return {
        containerFound: true, toastFound: !!t,
        containerPointerEvents: ccs.pointerEvents,
        containerZ: ccs.zIndex,
        toastPointerEvents: t ? getComputedStyle(t).pointerEvents : null,
        toastRect: t ? { top: Math.round(t.getBoundingClientRect().top), left: Math.round(t.getBoundingClientRect().left) } : null,
      };
    });
    c.toast = toast;
    c.toastShows = toast.containerFound && toast.toastFound;
    // G6: toast container pointer-events: none -> modal not blocked
    c.toastContainerPointerNone = toast.containerPointerEvents === 'none';
    // G3: toast z above modal band
    const zHost = await zOf('#toast-host-dialog');
    c.zToastContainer = toast.containerZ; c.zHostDialog = zHost;
    c.toastAboveModal = toast.containerZ && zHost != null && Number.parseInt(toast.containerZ, 10) > zHost;
    await shot('case5-toast-over-modal');
    // G6: the dialog's close button is still clickable while toast is shown
    let closeClickable = false;
    try {
      await page.click('[data-testid="toast-host-dialog-close"]', { timeout: 4000 });
      await page.waitForTimeout(350);
      closeClickable = !(await present('#toast-host-dialog'));
    } catch (e) { closeClickable = false; c.closeClickError = String(e).slice(0, 150); }
    c.closeClickableWithToast = closeClickable;
    await page.waitForTimeout(300);
    c.overflowRestored = await bodyOverflow();
    results.toast_over_modal = c;
  } catch (e) { results.toast_over_modal = { ...(results.toast_over_modal || {}), _caseError: String(e).slice(0, 200) }; }

  // ============ CASE 4: Submenu triple-nest (RUN LAST — see note above) ======
  try {
    const c = {};
    await click('open-submenu-host-dialog');
    c.hostRenders = await present('#submenu-host-dialog');
    await click('submenu-trigger');
    const menu = await rectOf('[data-testid="submenu-menu"]');
    c.menuRenders = !!menu && menu.visibility !== 'hidden';
    c.menuInViewport = menu ? inViewport(menu) : false;
    await shot('case4-submenu-menu');
    // hover the subtrigger to open subcontent across portal boundary
    await page.hover('[data-testid="submenu-subtrigger"]'); await page.waitForTimeout(600);
    const sub = await rectOf('[data-testid="submenu-subcontent"]');
    c.subcontentRenders = !!sub && sub.visibility !== 'hidden';
    c.subcontentInViewport = sub ? inViewport(sub) : false;
    await shot('case4-submenu-subcontent');
    // click the nested item -> updates submenu-picked (proves ctx across 2 portal hops)
    if (await present('[data-testid="submenu-nested-item"]')) {
      await page.click('[data-testid="submenu-nested-item"]'); await page.waitForTimeout(1200);
    }
    const pickedText = await page.evaluate(() => {
      const el = document.querySelector('[data-testid="submenu-picked"]');
      return el ? el.textContent.trim() : null;
    });
    c.pickedText = pickedText;
    c.nestedItemSelected = pickedText === 'Picked: nested';
    c.dialogStillOpenAfterPick = await present('#submenu-host-dialog');
    await shot('case4-submenu-picked');
    // Check for a runtime panic provoked by the nested-item select.
    c.runtimePanic = runtimeCrashed;
    // cleanup
    if (await present('#submenu-host-dialog')) {
      try { await click('submenu-host-dialog-close'); } catch (_) {}
    }
    results.submenu_triple_nest = c;
  } catch (e) { results.submenu_triple_nest = { ...(results.submenu_triple_nest || {}), _caseError: String(e).slice(0, 200) }; }

  results.consoleErrors = [...new Set(consoleErrors)].slice(0, 40);
} catch (e) {
  results._fatal = String(e.stack || e).slice(0, 600);
  results.consoleErrors = [...new Set(consoleErrors)].slice(0, 40);
}

console.log('=====OVERLAY-NESTING RESULTS=====');
console.log(JSON.stringify(results, null, 2));
await browser.close();
