import { MICRO_HARNESS_COMPONENTS } from "./micro-harness-policy.mjs";
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import path from "node:path";
import { defineConfig, devices } from "@playwright/test";

const playwrightDir = __dirname;
const workspaceRoot = path.resolve(playwrightDir, "..");

const runHeaded = process.env.PLAYWRIGHT_HEADED === "1";
const externalBaseUrl = process.env.PLAYWRIGHT_BASE_URL;
const chromiumExecutablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE;
const localBasePort = externalBaseUrl ? null : getLocalBasePort();
const configuredTargetDir =
  process.env.PLAYWRIGHT_TARGET_DIR ?? process.env.CARGO_TARGET_DIR;
// Keep Cargo's target stable across the separate Playwright processes used by
// run-suite. Set PLAYWRIGHT_TARGET_DIR when concurrent runs need isolation.
const playwrightTargetDir = externalBaseUrl
  ? null
  : path.resolve(configuredTargetDir ?? path.join(workspaceRoot, "target"));
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
const SUPPORT_SPECS = ["**/kache-build-env.spec.mjs"];

const testsDir = path.join(playwrightDir, "tests");

function specPattern(component: string): string {
  const basename = existsSync(
    path.resolve(testsDir, `${component}.spec.ts`),
  )
    ? component
    : component.replaceAll("_", "-");
  return `**/${basename}.spec.ts`;
}

function getTargetComponent(args = process.argv): string | null {
  if (process.env.PLAYWRIGHT_TARGET_COMPONENT) {
    const envComp = process.env.PLAYWRIGHT_TARGET_COMPONENT.replace(/-/g, "_");
    if (MICRO_HARNESS_SPECS.has(envComp)) return envComp;
  }

  const selectedComponents = new Set<string>();

  for (const arg of args) {
    if (arg.startsWith("-")) continue;
    const specMatch = arg.match(/(?:^|[\\/])([^\\/]+)\.spec\.(?:ts|js|mjs)(?:$|:)/);
    if (specMatch) {
      const comp = specMatch[1].replace(/-/g, "_");
      if (MICRO_HARNESS_SPECS.has(comp)) {
        selectedComponents.add(comp);
      }
      continue;
    }
    const bareName = arg.trim().replace(/-/g, "_");
    if (MICRO_HARNESS_SPECS.has(bareName)) {
      selectedComponents.add(bareName);
    }
  }

  if (selectedComponents.size === 1) {
    return Array.from(selectedComponents)[0];
  }

  return null;
}

const targetComponent = getTargetComponent();

export default defineConfig({
  testDir: testsDir,
  testIgnore: [
    ...SUPPORT_SPECS,
    ...(previewOnly ? MICRO_HARNESS_COMPONENTS.map(specPattern) : []),
  ],
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
        // The preview web server starts only after dx finishes the WASM build.
        // A cold or cache-miss build can exceed Playwright's 60-second default.
        timeout: 5 * 60 * 1000,
        // Give start-preview time to stop dx cleanly before Playwright tears
        // down the web-server process group.
        gracefulShutdown: { signal: "SIGTERM", timeout: 60 * 1000 },
        cwd: targetComponent
          ? path.join(workspaceRoot, "test-harness")
          : path.join(workspaceRoot, "preview"),
        command: `exec node ${JSON.stringify(path.join(playwrightDir, "start-preview.mjs"))} ${JSON.stringify(path.join(playwrightTargetDir!, "dx", targetComponent ?? "preview", "debug/web/public"))} ${localBasePort} ${targetComponent ?? ""}`,
        port: localBasePort,
        stdout: "pipe",
        env: {
          ...process.env,
          CARGO_TARGET_DIR: playwrightTargetDir!,
          ...(process.env.PLAYWRIGHT_TARGET_DIR
            ? { PLAYWRIGHT_TARGET_DIR: playwrightTargetDir! }
            : {}),
          PLAYWRIGHT_TARGET_COMPONENT: targetComponent ?? "",
        },
      },
});
