import { MICRO_HARNESS_COMPONENTS } from "./micro-harness-policy.mjs";
import { createReadStream, existsSync, readFileSync, rmSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { dirname, extname, join, normalize, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";
import { createKacheBuildEnvironment } from "./kache-build-env.mjs";

// How long to wait for `dx build` to exit cleanly before sending SIGKILL.
const BUILD_SHUTDOWN_GRACE_MS = 1500;

const MIME_TYPES = new Map([
  [".css", "text/css; charset=utf-8"],
  [".html", "text/html; charset=utf-8"],
  [".js", "application/javascript; charset=utf-8"],
  [".json", "application/json; charset=utf-8"],
  [".png", "image/png"],
  [".svg", "image/svg+xml"],
  [".wasm", "application/wasm"],
  [".woff", "font/woff"],
  [".woff2", "font/woff2"],
]);

const SPA_ROUTE_EXTENSIONS = new Set(["", ".html"]);

const MICRO_HARNESS_COMPONENT_SET = new Set(MICRO_HARNESS_COMPONENTS);

const rootArg = process.argv[2];
const portArg = process.argv[3];
const cliComponentArg = process.argv[4] || "";
const envComponentArg = process.env.PLAYWRIGHT_TARGET_COMPONENT || "";

if (!rootArg || !portArg) {
  console.error("Usage: node start-preview.mjs <public-dir> <port> [component]");
  process.exit(1);
}
if (!/^\d+$/.test(portArg) || Number(portArg) <= 0 || Number(portArg) > 65_535) {
  console.error(`Invalid port: ${portArg}`);
  process.exit(1);
}

if (cliComponentArg && envComponentArg && cliComponentArg !== envComponentArg) {
  console.error(
    `Requested component mismatch: argument=${cliComponentArg}, environment=${envComponentArg}`,
  );
  process.exit(1);
}

const componentArg = cliComponentArg || envComponentArg;
const expectedTarget = componentArg || "preview";
const explicitPlaywrightTargetDir = process.env.PLAYWRIGHT_TARGET_DIR;
const targetDir = resolve(
  explicitPlaywrightTargetDir || process.env.CARGO_TARGET_DIR || join(process.cwd(), "../target"),
);
const playwrightRunsRoot = resolve(
  dirname(fileURLToPath(import.meta.url)),
  "../target/playwright",
);
const ownedTargetMarker = process.env.PLAYWRIGHT_OWNED_TARGET_DIR;
const ownedPlaywrightTargetDir =
  ownedTargetMarker === targetDir &&
  dirname(targetDir) === playwrightRunsRoot &&
  /^run-\d+-[0-9a-f-]{36}$/i.test(targetDir.slice(playwrightRunsRoot.length + 1))
    ? targetDir
    : null;

function cleanupOwnedPlaywrightTarget() {
  if (ownedPlaywrightTargetDir) {
    rmSync(ownedPlaywrightTargetDir, { recursive: true, force: true });
  }
}

// ─── Build phase ─────────────────────────────────────────────────────────────

let shuttingDown = false;
let buildProcess = null;
let buildExitPromise = null;
let shutdownRequest = null;
const signalListeners = new Map();
const stdinCloseHandler = () => handleSignal("SIGTERM", 0);

function isBuildRunning(build = buildProcess) {
  return Boolean(build && build.exitCode === null && !build.killed);
}

// The build is spawned directly (without a detached shell), so terminating the
// child is sufficient and lets Playwright reap the complete webServer tree.
function signalBuild(build, signal) {
  if (!isBuildRunning(build)) {
    return;
  }

  try {
    build.kill(signal);
  } catch (error) {
    // ESRCH = process already gone; ignore it.
    if (!(error instanceof Error) || !("code" in error) || error.code !== "ESRCH") {
      throw error;
    }
  }
}

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function shutdownBuild(signal) {
  const build = buildProcess;
  const exitPromise = buildExitPromise;

  if (!isBuildRunning(build) || !exitPromise) {
    return;
  }

  signalBuild(build, signal);

  // Give the build a moment to exit gracefully before escalating to SIGKILL.
  const exitedWithinGrace = await Promise.race([
    exitPromise.then(() => true, () => true),
    wait(BUILD_SHUTDOWN_GRACE_MS).then(() => false),
  ]);

  if (exitedWithinGrace) {
    return;
  }

  signalBuild(build, "SIGKILL");
  await exitPromise.catch(() => {});
}

async function handleSignal(signal, exitCode) {
  if (shuttingDown) {
    return;
  }

  shuttingDown = true;
  shutdownRequest = { exitCode };
  await shutdownBuild(signal);
  cleanupOwnedPlaywrightTarget();
  process.exit(exitCode);
}

// Register signal handlers before spawning the build so we can clean up if
// Playwright kills this process while the build is still running.
for (const [signal, exitCode] of [
  ["SIGINT", 130],
  ["SIGTERM", 143],
  ["SIGHUP", 129],
]) {
  const listener = () => handleSignal(signal, exitCode);
  signalListeners.set(signal, listener);
  process.on(signal, listener);
}

// Playwright pipes stdin and closes it when it wants the server to shut down.
process.stdin.on("end", stdinCloseHandler);
process.stdin.on("close", stdinCloseHandler);

if (process.stdin.isTTY) {
  process.stdin.resume();
}

function validateLaunchRequest() {
  if (componentArg && !MICRO_HARNESS_COMPONENT_SET.has(componentArg)) {
    throw new Error(`Component is not eligible for the micro harness: ${componentArg}`);
  }

  const rootDir = resolve(rootArg);
  const expectedRoot = resolve(
    targetDir,
    "dx",
    expectedTarget,
    "debug/web/public",
  );
  if (rootDir !== expectedRoot) {
    throw new Error(
      `Requested output mismatch: expected ${expectedRoot}, received ${rootDir}`,
    );
  }

  if (componentArg) {
    const harnessBin = join(process.cwd(), `src/bin/${componentArg}.rs`);
    if (!existsSync(harnessBin) || !statSync(harnessBin).isFile()) {
      throw new Error(`Micro-harness source binary does not exist: ${harnessBin}`);
    }
  }

  // A successful build must recreate its public tree. Removing prior output
  // prevents a no-op or misdirected build from serving stale same-target files.
  rmSync(rootDir, { recursive: true, force: true });
}

function validateBuiltOutput() {
  const rootDir = resolve(rootArg);
  if (!existsSync(rootDir) || !statSync(rootDir).isDirectory()) {
    throw new Error(`Built public directory does not exist: ${rootDir}`);
  }

  const indexPath = join(rootDir, "index.html");
  if (!existsSync(indexPath) || !statSync(indexPath).isFile()) {
    throw new Error(`Built output is missing index.html: ${indexPath}`);
  }

  const indexHtml = readFileSync(indexPath, "utf8");
  const scriptTargets = [
    ...indexHtml.matchAll(/(?:src=["'][^"']*\/wasm\/)([^/"']+)\.js["']/g),
  ].map((match) => match[1]);
  if (scriptTargets.length !== 1 || scriptTargets[0] !== expectedTarget) {
    throw new Error(
      `Built output identity mismatch: expected ${expectedTarget}, found ${scriptTargets.join(", ") || "none"}`,
    );
  }

  for (const asset of [
    join(rootDir, "wasm", `${expectedTarget}.js`),
    join(rootDir, "wasm", `${expectedTarget}_bg.wasm`),
  ]) {
    if (!existsSync(asset) || !statSync(asset).isFile()) {
      throw new Error(`Built output is missing target asset: ${asset}`);
    }
  }
}

async function runBuild() {
  validateLaunchRequest();
  const buildArgs = ["build", "--web"];
  if (componentArg) {
    buildArgs.push("--bin", componentArg);
    console.log(`[Playwright] Fast-path building micro-binary harness: ${componentArg}`);
  }

  // Keep Kache enabled. Kache 0.14.0 understands Dioxus's nested
  // RUSTC_WORKSPACE_WRAPPER invocation.
  // MISE_NO_ENV prevents Dioxus’s nested tool invocation from reloading Mise;
  // establish the repository’s canonical cache policy before that boundary.
  const buildEnv = { ...createKacheBuildEnvironment(), MISE_NO_ENV: "1" };

  await new Promise((resolve, reject) => {
    buildProcess = spawn("dx", buildArgs, {
      stdio: "inherit",
      env: buildEnv,
    });

    buildExitPromise = new Promise((resolveExit) => {
      buildProcess.once("exit", () => resolveExit());
    });

    buildProcess.once("error", reject);
    buildProcess.once("exit", (code, signal) => {
      buildProcess = null;
      buildExitPromise = null;

      if (signal) {
        reject(new Error(`dx build --web exited from ${signal}`));
        return;
      }

      if (code !== 0) {
        reject(new Error(`dx build --web exited with code ${code ?? "null"}`));
        return;
      }

      resolve();
    });
  });
  validateBuiltOutput();
}

function cleanupBuildHandlers() {
  for (const [signal, listener] of signalListeners) {
    process.off(signal, listener);
  }

  process.stdin.off("end", stdinCloseHandler);
  process.stdin.off("close", stdinCloseHandler);
}

// ─── Server phase ─────────────────────────────────────────────────────────────

function startPreviewServer(rootArg, portArg) {
  const rootDir = resolve(rootArg);

  if (!existsSync(rootDir) || !statSync(rootDir).isDirectory()) {
    throw new Error(`Preview directory does not exist: ${rootDir}`);
  }

  const port = Number(portArg);
  // Read index.html once at startup; served for every unknown path so the
  // Dioxus WASM router can handle client-side navigation.
  const indexHtml = readFileSync(join(rootDir, "index.html"));

  const server = createServer((req, res) => {
    const url = new URL(req.url ?? "/", `http://${req.headers.host ?? "127.0.0.1"}`);
    const pathname = url.pathname === "/" ? "/index.html" : url.pathname;

    // Strip leading `..` segments to prevent path traversal outside rootDir.
    const safePath = normalize(pathname).replace(/^(\.\.(\/|\\|$))+/, "");
    const filePath = join(rootDir, safePath);

    if (existsSync(filePath) && statSync(filePath).isFile()) {
      res.writeHead(200, {
        "Content-Type": MIME_TYPES.get(extname(filePath)) ?? "application/octet-stream",
        "Cache-Control": "no-cache",
      });
      createReadStream(filePath).pipe(res);
      return;
    }

    if (!SPA_ROUTE_EXTENSIONS.has(extname(pathname))) {
      res.writeHead(404, {
        "Content-Type": "text/plain; charset=utf-8",
        "Cache-Control": "no-cache",
      });
      res.end(`Asset not found: ${pathname}`);
      return;
    }

    // Unknown path → fall back to index.html for client-side routing.
    res.writeHead(200, {
      "Content-Type": "text/html; charset=utf-8",
      "Cache-Control": "no-cache",
    });
    res.end(indexHtml);
  });

  return new Promise((resolveServer, rejectServer) => {
    let serverShuttingDown = false;
    let serverClosed = false;

    const cleanupServerHandlers = () => {
      for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"]) {
        process.off(signal, serverSignalHandlers[signal]);
      }

      process.stdin.off("end", handleServerStdinEnd);
      process.stdin.off("close", handleServerStdinEnd);
    };

    const finalizeExit = (code = 0) => {
      cleanupServerHandlers();
      // Server.close has completed before this function is called on shutdown.
      cleanupOwnedPlaywrightTarget();
      process.exit(code);
    };

    const shutdown = (exitCode) => {
      if (serverClosed) {
        if (typeof exitCode === "number") {
          finalizeExit(exitCode);
        }
        return;
      }

      if (serverShuttingDown) {
        return;
      }

      serverShuttingDown = true;
      // Playwright gives this process a graceful-shutdown window so the
      // launcher can remove its owned target directory. Do not let an active
      // browser connection hold server.close() open until Playwright force-kills
      // the process group.
      server.closeAllConnections?.();
      server.close((error) => {
        serverClosed = true;


        if (error) {
          console.error(error);
          finalizeExit(1);
          return;
        }

        if (typeof exitCode === "number") {
          finalizeExit(exitCode);
        }
      });
    };

    const serverSignalHandlers = {
      SIGINT: () => shutdown(130),
      SIGTERM: () => shutdown(143),
      SIGHUP: () => shutdown(129),
    };

    const handleServerStdinEnd = () => shutdown(0);

    for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"]) {
      process.on(signal, serverSignalHandlers[signal]);
    }

    process.stdin.on("end", handleServerStdinEnd);
    process.stdin.on("close", handleServerStdinEnd);

    if (process.stdin.isTTY) {
      process.stdin.resume();
    }

    server.once("error", (error) => {
      cleanupServerHandlers();
      cleanupOwnedPlaywrightTarget();
      rejectServer(error);
    });

    server.listen(port, "127.0.0.1", () => {
      console.log(`Preview server ready at http://127.0.0.1:${port}`);
      resolveServer(server);
    });
  });
}

// ─── Entry point ──────────────────────────────────────────────────────────────

runBuild()
  .then(() => {
    // Build succeeded; swap signal handlers from build-phase to server-phase.
    cleanupBuildHandlers();
    return startPreviewServer(rootArg, portArg);
  })
  .catch((error) => {
    // Suppress errors caused by intentional shutdown (signal / stdin close).
    if (shutdownRequest) {
      return;
    }

    console.error(error instanceof Error ? error.message : error);
    cleanupOwnedPlaywrightTarget();
    process.exit(1);
  });
