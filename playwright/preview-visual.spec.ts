import { expect, test, type Page } from "@playwright/test";
import fs from "fs";
import path from "path";

type PreviewCase = {
  component: string;
  demo: string;
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

const skippedPreviewCases = new Map<string, string>([
  [
    "virtual_list/random_heights",
    "Adaptive virtual-list measurement can produce browser-dependent scroll content after first paint.",
  ],
  [
    "data_table/virtualized",
    "Virtualized row measurement and scroll position are browser-dependent after first paint; behavior is covered by data_table.spec.ts.",
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
      const demosDir = path.join(previewRoot, component, "demos");

      return fs.existsSync(metadataPath) && fs.existsSync(demosDir);
    })
    .sort((a, b) => a.localeCompare(b));
}

function demosFor(component: string): string[] {
  const demosDir = path.join(previewRoot, component, "demos");

  return fs
    .readdirSync(demosDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort((a, b) => a.localeCompare(b));
}

const previewCases: PreviewCase[] = previewComponents().flatMap((component) =>
  demosFor(component)
    .map((demo) => ({ component, demo }))
    .filter(
      (previewCase) =>
        !skippedPreviewCases.has(
          `${previewCase.component}/${previewCase.demo}`,
        ),
    ),
);

const systemDarkCases: PreviewCase[] = [
  { component: "button", demo: "main" },
  { component: "textarea", demo: "autosize" },
  { component: "time_picker", demo: "main" },
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
    demo: previewCase.demo,
  });

  if (theme) {
    params.set("dark_mode", String(theme === "dark"));
  }

  await page.goto(`/component/block/?${params}`, {
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
    `${previewCase.component}-${previewCase.demo}-${themeName}.png`,
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
      test(`${previewCase.component}/${previewCase.demo} ${theme}`, async ({
        page,
      }) => {
        await prepareVisualPage(page);
        await gotoPreviewCase(page, previewCase, theme);
        await snapshotPreviewCase(page, previewCase, theme);
      });
    }
  }

  for (const previewCase of systemDarkCases) {
    test(`${previewCase.component}/${previewCase.demo} system dark`, async ({
      page,
    }) => {
      await page.emulateMedia({ colorScheme: "dark" });
      await prepareVisualPage(page);
      await gotoPreviewCase(page, previewCase);
      await snapshotPreviewCase(page, previewCase, "system-dark");
    });
  }
});
