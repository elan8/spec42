import assert from 'node:assert/strict';
import { test } from 'node:test';
import { baselineFromReport, compareBaseline, sysmlCorpora } from './omg-corpus-baseline.mjs';

function diagnostic(code, file = 'Example.sysml', severity = 2, line = 1) {
  return { file, code, severity, range: {
    start: { line, character: 0 }, end: { line, character: 5 },
  } };
}

function inventory(validation = [], kerml = []) {
  const corpora = sysmlCorpora.map(corpus => ({ corpus, files: 1, diagnostics: [] }));
  corpora[0].diagnostics = validation;
  corpora.push({ corpus: 'kerml/src/examples', files: 1, diagnostics: kerml });
  return { libraryVersion: 'pinned-release', releaseCommit: 'pinned-commit', corpora };
}

test('a removal cannot hide an unrelated addition', () => {
  const previous = inventory([diagnostic('old')]);
  assert.deepEqual(compareBaseline(previous, inventory()), {
    added: [], removed: [{ occurrence: [sysmlCorpora[0], 'Example.sysml', 2, 'old', 1, 0, 1, 5], count: 1 }],
  });
  const result = compareBaseline(previous, inventory([diagnostic('new')]));
  assert.equal(result.added[0].occurrence[3], 'new');
  assert.equal(result.removed[0].occurrence[3], 'old');
});

test('occurrence identity includes its file, severity, range, and multiplicity', () => {
  const original = diagnostic('code');
  for (const changed of [diagnostic('code', 'Other.sysml'), diagnostic('code', 'Example.sysml', 1),
    diagnostic('code', 'Example.sysml', 2, 3)]) {
    const { added, removed } = compareBaseline(inventory([original]), inventory([changed]));
    assert.equal(added.length, 1);
    assert.equal(removed.length, 1);
  }
  const { added, removed } = compareBaseline(inventory([original]), inventory([original, original, original]));
  assert.equal(added[0].count, 2);
  assert.deepEqual(removed, []);
});

test('rejects a release or inventory mismatch and leaves KerML informational', () => {
  const report = inventory();
  assert.throws(() => compareBaseline(report, { ...report, releaseCommit: 'another' }), /same pinned OMG release/);
  const smaller = inventory();
  smaller.corpora.shift();
  assert.throws(() => compareBaseline(report, smaller), /Missing or incomplete corpus/);
  const larger = inventory();
  larger.corpora[0].files += 1;
  assert.throws(() => compareBaseline(report, larger), /file count changed/);
  assert.deepEqual(compareBaseline(report, inventory([], [diagnostic('kerml_only')])), { added: [], removed: [] });
});

test('recording is deterministic and retains every gated occurrence', () => {
  const first = { ...diagnostic('first'), message: 'Display text' };
  const second = diagnostic('second');
  const report = inventory([first, second, first], [diagnostic('informational')]);
  const baseline = baselineFromReport(report);
  assert.deepEqual(baseline, baselineFromReport(inventory([second, first, first])));
  assert.deepEqual(compareBaseline(baseline, report), { added: [], removed: [] });
  assert.equal(baseline.corpora[0].diagnostics[0].message, undefined);
});
