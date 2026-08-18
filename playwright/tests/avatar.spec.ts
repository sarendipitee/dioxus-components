import { expect, test, type Page } from "@playwright/test";

const AVATAR_DEMO_URL = "/components/avatar";
const LOADED_AVATAR_URL =
  "https://avatars.githubusercontent.com/u/66571940?s=96&v=4";
const LOADING_AVATAR_URL = "https://httpbin.org/delay/3600";
const ERROR_AVATAR_URL = "https://invalid-url.example/image.jpg";
const ONE_PIXEL_PNG = Buffer.from(
  "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=",
  "base64",
);

async function loadAvatarDemo(page: Page) {
  await page.route(LOADED_AVATAR_URL, (route) =>
    route.fulfill({ contentType: "image/png", body: ONE_PIXEL_PNG }),
  );
  await page.route(ERROR_AVATAR_URL, (route) =>
    route.fulfill({
      contentType: "image/png",
      body: Buffer.from("invalid image"),
    }),
  );
  await page.route(LOADING_AVATAR_URL, () => {});
  await page.goto(AVATAR_DEMO_URL, {
    timeout: 30 * 1000,
    waitUntil: "domcontentloaded",
  });
}

test("renders a loaded avatar with its accessible image attributes", async ({
  page,
}) => {
  await loadAvatarDemo(page);

  const avatar = page.getByRole("img", { name: "Basic avatar", exact: true });
  const image = avatar.getByRole("img", { name: "User avatar", exact: true });

  await expect(avatar).toHaveAttribute("data-state", "loaded");
  await expect(image).toHaveAttribute("src", LOADED_AVATAR_URL);
  await expect(image).toHaveAttribute("draggable", "false");
});

test("keeps a loading avatar image present and decorative", async ({
  page,
}) => {
  await loadAvatarDemo(page);

  const avatar = page.getByRole("img", { name: "Loading avatar", exact: true });
  const image = avatar.locator("img");

  await expect(avatar).toHaveAttribute("data-state", "loading");
  await expect(image).toHaveCount(1);
  await expect(image).toHaveAttribute("alt", "");
});

test("renders the error fallback without an image", async ({ page }) => {
  await loadAvatarDemo(page);

  const avatar = page.getByRole("img", { name: "Error avatar", exact: true });

  await expect(avatar).toHaveAttribute("data-state", "error");
  await expect(avatar.getByText("JK", { exact: true })).toBeVisible();
  await expect(avatar.getByRole("img")).toHaveCount(0);
});

test("renders the empty fallback without an image", async ({ page }) => {
  await loadAvatarDemo(page);

  const avatar = page.getByRole("img", { name: "Empty avatar", exact: true });

  await expect(avatar).toHaveAttribute("data-state", "empty");
  await expect(avatar.getByText("NA", { exact: true })).toBeVisible();
  await expect(avatar.getByRole("img")).toHaveCount(0);
});
