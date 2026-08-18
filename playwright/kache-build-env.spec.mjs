import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { chmodSync, existsSync, mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { createKacheBuildEnvironment } from "./kache-build-env.mjs";

const launcherPath = fileURLToPath(new URL("./start-preview.mjs", import.meta.url));

function runLauncher({ rootDir, targetDir, environment, ownedTargetDir = "" }) {
  return new Promise((resolve) => {
    const child = spawn(process.execPath, [launcherPath, rootDir, "39587", "button"], {
      cwd: join(process.cwd(), "test-harness"),
      env: {
        ...process.env,
        ...environment,
        CARGO_TARGET_DIR: targetDir,
        PLAYWRIGHT_OWNED_TARGET_DIR: ownedTargetDir,
      },
      stdio: ["pipe", "pipe", "pipe"],
    });
    let stdout = "";
    let stderr = "";
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
      if (stdout.includes("Preview server ready")) child.kill("SIGTERM");
    });
    child.stderr.on("data", (chunk) => { stderr += chunk; });
    const timer = setTimeout(() => child.kill("SIGKILL"), 5000);
    child.on("exit", (code, signal) => {
      clearTimeout(timer);
      resolve({ code, signal, stdout, stderr });
    });
  });
}

const testRoot = mkdtempSync(join(tmpdir(), "dioxus-components-playwright-kache-"));

try {
  const binDir = join(testRoot, "bin");
  mkdirSync(binDir);
  const kache = join(binDir, "kache");
  const notKache = join(binDir, "not-kache");
  for (const executable of [kache, notKache]) {
    writeFileSync(executable, "#!/usr/bin/env sh\nexit 0\n");
    chmodSync(executable, 0o755);
  }

  const baseEnv = {
    HOME: testRoot,
    PATH: `${binDir}:${process.env.PATH}`,
    XDG_CONFIG_HOME: join(testRoot, "config"),
  };
  const environment = createKacheBuildEnvironment({
    ...baseEnv,
    CARGO_INCREMENTAL: "1",
  });

  assert.equal(environment.RUSTC_WRAPPER, kache);
  assert.equal(environment.CARGO_INCREMENTAL, "0");
  const dx = join(binDir, "dx");
  const capturePath = join(testRoot, "dx-env");
  writeFileSync(
    dx,
    `#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");
fs.writeFileSync(process.env.DX_CAPTURE, JSON.stringify({
  CARGO_INCREMENTAL: process.env.CARGO_INCREMENTAL,
  CARGO_TARGET_DIR: process.env.CARGO_TARGET_DIR,
  PLAYWRIGHT_OWNED_TARGET_DIR: process.env.PLAYWRIGHT_OWNED_TARGET_DIR,
  RUSTC_WRAPPER: process.env.RUSTC_WRAPPER,
  MISE_NO_ENV: process.env.MISE_NO_ENV,
  args: process.argv.slice(2),
}));
const root = path.join(process.env.CARGO_TARGET_DIR, "dx", "button", "debug", "web", "public");
fs.mkdirSync(path.join(root, "wasm"), { recursive: true });
fs.writeFileSync(path.join(root, "index.html"), '<script src="/wasm/button.js"></script>');
fs.writeFileSync(path.join(root, "wasm/button.js"), "");
fs.writeFileSync(path.join(root, "wasm/button_bg.wasm"), "");
`,
  );
  chmodSync(dx, 0o755);

  const targetDir = join(testRoot, "target");
  const outputRoot = join(targetDir, "dx/button/debug/web/public");
  const launchResult = await runLauncher({
    rootDir: outputRoot,
    targetDir,
    environment: {
      PATH: `${binDir}:${process.env.PATH}`,
      DX_CAPTURE: capturePath,
      CARGO_INCREMENTAL: "1",
      RUSTC_WRAPPER: "kache",
      MISE_NO_ENV: "caller-value",
      HOME: testRoot,
      XDG_CONFIG_HOME: join(testRoot, "config"),
    },
  });
  assert.equal(launchResult.code, 143);
  const captured = JSON.parse(readFileSync(capturePath, "utf8"));
  assert.equal(captured.CARGO_INCREMENTAL, "0");
  assert.equal(captured.RUSTC_WRAPPER, kache);
  assert.equal(captured.MISE_NO_ENV, "1");
  assert.deepEqual(captured.args, ["build", "--web", "--bin", "button"]);
  // CARGO_TARGET_DIR alone remains caller-owned, even when the launcher shuts down.
  assert.equal(existsSync(targetDir), true);
  const playwrightRunsRoot = join(process.cwd(), "target/playwright");
  mkdirSync(playwrightRunsRoot, { recursive: true });
  const ownedTargetDir = join(
    playwrightRunsRoot,
    "run-12345-123e4567-e89b-12d3-a456-426614174000",
  );
  mkdirSync(ownedTargetDir);
  const ownedResult = await runLauncher({
    rootDir: join(ownedTargetDir, "dx/button/debug/web/public"),
    targetDir: ownedTargetDir,
    ownedTargetDir,
    environment: {
      PATH: `${binDir}:${process.env.PATH}`,
      DX_CAPTURE: join(testRoot, "owned-dx-env"),
      RUSTC_WRAPPER: "kache",
      HOME: testRoot,
      XDG_CONFIG_HOME: join(testRoot, "config"),
    },
  });
  const ownedCaptured = JSON.parse(
    readFileSync(join(testRoot, "owned-dx-env"), "utf8"),
  );
  assert.equal(ownedCaptured.CARGO_TARGET_DIR, ownedTargetDir);
  assert.equal(ownedCaptured.PLAYWRIGHT_OWNED_TARGET_DIR, ownedTargetDir);
  assert.equal(ownedResult.code, 143);
  assert.equal(existsSync(ownedTargetDir), false);

  const invalidCapturePath = join(testRoot, "invalid-dx-env");
  const invalidTargetDir = join(
    playwrightRunsRoot,
    "run-12346-123e4567-e89b-12d3-a456-426614174001",
  );
  mkdirSync(invalidTargetDir);
  writeFileSync(join(invalidTargetDir, "marker"), "owned");
  const invalidResult = await runLauncher({
    rootDir: join(invalidTargetDir, "dx/button/debug/web/public"),
    targetDir: invalidTargetDir,
    ownedTargetDir: invalidTargetDir,
    environment: {
      PATH: `${binDir}:${process.env.PATH}`,
      DX_CAPTURE: invalidCapturePath,
      RUSTC_WRAPPER: notKache,
      HOME: testRoot,
      XDG_CONFIG_HOME: join(testRoot, "invalid-config"),
    },
  });
  assert.equal(invalidResult.code, 1);
  assert.match(invalidResult.stderr, /Playwright builds require Kache/);
  assert.equal(existsSync(invalidCapturePath), false);
  assert.equal(existsSync(invalidTargetDir), false);

  assert.equal(environment.KACHE_S3_ENDPOINT, "http://127.0.0.1:19000");
  assert.equal(environment.KACHE_S3_BUCKET, "shared-local");
  assert.equal(environment.KACHE_CONFIG, join(testRoot, "config/kache/config.toml"));

  const overriddenEnvironment = createKacheBuildEnvironment({
    ...baseEnv,
    CI: "true",
    KACHE_S3_ENDPOINT: "https://cache.example.test",
    KACHE_S3_BUCKET: "caller-bucket",
    KACHE_S3_PREFIX: "caller-prefix",
    RUSTC_WRAPPER: "kache",
  });
  assert.equal(overriddenEnvironment.RUSTC_WRAPPER, kache);
  assert.equal(overriddenEnvironment.KACHE_S3_ENDPOINT, "https://cache.example.test");
  assert.equal(overriddenEnvironment.KACHE_S3_BUCKET, "caller-bucket");
  assert.equal(overriddenEnvironment.KACHE_S3_PREFIX, "caller-prefix");

  assert.throws(
    () => createKacheBuildEnvironment({ ...baseEnv, RUSTC_WRAPPER: notKache }),
    /Playwright builds require Kache/,
  );

  console.log("Playwright Kache environment specs passed");
} finally {
  rmSync(testRoot, { recursive: true, force: true });
}