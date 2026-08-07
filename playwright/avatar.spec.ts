import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("/components/avatar", { timeout: 30 * 1000 });
  let image = page.getByRole("img", { name: "User avatar" }).first();
  await expect(image).toHaveAttribute("src", "https://avatars.githubusercontent.com/u/66571940?s=96&v=4");

  await expect(page.getByLabel("Error avatar").getByText("JK")).toBeVisible();
});
