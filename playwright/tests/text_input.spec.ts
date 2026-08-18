import { expect, test } from "@playwright/test";

test("clearable TextInput clears controlled value", async ({ page }) => {
  await page.goto("/components/text_input/block#main", { timeout: 30 * 1000 });

  const email = page.getByRole("textbox", { name: "Email", exact: true });
  await email.fill("customer@example.com");
  const wrapper = email.locator(
    "xpath=ancestor::*[@data-slot='input-wrapper'][1]",
  );
  await wrapper.getByRole("button", { name: "Clear value" }).click();
  await expect(email).toHaveValue("");
  await expect(page.locator("#text-input-value")).toContainText("Value:");
});
