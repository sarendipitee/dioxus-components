import { expect, test, type Page } from "@playwright/test";
import fs from "fs";
import path from "path";

type PreviewCase = {
  component: string;
  variant: string;
};

type WorkspaceManifest = {
  members?: string[];
};

const repoRoot =
  path.basename(process.cwd()) === "playwright"
    ? path.resolve(process.cwd(), "..")
    : process.cwd();
const previewRoot = path.join(repoRoot, "preview/src/components");
const workspaceManifestPath = path.join(repoRoot, "component.json");
const BASE = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:8080";

const skippedPreviewCases = new Map<string, string>([
  [
    "virtual_list/random_heights",
    "Adaptive virtual-list measurement can produce browser-dependent scroll content after first paint.",
  ],
]);

function previewComponents(): string[] {
  const manifest = JSON.parse(
    fs.readFileSync(workspaceManifestPath, "utf8"),
  ) as WorkspaceManifest;

  return (manifest.members ?? [])
    .filter((member) => member.startsWith("preview/src/components/"))
    .map((member) => path.basename(member))
    .filter((component) => {
      const metadataPath = path.join(previewRoot, component, "component.json");
      const variantsDir = path.join(previewRoot, component, "variants");

      return fs.existsSync(metadataPath) && fs.existsSync(variantsDir);
    })
    .sort((a, b) => a.localeCompare(b));
}

function variantsFor(component: string): string[] {
  const variantsDir = path.join(previewRoot, component, "variants");

  return fs
    .readdirSync(variantsDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort((a, b) => a.localeCompare(b));
}

const previewCases: PreviewCase[] = previewComponents().flatMap((component) =>
  variantsFor(component)
    .map((variant) => ({ component, variant }))
    .filter(
      (previewCase) =>
        !skippedPreviewCases.has(
          `${previewCase.component}/${previewCase.variant}`,
        ),
    ),
);

const systemDarkCases: PreviewCase[] = [
  { component: "button", variant: "main" },
  { component: "textarea", variant: "autosize" },
  { component: "time_picker", variant: "main" },
];

async function prepareVisualPage(page: Page) {
  await page.context().clearCookies();
  await page.addInitScript(() => {
    window.localStorage.clear();
  });
}

async function stabilizeVisualPage(page: Page) {
  await page.addStyleTag({
    content: `
      *,
      *::before,
      *::after {
        animation-delay: 0s !important;
        animation-duration: 0s !important;
        caret-color: transparent !important;
        transition-delay: 0s !important;
        transition-duration: 0s !important;
      }
    `,
  });
}

async function gotoPreviewCase(
  page: Page,
  previewCase: PreviewCase,
  theme?: "light" | "dark",
) {
  const params = new URLSearchParams({
    name: previewCase.component,
    variant: previewCase.variant,
  });

  if (theme) {
    params.set("dark_mode", String(theme === "dark"));
  }

  await page.goto(`${BASE}/component/block/?${params}`, {
    waitUntil: "domcontentloaded",
  });
  await stabilizeVisualPage(page);
  await page.locator("body").waitFor({ state: "visible" });
  await page.evaluate(() => document.fonts.ready);
}

async function snapshotPreviewCase(
  page: Page,
  previewCase: PreviewCase,
  themeName: string,
) {
  await expect(page.locator("body")).toHaveScreenshot(
    `${previewCase.component}-${previewCase.variant}-${themeName}.png`,
    {
      animations: "disabled",
      caret: "hide",
      maxDiffPixelRatio: 0.01,
    },
  );
}

test.describe("preview visual baselines", () => {
  test.use({ viewport: { width: 1024, height: 768 } });

  for (const previewCase of previewCases) {
    for (const theme of ["light", "dark"] as const) {
      test(`${previewCase.component}/${previewCase.variant} ${theme}`, async ({
        page,
      }) => {
        await prepareVisualPage(page);
        await gotoPreviewCase(page, previewCase, theme);
        await snapshotPreviewCase(page, previewCase, theme);
      });
    }
  }

  for (const previewCase of systemDarkCases) {
    test(`${previewCase.component}/${previewCase.variant} system dark`, async ({
      page,
    }) => {
      await page.emulateMedia({ colorScheme: "dark" });
      await prepareVisualPage(page);
      await gotoPreviewCase(page, previewCase);
      await snapshotPreviewCase(page, previewCase, "system-dark");
    });
  }
});
