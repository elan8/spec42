import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { buildSite, releases, version } from './build.mjs';
import navigation from './navigation.json' with { type: 'json' };

test('stable versions sort numerically and retain every major/minor and patch', () => {
  assert.deepEqual(releases(['v0.59.0', 'v0.60.2', 'v0.60.10', 'v1.0.0', 'v2.0.0', 'v0.61.0', 'v2.1.0-rc.1', 'v01.0.0', 'v0.60.2']),
    ['v2.0.0', 'v1.0.0', 'v0.61.0', 'v0.60.10', 'v0.60.2']);
  assert.equal(version('main'), null);
});

test('snapshots use tagged content, preserve links and isolate unreleased changes', () => {
  const repo = fs.mkdtempSync(path.join(os.tmpdir(), 'spec42-docs-'));
  const out = path.join(repo, 'target/docs-site');
  const git = (...args) => execFileSync('git', args, { cwd: repo, stdio: 'pipe' });
  const write = (file, text) => { fs.mkdirSync(path.dirname(path.join(repo, file)), { recursive: true }); fs.writeFileSync(path.join(repo, file), text); };
  try {
    git('init');
    git('config', 'user.email', 'docs-test@example.invalid');
    git('config', 'user.name', 'Documentation test');
    for (const [, , source] of navigation.flatMap(group => group.pages)) {
      if (source && !source.endsWith('CALCULATION-EXPRESSIONS.md')) write(source, '# A guide\n\n## A section\n\nReleased content.\n');
    }
    write('docs/user/GETTING-STARTED.md', '# Install\n\n[Examples](EXAMPLES.md#a-section)\n\n[Source](../../README.md)\n');
    write('README.md', '# Source\n');
    git('add', '.'); git('commit', '-m', 'Release docs');
    for (const tag of ['v0.60.0', 'v0.60.10', 'v0.61.0', 'v1.0.0']) git('tag', tag);
    write('docs/user/EXAMPLES.md', '# Examples\n\n## A section\n\nUnreleased content.\n');
    write('docs/user/CALCULATION-EXPRESSIONS.md', '# New guide\n');
    buildSite({ repo, out, tags: ['v0.60.0', 'v0.60.10', 'v0.61.0', 'v1.0.0'] });
    const released = fs.readFileSync(path.join(out, 'v0.60.0/examples.html'), 'utf8');
    const dev = fs.readFileSync(path.join(out, 'dev/examples.html'), 'utf8');
    assert.match(released, /Released content/);
    assert.doesNotMatch(released, /Unreleased content/);
    assert.match(dev, /Unreleased content/);
    assert.match(dev, /noindex/);
    const start = fs.readFileSync(path.join(out, 'v0.60.0/getting-started.html'), 'utf8');
    assert.match(start, /href="examples.html#a-section"/);
    assert.match(start, /blob\/v0.60.0\/README.md/);
    assert.match(fs.readFileSync(path.join(out, '0.60/index.html'), 'utf8'), /url=\/spec42\/v0.60.10\/index.html/);
    assert.match(fs.readFileSync(path.join(out, '0.61/index.html'), 'utf8'), /url=\/spec42\/v0.61.0\/index.html/);
    assert.match(fs.readFileSync(path.join(out, '1.0/index.html'), 'utf8'), /url=\/spec42\/v1.0.0\/index.html/);
    assert.match(fs.readFileSync(path.join(out, 'latest/index.html'), 'utf8'), /url=\/spec42\/v1.0.0\/index.html/);
    assert.match(fs.readFileSync(path.join(out, 'v0.60.0/calculations.html'), 'utf8'), /This release does not include this guide/);
    assert.deepEqual(JSON.parse(fs.readFileSync(path.join(out, 'versions.json'))).latest, 'v1.0.0');
    buildSite({ repo, out, tags: ['v0.60.0'] });
    assert.equal(fs.existsSync(path.join(out, 'v1.0.0')), false, 'removed releases leave no stale output');
    write('docs/user/GETTING-STARTED.md', '# Install\n\n[Broken anchor](EXAMPLES.md#missing)\n');
    assert.throws(() => buildSite({ repo, out, tags: ['v0.60.0'] }), /Broken link in dev/);
    write('docs/user/GETTING-STARTED.md', '# Install\n\n[Broken file](MISSING.md)\n');
    assert.throws(() => buildSite({ repo, out, tags: ['v0.60.0'] }), /Missing link target/);
    assert.throws(() => buildSite({ repo, out, tags: [] }), /No published stable releases/);
  } finally {
    if (!repo.startsWith(path.join(os.tmpdir(), 'spec42-docs-'))) throw new Error('Unexpected test directory');
    fs.rmSync(repo, { recursive: true, force: true });
  }
});
