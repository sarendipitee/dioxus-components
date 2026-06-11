#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";

const themePath = path.resolve(
  import.meta.dirname,
  "../themes/default.css",
);

const textContrastThreshold = 4.5;
const focusContrastThreshold = 3;

const textPairs = [
  ["surface", "surface-fg"],
  ["surface-muted", "surface-muted-fg"],
  ["surface-hover", "surface-hover-fg"],
  ["surface-active", "surface-active-fg"],
  ["surface-selected", "surface-selected-fg"],
  ["input", "input-fg"],
  ["input-hover", "input-hover-fg"],
  ["input-focus", "input-focus-fg"],
  ["overlay", "overlay-fg"],
  ["accent", "accent-fg"],
  ["accent-hover", "accent-hover-fg"],
  ["accent-active", "accent-active-fg"],
  ["accent-subtle", "accent-subtle-fg"],
  ["success", "success-fg"],
  ["success-subtle", "success-subtle-fg"],
  ["warning", "warning-fg"],
  ["warning-subtle", "warning-subtle-fg"],
  ["danger", "danger-fg"],
  ["danger-hover", "danger-hover-fg"],
  ["danger-active", "danger-active-fg"],
  ["danger-subtle", "danger-subtle-fg"],
  ["info", "info-fg"],
  ["info-subtle", "info-subtle-fg"],
];

const focusAdjacentPairs = ["surface", "input", "overlay"];

const themeCss = fs.readFileSync(themePath, "utf8");

function collectDeclarations(selectorStart) {
  const start = themeCss.indexOf(selectorStart);
  if (start === -1) {
    throw new Error(`Missing theme selector: ${selectorStart}`);
  }

  const blockStart = themeCss.indexOf("{", start);
  let depth = 0;

  for (let index = blockStart; index < themeCss.length; index += 1) {
    const char = themeCss[index];

    if (char === "{") {
      depth += 1;
    } else if (char === "}") {
      depth -= 1;

      if (depth === 0) {
        const block = themeCss.slice(blockStart + 1, index);
        const declarations = new Map();
        const declarationPattern = /--([a-z0-9-]+):\s*([^;]+);/g;

        for (const match of block.matchAll(declarationPattern)) {
          declarations.set(match[1], match[2].trim());
        }

        return declarations;
      }
    }
  }

  throw new Error(`Unclosed theme selector: ${selectorStart}`);
}

const themes = [
  [":root", collectDeclarations(":root {"), "light"],
  ['html[data-theme="dark"]', collectDeclarations(":root {"), "dark"],
];

function gatedValue(value, mode) {
  if (!value) {
    return value;
  }

  const lightStart = "var(--is-light, ";
  const darkStart = ") var(--is-dark, ";

  if (!value.startsWith(lightStart) || !value.endsWith(")")) {
    return value;
  }

  const splitIndex = value.indexOf(darkStart);
  if (splitIndex === -1) {
    return value;
  }

  const lightValue = value.slice(lightStart.length, splitIndex);
  const darkValue = value.slice(splitIndex + darkStart.length, -1);

  return mode === "dark" ? darkValue : lightValue;
}

function srgbToLinear(channels) {
  return channels.map((channel) => {
    const normalized = channel / 255;

    return normalized <= 0.04045
      ? normalized / 12.92
      : ((normalized + 0.055) / 1.055) ** 2.4;
  });
}

function parseColor(value) {
  const hexMatch = value.match(/^#([0-9a-f]{6})$/i);
  if (hexMatch) {
    return srgbToLinear(
      hexMatch[1].match(/.{2}/g).map((channel) => parseInt(channel, 16)),
    );
  }

  const rgbMatch = value.match(
    /^rgba?\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)(?:\s*,\s*[0-9.]+)?\s*\)$/,
  );

  if (!rgbMatch) {
    throw new Error(`Expected hex or RGB color value, got: ${value}`);
  }

  return srgbToLinear(
    [Number(rgbMatch[1]), Number(rgbMatch[2]), Number(rgbMatch[3])],
  );
}

function relativeLuminance(rgb) {
  return 0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2];
}

function contrastRatio(first, second) {
  const light = Math.max(relativeLuminance(first), relativeLuminance(second));
  const dark = Math.min(relativeLuminance(first), relativeLuminance(second));

  return (light + 0.05) / (dark + 0.05);
}

function colorFor(declarations, token, mode) {
  const value = gatedValue(declarations.get(token), mode);

  if (!value) {
    throw new Error(`Missing token: --${token}`);
  }

  return parseColor(value);
}

const failures = [];

for (const [themeName, declarations, mode] of themes) {
  for (const [backgroundToken, foregroundToken] of textPairs) {
    const ratio = contrastRatio(
      colorFor(declarations, backgroundToken, mode),
      colorFor(declarations, foregroundToken, mode),
    );

    if (ratio < textContrastThreshold) {
      failures.push(
        `${themeName}: --${foregroundToken} on --${backgroundToken} is ${ratio.toFixed(2)}:1, expected >= ${textContrastThreshold}:1`,
      );
    }
  }

  for (const adjacentToken of focusAdjacentPairs) {
    const ratio = contrastRatio(
      colorFor(declarations, "focus", mode),
      colorFor(declarations, adjacentToken, mode),
    );

    if (ratio < focusContrastThreshold) {
      failures.push(
        `${themeName}: --focus against --${adjacentToken} is ${ratio.toFixed(2)}:1, expected >= ${focusContrastThreshold}:1`,
      );
    }
  }
}

if (failures.length > 0) {
  console.error("Theme contrast validation failed:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log(
  `Theme contrast validation passed for ${textPairs.length} text pairs and ${focusAdjacentPairs.length} focus-adjacent pairs in ${themes.length} themes.`,
);
