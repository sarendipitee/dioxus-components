import { execFileSync } from "node:child_process";
import path from "node:path";
import { defineConfig, devices } from "@playwright/test";

const runHeaded = process.env.PLAYWRIGHT_HEADED === "1";
const externalBaseUrl = process.env.PLAYWRIGHT_BASE_URL;
const chromiumExecutablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE;
const localBasePort = externalBaseUrl
  ? null
  : getLocalBasePort();
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
    return Number.parseInt(existingPort, 10);
  }

  const port = findAvailablePort();
  process.env.PLAYWRIGHT_LOCAL_BASE_PORT = String(port);
  return port;
}

export default defineConfig({
  testDir: ".",
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
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

  // Each test is given 5 minutes.
  timeout: 5 * 60 * 1000,

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
        cwd: path.join(process.cwd(), "../preview"),
        command: `node ../playwright/start-preview.mjs ../target/dx/preview/debug/web/public ${localBasePort}`,
        port: localBasePort,
        timeout: 50 * 60 * 1000,
        reuseExistingServer: false,
        stdout: "pipe",
      },
});
