import { MICRO_HARNESS_COMPONENTS } from "./micro-harness-policy.mjs";
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import path from "node:path";
import { defineConfig, devices } from "@playwright/test";

const runHeaded = process.env.PLAYWRIGHT_HEADED === "1";
const externalBaseUrl = process.env.PLAYWRIGHT_BASE_URL;
const chromiumExecutablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE;
const localBasePort = externalBaseUrl ? null : getLocalBasePort();
const playwrightTargetDir = externalBaseUrl
  ? null
  : path.resolve(process.cwd(), `../target/playwright/${localBasePort}`);
const baseURL = externalBaseUrl ?? `http://127.0.0.1:${localBasePort}`;

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
// import dotenv from 'dotenv';
// import path from 'path';
// dotenv.config({ path: path.resolve(__dirname, '.env') });

function findAvailablePort() {
  const script = `
    const net = require("node:net");
    const server = net.createServer();
    server.unref();
    server.once("error", (error) => {
      process.stderr.write(String(error) + "\\n");
      process.exit(1);
    });
    server.listen({ host: "127.0.0.1", port: 0, exclusive: true }, () => {
      const address = server.address();
      if (!address || typeof address === "string") {
        process.stderr.write("Failed to resolve Playwright preview port\\n");
        process.exit(1);
      }
      server.close(() => {
        process.stdout.write(String(address.port));
      });
    });
  `;

  return Number.parseInt(
    execFileSync(process.execPath, ["-e", script], {
      encoding: "utf8",
    }).trim(),
    10,
  );
}
function getLocalBasePort() {
  const existingPort = process.env.PLAYWRIGHT_LOCAL_BASE_PORT;
  if (existingPort) {
    if (!/^\d+$/.test(existingPort)) {
      throw new Error(`Invalid PLAYWRIGHT_LOCAL_BASE_PORT: ${existingPort}`);
    }

    const port = Number(existingPort);
    if (!Number.isInteger(port) || port <= 0 || port > 65_535) {
      throw new Error(`Invalid PLAYWRIGHT_LOCAL_BASE_PORT: ${existingPort}`);
    }
    return port;
  }

  const port = findAvailablePort();
  process.env.PLAYWRIGHT_LOCAL_BASE_PORT = String(port);
  return port;
}

// The micro harness only implements isolated block routes. The shared policy
// keeps config selection and launcher validation fail-closed together.
const MICRO_HARNESS_SPECS = new Set(MICRO_HARNESS_COMPONENTS);
const previewOnly = process.env.PLAYWRIGHT_PREVIEW_ONLY === "1";

function specPattern(component: string): string {
  const basename = existsSync(
    path.resolve(process.cwd(), `${component}.spec.ts`),
  )
    ? component
    : component.replaceAll("_", "-");
  return `**/${basename}.spec.ts`;
}

function getTargetComponent(args = process.argv): string | null {
  const selectedSpecs = args.flatMap((arg) => {
    const match = arg.match(/(^|.*[\\/])([^\\/]+)\.spec\.ts(?:$|:)/);
    if (!match) return [];

    const specPath = path.resolve(match[1] || ".", `${match[2]}.spec.ts`);
    return [{
      component: match[2].replace(/-/g, "_"),
      specPath,
    }];
  });

  if (selectedSpecs.length === 0) return null;

  const uniqueSpecPaths = new Set(selectedSpecs.map(({ specPath }) => specPath));
  if (uniqueSpecPaths.size !== 1) return null;

  const [{ component, specPath }] = selectedSpecs;
  const canonicalSpecPath = path.resolve(process.cwd(), `${path.basename(specPath)}`);
  if (specPath !== canonicalSpecPath) return null;

  return MICRO_HARNESS_SPECS.has(component) ? component : null;
}

const targetComponent = getTargetComponent();

export default defineConfig({
  testDir: ".",
  testIgnore: previewOnly
    ? MICRO_HARNESS_COMPONENTS.map(specPattern)
    : undefined,
  testMatch: targetComponent ? [specPattern(targetComponent)] : undefined,
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* WASM pages are memory-intensive; parallel browsers destabilize long suites. */
  workers: 1,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: process.env.CI ? [["list"], ["html", { open: "never" }]] : "list",
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    headless: !runHeaded,

    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL,

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: "on-first-retry",
  },

  // Bound failed tests; individual assertions may use shorter limits.
  timeout: 60 * 1000,

  /* Configure projects for major browsers */
  projects: [
    {
      name: "chromium",
      grepInvert: /mobile/,
      use: {
        ...devices["Desktop Chrome"],
        ...(chromiumExecutablePath
          ? { launchOptions: { executablePath: chromiumExecutablePath } }
          : {}),
      },
    },

    {
      name: "firefox",
      grepInvert: /mobile/,
      use: { ...devices["Desktop Firefox"] },
    },

    {
      name: "webkit",
      grepInvert: /mobile/,
      use: { ...devices["Desktop Safari"] },
      // Webkit is slower, so we give it more time.
      expect: {
        timeout: 30 * 1000, // 30 seconds
      },
    },

    // Temporarily disabled mobile tests in CI. The mobile browser CI downloads acts different than the local tests which pass
    // /* Test against mobile viewports. */
    // {
    //   name: "Mobile Chrome",
    //   grep: /mobile/,
    //   use: { ...devices["Pixel 5"] },
    // },

    // {
    //   name: "Mobile Safari",
    //   grep: /mobile/,
    //   use: { ...devices["iPhone 12"] },
    // },
  ],

  /* Run your local dev server before starting the tests */
  webServer: externalBaseUrl
    ? undefined
    : {
        cwd: targetComponent
          ? path.join(process.cwd(), "../test-harness")
          : path.join(process.cwd(), "../preview"),
        command: `exec node ../playwright/start-preview.mjs ${playwrightTargetDir}/dx/${targetComponent ?? "preview"}/debug/web/public ${localBasePort} ${targetComponent ?? ""}`,
        port: localBasePort,
        timeout: 50 * 60 * 1000,
        reuseExistingServer: false,
        stdout: "pipe",
        env: {
          ...process.env,
          CARGO_TARGET_DIR: playwrightTargetDir!,
          PLAYWRIGHT_TARGET_COMPONENT: targetComponent ?? "",
        },
      },
});
