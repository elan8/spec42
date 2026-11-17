import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { documentation, prepare, releases, rewriteLinks, version, withoutHtmlComments, writeLogo } from '../scripts/prepare.mjs';

test('stable versions sort numerically and retain every major/minor and patch', () => {
  assert.deepEqual(releases(['v0.59.0', 'v0.60.2', 'v0.60.10', 'v1.0.0', 'v2.0.0', 'v0.61.0', 'v2.1.0-rc.1', 'v01.0.0', 'v0.60.2']),
    ['v2.0.0', 'v1.0.0', 'v0.61.0', 'v0.60.10', 'v0.60.2']);
  assert.equal(version('main'), null);
});

test('generated-file comments are hidden but comments in code fences are kept', () => {
  const fence = '```';
  const kept = ['# Title', `${fence}html`, '<!-- example -->', fence, ''].join('\n');
  assert.equal(withoutHtmlComments(`<!-- GENERATED -->\n${kept}`), kept);
});

test('links resolve to guide pages or the matching source revision; code is untouched', () => {
  const context = { route: 'getting-started', source: 'docs/user/GETTING-STARTED.md', ref: 'v1.2.3', exists: target => target !== 'docs/missing.md' };
  const out = rewriteLinks([
    '[Examples](EXAMPLES.md#a-section) and [source](../../README.md) and ![shot](../img/a.png)',
    'Label with code: [`.github/x.md`](../../.github/x.md) stays a link; `[not](../../a.md)` stays code.',
    '[web](https://example.com/a) and [anchor](#here)',
    '```md',
    '[kept](../../b.md)',
    '```'
  ].join('\n'), context).split('\n');
  assert.equal(out[0], '[Examples](examples.md#a-section) and [source](https://github.com/elan8/spec42/blob/v1.2.3/README.md) and ![shot](https://github.com/elan8/spec42/raw/v1.2.3/docs/img/a.png)');
  assert.equal(out[1], 'Label with code: [`.github/x.md`](https://github.com/elan8/spec42/blob/v1.2.3/.github/x.md) stays a link; `[not](../../a.md)` stays code.');
  assert.equal(out[2], '[web](https://example.com/a) and [anchor](#here)');
  assert.equal(out[4], '[kept](../../b.md)');
  assert.throws(() => rewriteLinks('[x](../missing.md)', context), /Missing link target: docs\/missing\.md/);
  assert.throws(() => rewriteLinks('[x](../../../outside.md)', context), /Unresolved link/);
});

test('snapshots use tagged content, preserve links and isolate unreleased changes', () => {
  const repo = fs.mkdtempSync(path.join(os.tmpdir(), 'spec42-docs-'));
  const siteRoot = path.join(repo, 'out/site');
  const manifestPath = path.join(repo, 'out/manifest.json');
  const git = (...args) => execFileSync('git', args, { cwd: repo, stdio: 'pipe' });
  const write = (file, text) => { fs.mkdirSync(path.dirname(path.join(repo, file)), { recursive: true }); fs.writeFileSync(path.join(repo, file), text); };
  const read = file => fs.readFileSync(path.join(siteRoot, file), 'utf8');
  const options = tags => ({ repo, tags, siteRoot, manifestPath });
  try {
    git('init');
    git('config', 'user.email', 'docs-test@example.invalid');
    git('config', 'user.name', 'Documentation test');
    for (const { source } of Object.values(documentation.pages)) {
      if (source && !source.endsWith('CALCULATION-EXPRESSIONS.md')) write(source, '# A guide\n\n## A section\n\nReleased content. {{ not a template }}\n');
    }
    write('docs/user/GETTING-STARTED.md', '<!-- GENERATED -->\n# Install\n\n[Examples](EXAMPLES.md#a-section)\n\n[Source](../../README.md)\n');
    write('README.md', '# Source\n');
    git('add', '.'); git('commit', '-m', 'Release docs');
    for (const tag of ['v0.60.0', 'v0.60.10', 'v0.61.0', 'v1.0.0']) git('tag', tag);
    write('docs/user/EXAMPLES.md', '# Examples\n\n## A section\n\nUnreleased content.\n');
    write('docs/user/CALCULATION-EXPRESSIONS.md', '# New guide\n');

    const manifest = prepare(options(['v0.60.0', 'v0.60.10', 'v0.61.0', 'v1.0.0']));
    assert.equal(manifest.latest, 'v1.0.0');
    assert.deepEqual(manifest.versions, ['v1.0.0', 'v0.61.0', 'v0.60.10', 'v0.60.0']);
    assert.deepEqual(manifest.missing['v0.60.0'], ['calculations'], 'a guide new since the tag is reported missing');
    assert.deepEqual(manifest.missing.dev, []);
    assert.deepEqual(JSON.parse(fs.readFileSync(manifestPath, 'utf8')), manifest);

    const released = read('archive/v0.60.0/examples.md');
    const dev = read('src/examples.md');
    assert.match(released, /Released content/);
    assert.doesNotMatch(released, /Unreleased content/);
    assert.match(dev, /Unreleased content/);
    assert.match(released, /source: docs\/user\/EXAMPLES\.md\nref: v0\.60\.0/);
    assert.match(dev, /ref: main/);
    assert.match(released, /&#123;&#123; not a template }}/, 'interpolation in prose is escaped');

    const start = read('archive/v0.60.0/getting-started.md');
    assert.match(start, /\]\(examples\.md#a-section\)/);
    assert.match(start, /blob\/v0\.60\.0\/README\.md/);
    assert.doesNotMatch(start, /GENERATED/);
    assert.match(read('archive/v0.60.0/calculations.md'), /This release does not include this guide/);
    assert.doesNotMatch(read('archive/v0.60.0/calculations.md'), /\nsource:/, 'a stub names no source');
    assert.match(read('src/calculations.md'), /New guide/);

    prepare(options(['v0.60.0']));
    assert.equal(fs.existsSync(path.join(siteRoot, 'archive/v1.0.0')), false, 'removed releases leave no stale output');

    write('docs/user/GETTING-STARTED.md', '# Install\n\n[Broken file](MISSING.md)\n');
    assert.throws(() => prepare(options(['v0.60.0'])), /Missing link target/);
    assert.throws(() => prepare(options([])), /No published stable releases/);
    write('docs/user/GETTING-STARTED.md', '# Install\n');
    assert.deepEqual(prepare({ ...options([]), quick: true }).versions, [], 'a quick build prepares only the working tree');
  } finally {
    if (!repo.startsWith(path.join(os.tmpdir(), 'spec42-docs-'))) throw new Error('Unexpected test directory');
    fs.rmSync(repo, { recursive: true, force: true });
  }
});

test('the site logo is derived from the repository mark, not a second copy', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'spec42-logo-'));
  try {
    writeLogo({ siteRoot: root });
    const svg = fs.readFileSync(path.join(root, 'public/logo.svg'), 'utf8');
    const mark = fs.readFileSync(new URL('../../vscode/design/logo.svg', import.meta.url), 'utf8');
    assert.ok(svg.includes('fill="#0891b2"'), 'cyan tile');
    assert.ok(!svg.includes('currentColor'), 'the glyph is drawn in the tile foreground colour');
    // Every shape of the source mark is in the derived logo.
    for (const [, d] of mark.matchAll(/\sd="([^"]+)"/g)) assert.ok(svg.includes(d), d);
    for (const [, x, y] of mark.matchAll(/<rect x="(\d+)" y="(\d+)"/g)) assert.ok(svg.includes(`<rect x="${x}" y="${y}"`), `rect ${x},${y}`);
  } finally {
    if (!root.startsWith(path.join(os.tmpdir(), 'spec42-logo-'))) throw new Error('Unexpected test directory');
    fs.rmSync(root, { recursive: true, force: true });
  }
});
