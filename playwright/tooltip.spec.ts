import { test, expect, type Page } from "@playwright/test";

const FRAME = "#component-preview-frame";

function fixture(page: Page) {
  const frame = page.locator(FRAME).first();
  return {
    frame,
    root: frame.getByTestId("tooltip-root"),
    trigger: frame.getByTestId("tooltip-trigger"),
    content: page.getByTestId("tooltip-content"),
  };
}

test.describe("Tooltip", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/components/tooltip");
  });

  test("starts closed and links the native trigger to the open tooltip", async ({ page }) => {
    const { root, trigger, content } = fixture(page);

    await expect(root).toHaveAttribute("data-tooltip-root", "main");
    await expect(trigger).toHaveRole("button");
    await expect(trigger).toHaveAttribute("type", "button");
    await expect(trigger).toHaveAttribute("data-tooltip-trigger", "main");
    await expect(content).toHaveCount(0);
    await expect(trigger).not.toHaveAttribute("aria-describedby");

    await trigger.hover();
    await expect(content).toBeVisible();
    await expect(content).toHaveRole("tooltip");
    await expect(content).toHaveAttribute("data-tooltip-content", "main");
    const id = await content.getAttribute("id");
    expect(id).toBeTruthy();
    await expect(trigger).toHaveAttribute("aria-describedby", id!);
  });

  test("hover and focus open; pointer exit, blur, and Escape close", async ({ page }) => {
    const { frame, trigger, content } = fixture(page);

    await trigger.hover();
    await expect(content).toBeVisible();
    await frame.hover({ position: { x: 2, y: 2 } });
    await expect(content).toHaveCount(0);

    await trigger.focus();
    await expect(content).toBeVisible();
    await trigger.blur();
    await expect(content).toHaveCount(0);

    await trigger.focus();
    await expect(content).toBeVisible();
    await trigger.press("Escape");
    await expect(content).toHaveCount(0);
    await expect(trigger).toBeFocused();
    await expect(trigger).not.toHaveAttribute("aria-describedby");
  });

  test("disabled trigger is unavailable and cannot open", async ({ page }) => {
    const { frame } = fixture(page);
    const trigger = frame.getByTestId("disabled-trigger");

    await expect(trigger).toBeDisabled();
    await expect(trigger).toHaveAttribute("aria-disabled", "true");
    await expect(trigger).toHaveAttribute("tabindex", "-1");
    await trigger.hover();
    await expect(page.getByText("Disabled tooltip content")).toHaveCount(0);
  });

  test("controlled changes are observable and closed content leaves the portal", async ({ page }) => {
    const { frame } = fixture(page);
    const toggle = frame.getByTestId("controlled-toggle");
    const state = frame.getByTestId("controlled-state");
    const callbackCount = frame.getByTestId("controlled-callback-count");
    const content = page.getByTestId("controlled-content");

    await expect(state).toHaveText("closed");
    await expect(callbackCount).toHaveText("0");
    await toggle.click();
    await expect(state).toHaveText("open");
    await expect(content).toBeVisible();
    await expect(callbackCount).toHaveText("1");
    await toggle.click();
    await expect(state).toHaveText("closed");
    await expect(content).toHaveCount(0);
  });

  test("content reports placement and removes animation for reduced motion", async ({ page }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
    const { trigger, content } = fixture(page);

    await trigger.hover();
    await expect(content).toBeVisible();
    await expect(content).toHaveAttribute("data-side", "left");
    await expect(content).toHaveAttribute("data-floating", "true");
    await expect(content).toHaveCSS("animation-name", "none");
  });
});
