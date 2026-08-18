import { expect, test } from "@playwright/test";

async function gotoDemo(page: import("@playwright/test").Page, demo: string) {
  await page.goto(`/components/button_group/block#${demo}`, {
    timeout: 30_000,
    waitUntil: "load",
  });
}

const root = (page: import("@playwright/test").Page) =>
  page.locator("#dx-preview-block-root");

test.describe("button group", () => {
  test("joins buttons into a single vertical group with group semantics", async ({
    page,
  }) => {
    await gotoDemo(page, "vertical");
    const group = root(page)
      .getByRole("group", { name: "Adjust value" })
      .first();

    await expect(group).toHaveAttribute("data-orientation", "vertical");
    await expect(group).toHaveAttribute("role", "group");

    const increase = group.getByRole("button", { name: "Increase value" });
    const decrease = group.getByRole("button", { name: "Decrease value" });

    await expect(increase).toBeEnabled();
    await expect(decrease).toBeEnabled();

  });

  test("shares borders between horizontal members", async ({ page }) => {
    await gotoDemo(page, "main");
    const group = root(page)
      .getByRole("group")
      .filter({ has: page.getByRole("button", { name: "Archive" }) });
    await expect(group).toHaveAttribute("data-orientation", "horizontal");

    const archive = group.getByRole("button", { name: "Archive" });
    const report = group.getByRole("button", { name: "Report" });
    await expect(archive).toBeVisible();
    await expect(report).toBeVisible();

    await archive.click();
    await expect(root(page).getByTestId("last-action")).toHaveText(
      "Last action: Archive",
    );
  });

  test("keeps nested clusters visually distinct while members stay buttons", async ({
    page,
  }) => {
    await gotoDemo(page, "nested");
    const outer = root(page).getByRole("group").first();
    await expect(outer).toHaveAttribute("role", "group");
    const innerGroups = outer.getByRole("group");
    await expect(innerGroups).toHaveCount(2);
    await expect(outer).toHaveCSS("gap", /^(?!0px$).+/);

    const back = outer.getByRole("button", { name: "Back" });
    const next = outer.getByRole("button", { name: "Next" });
    const more = outer.getByRole("button", { name: "More" });

    for (const corner of [
      "border-top-left-radius",
      "border-top-right-radius",
      "border-bottom-left-radius",
      "border-bottom-right-radius",
    ]) {
      await expect(back).toHaveCSS(corner, /^(?!0px$).+/);
    }
    await expect(next).toHaveCSS("border-top-left-radius", /^(?!0px$).+/);
    await expect(next).toHaveCSS("border-bottom-left-radius", /^(?!0px$).+/);
    await expect(more).toHaveCSS("border-top-right-radius", /^(?!0px$).+/);
    await expect(more).toHaveCSS("border-bottom-right-radius", /^(?!0px$).+/);
  });

  test("renders a separator that splits the group", async ({ page }) => {
    await gotoDemo(page, "separator");
    const group = root(page).getByTestId("separator-group").first();
    await expect(group.getByRole("separator")).toHaveCount(1);
    await expect(group).toHaveAttribute("data-orientation", "horizontal");
  });

  test("merges an input shell with its action button", async ({ page }) => {
    await gotoDemo(page, "input");
    const group = root(page).getByRole("group").first();
    const shell = group.locator(".dx_input_wrapper > .dx_input");
    const search = group.getByRole("button", { name: "Search" });
    const input = shell.locator("input").first();
    const searchText = "release notes";

    await input.fill(searchText);
    await expect(root(page).getByTestId("search-value")).toHaveText(
      `Query: ${searchText}`,
    );
    await expect(search).toBeVisible();

    await expect(shell).toHaveCSS("border-top-left-radius", /^(?!0px$).+/);
    await expect(shell).toHaveCSS("border-bottom-left-radius", /^(?!0px$).+/);
    await expect(shell).toHaveCSS("border-top-right-radius", "0px");
    await expect(shell).toHaveCSS("border-bottom-right-radius", "0px");
    await expect(search).toHaveCSS("border-top-left-radius", "0px");
    await expect(search).toHaveCSS("border-bottom-left-radius", "0px");
    await expect(search).toHaveCSS("border-top-right-radius", /^(?!0px$).+/);
    await expect(search).toHaveCSS("border-bottom-right-radius", /^(?!0px$).+/);

    const shellBox = await shell.boundingBox();
    const searchBox = await search.boundingBox();
    expect(shellBox).not.toBeNull();
    expect(searchBox).not.toBeNull();
    expect(shellBox && searchBox).toBeTruthy();
    const overlap = shellBox!.x + shellBox!.width - searchBox!.x;
    expect(overlap).toBeGreaterThanOrEqual(0.5);
    expect(overlap).toBeLessThanOrEqual(1.5);
  });

  test("connects a dropdown trigger button to its group", async ({ page }) => {
    await gotoDemo(page, "main");
    const snooze = root(page).getByRole("button", { name: "Snooze" });
    const ellipsis = root(page).getByRole("button", {
      name: "More options",
    });
    await expect(snooze).toBeVisible();
    await expect(ellipsis).toHaveCSS("border-top-left-radius", "0px");
    await expect(ellipsis).toHaveCSS("border-bottom-left-radius", "0px");
    await expect(ellipsis).toHaveCSS("border-top-right-radius", /^(?!0px$).+/);
    await expect(ellipsis).toHaveCSS(
      "border-bottom-right-radius",
      /^(?!0px$).+/,
    );
  });

  test("opens a popover anchored to a button group", async ({ page }) => {
    await gotoDemo(page, "popover");
    await root(page).getByTestId("popover-trigger").click();
    await expect(page.getByTestId("popover-content")).toBeVisible();
  });

  test("keeps split groups functional alongside a chevron menu", async ({
    page,
  }) => {
    await gotoDemo(page, "split");
    const trigger = root(page).getByRole("button", { name: "More options" });
    await trigger.click();

    await expect(page.getByRole("menu")).toBeVisible();
    await page.getByRole("menuitem", { name: "Option 2" }).click();
    await expect(root(page)).toContainText("Selected option: option-2");
  });
});
