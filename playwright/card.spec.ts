import { expect, test, type Locator, type Page } from "@playwright/test";

const CARD_DEMO_URL = "/components/card/block#main";

async function gotoCardDemo(page: Page) {
  await page.goto(CARD_DEMO_URL, {
    timeout: 30 * 1000,
    waitUntil: "load",
  });
}
test("card roots expose section slots and accessible content", async ({
  page,
}) => {
  await gotoCardDemo(page);

  const preview = page.locator("#dx-preview-block-root");
  const cards = preview.locator('[data-slot="card"]');
  await expect(cards).toHaveCount(2);

  const firstCard = cards.nth(0);
  await expect(firstCard.locator('[data-slot="card-header"]')).toHaveCount(1);
  await expect(firstCard.locator('[data-slot="card-content"]')).toHaveCount(1);
  await expect(firstCard.locator('[data-slot="card-footer"]')).toHaveCount(1);
  const action = firstCard.locator('[data-slot="card-action"]');
  await expect(action).toHaveCount(1);
  await expect(action.getByRole("button", { name: "Sign Up" })).toBeVisible();
  await expect(cards.nth(1).locator('[data-slot="card-header"]')).toHaveCount(
    1,
  );
  await expect(cards.nth(1).locator('[data-slot="card-content"]')).toHaveCount(
    0,
  );
  await expect(cards.nth(1).locator('[data-slot="card-footer"]')).toHaveCount(
    0,
  );

  await expect(firstCard.getByRole("heading", { level: 3 })).toHaveText(
    "Login to your account",
  );
  await expect(firstCard.locator('[data-slot="card-description"]')).toHaveText(
    "Enter your email below to login to your account",
  );
  await expect(cards.nth(1).getByRole("heading", { level: 3 })).toHaveText(
    "Team workspace",
  );
  await expect(
    cards.nth(1).locator('[data-slot="card-description"]'),
  ).toHaveText("Invite collaborators and manage access.");
});

test("card login form controls and actions preserve browser semantics", async ({
  page,
}) => {
  await gotoCardDemo(page);

  const preview = page.locator("#dx-preview-block-root");
  const firstCard = preview.locator('[data-slot="card"]').first();
  const form = firstCard.locator('[data-slot="card-content"] #login-form');
  const email = form.locator("#email");
  const password = form.locator("#password");

  await expect(email).toHaveAttribute("type", "email");
  await expect(email).toHaveAttribute("placeholder", "m@example.com");
  await expect(password).toHaveAttribute("type", "password");
  await expect(password).not.toHaveAttribute("placeholder");
  await expect(form.locator('label[for="email"]')).toHaveText("Email");
  await expect(form.locator('label[for="password"]')).toHaveText("Password");
  await expect(email).toHaveAccessibleName("Email");
  await expect(password).toHaveAccessibleName("Password");

  const signUp = firstCard.getByRole("button", { name: "Sign Up" });
  const forgot = firstCard.getByRole("link", { name: "Forgot your password?" });
  const login = firstCard.getByRole("button", { name: "Login", exact: true });
  const google = firstCard.getByRole("button", { name: "Login with Google" });
  await expect(signUp).toHaveAttribute("type", "button");
  await expect(signUp).not.toHaveAttribute("form");
  await expect(forgot).toHaveAttribute("href", "#");
  await expect(login).toHaveAttribute("type", "submit");
  await expect(login).toHaveAttribute("form", "login-form");
  await expect(google).toHaveAttribute("type", "button");
  await expect(google).not.toHaveAttribute("form");

  await form.evaluate((element) => {
    element.addEventListener("submit", (event) => {
      event.preventDefault();
      (element as HTMLFormElement).dataset.submitted = "true";
    });
  });
  await login.click();
  await expect(form).toHaveAttribute("data-submitted", "true");
});

async function computedColorForCssColor(locator: Locator, color: string) {
  return locator.evaluate((element: Element, cssColor: string) => {
    const probe = document.createElement("span");
    probe.style.color = cssColor;
    probe.style.position = "absolute";
    probe.style.visibility = "hidden";
    element.appendChild(probe);
    const computedColor = getComputedStyle(probe).color;
    probe.remove();
    return computedColor;
  }, color);
}

test("card title and description use shared typography roles", async ({
  page,
}) => {
  await gotoCardDemo(page);

  const cards = page.locator('#dx-preview-block-root [data-slot="card"]');
  const shorthandCard = cards.nth(0);
  const wrapperCard = cards.nth(1);

  await assertCardTypography(shorthandCard, "P");
  await assertCardTypography(wrapperCard, "DIV");
});

async function assertCardTypography(
  card: Locator,
  descriptionTagName: "P" | "DIV",
) {
  const title = card.locator('[data-slot="card-title"]');
  const description = card.locator('[data-slot="card-description"]');

  await expect(card).toBeVisible();
  await expect(title).toHaveCount(1);
  await expect(description).toHaveCount(1);
  await expect(title).toHaveJSProperty("tagName", "H3");
  await expect(description).toHaveJSProperty("tagName", descriptionTagName);
  await expect(title).toHaveClass(/dx_heading/);
  await expect(title).toHaveAttribute("data-size", "md");
  await expect(title).toHaveAttribute("data-weight", "semibold");
  await expect(title).toHaveAttribute("data-tone", "default");
  await expect(title).toHaveAttribute("data-wrap", "wrap");
  await expect(title).toHaveAttribute("data-truncate", "false");
  await expect(description).toHaveClass(/dx_text/);
  await expect(description).toHaveAttribute("data-size", "sm");
  await expect(description).toHaveAttribute("data-tone", "surface-muted");
  await expect(description).toHaveAttribute("data-weight", "inherit");
  await expect(description).toHaveAttribute("data-wrap", "wrap");
  await expect(description).toHaveAttribute("data-truncate", "false");
  await expect(title.locator("h1,h2,h3,h4,h5,h6,p")).toHaveCount(0);
  await expect(description.locator("h1,h2,h3,h4,h5,h6,p")).toHaveCount(0);

  const [
    cardColor,
    titleColor,
    descriptionColor,
    surfaceMutedColor,
    titleMetrics,
  ] = await Promise.all([
    card.evaluate((element) => getComputedStyle(element).color),
    title.evaluate((element) => getComputedStyle(element).color),
    description.evaluate((element) => getComputedStyle(element).color),
    computedColorForCssColor(card, "var(--surface-muted-fg)"),
    title.evaluate((element) => {
      const style = getComputedStyle(element);
      return {
        fontSize: Number.parseFloat(style.fontSize),
        lineHeight: Number.parseFloat(style.lineHeight),
      };
    }),
  ]);
  expect(titleColor).toBe(cardColor);
  expect(descriptionColor).toBe(surfaceMutedColor);
  expect(titleMetrics.lineHeight).toBeCloseTo(titleMetrics.fontSize, 5);
}
