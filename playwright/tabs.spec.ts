import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("/components/tabs/block#main");

  const activePanel = page.locator('[role="tabpanel"][data-state="active"]').first();
  const overviewTab = page.getByRole("tab", { name: "Overview" });
  const metricsTab = page.getByRole("tab", { name: "Metrics" });
  const filesTab = page.getByRole("tab", { name: "Files" });

  await expect(overviewTab).toHaveAttribute("aria-selected", "true");
  await expect(activePanel).toContainText("Overview content");

  // Automatic tabs should activate as focus moves with arrow navigation.
  await overviewTab.focus();
  await page.keyboard.press("ArrowRight");
  await expect(metricsTab).toBeFocused();
  await expect(metricsTab).toHaveAttribute("aria-selected", "true");
  await expect(activePanel).toContainText("Metrics content");

  await page.keyboard.press("ArrowRight");
  await expect(filesTab).toBeFocused();
  await expect(filesTab).toHaveAttribute("aria-selected", "true");
  await expect(activePanel).toContainText("Files content");

  await page.keyboard.press("ArrowRight");
  await expect(overviewTab).toBeFocused();
  await expect(overviewTab).toHaveAttribute("aria-selected", "true");
  await expect(activePanel).toContainText("Overview content");

  await filesTab.click();
  await expect(activePanel).toContainText("Files content");
  await metricsTab.click();
  await expect(activePanel).toContainText("Metrics content");
  await overviewTab.click();
  await expect(activePanel).toContainText("Overview content");
});
