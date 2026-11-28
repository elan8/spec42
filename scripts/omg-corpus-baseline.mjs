// Compare immutable diagnostic occurrences in the pinned SysML corpus.
import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

export const sysmlCorpora = [
  'sysml/src/validation',
  'sysml/src/examples',
  'sysml/src/training',
];

function occurrences(report) {
  const byCorpus = new Map(report.corpora.map(corpus => [corpus.corpus, corpus]));
  if (byCorpus.size !== report.corpora.length) throw new Error('Duplicate corpus in diagnostic inventory');
  const entries = new Map();
  for (const name of sysmlCorpora) {
    const corpus = byCorpus.get(name);
    if (!corpus || !Number.isInteger(corpus.files) || corpus.files < 1 || !Array.isArray(corpus.diagnostics)) {
      throw new Error(`Missing or incomplete corpus ${name}`);
    }
    for (const { file, severity, code, range } of corpus.diagnostics) {
      if ((severity !== 1 && severity !== 2) || typeof file !== 'string' || typeof code !== 'string'
          || !range || ![range.start?.line, range.start?.character, range.end?.line, range.end?.character]
            .every(value => Number.isInteger(value) && value >= 0)) {
        throw new Error(`Invalid diagnostic occurrence in ${name}`);
      }
      const key = JSON.stringify([
        name, file, severity, code, range.start.line, range.start.character,
        range.end.line, range.end.character,
      ]);
      entries.set(key, (entries.get(key) ?? 0) + 1);
    }
  }
  return entries;
}

export function baselineFromReport(report) {
  const { libraryVersion, releaseCommit } = report;
  if (!libraryVersion || !releaseCommit) throw new Error('The report must identify the pinned OMG release');
  const indexed = occurrences(report);
  return {
    libraryVersion,
    releaseCommit,
    corpora: sysmlCorpora.map(name => ({
      corpus: name,
      files: report.corpora.find(corpus => corpus.corpus === name).files,
      diagnostics: [...indexed.keys()].sort().flatMap(key => {
        const [corpus, file, severity, code, line, column, endLine, endColumn] = JSON.parse(key);
        if (corpus !== name) return [];
        const range = { start: { line, character: column }, end: { line: endLine, character: endColumn } };
        return Array.from({ length: indexed.get(key) }, () => ({ file, severity, code, range }));
      }),
    })),
  };
}

export function compareBaseline(baseline, report) {
  if (!baseline.libraryVersion || baseline.libraryVersion !== report.libraryVersion
      || !baseline.releaseCommit || baseline.releaseCommit !== report.releaseCommit) {
    throw new Error('The baseline and report must use the same pinned OMG release');
  }
  const before = occurrences(baseline);
  const after = occurrences(report);
  for (const name of sysmlCorpora) {
    const fileCount = report => report.corpora.find(corpus => corpus.corpus === name).files;
    if (fileCount(baseline) !== fileCount(report)) {
      throw new Error(`The file count changed for pinned corpus ${name}`);
    }
  }
  const difference = (left, right) => [...left.keys()].sort().flatMap(key => {
    const count = left.get(key) - (right.get(key) ?? 0);
    return count > 0 ? [{ occurrence: JSON.parse(key), count }] : [];
  });
  return { added: difference(after, before), removed: difference(before, after) };
}

function describe({ occurrence: [corpus, file, severity, code, line, column], count }) {
  return `${corpus}/${file}:${line + 1}:${column + 1} ${severity === 1 ? 'error' : 'warning'} ${code} (${count})`;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const [baseline, report, operation] = process.argv.slice(2);
  if (!baseline || !report || (operation && operation !== '--record')) {
    console.error('Usage: node scripts/omg-corpus-baseline.mjs <baseline.json> <diagnostics.json> [--record]');
    process.exitCode = 2;
  } else {
    const read = file => JSON.parse(readFileSync(file, 'utf8'));
    const current = read(report);
    if (operation === '--record') {
      writeFileSync(baseline, `${JSON.stringify(baselineFromReport(current), null, 2)}\n`);
    } else {
      const { added, removed } = compareBaseline(read(baseline), current);
      for (const occurrence of removed) console.log(`Resolved: ${describe(occurrence)}`);
      for (const occurrence of added) console.error(`New: ${describe(occurrence)}`);
      if (removed.length) console.log('Update the reviewed baseline to record the resolved diagnostics.');
      if (added.length || removed.length) process.exitCode = 1;
    }
  }
}
