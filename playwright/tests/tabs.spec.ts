import { test, expect } from "@playwright/test";

test.describe("Tabs", () => {
  test("exposes tab roles, reciprocal ARIA links, selection, and automatic keyboard behavior", async ({
    page,
  }) => {
    await page.goto("/components/tabs/block#main");

    const tablist = page.getByRole("tablist", { name: "Automatic tabs demo" });
    const tabs = tablist.getByRole("tab");
    await expect(tabs).toHaveCount(3);
    await expect(tablist).toHaveAttribute("aria-orientation", "horizontal");
    for (let index = 0; index < 3; index += 1) {
      const tab = tabs.nth(index);
      const panel = page.locator(
        `[role="tabpanel"]#${await tab.getAttribute("aria-controls")}`,
      );
      const tabId = await tab.getAttribute("id");
      const panelId = await panel.getAttribute("id");
      expect(tabId).toBeTruthy();
      expect(panelId).toBeTruthy();
      await expect(tab).toHaveAttribute("aria-controls", panelId!);
      await expect(panel).toHaveAttribute("aria-labelledby", tabId!);
      await expect(tab).toHaveAttribute("tabindex", index === 0 ? "0" : "-1");
      await expect(panel).toHaveAttribute("tabindex", "0");
    }
    await expect(tabs.nth(0)).toHaveAttribute("aria-selected", "true");
    await expect(tabs.nth(1)).toHaveAttribute("aria-selected", "false");

    await tabs.nth(0).focus();
    await page.keyboard.press("End");
    await expect(tabs.nth(2)).toBeFocused();
    await expect(tabs.nth(2)).toHaveAttribute("aria-selected", "true");
    await page.keyboard.press("ArrowRight");
    await expect(tabs.nth(0)).toBeFocused();
    await page.keyboard.press("Home");
    await expect(tabs.nth(0)).toBeFocused();
    await tabs.nth(1).click();
    await expect(tabs.nth(1)).toHaveAttribute("aria-selected", "true");
    await expect(
      page.locator(
        `[role="tabpanel"]#${await tabs.nth(1).getAttribute("aria-controls")}`,
      ),
    ).toBeVisible();
  });

  test("manual activation separates focus from selection and skips disabled tabs", async ({
    page,
  }) => {
    await page.goto("/components/tabs/block#manual");
    const tablist = page.getByRole("tablist", { name: "Manual tabs demo" });
    const overview = tablist.getByRole("tab", { name: "Overview" });
    const metrics = tablist.getByRole("tab", { name: "Metrics" });
    const files = tablist.getByRole("tab", { name: "Files" });
    await overview.focus();
    await page.keyboard.press("ArrowRight");
    await expect(metrics).toBeFocused();
    await expect(metrics).toHaveAttribute("aria-selected", "false");
    await expect(overview).toHaveAttribute("aria-selected", "true");
    await page.keyboard.press("ArrowRight");
    await expect(overview).toBeFocused();
    await expect(files).toBeDisabled();
    await expect(files).toHaveAttribute("data-disabled", "true");
    await expect(files).toHaveAttribute("tabindex", "-1");
    await files.click({ force: true });
    await metrics.focus();
    await page.keyboard.press("Enter");
    await expect(metrics).toHaveAttribute("aria-selected", "true");
    await overview.focus();
    await page.keyboard.press("Space");
    await expect(overview).toHaveAttribute("aria-selected", "true");
  });

  test("vertical tabs use vertical arrows and ignore horizontal arrows", async ({
    page,
  }) => {
    await page.goto("/components/tabs/block#vertical");
    const tablist = page.getByRole("tablist", { name: "Vertical tabs demo" });
    await expect(tablist).toHaveAttribute("aria-orientation", "vertical");
    const overview = tablist.getByRole("tab", { name: "Overview" });
    const activity = tablist.getByRole("tab", { name: "Activity" });
    await overview.focus();
    await page.keyboard.press("ArrowRight");
    await expect(overview).toBeFocused();
    await page.keyboard.press("ArrowDown");
    await expect(activity).toBeFocused();
    await expect(activity).toHaveAttribute("aria-selected", "true");
    await page.keyboard.press("ArrowUp");
    await expect(overview).toBeFocused();
  });

  test("controlled tabs count genuine changes, support deactivation, and retain inactive panels", async ({
    page,
  }) => {
    await page.goto("/components/tabs/block#controlled");
    const root = page.getByTestId("tabs-controlled-root");
    await expect(page.getByTestId("tabs-controlled-list")).toBeVisible();
    await expect(page.getByTestId("tabs-controlled-list")).toHaveAttribute(
      "role",
      "tablist",
    );
    await expect(
      page.getByTestId("tabs-controlled-overview-trigger"),
    ).toHaveAttribute("role", "tab");
    await expect(
      page.getByTestId("tabs-controlled-overview-panel"),
    ).toHaveAttribute("role", "tabpanel");
    const output = page.getByTestId("tabs-controlled-output");
    await expect(root).toHaveAttribute("aria-label", "Controlled tabs fixture");
    await expect(output).toHaveText(
      "Selected value: overview; Callback count: 0",
    );
    const metrics = root.getByRole("tab", { name: "Metrics" });
    await metrics.click();
    await expect(output).toHaveText(
      "Selected value: metrics; Callback count: 1",
    );
    await metrics.click();
    await expect(output).toHaveText("Selected value: none; Callback count: 2");
    await metrics.click();
    await expect(output).toHaveText(
      "Selected value: metrics; Callback count: 3",
    );
    await expect(
      page.getByTestId("tabs-controlled-overview-panel"),
    ).toBeHidden();
    await expect(root.locator('[role="tabpanel"]').first()).toBeAttached();
    await expect(root.locator('[role="tabpanel"]').first()).toHaveAttribute(
      "data-state",
      "inactive",
    );
  });

  test("controlled tabs add, select, and remove Reports dynamically", async ({
    page,
  }) => {
    await page.goto("/components/tabs/block#controlled");
    const root = page.getByTestId("tabs-controlled-root");
    const toggle = page.getByRole("button", { name: "Add reports tab" });
    await expect(root.getByRole("tab", { name: "Reports" })).toHaveCount(0);
    await toggle.click();
    const reports = root.getByRole("tab", { name: "Reports" });
    await expect(reports).toBeVisible();
    await reports.click();
    await expect(page.getByTestId("tabs-controlled-output")).toContainText(
      "Selected value: reports",
    );
    await expect(root.getByRole("tabpanel", { name: "Reports" })).toBeVisible();
    await toggle.click();
    await expect(reports).toHaveCount(0);
    await expect(page.getByTestId("tabs-controlled-output")).toContainText(
      "Selected value: overview",
    );
  });

  test("non-looping tabs stop roving focus at each boundary", async ({
    page,
  }) => {
    await page.goto("/components/tabs/block#pills");
    const tablist = page.getByTestId("tabs-nonloop-list");
    await expect(tablist).toHaveAttribute("aria-orientation", "horizontal");
    const account = tablist.getByRole("tab", { name: "Account" });
    const security = tablist.getByRole("tab", { name: "Security" });

    await account.focus();
    await page.keyboard.press("ArrowLeft");
    await expect(account).toBeFocused();
    await expect(account).toHaveAttribute("aria-selected", "true");

    await security.focus();
    await page.keyboard.press("ArrowRight");
    await expect(security).toBeFocused();
    await expect(account).toHaveAttribute("aria-selected", "true");
    await expect(security).toHaveAttribute("aria-selected", "false");
  });
});
