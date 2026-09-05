import { test, expect, type Page } from "@playwright/test";

const URL = "/components/tree_view/block";
const LOAD_TIMEOUT = 30 * 1000;

async function loadTree(page: Page) {
  await page.goto(URL, { timeout: LOAD_TIMEOUT, waitUntil: "networkidle" });

  const tree = page.locator("#tree-view");
  await expect(tree).toBeVisible({ timeout: LOAD_TIMEOUT });
  return tree;
}

async function loadDataTree(page: Page) {
  await page.goto(`${URL}#data`, {
    timeout: LOAD_TIMEOUT,
    waitUntil: "networkidle",
  });

  const tree = page.locator("#tree-data-view");
  await expect(tree).toBeVisible({ timeout: LOAD_TIMEOUT });
  return tree;
}

test("renders an accessible styled tree", async ({ page }) => {
  const tree = await loadTree(page);

  await expect(tree).toHaveRole("tree");
  await expect(tree).toHaveAttribute("aria-label", "File explorer");
  await expect(tree).toHaveClass(/dx_tree_view/);
  await expect(tree.locator('[role="treeitem"]').first()).toHaveCSS(
    "border-radius",
    "8px",
  );
});

test("keeps leaf rows compact while aligning their labels", async ({ page }) => {
  const tree = await loadTree(page);
  const groupLabel = tree
    .getByTestId("tree-src")
    .locator(".dx_tree_view_label");
  const leafContent = tree.getByTestId("tree-readme");
  const leafLabel = leafContent.locator(".dx_tree_view_label");

  await expect(
    leafContent.locator(".dx_tree_view_indicator"),
  ).toHaveCSS("visibility", "hidden");
  await expect(leafContent).toHaveCSS("margin-inline-start", "0px");

  const [groupContentLeft, leafContentLeft, groupLabelLeft, leafLabelLeft] =
    await Promise.all([
      tree.getByTestId("tree-src").evaluate((element) =>
        element.getBoundingClientRect().left,
      ),
      leafContent.evaluate((element) => element.getBoundingClientRect().left),
    groupLabel.evaluate((element) => element.getBoundingClientRect().left),
    leafLabel.evaluate((element) => element.getBoundingClientRect().left),
    ]);
  expect(Math.abs(groupContentLeft - leafContentLeft)).toBeLessThanOrEqual(1);
  expect(Math.abs(groupLabelLeft - leafLabelLeft)).toBeLessThanOrEqual(1);
});

test("expands and collapses nested groups", async ({ page }) => {
  const tree = await loadTree(page);
  const src = tree.getByTestId("tree-src");
  const components = tree.getByTestId("tree-components");

  await expect(src).toHaveAttribute("aria-expanded", "true");
  await expect(components).toBeVisible();

  await src.click();
  await expect(src).toHaveAttribute("aria-expanded", "false");
  await expect(components).toBeHidden();

  await src.click();
  await expect(src).toHaveAttribute("aria-expanded", "true");
  await expect(components).toBeVisible();
});

test("supports tree keyboard navigation and branch commands", async ({
  page,
}) => {
  const tree = await loadTree(page);
  const src = tree.getByTestId("tree-src");
  const components = tree.getByTestId("tree-components");
  const button = tree.getByTestId("tree-button");
  const readme = tree.getByTestId("tree-readme");

  await src.focus();
  await page.keyboard.press("ArrowDown");
  await expect(components).toBeFocused();
  await page.keyboard.press("End");
  await expect(readme).toBeFocused();
  await page.keyboard.press("Home");
  await expect(src).toBeFocused();

  await src.click();
  await src.focus();
  await page.keyboard.press("ArrowRight");
  await expect(src).toHaveAttribute("aria-expanded", "true");
  await page.keyboard.press("ArrowLeft");
  await expect(src).toHaveAttribute("aria-expanded", "false");

  await src.focus();
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("ArrowRight");
  await expect(components).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(button).toBeFocused();
  await page.keyboard.press("ArrowLeft");
  await expect(components).toBeFocused();
});

test("exposes selection state on selected tree items", async ({ page }) => {
  const tree = await loadTree(page);
  const readme = tree.getByTestId("tree-readme");

  await expect(readme).toHaveAttribute("aria-selected", "true");
  await expect(readme).toHaveAttribute("aria-level", "1");
  await expect(readme).toHaveAttribute("aria-posinset", "2");
  await expect(readme).toHaveAttribute("aria-setsize", "2");
});

test("renders data-oriented trees through the controller", async ({ page }) => {
  const tree = await loadDataTree(page);

  await expect(tree).toHaveRole("tree");
  await expect(
    tree.locator('.dx_tree_view_item_content > .dx_tree_view_icon'),
  ).toHaveCount(6);
  await expect(
    tree.getByRole("treeitem", { name: "README.md" }),
  ).toHaveAttribute("aria-selected", "true");
  await expect(
    tree.getByRole("treeitem", { name: "components" }),
  ).toBeVisible();
  await expect(
    tree.getByRole("treeitem", { name: "button.rs" }),
  ).toBeVisible();

  await page.getByRole("button", { name: "Collapse all" }).click();
  await expect(
    tree.getByRole("treeitem", { name: "components" }),
  ).toBeHidden();

  await page.getByRole("button", { name: "Expand all" }).click();
  await expect(
    tree.getByRole("treeitem", { name: "components" }),
  ).toBeVisible();
  await expect(
    tree.getByRole("treeitem", { name: "button.rs" }),
  ).toBeVisible();
});
