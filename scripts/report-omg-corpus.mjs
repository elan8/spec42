// Produce a deterministic error and warning inventory for the nightly OMG corpus job.
import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { isAbsolute, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { sysmlCorpora } from './omg-corpus-baseline.mjs';

const corpora = [...sysmlCorpora, 'kerml/src/examples'];

function collect(binary, release, corpus) {
  const root = join(release, corpus);
  const tracked = git(release, 'ls-files', '-z', '--', corpus)
    .split('\0').filter(path => /\.(sysml|kerml)$/iu.test(path))
    .map(path => path.slice(corpus.length + 1));
  const unseen = new Set(tracked);
  const command = spawnSync(binary, [
    '--disable-kpar-library', 'domain', '--disable-kpar-library', 'method',
    'check', root, '--workspace-root', root, '--format', 'json',
  ], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024, timeout: 180_000 });
  if (command.error) throw new Error(`${corpus}: could not run spec42 check`, { cause: command.error });
  // The CLI exits 1 when it successfully reports model errors and 2 on execution failure.
  if (command.status !== 0 && command.status !== 1) {
    throw new Error(`${corpus}: spec42 check failed (${command.status ?? command.signal})\n${command.stderr.trim()}`);
  }
  const { documents, summary } = JSON.parse(command.stdout);
  if (!Array.isArray(documents) || !documents.length || summary?.document_count !== documents.length) {
    throw new Error(`${corpus}: missing or incomplete Spec42 document inventory`);
  }

  const counts = new Map();
  const diagnostics = [];
  let errors = 0;
  let warnings = 0;
  for (const document of documents) {
    const path = relative(root, fileURLToPath(document.uri));
    const file = path.split(sep).join('/');
    if (isAbsolute(path) || !unseen.delete(file)) {
      throw new Error(`${corpus}: Spec42 reported an unexpected or duplicate document: ${document.uri}`);
    }
    for (const { severity, code, range, message } of document.diagnostics) {
      if (severity !== 1 && severity !== 2) continue;
      if (severity === 1) errors += 1;
      else warnings += 1;
      diagnostics.push({ file, severity, code, range, message });
      const entry = counts.get(code) ?? { code, errors: 0, warnings: 0 };
      if (severity === 1) entry.errors += 1;
      else entry.warnings += 1;
      counts.set(code, entry);
    }
  }
  if (unseen.size) throw new Error(`${corpus}: Spec42 skipped ${[...unseen].sort().join(', ')}`);
  if (errors !== summary.error_count || warnings !== summary.warning_count || command.status !== Number(errors > 0)) {
    throw new Error(`${corpus}: Spec42 diagnostic totals disagree with the CLI summary`);
  }
  const byCode = [...counts.values()].sort((left, right) => compare(left.code, right.code));
  diagnostics.sort((left, right) => (
    compare(left.file, right.file)
    || left.range.start.line - right.range.start.line
    || left.range.start.character - right.range.start.character
    || left.range.end.line - right.range.end.line
    || left.range.end.character - right.range.end.character
    || left.severity - right.severity
    || compare(left.code, right.code)
    || compare(left.message, right.message)
  ));
  return { corpus, files: documents.length, errors, warnings, byCode, diagnostics };
}

function compare(left, right) {
  return left < right ? -1 : left > right ? 1 : 0;
}

function git(release, ...args) {
  const result = spawnSync('git', ['-C', release, ...args], { encoding: 'utf8' });
  if (result.error || result.status !== 0) {
    throw new Error(`Could not read the pinned OMG checkout: ${result.stderr?.trim()}`, { cause: result.error });
  }
  return result.stdout;
}

function markdown(reports) {
  const sections = [
    '## OMG corpus diagnostics',
    '',
    '| Corpus | Files | Errors | Warnings |',
    '| --- | ---: | ---: | ---: |',
    ...reports.map(({ corpus, files, errors, warnings }) => `| \`${corpus}\` | ${files} | ${errors} | ${warnings} |`),
    '',
    'See the `omg-corpus-report` artifact for diagnostic files, zero-based ranges, and messages.',
  ];
  for (const { corpus, byCode } of reports) {
    sections.push('', `### \`${corpus}\``, '', '| Code | Errors | Warnings |', '| --- | ---: | ---: |');
    for (const { code, errors, warnings } of byCode) {
      sections.push(`| \`${code}\` | ${errors} | ${warnings} |`);
    }
  }
  return `${sections.join('\n')}\n`;
}

const [binary, release, output] = process.argv.slice(2);
if (!binary || !release || !output) {
  console.error('Usage: node scripts/report-omg-corpus.mjs <spec42-bin> <SysML-v2-Release-dir> <output-dir>');
  process.exitCode = 2;
} else {
  const libraryVersion = JSON.parse(readFileSync(
    new URL('../config/standard-library.json', import.meta.url), 'utf8',
  )).version;
  const releaseRoot = resolve(release);
  const [pinnedCommit, releaseCommit] = git(
    releaseRoot, 'rev-parse', `${libraryVersion}^{commit}`, 'HEAD',
  ).trim().split(/\s+/);
  if (pinnedCommit !== releaseCommit || git(releaseRoot, 'status', '--porcelain', '--', ...corpora)) {
    throw new Error(`Expected a checkout of the pinned OMG release ${libraryVersion}`);
  }
  const reports = corpora.map(corpus => collect(resolve(binary), releaseRoot, corpus));
  mkdirSync(output, { recursive: true });
  writeFileSync(join(output, 'diagnostics.json'), `${JSON.stringify({
    libraryVersion, releaseCommit, corpora: reports,
  }, null, 2)}\n`);
  writeFileSync(join(output, 'summary.md'), markdown(reports));
  for (const { corpus, files, errors, warnings } of reports) {
    console.log(`${corpus}: ${files} files, ${errors} errors, ${warnings} warnings`);
  }
}
