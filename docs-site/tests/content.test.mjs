import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { documentation, toolDir } from '../scripts/prepare.mjs';

const repo = path.resolve(toolDir, '..');

test('every published guide is headed with the title the sidebar and the extension Help use', () => {
  const mismatches = [];
  for (const { route, title, source } of Object.values(documentation.pages)) {
    if (!source) continue;
    const heading = fs.readFileSync(path.join(repo, source), 'utf8').split(/\r?\n/).find(line => /^# /.test(line));
    if (heading !== `# ${title}`) mismatches.push(`${route}: "${heading}" should be "# ${title}" (documentation.json)`);
  }
  assert.deepEqual(mismatches, []);
});
