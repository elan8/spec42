import fs from 'node:fs';
import { defineVersionedConfig } from '@viteplus/versions';
import { sharedConfig } from './shared.mjs';
import { groups, manifestFile } from '../../scripts/prepare.mjs';

// Written by `scripts/prepare.mjs`: the latest stable tag and the guides each older tag lacks.
const manifest = JSON.parse(fs.readFileSync(manifestFile, 'utf8'));
const latest = manifest.latest;

const without = routes => groups
  .map(group => ({ ...group, pages: group.pages.filter(page => !routes.includes(page.route)) }))
  .filter(group => group.pages.length);

const shared = sharedConfig({
  product: 'Spec42',
  slug: 'spec42',
  repo: 'elan8/spec42',
  base: process.env.DOCS_BASE || '/spec42/',
  latest: latest ?? 'v0.0.0',
  development: 'dev',
  description: 'Spec42 user guides: build, validate and explore SysML v2 models in VS Code and on the command line.',
  footer: 'Spec42 is the SysML v2 language server and tooling behind Elan8 model-based engineering.',
  license: 'MIT License',
  groups,
  // An older release shows only the guides it shipped.
  sidebarByVersion: Object.fromEntries(Object.entries(manifest.missing).filter(([, routes]) => routes.length).map(([version, routes]) => [version, without(routes)])),
  // Guides are Markdown without raw HTML; keep it that way so prose such as `<name>` stays text.
  markdown: { html: false }
});

export default defineVersionedConfig({
  ...shared,
  // The switcher is the `VersionSwitcher` component placed in the shared nav; the plugin's own
  // automatic dropdown would repeat it.
  versionsConfig: latest ? { current: 'dev', latest, versionSwitcher: false } : { current: 'dev', versionSwitcher: false }
});
