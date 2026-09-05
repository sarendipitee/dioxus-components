#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { MICRO_HARNESS_COMPONENTS } from "./micro-harness-policy.mjs";

const playwrightDir = dirname(fileURLToPath(import.meta.url));
const testsDir = join(playwrightDir, "tests");
const suiteTargetDir =
  process.env.PLAYWRIGHT_TARGET_DIR ??
  process.env.CARGO_TARGET_DIR ??
  join(playwrightDir, "../target");
const shardIndex = parseShardValue("PLAYWRIGHT_SHARD_INDEX", 1);
const shardTotal = parseShardValue("PLAYWRIGHT_SHARD_TOTAL", 1);

if (shardIndex > shardTotal) {
  throw new Error(
    `PLAYWRIGHT_SHARD_INDEX (${shardIndex}) exceeds PLAYWRIGHT_SHARD_TOTAL (${shardTotal})`,
  );
}

const assignedComponents = MICRO_HARNESS_COMPONENTS.filter(
  (_, index) => index % shardTotal === shardIndex - 1,
);

for (const component of assignedComponents) {
  const spec = componentSpec(component);
  runPlaywright(["test", spec, "--project=chromium"], {
    PLAYWRIGHT_TARGET_DIR: suiteTargetDir,
  });
}

const previewArgs = ["test", "--project=chromium"];
if (shardTotal > 1) {
  previewArgs.push(`--shard=${shardIndex}/${shardTotal}`);
}
runPlaywright(previewArgs, {
  PLAYWRIGHT_PREVIEW_ONLY: "1",
  PLAYWRIGHT_TARGET_DIR: suiteTargetDir,
});

function parseShardValue(name, fallback) {
  const rawValue = process.env[name];
  if (rawValue === undefined) return fallback;
  if (!/^\d+$/.test(rawValue)) {
    throw new Error(`${name} must be a positive integer, received: ${rawValue}`);
  }

  const value = Number(rawValue);
  if (!Number.isSafeInteger(value) || value < 1) {
    throw new Error(`${name} must be a positive integer, received: ${rawValue}`);
  }
  return value;
}

function componentSpec(component) {
  const basename = existsSync(join(testsDir, `${component}.spec.ts`))
    ? component
    : component.replaceAll("_", "-");
  const spec = `${basename}.spec.ts`;
  if (!existsSync(join(testsDir, spec))) {
    throw new Error(`Missing Playwright spec for micro harness: ${component}`);
  }
  return join(testsDir, spec);
}

function runPlaywright(args, extraEnv = {}) {
  const npx = process.platform === "win32" ? "npx.cmd" : "npx";
  const result = spawnSync(npx, ["playwright", ...args], {
    cwd: playwrightDir,
    stdio: "inherit",
    env: {
      ...process.env,
      ...extraEnv,
    },
  });
  if (result.error) throw result.error;
  if (result.status !== 0) process.exit(result.status ?? 1);
}
