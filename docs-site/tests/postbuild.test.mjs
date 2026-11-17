import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { checkLinks, redirectPage, redirects, writeRedirects } from '../scripts/postbuild.mjs';

const versions = ['v0.61.1', 'v0.61.0', 'v0.60.2', 'v0.60.0'];

test('aliases and former entry points redirect to the newest matching release', () => {
  const map = new Map(redirects({ base: '/spec42/', versions }));
  assert.equal(map.get('latest/features.html'), '/spec42/v0.61.1/features');
  assert.equal(map.get('latest/index.html'), '/spec42/v0.61.1/');
  assert.equal(map.get('0.61/features.html'), '/spec42/v0.61.1/features');
  assert.equal(map.get('0.60/features.html'), '/spec42/v0.60.2/features');
  assert.equal(map.get('guide/getting-started.html'), '/spec42/v0.61.1/getting-started');
  assert.equal(map.get('reference/whats-included.html'), '/spec42/v0.61.1/included');
  const page = redirectPage('/spec42/v0.61.1/features');
  assert.ok(page.includes('content="0;url=/spec42/v0.61.1/features"'));
  assert.ok(page.includes('rel="canonical" href="/spec42/v0.61.1/features"'));
});

test('built-site link check reports missing pages and headings, and accepts valid links', () => {
  const dist = fs.mkdtempSync(path.join(os.tmpdir(), 'spec42-dist-'));
  try {
    const page = (name, body) => { fs.mkdirSync(path.dirname(path.join(dist, name)), { recursive: true }); fs.writeFileSync(path.join(dist, name), `<html><body>${body}</body></html>`); };
    page('v1/a.html', '<h2 id="top">Top</h2><a href="/spec42/v1/b#part">ok</a><a href="/spec42/v1/b.html#part">also ok</a><a href="/spec42/v1/">home</a><a href="#top">self</a><a href="https://example.com/x#y">external</a><a href="/spec42/assets/app.js">asset</a>');
    page('v1/b.html', '<h2 id="part">Part</h2>');
    page('v1/index.html', '<p>home</p>');
    assert.deepEqual(checkLinks(dist, '/spec42/'), []);
    page('v1/c.html', '<a href="/spec42/v1/missing">gone</a><a href="/spec42/v1/b#nope">bad heading</a><a href="#nowhere">bad self</a>');
    const problems = checkLinks(dist, '/spec42/');
    assert.equal(problems.length, 3);
    assert.ok(problems.some(p => p.includes('/spec42/v1/missing does not exist')));
    assert.ok(problems.some(p => p.includes('/spec42/v1/b#nope has no matching heading')));
    assert.ok(problems.some(p => p.includes('#nowhere has no matching heading')));
    // Redirect stubs are not checked as pages, and writing them creates both spellings' file.
    assert.equal(writeRedirects(dist, { base: '/spec42/', versions }) > 0, true);
    assert.ok(fs.existsSync(path.join(dist, 'latest/features.html')));
  } finally {
    if (!dist.startsWith(path.join(os.tmpdir(), 'spec42-dist-'))) throw new Error('Unexpected test directory');
    fs.rmSync(dist, { recursive: true, force: true });
  }
});
