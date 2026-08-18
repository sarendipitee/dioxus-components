import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const workspaceRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

export function createKacheBuildEnvironment(callerEnv = process.env) {
  const result = spawnSync(
    "bash",
    ["-c", '. "$1/scripts/mise-env.sh" && env -0', "--", workspaceRoot],
    {
      encoding: "buffer",
      env: {
        ...callerEnv,
        DIOXUS_COMPONENTS_REQUIRE_KACHE: "1",
        MISE_PROJECT_ROOT: workspaceRoot,
      },
    },
  );

  if (result.error) {
    throw new Error(`Unable to establish the required Kache environment: ${result.error.message}`);
  }
  if (result.status !== 0) {
    const detail = result.stderr.toString("utf8").trim();
    throw new Error(
      `Unable to establish the required Kache environment${detail ? `: ${detail}` : ""}`,
    );
  }

  return Object.fromEntries(
    result.stdout
      .toString("utf8")
      .split("\0")
      .filter(Boolean)
      .map((entry) => {
        const separator = entry.indexOf("=");
        return [entry.slice(0, separator), entry.slice(separator + 1)];
      }),
  );
}