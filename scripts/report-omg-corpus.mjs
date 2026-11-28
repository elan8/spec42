// Produce a deterministic error and warning inventory for the nightly OMG corpus job.
import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { isAbsolute, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const corpora = [
  'sysml/src/validation',
  'sysml/src/examples',
  'sysml/src/training',
  'kerml/src/examples',
];

function collect(binary, release, corpus) {
  const root = join(release, corpus);
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
    if (path === '..' || path.startsWith(`..${sep}`) || isAbsolute(path)) {
      throw new Error(`${corpus}: Spec42 reported a document outside the corpus: ${document.uri}`);
    }
    const file = path.split(sep).join('/');
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
  if (errors !== summary.error_count || warnings !== summary.warning_count || command.status !== Number(errors > 0)) {
    throw new Error(`${corpus}: Spec42 diagnostic totals disagree with the CLI summary`);
  }
  const byCode = [...counts.values()].sort((left, right) => compare(left.code, right.code));
  diagnostics.sort((left, right) => (
    compare(left.file, right.file)
    || left.range.start.line - right.range.start.line
    || left.range.start.character - right.range.start.character
    || compare(left.code, right.code)
  ));
  return { corpus, files: documents.length, errors, warnings, byCode, diagnostics };
}

function compare(left, right) {
  return left < right ? -1 : left > right ? 1 : 0;
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
  const reports = corpora.map(corpus => collect(resolve(binary), resolve(release), corpus));
  mkdirSync(output, { recursive: true });
  writeFileSync(join(output, 'diagnostics.json'), `${JSON.stringify({ corpora: reports }, null, 2)}\n`);
  writeFileSync(join(output, 'summary.md'), markdown(reports));
  for (const { corpus, files, errors, warnings } of reports) {
    console.log(`${corpus}: ${files} files, ${errors} errors, ${warnings} warnings`);
  }
}
