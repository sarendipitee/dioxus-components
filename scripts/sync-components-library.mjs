#!/usr/bin/env node

import {
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const registryManifestPath = join(repoRoot, 'component.json');
const previewComponentsDir = join(repoRoot, 'preview/src/components');
const previewAssetsDir = join(repoRoot, 'preview/assets');
const crateRoot = join(repoRoot, 'dioxus-components');
const crateComponentsDir = join(crateRoot, 'src/components');

const registryManifest = JSON.parse(readFileSync(registryManifestPath, 'utf8'));
const componentEntries = registryManifest.members
  .map((member) => member.replace(/^preview\/src\/components\//, ''))
  .sort((left, right) => left.localeCompare(right));

const existingCrateArtifacts = new Map();
for (const componentName of componentEntries) {
  const componentDir = join(crateComponentsDir, componentName);
  const componentSource = join(componentDir, 'component.rs');
  const styleSource = join(componentDir, 'style.css');
  existingCrateArtifacts.set(componentName, {
    componentSource: existsSync(componentSource) ? readFileSync(componentSource, 'utf8') : null,
    styleSource: existsSync(styleSource) ? readFileSync(styleSource, 'utf8') : null,
  });
}

rmSync(crateComponentsDir, { recursive: true, force: true });
mkdirSync(crateComponentsDir, { recursive: true });
mkdirSync(join(crateRoot, 'assets'), { recursive: true });

for (const componentName of componentEntries) {
  const sourceDir = join(previewComponentsDir, componentName);
  const targetDir = join(crateComponentsDir, componentName);

  mkdirSync(targetDir, { recursive: true });
  const previewComponentSource = readFileSync(join(sourceDir, 'component.rs'), 'utf8');
  const existingCrateArtifact = existingCrateArtifacts.get(componentName);
  const previewIsReexportShim = isPreviewReexportShim(previewComponentSource);

  if (previewIsReexportShim && !existingCrateArtifact?.componentSource) {
    throw new Error(
      `Refusing to sync preview shim for "${componentName}" into dioxus-components without an existing crate implementation. Add dioxus-components/src/components/${componentName}/component.rs first.`,
    );
  }
  if (previewIsReexportShim && !existingCrateArtifact?.styleSource) {
    throw new Error(
      `Refusing to sync preview shim for "${componentName}" into dioxus-components without an existing crate stylesheet. Add dioxus-components/src/components/${componentName}/style.css first.`,
    );
  }

  writeFileSync(
    join(targetDir, 'style.css'),
    previewIsReexportShim ? existingCrateArtifact.styleSource : readFileSync(join(sourceDir, 'style.css'), 'utf8'),
    'utf8',
  );
  writeFileSync(
    join(targetDir, 'component.rs'),
    previewIsReexportShim && existingCrateArtifact.componentSource
      ? existingCrateArtifact.componentSource
      : previewComponentSource,
    'utf8',
  );

  normalizeStyleSheet(join(targetDir, 'style.css'));

  const componentSource = join(targetDir, 'component.rs');
  const moduleSource = join(targetDir, 'mod.rs');
  if (existsSync(componentSource) && readFileSync(componentSource, 'utf8').trim() === '') {
    writeFileSync(moduleSource, 'mod component;\n', 'utf8');
  } else if (existsSync(componentSource)) {
    writeFileSync(moduleSource, 'mod component;\npub use component::*;\n', 'utf8');
    writeFileSync(
      componentSource,
      convertGeneratedComponent(componentName, readFileSync(componentSource, 'utf8')),
      'utf8',
    );
  }
}

cpSync(
  join(previewAssetsDir, 'dx-components-theme.css'),
  join(crateRoot, 'assets/dx-components-theme.css'),
);

const publicModules = [];
const publicReexports = [];
const globalStyleComponents = [];
for (const componentName of componentEntries) {
  publicModules.push(`pub mod ${componentName};`);

  const componentSource = join(crateComponentsDir, componentName, 'component.rs');
  if (existsSync(componentSource) && readFileSync(componentSource, 'utf8').trim() !== '') {
    publicReexports.push(`pub use ${componentName}::*;`);
  }

  if (!componentUsesCssModule(componentSource)) {
    globalStyleComponents.push(componentName);
  }
}

writeFileSync(
  join(crateComponentsDir, 'mod.rs'),
  `${publicModules.join('\n')}\n\n${publicReexports.join('\n')}\n`,
  'utf8',
);

writeFileSync(
  join(crateRoot, 'src/styles.rs'),
  `pub const COMPONENT_CSS: &str = concat!(\n${globalStyleComponents
    .map((componentName) => `    include_str!("components/${componentName}/style.css"),`)
    .join('\n')}\n);\n\npub const THEME_CSS: &str = include_str!("../assets/dx-components-theme.css");\n`,
  'utf8',
);

console.log(`Synced ${componentEntries.length} components into dioxus-components.`);

function componentUsesCssModule(componentSource) {
  return (
    existsSync(componentSource) &&
    /#\[css_module\("[^"]+"\)\]/.test(readFileSync(componentSource, 'utf8'))
  );
}

function isPreviewReexportShim(source) {
  return /^pub use dioxus_components::[a-z0-9_]+::\*;\s*$/.test(source.trim());
}

function normalizeStyleSheet(styleSource) {
  const markerPattern = /\n?\/\* dioxus-components-css-module-source: .* \*\/\n?$/;
  const css = readFileSync(styleSource, 'utf8').replace(markerPattern, '').trimEnd();
  writeFileSync(styleSource, `${css}\n`, 'utf8');
}

function convertGeneratedComponent(componentName, source) {
  let converted = convertCssModuleClasses(source);

  if (componentName === 'input') {
    converted = convertInputClassToMergedAttributes(converted);
  }

  return converted;
}

function convertInputClassToMergedAttributes(source) {
  if (source.includes('let attributes = merge_attributes(vec![base, attributes]);')) {
    return source;
  }

  let converted = source.replace(
    'use dioxus::prelude::*;\n',
    'use dioxus::prelude::*;\nuse dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};\n',
  );

  converted = converted.replace(
    ') -> Element {\n    rsx! {',
    `) -> Element {
    let base = attributes!(input {
        class: Styles::dx_input,
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {`,
  );

  converted = converted.replace(
    /\n\s+class: Styles::dx_input\.to_string\(\),\n(\s+oninput:)/,
    '\n$1',
  );

  return converted;
}

function convertCssModuleClasses(source) {
  let converted = source;

  if (converted.includes('.inner')) {
    converted = converted.replace(
      /fn to_class\(self\) -> &'static str/g,
      'fn to_class(self) -> String',
    );
  }

  return converted
    .replace(/Styles::([A-Za-z0-9_]+)\.inner/g, 'Styles::$1')
    .replace(/\bStyles::([A-Za-z0-9_]+)\b(?!\s*(?:\.|::|\())/g, 'Styles::$1');
}
