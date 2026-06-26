import { createReadStream, existsSync, readFileSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { extname, join, normalize, resolve } from "node:path";
import { spawn } from "node:child_process";

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

const rootArg = process.argv[2];
const portArg = process.argv[3];

if (!rootArg || !portArg) {
  console.error("Usage: node start-preview.mjs <public-dir> <port>");
  process.exit(1);
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

// On Unix, kill the whole process group (detached spawn sets pgid = pid).
// On Windows, fall back to build.kill() since process groups work differently.
function signalBuild(build, signal) {
  if (!isBuildRunning(build)) {
    return;
  }

  try {
    if (build.pid && process.platform !== "win32") {
      process.kill(-build.pid, signal);
      return;
    }

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

async function runBuild() {
  await new Promise((resolve, reject) => {
    buildProcess = spawn("dx", ["build", "--web"], {
      // detached = new process group so signalBuild can kill the whole tree.
      detached: process.platform !== "win32",
      stdio: "inherit",
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

  const port = Number.parseInt(portArg, 10);

  if (!Number.isInteger(port) || port <= 0) {
    throw new Error(`Invalid port: ${portArg}`);
  }

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
    process.exit(1);
  });
